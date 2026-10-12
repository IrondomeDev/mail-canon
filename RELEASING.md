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

Public Packagist expects `composer.json` at the repository root; see its [submission guidance](https://packagist.org/about?type=application). The generated [PHP distribution mirror](https://github.com/IrondomeDev/mail-canon-php) has `php/` as its root and standard `vX.Y.Z` tags. Continue editing PHP in this monorepo. After committing and testing a release, export it with `python tools/export_php.py --output /path/to/empty/distribution`, review the snapshot, and commit it in the distribution repository. The export includes shared fixtures, standalone CI, and `SOURCE.json` identifying the originating commit. Tag the tested mirror commit with `vX.Y.Z` and the source commit with `php/vX.Y.Z`. Submit the mirror URL to Packagist, rather than the development repository URL.

## Registry access

The MIT license is included at the root and in every package. Source review precedes public releases; repository visibility and registry publishing are separate steps.

The package CI workflow builds and tests without publishing. The separate NuGet publishing workflow runs only when manually dispatched on `main`. Publish each package after its release commit passes CI:

- Rust: `cargo publish` with a crates.io account and publishing token configured using `cargo login`.
- JavaScript: `npm publish --access public` after `npm login`. The unscoped package name is `mail-canon`, managed through the `irondome-dev:developers` organization team with read/write access. The existing SDK remains under `@irondome-dev`. Complete any requested browser/2FA approval. Verify organization access with `npm access list packages irondome-dev:developers --json`; publishing still authenticates an individual maintainer.
- Go: push `go/vX.Y.Z` to the public repository, then resolve `github.com/IrondomeDev/mail-canon/go@vX.Y.Z` through the public Go proxy. No separate registry account is needed.
- Python: publication is on hold pending PyPI organization approval.
- PHP: submit the public PHP distribution mirror to [Packagist](https://packagist.org/about) using an authorized maintainer account. Composer is the package manager.
- Java: verify the `dev.irondome` namespace in [Maven Central](https://central.sonatype.org/register/namespace/) and configure a GPG signing key and Central publishing credentials under server ID `central` in a private Maven settings file. `mvn --batch-mode -Prelease -s /path/to/settings.xml deploy` builds binary, sources, and Javadoc JARs, signs them, and publishes through the Central Portal after validation. Supply the GPG passphrase via `MAVEN_GPG_PASSPHRASE` or a local agent; do not put it in the POM or command line. Normal `mvn verify` runs without publishing or signing credentials.
- C#: configure a [NuGet trusted publishing policy](https://learn.microsoft.com/en-us/nuget/nuget-org/trusted-publishing) owned by `irondome`, for GitHub owner `IrondomeDev`, repository `mail-canon`, workflow file `publish-nuget.yml`, and environment `nuget`. Allow new packages and versions, restricted to the exact package pattern `mail-canon`. Dispatch `.github/workflows/publish-nuget.yml` on `main` with the personal NuGet profile username that created the policy. Leave `publish` false for a validation-only run; set it true to publish after tests and packaging pass. The workflow exchanges GitHub OIDC credentials for a temporary NuGet API key; no stored registry key is required.

npm can briefly expose a `0.0.0-stage` placeholder while processing a new package, even after a successful ordinary publish. Wait and recheck the public version and `latest` tag before assuming another approval is required. For an explicitly staged release, use npm CLI 11.15+ to inspect it with `npm stage list mail-canon` and `npm stage view <stage-id>`, then approve the reviewed artifact with `npm stage approve <stage-id>` and complete 2FA. The account's Staged Packages tab offers the same approval flow. Announce only the verified library version. See [npm staged publishing](https://docs.npmjs.com/staged-publishing/).

Never commit registry tokens, API keys, or signing keys. Configure them in local credential stores or the publishing environment.

## First public release

1. Confirm ownership of every package namespace and the intended license attribution. Check that repository links, maintainer details, and package metadata are accurate.
2. Review Git history and tracked files for credentials, private data, and internal-only material before changing visibility. Make the repository public only after owner review.
3. Run the complete CI matrix on the exact commit to release, using the declared dependencies. Review dependency advisories and record resolved versions. Check that `go mod tidy` leaves `go.mod` and `go.sum` unchanged.
4. Inspect each built artifact for its README, license, public API, and expected contents. Verify installation in a clean consumer project. Confirm registry-specific requirements, including Maven signing and the PHP distribution mirror.
5. Set the package version, tag the reviewed commit with the appropriate prefix, and write release notes naming the package and any matching-policy changes. Publish only the intended package.
6. Verify registry installation and update documentation with the actual released version and installation command. Add registry version badges once releases exist.

Matching rules and IDNA dependencies can change stored account keys. Document behavior changes and migration requirements explicitly. Consumers should persist one authoritative normalization result across services and verify mailbox ownership independently of matching keys.

Future release automation should publish only from reviewed tags, scope credentials to the relevant registry, and keep publishing separate from pull-request validation.
