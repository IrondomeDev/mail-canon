# Releasing Mail Canon

## Repository model

Keep development in `IrondomeDev/mail-canon`. Seven language implementations share one behavior contract, test corpus, and review process. A behavior change should be reviewed together across languages, even when packages ship on different schedules.

Each package owns its manifest version and release artifact. Use language-prefixed Git tags so a release of one package does not imply a release of the others:

| Package | Tag convention | Distribution |
| --- | --- | --- |
| Python | `python/vX.Y.Z` | PyPI wheel and source archive |
| Rust | `rust/vX.Y.Z` | crates.io package |
| JavaScript | `javascript/vX.Y.Z` | npm tarball |
| Java | `java/vX.Y.Z` | Maven artifacts |
| Go | `go/vX.Y.Z` | Go module from the `go/` directory |
| C# | `csharp/vX.Y.Z` | NuGet package |
| PHP | `php/vX.Y.Z` in the development repository | Composer package from a distribution repository |

Public Packagist expects `composer.json` at the repository root; see its [submission guidance](https://packagist.org/about?type=application). Before a Packagist release, create a generated distribution mirror with `php/` as its root and standard `vX.Y.Z` tags. Continue editing PHP in this monorepo. The mirror should contain generated releases rather than a separate development history to maintain manually. Local Composer path installation remains available before publication.

## Private review

Keep this repository private until the owner finishes reviewing the source, documentation, package identifiers, and license. The MIT license is included at the root and in every package. Adding it does not change GitHub repository visibility.

No packages have been published from this source tree. Do not treat proposed registry names as reserved namespaces. Existing CI only builds and tests; it does not publish artifacts.

## First public release

1. Confirm ownership of every package namespace and the intended license attribution. Check that repository links, maintainer details, and package metadata are accurate.
2. Review Git history and tracked files for credentials, private data, and internal-only material before changing visibility. Make the repository public only after owner review.
3. Run the complete CI matrix on the exact commit to release, using the declared dependencies. Review dependency advisories and record resolved versions. Check that `go mod tidy` leaves `go.mod` and `go.sum` unchanged.
4. Inspect each built artifact for its README, license, public API, and expected contents. Verify installation in a clean consumer project. Confirm registry-specific requirements, including Maven signing and the PHP distribution mirror.
5. Set the package version, tag the reviewed commit with the appropriate prefix, and write release notes naming the package and any matching-policy changes. Publish only the intended package.
6. Verify registry installation and update documentation with the actual released version and installation command. Add registry version badges once releases exist.

Matching rules and IDNA dependencies can change stored account keys. Document behavior changes and migration requirements explicitly. Consumers should persist one authoritative normalization result across services and verify mailbox ownership independently of matching keys.

Future release automation should publish only from reviewed tags, scope credentials to the relevant registry, and keep publishing separate from pull-request validation.
