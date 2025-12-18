<?php

namespace App\Enums;

enum SubscriptionStatus: string
{
    case Active = 'active';
    case Pending = 'pending';
    case OnHold = 'on-hold';
    case Cancelled = 'cancelled';
    case Expired = 'expired';
    case PendingCancel = 'pending-cancel';
    case Trial = 'trial';
}
