# IronDome Mail Canon

[![Releases](https://img.shields.io/badge/releases-package_versions-F76808)](https://github.com/IrondomeDev/mail-canon/releases)
[![npm](https://img.shields.io/npm/v/mail-canon?color=F76808)](https://www.npmjs.com/package/mail-canon)
[![crates.io](https://img.shields.io/crates/v/mail-canon?color=F76808)](https://crates.io/crates/mail-canon)
[![Go Reference](https://img.shields.io/badge/Go-reference-F76808)](https://pkg.go.dev/github.com/IrondomeDev/mail-canon/go)
[![Packagist](https://img.shields.io/packagist/v/irondome/mail-canon?color=F76808)](https://packagist.org/packages/irondome/mail-canon)
[![License: MIT](https://img.shields.io/badge/license-MIT-F76808)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-behavior_contract-F76808)](docs/BEHAVIOR.md)
[![CI](https://github.com/IrondomeDev/mail-canon/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/IrondomeDev/mail-canon/actions/workflows/ci.yml)

**A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.**

Available in Python, Rust, JavaScript, Java, Go, PHP, and C#.

[Package guides](#packages) · [Behavior contract](docs/BEHAVIOR.md) · [Contributing](CONTRIBUTING.md) · [Release guide](RELEASING.md) · [IronDome](https://irondome.dev)

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
| [python](python/README.md) | `mail-canon` | `mail_canon.normalize`, `canonicalize` |
| [rust](rust/README.md) | `mail-canon` | `mail_canon::{normalize, canonicalize, normalize_into}` |
| [javascript](javascript/README.md) | `mail-canon` | ESM and CommonJS, TypeScript declarations |
| [java](java/README.md) | `dev.irondome:mail-canon` | `MailCanon.normalize`, `canonicalize` |
| [go](go/README.md) | `github.com/IrondomeDev/mail-canon/go` | `mailcanon.Normalize`, `Canonicalize` |
| [php](php/README.md) | `irondome/mail-canon` | `MailCanon::normalize`, `canonicalize` |
| [csharp](csharp/README.md) | `mail-canon` | `EmailNormalizer.Normalize`, `Canonicalize`, `TryNormalize` |

Package releases are independent:

| Language | Registry | Status |
| --- | --- | --- |
| JavaScript | [npm](https://www.npmjs.com/package/mail-canon) | Released: `0.1.0` |
| Rust | [crates.io](https://crates.io/crates/mail-canon) | Released: `0.1.0` |
| Go | [Go module](https://pkg.go.dev/github.com/IrondomeDev/mail-canon/go@v0.1.0) | Released: `v0.1.0` |
| Python | PyPI | Pending organization approval; install from source |
| PHP | [Packagist](https://packagist.org/packages/irondome/mail-canon) | Released: `0.1.0` |
| Java | [Maven Central](https://central.sonatype.com/artifact/dev.irondome/mail-canon) | `0.1.0` validated; registry publishing in progress |
| C# | [NuGet](https://www.nuget.org/packages/mail-canon) | `0.1.0` uploaded through trusted publishing; registry processing in progress |

See [GitHub releases](https://github.com/IrondomeDev/mail-canon/releases) for versioned release notes and the [release guide](RELEASING.md) for registry setup.

The unscoped npm package `mail-canon` is managed through the [irondome-dev organization](https://www.npmjs.com/org/irondome-dev)'s developers team.

## Quick start

Python is available from the checkout while PyPI approval is pending:

```sh
python -m pip install ./python
```

```python
from mail_canon import normalize_strict

result = normalize_strict("Alice.Example+news@GoogleMail.com")
print(result.canonical)  # aliceexample@gmail.com
```

For JavaScript:

```sh
npm install mail-canon
```

```javascript
import { normalizeStrict } from 'mail-canon';

const result = normalizeStrict('Alice.Example+news@GoogleMail.com');
console.log(result.canonical); // aliceexample@gmail.com
```

For Rust, add to your application's `Cargo.toml`:

```toml
[dependencies]
mail-canon = "0.1.0"
```

For Go, run from an existing application module:

```sh
go get github.com/IrondomeDev/mail-canon/go@v0.1.0
```

Use strict APIs for untrusted address fields. Display-name imports have a separate, more permissive parsing policy described below.

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

All seven test suites consume the shared [normalization](testdata/vectors.tsv) and [strict-address](testdata/strict-vectors.tsv) corpora. They cover provider rules, cleanup, malformed inputs, limits, and canonical-output idempotence. Each package README contains its test command. The [CI workflow](.github/workflows/ci.yml) checks all seven implementations across supported runtime versions, including Windows and Linux for C#, without silently skipping IDNA cases.

Input is bounded at 512 UTF-8 bytes before cleanup. Ordinary ASCII domains bypass IDNA; ASCII Punycode labels are validated through IDNA. Rust additionally exposes a caller-buffer path with no heap allocation for ordinary ASCII input without Punycode. Benchmarks are included for Python, JavaScript, and Go. They measure local execution, not a latency guarantee or a cross-language ranking.

## Development and release

Development lives in one repository so implementations share a behavior contract, regression fixtures, and review history. Packages can release independently; see the [release guide](RELEASING.md) for versioning, registry requirements, and the public-release checklist.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for cross-language changes and [official references](docs/REFERENCES.md) for the language and provider documentation used. CI validates packages; registry publication is a separate release step.

## License

MIT © 2026 Iron Dome. See [LICENSE](LICENSE). Each language package includes the same license.

Maintained by [IronDome](https://github.com/IrondomeDev).
