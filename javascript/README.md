# mail-canon for JavaScript

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

Node.js 20+ for the test runner; ESM, CommonJS, and bundler-based browser applications.

## Install and use

Create an npm tarball from this repository's `javascript/` directory:

```sh
npm install
npm pack
```

Install the generated `.tgz` in your application using `npm install /path/to/irondome-mail-canon-0.1.0.tgz`. This installs the declared `tr46` dependency.

```js
import {normalize, canonicalize, InvalidEmail} from '@irondome/mail-canon';

const result = normalize('Alice.Example+news@GoogleMail.com');
// {domain: 'googlemail.com', canonical: 'aliceexample@gmail.com', provider: 'gmail'}
const address = canonicalize('First.Last+news@example.com');
// first.last+news@example.com

try {
  normalize('not-an-address');
} catch (error) {
  if (!(error instanceof InvalidEmail)) throw error;
}
```

CommonJS uses `const {normalize} = require('@irondome/mail-canon')`. TypeScript declarations cover both import styles. `normalize` is synchronous. Browser applications need a bundler that resolves CommonJS and `tr46`; the ESM wrapper is not a standalone browser script. No Node-specific API is used in the parser.


## Validate and measure

Run from the `javascript/` directory of the complete repository; tests use `../testdata/vectors.tsv`.

```sh
npm install
npm test
npm run bench
npm pack --dry-run
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

This is unreleased source. Registry identifiers are proposed, and installation examples use local files rather than implying a published package exists. Licensed under [MIT](LICENSE).
