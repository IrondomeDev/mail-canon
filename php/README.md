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

A future Packagist release needs a repository or distribution rooted at `php/`; the monorepo root is not a Composer package. The path repository above is usable without publishing.

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


## Validate and measure

Run from the `php/` directory of the complete repository; tests use `../testdata/vectors.tsv`.

```sh
composer install
composer test
find src tests -name '*.php' -exec php -l {} \;
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
