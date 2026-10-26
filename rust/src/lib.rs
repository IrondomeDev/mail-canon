#![forbid(unsafe_code)]
//! A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.

use idna::uts46::{AsciiDenyList, DnsLength, Hyphens, Uts46};
use std::{borrow::Cow, fmt};

pub const MAX_INPUT_BYTES: usize = 512;
pub const OUTPUT_CAPACITY: usize = 512;

// Exact Fastmail first-party domains, embedded for allocation-free ASCII lookup.
const FASTMAIL_DOMAINS: &[&[u8]] = &[
    b"123mail.org",
    b"150mail.com",
    b"150ml.com",
    b"16mail.com",
    b"2-mail.com",
    b"4email.net",
    b"50mail.com",
    b"airpost.net",
    b"aliencamel.com",
    b"allmail.net",
    b"bestmail.us",
    b"cluemail.com",
    b"elitemail.org",
    b"emailcorner.net",
    b"emailengine.net",
    b"emailengine.org",
    b"emailgroups.net",
    b"emailplus.org",
    b"emailuser.net",
    b"eml.cc",
    b"f-m.fm",
    b"fast-email.com",
    b"fast-mail.org",
    b"fastem.com",
    b"fastemail.us",
    b"fastemailer.com",
    b"fastest.cc",
    b"fastimap.com",
    b"fastmail.ca",
    b"fastmail.cn",
    b"fastmail.co.uk",
    b"fastmail.com",
    b"fastmail.com.au",
    b"fastmail.de",
    b"fastmail.es",
    b"fastmail.fm",
    b"fastmail.fr",
    b"fastmail.im",
    b"fastmail.in",
    b"fastmail.jp",
    b"fastmail.mx",
    b"fastmail.net",
    b"fastmail.nl",
    b"fastmail.org",
    b"fastmail.se",
    b"fastmail.to",
    b"fastmail.tw",
    b"fastmail.uk",
    b"fastmail.us",
    b"fastmailbox.net",
    b"fastmessaging.com",
    b"fea.st",
    b"fmail.co.uk",
    b"fmailbox.com",
    b"fmgirl.com",
    b"fmguy.com",
    b"foobox.com",
    b"foobox.net",
    b"ftml.net",
    b"h-mail.us",
    b"hailmail.net",
    b"imap-mail.com",
    b"imap.cc",
    b"imapmail.org",
    b"immerbox.com",
    b"immermail.com",
    b"inoutbox.com",
    b"internet-e-mail.com",
    b"internet-mail.org",
    b"internetemails.net",
    b"internetmailing.net",
    b"jetemail.net",
    b"justemail.net",
    b"letterboxes.org",
    b"lifetimeaddress.com",
    b"mail-central.com",
    b"mail-page.com",
    b"mailas.com",
    b"mailbolt.com",
    b"mailc.net",
    b"mailcan.com",
    b"mailforce.net",
    b"mailhaven.com",
    b"mailingaddress.org",
    b"mailite.com",
    b"mailmight.com",
    b"mailnew.com",
    b"mailsent.net",
    b"mailservice.ms",
    b"mailup.net",
    b"mailworks.org",
    b"mailzone.com",
    b"ml1.net",
    b"mm.st",
    b"myfastmail.com",
    b"mymacmail.com",
    b"nospammail.net",
    b"onepost.net",
    b"ownmail.net",
    b"penguinmail.com",
    b"permanentmail.com",
    b"petml.com",
    b"pobox.com",
    b"postinbox.com",
    b"postpro.net",
    b"proinbox.com",
    b"promessage.com",
    b"realemail.net",
    b"reallyfast.biz",
    b"reallyfast.info",
    b"rushpost.com",
    b"sent.as",
    b"sent.at",
    b"sent.com",
    b"siemprebox.com",
    b"siempremail.com",
    b"speedpost.net",
    b"speedymail.org",
    b"ssl-mail.com",
    b"swift-mail.com",
    b"the-fastest.net",
    b"the-quickest.com",
    b"theinternetemail.com",
    b"veribox.net",
    b"veryfast.biz",
    b"veryspeedy.net",
    b"warpmail.net",
    b"xsmail.com",
    b"yepmail.net",
    b"your-mail.com",
];

type FastmailMatch<'a> = (&'a [u8], Option<&'a [u8]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidEmail,
    BufferTooSmall,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidEmail => "invalid email",
            Self::BufferTooSmall => "output buffer too small",
        })
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider { Custom, Gmail, Microsoft, Icloud, Proton, Yahoo, Fastmail }
impl Provider {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Custom => "custom", Self::Gmail => "gmail", Self::Microsoft => "microsoft",
            Self::Icloud => "icloud", Self::Proton => "proton", Self::Yahoo => "yahoo", Self::Fastmail => "fastmail",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailIdentity {
    pub domain: String,
    pub canonical: String,
    pub provider: Provider,
}

/// Byte lengths in the caller's buffer: domain, then canonical address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmailParts {
    pub domain_len: usize,
    pub canonical_len: usize,
    pub provider: Provider,
}

fn ascii_trim(s: &str) -> &str {
    s.trim_matches(|c| matches!(c, '\t' | '\n' | '\x0c' | '\r' | ' '))
}

/// Write domain followed by canonical address. Ordinary ASCII without Punycode allocates no heap memory.
/// On error, the output buffer is unchanged.
pub fn normalize_into(email: &str, output: &mut [u8]) -> Result<EmailParts, Error> {
    parse_into(email, output, false)
}

/// Write a bare address without wrappers, whitespace or cleanup.
/// On error, the output buffer is unchanged.
pub fn normalize_strict_into(email: &str, output: &mut [u8]) -> Result<EmailParts, Error> {
    parse_into(email, output, true)
}

fn parse_into(email: &str, output: &mut [u8], strict: bool) -> Result<EmailParts, Error> {
    use Error::InvalidEmail as Invalid;
    if email.is_empty() || email.len() > MAX_INPUT_BYTES { return Err(Invalid); }
    for c in email.chars() {
        if c <= '\u{1f}' || ('\u{7f}'..='\u{9f}').contains(&c) || matches!(c, '\u{2028}' | '\u{2029}') {
            return Err(Invalid);
        }
        if strict && (c.is_whitespace() || "<>:?#\u{200b}\u{200c}\u{200d}\u{feff}\u{2060}\u{ff0e}\u{3002}\u{ff61}\u{2024}\u{ff0b}\u{fe62}\u{ff20}".contains(c)) {
            return Err(Invalid);
        }
    }
    let mut cleaned = [0u8; MAX_INPUT_BYTES];
    let trimmed = email.trim();
    let mut value = if trimmed.is_ascii() {
        trimmed
    } else {
        let mut length = 0;
        for c in trimmed.chars() {
            let mapped = match c {
                '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{feff}' | '\u{2060}' => continue,
                '\u{ff0e}' | '\u{3002}' | '\u{ff61}' | '\u{2024}' => '.',
                '\u{ff0b}' | '\u{fe62}' => '+', '\u{ff20}' => '@', other => other,
            };
            let next = length + mapped.len_utf8();
            mapped.encode_utf8(&mut cleaned[length..next]);
            length = next;
        }
        std::str::from_utf8(&cleaned[..length]).map_err(|_| Invalid)?
    };
    value = ascii_trim(value);
    if value.get(..7).is_some_and(|s| s.eq_ignore_ascii_case("mailto:")) {
        value = ascii_trim(&value[7..]);
        if let Some(end) = value.find(['?', '#']) { value = ascii_trim(&value[..end]); }
    }
    if value.contains(['<', '>']) {
        let (left, right) = (value.find('<').ok_or(Invalid)?, value.rfind('>').ok_or(Invalid)?);
        if left >= right || right != value.len() - 1 || value[..left].contains(['@', '<', '>', ',', ';', ':'])
            || value[left + 1..right].contains(['<', '>']) { return Err(Invalid); }
        value = ascii_trim(&value[left + 1..right]);
    }
    let (local_raw, domain_raw) = value.split_once('@').ok_or(Invalid)?;
    if domain_raw.contains('@') { return Err(Invalid); }
    let local_raw = ascii_trim(local_raw).as_bytes();
    if local_raw.is_empty() || local_raw.len() > 64 { return Err(Invalid); }
    let mut local = [0u8; 64];
    for (i, &b) in local_raw.iter().enumerate() {
        if !b.is_ascii_alphanumeric() && !b"._%+-".contains(&b) { return Err(Invalid); }
        if b == b'.' && (i == 0 || i + 1 == local_raw.len() || local_raw[i - 1] == b'.') { return Err(Invalid); }
        local[i] = b.to_ascii_lowercase();
    }
    let raw_domain = ascii_trim(domain_raw);
    let ace = raw_domain.split('.').any(|label| label.get(..4).is_some_and(|s| s.eq_ignore_ascii_case("xn--")));
    let encoded: Cow<'_, str> = if raw_domain.is_ascii() && !ace {
        Cow::Borrowed(raw_domain)
    } else {
        Uts46::new().to_ascii(raw_domain.as_bytes(), AsciiDenyList::STD3,
            Hyphens::CheckFirstLast, DnsLength::VerifyAllowRootDot).map_err(|_| Invalid)?
    };
    let encoded = encoded.strip_suffix('.').unwrap_or(&encoded);
    if encoded.is_empty() || encoded.len() > 253 { return Err(Invalid); }
    let mut domain = [0u8; 253];
    let mut label_start = 0;
    let mut dots = 0;
    for (i, b) in encoded.bytes().enumerate() {
        if b == b'.' {
            if i == label_start || i - label_start > 63 || domain[label_start] == b'-' || domain[i - 1] == b'-' { return Err(Invalid); }
            dots += 1;
            label_start = i + 1;
        } else if !b.is_ascii_alphanumeric() && b != b'-' { return Err(Invalid); }
        domain[i] = b.to_ascii_lowercase();
    }
    let dl = encoded.len();
    if dots == 0 || label_start == dl || dl - label_start > 63 || domain[label_start] == b'-' || domain[dl - 1] == b'-' { return Err(Invalid); }
    let domain = &domain[..dl];
    let (provider, mapped, fastmail): (Provider, &[u8], Option<FastmailMatch<'_>>) = match domain {
        b"gmail.com" | b"googlemail.com" => (Provider::Gmail, b"gmail.com", None),
        b"outlook.com" | b"hotmail.com" | b"live.com" | b"msn.com" => (Provider::Microsoft, domain, None),
        b"icloud.com" | b"me.com" | b"mac.com" => (Provider::Icloud, b"icloud.com", None),
        b"proton.me" | b"protonmail.com" | b"protonmail.ch" | b"pm.me" => (Provider::Proton, domain, None),
        b"yahoo.com" | b"ymail.com" | b"rocketmail.com" => (Provider::Yahoo, domain, None),
        _ => {
            let fastmail = if FASTMAIL_DOMAINS.binary_search(&domain).is_ok() {
                Some((domain, None))
            } else if dots >= 2 {
                let dot = domain.iter().position(|&b| b == b'.').ok_or(Invalid)?;
                let username = &domain[..dot];
                let root = &domain[dot + 1..];
                FASTMAIL_DOMAINS.binary_search(&root).ok().map(|_| (root, Some(username)))
            } else { None };
            if let Some((root, _)) = fastmail {
                (Provider::Fastmail, root, fastmail)
            } else {
                (Provider::Custom, domain, None)
            }
        }
    };
    let mut ll = local_raw.len();
    if provider == Provider::Fastmail {
        if let Some((_, Some(username))) = fastmail {
            ll = username.len();
            local[..ll].copy_from_slice(username);
        } else if let Some(plus) = local[..ll].iter().position(|&b| b == b'+') { ll = plus; }
    } else if !matches!(provider, Provider::Custom | Provider::Yahoo) {
        if let Some(plus) = local[..ll].iter().position(|&b| b == b'+') { ll = plus; }
    }
    if provider == Provider::Proton {
        let mut count = 0;
        for i in 0..ll {
            let b = local[i];
            if matches!(b, b'.' | b'_' | b'-') { continue; }
            local[count] = b;
            count += 1;
        }
        ll = count;
    } else if provider == Provider::Gmail {
        let mut count = 0;
        for i in 0..ll {
            let b = local[i];
            if b == b'.' { continue; }
            if !b.is_ascii_alphanumeric() { return Err(Invalid); }
            local[count] = b;
            count += 1;
        }
        ll = count;
    }
    if ll == 0 || local[ll - 1] == b'.' { return Err(Invalid); }
    let cl = ll + 1 + mapped.len();
    if dl + cl > OUTPUT_CAPACITY { return Err(Invalid); }
    if output.len() < dl + cl { return Err(Error::BufferTooSmall); }
    output[..dl].copy_from_slice(domain);
    output[dl..dl + ll].copy_from_slice(&local[..ll]);
    output[dl + ll] = b'@';
    output[dl + ll + 1..dl + cl].copy_from_slice(mapped);
    Ok(EmailParts { domain_len: dl, canonical_len: cl, provider })
}

/// Return the normalized input domain, canonical address, and provider.
pub fn normalize(email: &str) -> Result<EmailIdentity, Error> {
    let mut output = [0u8; OUTPUT_CAPACITY];
    let parts = normalize_into(email, &mut output)?;
    let (domain, rest) = output.split_at(parts.domain_len);
    Ok(EmailIdentity {
        domain: std::str::from_utf8(domain).map_err(|_| Error::InvalidEmail)?.to_owned(),
        canonical: std::str::from_utf8(&rest[..parts.canonical_len]).map_err(|_| Error::InvalidEmail)?.to_owned(),
        provider: parts.provider,
    })
}

/// Return only the canonical address, without allocating an owned domain.
pub fn canonicalize(email: &str) -> Result<String, Error> {
    let mut output = [0u8; OUTPUT_CAPACITY];
    let parts = normalize_into(email, &mut output)?;
    let bytes = &output[parts.domain_len..parts.domain_len + parts.canonical_len];
    Ok(std::str::from_utf8(bytes).map_err(|_| Error::InvalidEmail)?.to_owned())
}

/// Normalize one bare address with the same provider matching policy.
pub fn normalize_strict(email: &str) -> Result<EmailIdentity, Error> {
    let mut output = [0u8; OUTPUT_CAPACITY];
    let parts = normalize_strict_into(email, &mut output)?;
    let (domain, rest) = output.split_at(parts.domain_len);
    Ok(EmailIdentity {
        domain: std::str::from_utf8(domain).map_err(|_| Error::InvalidEmail)?.to_owned(),
        canonical: std::str::from_utf8(&rest[..parts.canonical_len]).map_err(|_| Error::InvalidEmail)?.to_owned(),
        provider: parts.provider,
    })
}

/// Return a matching key with address-only validation.
pub fn canonicalize_strict(email: &str) -> Result<String, Error> {
    let mut output = [0u8; OUTPUT_CAPACITY];
    let parts = normalize_strict_into(email, &mut output)?;
    let bytes = &output[parts.domain_len..parts.domain_len + parts.canonical_len];
    Ok(std::str::from_utf8(bytes).map_err(|_| Error::InvalidEmail)?.to_owned())
}
