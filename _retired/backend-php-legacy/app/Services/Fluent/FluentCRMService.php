<?php

declare(strict_types=1);

namespace App\Services\Fluent;

use App\Models\Contact;
use App\Models\CrmActivity;
use App\Models\EmailAutomationWorkflow;
use App\Services\Email\EmailAutomationService;
use App\Services\LeadScoringService;
use App\Services\HealthScoringService;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;

/**
 * FluentCRM Service - Contact Management & Email Marketing
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive CRM functionality matching FluentCRM Pro:
 * - Contact management and lifecycle tracking
 * - List and segment management
 * - Tagging system
 * - Email campaign management
 * - Marketing automation
 * - Consent management (GDPR)
 * - Timeline and activity tracking
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class FluentCRMService
{
    public function __construct(
        private readonly EmailAutomationService $automationService,
        private readonly LeadScoringService $leadScoringService,
        private readonly HealthScoringService $healthScoringService,
    ) {}

    // ═══════════════════════════════════════════════════════════════════════════
    // CONTACT MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Find or create a contact by email
     */
    public function findOrCreateContact(array $data): Contact
    {
        $email = strtolower(trim($data['email']));
        unset($data['email']);

        $contact = Contact::firstOrNew(['email' => $email]);

        // Only update fields that are not already set (preserve existing data)
        foreach ($data as $key => $value) {
            if ($key === 'custom_fields') {
                // Merge custom fields
                $existing = $contact->custom_fields ?? [];
                $contact->custom_fields = array_merge($existing, $value);
            } elseif (empty($contact->$key) && !empty($value)) {
                $contact->$key = $value;
            }
        }

        // Set default values for new contacts
        if (!$contact->exists) {
            $contact->status = $contact->status ?? 'active';
            $contact->lifecycle_stage = $contact->lifecycle_stage ?? 'lead';
            $contact->lead_score = $contact->lead_score ?? 0;
            $contact->health_score = $contact->health_score ?? 50;
            $contact->engagement_score = $contact->engagement_score ?? 0;
            $contact->value_score = $contact->value_score ?? 0;
        }

        $contact->save();

        return $contact;
    }

    /**
     * Create a new contact
     */
    public function createContact(array $data): Contact
    {
        return $this->findOrCreateContact($data);
    }

    /**
     * Find contact by email
     */
    public function findByEmail(string $email): ?Contact
    {
        return Contact::where('email', strtolower(trim($email)))->first();
    }

    /**
     * Update contact data
     */
    public function updateContact(Contact $contact, array $data): Contact
    {
        if (isset($data['custom_fields'])) {
            $existing = $contact->custom_fields ?? [];
            $data['custom_fields'] = array_merge($existing, $data['custom_fields']);
        }

        $contact->update($data);
        return $contact->fresh();
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TAGGING
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Apply tags to a contact
     */
    public function applyTags(Contact $contact, array $tags): void
    {
        $existingTags = $contact->tags ?? [];
        $newTags = array_unique(array_merge($existingTags, $tags));
        $contact->update(['tags' => $newTags]);
    }

    /**
     * Remove tags from a contact
     */
    public function removeTags(Contact $contact, array $tags): void
    {
        $existingTags = $contact->tags ?? [];
        $newTags = array_diff($existingTags, $tags);
        $contact->update(['tags' => array_values($newTags)]);
    }

    /**
     * Check if contact has specific tag
     */
    public function hasTag(Contact $contact, string $tag): bool
    {
        return in_array($tag, $contact->tags ?? []);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // LISTS & SEGMENTS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Add contact to lists
     */
    public function addToLists(Contact $contact, array $listIds): void
    {
        $contact->lists()->syncWithoutDetaching($listIds);
    }

    /**
     * Remove contact from lists
     */
    public function removeFromLists(Contact $contact, array $listIds): void
    {
        $contact->lists()->detach($listIds);
    }

    /**
     * Add contact to segments
     */
    public function addToSegments(Contact $contact, array $segmentIds): void
    {
        $contact->segments()->syncWithoutDetaching($segmentIds);
    }

    /**
     * Remove contact from segments
     */
    public function removeFromSegments(Contact $contact, array $segmentIds): void
    {
        $contact->segments()->detach($segmentIds);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AUTOMATION
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Trigger automation workflow by event name
     */
    public function triggerAutomation(string $trigger, Contact $contact, array $data = []): array
    {
        $triggered = [];

        try {
            $result = $this->automationService->triggerWorkflow($trigger, array_merge([
                'contact' => $contact,
                'email' => $contact->email,
            ], $data));

            if ($result) {
                $triggered[] = $trigger;
            }
        } catch (\Exception $e) {
            Log::warning('FluentCRM: Automation trigger failed', [
                'trigger' => $trigger,
                'contact_id' => $contact->id,
                'error' => $e->getMessage(),
            ]);
        }

        return $triggered;
    }

    /**
     * Trigger automation by ID
     */
    public function triggerAutomationById(int $automationId, Contact $contact, array $data = []): bool
    {
        $workflow = EmailAutomationWorkflow::find($automationId);

        if (!$workflow || !$workflow->is_active) {
            return false;
        }

        try {
            $this->automationService->triggerWorkflow($workflow->trigger_event, array_merge([
                'contact' => $contact,
                'email' => $contact->email,
            ], $data));

            return true;
        } catch (\Exception $e) {
            Log::warning('FluentCRM: Automation ID trigger failed', [
                'automation_id' => $automationId,
                'contact_id' => $contact->id,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DOUBLE OPT-IN
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Send double opt-in confirmation email
     */
    public function sendDoubleOptInEmail(Contact $contact): bool
    {
        try {
            // Generate confirmation token
            $token = bin2hex(random_bytes(32));

            $contact->update([
                'confirmation_token' => $token,
                'status' => 'pending',
            ]);

            // Trigger double opt-in email automation
            $this->automationService->triggerWorkflow('double_optin', [
                'contact' => $contact,
                'email' => $contact->email,
                'confirmation_url' => config('app.url') . "/confirm-subscription/{$token}",
            ]);

            Log::info('FluentCRM: Double opt-in email sent', [
                'contact_id' => $contact->id,
                'email' => $contact->email,
            ]);

            return true;

        } catch (\Exception $e) {
            Log::error('FluentCRM: Double opt-in email failed', [
                'contact_id' => $contact->id,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Confirm subscription via token
     */
    public function confirmSubscription(string $token): ?Contact
    {
        $contact = Contact::where('confirmation_token', $token)->first();

        if (!$contact) {
            return null;
        }

        $contact->update([
            'status' => 'active',
            'confirmation_token' => null,
            'confirmed_at' => now(),
        ]);

        // Apply confirmed tag
        $this->applyTags($contact, ['confirmed', 'double-optin-confirmed']);

        // Trigger confirmation automation
        $this->triggerAutomation('subscription_confirmed', $contact);

        return $contact;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CONSENT MANAGEMENT (GDPR)
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Record consent for a contact
     */
    public function recordConsent(string $email, string $type, bool $granted, array $metadata = []): void
    {
        DB::table('contact_consents')->updateOrInsert(
            ['email' => strtolower($email), 'type' => $type],
            [
                'granted' => $granted,
                'granted_at' => $granted ? now() : null,
                'revoked_at' => !$granted ? now() : null,
                'ip_address' => $metadata['ip'] ?? null,
                'user_agent' => $metadata['user_agent'] ?? null,
                'source' => $metadata['source'] ?? null,
                'consent_text' => $metadata['consent_text'] ?? null,
                'metadata' => json_encode($metadata),
                'updated_at' => now(),
            ]
        );
    }

    /**
     * Check if contact has marketing consent
     */
    public function hasMarketingConsent(string $email): bool
    {
        $consent = DB::table('contact_consents')
            ->where('email', strtolower($email))
            ->where('type', 'marketing')
            ->first();

        return $consent?->granted ?? false;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TIMELINE & ACTIVITY
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Add timeline entry for contact
     */
    public function addTimelineEntry(Contact $contact, array $data): CrmActivity
    {
        return CrmActivity::create([
            'contact_id' => $contact->id,
            'user_id' => $data['user_id'] ?? null,
            'type' => $data['type'],
            'title' => $data['title'],
            'description' => $data['description'] ?? null,
            'data' => $data['data'] ?? [],
            'created_at' => now(),
        ]);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ANALYTICS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get campaign count for contact
     */
    public function getContactCampaignCount(int $contactId): int
    {
        return DB::table('email_campaign_sends')
            ->where('contact_id', $contactId)
            ->count();
    }

    /**
     * Get email open rate for period
     */
    public function getEmailOpenRate(\DateTime $startDate): float
    {
        $stats = DB::table('email_campaign_sends')
            ->where('sent_at', '>=', $startDate)
            ->selectRaw('COUNT(*) as total, SUM(CASE WHEN opened_at IS NOT NULL THEN 1 ELSE 0 END) as opened')
            ->first();

        if (!$stats || $stats->total == 0) {
            return 0;
        }

        return round(($stats->opened / $stats->total) * 100, 2);
    }

    /**
     * Get email click rate for period
     */
    public function getEmailClickRate(\DateTime $startDate): float
    {
        $stats = DB::table('email_campaign_sends')
            ->where('sent_at', '>=', $startDate)
            ->selectRaw('COUNT(*) as total, SUM(CASE WHEN clicked_at IS NOT NULL THEN 1 ELSE 0 END) as clicked')
            ->first();

        if (!$stats || $stats->total == 0) {
            return 0;
        }

        return round(($stats->clicked / $stats->total) * 100, 2);
    }
}
