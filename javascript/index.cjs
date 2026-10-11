'use strict';

let idna;
const ASCII_SPACE = /^[\t\n\f\r ]+|[\t\n\f\r ]+$/g;
const UNICODE_SPACE = /^[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+|[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+$/g;
const NON_ASCII = /[^\x00-\x7f]/;
const CLEAN = /[\u200b\u200c\u200d\ufeff\u2060\uff0e\u3002\uff61\u2024\uff0b\ufe62\uff20]/g;
const MAPPINGS = new Map([
  ['\uff0e', '.'], ['\u3002', '.'], ['\uff61', '.'], ['\u2024', '.'],
  ['\uff0b', '+'], ['\ufe62', '+'], ['\uff20', '@'],
]);
const RULES = new Map([
  ['gmail.com', ['gmail', 'gmail.com']], ['googlemail.com', ['gmail', 'gmail.com']],
  ...['outlook.com', 'hotmail.com', 'live.com', 'msn.com'].map(d => [d, ['microsoft', d]]),
  ...['icloud.com', 'me.com', 'mac.com'].map(d => [d, ['icloud', 'icloud.com']]),
  ...['proton.me', 'protonmail.com', 'pm.me'].map(d => [d, ['proton', d]]),
  ...['yahoo.com', 'ymail.com', 'rocketmail.com'].map(d => [d, ['yahoo', d]]),
]);
const IDNA_OPTIONS = Object.freeze({
  checkBidi: true, checkHyphens: false, checkJoiners: true,
  transitionalProcessing: false, useSTD3ASCIIRules: true, verifyDNSLength: true,
});

class InvalidEmail extends Error {
  constructor() { super('invalid email'); this.name = 'InvalidEmail'; }
}

function byteLength(text) {
  let bytes = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i);
    if (c < 0x80) bytes++;
    else if (c < 0x800) bytes += 2;
    else if (c >= 0xd800 && c <= 0xdbff) {
      const next = text.charCodeAt(++i);
      if (!(next >= 0xdc00 && next <= 0xdfff)) throw new InvalidEmail();
      bytes += 4;
    } else {
      if (c >= 0xdc00 && c <= 0xdfff) throw new InvalidEmail();
      bytes += 3;
    }
    if (bytes > 512) throw new InvalidEmail();
  }
  return bytes;
}

function parse(raw, identity) {
  if (typeof raw !== 'string') throw new TypeError('email must be a string');
  if (!raw.length || raw.length > 512) throw new InvalidEmail();
  const unicode = NON_ASCII.test(raw);
  if (unicode) byteLength(raw);
  let value = raw.replace(UNICODE_SPACE, '');
  if (unicode) value = value.replace(CLEAN, c => MAPPINGS.get(c) || '');
  value = value.replace(ASCII_SPACE, '');
  if (value.slice(0, 7).toLowerCase() === 'mailto:') {
    value = value.slice(7).replace(ASCII_SPACE, '');
    const q = value.indexOf('?'), h = value.indexOf('#');
    value = value.slice(0, Math.min(q < 0 ? value.length : q, h < 0 ? value.length : h)).replace(ASCII_SPACE, '');
  }
  const left = value.indexOf('<'), right = value.lastIndexOf('>');
  if (left >= 0 && right >= 0) {
    if (left >= right) throw new InvalidEmail();
    value = value.slice(left + 1, right).replace(ASCII_SPACE, '');
  }
  const at = value.indexOf('@');
  if (at < 0 || value.indexOf('@', at + 1) >= 0) throw new InvalidEmail();
  let local = value.slice(0, at).replace(ASCII_SPACE, '');
  let domain = value.slice(at + 1).replace(ASCII_SPACE, '');
  if (!local.length || local.length > 64 || /[^A-Za-z0-9._%+\-]/.test(local) ||
      local[0] === '.' || local.endsWith('.') || local.includes('..') || !domain) throw new InvalidEmail();
  if (NON_ASCII.test(domain)) {
    idna ??= require('tr46');
    domain = idna.toASCII(domain, IDNA_OPTIONS);
    if (!domain) throw new InvalidEmail();
  }
  if (domain.endsWith('.')) domain = domain.slice(0, -1);
  if (!domain || domain.length > 253 || !domain.includes('.') || /[^A-Za-z0-9.\-]/.test(domain)) throw new InvalidEmail();
  domain = domain.toLowerCase();
  let start = 0;
  for (let i = 0; i <= domain.length; i++) {
    if (i !== domain.length && domain.charCodeAt(i) !== 46) continue;
    const length = i - start;
    if (!length || length > 63 || domain[start] === '-' || domain[i - 1] === '-') throw new InvalidEmail();
    start = i + 1;
  }
  local = local.toLowerCase();
  const rule = RULES.get(domain);
  const provider = rule ? rule[0] : 'custom';
  const mapped = rule ? rule[1] : domain;
  if (provider !== 'custom' && provider !== 'yahoo') {
    const plus = local.indexOf('+');
    if (plus >= 0) local = local.slice(0, plus);
  }
  if (provider === 'gmail') {
    if (/[^a-z0-9.]/.test(local)) throw new InvalidEmail();
    local = local.replace(/\./g, '');
  }
  if (!local || local.endsWith('.')) throw new InvalidEmail();
  const canonical = local + '@' + mapped;
  if (domain.length + canonical.length > 512) throw new InvalidEmail();
  return identity ? {domain, canonical, provider} : canonical;
}

/** Return normalized input domain, canonical address, and provider group. */
function normalize(email) { return parse(email, true); }
/** Return the canonical address, using the same validation as normalize. */
function canonicalize(email) { return parse(email, false); }

exports.InvalidEmail = InvalidEmail;
exports.normalize = normalize;
exports.canonicalize = canonicalize;
