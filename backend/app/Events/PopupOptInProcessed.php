<?php

declare(strict_types=1);

namespace App\Events;

use App\Models\Popup;
use App\Models\Contact;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Queue\SerializesModels;

/**
 * PopupOptInProcessed Event
 *
 * Fired when a popup opt-in is successfully processed through FluentCRM.
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class PopupOptInProcessed
{
    use Dispatchable, SerializesModels;

    public function __construct(
        public readonly Popup $popup,
        public readonly Contact $contact,
        public readonly array $result,
    ) {}
}
