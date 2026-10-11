# mail-canon for Rust

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

A current stable Rust toolchain and the `idna` crate. A minimum supported Rust version has not been certified.

## Install and use

Add a path dependency to your application's `Cargo.toml`:

```toml
[dependencies]
mail-canon = { package = "irondome-mail-canon", path = "/path/to/mail-canon/rust" }
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

The caller-buffer operation allocates no heap memory for ASCII input. The owned helpers allocate their returned strings. IDNA processing may allocate. All code is safe Rust.


## Validate and measure

Run from the `rust/` directory of the complete repository; tests use `../testdata/vectors.tsv`.

```sh
cargo test
cargo clippy --all-targets
cargo package --list
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
