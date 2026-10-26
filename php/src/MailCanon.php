<?php
declare(strict_types=1);
namespace IronDome\MailCanon;

/** A simple package to normalize and canonicalise email addresses across Gmail, Outlook, Yahoo, and more, by the irondome.dev team. */
final class MailCanon
{
    private const PROTON_SEPARATORS = ['.', '_', '-'];

    private const SPACE = " \t\n\r\x0c";
    private const CLEAN = [
        "\u{200b}" => '', "\u{200c}" => '', "\u{200d}" => '', "\u{feff}" => '', "\u{2060}" => '',
        "\u{ff0e}" => '.', "\u{3002}" => '.', "\u{ff61}" => '.', "\u{2024}" => '.',
        "\u{ff0b}" => '+', "\u{fe62}" => '+', "\u{ff20}" => '@',
    ];
    /** Exact first-party Fastmail domains from the upstream documentation list. */
    private static array $fastmailDomains = [
        '123mail.org' => true, '150mail.com' => true, '150ml.com' => true, '16mail.com' => true, '2-mail.com' => true, '4email.net' => true, '50mail.com' => true,
        'airpost.net' => true, 'aliencamel.com' => true, 'allmail.net' => true, 'bestmail.us' => true, 'cluemail.com' => true, 'elitemail.org' => true,
        'emailcorner.net' => true, 'emailengine.net' => true, 'emailengine.org' => true, 'emailgroups.net' => true, 'emailplus.org' => true, 'emailuser.net' => true,
        'eml.cc' => true, 'f-m.fm' => true, 'fast-email.com' => true, 'fast-mail.org' => true, 'fastem.com' => true, 'fastemail.us' => true,
        'fastemailer.com' => true, 'fastest.cc' => true, 'fastimap.com' => true, 'fastmail.ca' => true, 'fastmail.cn' => true, 'fastmail.co.uk' => true,
        'fastmail.com' => true, 'fastmail.com.au' => true, 'fastmail.de' => true, 'fastmail.es' => true, 'fastmail.fm' => true, 'fastmail.fr' => true,
        'fastmail.im' => true, 'fastmail.in' => true, 'fastmail.jp' => true, 'fastmail.mx' => true, 'fastmail.net' => true, 'fastmail.nl' => true,
        'fastmail.org' => true, 'fastmail.se' => true, 'fastmail.to' => true, 'fastmail.tw' => true, 'fastmail.uk' => true, 'fastmail.us' => true,
        'fastmailbox.net' => true, 'fastmessaging.com' => true, 'fea.st' => true, 'fmail.co.uk' => true, 'fmailbox.com' => true, 'fmgirl.com' => true,
        'fmguy.com' => true, 'foobox.com' => true, 'foobox.net' => true, 'ftml.net' => true, 'h-mail.us' => true, 'hailmail.net' => true,
        'imap-mail.com' => true, 'imap.cc' => true, 'imapmail.org' => true, 'immerbox.com' => true, 'immermail.com' => true, 'inoutbox.com' => true,
        'internet-e-mail.com' => true, 'internet-mail.org' => true, 'internetemails.net' => true, 'internetmailing.net' => true, 'jetemail.net' => true, 'justemail.net' => true,
        'letterboxes.org' => true, 'lifetimeaddress.com' => true, 'mail-central.com' => true, 'mail-page.com' => true, 'mailas.com' => true, 'mailbolt.com' => true,
        'mailc.net' => true, 'mailcan.com' => true, 'mailforce.net' => true, 'mailhaven.com' => true, 'mailingaddress.org' => true, 'mailite.com' => true,
        'mailmight.com' => true, 'mailnew.com' => true, 'mailsent.net' => true, 'mailservice.ms' => true, 'mailup.net' => true, 'mailworks.org' => true,
        'mailzone.com' => true, 'ml1.net' => true, 'mm.st' => true, 'myfastmail.com' => true, 'mymacmail.com' => true, 'nospammail.net' => true,
        'onepost.net' => true, 'ownmail.net' => true, 'penguinmail.com' => true, 'permanentmail.com' => true, 'petml.com' => true, 'pobox.com' => true,
        'postinbox.com' => true, 'postpro.net' => true, 'proinbox.com' => true, 'promessage.com' => true, 'realemail.net' => true, 'reallyfast.biz' => true,
        'reallyfast.info' => true, 'rushpost.com' => true, 'sent.as' => true, 'sent.at' => true, 'sent.com' => true, 'siemprebox.com' => true,
        'siempremail.com' => true, 'speedpost.net' => true, 'speedymail.org' => true, 'ssl-mail.com' => true, 'swift-mail.com' => true, 'the-fastest.net' => true,
        'the-quickest.com' => true, 'theinternetemail.com' => true, 'veribox.net' => true, 'veryfast.biz' => true, 'veryspeedy.net' => true, 'warpmail.net' => true,
        'xsmail.com' => true, 'yepmail.net' => true, 'your-mail.com' => true,
    ];

    private function __construct() {}

    /** Return the normalized input domain, canonical address, and provider. */
    public static function normalize(mixed $email): EmailIdentity
    {
        return self::parse($email, false);
    }

    /** Normalize one bare address without wrappers, whitespace or cleanup. */
    public static function normalizeStrict(mixed $email): EmailIdentity
    {
        return self::parse($email, true);
    }

    private static function parse(mixed $email, bool $strict): EmailIdentity
    {
        if (!is_string($email)) { throw new \TypeError('email must be a string'); }
        if ($email === '' || strlen($email) > 512) { throw new InvalidEmail(); }
        if (preg_match('//u', $email) !== 1 || preg_match('/[\x{0000}-\x{001f}\x{007f}-\x{009f}\x{2028}\x{2029}]/u', $email) === 1) { throw new InvalidEmail(); }
        if ($strict && preg_match('/[\x{0020}\x{00a0}\x{1680}\x{2000}-\x{200d}\x{2024}\x{202f}\x{205f}\x{2060}\x{3000}\x{3002}\x{feff}\x{fe62}\x{ff0b}\x{ff0e}\x{ff20}\x{ff61}<>:?#]/u', $email) === 1) { throw new InvalidEmail(); }
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
        if ($left !== false || $right !== false) {
            if ($left === false || $right === false || $left >= $right || $right !== strlen($value) - 1
                || strpbrk(substr($value, 0, $left), '@<>,;:') !== false
                || strpbrk(substr($value, $left + 1, $right - $left - 1), '<>') !== false) { throw new InvalidEmail(); }
            $value = trim(substr($value, $left + 1, $right - $left - 1), self::SPACE);
        }
        $at = strpos($value, '@');
        if ($at === false || strpos($value, '@', $at + 1) !== false) { throw new InvalidEmail(); }
        $local = trim(substr($value, 0, $at), self::SPACE);
        $domain = trim(substr($value, $at + 1), self::SPACE);
        if ($local === '' || strlen($local) > 64 || $domain === '' || preg_match('/\A[A-Za-z0-9._%+\-]+\z/', $local) !== 1) { throw new InvalidEmail(); }
        if ($local[0] === '.' || str_ends_with($local, '.') || str_contains($local, '..')) { throw new InvalidEmail(); }
        if (preg_match('/[^\x00-\x7f]/', $domain) === 1 || preg_match('/(?:\A|\.)xn--/i', $domain) === 1) {
            if (!function_exists('idn_to_ascii')) { throw new \LogicException('mail-canon requires ext-intl for IDNA domains'); }
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
            'proton.me', 'protonmail.com', 'protonmail.ch', 'pm.me' => 'proton',
            'yahoo.com', 'ymail.com', 'rocketmail.com' => 'yahoo',
            default => 'custom',
        };
        $fastmailSubdomain = '';
        if ($provider === 'custom') {
            if (isset(self::$fastmailDomains[$domain])) {
                $provider = 'fastmail';
            } else {
                $dot = strpos($domain, '.');
                if ($dot !== false && $dot > 0 && strrpos($domain, '.') > $dot) {
                    $root = substr($domain, $dot + 1);
                    if (isset(self::$fastmailDomains[$root])) {
                        $provider = 'fastmail';
                        $mappedRoot = $root;
                        $fastmailSubdomain = substr($domain, 0, $dot);
                    }
                }
            }
        }
        $mapped = match ($provider) { 'gmail' => 'gmail.com', 'icloud' => 'icloud.com', default => $domain };
        if (isset($mappedRoot)) { $mapped = $mappedRoot; }
        $local = strtolower($local);
        if ($provider === 'fastmail' && $fastmailSubdomain !== '') {
            $local = $fastmailSubdomain;
        } elseif ($provider !== 'custom' && $provider !== 'yahoo') {
            $plus = strpos($local, '+');
            if ($plus !== false) { $local = substr($local, 0, $plus); }
        }
        if ($provider === 'gmail') {
            if (preg_match('/\A[a-z0-9.]+\z/', $local) !== 1) { throw new InvalidEmail(); }
            $local = str_replace('.', '', $local);
        }
        if ($provider === 'proton') { $local = str_replace(self::PROTON_SEPARATORS, '', $local); }
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

    /** Return a matching key with address-only validation. */
    public static function canonicalizeStrict(mixed $email): string
    {
        return self::normalizeStrict($email)->canonical;
    }
}
