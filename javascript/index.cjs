'use strict';

let idna;
const ASCII_SPACE = /^[\t\n\f\r ]+|[\t\n\f\r ]+$/g;
const UNICODE_SPACE = /^[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+|[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+$/g;
const NON_ASCII = /[^\x00-\x7f]/;
const PROTON_SEPARATORS = /[._-]/g;
const CLEAN = /[\u200b\u200c\u200d\ufeff\u2060\uff0e\u3002\uff61\u2024\uff0b\ufe62\uff20]/g;
const MAPPINGS = new Map([
  ['\uff0e', '.'], ['\u3002', '.'], ['\uff61', '.'], ['\u2024', '.'],
  ['\uff0b', '+'], ['\ufe62', '+'], ['\uff20', '@'],
]);
const RULES = new Map([
  ['gmail.com', ['gmail', 'gmail.com']], ['googlemail.com', ['gmail', 'gmail.com']],
  ...['outlook.com', 'hotmail.com', 'live.com', 'msn.com'].map(d => [d, ['microsoft', d]]),
  ...['icloud.com', 'me.com', 'mac.com'].map(d => [d, ['icloud', 'icloud.com']]),
  ...['proton.me', 'protonmail.com', 'protonmail.ch', 'pm.me'].map(d => [d, ['proton', d]]),
  ...`123mail.org 150mail.com 150ml.com 16mail.com 2-mail.com 4email.net 50mail.com airpost.net aliencamel.com allmail.net bestmail.us cluemail.com elitemail.org emailcorner.net emailengine.net emailengine.org emailgroups.net emailplus.org emailuser.net eml.cc f-m.fm fast-email.com fast-mail.org fastem.com fastemail.us fastemailer.com fastest.cc fastimap.com fastmail.ca fastmail.cn fastmail.co.uk fastmail.com fastmail.com.au fastmail.de fastmail.es fastmail.fm fastmail.fr fastmail.im fastmail.in fastmail.jp fastmail.mx fastmail.net fastmail.nl fastmail.org fastmail.se fastmail.to fastmail.tw fastmail.uk fastmail.us fastmailbox.net fastmessaging.com fea.st fmail.co.uk fmailbox.com fmgirl.com fmguy.com foobox.com foobox.net ftml.net h-mail.us hailmail.net imap-mail.com imap.cc imapmail.org immerbox.com immermail.com inoutbox.com internet-e-mail.com internet-mail.org internetemails.net internetmailing.net jetemail.net justemail.net letterboxes.org lifetimeaddress.com mail-central.com mail-page.com mailas.com mailbolt.com mailc.net mailcan.com mailforce.net mailhaven.com mailingaddress.org mailite.com mailmight.com mailnew.com mailsent.net mailservice.ms mailup.net mailworks.org mailzone.com ml1.net mm.st myfastmail.com mymacmail.com nospammail.net onepost.net ownmail.net penguinmail.com permanentmail.com petml.com pobox.com postinbox.com postpro.net proinbox.com promessage.com realemail.net reallyfast.biz reallyfast.info rushpost.com sent.as sent.at sent.com siemprebox.com siempremail.com speedpost.net speedymail.org ssl-mail.com swift-mail.com the-fastest.net the-quickest.com theinternetemail.com veribox.net veryfast.biz veryspeedy.net warpmail.net xsmail.com yepmail.net your-mail.com`.split(/\s+/).map(d => [d, ['fastmail', d]]),
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

function parse(raw, identity, strict = false) {
  if (typeof raw !== 'string') throw new TypeError('email must be a string');
  if (!raw.length || raw.length > 512) throw new InvalidEmail();
  if (/[\x00-\x1f\x7f-\x9f\u2028\u2029]/.test(raw)) throw new InvalidEmail();
  if (strict && /[\u0020\u00a0\u1680\u2000-\u200d\u2024\u202f\u205f\u2060\u3000\u3002\ufeff\ufe62\uff0b\uff0e\uff20\uff61<>:?#]/.test(raw)) throw new InvalidEmail();
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
  if (left >= 0 || right >= 0) {
    if (left < 0 || right <= left || right !== value.length - 1 ||
        /[@<>,;:]/.test(value.slice(0, left)) || /[<>]/.test(value.slice(left + 1, right))) throw new InvalidEmail();
    value = value.slice(left + 1, right).replace(ASCII_SPACE, '');
  }
  const at = value.indexOf('@');
  if (at < 0 || value.indexOf('@', at + 1) >= 0) throw new InvalidEmail();
  let local = value.slice(0, at).replace(ASCII_SPACE, '');
  let domain = value.slice(at + 1).replace(ASCII_SPACE, '');
  if (!local.length || local.length > 64 || /[^A-Za-z0-9._%+\-]/.test(local) ||
      local[0] === '.' || local.endsWith('.') || local.includes('..') || !domain) throw new InvalidEmail();
  if (domain.endsWith('.')) domain = domain.slice(0, -1);
  if (NON_ASCII.test(domain) || /(?:^|\.)xn--/i.test(domain)) {
    idna ??= require('tr46');
    domain = idna.toASCII(domain, IDNA_OPTIONS);
    if (!domain) throw new InvalidEmail();
  }
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
  let rule = RULES.get(domain);
  let fastmailSubdomain = '';
  if (!rule) {
    const dot = domain.indexOf('.');
    if (dot > 0 && domain.indexOf('.', dot + 1) >= 0) {
      const rootRule = RULES.get(domain.slice(dot + 1));
      if (rootRule?.[0] === 'fastmail') {
        rule = rootRule;
        fastmailSubdomain = domain.slice(0, dot);
      }
    }
  }
  const provider = rule ? rule[0] : 'custom';
  const mapped = rule ? rule[1] : domain;
  if (provider === 'fastmail' && fastmailSubdomain) {
    // Fastmail's one-level recipient@alias.domain form routes to the alias.
    local = fastmailSubdomain;
  } else if (provider !== 'custom' && provider !== 'yahoo') {
    const plus = local.indexOf('+');
    if (plus >= 0) local = local.slice(0, plus);
  }
  if (provider === 'proton') local = local.replace(PROTON_SEPARATORS, '');
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
/** Normalize one bare address without wrappers, whitespace or cleanup. */
function normalizeStrict(email) { return parse(email, true, true); }
/** Return a matching key with address-only validation. */
function canonicalizeStrict(email) { return parse(email, false, true); }

exports.InvalidEmail = InvalidEmail;
exports.normalize = normalize;
exports.canonicalize = canonicalize;
exports.normalizeStrict = normalizeStrict;
exports.canonicalizeStrict = canonicalizeStrict;
