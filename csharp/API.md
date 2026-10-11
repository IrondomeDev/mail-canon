# C# API

### `EmailNormalizer.Normalize(string email): EmailIdentity`

Return a readonly record struct with `Domain`, `Canonical`, and `Provider` properties. Null throws `ArgumentNullException`. Malformed UTF-16, unsupported syntax, and length violations throw `InvalidEmailException`.

### `EmailNormalizer.Canonicalize(string email): string`

Return the canonical address with the same validation and exceptions. It delegates to `Normalize`; it does not promise to avoid allocating the normalized domain.

### `EmailNormalizer.TryNormalize(string? email, out EmailIdentity identity): bool`

Return `true` and an identity for accepted input. Return `false` and `default(EmailIdentity)` for null or rejected input. The default struct's string fields are null; do not read them as a successful result. Runtime or configuration failures other than rejected input are not deliberately hidden.

### `EmailIdentity`

Readonly record struct containing the three result strings. Manual construction does not validate them. Provider values are the six lowercase names below.

### `InvalidEmailException`

A `FormatException` subclass. The fixed message excludes the submitted address. Public functions do not send the address anywhere or retain it.

### Runtime profile

`IdnMapping` is configured with STD3 rules and disallows unassigned code points. Results for edge-case Unicode labels can depend on ICU, NLS, OS, and runtime versions. Test the application's deployment profile before persisting keys or sharing them with another language implementation.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

Outer Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. An ASCII domain is checked directly; a supported Unicode domain is converted to ASCII. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud removes `+tags` and folds `me.com` and `mac.com` to `icloud.com`. Proton removes `+tags` but preserves its three domains. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
