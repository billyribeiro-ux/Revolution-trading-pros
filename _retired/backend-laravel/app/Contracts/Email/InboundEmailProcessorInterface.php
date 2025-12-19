<?php

declare(strict_types=1);

namespace App\Contracts\Email;

use App\DataTransferObjects\Email\InboundEmailPayload;
use App\Models\EmailMessage;

/**
 * Contract for inbound email processing.
 *
 * Allows for multiple implementations (real, mock, logged)
 * and facilitates testing with dependency injection.
 *
 * @version 1.0.0
 */
interface InboundEmailProcessorInterface
{
    /**
     * Process an inbound email payload.
     *
     * @param InboundEmailPayload $payload Normalized email payload
     * @return EmailMessage The created email message
     * @throws \App\Exceptions\Email\InboundEmailException On processing failure
     */
    public function process(InboundEmailPayload $payload): EmailMessage;

    /**
     * Check if the processor supports a given provider.
     *
     * @param string $provider Provider name (postmark, ses, sendgrid)
     */
    public function supportsProvider(string $provider): bool;
}
