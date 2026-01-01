<?php

declare(strict_types=1);

namespace App\Observers;

use App\Models\Contact;
use App\Services\Integration\IntegrationHub;
use App\Services\Integration\WebSocketService;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;

/**
 * ContactObserver - Wires CRM contacts to the ecosystem
 *
 * Triggers:
 * - Cache invalidation
 * - WebSocket updates for admin dashboard
 * - Segment recalculation
 * - Score change notifications
 */
class ContactObserver
{
    public function __construct(
        private readonly IntegrationHub $integrationHub,
        private readonly WebSocketService $webSocketService
    ) {}

    /**
     * Handle the Contact "created" event.
     */
    public function created(Contact $contact): void
    {
        // Broadcast to admin CRM dashboard
        $this->webSocketService->broadcastContactUpdate($contact->id, 'created', [
            'email' => $contact->email,
            'name' => trim("{$contact->first_name} {$contact->last_name}"),
            'source' => $contact->source,
        ]);
    }

    /**
     * Handle the Contact "updated" event.
     */
    public function updated(Contact $contact): void
    {
        // Invalidate cached profile
        $this->integrationHub->invalidateContactCache($contact->id);

        // Track significant changes
        $significantFields = ['lead_score', 'health_score', 'lifecycle_stage', 'is_vip', 'lifetime_value'];
        $changedSignificant = array_intersect($significantFields, array_keys($contact->getDirty()));

        if (!empty($changedSignificant)) {
            $this->webSocketService->broadcastContactUpdate($contact->id, 'significant_update', [
                'changed_fields' => $changedSignificant,
                'lead_score' => $contact->lead_score,
                'health_score' => $contact->health_score,
                'lifecycle_stage' => $contact->lifecycle_stage,
            ]);
        }

        // Score change notifications
        if ($contact->isDirty('lead_score')) {
            $oldScore = $contact->getOriginal('lead_score');
            $newScore = $contact->lead_score;

            // Significant score increase - might be sales ready
            if ($newScore >= 80 && $oldScore < 80) {
                $this->notifyScoreThreshold($contact, 'sales_ready');
            }
        }
    }

    /**
     * Handle the Contact "deleted" event.
     */
    public function deleted(Contact $contact): void
    {
        $this->integrationHub->invalidateContactCache($contact->id);

        $this->webSocketService->broadcastContactUpdate($contact->id, 'deleted', [
            'email' => $contact->email,
        ]);
    }

    /**
     * Notify about score threshold
     */
    private function notifyScoreThreshold(Contact $contact, string $threshold): void
    {
        // Could trigger automations or notifications here
        Log::info('ContactObserver: Score threshold reached', [
            'contact_id' => $contact->id,
            'threshold' => $threshold,
            'lead_score' => $contact->lead_score,
        ]);
    }
}
