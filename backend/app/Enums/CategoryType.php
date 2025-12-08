<?php

namespace App\Enums;

enum CategoryType: string
{
    case GENERAL = 'general';
    case BLOG = 'blog';
    case PRODUCT = 'product';
    case NEWS = 'news';
    case DOCUMENTATION = 'documentation';
    case FAQ = 'faq';
}
