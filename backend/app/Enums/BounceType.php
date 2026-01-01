<?php

declare(strict_types=1);

namespace App\Enums;

enum BounceType: string
{
    case HARD = 'hard';
    case SOFT = 'soft';
    case TRANSIENT = 'transient';
    case UNKNOWN = 'unknown';
}
