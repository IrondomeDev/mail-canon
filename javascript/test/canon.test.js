import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import test from 'node:test';
import {InvalidEmail, normalize, canonicalize, normalizeStrict, canonicalizeStrict} from '../index.js';

for (const row of readFileSync(new URL('../../testdata/strict-vectors.tsv', import.meta.url), 'utf8').trimEnd().split(/\r?\n/)) {
  if (row.startsWith('#')) continue;
  const [name, encoded, domain, canonical, provider] = row.split('\t');
  test(`strict: ${name}`, () => {
    const raw = Buffer.from(encoded, 'hex').toString('utf8');
    if (domain === '-') {
      assert.throws(() => normalizeStrict(raw), InvalidEmail);
      assert.throws(() => canonicalizeStrict(raw), InvalidEmail);
    } else {
      assert.deepEqual(normalizeStrict(raw), {domain, canonical, provider});
      assert.equal(canonicalizeStrict(raw), canonical);
      assert.equal(canonicalizeStrict(canonical), canonical);
    }
  });
}

const rows = readFileSync(new URL('../../testdata/vectors.tsv', import.meta.url), 'utf8').trimEnd().split(/\r?\n/);
for (const row of rows) {
  if (row.startsWith('#')) continue;
  const [name, encoded, domain, canonical, provider] = row.split('\t');
  test(name, () => {
    const raw = Buffer.from(encoded, 'hex').toString('utf8');
    if (domain === '-') {
      assert.throws(() => normalize(raw), InvalidEmail);
      assert.throws(() => canonicalize(raw), InvalidEmail);
    } else {
      assert.deepEqual(normalize(raw), {domain, canonical, provider});
      assert.equal(canonicalize(raw), canonical);
      assert.equal(canonicalize(canonical), canonical);
    }
  });
}

test('non-string inputs are not coerced', () => {
  for (const raw of [null, undefined, true, 12, {}, [], new String('u@example.com')]) {
    for (const fn of [normalize, canonicalize, normalizeStrict, canonicalizeStrict]) assert.throws(() => fn(raw), TypeError);
  }
});

test('ill-formed UTF-16 is rejected before wrapper removal', () => {
  for (const raw of ['\ud800@example.com', 'name \udfff <u@example.com>']) {
    assert.throws(() => normalize(raw), InvalidEmail);
    assert.throws(() => normalizeStrict(raw), InvalidEmail);
  }
});

test('CommonJS and ESM use the same implementation', () => {
  const require = createRequire(import.meta.url);
  assert.equal(require('../index.cjs').normalize, normalize);
  assert.equal(require('../index.cjs').normalizeStrict, normalizeStrict);
  assert.equal(require('../index.cjs').canonicalizeStrict, canonicalizeStrict);
});

test('deterministic random inputs remain bounded and idempotent', () => {
  const alphabet = Array.from('abcXYZ019._+-@<> %\t\r\n\x00é。\u200b');
  let seed = 82751;
  const next = () => (seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0);
  for (let n = 0; n < 5000; n++) {
    let raw = '';
    const length = next() % 100;
    for (let i = 0; i < length; i++) raw += alphabet[next() % alphabet.length];
    let value;
    try { value = normalize(raw); }
    catch (error) { assert.ok(error instanceof InvalidEmail); continue; }
    assert.equal(canonicalize(value.canonical), value.canonical);
  }
});
