# PHP API

### `MailCanon::normalize(mixed $email): EmailIdentity`

Accept only a PHP string. Non-string input, including `null`, throws `TypeError` even from a caller without `strict_types`. Invalid UTF-8, unsupported syntax, and length violations throw `InvalidEmail`.

### `MailCanon::canonicalize(mixed $email): string`

Return only the canonical address with the same validation and errors. This helper calls `normalize` internally and does not promise fewer allocations.

### `EmailIdentity`

A final readonly class with public string properties `domain`, `canonical`, and `provider`. Its constructor only stores values; use `normalize` to validate addresses.

### `InvalidEmail`

An `InvalidArgumentException` subclass whose fixed message excludes the submitted address. Missing `intl` support is a `LogicException` on the Unicode-domain path, not an invalid-address result; install the required extension rather than catching that as user input failure.

### Runtime requirements

The manifest requires PHP 8.2 and `ext-intl`. ICU version differences can affect Unicode domains. The parser's ASCII lowercasing relies on PHP 8.2's locale-independent `strtolower` behavior.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

Outer Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. An ASCII domain is checked directly; a supported Unicode domain is converted to ASCII. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud removes `+tags` and folds `me.com` and `mac.com` to `icloud.com`. Proton removes `+tags` but preserves its three domains. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
