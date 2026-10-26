# JavaScript API

### `normalize(email: string): EmailIdentity`

Return `{domain, canonical, provider}`. TypeScript exposes readonly fields; the returned JavaScript object is not frozen. Each call returns a new object. Invalid string input throws `InvalidEmail`; non-string values, including `null`, `undefined`, boxed strings, and byte arrays, throw `TypeError`.

### `canonicalize(email: string): string`

Return the canonical address using the same parser and errors, without constructing an identity object. Both exports are synchronous and perform no network access.

### `EmailIdentity` and `Provider`

Declaration-only TypeScript types. `domain` and `canonical` are strings. `provider` is the union `custom | gmail | microsoft | icloud | proton | fastmail | yahoo`.

### `InvalidEmail`

An exported `Error` subclass with `name === "InvalidEmail"`. Error messages exclude the submitted address. A missing `tr46` installation remains a module-loading error, not an invalid-address result.

### Module exports

ESM and CommonJS resolve to the same parser. The package exports only its root entry point, not internal helpers. IDNA conversion is loaded on the IDNA path (Unicode domains or ASCII Punycode labels). Bundlers can still include it in the application bundle.

## Strict address-only API

`normalizeStrict(email: string): EmailIdentity` and `canonicalizeStrict(email: string): string`, exported through ESM, CommonJS, and both TypeScript declarations.

Use these functions for an address field supplied by an untrusted caller. They reject wrappers, `mailto:`, query/fragment suffixes, all whitespace, and characters from the explicit cleanup table rather than silently trimming or removing them. They return the same result types and errors as their non-strict counterparts. Supported Unicode domains still undergo native IDNA mapping; strict parsing does not freeze a Unicode profile or verify ownership.

Use `normalize` / `canonicalize` (with language-appropriate casing) only when intentionally importing a display name or `mailto:` value. Every API rejects C0/C1 controls and Unicode line/paragraph separators before cleanup. Display parsing accepts one balanced `<address>` wrapper, no trailing content, and no mailbox/list/header delimiters (`@<>,;:`) in the display name.

Never use canonical equality alone to authorize login, recovery, account linking, or merging. Verify each mailbox and keep the submitted address. Persist one authoritative canonical key across services; dependency or Unicode-profile changes require an explicit migration.

## Shared behavior

Input must be a string containing at most 512 UTF-8 bytes before cleanup. The local part is restricted to 1 through 64 ASCII letters, digits, or `._%+-`, with no leading, trailing, or consecutive dots. Non-string or null handling follows the API details above. No string coercion is performed by dynamic APIs.

In display parsing, outer non-control Unicode whitespace is trimmed; five invisible formatting characters are removed; selected fullwidth punctuation is mapped. Limited `mailto:` and `Name <address>` wrappers are accepted. `mailto:` query/fragment suffixes are discarded, not decoded. Ordinary ASCII domains are checked directly; Unicode domains and ASCII `xn--` labels are validated through IDNA. One trailing root dot is removed. Domain labels are 1 through 63 bytes, the domain is at most 253 bytes, and `len(domain) + len(canonical)` is at most 512 ASCII bytes.

All local parts are lowercased. Gmail removes dots and `+tags` and folds `googlemail.com` to `gmail.com`. Microsoft consumer domains remove `+tags` but keep dots and domains. iCloud retains its existing `+tag` removal and `me.com`/`mac.com` folding policy. Proton removes the first `+suffix`, then removes dots, underscores, and hyphens; its `proton.me`, `protonmail.com`, `protonmail.ch`, and `pm.me` domains remain distinct. Fastmail removes `+suffix` on its exact first-party domains, preserving dots and domains. Its one-level `tag@alias.fastmail-domain` form maps to `alias@fastmail-domain`; the returned `domain` remains the normalized input domain. Fastmail's published exact domain set is maintained in each implementation; custom domains and MX inference are not used. Yahoo and custom domains preserve dots, plus tags, and hyphens. An empty or dot-ending base after alias removal is rejected.

Only exact provider domains match. Custom Google Workspace or Microsoft 365 domains do not inherit consumer rules. The returned `domain` is always the normalized input domain, not necessarily the domain after `@` in `canonical`.

Canonical values are lossy matching keys, not ownership evidence. Keep the submitted address and verify ownership before linking accounts. Custom-domain local-part casing can be significant, and Apple legacy-domain eligibility depends on account history. This API does not resolve arbitrary configured aliases or verify deliverability.

IDNA implementations and Unicode data differ across languages and runtimes. Common examples share test vectors; exhaustive Unicode acceptance parity is not promised. See the repository's [behavior contract](../docs/BEHAVIOR.md) for the exact cleanup table, provider domains, and interoperability limits.
