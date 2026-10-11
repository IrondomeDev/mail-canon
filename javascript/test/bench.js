import {performance} from 'node:perf_hooks';
import {createRequire} from 'node:module';
import {normalize} from '../index.js';

const asciiOnly = process.argv.includes('--ascii');
const cases = [
  ['ascii', 'user@example.com'],
  ['gmail', 'User.Name+tag@GoogleMail.com'],
  ...asciiOnly ? [] : [['idn', 'User@bücher.example']],
];
const iterations = 100_000;
const samples = 5;
const results = {};
let checksum = 0;
for (const [name, value] of cases) {
  for (let i = 0; i < 10_000; i++) checksum += normalize(value).canonical.length;
  const timings = [];
  for (let sample = 0; sample < samples; sample++) {
    const start = performance.now();
    for (let i = 0; i < iterations; i++) checksum += normalize(value).canonical.length;
    timings.push((performance.now() - start) * 1e6 / iterations);
  }
  results[name] = {
    median_ns_per_call: Math.round([...timings].sort((a, b) => a - b)[2]),
    sample_ns_per_call: timings.map(Math.round),
  };
}
const require = createRequire(import.meta.url);
console.log(JSON.stringify({
  node: process.version,
  tr46: asciiOnly ? 'not loaded; ASCII-only run' : require('tr46/package.json').version,
  iterations_per_sample: iterations, samples, includes_startup: false, checksum, results,
}, null, 2));
