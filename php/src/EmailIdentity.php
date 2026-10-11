<?php
declare(strict_types=1);
namespace IronDome\MailCanon;

/** Normalized input domain, canonical address, and provider group. */
final readonly class EmailIdentity
{
    public function __construct(
        public string $domain,
        public string $canonical,
        public string $provider,
    ) {}
}
