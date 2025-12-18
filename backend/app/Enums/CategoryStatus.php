<?php

namespace App\Enums;

enum CategoryStatus: string
{
    case ACTIVE = 'active';
    case INACTIVE = 'inactive';
    case DRAFT = 'draft';
    case ARCHIVED = 'archived';
}
