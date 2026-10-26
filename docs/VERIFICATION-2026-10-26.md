# Provider policy qualification, 2026-10-26

Candidate 0.2.0, not published. All seven local Linux language implementations
pass the shared corpus of 550 display and 407 strict vectors. The corpus includes
all 130 official Fastmail domains and Proton punctuation/protonmail.ch rules.
Existing Microsoft semantics are retained.

- JavaScript: 961 tests pass.
- Python: four unittest methods pass using the declared dependency floor, resolved
  to idna 3.20 in an isolated venv; pip check passes.
- Go: race tests and vet pass.
- PHP 8.5.4 with intl: shared vectors, strict and malformed UTF-8 tests and lint pass.
- Rust 1.97.1: five conformance tests and doc tests pass; Clippy has no warnings.
- Java 25.0.4.1 / Maven 3.10.0: 960 tests pass; 11 existing Javadoc warnings remain.
- .NET SDK 8.0.418: display/strict/null/UTF-16 checks pass; local NuGet pack succeeds.

[Raw qualification logs](../evidence/provider-policy-tests-2026-10-26.txt).
Windows and other CI matrices were not run locally. No registry publication or
Git push was performed. Before upgrading persisted canonical keys, audit collisions
and migrate them under the consuming application's ownership policy.
