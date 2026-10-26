using System.Globalization;

namespace IronDome.MailCanon;

/// <summary>Normalized input domain, canonical address, and provider group.</summary>
public readonly record struct EmailIdentity(string Domain, string Canonical, string Provider);

public sealed class InvalidEmailException : FormatException
{
    public InvalidEmailException() : base("invalid email") { }
}

/// <summary>A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team.</summary>
public static class EmailNormalizer
{
    private static readonly IdnMapping Idn = new() { UseStd3AsciiRules = true, AllowUnassigned = false };
    private static readonly HashSet<string> FastmailDomains = new(StringComparer.Ordinal)
    {
        "123mail.org",
        "150mail.com",
        "150ml.com",
        "16mail.com",
        "2-mail.com",
        "4email.net",
        "50mail.com",
        "airpost.net",
        "aliencamel.com",
        "allmail.net",
        "bestmail.us",
        "cluemail.com",
        "elitemail.org",
        "emailcorner.net",
        "emailengine.net",
        "emailengine.org",
        "emailgroups.net",
        "emailplus.org",
        "emailuser.net",
        "eml.cc",
        "f-m.fm",
        "fast-email.com",
        "fast-mail.org",
        "fastem.com",
        "fastemail.us",
        "fastemailer.com",
        "fastest.cc",
        "fastimap.com",
        "fastmail.ca",
        "fastmail.cn",
        "fastmail.co.uk",
        "fastmail.com",
        "fastmail.com.au",
        "fastmail.de",
        "fastmail.es",
        "fastmail.fm",
        "fastmail.fr",
        "fastmail.im",
        "fastmail.in",
        "fastmail.jp",
        "fastmail.mx",
        "fastmail.net",
        "fastmail.nl",
        "fastmail.org",
        "fastmail.se",
        "fastmail.to",
        "fastmail.tw",
        "fastmail.uk",
        "fastmail.us",
        "fastmailbox.net",
        "fastmessaging.com",
        "fea.st",
        "fmail.co.uk",
        "fmailbox.com",
        "fmgirl.com",
        "fmguy.com",
        "foobox.com",
        "foobox.net",
        "ftml.net",
        "h-mail.us",
        "hailmail.net",
        "imap-mail.com",
        "imap.cc",
        "imapmail.org",
        "immerbox.com",
        "immermail.com",
        "inoutbox.com",
        "internet-e-mail.com",
        "internet-mail.org",
        "internetemails.net",
        "internetmailing.net",
        "jetemail.net",
        "justemail.net",
        "letterboxes.org",
        "lifetimeaddress.com",
        "mail-central.com",
        "mail-page.com",
        "mailas.com",
        "mailbolt.com",
        "mailc.net",
        "mailcan.com",
        "mailforce.net",
        "mailhaven.com",
        "mailingaddress.org",
        "mailite.com",
        "mailmight.com",
        "mailnew.com",
        "mailsent.net",
        "mailservice.ms",
        "mailup.net",
        "mailworks.org",
        "mailzone.com",
        "ml1.net",
        "mm.st",
        "myfastmail.com",
        "mymacmail.com",
        "nospammail.net",
        "onepost.net",
        "ownmail.net",
        "penguinmail.com",
        "permanentmail.com",
        "petml.com",
        "pobox.com",
        "postinbox.com",
        "postpro.net",
        "proinbox.com",
        "promessage.com",
        "realemail.net",
        "reallyfast.biz",
        "reallyfast.info",
        "rushpost.com",
        "sent.as",
        "sent.at",
        "sent.com",
        "siemprebox.com",
        "siempremail.com",
        "speedpost.net",
        "speedymail.org",
        "ssl-mail.com",
        "swift-mail.com",
        "the-fastest.net",
        "the-quickest.com",
        "theinternetemail.com",
        "veribox.net",
        "veryfast.biz",
        "veryspeedy.net",
        "warpmail.net",
        "xsmail.com",
        "yepmail.net",
        "your-mail.com"
    };

    private static bool ContainsRtl(char c) => c is >= '\u0590' and <= '\u08ff'
        or >= '\ufb1d' and <= '\ufdff' or >= '\ufe70' and <= '\ufeff';

    private static bool AsciiSpace(char c) => c is ' ' or '\t' or '\n' or '\r' or '\f';
    private static bool UnicodeSpace(char c) => c is >= '\u0009' and <= '\u000d'
        or '\u0020' or '\u0085' or '\u00a0' or '\u1680'
        or >= '\u2000' and <= '\u200a' or '\u2028' or '\u2029' or '\u202f' or '\u205f' or '\u3000';

    private static ReadOnlySpan<char> Trim(ReadOnlySpan<char> s, bool unicode = false)
    {
        int start = 0, end = s.Length;
        while (start < end && (unicode ? UnicodeSpace(s[start]) : AsciiSpace(s[start]))) start++;
        while (end > start && (unicode ? UnicodeSpace(s[end - 1]) : AsciiSpace(s[end - 1]))) end--;
        return s[start..end];
    }

    private static bool HasMailtoPrefix(ReadOnlySpan<char> value)
    {
        if (value.Length < 7) return false;
        for (int i = 0; i < 7; i++)
        {
            char c = value[i];
            if (c is >= 'A' and <= 'Z') c = (char)(c + 32);
            if (c != "mailto:"[i]) return false;
        }
        return true;
    }

    private static void CheckLength(string s)
    {
        if (s.Length is 0 or > 512) throw new InvalidEmailException();
        int bytes = 0;
        for (int i = 0; i < s.Length; i++)
        {
            char c = s[i];
            if (c < 128) bytes++;
            else if (c < 2048) bytes += 2;
            else if (char.IsHighSurrogate(c))
            {
                if (++i >= s.Length || !char.IsLowSurrogate(s[i])) throw new InvalidEmailException();
                bytes += 4;
            }
            else
            {
                if (char.IsLowSurrogate(c)) throw new InvalidEmailException();
                bytes += 3;
            }
            if (bytes > 512) throw new InvalidEmailException();
        }
    }

    /// <summary>Return the normalized input domain, canonical address, and provider.</summary>
    public static EmailIdentity Normalize(string email)
        => Normalize(email, false);

    /// <summary>Normalize one bare address without wrappers, whitespace or cleanup.</summary>
    public static EmailIdentity NormalizeStrict(string email)
        => Normalize(email, true);

    private static EmailIdentity Normalize(string email, bool strict)
    {
        ArgumentNullException.ThrowIfNull(email);
        CheckLength(email);
        foreach (char c in email)
        {
            if (c < 32 || c is >= '\u007f' and <= '\u009f' or '\u2028' or '\u2029') throw new InvalidEmailException();
            if (strict && (UnicodeSpace(c) || "<>:?#\u200b\u200c\u200d\ufeff\u2060\uff0e\u3002\uff61\u2024\uff0b\ufe62\uff20".Contains(c))) throw new InvalidEmailException();
        }
        ReadOnlySpan<char> value = Trim(email.AsSpan(), true);
        Span<char> cleaned = stackalloc char[512];
        int used = 0;
        bool changed = false;
        foreach (char c in value)
        {
            char mapped = c switch
            {
                '\u200b' or '\u200c' or '\u200d' or '\ufeff' or '\u2060' => '\0',
                '\uff0e' or '\u3002' or '\uff61' or '\u2024' => '.',
                '\uff0b' or '\ufe62' => '+', '\uff20' => '@', _ => c
            };
            // A real NUL is retained and rejected by validation.
            if (mapped == '\0' && c != '\0') { changed = true; continue; }
            changed |= mapped != c;
            cleaned[used++] = mapped;
        }
        if (changed) value = new string(cleaned[..used]).AsSpan();
        value = Trim(value);
        if (HasMailtoPrefix(value))
        {
            value = Trim(value[7..]);
            int end = value.IndexOfAny('?', '#');
            if (end >= 0) value = Trim(value[..end]);
        }
        int left = value.IndexOf('<'), right = value.LastIndexOf('>');
        if (left >= 0 || right >= 0)
        {
            if (left < 0 || right <= left || right != value.Length - 1
                || value[..left].IndexOfAny("@<>,;:".AsSpan()) >= 0
                || value[(left + 1)..right].IndexOfAny('<', '>') >= 0) throw new InvalidEmailException();
            value = Trim(value[(left + 1)..right]);
        }
        int at = value.IndexOf('@');
        if (at < 0 || value[(at + 1)..].Contains('@')) throw new InvalidEmailException();
        ReadOnlySpan<char> local = Trim(value[..at]);
        ReadOnlySpan<char> domain = Trim(value[(at + 1)..]);
        if (local.Length is 0 or > 64 || domain.IsEmpty) throw new InvalidEmailException();
        Span<char> localBuffer = stackalloc char[64];
        for (int i = 0; i < local.Length; i++)
        {
            char c = local[i];
            if (c is >= 'A' and <= 'Z') c = (char)(c + 32);
            if (!(c is >= 'a' and <= 'z' or >= '0' and <= '9' or '.' or '_' or '%' or '+' or '-')) throw new InvalidEmailException();
            if (c == '.' && (i == 0 || i == local.Length - 1 || local[i - 1] == '.')) throw new InvalidEmailException();
            localBuffer[i] = c;
        }
        bool unicodeDomain = false;
        foreach (char c in domain) unicodeDomain |= c > 127;
        bool aceDomain = domain.StartsWith("xn--", StringComparison.OrdinalIgnoreCase)
            || domain.Contains(".xn--", StringComparison.OrdinalIgnoreCase);
        if (unicodeDomain || aceDomain)
        {
            for (int offset = 0; offset < domain.Length;)
            {
                int dot = domain[offset..].IndexOf('.');
                ReadOnlySpan<char> label = dot < 0 ? domain[offset..] : domain.Slice(offset, dot);
                bool rtl = false, latinAscii = false;
                foreach (char c in label)
                {
                    rtl |= ContainsRtl(c);
                    latinAscii |= c is >= 'A' and <= 'Z' or >= 'a' and <= 'z';
                }
                if (rtl && latinAscii) throw new InvalidEmailException();
                if (dot < 0) break;
                offset += dot + 1;
            }
            try
            {
                string converted = Idn.GetAscii(domain.ToString());
                // Some platform backends leave invalid ASCII A-labels unchanged.
                foreach (string label in converted.Split('.'))
                {
                    if (!label.StartsWith("xn--", StringComparison.OrdinalIgnoreCase)) continue;
                    string decoded = Idn.GetUnicode(label);
                    if (decoded.All(c => c < 128) || !Idn.GetAscii(decoded).Equals(label, StringComparison.OrdinalIgnoreCase)) throw new InvalidEmailException();
                }
                domain = converted.AsSpan();
            }
            catch (ArgumentException) { throw new InvalidEmailException(); }
        }
        if (!domain.IsEmpty && domain[^1] == '.') domain = domain[..^1];
        if (domain.Length is 0 or > 253) throw new InvalidEmailException();
        Span<char> domainBuffer = stackalloc char[253];
        int start = 0, dots = 0;
        for (int i = 0; i < domain.Length; i++)
        {
            char c = domain[i];
            if (c is >= 'A' and <= 'Z') c = (char)(c + 32);
            if (c == '.')
            {
                if (i == start || i - start > 63 || domainBuffer[start] == '-' || domainBuffer[i - 1] == '-') throw new InvalidEmailException();
                start = i + 1;
                dots++;
            }
            else if (!(c is >= 'a' and <= 'z' or >= '0' and <= '9' or '-')) throw new InvalidEmailException();
            domainBuffer[i] = c;
        }
        if (dots == 0 || start == domain.Length || domain.Length - start > 63 || domainBuffer[start] == '-' || domainBuffer[domain.Length - 1] == '-') throw new InvalidEmailException();
        string normalizedDomain = new(domainBuffer[..domain.Length]);
        string? fastmailUsername = null;
        string? fastmailRoot = null;
        string provider = normalizedDomain switch
        {
            "gmail.com" or "googlemail.com" => "gmail",
            "outlook.com" or "hotmail.com" or "live.com" or "msn.com" => "microsoft",
            "icloud.com" or "me.com" or "mac.com" => "icloud",
            "proton.me" or "protonmail.com" or "protonmail.ch" or "pm.me" => "proton",
            "yahoo.com" or "ymail.com" or "rocketmail.com" => "yahoo", _ => "custom"
        };
        if (provider == "custom")
        {
            if (FastmailDomains.Contains(normalizedDomain)) fastmailRoot = normalizedDomain;
            else if (dots >= 2)
            {
                int dot = normalizedDomain.IndexOf('.');
                string root = normalizedDomain[(dot + 1)..];
                if (FastmailDomains.Contains(root))
                {
                    fastmailRoot = root;
                    fastmailUsername = normalizedDomain[..dot];
                }
            }
            if (fastmailRoot is not null) provider = "fastmail";
        }
        string mappedDomain = provider switch { "gmail" => "gmail.com", "icloud" => "icloud.com",
            "fastmail" => fastmailRoot!, _ => normalizedDomain };
        int length = local.Length;
        if (provider == "fastmail" && fastmailUsername is not null)
        {
            length = fastmailUsername.Length;
            fastmailUsername.AsSpan().CopyTo(localBuffer);
        }
        else if (provider is not ("custom" or "yahoo"))
        {
            int plus = localBuffer[..length].IndexOf('+');
            if (plus >= 0) length = plus;
        }
        if (provider == "proton")
        {
            int write = 0;
            for (int i = 0; i < length; i++)
            {
                char c = localBuffer[i];
                if (c is '.' or '_' or '-') continue;
                localBuffer[write++] = c;
            }
            length = write;
        }
        else if (provider == "gmail")
        {
            int write = 0;
            for (int i = 0; i < length; i++)
            {
                char c = localBuffer[i];
                if (c == '.') continue;
                if (!(c is >= 'a' and <= 'z' or >= '0' and <= '9')) throw new InvalidEmailException();
                localBuffer[write++] = c;
            }
            length = write;
        }
        if (length == 0 || localBuffer[length - 1] == '.' || domain.Length + length + 1 + mappedDomain.Length > 512) throw new InvalidEmailException();
        string canonical = string.Concat(localBuffer[..length], "@".AsSpan(), mappedDomain.AsSpan());
        return new EmailIdentity(normalizedDomain, canonical, provider);
    }

    /// <summary>Return only the canonical address with Normalize's validation.</summary>
    public static string Canonicalize(string email) => Normalize(email).Canonical;

    /// <summary>Return a matching key with address-only validation.</summary>
    public static string CanonicalizeStrict(string email) => NormalizeStrict(email).Canonical;

    /// <summary>Return false for null or invalid bare-address input.</summary>
    public static bool TryNormalizeStrict(string? email, out EmailIdentity identity)
    {
        identity = default;
        if (email is null) return false;
        try { identity = NormalizeStrict(email); return true; }
        catch (InvalidEmailException) { return false; }
    }

    /// <summary>Return false for null or invalid input; otherwise return the identity.</summary>
    public static bool TryNormalize(string? email, out EmailIdentity identity)
    {
        identity = default;
        if (email is null) return false;
        try { identity = Normalize(email); return true; }
        catch (InvalidEmailException) { return false; }
    }
}
