<?php

declare(strict_types=1);

namespace App\Services\Forms;

use App\Models\Form;
use App\Models\FormSubmission;
use App\Services\EmailService;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Carbon\Carbon;

/**
 * FormDoubleOptInService - FluentForms 6.1.8 (December 2025)
 *
 * Double opt-in verification for form submissions.
 * Sends verification emails and tracks confirmation status.
 * GDPR compliant with consent tracking.
 *
 * Features:
 * - Email verification with secure tokens
 * - Configurable expiration times
 * - Resend with rate limiting
 * - Consent tracking for GDPR
 * - Custom email templates per form
 */
class FormDoubleOptInService
{
    private const TOKEN_TTL = 86400; // 24 hours
    private const RESEND_COOLDOWN = 60; // 60 seconds
    private const MAX_RESENDS = 5;
    private const CACHE_PREFIX = 'form_doi:';

    public function __construct(
        private readonly EmailService $emailService
    ) {}

    /**
     * Initiate double opt-in for a submission
     */
    public function initiateVerification(FormSubmission $submission, string $email): array
    {
        // Get form settings
        $form = $submission->form;
        $settings = $form->settings ?? [];
        $doiSettings = $settings['double_opt_in'] ?? [];

        if (!($doiSettings['enabled'] ?? false)) {
            return [
                'success' => true,
                'message' => 'Double opt-in not required.',
                'status' => 'not_required',
            ];
        }

        // Check if already verified
        $existingStatus = $submission->getMetadata('doi_status');
        if ($existingStatus === 'confirmed') {
            return [
                'success' => true,
                'message' => 'Email already verified.',
                'status' => 'already_verified',
            ];
        }

        // Generate verification token
        $token = $this->generateToken();
        $tokenHash = hash('sha256', $token);
        $expiresAt = now()->addSeconds($doiSettings['expiration_hours'] ?? 24 * 3600);

        // Store token data
        $tokenData = [
            'submission_id' => $submission->id,
            'form_id' => $form->id,
            'email' => $email,
            'created_at' => now()->toIso8601String(),
            'expires_at' => $expiresAt->toIso8601String(),
        ];

        Cache::put(
            self::CACHE_PREFIX . 'token:' . $tokenHash,
            $tokenData,
            $expiresAt
        );

        // Update submission metadata
        $submission->addMetadata('doi_status', 'pending');
        $submission->addMetadata('doi_sent_at', now()->toIso8601String());
        $submission->addMetadata('doi_expires_at', $expiresAt->toIso8601String());
        $submission->addMetadata('doi_email', $email);
        $submission->addMetadata('doi_attempts', 1);

        // Send verification email
        $this->sendVerificationEmail($submission, $form, $email, $token, $expiresAt);

        Log::info('FormDoubleOptInService: Verification initiated', [
            'submission_id' => $submission->id,
            'email' => $this->maskEmail($email),
        ]);

        return [
            'success' => true,
            'message' => 'Verification email sent. Please check your inbox.',
            'status' => 'sent',
            'expires_at' => $expiresAt->toIso8601String(),
        ];
    }

    /**
     * Verify submission with token
     */
    public function verify(string $token): array
    {
        $tokenHash = hash('sha256', $token);
        $tokenData = Cache::get(self::CACHE_PREFIX . 'token:' . $tokenHash);

        if (!$tokenData) {
            return [
                'success' => false,
                'message' => 'Invalid or expired verification link.',
                'status' => 'invalid_token',
            ];
        }

        // Check expiration
        $expiresAt = Carbon::parse($tokenData['expires_at']);
        if (now()->isAfter($expiresAt)) {
            Cache::forget(self::CACHE_PREFIX . 'token:' . $tokenHash);
            return [
                'success' => false,
                'message' => 'Verification link has expired. Please request a new one.',
                'status' => 'expired',
            ];
        }

        $submission = FormSubmission::find($tokenData['submission_id']);
        if (!$submission) {
            return [
                'success' => false,
                'message' => 'Submission not found.',
                'status' => 'not_found',
            ];
        }

        // Update submission status
        $submission->addMetadata('doi_status', 'confirmed');
        $submission->addMetadata('doi_confirmed_at', now()->toIso8601String());
        $submission->addMetadata('doi_consent_ip', request()?->ip());

        // Clear token
        Cache::forget(self::CACHE_PREFIX . 'token:' . $tokenHash);

        // Process confirmed submission (trigger notifications, integrations, etc.)
        $this->processConfirmedSubmission($submission);

        Log::info('FormDoubleOptInService: Submission verified', [
            'submission_id' => $submission->id,
        ]);

        return [
            'success' => true,
            'message' => 'Your email has been verified successfully!',
            'status' => 'confirmed',
            'submission_id' => $submission->submission_id,
        ];
    }

    /**
     * Resend verification email
     */
    public function resend(int $submissionId): array
    {
        $submission = FormSubmission::find($submissionId);
        if (!$submission) {
            return [
                'success' => false,
                'message' => 'Submission not found.',
            ];
        }

        $doiStatus = $submission->getMetadata('doi_status');
        if ($doiStatus === 'confirmed') {
            return [
                'success' => false,
                'message' => 'Email is already verified.',
            ];
        }

        // Check rate limiting
        $rateLimitKey = self::CACHE_PREFIX . 'rate:' . $submissionId;
        $lastSent = Cache::get($rateLimitKey);

        if ($lastSent) {
            $waitTime = self::RESEND_COOLDOWN - now()->diffInSeconds(Carbon::parse($lastSent));
            if ($waitTime > 0) {
                return [
                    'success' => false,
                    'message' => "Please wait {$waitTime} seconds before requesting another email.",
                    'wait_seconds' => $waitTime,
                ];
            }
        }

        // Check max attempts
        $attempts = (int) ($submission->getMetadata('doi_attempts') ?? 0);
        if ($attempts >= self::MAX_RESENDS) {
            return [
                'success' => false,
                'message' => 'Maximum resend attempts reached. Please contact support.',
            ];
        }

        $email = $submission->getMetadata('doi_email');
        if (!$email) {
            $email = $submission->getFieldValue('email');
        }

        if (!$email) {
            return [
                'success' => false,
                'message' => 'No email address found for this submission.',
            ];
        }

        // Generate new token
        $form = $submission->form;
        $settings = $form->settings ?? [];
        $doiSettings = $settings['double_opt_in'] ?? [];

        $token = $this->generateToken();
        $tokenHash = hash('sha256', $token);
        $expiresAt = now()->addSeconds($doiSettings['expiration_hours'] ?? 24 * 3600);

        Cache::put(
            self::CACHE_PREFIX . 'token:' . $tokenHash,
            [
                'submission_id' => $submission->id,
                'form_id' => $form->id,
                'email' => $email,
                'created_at' => now()->toIso8601String(),
                'expires_at' => $expiresAt->toIso8601String(),
            ],
            $expiresAt
        );

        // Update metadata
        $submission->addMetadata('doi_sent_at', now()->toIso8601String());
        $submission->addMetadata('doi_expires_at', $expiresAt->toIso8601String());
        $submission->addMetadata('doi_attempts', $attempts + 1);

        // Set rate limit
        Cache::put($rateLimitKey, now()->toIso8601String(), self::RESEND_COOLDOWN);

        // Send email
        $this->sendVerificationEmail($submission, $form, $email, $token, $expiresAt);

        return [
            'success' => true,
            'message' => 'Verification email has been resent.',
            'attempts' => $attempts + 1,
            'max_attempts' => self::MAX_RESENDS,
            'expires_at' => $expiresAt->toIso8601String(),
        ];
    }

    /**
     * Get verification status for a submission
     */
    public function getStatus(int $submissionId): array
    {
        $submission = FormSubmission::find($submissionId);
        if (!$submission) {
            return [
                'status' => 'not_found',
                'email' => null,
            ];
        }

        $status = $submission->getMetadata('doi_status') ?? 'pending';
        $email = $submission->getMetadata('doi_email') ?? $submission->getFieldValue('email');

        return [
            'status' => $status,
            'email' => $email,
            'sentAt' => $submission->getMetadata('doi_sent_at'),
            'confirmedAt' => $submission->getMetadata('doi_confirmed_at'),
            'expiresAt' => $submission->getMetadata('doi_expires_at'),
            'attempts' => (int) ($submission->getMetadata('doi_attempts') ?? 0),
        ];
    }

    /**
     * Check if form requires double opt-in
     */
    public function requiresVerification(Form $form): bool
    {
        $settings = $form->settings ?? [];
        return $settings['double_opt_in']['enabled'] ?? false;
    }

    /**
     * Configure double opt-in for a form
     */
    public function configureForm(int $formId, array $config): array
    {
        $form = Form::find($formId);
        if (!$form) {
            return ['success' => false, 'message' => 'Form not found'];
        }

        $settings = $form->settings ?? [];
        $settings['double_opt_in'] = [
            'enabled' => $config['enabled'] ?? false,
            'expiration_hours' => $config['expiration_hours'] ?? 24,
            'email_subject' => $config['email_subject'] ?? 'Please verify your submission',
            'email_template' => $config['email_template'] ?? null,
            'success_message' => $config['success_message'] ?? 'Your email has been verified!',
            'redirect_url' => $config['redirect_url'] ?? null,
        ];

        $form->settings = $settings;
        $form->save();

        return ['success' => true, 'message' => 'Double opt-in settings updated'];
    }

    // =========================================================================
    // PRIVATE METHODS
    // =========================================================================

    private function generateToken(): string
    {
        return Str::random(64);
    }

    private function sendVerificationEmail(
        FormSubmission $submission,
        Form $form,
        string $email,
        string $token,
        Carbon $expiresAt
    ): void {
        $settings = $form->settings ?? [];
        $doiSettings = $settings['double_opt_in'] ?? [];

        $verifyUrl = config('app.frontend_url') . '/forms/verify?token=' . $token;

        $this->emailService->send([
            'to' => $email,
            'subject' => $doiSettings['email_subject'] ?? "Please verify your submission for {$form->name}",
            'template' => $doiSettings['email_template'] ?? 'emails.forms.double-opt-in',
            'data' => [
                'form_name' => $form->name,
                'submission_id' => $submission->submission_id,
                'verify_url' => $verifyUrl,
                'expires_at' => $expiresAt->format('M d, Y \a\t g:i A'),
                'expires_in' => $expiresAt->diffForHumans(),
            ],
        ]);
    }

    private function processConfirmedSubmission(FormSubmission $submission): void
    {
        // Trigger form notifications
        event('form.submission.verified', $submission);

        // You could also trigger integrations here
        // e.g., add to CRM, email list, etc.
    }

    private function maskEmail(string $email): string
    {
        $parts = explode('@', $email);
        $name = $parts[0];
        $domain = $parts[1] ?? '';
        $masked = substr($name, 0, 2) . str_repeat('*', max(0, strlen($name) - 2));
        return $masked . '@' . $domain;
    }
}
