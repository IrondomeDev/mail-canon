# mail-canon

Small email normalization packages for Python, Rust, JavaScript, Java, Go, PHP, and C#.

Normalize an address and derive a provider-aware matching key. No HTTP calls, DNS queries, telemetry, or mailbox lookups. Each implementation is independent; there is no native service or shared runtime to deploy.

```text
Input:      Alice.Example+news@GoogleMail.com
Domain:     googlemail.com
Canonical:  aliceexample@gmail.com
Provider:   gmail
```

`normalize` returns the normalized input domain, canonical address, and provider group. `canonicalize` returns only the canonical address. Names follow each language's conventions.

## Packages

| Folder | Package name | Interface |
| --- | --- | --- |
| [python](python/README.md) | `irondome-mail-canon` | `mail_canon.normalize`, `canonicalize` |
| [rust](rust/README.md) | `irondome-mail-canon` | `mail_canon::{normalize, canonicalize, normalize_into}` |
| [javascript](javascript/README.md) | `@irondome/mail-canon` | ESM and CommonJS, TypeScript declarations |
| [java](java/README.md) | `dev.irondome:mail-canon` | `MailCanon.normalize`, `canonicalize` |
| [go](go/README.md) | `github.com/IrondomeDev/mail-canon/go` | `mailcanon.Normalize`, `Canonicalize` |
| [php](php/README.md) | `irondome/mail-canon` | `MailCanon::normalize`, `canonicalize` |
| [csharp](csharp/README.md) | `IronDome.MailCanon` | `EmailNormalizer.Normalize`, `Canonicalize`, `TryNormalize` |

This is an unreleased 0.1.0 source tree. Package names are proposed identifiers, not claims that registry releases or namespace ownership have been established. Each README shows local installation.

## Strict input validation

For untrusted address fields, use the strict address-only API:

| Language | Identity / canonical key |
| --- | --- |
| Python | `normalize_strict` / `canonicalize_strict` |
| Rust | `normalize_strict` / `canonicalize_strict` / `normalize_strict_into` |
| JavaScript, Java, PHP | `normalizeStrict` / `canonicalizeStrict` |
| Go | `NormalizeStrict` / `CanonicalizeStrict` |
| C# | `NormalizeStrict` / `CanonicalizeStrict` / `TryNormalizeStrict` |

Strict parsing rejects wrappers, `mailto:`, whitespace, query/fragment suffixes, and explicit cleanup characters. Unicode domains still use native IDNA mapping. Existing `normalize` / `canonicalize` APIs remain available for deliberate display imports; all APIs reject controls, extra recipients in wrappers, and trailing wrapper content. Pass the returned address to downstream code rather than treating an accepted import as proof that the original string is safe.

## Matching policy

All accepted local parts and domains are lowercased. Provider rules apply only to the exact domains listed here, not to their subdomains or custom domains hosted by those providers.

| Group | Input domains | Canonical domain | Local-part transformation |
| --- | --- | --- | --- |
| Gmail | `gmail.com`, `googlemail.com` | `gmail.com` | Remove `+tag` and dots |
| Microsoft | `outlook.com`, `hotmail.com`, `live.com`, `msn.com` | Keep input domain | Remove `+tag`; keep dots |
| iCloud | `icloud.com`, `me.com`, `mac.com` | `icloud.com` | Remove `+tag`; keep dots |
| Proton | `proton.me`, `protonmail.com`, `pm.me` | Keep input domain | Remove `+tag`; keep dots |
| Yahoo | `yahoo.com`, `ymail.com`, `rocketmail.com` | Keep input domain | Keep dots, plus tags, and hyphens |
| Custom | Any other accepted domain | Keep input domain | Keep dots, plus tags, and hyphens |

For example, `First.Last+news@example.com` becomes `first.last+news@example.com`, not `firstlast@example.com`. The `domain` field retains `googlemail.com` or `me.com` even when the canonical address uses a different domain.

## Integration boundaries

Canonicalization is a lossy matching policy, not proof that two addresses belong to one person. Never authorize login, recovery, account linking, or merging solely from canonical equality. Keep the submitted address separately and verify ownership of every mailbox involved. Lowercasing a custom-domain local part and folding Apple legacy domains can conflate addresses under some configurations. A provider group is a rule label, not a verified hosting or deliverability result.

The parser deliberately supports a restricted ASCII local part, not every RFC mailbox syntax. It accepts limited `mailto:` and display-name wrappers, performs bounded Unicode cleanup, and converts supported internationalized domains to ASCII. It does not decode URL escapes or support internationalized local parts.

Native IDNA libraries differ in Unicode versions and acceptance rules. The shared corpus tests the documented examples; exhaustive Unicode acceptance parity across runtimes is not promised. Persist one authoritative normalization result across services; do not independently derive identity keys in different languages. Treat dependency/profile changes as explicit key migrations. See [the behavior contract](docs/BEHAVIOR.md).

## Tests and performance

All seven test suites consume the shared [normalization](testdata/vectors.tsv) and [strict-address](testdata/strict-vectors.tsv) corpora. They cover provider rules, cleanup, malformed inputs, limits, and canonical-output idempotence. Each package README contains its test command. The proposed CI workflow runs complete suites without silently skipping IDNA cases.

Input is bounded at 512 UTF-8 bytes before cleanup. Ordinary ASCII domains bypass IDNA; ASCII Punycode labels are validated through IDNA. Rust additionally exposes a caller-buffer path with no heap allocation for ordinary ASCII input without Punycode. Benchmarks are included for Python, JavaScript, and Go. They measure local execution, not a latency guarantee or a cross-language ranking.

## Development and release

Read [CONTRIBUTING.md](CONTRIBUTING.md) for cross-language changes and [official references](docs/REFERENCES.md) for the language and provider documentation used. No workflow publishes packages, uses application secrets, or changes repository settings. Registry names, dependency resolution, and the proposed MIT license must be reviewed before a release.
