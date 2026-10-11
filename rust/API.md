# Rust API

### `normalize(email: &str) -> Result<EmailIdentity, Error>`

Return owned `domain` and `canonical` strings and a `Provider` enum. `&str` already guarantees well-formed UTF-8. Invalid syntax or length returns `Error::InvalidEmail`.

### `canonicalize(email: &str) -> Result<String, Error>`

Return only the canonical string with the same validation. No owned domain string is allocated.

### `normalize_into(email: &str, output: &mut [u8]) -> Result<EmailParts, Error>`

Write normalized domain bytes followed immediately by canonical address bytes. `EmailParts.domain_len` and `canonical_len` delimit them; there is no separator or NUL terminator. The buffer can be any length. `OUTPUT_CAPACITY` bytes always suffice for accepted input. A smaller buffer succeeds when the actual result fits.

`Error::BufferTooSmall` reports insufficient output capacity after input validation. Invalid input reports `Error::InvalidEmail`. On either error the entire output buffer is unchanged. Bytes beyond the returned lengths are untouched. The ASCII path uses bounded stack storage and does not allocate; Unicode-domain conversion can allocate.

### `EmailIdentity`, `EmailParts`, and `Provider`

The structs have public fields. `Provider` variants are `Custom`, `Gmail`, `Microsoft`, `Icloud`, `Proton`, and `Yahoo`; `as_str()` returns their lowercase wire names. `EmailParts` contains the byte lengths and provider, not references into the buffer.

### `Error`, `MAX_INPUT_BYTES`, and `OUTPUT_CAPACITY`

`Error` implements `Display` and `std::error::Error`. Both constants are `512`. The first limits the raw input; the second bounds the sum of returned domain and canonical bytes, not each field separately.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

Outer Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. An ASCII domain is checked directly; a supported Unicode domain is converted to ASCII. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud removes `+tags` and folds `me.com` and `mac.com` to `icloud.com`. Proton removes `+tags` but preserves its three domains. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
