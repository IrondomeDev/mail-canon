# mail-canon for Go

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

Go 1.25+ and `golang.org/x/net/idna`.

## Install and use

In an existing application module, use a local replacement before a release exists:

```sh
go mod edit -require=github.com/IrondomeDev/mail-canon/go@v0.0.0
go mod edit -replace=github.com/IrondomeDev/mail-canon/go=/path/to/mail-canon/go
go mod tidy
```

```go
package main

import (
    "fmt"
    mailcanon "github.com/IrondomeDev/mail-canon/go"
)

func main() {
    result, err := mailcanon.Normalize("Alice.Example+news@GoogleMail.com")
    if err != nil {
        return
    }
    fmt.Println(result.Domain)    // googlemail.com
    fmt.Println(result.Canonical) // aliceexample@gmail.com
    fmt.Println(result.Provider)  // gmail
}
```

The provider parser uses bounded buffers and an explicit domain-conversion profile. Public functions return owned strings, not slices into reused memory. The `go/` submodule is versioned separately with `go/vX.Y.Z` release tags.


## Address fields and display imports

Use `NormalizeStrict` / `CanonicalizeStrict` for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `go/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
go mod tidy
go test -race ./...
go test -run='^$' -fuzz=FuzzNormalize -fuzztime=10s
go test -run='^$' -bench=BenchmarkNormalize -benchmem
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
