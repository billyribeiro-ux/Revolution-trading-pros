<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailAuthMethod: string
{
    case PLAIN = 'plain';
    case LOGIN = 'login';
    case CRAM_MD5 = 'cram-md5';
    case OAUTH2 = 'oauth2';
    case API_KEY = 'api_key';
}
