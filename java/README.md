# mail-canon for Java

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

Java 17+ and Maven; ICU4J supplies nontransitional domain conversion.

## Install and use

From this repository's `java/` directory:

```sh
mvn --batch-mode clean install
```

This installs the unreleased artifact in your local Maven repository. Add it to your application:

```xml
<dependency>
  <groupId>dev.irondome</groupId>
  <artifactId>mail-canon</artifactId>
  <version>0.1.0</version>
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

Normalization uses locale-independent ASCII rules. IDNA initialization is deferred to Unicode-domain conversion. The result is a Java record; no service client or dependency injection container is required.


## Validate and measure

Run from the `java/` directory of the complete repository; tests use `../testdata/vectors.tsv`.

```sh
mvn --batch-mode test
mvn --batch-mode package
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
