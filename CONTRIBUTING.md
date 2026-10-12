# Contributing

Start with the [behavior contract](docs/BEHAVIOR.md) and the README for the language you are changing. Use a focused pull request with a description of the observable change and the checks you ran. Replace real email addresses with synthetic examples in issues, fixtures, and logs.

Keep this project limited to local email normalization and canonicalization. No network checks, credentials, provider APIs, telemetry, or domain databases belong in these packages.

## Changing behavior

Add an accepted or rejected case to `testdata/vectors.tsv` (display import) or `testdata/strict-vectors.tsv` (bare addresses), update the behavior contract, and implement the rule in all seven packages. The fixture columns are name, UTF-8 input encoded as lowercase hex, expected normalized domain, expected canonical address, and expected provider. A `-` in each of the last three columns means rejection. Hex preserves empty strings, whitespace, and control characters across test runners. Test names must be unique.

Every accepted vector is also tested for canonical-output idempotence. Add explicit boundaries for new transformations. Keep errors deterministic and never include the submitted address in an exception message.

Run each language's documented test command from its package directory. Test dependencies belong in development/test scopes. Do not skip Unicode-domain tests when an IDNA dependency is missing: install the dependency or report that the complete suite was not executed.

## Performance

Measure changed code with representative plain ASCII, provider-alias, and Unicode-domain inputs. Include warmup and consume the result. Report the runtime, dependency versions, sample count, and whether setup or IDNA initialization is included. Do not compare languages based on unrelated benchmark harnesses or turn local timings into service-level promises.

Preserve the ASCII fast path. Rust's caller-buffer operation must remain allocation-free for ordinary ASCII input without Punycode and must not mutate the output on error. Do not introduce regexes with unbounded backtracking or remove input-size checks.

## Before the first release

Confirm the license and package namespaces. Run all CI jobs against the declared dependencies, including both Windows and Linux for C#. Review dependency advisories and record the resolved versions in release evidence. Commit the Go submodule's generated `go.sum` and check that `go mod tidy` leaves both module files unchanged. Never fabricate checksums. The Go dependency floor is `golang.org/x/net v0.57.0` (Go 1.25+); unlike v0.55.0, it rejects ASCII-only Punycode decoding with older Unicode tables too. Python requires `idna>=3.15,<4` for the audited security fixes.

Check wheels, npm tarballs, Rust packages, JARs, Composer source distributions, and NuGet packages for the expected public files. Java registry publication additionally needs the registry's required signing and metadata. Composer can use its documented local path repository before Packagist registration. The Go submodule's version tags use the `go/vX.Y.Z` convention when publishing versioned releases.

Version changes to the matching policy explicitly. Existing users should not have their saved matching keys silently rewritten. Release automation and registry credentials are intentionally outside this change.

See [RELEASING.md](RELEASING.md) for the monorepo release model, tag conventions, and the first-public-release checklist.
