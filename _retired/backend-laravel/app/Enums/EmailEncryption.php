<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailEncryption: string
{
    case NONE = 'none';
    case SSL = 'ssl';
    case TLS = 'tls';
    case STARTTLS = 'starttls';
}
