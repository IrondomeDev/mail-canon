use mail_canon::{canonicalize, normalize, normalize_into, Error, OUTPUT_CAPACITY};

fn unhex(hex: &str) -> String {
    let bytes: Vec<u8> = hex.as_bytes().chunks_exact(2).map(|pair| {
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
