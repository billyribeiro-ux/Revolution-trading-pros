<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailSettingStatus: string
{
    case ACTIVE = 'active';
    case INACTIVE = 'inactive';
    case TESTING = 'testing';
    case ERROR = 'error';
}
