package dev.irondome.mailcanon;

import com.ibm.icu.text.IDNA;
import java.util.Locale;
import java.util.Objects;

/** Bounded, provider-aware email normalization. */
public final class MailCanon {
    private static final class DomainIdna {
        private static final IDNA INSTANCE = IDNA.getUTS46Instance(IDNA.USE_STD3_RULES
            | IDNA.CHECK_BIDI | IDNA.CHECK_CONTEXTJ | IDNA.NONTRANSITIONAL_TO_ASCII);
    }

    private MailCanon() {}

    public record Identity(String domain, String canonical, String provider) {}

    public static final class InvalidEmailException extends IllegalArgumentException {
        private static final long serialVersionUID = 1L;
        public InvalidEmailException() { super("invalid email"); }
    }

    private static InvalidEmailException invalid() { return new InvalidEmailException(); }

    private static boolean asciiSpace(char c) {
        return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f';
    }

    private static boolean unicodeSpace(char c) {
        return c >= 9 && c <= 13 || c == 32 || c == 0x85 || c == 0xa0 || c == 0x1680
            || c >= 0x2000 && c <= 0x200a || c == 0x2028 || c == 0x2029
            || c == 0x202f || c == 0x205f || c == 0x3000;
    }

    private static String trim(String value, boolean unicode) {
        int start = 0, end = value.length();
        while (start < end && (unicode ? unicodeSpace(value.charAt(start)) : asciiSpace(value.charAt(start)))) start++;
        while (end > start && (unicode ? unicodeSpace(value.charAt(end - 1)) : asciiSpace(value.charAt(end - 1)))) end--;
        return value.substring(start, end);
    }

    private static boolean ascii(String value) {
        for (int i = 0; i < value.length(); i++) if (value.charAt(i) > 127) return false;
        return true;
    }

    private static void checkLength(String input) {
        if (input.isEmpty() || input.length() > 512) throw invalid();
        int bytes = 0;
        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            if (c < 128) bytes++;
            else if (c < 2048) bytes += 2;
            else if (Character.isHighSurrogate(c)) {
                if (++i >= input.length() || !Character.isLowSurrogate(input.charAt(i))) throw invalid();
                bytes += 4;
            } else {
                if (Character.isLowSurrogate(c)) throw invalid();
                bytes += 3;
            }
            if (bytes > 512) throw invalid();
        }
    }

    private static String clean(String raw) {
        String value = trim(raw, true);
        if (ascii(value)) return trim(value, false);
        StringBuilder result = new StringBuilder(value.length());
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            switch (c) {
                case '\u200b', '\u200c', '\u200d', '\ufeff', '\u2060' -> { continue; }
                case '\uff0e', '\u3002', '\uff61', '\u2024' -> c = '.';
                case '\uff0b', '\ufe62' -> c = '+';
                case '\uff20' -> c = '@';
                default -> { }
            }
            result.append(c);
        }
        return trim(result.toString(), false);
    }

    /** Return the normalized input domain, canonical address, and provider. */
    public static Identity normalize(String email) {
        return normalize(email, false);
    }

    /** Normalize one bare address without wrappers, whitespace or cleanup. */
    public static Identity normalizeStrict(String email) {
        return normalize(email, true);
    }

    private static Identity normalize(String email, boolean strict) {
        Objects.requireNonNull(email, "email");
        checkLength(email);
        for (int i = 0; i < email.length(); i++) {
            char c = email.charAt(i);
            if (c < 32 || c >= 127 && c <= 159 || c == '\u2028' || c == '\u2029') throw invalid();
            if (strict && (unicodeSpace(c) || "<>:?#\u200b\u200c\u200d\ufeff\u2060\uff0e\u3002\uff61\u2024\uff0b\ufe62\uff20".indexOf(c) >= 0)) throw invalid();
        }
        String value = clean(email);
        if (value.length() >= 7 && value.substring(0, 7).toLowerCase(Locale.ROOT).equals("mailto:")) {
            value = trim(value.substring(7), false);
            int q = value.indexOf('?'), h = value.indexOf('#');
            int end = Math.min(q < 0 ? value.length() : q, h < 0 ? value.length() : h);
            value = trim(value.substring(0, end), false);
        }
        int left = value.indexOf('<'), right = value.lastIndexOf('>');
        if (left >= 0 || right >= 0) {
            if (left < 0 || right <= left || right != value.length() - 1
                || value.substring(0, left).chars().anyMatch(c -> "@<>,;:".indexOf(c) >= 0)
                || value.substring(left + 1, right).indexOf('<') >= 0
                || value.substring(left + 1, right).indexOf('>') >= 0) throw invalid();
            value = trim(value.substring(left + 1, right), false);
        }
        int at = value.indexOf('@');
        if (at < 0 || value.indexOf('@', at + 1) >= 0) throw invalid();
        String local = trim(value.substring(0, at), false);
        String domain = trim(value.substring(at + 1), false);
        if (local.isEmpty() || local.length() > 64 || domain.isEmpty()) throw invalid();
        char[] buffer = new char[local.length()];
        for (int i = 0; i < local.length(); i++) {
            char c = local.charAt(i);
            if (c >= 'A' && c <= 'Z') c += 32;
            if (!(c >= 'a' && c <= 'z' || c >= '0' && c <= '9' || "._%+-".indexOf(c) >= 0)) throw invalid();
            if (c == '.' && (i == 0 || i == local.length() - 1 || local.charAt(i - 1) == '.')) throw invalid();
            buffer[i] = c;
        }
        String lowerDomain = domain.toLowerCase(Locale.ROOT);
        if (!ascii(domain) || lowerDomain.startsWith("xn--") || lowerDomain.contains(".xn--")) {
            IDNA.Info info = new IDNA.Info();
            StringBuilder result = new StringBuilder(domain.length());
            DomainIdna.INSTANCE.nameToASCII(domain, result, info);
            for (IDNA.Error error : info.getErrors()) if (error != IDNA.Error.HYPHEN_3_4) throw invalid();
            domain = result.toString();
        }
        if (domain.endsWith(".")) domain = domain.substring(0, domain.length() - 1);
        if (domain.isEmpty() || domain.length() > 253) throw invalid();
        int start = 0, dots = 0;
        for (int i = 0; i <= domain.length(); i++) {
            if (i == domain.length() || domain.charAt(i) == '.') {
                if (i == start || i - start > 63 || domain.charAt(start) == '-' || domain.charAt(i - 1) == '-') throw invalid();
                if (i < domain.length()) dots++;
                start = i + 1;
            } else {
                char c = domain.charAt(i);
                if (!(c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z' || c >= '0' && c <= '9' || c == '-')) throw invalid();
            }
        }
        if (dots == 0) throw invalid();
        domain = domain.toLowerCase(Locale.ROOT);
        String provider = switch (domain) {
            case "gmail.com", "googlemail.com" -> "gmail";
            case "outlook.com", "hotmail.com", "live.com", "msn.com" -> "microsoft";
            case "icloud.com", "me.com", "mac.com" -> "icloud";
            case "proton.me", "protonmail.com", "pm.me" -> "proton";
            case "yahoo.com", "ymail.com", "rocketmail.com" -> "yahoo";
            default -> "custom";
        };
        String mapped = switch (provider) { case "gmail" -> "gmail.com"; case "icloud" -> "icloud.com"; default -> domain; };
        int length = buffer.length;
        if (!provider.equals("custom") && !provider.equals("yahoo")) {
            for (int i = 0; i < length; i++) if (buffer[i] == '+') { length = i; break; }
        }
        if (provider.equals("gmail")) {
            int write = 0;
            for (int i = 0; i < length; i++) {
                char c = buffer[i];
                if (c == '.') continue;
                if (!(c >= 'a' && c <= 'z' || c >= '0' && c <= '9')) throw invalid();
                buffer[write++] = c;
            }
            length = write;
        }
        if (length == 0 || buffer[length - 1] == '.' || domain.length() + length + 1 + mapped.length() > 512) throw invalid();
        String canonical = new String(buffer, 0, length) + "@" + mapped;
        return new Identity(domain, canonical, provider);
    }

    /** Return only the canonical address with normalize's validation. */
    public static String canonicalize(String email) { return normalize(email).canonical(); }

    /** Return a matching key with address-only validation. */
    public static String canonicalizeStrict(String email) { return normalizeStrict(email).canonical(); }
}
