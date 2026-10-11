<?php
declare(strict_types=1);
namespace IronDome\MailCanon;

/** Bounded, provider-aware email normalization. */
final class MailCanon
{
    private const SPACE = " \t\n\r\x0c";
    private const CLEAN = [
        "\u{200b}" => '', "\u{200c}" => '', "\u{200d}" => '', "\u{feff}" => '', "\u{2060}" => '',
        "\u{ff0e}" => '.', "\u{3002}" => '.', "\u{ff61}" => '.', "\u{2024}" => '.',
        "\u{ff0b}" => '+', "\u{fe62}" => '+', "\u{ff20}" => '@',
    ];

    private function __construct() {}

    /** Return the normalized input domain, canonical address, and provider. */
    public static function normalize(mixed $email): EmailIdentity
    {
        if (!is_string($email)) { throw new \TypeError('email must be a string'); }
        if ($email === '' || strlen($email) > 512) { throw new InvalidEmail(); }
        if (preg_match('/[\x80-\xff]/', $email) === 0) {
            $value = trim($email, " \t\n\r\x0b\x0c");
        } else {
            if (preg_match('//u', $email) !== 1) { throw new InvalidEmail(); }
            $space = '[\x{0009}-\x{000d}\x{0020}\x{0085}\x{00a0}\x{1680}\x{2000}-\x{200a}\x{2028}\x{2029}\x{202f}\x{205f}\x{3000}]';
            $value = preg_replace('/\A' . $space . '+|' . $space . '+\z/u', '', $email);
            if ($value === null) { throw new InvalidEmail(); }
            $value = trim(strtr($value, self::CLEAN), self::SPACE);
        }
        if (strncasecmp($value, 'mailto:', 7) === 0) {
            $value = trim(substr($value, 7), self::SPACE);
            $value = trim(substr($value, 0, strcspn($value, '?#')), self::SPACE);
        }
        $left = strpos($value, '<');
        $right = strrpos($value, '>');
        if ($left !== false && $right !== false) {
            if ($left >= $right) { throw new InvalidEmail(); }
            $value = trim(substr($value, $left + 1, $right - $left - 1), self::SPACE);
        }
        $at = strpos($value, '@');
        if ($at === false || strpos($value, '@', $at + 1) !== false) { throw new InvalidEmail(); }
        $local = trim(substr($value, 0, $at), self::SPACE);
        $domain = trim(substr($value, $at + 1), self::SPACE);
        if ($local === '' || strlen($local) > 64 || $domain === '' || preg_match('/\A[A-Za-z0-9._%+\-]+\z/', $local) !== 1) { throw new InvalidEmail(); }
        if ($local[0] === '.' || str_ends_with($local, '.') || str_contains($local, '..')) { throw new InvalidEmail(); }
        if (preg_match('/[^\x00-\x7f]/', $domain) === 1) {
            if (!function_exists('idn_to_ascii')) { throw new \LogicException('mail-canon requires ext-intl for Unicode domains'); }
            $info = [];
            idn_to_ascii($domain, IDNA_NONTRANSITIONAL_TO_ASCII | IDNA_USE_STD3_RULES | IDNA_CHECK_BIDI | IDNA_CHECK_CONTEXTJ, INTL_IDNA_VARIANT_UTS46, $info);
            if (!isset($info['result'], $info['errors']) || ($info['errors'] & ~IDNA_ERROR_HYPHEN_3_4) !== 0) { throw new InvalidEmail(); }
            $domain = $info['result'];
        }
        if (str_ends_with($domain, '.')) { $domain = substr($domain, 0, -1); }
        $domain = strtolower($domain);
        if ($domain === '' || strlen($domain) > 253 || !str_contains($domain, '.') || preg_match('/\A[a-z0-9.\-]+\z/', $domain) !== 1) { throw new InvalidEmail(); }
        foreach (explode('.', $domain) as $label) {
            if ($label === '' || strlen($label) > 63 || $label[0] === '-' || str_ends_with($label, '-')) { throw new InvalidEmail(); }
        }
        $provider = match ($domain) {
            'gmail.com', 'googlemail.com' => 'gmail',
            'outlook.com', 'hotmail.com', 'live.com', 'msn.com' => 'microsoft',
            'icloud.com', 'me.com', 'mac.com' => 'icloud',
            'proton.me', 'protonmail.com', 'pm.me' => 'proton',
            'yahoo.com', 'ymail.com', 'rocketmail.com' => 'yahoo',
            default => 'custom',
        };
        $mapped = match ($provider) { 'gmail' => 'gmail.com', 'icloud' => 'icloud.com', default => $domain };
        $local = strtolower($local);
        if ($provider !== 'custom' && $provider !== 'yahoo') {
            $plus = strpos($local, '+');
            if ($plus !== false) { $local = substr($local, 0, $plus); }
        }
        if ($provider === 'gmail') {
            if (preg_match('/\A[a-z0-9.]+\z/', $local) !== 1) { throw new InvalidEmail(); }
            $local = str_replace('.', '', $local);
        }
        if ($local === '' || str_ends_with($local, '.')) { throw new InvalidEmail(); }
        $canonical = $local . '@' . $mapped;
        if (strlen($domain) + strlen($canonical) > 512) { throw new InvalidEmail(); }
        return new EmailIdentity($domain, $canonical, $provider);
    }

    /** Return only the canonical address with normalize's validation. */
    public static function canonicalize(mixed $email): string
    {
        return self::normalize($email)->canonical;
    }
}
