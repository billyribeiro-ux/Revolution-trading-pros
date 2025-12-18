<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\NewsletterSubscription;
use App\Models\Contact;
use App\Services\Integration\ConsentService;
use App\Services\Integration\IntegrationHub;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Str;
use Carbon\Carbon;

/**
 * DoubleOptInService - GDPR-Compliant Email Subscription
 *
 * Matches FluentCRM Pro double opt-in features:
 * - Email verification with secure tokens
 * - Customizable confirmation emails
 * - Expiring verification links
 * - Consent tracking
 * - Welcome email sequences
 * - Preference center integration
 */
class DoubleOptInService
{
    private const TOKEN_TTL = 48 * 3600; // 48 hours
    private const CACHE_PREFIX = 'doi:';

    public function __construct(
        private readonly EmailService $emailService,
        private readonly ConsentService $consentService,
        private readonly IntegrationHub $integrationHub
    ) {}

    /**
     * Initiate double opt-in process
     */
    public function initiateOptIn(array $data): array
    {
        $email = $data['email'];
        $source = $data['source'] ?? 'website';
        $lists = $data['lists'] ?? [];
        $customFields = $data['custom_fields'] ?? [];

        // Check if already subscribed
        $existing = NewsletterSubscription::where('email', $email)->first();
        if ($existing && $existing->status === 'active') {
            return [
                'success' => false,
                'message' => 'This email is already subscribed.',
                'status' => 'already_subscribed',
            ];
        }

        // Generate verification token
        $token = $this->generateVerificationToken();
        $tokenHash = hash('sha256', $token);

        // Create or update subscription
        $subscription = NewsletterSubscription::updateOrCreate(
            ['email' => $email],
            [
                'status' => 'pending',
                'first_name' => $data['first_name'] ?? null,
                'last_name' => $data['last_name'] ?? null,
                'verification_token' => $tokenHash,
                'double_opt_in' => true,
                'source' => $source,
                'consent_ip' => request()->ip(),
                'consent_method' => 'double_opt_in',
                'subscription_preferences' => $lists,
                'metadata' => array_merge($customFields, [
                    'signup_date' => now()->toIso8601String(),
                    'signup_url' => request()->url(),
                ]),
            ]
        );

        // Store token with expiry for verification
        Cache::put(
            self::CACHE_PREFIX . $tokenHash,
            [
                'email' => $email,
                'subscription_id' => $subscription->id,
                'created_at' => now()->toIso8601String(),
            ],
            self::TOKEN_TTL
        );

        // Send confirmation email
        $this->sendConfirmationEmail($subscription, $token);

        Log::info('DoubleOptInService: Initiated opt-in', [
            'email' => $this->maskEmail($email),
            'source' => $source,
        ]);

        return [
            'success' => true,
            'message' => 'Please check your email to confirm your subscription.',
            'status' => 'pending_verification',
            'subscription_id' => $subscription->id,
        ];
    }

    /**
     * Verify subscription with token
     */
    public function verifyOptIn(string $token): array
    {
        $tokenHash = hash('sha256', $token);
        $tokenData = Cache::get(self::CACHE_PREFIX . $tokenHash);

        if (!$tokenData) {
            return [
                'success' => false,
                'message' => 'Invalid or expired verification link.',
                'status' => 'invalid_token',
            ];
        }

        $subscription = NewsletterSubscription::find($tokenData['subscription_id']);

        if (!$subscription) {
            return [
                'success' => false,
                'message' => 'Subscription not found.',
                'status' => 'not_found',
            ];
        }

        if ($subscription->status === 'active') {
            return [
                'success' => true,
                'message' => 'Your subscription is already confirmed.',
                'status' => 'already_verified',
            ];
        }

        // Activate subscription
        $subscription->update([
            'status' => 'active',
            'verified_at' => now(),
            'consent_given_at' => now(),
        ]);

        // Record consent
        $this->consentService->recordConsent(
            $subscription->email,
            ConsentService::PURPOSE_NEWSLETTER,
            true,
            [
                'source' => 'double_opt_in',
                'ip' => $subscription->consent_ip,
                'verified_at' => now()->toIso8601String(),
            ]
        );

        // Clear token
        Cache::forget(self::CACHE_PREFIX . $tokenHash);

        // Process through integration hub
        $this->integrationHub->processNewsletterSubscription($subscription, 'confirmed');

        // Send welcome email
        $this->sendWelcomeEmail($subscription);

        Log::info('DoubleOptInService: Subscription verified', [
            'email' => $this->maskEmail($subscription->email),
        ]);

        return [
            'success' => true,
            'message' => 'Your subscription has been confirmed. Welcome!',
            'status' => 'verified',
        ];
    }

    /**
     * Resend confirmation email
     */
    public function resendConfirmation(string $email): array
    {
        $subscription = NewsletterSubscription::where('email', $email)
            ->where('status', 'pending')
            ->first();

        if (!$subscription) {
            return [
                'success' => false,
                'message' => 'No pending subscription found for this email.',
                'status' => 'not_found',
            ];
        }

        // Check rate limiting (max 3 resends per hour)
        $resendKey = "doi:resend:{$email}";
        $resendCount = Cache::get($resendKey, 0);

        if ($resendCount >= 3) {
            return [
                'success' => false,
                'message' => 'Too many resend requests. Please try again later.',
                'status' => 'rate_limited',
            ];
        }

        // Generate new token
        $token = $this->generateVerificationToken();
        $tokenHash = hash('sha256', $token);

        // Update subscription with new token
        $subscription->update(['verification_token' => $tokenHash]);

        // Store new token
        Cache::put(
            self::CACHE_PREFIX . $tokenHash,
            [
                'email' => $email,
                'subscription_id' => $subscription->id,
                'created_at' => now()->toIso8601String(),
            ],
            self::TOKEN_TTL
        );

        // Send confirmation email
        $this->sendConfirmationEmail($subscription, $token);

        // Update rate limit
        Cache::put($resendKey, $resendCount + 1, 3600);

        return [
            'success' => true,
            'message' => 'Confirmation email has been resent.',
            'status' => 'resent',
        ];
    }

    /**
     * Unsubscribe with token
     */
    public function unsubscribe(string $token, ?string $reason = null): array
    {
        $subscription = NewsletterSubscription::where('unsubscribe_token', $token)->first();

        if (!$subscription) {
            // Try finding by email hash if token looks like email
            if (filter_var($token, FILTER_VALIDATE_EMAIL)) {
                $subscription = NewsletterSubscription::where('email', $token)->first();
            }
        }

        if (!$subscription) {
            return [
                'success' => false,
                'message' => 'Subscription not found.',
                'status' => 'not_found',
            ];
        }

        $subscription->update([
            'status' => 'unsubscribed',
            'unsubscribed_at' => now(),
            'unsubscribe_reason' => $reason,
        ]);

        // Record consent withdrawal
        $this->consentService->withdrawConsent(
            $subscription->email,
            ConsentService::PURPOSE_NEWSLETTER
        );

        // Process through integration hub
        $this->integrationHub->processNewsletterSubscription($subscription, 'unsubscribed');

        Log::info('DoubleOptInService: Unsubscribed', [
            'email' => $this->maskEmail($subscription->email),
            'reason' => $reason,
        ]);

        return [
            'success' => true,
            'message' => 'You have been successfully unsubscribed.',
            'status' => 'unsubscribed',
        ];
    }

    /**
     * Get subscription preferences
     */
    public function getPreferences(string $email): ?array
    {
        $subscription = NewsletterSubscription::where('email', $email)->first();

        if (!$subscription) {
            return null;
        }

        return [
            'email' => $email,
            'status' => $subscription->status,
            'preferences' => $subscription->subscription_preferences ?? [],
            'verified' => $subscription->verified_at !== null,
            'subscribed_at' => $subscription->consent_given_at?->toIso8601String(),
        ];
    }

    /**
     * Update subscription preferences
     */
    public function updatePreferences(string $email, array $preferences): array
    {
        $subscription = NewsletterSubscription::where('email', $email)->first();

        if (!$subscription) {
            return [
                'success' => false,
                'message' => 'Subscription not found.',
            ];
        }

        $subscription->update([
            'subscription_preferences' => $preferences,
        ]);

        return [
            'success' => true,
            'message' => 'Preferences updated successfully.',
        ];
    }

    /**
     * Generate secure verification token
     */
    private function generateVerificationToken(): string
    {
        return Str::random(64);
    }

    /**
     * Send confirmation email
     */
    private function sendConfirmationEmail(NewsletterSubscription $subscription, string $token): void
    {
        $confirmUrl = config('app.frontend_url') . '/newsletter/confirm?token=' . $token;

        $this->emailService->send([
            'to' => $subscription->email,
            'subject' => 'Please Confirm Your Subscription',
            'template' => 'emails.newsletter.confirm',
            'data' => [
                'first_name' => $subscription->first_name ?? 'there',
                'confirm_url' => $confirmUrl,
                'expires_in' => '48 hours',
            ],
        ]);
    }

    /**
     * Send welcome email
     */
    private function sendWelcomeEmail(NewsletterSubscription $subscription): void
    {
        $preferencesUrl = config('app.frontend_url') . '/newsletter/preferences?email=' . urlencode($subscription->email);
        $unsubscribeUrl = config('app.frontend_url') . '/newsletter/unsubscribe?token=' . $subscription->unsubscribe_token;

        $this->emailService->send([
            'to' => $subscription->email,
            'subject' => 'Welcome to Our Newsletter!',
            'template' => 'emails.newsletter.welcome',
            'data' => [
                'first_name' => $subscription->first_name ?? 'there',
                'preferences_url' => $preferencesUrl,
                'unsubscribe_url' => $unsubscribeUrl,
            ],
        ]);
    }

    /**
     * Mask email for logging
     */
    private function maskEmail(string $email): string
    {
        $parts = explode('@', $email);
        $name = $parts[0];
        $domain = $parts[1] ?? '';
        $masked = substr($name, 0, 2) . str_repeat('*', max(0, strlen($name) - 2));
        return $masked . '@' . $domain;
    }
}
