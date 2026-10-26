// Package mailcanon normalizes and canonicalises email addresses across Gmail,
// Outlook, Yahoo, and more, by the irondome.dev team.
package mailcanon

import (
	"errors"
	"strings"
	"unicode"
	"unicode/utf8"

	"golang.org/x/net/idna"
)

// ErrInvalidEmail indicates unsupported input syntax or a length limit violation.
var ErrInvalidEmail = errors.New("invalid email")

// Identity contains the normalized input domain, canonical address, and provider.
type Identity struct {
	Domain    string `json:"domain"`
	Canonical string `json:"canonical"`
	Provider  string `json:"provider"`
}

// DNS lengths are checked after removing one root dot; native IDNA tables differ
// in whether VerifyDNSLength permits that root dot.
var domainProfile = idna.New(idna.MapForLookup(), idna.Transitional(false),
	idna.StrictDomainName(true), idna.ValidateLabels(true), idna.CheckHyphens(false),
	idna.CheckJoiners(true), idna.BidiRule(), idna.VerifyDNSLength(false))

// Fastmail first-party domains, matching the upstream source list bundled in
// testdata/fastmail-domains.txt. Keep this table static: normalization is local.
var fastmailDomains = map[string]struct{}{
	"123mail.org": {}, "150mail.com": {}, "150ml.com": {}, "16mail.com": {}, "2-mail.com": {}, "4email.net": {}, "50mail.com": {},
	"airpost.net": {}, "aliencamel.com": {}, "allmail.net": {}, "bestmail.us": {}, "cluemail.com": {}, "elitemail.org": {},
	"emailcorner.net": {}, "emailengine.net": {}, "emailengine.org": {}, "emailgroups.net": {}, "emailplus.org": {}, "emailuser.net": {},
	"eml.cc": {}, "f-m.fm": {}, "fast-email.com": {}, "fast-mail.org": {}, "fastem.com": {}, "fastemail.us": {},
	"fastemailer.com": {}, "fastest.cc": {}, "fastimap.com": {}, "fastmail.ca": {}, "fastmail.cn": {}, "fastmail.co.uk": {},
	"fastmail.com": {}, "fastmail.com.au": {}, "fastmail.de": {}, "fastmail.es": {}, "fastmail.fm": {}, "fastmail.fr": {},
	"fastmail.im": {}, "fastmail.in": {}, "fastmail.jp": {}, "fastmail.mx": {}, "fastmail.net": {}, "fastmail.nl": {},
	"fastmail.org": {}, "fastmail.se": {}, "fastmail.to": {}, "fastmail.tw": {}, "fastmail.uk": {}, "fastmail.us": {},
	"fastmailbox.net": {}, "fastmessaging.com": {}, "fea.st": {}, "fmail.co.uk": {}, "fmailbox.com": {}, "fmgirl.com": {},
	"fmguy.com": {}, "foobox.com": {}, "foobox.net": {}, "ftml.net": {}, "h-mail.us": {}, "hailmail.net": {},
	"imap-mail.com": {}, "imap.cc": {}, "imapmail.org": {}, "immerbox.com": {}, "immermail.com": {}, "inoutbox.com": {},
	"internet-e-mail.com": {}, "internet-mail.org": {}, "internetemails.net": {}, "internetmailing.net": {}, "jetemail.net": {}, "justemail.net": {},
	"letterboxes.org": {}, "lifetimeaddress.com": {}, "mail-central.com": {}, "mail-page.com": {}, "mailas.com": {}, "mailbolt.com": {},
	"mailc.net": {}, "mailcan.com": {}, "mailforce.net": {}, "mailhaven.com": {}, "mailingaddress.org": {}, "mailite.com": {},
	"mailmight.com": {}, "mailnew.com": {}, "mailsent.net": {}, "mailservice.ms": {}, "mailup.net": {}, "mailworks.org": {},
	"mailzone.com": {}, "ml1.net": {}, "mm.st": {}, "myfastmail.com": {}, "mymacmail.com": {}, "nospammail.net": {},
	"onepost.net": {}, "ownmail.net": {}, "penguinmail.com": {}, "permanentmail.com": {}, "petml.com": {}, "pobox.com": {},
	"postinbox.com": {}, "postpro.net": {}, "proinbox.com": {}, "promessage.com": {}, "realemail.net": {}, "reallyfast.biz": {},
	"reallyfast.info": {}, "rushpost.com": {}, "sent.as": {}, "sent.at": {}, "sent.com": {}, "siemprebox.com": {},
	"siempremail.com": {}, "speedpost.net": {}, "speedymail.org": {}, "ssl-mail.com": {}, "swift-mail.com": {}, "the-fastest.net": {},
	"the-quickest.com": {}, "theinternetemail.com": {}, "veribox.net": {}, "veryfast.biz": {}, "veryspeedy.net": {}, "warpmail.net": {},
	"xsmail.com": {}, "yepmail.net": {}, "your-mail.com": {},
}

func ascii(s string) bool {
	for i := 0; i < len(s); i++ {
		if s[i] >= utf8.RuneSelf {
			return false
		}
	}
	return true
}

func trim(s string) string { return strings.Trim(s, " \t\n\r\f") }

func hasACE(domain string) bool {
	for domain != "" {
		label, rest, _ := strings.Cut(domain, ".")
		if len(label) >= 4 && strings.EqualFold(label[:4], "xn--") {
			return true
		}
		domain = rest
	}
	return false
}

// Normalize returns an identity or ErrInvalidEmail. It does not query the network.
func Normalize(email string) (Identity, error) {
	return normalize(email, false)
}

// NormalizeStrict accepts one bare address without wrappers, whitespace or cleanup.
func NormalizeStrict(email string) (Identity, error) {
	return normalize(email, true)
}

func normalize(email string, strict bool) (Identity, error) {
	if len(email) == 0 || len(email) > 512 || !utf8.ValidString(email) {
		return Identity{}, ErrInvalidEmail
	}
	for _, c := range email {
		if c < 32 || c >= 127 && c <= 159 || c == '\u2028' || c == '\u2029' {
			return Identity{}, ErrInvalidEmail
		}
		if strict && (unicode.IsSpace(c) || strings.ContainsRune("<>:?#\u200b\u200c\u200d\ufeff\u2060\uff0e\u3002\uff61\u2024\uff0b\ufe62\uff20", c)) {
			return Identity{}, ErrInvalidEmail
		}
	}
	value := strings.TrimFunc(email, unicode.IsSpace)
	if !ascii(value) {
		var cleaned [512]byte
		size := 0
		for _, c := range value {
			switch c {
			case '\u200b', '\u200c', '\u200d', '\ufeff', '\u2060':
				continue
			case '\uff0e', '\u3002', '\uff61', '\u2024':
				c = '.'
			case '\uff0b', '\ufe62':
				c = '+'
			case '\uff20':
				c = '@'
			}
			size += utf8.EncodeRune(cleaned[size:], c)
		}
		value = string(cleaned[:size])
	}
	value = trim(value)
	if len(value) >= 7 && strings.EqualFold(value[:7], "mailto:") {
		value = trim(value[7:])
		if end := strings.IndexAny(value, "?#"); end >= 0 {
			value = trim(value[:end])
		}
	}
	left, right := strings.IndexByte(value, '<'), strings.LastIndexByte(value, '>')
	if left >= 0 || right >= 0 {
		if left < 0 || right <= left || right != len(value)-1 ||
			strings.ContainsAny(value[:left], "@<>,;:") || strings.ContainsAny(value[left+1:right], "<>") {
			return Identity{}, ErrInvalidEmail
		}
		value = trim(value[left+1 : right])
	}
	at := strings.IndexByte(value, '@')
	if at < 0 || strings.Contains(value[at+1:], "@") {
		return Identity{}, ErrInvalidEmail
	}
	local := trim(value[:at])
	domain := trim(value[at+1:])
	if len(local) == 0 || len(local) > 64 || len(domain) == 0 {
		return Identity{}, ErrInvalidEmail
	}
	var localBytes [64]byte
	for i := 0; i < len(local); i++ {
		b := local[i]
		if b >= 'A' && b <= 'Z' {
			b += 'a' - 'A'
		}
		if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9' || strings.IndexByte("._%+-", b) >= 0) {
			return Identity{}, ErrInvalidEmail
		}
		if b == '.' && (i == 0 || i == len(local)-1 || local[i-1] == '.') {
			return Identity{}, ErrInvalidEmail
		}
		localBytes[i] = b
	}
	if !ascii(domain) || hasACE(domain) {
		converted, err := domainProfile.ToASCII(domain)
		if err != nil {
			return Identity{}, ErrInvalidEmail
		}
		domain = converted
	}
	domain = strings.TrimSuffix(domain, ".")
	if len(domain) == 0 || len(domain) > 253 {
		return Identity{}, ErrInvalidEmail
	}
	var domainBytes [253]byte
	start, dots := 0, 0
	for i := 0; i < len(domain); i++ {
		b := domain[i]
		if b >= 'A' && b <= 'Z' {
			b += 'a' - 'A'
		}
		if b == '.' {
			if i == start || i-start > 63 || domainBytes[start] == '-' || domainBytes[i-1] == '-' {
				return Identity{}, ErrInvalidEmail
			}
			start, dots = i+1, dots+1
		} else if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9' || b == '-') {
			return Identity{}, ErrInvalidEmail
		}
		domainBytes[i] = b
	}
	dl := len(domain)
	if dots == 0 || start == dl || dl-start > 63 || domainBytes[start] == '-' || domainBytes[dl-1] == '-' {
		return Identity{}, ErrInvalidEmail
	}
	domain = string(domainBytes[:dl])
	provider, mapped := "custom", domain
	switch domain {
	case "gmail.com", "googlemail.com":
		provider, mapped = "gmail", "gmail.com"
	case "outlook.com", "hotmail.com", "live.com", "msn.com":
		provider = "microsoft"
	case "icloud.com", "me.com", "mac.com":
		provider, mapped = "icloud", "icloud.com"
	case "proton.me", "protonmail.com", "protonmail.ch", "pm.me":
		provider = "proton"
	case "yahoo.com", "ymail.com", "rocketmail.com":
		provider = "yahoo"
	default:
		if _, ok := fastmailDomains[domain]; ok {
			provider = "fastmail"
		} else if dot := strings.IndexByte(domain, '.'); dot > 0 && strings.IndexByte(domain[dot+1:], '.') >= 0 {
			root := domain[dot+1:]
			if _, ok := fastmailDomains[root]; ok {
				provider, mapped = "fastmail", root
				copy(localBytes[:], domain[:dot])
			}
		}
	}
	ll := len(local)
	if provider == "fastmail" && mapped != domain {
		ll = strings.IndexByte(domain, '.')
	} else if provider != "custom" && provider != "yahoo" {
		for i := 0; i < ll; i++ {
			if localBytes[i] == '+' {
				ll = i
				break
			}
		}
	}
	if provider == "gmail" {
		w := 0
		for i := 0; i < ll; i++ {
			b := localBytes[i]
			if b == '.' {
				continue
			}
			if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9') {
				return Identity{}, ErrInvalidEmail
			}
			localBytes[w] = b
			w++
		}
		ll = w
	} else if provider == "proton" {
		w := 0
		for i := 0; i < ll; i++ {
			b := localBytes[i]
			if b == '.' || b == '_' || b == '-' { continue }
			localBytes[w] = b
			w++
		}
		ll = w
	}
	if ll == 0 || localBytes[ll-1] == '.' || dl+ll+1+len(mapped) > 512 {
		return Identity{}, ErrInvalidEmail
	}
	canonical := string(localBytes[:ll]) + "@" + mapped
	return Identity{Domain: domain, Canonical: canonical, Provider: provider}, nil
}

// Canonicalize returns only the canonical address with Normalize's validation.
func Canonicalize(email string) (string, error) {
	identity, err := Normalize(email)
	return identity.Canonical, err
}

// CanonicalizeStrict returns a matching key with NormalizeStrict's validation.
func CanonicalizeStrict(email string) (string, error) {
	identity, err := NormalizeStrict(email)
	return identity.Canonical, err
}
