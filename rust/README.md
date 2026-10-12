# mail-canon for Rust

A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.

No network access, telemetry, or mailbox lookup.

## Requirements

A current stable Rust toolchain and the `idna` crate. A minimum supported Rust version has not been certified.

## Install and use

Add the package to your application's `Cargo.toml`:

```toml
[dependencies]
mail-canon = "0.1.1"
```

```rust
use mail_canon::{canonicalize, normalize};

fn main() -> Result<(), mail_canon::Error> {
    let result = normalize("Alice.Example+news@GoogleMail.com")?;
    assert_eq!(result.domain, "googlemail.com");
    assert_eq!(result.canonical, "aliceexample@gmail.com");
    assert_eq!(result.provider.as_str(), "gmail");
    assert_eq!(canonicalize("First.Last+news@example.com")?,
               "first.last+news@example.com");
    Ok(())
}
```

For a reusable output buffer:

```rust
use mail_canon::{normalize_into, OUTPUT_CAPACITY};

fn main() -> Result<(), mail_canon::Error> {
    let mut output = [0u8; OUTPUT_CAPACITY];
    let parts = normalize_into("A.B+tag@gmail.com", &mut output)?;
    let domain = &output[..parts.domain_len];
    let canonical = &output[parts.domain_len..parts.domain_len + parts.canonical_len];
    assert_eq!(domain, b"gmail.com");
    assert_eq!(canonical, b"ab@gmail.com");
    Ok(())
}
```

The caller-buffer operation allocates no heap memory for ordinary ASCII input without Punycode. The owned helpers allocate their returned strings. IDNA processing may allocate. All code is safe Rust.


## Address fields and display imports

Use `normalize_strict` / `canonicalize_strict` (or `normalize_strict_into`) for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `rust/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
cargo test
cargo clippy --all-targets
cargo package --list
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

For local development, use `mail-canon = { path = "/path/to/mail-canon/rust" }`. Licensed under [MIT](LICENSE).
