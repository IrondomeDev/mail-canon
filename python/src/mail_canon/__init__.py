"""A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team."""

from dataclasses import dataclass
import re
from typing import Literal


__all__ = ["EmailIdentity", "InvalidEmail", "Provider", "normalize", "canonicalize", "normalize_strict", "canonicalize_strict"]
Provider = Literal["custom", "gmail", "microsoft", "icloud", "proton", "fastmail", "yahoo"]
_ASCII_SPACE = " \t\n\r\f"
_UNICODE_SPACE = "\t\n\v\f\r \x85\xa0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000"
_CLEAN = str.maketrans({**dict.fromkeys("\u200b\u200c\u200d\ufeff\u2060"), **dict.fromkeys("\uff0e\u3002\uff61\u2024", "."), **dict.fromkeys("\uff0b\ufe62", "+"), "\uff20": "@"})
_LOCAL = re.compile(r"[a-zA-Z0-9._%+\-]+", re.ASCII)
_DOMAIN = re.compile(r"[a-zA-Z0-9.\-]+", re.ASCII)
_GMAIL = re.compile(r"[a-z0-9.]+", re.ASCII)
_PROTON_DELETE = str.maketrans("", "", "._-")
_RULES: dict[str, tuple[Provider, str | None]] = {
    "gmail.com": ("gmail", "gmail.com"), "googlemail.com": ("gmail", "gmail.com"),
    **dict.fromkeys(("outlook.com", "hotmail.com", "live.com", "msn.com"), ("microsoft", None)),
    **dict.fromkeys(("icloud.com", "me.com", "mac.com"), ("icloud", "icloud.com")),
    **dict.fromkeys(("proton.me", "protonmail.com", "protonmail.ch", "pm.me"), ("proton", None)),
    **dict.fromkeys("""123mail.org 150mail.com 150ml.com 16mail.com 2-mail.com 4email.net 50mail.com airpost.net aliencamel.com allmail.net bestmail.us cluemail.com elitemail.org emailcorner.net emailengine.net emailengine.org emailgroups.net emailplus.org emailuser.net eml.cc f-m.fm fast-email.com fast-mail.org fastem.com fastemail.us fastemailer.com fastest.cc fastimap.com fastmail.ca fastmail.cn fastmail.co.uk fastmail.com fastmail.com.au fastmail.de fastmail.es fastmail.fm fastmail.fr fastmail.im fastmail.in fastmail.jp fastmail.mx fastmail.net fastmail.nl fastmail.org fastmail.se fastmail.to fastmail.tw fastmail.uk fastmail.us fastmailbox.net fastmessaging.com fea.st fmail.co.uk fmailbox.com fmgirl.com fmguy.com foobox.com foobox.net ftml.net h-mail.us hailmail.net imap-mail.com imap.cc imapmail.org immerbox.com immermail.com inoutbox.com internet-e-mail.com internet-mail.org internetemails.net internetmailing.net jetemail.net justemail.net letterboxes.org lifetimeaddress.com mail-central.com mail-page.com mailas.com mailbolt.com mailc.net mailcan.com mailforce.net mailhaven.com mailingaddress.org mailite.com mailmight.com mailnew.com mailsent.net mailservice.ms mailup.net mailworks.org mailzone.com ml1.net mm.st myfastmail.com mymacmail.com nospammail.net onepost.net ownmail.net penguinmail.com permanentmail.com petml.com pobox.com postinbox.com postpro.net proinbox.com promessage.com realemail.net reallyfast.biz reallyfast.info rushpost.com sent.as sent.at sent.com siemprebox.com siempremail.com speedpost.net speedymail.org ssl-mail.com swift-mail.com the-fastest.net the-quickest.com theinternetemail.com veribox.net veryfast.biz veryspeedy.net warpmail.net xsmail.com yepmail.net your-mail.com""".split(), ("fastmail", None)),
    **dict.fromkeys(("yahoo.com", "ymail.com", "rocketmail.com"), ("yahoo", None)),
}


class InvalidEmail(ValueError):
    """The input is outside the supported email syntax."""


@dataclass(frozen=True, slots=True)
class EmailIdentity:
    """Normalized input domain, canonical address, and provider group."""

    domain: str
    canonical: str
    provider: Provider


def _parts(raw: str, *, strict: bool = False) -> tuple[str, str, Provider]:
    if not isinstance(raw, str):
        raise TypeError("email must be a string")
    if not raw or len(raw) > 512:
        raise InvalidEmail("invalid email")
    if any(ord(c) < 32 or 127 <= ord(c) <= 159 or c in "\u2028\u2029" for c in raw):
        raise InvalidEmail("invalid email")
    if strict and any(c in _UNICODE_SPACE or ord(c) in _CLEAN or c in "<>:?#" for c in raw):
        raise InvalidEmail("invalid email")
    if raw.isascii():
        value = raw.strip(_UNICODE_SPACE)
    else:
        try:
            if len(raw.encode("utf-8")) > 512:
                raise InvalidEmail("invalid email")
        except UnicodeEncodeError:
            raise InvalidEmail("invalid email") from None
        value = raw.strip(_UNICODE_SPACE).translate(_CLEAN)
    value = value.strip(_ASCII_SPACE)
    if value[:7].lower() == "mailto:":
        value = value[7:].strip(_ASCII_SPACE)
        query, fragment = value.find("?"), value.find("#")
        end = min(query if query >= 0 else len(value), fragment if fragment >= 0 else len(value))
        value = value[:end].strip(_ASCII_SPACE)
    left, right = value.find("<"), value.rfind(">")
    if left >= 0 or right >= 0:
        if (left < 0 or right <= left or right != len(value) - 1
                or any(c in value[:left] for c in "@<>,;:")
                or any(c in value[left + 1:right] for c in "<>")):
            raise InvalidEmail("invalid email")
        value = value[left + 1:right].strip(_ASCII_SPACE)
    at = value.find("@")
    if at < 0 or "@" in value[at + 1:]:
        raise InvalidEmail("invalid email")
    local = value[:at].strip(_ASCII_SPACE)
    domain = value[at + 1:].strip(_ASCII_SPACE)
    if not 1 <= len(local) <= 64 or not _LOCAL.fullmatch(local):
        raise InvalidEmail("invalid email")
    if local[0] == "." or local[-1] == "." or ".." in local:
        raise InvalidEmail("invalid email")
    if not domain:
        raise InvalidEmail("invalid email")
    if not domain.isascii() or any(label.lower().startswith("xn--") for label in domain.split(".")):
        import idna

        try:
            if domain.isascii():
                # Validate A-labels without changing unrelated ASCII DNS labels.
                for label in domain.split("."):
                    if label.lower().startswith("xn--"):
                        idna.encode(label, uts46=True, transitional=False, std3_rules=True)
            else:
                domain = idna.encode(domain, uts46=True, transitional=False, std3_rules=True).decode("ascii")
        except idna.IDNAError:
            raise InvalidEmail("invalid email") from None
    domain = domain.removesuffix(".").lower()
    if not 1 <= len(domain) <= 253 or "." not in domain or not _DOMAIN.fullmatch(domain):
        raise InvalidEmail("invalid email")
    for label in domain.split("."):
        if not 1 <= len(label) <= 63 or label[0] == "-" or label[-1] == "-":
            raise InvalidEmail("invalid email")
    local = local.lower()
    provider, mapped = _RULES.get(domain, ("custom", None))
    fastmail_subdomain = ""
    if provider == "custom":
        dot = domain.find(".")
        if dot > 0 and domain.find(".", dot + 1) >= 0:
            root = domain[dot + 1:]
            root_rule = _RULES.get(root)
            if root_rule is not None and root_rule[0] == "fastmail":
                provider, mapped, fastmail_subdomain = "fastmail", root, domain[:dot]
    if provider == "fastmail" and fastmail_subdomain:
        local = fastmail_subdomain
    elif provider in ("gmail", "microsoft", "icloud", "proton", "fastmail"):
        local = local.partition("+")[0]
    if provider == "gmail":
        if not _GMAIL.fullmatch(local):
            raise InvalidEmail("invalid email")
        local = local.replace(".", "")
    if provider == "proton":
        local = local.translate(_PROTON_DELETE)
    if not local or local[-1] == ".":
        raise InvalidEmail("invalid email")
    canonical = local + "@" + (mapped or domain)
    if len(domain) + len(canonical) > 512:
        raise InvalidEmail("invalid email")
    return domain, canonical, provider


def normalize(email: str) -> EmailIdentity:
    """Normalize an email or raise InvalidEmail; non-strings raise TypeError."""
    return EmailIdentity(*_parts(email))


def canonicalize(email: str) -> str:
    """Return only the canonical address with the same validation as normalize."""
    return _parts(email)[1]


def normalize_strict(email: str) -> EmailIdentity:
    """Normalize one bare address, rejecting wrappers, whitespace and cleanup."""
    return EmailIdentity(*_parts(email, strict=True))


def canonicalize_strict(email: str) -> str:
    """Return a matching key with normalize_strict's address-only validation."""
    return _parts(email, strict=True)[1]
