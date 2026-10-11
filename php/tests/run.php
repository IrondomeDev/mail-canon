<?php
declare(strict_types=1);
require __DIR__ . '/../src/InvalidEmail.php';
require __DIR__ . '/../src/EmailIdentity.php';
require __DIR__ . '/../src/MailCanon.php';

use IronDome\MailCanon\InvalidEmail;
use IronDome\MailCanon\MailCanon;

function check(bool $condition, string $name): void {
    if (!$condition) { throw new RuntimeException($name); }
}
$count = 0;
foreach (file(__DIR__ . '/../../testdata/vectors.tsv', FILE_IGNORE_NEW_LINES) as $row) {
    if (str_starts_with($row, '#')) { continue; }
    [$name, $encoded, $domain, $canonical, $provider] = explode("\t", $row);
    $raw = hex2bin($encoded);
    if ($domain === '-') {
        foreach (['normalize', 'canonicalize'] as $fn) {
            try { MailCanon::$fn($raw); throw new RuntimeException($name . ': accepted invalid input'); }
            catch (InvalidEmail) { }
        }
    } else {
        $value = MailCanon::normalize($raw);
        check([$value->domain, $value->canonical, $value->provider] === [$domain, $canonical, $provider], $name);
        check(MailCanon::canonicalize($raw) === $canonical, $name);
        check(MailCanon::canonicalize($canonical) === $canonical, $name . ': not idempotent');
    }
    $count++;
}
foreach ([null, 1, true, [], new stdClass()] as $value) {
    foreach (['normalize', 'canonicalize'] as $fn) {
        try { MailCanon::$fn($value); throw new RuntimeException('accepted non-string'); }
        catch (TypeError) { }
    }
}
foreach (["\xff@example.com", "name \xed\xa0\x80 <u@example.com>"] as $raw) {
    try { MailCanon::normalize($raw); throw new RuntimeException('accepted invalid UTF-8'); }
    catch (InvalidEmail) { }
}
echo "Passed $count vectors, type checks, and malformed UTF-8 checks.\n";
