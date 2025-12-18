<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailType: string
{
    case TRANSACTIONAL = 'transactional';
    case MARKETING = 'marketing';
    case NOTIFICATION = 'notification';
    case NEWSLETTER = 'newsletter';
    case WELCOME = 'welcome';
    case PASSWORD_RESET = 'password_reset';
    case VERIFICATION = 'verification';
    case INVOICE = 'invoice';
    case PROMOTIONAL = 'promotional';
}
