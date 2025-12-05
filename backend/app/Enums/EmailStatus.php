<?php

declare(strict_types=1);

namespace App\Enums;

enum EmailStatus: string
{
    case PENDING = 'pending';
    case QUEUED = 'queued';
    case SENT = 'sent';
    case DELIVERED = 'delivered';
    case OPENED = 'opened';
    case CLICKED = 'clicked';
    case BOUNCED = 'bounced';
    case FAILED = 'failed';
    case SPAM = 'spam';
    case UNSUBSCRIBED = 'unsubscribed';
}
