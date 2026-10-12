# Rust API

### `normalize(email: &str) -> Result<EmailIdentity, Error>`

Return owned `domain` and `canonical` strings and a `Provider` enum. `&str` already guarantees well-formed UTF-8. Invalid syntax or length returns `Error::InvalidEmail`.

### `canonicalize(email: &str) -> Result<String, Error>`

Return only the canonical string with the same validation. No owned domain string is allocated.

### `normalize_into(email: &str, output: &mut [u8]) -> Result<EmailParts, Error>`

Write normalized domain bytes followed immediately by canonical address bytes. `EmailParts.domain_len` and `canonical_len` delimit them; there is no separator or NUL terminator. The buffer can be any length. `OUTPUT_CAPACITY` bytes always suffice for accepted input. A smaller buffer succeeds when the actual result fits.

`Error::BufferTooSmall` reports insufficient output capacity after input validation. Invalid input reports `Error::InvalidEmail`. On either error the entire output buffer is unchanged. Bytes beyond the returned lengths are untouched. The ordinary ASCII path without Punycode uses bounded stack storage and does not allocate; IDNA validation can allocate.

### `EmailIdentity`, `EmailParts`, and `Provider`

The structs have public fields. `Provider` variants are `Custom`, `Gmail`, `Microsoft`, `Icloud`, `Proton`, and `Yahoo`; `as_str()` returns their lowercase wire names. `EmailParts` contains the byte lengths and provider, not references into the buffer.

### `Error`, `MAX_INPUT_BYTES`, and `OUTPUT_CAPACITY`

`Error` implements `Display` and `std::error::Error`. Both constants are `512`. The first limits the raw input; the second bounds the sum of returned domain and canonical bytes, not each field separately.

## Strict address-only API

`normalize_strict(&str) -> Result<EmailIdentity, Error>`, `canonicalize_strict(&str) -> Result<String, Error>`, and `normalize_strict_into(&str, &mut [u8]) -> Result<EmailParts, Error>`. The caller-buffer function preserves the entire output on either error, just like `normalize_into`.

Use these functions for an address field supplied by an untrusted caller. They reject wrappers, `mailto:`, query/fragment suffixes, all whitespace, and characters from the explicit cleanup table rather than silently trimming or removing them. They return the same result types and errors as their non-strict counterparts. Supported Unicode domains still undergo native IDNA mapping; strict parsing does not freeze a Unicode profile or verify ownership.

Use `normalize` / `canonicalize` (with language-appropriate casing) only when intentionally importing a display name or `mailto:` value. Every API rejects C0/C1 controls and Unicode line/paragraph separators before cleanup. Display parsing accepts one balanced `<address>` wrapper, no trailing content, and no mailbox/list/header delimiters (`@<>,;:`) in the display name.

Never use canonical equality alone to authorize login, recovery, account linking, or merging. Verify each mailbox and keep the submitted address. Persist one authoritative canonical key across services; dependency or Unicode-profile changes require an explicit migration.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

In display parsing, outer non-control Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. Ordinary ASCII domains are checked directly; Unicode domains and ASCII `xn--` labels are validated through IDNA. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud removes `+tags` and folds `me.com` and `mac.com` to `icloud.com`. Proton removes `+tags` but preserves its three domains. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
