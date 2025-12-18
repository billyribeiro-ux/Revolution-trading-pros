<?php

declare(strict_types=1);

namespace App\Enums;

enum ContentType: string
{
    case HTML = 'html';
    case TEXT = 'text';
    case BOTH = 'both';
    case MARKDOWN = 'markdown';
}
