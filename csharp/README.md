# mail-canon for C#

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

.NET 8+ with normal globalization support. Globalization-invariant deployments are not a certified Unicode-domain profile.

## Install and use

Reference this source project from your application:

```sh
dotnet add reference /path/to/mail-canon/csharp/src/MailCanon.csproj
```

```csharp
using IronDome.MailCanon;

var result = EmailNormalizer.Normalize("Alice.Example+news@GoogleMail.com");
Console.WriteLine(result.Domain);    // googlemail.com
Console.WriteLine(result.Canonical); // aliceexample@gmail.com
Console.WriteLine(result.Provider);  // gmail

if (EmailNormalizer.TryNormalize("not-an-address", out var identity))
{
    Console.WriteLine(identity.Canonical);
}
```

For local NuGet packaging, run `dotnet pack src/MailCanon.csproj -c Release -o artifacts` from `csharp/`. No package has been published by this source change.

The parser uses bounded stack spans for cleanup and ASCII validation. Returned strings are owned. Domain conversion uses the runtime's `IdnMapping`, which can differ between ICU and Windows NLS environments.


## Address fields and display imports

Use `NormalizeStrict` / `CanonicalizeStrict` (or `TryNormalizeStrict`) for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `csharp/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
dotnet run --project tests/MailCanon.Tests.csproj -c Release
dotnet pack src/MailCanon.csproj -c Release -o artifacts
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
