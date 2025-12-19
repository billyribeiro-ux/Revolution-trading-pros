<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailPriority: string
{
    case LOW = 'low';
    case NORMAL = 'normal';
    case HIGH = 'high';
    case URGENT = 'urgent';
}
