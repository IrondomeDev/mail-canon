# Mail Canon 0.2.0 publication verification — 2026-10-27

Version **0.2.0** is published for six language distributions. This is the
Proton/Fastmail provider-policy release, not a 2.0.0 release. Python publication
remains on hold at the owner's explicit request while PyPI organization approval
is pending. No Python release tag or PyPI upload was created.

| Language | Public package | Released source | Verification |
| --- | --- | --- | --- |
| JavaScript | npm `mail-canon@0.2.0` | `89c1c5b` | `latest=0.2.0`; registry tarball hash equals reviewed artifact; clean ESM/CommonJS consumer passed |
| Go | `github.com/IrondomeDev/mail-canon/go@v0.2.0` | `89c1c5b` | Public proxy resolved tagged commit and checksum; clean consumer passed |
| PHP | Packagist `irondome/mail-canon:v0.2.0` | `89c1c5b` | Mirror `da907c5`; PHP 8.2/8.4 CI passed; Packagist version/source ref verified |
| C# | NuGet `mail-canon:0.2.0` | `89c1c5b` | Trusted publishing succeeded; public version and downloaded archive metadata/DLL verified |
| Rust | crates.io `mail-canon:0.2.0` | `0603cff` | Strict lint/tests/package verification, OIDC publication, public checksum and clean consumer passed |
| Java | Central `dev.irondome:mail-canon:0.2.0` | `32844be` | Maven 3.9.16 validation/deploy succeeded; public binary/POM/sources/Javadoc and all four signatures verified |

All source tags use the documented language prefix in `IrondomeDev/mail-canon`.
The PHP mirror `IrondomeDev/mail-canon-php` also has its standard `v0.2.0` tag and
`SOURCE.json` identifies the originating monorepo commit. Public GitHub releases
were finalized only after registry verification.

## Source and CI qualification

The original provider candidate failed only the Go formatting check on both Go
matrix versions. Applying gofmt produced release source `89c1c5b`; all **13 CI
jobs passed**, including Windows/Linux C#, Python 3.10/3.13, Java 17/21, Node
20/22, PHP 8.2/8.4, Go 1.25/stable and Rust.

- [Source CI](https://github.com/IrondomeDev/mail-canon/actions/runs/40467219457)
- [PHP mirror CI](https://github.com/IrondomeDev/mail-canon-php/actions/runs/40468524952)
- [NuGet publish](https://github.com/IrondomeDev/mail-canon/actions/runs/40468657523)
- [Rust publish](https://github.com/IrondomeDev/mail-canon/actions/runs/40469587486)
- [Maven publish](https://github.com/IrondomeDev/mail-canon/actions/runs/40470449360)

The initial Rust validation-only publisher run failed a new Rust 1.99 Clippy lint
in the conformance decoder. `0603cff` replaces the test's constant-size
`chunks_exact` with `as_chunks` and checks for incomplete hex input. The library
implementation is unchanged. Strict validation and source CI passed afterward.

JavaScript's **961 tests** passed locally. Its npm archive SHA-256 is
`908f7bb9fe01b682665bb3c857751a0ec8e21be8fc4af2d99cb2400b842b25fb`,
identical to Gate's historical vendor archive. Consumer checks exercised Proton,
Fastmail subdomain aliases and preserved Outlook behavior through the public npm,
Go and crates.io installations. Existing shared-corpus qualification remains in
[the original report](VERIFICATION-2026-10-26.md); it was not rewritten.

## Java signing-key replacement

The previous private signing key was unavailable on the VM. The owner explicitly
authorized a replacement, generated with a passphrase and stored outside the
repository. Only its public key was committed and uploaded to the public
keyserver. Private signing material/passphrase and Central credentials are held
in encrypted Actions secrets; no secret values are included in release artifacts
or evidence. The Maven publisher checks the expected fingerprint before signing.

New fingerprint: `0CA8828CD11C009694783F29ACFB5AAA1CEE5253`.
The original public key remains available for prior Java signatures. All four
0.2.0 artifact signatures were independently verified against the new key. See
[signing-key provenance](keys/README.md).

## Registry evidence and downstream Gate

The sanitized `evidence/registry-*-0.2.0-2026-10-27.json` records contain public
registry coordinates, checksums, source references and workflow links, without
credentials or author-local filesystem paths.

After verifying npm publication, Gate was pinned to the exact registry version
`mail-canon@0.2.0`. Archive integrity and the remaining dependency lock entries
were unchanged; Gate's package consumer check passed. Gate source was pushed to
`IrondomeDev/GATE` at `93a349c`. **Gate itself was not published.**

This release changes matching keys. Persist verified original addresses, audit
collisions, and use the consuming application's migration/ownership workflow.
Canonical equality alone does not authorize account linking or mailbox control.
