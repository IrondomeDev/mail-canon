# mail-canon for Java

A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.

No network access, telemetry, or mailbox lookup.

## Requirements

Java 17+ and Maven; ICU4J supplies nontransitional domain conversion.

## Install and use


Add the [Maven Central package](https://central.sonatype.com/artifact/dev.irondome/mail-canon) to your application:

```xml
<dependency>
  <groupId>dev.irondome</groupId>
  <artifactId>mail-canon</artifactId>
  <version>0.1.1</version>
</dependency>
```

```java
import dev.irondome.mailcanon.MailCanon;

var result = MailCanon.normalize("Alice.Example+news@GoogleMail.com");
assert result.domain().equals("googlemail.com");
assert result.canonical().equals("aliceexample@gmail.com");
assert result.provider().equals("gmail");

try {
    MailCanon.canonicalize("not-an-address");
} catch (MailCanon.InvalidEmailException error) {
    // Ask the caller for a supported address.
}
```

Normalization uses locale-independent ASCII rules. IDNA initialization is deferred to Unicode domains and ASCII Punycode labels. The result is a Java record; no service client or dependency injection container is required.


## Address fields and display imports

Use `normalizeStrict` / `canonicalizeStrict` for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `java/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
mvn --batch-mode test
mvn --batch-mode package
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

Licensed under [MIT](LICENSE).
