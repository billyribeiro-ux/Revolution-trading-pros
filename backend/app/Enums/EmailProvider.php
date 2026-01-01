<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailProvider: string
{
    case SMTP = 'smtp';
    case SENDGRID = 'sendgrid';
    case MAILGUN = 'mailgun';
    case POSTMARK = 'postmark';
    case SES = 'ses';
    case GMAIL = 'gmail';
    case OUTLOOK = 'outlook';
    case DEFAULT = 'smtp';
}
