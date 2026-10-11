<?php
declare(strict_types=1);
namespace IronDome\MailCanon;

final class InvalidEmail extends \InvalidArgumentException
{
    public function __construct() { parent::__construct('invalid email'); }
}
