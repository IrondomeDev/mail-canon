# Behavior contract

The public operations normalize one supported address or reject it. All successful outputs are ASCII. A result contains `domain` (normalized input domain), `canonical` (matching address), and `provider` (`custom`, `gmail`, `microsoft`, `icloud`, `proton`, or `yahoo`). An identity result does not verify a person, mailbox, or provider account.

## Input and cleanup order

A string must be nonempty and at most **512 UTF-8 bytes before cleanup**. Malformed Unicode is rejected. Statically typed APIs accept their language's string type; dynamic APIs reject non-string values without coercion. Null behavior is documented per package.

The exact cleanup sequence is:

1. Trim outer Unicode White_Space: U+0009 through U+000D, U+0020, U+0085, U+00A0, U+1680, U+2000 through U+200A, U+2028, U+2029, U+202F, U+205F, and U+3000.
2. Remove U+200B, U+200C, U+200D, U+FEFF, and U+2060 anywhere. Map U+FF0E, U+3002, U+FF61, and U+2024 to `.`; U+FF0B and U+FE62 to `+`; U+FF20 to `@`.
3. Trim ASCII space, tab, line feed, form feed, and carriage return. This inner trim does not include vertical tab or NUL.
4. Remove an ASCII-case-insensitive `mailto:` prefix when present. Trim the remainder, discard everything from its first `?` or `#`, then trim again. Percent escapes are not decoded.
5. If both `<` and `>` exist, extract from the first `<` to the last `>`, reject reversed delimiters, and trim. Text outside the delimiters is ignored. This is not a general email-header parser.
6. Require exactly one `@`. Trim the two sides using the ASCII rule above.

Removing invisible characters is intentionally lossy. Preserve the original input for audit or display when that distinction matters.

## Local part

Before provider rewriting, the local part must contain 1 through 64 ASCII bytes and only letters, digits, `.`, `_`, `%`, `+`, or `-`. Leading dots, trailing dots, and consecutive dots are rejected. Letters are lowercased independently of the current locale.

The whole submitted local part is validated before dropping a provider tag. Invalid characters inside a tag do not become acceptable merely because the tag would be discarded. Gmail's retained base accepts only letters, digits, and dots; its dots are then removed.

After provider rewriting, an empty base or a base ending in `.` is rejected. For example, `a.+tag@outlook.com` is rejected rather than returning `a.@outlook.com`. `a.+tag@gmail.com` returns `a@gmail.com` because Gmail dots are removed. This post-transform validation keeps accepted canonical outputs valid on a second pass.

## Domain

An ASCII input domain uses a direct scan. A non-ASCII input domain is passed to the package's IDNA implementation. The intended mapping is nontransitional UTS #46 with STD3-style domain validation and bidirectional/contextual-joiner checks. See the interoperability limits below.

After conversion, one final root dot is removed. The domain is lowercased and must have at least two nonempty labels. Only ASCII letters, digits, and hyphens are accepted within labels. Each label is 1 through 63 bytes and cannot start or end with a hyphen. The full domain is at most 253 bytes.

ASCII `xn--` labels are checked for this DNS shape, not decoded and revalidated. An ASCII address with a malformed punycode-looking label can therefore pass shape validation. This API does not establish that the domain exists or is registered.

## Provider policy

| Provider | Exact input domains | Canonical domain | Remove `+suffix` | Remove dots |
| --- | --- | --- | --- | --- |
| `gmail` | `gmail.com`, `googlemail.com` | `gmail.com` | Yes | Yes |
| `microsoft` | `outlook.com`, `hotmail.com`, `live.com`, `msn.com` | Unchanged | Yes | No |
| `icloud` | `icloud.com`, `me.com`, `mac.com` | `icloud.com` | Yes | No |
| `proton` | `proton.me`, `protonmail.com`, `pm.me` | Unchanged | Yes | No |
| `yahoo` | `yahoo.com`, `ymail.com`, `rocketmail.com` | Unchanged | No | No |
| `custom` | All other accepted domains | Unchanged | No | No |

Only the first `+` is used when removing a suffix. Custom domains are not classified through MX records. Google Workspace, Microsoft 365, and other custom-domain hosting do not opt a domain into these consumer-domain rules. Explicitly configured aliases with unrelated local parts cannot be resolved from the text of an address.

Apple's legacy-domain relationship depends on account history. This package deliberately folds the listed domains as a matching policy; it cannot establish whether a particular user owns those legacy aliases. Microsoft and Proton domain names are not folded together. Yahoo hyphenated addresses are not folded to a guessed base address.

## Size and return guarantees

`domain` is the normalized input domain, even when the canonical domain changes. `canonical` is the rewritten local part, `@`, and the canonical domain.

**The sum of the ASCII byte lengths of `domain` and `canonical` must not exceed 512.** This is a storage bound, not an RFC mailbox-length claim. It can reject a long domain that otherwise meets the individual 253-byte limit. The input-length limit, local-part limit, domain limit, and combined-output limit all apply independently.

For accepted inputs in the supported IDNA profile, canonicalizing the canonical output produces the same canonical string. The whole identity object need not be identical: the second pass sees the mapped domain rather than the original alias domain.

Rust's `normalize_into` writes `domain` immediately followed by `canonical`, without separators or terminators. Returned byte lengths identify each slice. The output buffer is not modified on error.

## IDNA interoperability limits

Internationalized domain conversion is delegated rather than implemented with a custom Unicode table:

| Package | Domain converter |
| --- | --- |
| Python | `idna`, UTS #46 remapping plus IDNA2008 validation |
| Rust | `idna::uts46::Uts46` |
| JavaScript | `tr46` |
| Java | ICU4J `IDNA` |
| Go | `golang.org/x/net/idna` with an explicit lookup profile |
| PHP | `intl` / ICU `idn_to_ascii` |
| C# | `System.Globalization.IdnMapping` |

Python's library rejects some labels that a UTS #46-only implementation can map, including some symbol or context-sensitive labels. Its third/fourth-position hyphen checks also differ on the Unicode path. .NET behavior depends on the operating system and whether ICU or NLS supplies globalization. The Unicode data version can differ between installed dependencies and runtimes.

The ports consequently do **not** claim exhaustive equality of Unicode-domain acceptance. Shared ASCII behavior and common internationalized examples have explicit vectors; they are not a proof over all Unicode. Cross-language identity systems should persist one authoritative canonical value or restrict accepted domains to a tested profile, and rerun the full corpus after dependency upgrades. Do not silently recompute stored keys with an upgraded runtime.

## Deliberate exclusions

There is no SMTP or DNS verification, disposable-domain classification, ownership verification, account-alias lookup, free-provider discovery, or network request. Quoted local parts, comments, multiple recipients, IP literals, internationalized local parts, and other RFC-valid forms outside the stated grammar are rejected. Do not replace a full RFC parser or a verification flow with this package.
