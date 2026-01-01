<?php

declare(strict_types=1);

namespace App\Contracts\Email;

use Illuminate\Http\Request;

/**
 * Contract for webhook signature verification.
 *
 * Implementations verify HMAC signatures from email providers.
 *
 * @version 1.0.0
 */
interface WebhookSignatureVerifierInterface
{
    /**
     * Verify the webhook request signature.
     *
     * @param Request $request The incoming webhook request
     * @return bool True if signature is valid
     * @throws \App\Exceptions\Email\InvalidSignatureException If verification fails
     */
    public function verify(Request $request): bool;

    /**
     * Get the provider name this verifier supports.
     *
     * @return string Provider name (postmark, ses, sendgrid)
     */
    public function getProvider(): string;

    /**
     * Check if signature verification is required.
     *
     * May return false in development environments.
     */
    public function isRequired(): bool;
}
