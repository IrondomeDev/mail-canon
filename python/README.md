# mail-canon for Python

A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.

No network access, telemetry, or mailbox lookup.

## Requirements

Python 3.10+ and `idna>=3.15,<4`.

## Install and use

From this repository's `python/` directory:

```sh
python -m pip install .
```

```python
from mail_canon import InvalidEmail, canonicalize, normalize

result = normalize("Alice.Example+news@GoogleMail.com")
assert result.domain == "googlemail.com"
assert result.canonical == "aliceexample@gmail.com"
assert result.provider == "gmail"
assert canonicalize("First.Last+news@example.com") == "first.last+news@example.com"

try:
    normalize("not-an-address")
except InvalidEmail:
    pass  # Ask the caller for a supported address.
```

Provider examples: `normalize("U_s-e.r+tag@protonmail.ch").canonical` is
`"user@protonmail.ch"`; `normalize("news@alice.fastmail.com")` returns
`domain="alice.fastmail.com"`, `canonical="alice@fastmail.com"`, and
`provider="fastmail"`. Fastmail matching uses a static exact first-party domain
list and does not infer custom-domain mail routing.

`EmailIdentity` is an immutable, slotted dataclass. Type annotations and `py.typed` are included. The Unicode IDNA library is imported only when a Unicode domain or ASCII Punycode label needs validation.


## Address fields and display imports

Use `normalize_strict` / `canonicalize_strict` for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `python/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
python -m pip install .
python -m unittest discover -s tests -v
python tests/bench.py
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
