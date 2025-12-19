<?php

declare(strict_types=1);

namespace App\Enums;

enum TemplateEngine: string
{
    case BLADE = 'blade';
    case TWIG = 'twig';
    case MARKDOWN = 'markdown';
    case HTML = 'html';
}
