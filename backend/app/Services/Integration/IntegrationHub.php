<?php

declare(strict_types=1);

namespace App\Services\Integration;

use App\Models\Contact;
use App\Models\Deal;
use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\Order;
use App\Models\User;
use App\Models\NewsletterSubscription;
use App\Models\CrmActivity;
use App\Models\EmailAutomationWorkflow;
use App\Services\LeadScoringService;
use App\Services\HealthScoringService;
use App\Services\ContactTimelineService;
use App\Services\Email\EmailAutomationService;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * IntegrationHub - Central Orchestration Service
 *
 * This is the brain of the unified ecosystem, connecting:
 * - Forms → CRM (contact creation, automations)
 * - Cart/Orders → CRM (LTV updates, purchase triggers)
 * - CRM → Email (campaigns, sequences)
 * - Consent → Everything (GDPR compliance layer)
 *
 * Matches and exceeds FluentCRM Pro + FluentForm Pro + Fluent Cart Pro integration capabilities.
 *
 * @author Revolution Trading Pros
 * @version 2.0.0
 */
class IntegrationHub
{
    private const CACHE_PREFIX = 'integration_hub:';
    private const CACHE_TTL = 3600; // 1 hour

    public function __construct(
        private readonly LeadScoringService $leadScoringService,
        private readonly HealthScoringService $healthScoringService,
        private readonly ContactTimelineService $timelineService,
        private readonly EmailAutomationService $automationService,
        private readonly ConsentService $consentService,
    ) {}

    /**
     * Process a form submission through the integration pipeline
     *
     * Pipeline stages:
     * 1. Extract contact data from submission
     * 2. Create or update contact in CRM
     * 3. Apply tags and custom fields
     * 4. Calculate lead score
     * 5. Add to segments
     * 6. Trigger automations
     * 7. Log activity to timeline
     */
    public function processFormSubmission(FormSubmission $submission): array
    {
        $result = [
            'contact_id' => null,
            'contact_created' => false,
            'tags_applied' => [],
            'segments_added' => [],
            'automations_triggered' => [],
            'timeline_entry_id' => null,
        ];

        try {
            DB::beginTransaction();

            $form = $submission->form;
            $formData = $submission->getDataArray();
            $integrationSettings = $form->settings['crm_integration'] ?? [];

            // Skip if CRM integration is disabled for this form
            if (!($integrationSettings['enabled'] ?? true)) {
                DB::commit();
                return $result;
            }

            // Step 1: Extract contact data
            $contactData = $this->extractContactDataFromSubmission($submission, $integrationSettings);

            if (empty($contactData['email'])) {
                Log::warning('IntegrationHub: No email found in form submission', [
                    'submission_id' => $submission->id,
                    'form_id' => $form->id,
                ]);
                DB::commit();
                return $result;
            }

            // Step 2: Check consent before proceeding
            if (!$this->consentService->hasMarketingConsent($contactData['email'])) {
                // Check if form has consent field that was checked
                $consentField = $this->findConsentFieldValue($formData, $form);
                if (!$consentField) {
                    Log::info('IntegrationHub: No consent for CRM integration', [
                        'email' => $contactData['email'],
                    ]);
                    DB::commit();
                    return $result;
                }
            }

            // Step 3: Create or update contact
            $contact = $this->findOrCreateContact($contactData);
            $result['contact_id'] = $contact->id;
            $result['contact_created'] = $contact->wasRecentlyCreated;

            // Step 4: Apply tags from form settings
            $tags = $integrationSettings['tags'] ?? [];
            if (!empty($tags)) {
                $this->applyTags($contact, $tags);
                $result['tags_applied'] = $tags;
            }

            // Step 5: Apply custom field mappings
            $fieldMappings = $integrationSettings['field_mappings'] ?? [];
            $this->applyFieldMappings($contact, $formData, $fieldMappings);

            // Step 6: Add to segments
            $segments = $integrationSettings['segments'] ?? [];
            if (!empty($segments)) {
                $this->addToSegments($contact, $segments);
                $result['segments_added'] = $segments;
            }

            // Step 7: Calculate and update lead score
            $this->leadScoringService->recalculateScore($contact);
            $this->healthScoringService->recalculateScore($contact);

            // Step 8: Log activity to timeline
            $timelineEntry = $this->timelineService->addFormSubmissionEvent($contact, $submission);
            $result['timeline_entry_id'] = $timelineEntry?->id;

            // Step 9: Trigger automations
            $automations = $this->triggerFormAutomations($contact, $submission, $form);
            $result['automations_triggered'] = $automations;

            // Step 10: Update contact's last activity
            $contact->update([
                'last_activity_at' => now(),
                'activities_count' => DB::raw('activities_count + 1'),
            ]);

            DB::commit();

            // Fire integration event for external listeners
            event(new \App\Events\FormCrmIntegrationCompleted($submission, $contact, $result));

            Log::info('IntegrationHub: Form submission processed', [
                'submission_id' => $submission->id,
                'contact_id' => $contact->id,
                'result' => $result,
            ]);

            return $result;

        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('IntegrationHub: Form submission processing failed', [
                'submission_id' => $submission->id,
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);
            throw $e;
        }
    }

    /**
     * Process an order through the CRM integration pipeline
     *
     * Pipeline stages:
     * 1. Find or create contact from order
     * 2. Update lifetime value
     * 3. Update subscription status
     * 4. Apply purchase tags
     * 5. Create/update deal
     * 6. Trigger purchase automations
     * 7. Log to timeline
     */
    public function processOrder(Order $order, string $eventType = 'created'): array
    {
        $result = [
            'contact_id' => null,
            'contact_updated' => false,
            'deal_id' => null,
            'deal_created' => false,
            'ltv_updated' => false,
            'tags_applied' => [],
            'automations_triggered' => [],
        ];

        try {
            DB::beginTransaction();

            $user = $order->user;
            if (!$user) {
                DB::commit();
                return $result;
            }

            // Step 1: Find or create contact
            $contact = $this->findOrCreateContactFromUser($user);
            $result['contact_id'] = $contact->id;

            // Step 2: Check consent
            if (!$this->consentService->hasTransactionalConsent($contact->email)) {
                Log::info('IntegrationHub: No transactional consent for order processing', [
                    'order_id' => $order->id,
                    'email' => $contact->email,
                ]);
                // Continue with limited processing - transactional emails still allowed
            }

            // Step 3: Update lifetime value
            $this->updateContactLifetimeValue($contact, $order, $eventType);
            $result['ltv_updated'] = true;
            $result['contact_updated'] = true;

            // Step 4: Apply purchase tags based on products
            $purchaseTags = $this->generatePurchaseTags($order);
            if (!empty($purchaseTags)) {
                $this->applyTags($contact, $purchaseTags);
                $result['tags_applied'] = $purchaseTags;
            }

            // Step 5: Create or update deal
            if ($eventType === 'created' || $eventType === 'completed') {
                $deal = $this->createOrUpdateDealFromOrder($contact, $order);
                if ($deal) {
                    $result['deal_id'] = $deal->id;
                    $result['deal_created'] = $deal->wasRecentlyCreated;
                }
            }

            // Step 6: Update subscription status if applicable
            $this->updateSubscriptionStatus($contact, $order);

            // Step 7: Recalculate scores
            $this->leadScoringService->recalculateScore($contact);
            $this->healthScoringService->recalculateScore($contact);

            // Step 8: Log to timeline
            $this->timelineService->addOrderEvent($contact, $order, $eventType);

            // Step 9: Trigger automations
            $automations = $this->triggerOrderAutomations($contact, $order, $eventType);
            $result['automations_triggered'] = $automations;

            // Step 10: Update activity tracking
            $contact->update([
                'last_activity_at' => now(),
                'last_contacted_at' => now(),
            ]);

            DB::commit();

            // Fire integration event
            event(new \App\Events\OrderCrmIntegrationCompleted($order, $contact, $result));

            Log::info('IntegrationHub: Order processed', [
                'order_id' => $order->id,
                'event_type' => $eventType,
                'contact_id' => $contact->id,
            ]);

            return $result;

        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('IntegrationHub: Order processing failed', [
                'order_id' => $order->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Process a newsletter subscription through the ecosystem
     */
    public function processNewsletterSubscription(NewsletterSubscription $subscription, string $eventType = 'subscribed'): array
    {
        $result = [
            'contact_id' => null,
            'contact_created' => false,
            'tags_applied' => [],
            'automations_triggered' => [],
        ];

        try {
            DB::beginTransaction();

            // Create or update contact
            $contact = $this->findOrCreateContact([
                'email' => $subscription->email,
                'first_name' => $subscription->first_name ?? null,
                'last_name' => $subscription->last_name ?? null,
                'source' => 'newsletter',
            ]);

            $result['contact_id'] = $contact->id;
            $result['contact_created'] = $contact->wasRecentlyCreated;

            // Apply newsletter subscriber tag
            $tags = ['newsletter-subscriber'];
            if ($eventType === 'unsubscribed') {
                $tags = ['newsletter-unsubscribed'];
                $contact->update(['is_unsubscribed' => true]);
            }

            $this->applyTags($contact, $tags);
            $result['tags_applied'] = $tags;

            // Update consent record
            $this->consentService->recordConsent($contact->email, 'newsletter', $eventType === 'subscribed', [
                'source' => 'newsletter_form',
                'ip' => $subscription->consent_ip,
                'timestamp' => now(),
            ]);

            // Log to timeline
            $this->timelineService->addNewsletterEvent($contact, $subscription, $eventType);

            // Trigger automations
            $automations = $this->automationService->triggerWorkflow("newsletter_{$eventType}", [
                'contact' => $contact,
                'subscription' => $subscription,
                'email' => $contact->email,
            ]);
            $result['automations_triggered'] = $automations ?? [];

            // Recalculate scores
            $this->healthScoringService->recalculateScore($contact);

            DB::commit();

            return $result;

        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('IntegrationHub: Newsletter subscription processing failed', [
                'subscription_id' => $subscription->id,
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Process email engagement events (opens, clicks)
     */
    public function processEmailEngagement(string $email, string $eventType, array $metadata = []): void
    {
        $contact = Contact::where('email', $email)->first();
        if (!$contact) {
            return;
        }

        $updates = ['last_activity_at' => now()];

        switch ($eventType) {
            case 'opened':
                $updates['email_opens'] = DB::raw('email_opens + 1');
                $updates['last_email_opened_at'] = now();
                break;
            case 'clicked':
                $updates['email_clicks'] = DB::raw('email_clicks + 1');
                $updates['last_email_clicked_at'] = now();
                break;
            case 'bounced':
                $updates['is_verified'] = false;
                break;
            case 'complained':
                $updates['do_not_contact'] = true;
                $updates['is_unsubscribed'] = true;
                break;
        }

        $contact->update($updates);

        // Log to timeline
        $this->timelineService->addEmailEngagementEvent($contact, $eventType, $metadata);

        // Recalculate engagement score
        $this->leadScoringService->recalculateScore($contact);

        // Trigger engagement automations
        $this->automationService->triggerWorkflow("email_{$eventType}", [
            'contact' => $contact,
            'email' => $email,
            'metadata' => $metadata,
        ]);
    }

    /**
     * Get unified contact profile with data from all systems
     */
    public function getUnifiedContactProfile(Contact $contact): array
    {
        $cacheKey = self::CACHE_PREFIX . "profile:{$contact->id}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($contact) {
            return [
                // Core contact data
                'contact' => $contact->toArray(),

                // Form submissions
                'form_submissions' => FormSubmission::where('user_id', $contact->user_id)
                    ->orWhereHas('data', function ($q) use ($contact) {
                        $q->where('field_name', 'email')
                          ->where('value', $contact->email);
                    })
                    ->with('form:id,name,slug')
                    ->latest()
                    ->limit(20)
                    ->get(),

                // Orders and purchases
                'orders' => Order::where('user_id', $contact->user_id)
                    ->with('items')
                    ->latest()
                    ->limit(20)
                    ->get(),

                // Deals
                'deals' => $contact->deals()
                    ->with(['pipeline', 'stage'])
                    ->latest()
                    ->limit(10)
                    ->get(),

                // Email engagement
                'email_engagement' => [
                    'total_opens' => $contact->email_opens,
                    'total_clicks' => $contact->email_clicks,
                    'last_opened' => $contact->last_email_opened_at,
                    'last_clicked' => $contact->last_email_clicked_at,
                ],

                // Consent status
                'consent' => $this->consentService->getConsentStatus($contact->email),

                // Segments
                'segments' => $contact->segments()->pluck('name', 'id'),

                // Tags
                'tags' => $contact->tags ?? [],

                // Scores
                'scores' => [
                    'lead_score' => $contact->lead_score,
                    'health_score' => $contact->health_score,
                    'engagement_score' => $contact->engagement_score,
                    'value_score' => $contact->value_score,
                ],

                // Activity summary
                'activity_summary' => [
                    'total_sessions' => $contact->total_sessions,
                    'total_activities' => $contact->activities_count,
                    'last_activity' => $contact->last_activity_at,
                    'last_contacted' => $contact->last_contacted_at,
                ],

                // Calculated metrics
                'metrics' => [
                    'lifetime_value' => $contact->lifetime_value,
                    'subscription_mrr' => $contact->subscription_mrr,
                    'days_as_customer' => $contact->converted_at
                        ? now()->diffInDays($contact->converted_at)
                        : null,
                ],
            ];
        });
    }

    /**
     * Invalidate cached contact profile
     */
    public function invalidateContactCache(int $contactId): void
    {
        Cache::forget(self::CACHE_PREFIX . "profile:{$contactId}");
    }

    // =========================================================================
    // PRIVATE HELPER METHODS
    // =========================================================================

    private function extractContactDataFromSubmission(FormSubmission $submission, array $settings): array
    {
        $formData = $submission->getDataArray();
        $fieldMappings = $settings['field_mappings'] ?? $this->getDefaultFieldMappings();

        $contactData = [];

        foreach ($fieldMappings as $formField => $contactField) {
            if (isset($formData[$formField]) && !empty($formData[$formField])) {
                $contactData[$contactField] = $formData[$formField];
            }
        }

        // Fallback: try to find email field by common names
        if (empty($contactData['email'])) {
            $emailFields = ['email', 'email_address', 'user_email', 'contact_email', 'e-mail'];
            foreach ($emailFields as $field) {
                if (!empty($formData[$field])) {
                    $contactData['email'] = $formData[$field];
                    break;
                }
            }
        }

        // Set source
        $contactData['source'] = $settings['source'] ?? 'form:' . $submission->form->slug;

        return $contactData;
    }

    private function getDefaultFieldMappings(): array
    {
        return [
            'email' => 'email',
            'email_address' => 'email',
            'first_name' => 'first_name',
            'firstname' => 'first_name',
            'last_name' => 'last_name',
            'lastname' => 'last_name',
            'name' => 'first_name',
            'phone' => 'phone',
            'phone_number' => 'phone',
            'mobile' => 'mobile',
            'company' => 'company',
            'company_name' => 'company',
            'job_title' => 'job_title',
            'position' => 'job_title',
            'website' => 'website',
            'address' => 'address_line1',
            'city' => 'city',
            'state' => 'state',
            'country' => 'country',
            'postal_code' => 'postal_code',
            'zip' => 'postal_code',
        ];
    }

    private function findOrCreateContact(array $data): Contact
    {
        $email = $data['email'];
        unset($data['email']);

        $contact = Contact::firstOrNew(['email' => $email]);

        // Only update fields that are not already set (preserve existing data)
        foreach ($data as $key => $value) {
            if (empty($contact->$key) && !empty($value)) {
                $contact->$key = $value;
            }
        }

        // Set default values for new contacts
        if (!$contact->exists) {
            $contact->status = 'active';
            $contact->lifecycle_stage = 'lead';
            $contact->lead_score = 0;
            $contact->health_score = 50;
            $contact->engagement_score = 0;
            $contact->value_score = 0;
        }

        $contact->save();

        return $contact;
    }

    private function findOrCreateContactFromUser(User $user): Contact
    {
        return $this->findOrCreateContact([
            'email' => $user->email,
            'first_name' => $user->first_name ?? $user->name,
            'last_name' => $user->last_name ?? '',
            'source' => 'user_registration',
            'user_id' => $user->id,
        ]);
    }

    private function findConsentFieldValue(array $formData, Form $form): bool
    {
        $consentFields = ['consent', 'marketing_consent', 'newsletter', 'subscribe', 'opt_in', 'gdpr_consent'];

        foreach ($consentFields as $field) {
            if (isset($formData[$field])) {
                $value = $formData[$field];
                // Handle various truthy values
                if ($value === true || $value === 'yes' || $value === '1' || $value === 'on' || $value === 'true') {
                    return true;
                }
            }
        }

        return false;
    }

    private function applyTags(Contact $contact, array $tags): void
    {
        $existingTags = $contact->tags ?? [];
        $newTags = array_unique(array_merge($existingTags, $tags));
        $contact->update(['tags' => $newTags]);
    }

    private function applyFieldMappings(Contact $contact, array $formData, array $mappings): void
    {
        $customFields = $contact->custom_fields ?? [];

        foreach ($mappings as $formField => $contactField) {
            if (!isset($formData[$formField])) {
                continue;
            }

            $value = $formData[$formField];

            // Check if it's a standard contact field
            if (in_array($contactField, $contact->getFillable())) {
                if (empty($contact->$contactField)) {
                    $contact->$contactField = $value;
                }
            } else {
                // Store in custom fields
                $customFields[$contactField] = $value;
            }
        }

        $contact->custom_fields = $customFields;
        $contact->save();
    }

    private function addToSegments(Contact $contact, array $segmentIds): void
    {
        $contact->segments()->syncWithoutDetaching($segmentIds);
    }

    private function triggerFormAutomations(Contact $contact, FormSubmission $submission, Form $form): array
    {
        $triggered = [];

        // Trigger generic form submission workflow
        $this->automationService->triggerWorkflow('form_submitted', [
            'contact' => $contact,
            'submission' => $submission,
            'form' => $form,
            'email' => $contact->email,
            'form_slug' => $form->slug,
        ]);
        $triggered[] = 'form_submitted';

        // Trigger form-specific workflow
        $this->automationService->triggerWorkflow("form_submitted:{$form->slug}", [
            'contact' => $contact,
            'submission' => $submission,
            'form' => $form,
            'email' => $contact->email,
        ]);
        $triggered[] = "form_submitted:{$form->slug}";

        // Check for form-specific automations in settings
        $automationIds = $form->settings['trigger_automations'] ?? [];
        foreach ($automationIds as $automationId) {
            $workflow = EmailAutomationWorkflow::find($automationId);
            if ($workflow && $workflow->is_active) {
                $this->automationService->triggerWorkflow($workflow->trigger_event, [
                    'contact' => $contact,
                    'submission' => $submission,
                    'email' => $contact->email,
                ]);
                $triggered[] = "automation:{$automationId}";
            }
        }

        return $triggered;
    }

    private function updateContactLifetimeValue(Contact $contact, Order $order, string $eventType): void
    {
        if ($eventType === 'completed' || $eventType === 'created') {
            $totalLTV = Order::where('user_id', $order->user_id)
                ->whereIn('status', ['completed', 'processing'])
                ->sum('total');

            $contact->update([
                'lifetime_value' => $totalLTV,
                'value_score' => min(100, floor($totalLTV / 100)), // Simple value score calculation
            ]);

            // Update lifecycle stage based on LTV
            if ($totalLTV >= 1000 && $contact->lifecycle_stage !== 'customer') {
                $contact->update([
                    'lifecycle_stage' => 'customer',
                    'converted_at' => $contact->converted_at ?? now(),
                ]);
            }

            if ($totalLTV >= 5000) {
                $contact->update(['is_vip' => true]);
                $this->applyTags($contact, ['vip', 'high-value-customer']);
            }
        } elseif ($eventType === 'refunded') {
            // Recalculate LTV excluding refunded orders
            $totalLTV = Order::where('user_id', $order->user_id)
                ->whereIn('status', ['completed', 'processing'])
                ->sum('total');

            $contact->update(['lifetime_value' => $totalLTV]);
        }
    }

    private function generatePurchaseTags(Order $order): array
    {
        $tags = ['customer', 'has-purchased'];

        foreach ($order->items as $item) {
            // Add product-specific tags
            if ($item->product) {
                $tags[] = 'purchased:' . Str::slug($item->product->name);

                // Add category tags
                if ($item->product->category) {
                    $tags[] = 'category:' . Str::slug($item->product->category->name);
                }
            }
        }

        // Add order value tags
        if ($order->total >= 100) {
            $tags[] = 'order-value:100+';
        }
        if ($order->total >= 500) {
            $tags[] = 'order-value:500+';
        }
        if ($order->total >= 1000) {
            $tags[] = 'order-value:1000+';
        }

        return array_unique($tags);
    }

    private function createOrUpdateDealFromOrder(Contact $contact, Order $order): ?Deal
    {
        // Find default pipeline
        $pipeline = \App\Models\Pipeline::where('is_default', true)->first();
        if (!$pipeline) {
            return null;
        }

        // Get appropriate stage
        $stage = $order->status === 'completed'
            ? $pipeline->stages()->where('name', 'like', '%won%')->first()
            : $pipeline->stages()->where('is_default', true)->first();

        if (!$stage) {
            $stage = $pipeline->stages()->first();
        }

        // Check if deal already exists for this order
        $deal = Deal::where('contact_id', $contact->id)
            ->where('custom_fields->order_id', $order->id)
            ->first();

        if ($deal) {
            // Update existing deal
            $deal->update([
                'amount' => $order->total,
                'stage_id' => $stage->id,
                'status' => $order->status === 'completed' ? 'won' : 'open',
            ]);
        } else {
            // Create new deal
            $deal = Deal::create([
                'name' => "Order #{$order->order_number}",
                'contact_id' => $contact->id,
                'pipeline_id' => $pipeline->id,
                'stage_id' => $stage->id,
                'amount' => $order->total,
                'currency' => $order->currency ?? 'USD',
                'probability' => $order->status === 'completed' ? 100 : 50,
                'status' => $order->status === 'completed' ? 'won' : 'open',
                'source_channel' => 'ecommerce',
                'custom_fields' => ['order_id' => $order->id],
            ]);
        }

        return $deal;
    }

    private function updateSubscriptionStatus(Contact $contact, Order $order): void
    {
        // Check if order contains subscription products
        $hasSubscription = $order->items->contains(function ($item) {
            return $item->product?->is_subscription ?? false;
        });

        if ($hasSubscription && $order->status === 'completed') {
            $contact->update([
                'subscription_status' => 'active',
                'subscription_plan_id' => $order->items->first(function ($item) {
                    return $item->product?->is_subscription ?? false;
                })?->product_id,
            ]);

            $this->applyTags($contact, ['subscriber', 'active-subscription']);
        }
    }

    private function triggerOrderAutomations(Contact $contact, Order $order, string $eventType): array
    {
        $triggered = [];

        // Map event types to automation triggers
        $triggerMap = [
            'created' => 'order_created',
            'completed' => 'purchase_completed',
            'processing' => 'order_processing',
            'shipped' => 'order_shipped',
            'delivered' => 'order_delivered',
            'refunded' => 'order_refunded',
            'cancelled' => 'order_cancelled',
        ];

        $triggerEvent = $triggerMap[$eventType] ?? "order_{$eventType}";

        $this->automationService->triggerWorkflow($triggerEvent, [
            'contact' => $contact,
            'order' => $order,
            'email' => $contact->email,
            'order_total' => $order->total,
            'order_number' => $order->order_number,
        ]);
        $triggered[] = $triggerEvent;

        // Trigger product-specific automations
        foreach ($order->items as $item) {
            if ($item->product) {
                $this->automationService->triggerWorkflow('product_purchased', [
                    'contact' => $contact,
                    'product_id' => $item->product_id,
                    'product_name' => $item->product->name,
                    'email' => $contact->email,
                ]);
                $triggered[] = "product_purchased:{$item->product_id}";
            }
        }

        // First purchase automation
        $orderCount = Order::where('user_id', $order->user_id)
            ->where('status', 'completed')
            ->count();

        if ($orderCount === 1 && $eventType === 'completed') {
            $this->automationService->triggerWorkflow('first_purchase', [
                'contact' => $contact,
                'order' => $order,
                'email' => $contact->email,
            ]);
            $triggered[] = 'first_purchase';
        }

        return $triggered;
    }
}
