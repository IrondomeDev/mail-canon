"""Bounded, provider-aware email normalization."""

from dataclasses import dataclass
import re
from typing import Literal


__all__ = ["EmailIdentity", "InvalidEmail", "Provider", "normalize", "canonicalize", "normalize_strict", "canonicalize_strict"]
Provider = Literal["custom", "gmail", "microsoft", "icloud", "proton", "yahoo"]
_ASCII_SPACE = " \t\n\r\f"
_UNICODE_SPACE = "\t\n\v\f\r \x85\xa0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000"
_CLEAN = str.maketrans({**dict.fromkeys("\u200b\u200c\u200d\ufeff\u2060"), **dict.fromkeys("\uff0e\u3002\uff61\u2024", "."), **dict.fromkeys("\uff0b\ufe62", "+"), "\uff20": "@"})
_LOCAL = re.compile(r"[a-zA-Z0-9._%+\-]+", re.ASCII)
_DOMAIN = re.compile(r"[a-zA-Z0-9.\-]+", re.ASCII)
_GMAIL = re.compile(r"[a-z0-9.]+", re.ASCII)
_RULES: dict[str, tuple[Provider, str | None]] = {
    "gmail.com": ("gmail", "gmail.com"), "googlemail.com": ("gmail", "gmail.com"),
    **dict.fromkeys(("outlook.com", "hotmail.com", "live.com", "msn.com"), ("microsoft", None)),
    **dict.fromkeys(("icloud.com", "me.com", "mac.com"), ("icloud", "icloud.com")),
    **dict.fromkeys(("proton.me", "protonmail.com", "pm.me"), ("proton", None)),
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
    if provider in ("gmail", "microsoft", "icloud", "proton"):
        local = local.partition("+")[0]
    if provider == "gmail":
        if not _GMAIL.fullmatch(local):
            raise InvalidEmail("invalid email")
        local = local.replace(".", "")
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
