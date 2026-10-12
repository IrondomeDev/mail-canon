# mail-canon for JavaScript

Normalize email addresses and derive a provider-aware matching key locally. No network access, service client, telemetry, or mailbox lookup.

## Requirements

Node.js 20+ for the test runner; ESM, CommonJS, and bundler-based browser applications.

## Install and use

Install from npm:

```sh
npm install mail-canon
```

This installs the declared `tr46` dependency.

```js
import {normalize, canonicalize, InvalidEmail} from 'mail-canon';

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

CommonJS uses `const {normalize} = require('mail-canon')`. TypeScript declarations cover both import styles. `normalize` is synchronous. Browser applications need a bundler that resolves CommonJS and `tr46`; the ESM wrapper is not a standalone browser script. No Node-specific API is used in the parser.


## Address fields and display imports

Use `normalizeStrict` / `canonicalizeStrict` for untrusted address fields. It rejects whitespace, display wrappers, `mailto:`, and explicit cleanup characters. Use the existing normalizer only for intentional display imports. Both paths reject controls and ambiguous wrappers; canonical keys still require mailbox verification before account linking.

## Validate and measure

Run from the `javascript/` directory of the complete repository; tests use both shared corpora in `../testdata/`.

```sh
npm install
npm test
npm run bench
npm pack --dry-run
```

See [API.md](API.md) for every public function, result field, error, and integration boundary. Keep the original address separately and verify mailbox ownership before linking accounts. Unicode-domain acceptance is not guaranteed to be identical across every runtime.

For local development, run `npm pack` from `javascript/` and install the resulting tarball in your application. Licensed under [MIT](LICENSE).
