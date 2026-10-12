# mail-canon for PHP

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

PHP 8.2+ with the `intl` extension and Composer.

## Install and use

Use a Composer path repository in your application before a registry release exists:

```json
{
  "repositories": [{"type": "path", "url": "/path/to/mail-canon/php", "options": {"versions": {"irondome/mail-canon": "0.1.0"}}}],
  "require": {"irondome/mail-canon": "0.1.0"}
}
```

The [PHP distribution repository](https://github.com/IrondomeDev/mail-canon-php) provides a root-level `composer.json` for Packagist. It is generated from this monorepo; submit changes to the development repository. Until the package is registered on Packagist, use the path repository above.

Run `composer install`, then:

```php
<?php
declare(strict_types=1);
require __DIR__ . '/vendor/autoload.php';

use IronDome\MailCanon\MailCanon;
use IronDome\MailCanon\InvalidEmail;

$result = MailCanon::normalize('Alice.Example+news@GoogleMail.com');
assert($result->domain === 'googlemail.com');
assert($result->canonical === 'aliceexample@gmail.com');
assert($result->provider === 'gmail');

try {
    MailCanon::canonicalize('not-an-address');
} catch (InvalidEmail $error) {
    // Ask the caller for a supported address.
}
```

The `intl` extension is required for the supported package, even though ASCII domains do not invoke it. The parser rejects non-string arguments explicitly, including when the caller uses PHP's coercive mode.


## Address fields and display imports

Use `normalizeStrict` / `canonicalizeStrict` for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the monorepo's `php/` directory or the distribution repository root. The distribution includes copies of both shared test corpora.

```sh
composer install
composer test
find src tests -name '*.php' -exec php -l {} \;
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

The Composer package name is `irondome/mail-canon`. Registry installation becomes available after Packagist registration. Licensed under [MIT](LICENSE).
