use mail_canon::{canonicalize, normalize, normalize_into, canonicalize_strict, normalize_strict, normalize_strict_into, Error, OUTPUT_CAPACITY};

#[test]
fn strict_vectors() {
    for row in include_str!("../../testdata/strict-vectors.tsv").lines().filter(|s| !s.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        let raw = unhex(fields[1]);
        let name = fields[0];
        let mut output = [0xa5; OUTPUT_CAPACITY];
        if fields[2] == "-" {
            assert_eq!(normalize_strict(&raw), Err(Error::InvalidEmail), "{name}");
            assert_eq!(canonicalize_strict(&raw), Err(Error::InvalidEmail), "{name}");
            assert_eq!(normalize_strict_into(&raw, &mut output), Err(Error::InvalidEmail), "{name}");
            assert!(output.iter().all(|&b| b == 0xa5));
        } else {
            let value = normalize_strict(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
            assert_eq!(value.domain, fields[2], "{name}");
            assert_eq!(value.canonical, fields[3], "{name}");
            assert_eq!(value.provider.as_str(), fields[4], "{name}");
            assert_eq!(canonicalize_strict(&raw).unwrap(), fields[3], "{name}");
            assert_eq!(canonicalize_strict(&value.canonical).unwrap(), value.canonical, "{name}");
            let p = normalize_strict_into(&raw, &mut output).unwrap();
            assert_eq!(&output[..p.domain_len], value.domain.as_bytes(), "{name}");
            assert_eq!(&output[p.domain_len..p.domain_len + p.canonical_len], value.canonical.as_bytes(), "{name}");
            let mut small = vec![0xa5; p.domain_len + p.canonical_len - 1];
            assert_eq!(normalize_strict_into(&raw, &mut small), Err(Error::BufferTooSmall));
            assert!(small.iter().all(|&b| b == 0xa5));
        }
    }
}

fn unhex(hex: &str) -> String {
    let (pairs, remainder) = hex.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty(), "test vector must contain complete hex bytes");
    let bytes: Vec<u8> = pairs.iter().map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect();
    String::from_utf8(bytes).unwrap()
}

#[test]
fn shared_vectors() {
    for row in include_str!("../../testdata/vectors.tsv").lines().filter(|s| !s.starts_with('#')) {
        let fields: Vec<_> = row.split('\t').collect();
        let raw = unhex(fields[1]);
        let name = fields[0];
        if fields[2] == "-" {
            assert_eq!(normalize(&raw), Err(Error::InvalidEmail), "{name}");
            assert_eq!(canonicalize(&raw), Err(Error::InvalidEmail), "{name}");
            continue;
        }
        let value = normalize(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(value.domain, fields[2], "{name}");
        assert_eq!(value.canonical, fields[3], "{name}");
        assert_eq!(value.provider.as_str(), fields[4], "{name}");
        assert_eq!(canonicalize(&raw).unwrap(), fields[3], "{name}");
        assert_eq!(canonicalize(&value.canonical).unwrap(), value.canonical, "{name}");
        let mut output = [0u8; OUTPUT_CAPACITY];
        let p = normalize_into(&raw, &mut output).unwrap();
        assert_eq!(&output[..p.domain_len], value.domain.as_bytes(), "{name}");
        assert_eq!(&output[p.domain_len..p.domain_len + p.canonical_len], value.canonical.as_bytes(), "{name}");
        let mut small = vec![0xa5; p.domain_len + p.canonical_len - 1];
        assert_eq!(normalize_into(&raw, &mut small), Err(Error::BufferTooSmall));
        assert!(small.iter().all(|&b| b == 0xa5));
    }
}

#[test]
fn invalid_inputs_do_not_modify_output() {
    let mut output = [0xa5; OUTPUT_CAPACITY];
    assert_eq!(normalize_into("a..b@example.com", &mut output), Err(Error::InvalidEmail));
    assert!(output.iter().all(|&b| b == 0xa5));
}

#[test]
fn proton_and_fastmail_rules() {
    let proton = normalize_strict("User.Name_-+extra@protonmail.com").unwrap();
    assert_eq!(proton.provider.as_str(), "proton");
    assert_eq!(proton.domain, "protonmail.com");
    assert_eq!(proton.canonical, "username@protonmail.com");
    assert_eq!(canonicalize_strict(&proton.canonical).unwrap(), proton.canonical);

    let direct = normalize_strict("User.Name+tag@fastmail.com").unwrap();
    assert_eq!(direct.provider.as_str(), "fastmail");
    assert_eq!(direct.domain, "fastmail.com");
    assert_eq!(direct.canonical, "user.name@fastmail.com");

    let alias = normalize_strict("tag@User-Name.fastmail.com.au").unwrap();
    assert_eq!(alias.provider.as_str(), "fastmail");
    assert_eq!(alias.domain, "user-name.fastmail.com.au");
    assert_eq!(alias.canonical, "user-name@fastmail.com.au");
    assert_eq!(canonicalize_strict(&alias.canonical).unwrap(), alias.canonical);

    assert_eq!(normalize_strict("tag@sub.user.fastmail.com").unwrap().provider.as_str(), "custom");
    assert_eq!(normalize_strict("tag@user.fastmail.example").unwrap().provider.as_str(), "custom");
    let mut output = [0xa5; OUTPUT_CAPACITY];
    assert_eq!(normalize_strict_into("+tag@proton.me", &mut output), Err(Error::InvalidEmail));
    assert!(output.iter().all(|&b| b == 0xa5));
}

#[test]
fn generated_inputs_are_idempotent() {
    let mut state = 82751u32;
    let alphabet: Vec<char> = "abcXYZ019._+-@<> %\t\r\n\0é。\u{200b}".chars().collect();
    for _ in 0..5000 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let length = state as usize % 100;
        let mut input = String::new();
        for _ in 0..length {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            input.push(alphabet[state as usize % alphabet.len()]);
        }
        if let Ok(value) = normalize(&input) {
            assert!(value.canonical.is_ascii());
            assert_eq!(canonicalize(&value.canonical).unwrap(), value.canonical);
        }
    }
}
