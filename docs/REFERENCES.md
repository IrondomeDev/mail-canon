# Official references

These references inform implementation choices and policy boundaries. A provider document is not a guarantee that every address rewritten by this package is deliverable or belongs to the same account. The behavior contract defines this package's policy.

## Providers

- Google: [Dots do not matter in Gmail addresses](https://support.google.com/mail/answer/7436150?hl=en). Consumer Gmail behavior differs from work and school custom domains.
- Microsoft: [Plus addressing in Exchange Online](https://learn.microsoft.com/en-us/exchange/recipients-in-exchange-online/plus-addressing-in-exchange-online). Routing and administrator configuration matter; this is not evidence that unrelated aliases can be inferred.
- Microsoft: [Add or remove an email alias in Outlook.com](https://support.microsoft.com/en-us/office/add-or-remove-an-email-alias-in-outlook-com-459b1989-356d-40fa-a689-8f285b13f1f2). Account aliases are configured relationships.
- Apple: [About your @icloud.com, @me.com, and @mac.com email addresses](https://support.apple.com/en-us/118230). Legacy-domain eligibility depends on account history.
- Proton: [Addresses and aliases](https://proton.me/support/addresses-and-aliases). Address types and aliases are distinct concepts; the package does not infer account configuration.

Yahoo rewriting is conservative: no suffix stripping or domain folding is attempted. No inference about an account's configured disposable addresses is made.

## Language and Unicode APIs

- Python: [Built-in string operations](https://docs.python.org/3/library/stdtypes.html#text-sequence-type-str), [dataclasses](https://docs.python.org/3/library/dataclasses.html), and the [idna maintainer documentation](https://github.com/kjd/idna).
- Rust: [`str`](https://doc.rust-lang.org/std/primitive.str.html), [`u8`](https://doc.rust-lang.org/std/primitive.u8.html), and [`idna::uts46::Uts46`](https://docs.rs/idna/1.1.0/idna/uts46/struct.Uts46.html).
- JavaScript: [ECMAScript string operations](https://tc39.es/ecma262/multipage/text-processing.html) and the [tr46 maintainer documentation](https://github.com/jsdom/tr46).
- Java: [`String`](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/String.html), [`Locale.ROOT`](https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/util/Locale.html#ROOT), and [ICU4J IDNA](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/text/IDNA.html).
- Go: [`unicode/utf8`](https://pkg.go.dev/unicode/utf8), [`strings`](https://pkg.go.dev/strings), and [`golang.org/x/net/idna`](https://pkg.go.dev/golang.org/x/net/idna).
- PHP: [`strtolower`](https://www.php.net/manual/en/function.strtolower.php), [`strtr`](https://www.php.net/manual/en/function.strtr.php), and [`idn_to_ascii`](https://www.php.net/manual/en/function.idn-to-ascii.php).
- C#: [`IdnMapping`](https://learn.microsoft.com/en-us/dotnet/api/system.globalization.idnmapping), [`ReadOnlySpan<T>`](https://learn.microsoft.com/en-us/dotnet/api/system.readonlyspan-1), and [.NET globalization and ICU](https://learn.microsoft.com/en-us/dotnet/core/extensions/globalization-icu).

IDNA is deliberately a dependency or runtime capability. A homegrown punycode encoder is not a substitute for normalization, mapping, validation, and bidirectional checks.


## Provider additions checked 2026-10-26

- Proton's [username rules](https://proton.me/support/change-username) document
  case and transparent `.`, `_`, `-`. Its [Key Transparency specification](https://proton.me/files/proton_keytransparency_whitepaper.pdf),
  section 3.2, defines stripping the first plus suffix before punctuation while
  retaining the domain. [Address documentation](https://proton.me/support/addresses-and-aliases)
  includes the legacy `protonmail.ch` domain. Domains remain distinct here.
- Fastmail's [published first-party domains](https://www.fastmail.help/hc/en-us/articles/6239365719055-What-domains-can-I-use-with-Fastmail)
  supply the static 130-domain allowlist. Its [plus/subdomain rules](https://www.fastmail.help/hc/en-us/articles/360060591053-Plus-addressing-and-subdomain-addressing)
  and [delivery order](https://www.fastmail.help/hc/en-us/articles/1500000278262-The-email-delivery-process)
  support bounded first-party handling. Arbitrary custom-domain routing stays out
  of scope because administrators select its behavior.
- Microsoft's [consumer plus-address documentation](https://support.microsoft.com/en-us/outlook/send-email-from-a-different-address-in-outlook-com)
  confirms the pre-existing Outlook family rule. Dots and domain names stay distinct.
- Apple's [legacy-domain history](https://support.apple.com/en-us/118230)
  explains why that existing matching policy is unsuitable as automatic proof of
  mailbox/account identity. This change does not broaden Apple matching.
