<?php

declare(strict_types=1);

namespace App\Observers;

use App\Models\FormSubmission;
use App\Services\Integration\IntegrationHub;
use App\Services\Integration\WebSocketService;
use Illuminate\Support\Facades\Log;

/**
 * FormSubmissionObserver - Wires form submissions to the ecosystem
 *
 * Triggers:
 * - CRM contact creation/update
 * - Automation workflows
 * - WebSocket notifications
 * - Analytics tracking
 */
class FormSubmissionObserver
{
    public function __construct(
        private readonly IntegrationHub $integrationHub,
        private readonly WebSocketService $webSocketService
    ) {}

    /**
     * Handle the FormSubmission "created" event.
     */
    public function created(FormSubmission $submission): void
    {
        try {
            // Process through the integration hub
            $this->integrationHub->processFormSubmission($submission);

            // Broadcast to admin dashboard
            $this->webSocketService->broadcastFormSubmission(
                $submission->form_id,
                [
                    'id' => $submission->id,
                    'submission_id' => $submission->submission_id,
                    'created_at' => $submission->created_at->toIso8601String(),
                    'status' => $submission->status,
                    'form_name' => $submission->form?->name,
                ]
            );

            Log::info('FormSubmissionObserver: Processed submission', [
                'submission_id' => $submission->id,
            ]);
        } catch (\Exception $e) {
            Log::error('FormSubmissionObserver: Failed to process submission', [
                'submission_id' => $submission->id,
                'error' => $e->getMessage(),
            ]);
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
}
