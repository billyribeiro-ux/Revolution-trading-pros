<?php

declare(strict_types=1);

namespace App\Enums;

enum ComplaintType: string
{
    case SPAM = 'spam';
    case ABUSE = 'abuse';
    case FRAUD = 'fraud';
    case OTHER = 'other';
}
