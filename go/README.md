# mail-canon for Go

A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.

No network access, telemetry, or mailbox lookup.

## Requirements

Go 1.25+ and `golang.org/x/net/idna`.

## Install and use

In an existing application module:

```sh
go get github.com/IrondomeDev/mail-canon/go@v0.1.1
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

Provider examples: `U_s-e.r+tag@protonmail.ch` canonicalizes to
`user@protonmail.ch`; `news@alice.fastmail.com` has input domain
`alice.fastmail.com` and canonical address `alice@fastmail.com`. Fastmail
matching uses a static first-party domain list and does not infer custom-domain
mail routing.

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

For local development, use `go mod edit -replace=github.com/IrondomeDev/mail-canon/go=/path/to/mail-canon/go`. Licensed under [MIT](LICENSE).
