# Java API

### `MailCanon.normalize(String email): MailCanon.Identity`

Return a record with `domain()`, `canonical()`, and `provider()` accessors. The strings are ASCII. Null throws `NullPointerException`. Unsupported syntax, malformed UTF-16, and length violations throw `MailCanon.InvalidEmailException`.

### `MailCanon.canonicalize(String email): String`

Return only the canonical address with the same validation and exceptions. This convenience method uses `normalize` internally; it does not promise fewer allocations than the record-returning API.

### `MailCanon.Identity`

An immutable public record. Its constructor stores fields without running the parser. Use `normalize` for validation. Provider values are the six lowercase group names documented below.

### `MailCanon.InvalidEmailException`

A nested public `IllegalArgumentException` subclass. The fixed message excludes the submitted address. An absent ICU4J dependency is an environment failure, not a rejected email.

### Concurrency

Methods use per-call parsing buffers and immutable configuration. No input or result is retained between calls. Do not change the JVM's global locale as part of an integration; the parser does not require it.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

Outer Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. An ASCII domain is checked directly; a supported Unicode domain is converted to ASCII. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud removes `+tags` and folds `me.com` and `mac.com` to `icloud.com`. Proton removes `+tags` but preserves its three domains. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
