<?php

declare(strict_types=1);

namespace App\Observers;

use App\Models\FormSubmission;
use App\Services\Integration\IntegrationHub;
use App\Services\Integration\WebSocketService;
use App\Services\Newsletter\FormNewsletterBridgeService;
use Illuminate\Support\Facades\Log;

/**
 * FormSubmissionObserver - Wires form submissions to the ecosystem
 *
 * Triggers:
 * - Newsletter subscription (via FormNewsletterBridgeService)
 * - CRM contact creation/update
 * - Automation workflows
 * - WebSocket notifications
 * - Analytics tracking
 *
 * @level ICT11 Principal Engineer
 */
class FormSubmissionObserver
{
    public function __construct(
        private readonly IntegrationHub $integrationHub,
        private readonly WebSocketService $webSocketService,
        private readonly FormNewsletterBridgeService $newsletterBridge
    ) {}

    /**
     * Handle the FormSubmission "created" event.
     */
    public function created(FormSubmission $submission): void
    {
        try {
            // 1. Process newsletter subscription if applicable
            $this->processNewsletterSubscription($submission);

            // 2. Process through the integration hub (CRM, automations, etc.)
            $this->integrationHub->processFormSubmission($submission);

            // 3. Broadcast to admin dashboard via WebSocket
            $this->webSocketService->broadcastFormSubmission(
                $submission->form_id,
                [
                    'id' => $submission->id,
                    'submission_id' => $submission->submission_id,
                    'created_at' => $submission->created_at->toIso8601String(),
                    'status' => $submission->status,
                    'form_name' => $submission->form?->name,
                    'has_newsletter' => $submission->newsletter_subscription_id !== null,
                ]
            );

            Log::info('FormSubmissionObserver: Processed submission', [
                'submission_id' => $submission->id,
                'newsletter_subscription_id' => $submission->newsletter_subscription_id,
            ]);
        } catch (\Exception $e) {
            Log::error('FormSubmissionObserver: Failed to process submission', [
                'submission_id' => $submission->id,
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);
        }
    }

    /**
     * Process newsletter subscription from form submission
     */
    private function processNewsletterSubscription(FormSubmission $submission): void
    {
        try {
            $subscription = $this->newsletterBridge->processSubmission($submission);

            if ($subscription) {
                Log::info('FormSubmissionObserver: Newsletter subscription created/updated', [
                    'submission_id' => $submission->id,
                    'subscription_id' => $subscription->id,
                    'email' => $subscription->email,
                    'status' => $subscription->status,
                ]);

                // Fire event for further processing (e.g., send verification email)
                event(new \App\Events\NewsletterSubscriptionCreated($subscription, $submission));
            }
        } catch (\Exception $e) {
            Log::error('FormSubmissionObserver: Newsletter subscription failed', [
                'submission_id' => $submission->id,
                'error' => $e->getMessage(),
            ]);
            // Don't throw - newsletter failure shouldn't break form submission
        }
    }

    /**
     * Handle the FormSubmission "updated" event.
     */
    public function updated(FormSubmission $submission): void
    {
        // Broadcast status changes
        if ($submission->isDirty('status')) {
            $this->webSocketService->broadcastFormSubmission(
                $submission->form_id,
                [
                    'id' => $submission->id,
                    'submission_id' => $submission->submission_id,
                    'status' => $submission->status,
                    'updated_at' => $submission->updated_at->toIso8601String(),
                    'action' => 'status_changed',
                ]
            );
        }
    }

    /**
     * Handle the FormSubmission "deleted" event.
     */
    public function deleted(FormSubmission $submission): void
    {
        Log::info('FormSubmissionObserver: Submission deleted', [
            'submission_id' => $submission->id,
            'form_id' => $submission->form_id,
        ]);
    }
}
