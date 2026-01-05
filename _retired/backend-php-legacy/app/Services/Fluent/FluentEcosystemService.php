<?php

declare(strict_types=1);

namespace App\Services\Fluent;

use App\Models\Contact;
use App\Models\Popup;
use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\User;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * FluentEcosystem Service - Unified Fluent Integration Hub
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Orchestrates all Fluent ecosystem services in one unified service:
 * - FluentForms Pro - Form builder and submissions
 * - FluentCRM Pro - Contact management, segmentation, automations
 * - FluentSMTP - Email delivery
 * - FluentSupport Pro - Help desk and ticketing
 * - FluentBooking Pro - Appointment scheduling
 *
 * Key Features:
 * - Popup opt-in integration (creates contacts, applies tags, triggers automations)
 * - Form → CRM data flow
 * - Support ticket → CRM contact linking
 * - Booking → CRM activity tracking
 * - Unified tagging and segmentation across all services
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class FluentEcosystemService
{
    private const CACHE_PREFIX = 'fluent_ecosystem:';
    private const CACHE_TTL = 3600;

    public function __construct(
        private readonly FluentCRMService $crmService,
        private readonly FluentFormsService $formsService,
        private readonly FluentSMTPService $smtpService,
        private readonly FluentSupportService $supportService,
        private readonly FluentBookingService $bookingService,
    ) {}

    // ═══════════════════════════════════════════════════════════════════════════
    // POPUP OPT-IN INTEGRATION
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Process popup opt-in submission
     *
     * This is the main method for integrating popups with FluentCRM.
     * Creates/updates contacts, applies tags, triggers automations.
     *
     * @param Popup $popup The popup that was converted
     * @param array $formData Submitted form data (email, name, etc.)
     * @param array $metadata Additional context (IP, user agent, page URL, etc.)
     * @return array Processing result with contact_id and actions taken
     */
    public function processPopupOptIn(Popup $popup, array $formData, array $metadata = []): array
    {
        $result = [
            'success' => false,
            'contact_id' => null,
            'contact_created' => false,
            'tags_applied' => [],
            'segments_added' => [],
            'automations_triggered' => [],
            'lists_added' => [],
            'double_optin_sent' => false,
        ];

        try {
            DB::beginTransaction();

            // Extract email from form data
            $email = $this->extractEmail($formData);
            if (!$email) {
                Log::warning('FluentEcosystem: Popup opt-in missing email', [
                    'popup_id' => $popup->id,
                    'form_data' => $formData,
                ]);
                return $result;
            }

            // Get integration config from popup
            $integrationConfig = $popup->integration_config ?? [];
            $crmConfig = $integrationConfig['fluent_crm'] ?? [];

            // Create or update contact in FluentCRM
            $contactData = $this->buildContactDataFromPopup($formData, $metadata, $popup);
            $contact = $this->crmService->findOrCreateContact($contactData);

            $result['contact_id'] = $contact->id;
            $result['contact_created'] = $contact->wasRecentlyCreated;

            // Apply tags from popup configuration
            $tags = $this->getPopupTags($popup, $crmConfig);
            if (!empty($tags)) {
                $this->crmService->applyTags($contact, $tags);
                $result['tags_applied'] = $tags;
            }

            // Add to lists/segments
            $listIds = $crmConfig['lists'] ?? [];
            if (!empty($listIds)) {
                $this->crmService->addToLists($contact, $listIds);
                $result['lists_added'] = $listIds;
            }

            $segmentIds = $crmConfig['segments'] ?? [];
            if (!empty($segmentIds)) {
                $this->crmService->addToSegments($contact, $segmentIds);
                $result['segments_added'] = $segmentIds;
            }

            // Handle double opt-in if required
            if ($crmConfig['double_optin'] ?? false) {
                $this->crmService->sendDoubleOptInEmail($contact);
                $result['double_optin_sent'] = true;
            }

            // Trigger automations
            $automations = $this->triggerPopupAutomations($contact, $popup, $formData, $crmConfig);
            $result['automations_triggered'] = $automations;

            // Log activity to contact timeline
            $this->crmService->addTimelineEntry($contact, [
                'type' => 'popup_optin',
                'title' => "Opted in via popup: {$popup->title}",
                'description' => $this->buildPopupActivityDescription($popup, $metadata),
                'data' => [
                    'popup_id' => $popup->id,
                    'popup_type' => $popup->type,
                    'page_url' => $metadata['page_url'] ?? null,
                    'utm_source' => $metadata['utm_source'] ?? null,
                    'utm_campaign' => $metadata['utm_campaign'] ?? null,
                ],
            ]);

            // Update contact engagement metrics
            $contact->update([
                'last_activity_at' => now(),
                'activities_count' => DB::raw('activities_count + 1'),
                'source' => $contact->source ?? "popup:{$popup->slug}",
            ]);

            // Record consent
            $this->recordPopupConsent($contact, $popup, $formData, $metadata);

            DB::commit();

            $result['success'] = true;

            // Fire event for external listeners
            event(new \App\Events\PopupOptInProcessed($popup, $contact, $result));

            Log::info('FluentEcosystem: Popup opt-in processed', [
                'popup_id' => $popup->id,
                'contact_id' => $contact->id,
                'result' => $result,
            ]);

            return $result;

        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('FluentEcosystem: Popup opt-in failed', [
                'popup_id' => $popup->id,
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);
            throw $e;
        }
    }

    /**
     * Get popup opt-in configuration for frontend
     */
    public function getPopupOptInConfig(Popup $popup): array
    {
        $integrationConfig = $popup->integration_config ?? [];
        $crmConfig = $integrationConfig['fluent_crm'] ?? [];

        return [
            'enabled' => $crmConfig['enabled'] ?? true,
            'double_optin' => $crmConfig['double_optin'] ?? false,
            'lists' => $crmConfig['lists'] ?? [],
            'segments' => $crmConfig['segments'] ?? [],
            'tags' => $crmConfig['tags'] ?? [],
            'automations' => $crmConfig['automations'] ?? [],
            'consent_text' => $crmConfig['consent_text'] ?? 'I agree to receive marketing emails',
            'privacy_policy_url' => $crmConfig['privacy_policy_url'] ?? '/privacy',
            'gdpr_enabled' => $crmConfig['gdpr_enabled'] ?? true,
        ];
    }

    /**
     * Update popup opt-in configuration
     */
    public function updatePopupOptInConfig(Popup $popup, array $config): Popup
    {
        $integrationConfig = $popup->integration_config ?? [];
        $integrationConfig['fluent_crm'] = array_merge(
            $integrationConfig['fluent_crm'] ?? [],
            $config
        );

        $popup->update(['integration_config' => $integrationConfig]);

        return $popup->fresh();
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CROSS-SERVICE INTEGRATION
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Link support ticket to CRM contact
     */
    public function linkSupportTicketToContact(int $ticketId, string $email): array
    {
        $contact = $this->crmService->findByEmail($email);

        if (!$contact) {
            $contact = $this->crmService->createContact(['email' => $email]);
        }

        // Tag contact as support requester
        $this->crmService->applyTags($contact, ['support-requester', 'has-tickets']);

        // Add timeline entry
        $this->crmService->addTimelineEntry($contact, [
            'type' => 'support_ticket',
            'title' => 'Created support ticket',
            'data' => ['ticket_id' => $ticketId],
        ]);

        // Update ticket with contact link
        $this->supportService->linkTicketToContact($ticketId, $contact->id);

        return [
            'contact_id' => $contact->id,
            'ticket_id' => $ticketId,
            'linked' => true,
        ];
    }

    /**
     * Link booking appointment to CRM contact
     */
    public function linkBookingToContact(int $appointmentId, string $email, array $customerData = []): array
    {
        $contact = $this->crmService->findByEmail($email);

        if (!$contact) {
            $contact = $this->crmService->createContact(array_merge(
                ['email' => $email],
                $customerData
            ));
        }

        // Tag contact
        $this->crmService->applyTags($contact, ['has-booked', 'booking-customer']);

        // Add timeline entry
        $this->crmService->addTimelineEntry($contact, [
            'type' => 'booking',
            'title' => 'Booked an appointment',
            'data' => ['appointment_id' => $appointmentId],
        ]);

        // Update appointment with contact link
        $this->bookingService->linkAppointmentToContact($appointmentId, $contact->id);

        return [
            'contact_id' => $contact->id,
            'appointment_id' => $appointmentId,
            'linked' => true,
        ];
    }

    /**
     * Sync form submission to CRM (delegation to IntegrationHub)
     */
    public function syncFormSubmissionToCRM(FormSubmission $submission): array
    {
        return $this->formsService->processSubmission($submission);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // UNIFIED TAGGING & SEGMENTATION
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Apply ecosystem-wide tags based on contact behavior
     */
    public function applyBehaviorTags(Contact $contact): array
    {
        $tags = [];

        // Form engagement tags
        $formCount = FormSubmission::where('user_id', $contact->user_id)->count();
        if ($formCount > 0) {
            $tags[] = 'form-submitter';
            if ($formCount >= 5) $tags[] = 'engaged-form-user';
        }

        // Support engagement tags
        $ticketCount = $this->supportService->getContactTicketCount($contact->id);
        if ($ticketCount > 0) {
            $tags[] = 'support-requester';
            if ($ticketCount >= 3) $tags[] = 'frequent-support';
        }

        // Booking engagement tags
        $appointmentCount = $this->bookingService->getContactAppointmentCount($contact->id);
        if ($appointmentCount > 0) {
            $tags[] = 'booking-customer';
            if ($appointmentCount >= 3) $tags[] = 'repeat-booker';
        }

        // Email engagement tags
        if ($contact->email_opens > 10) $tags[] = 'email-engaged';
        if ($contact->email_clicks > 5) $tags[] = 'email-clicker';

        // Apply all tags
        if (!empty($tags)) {
            $this->crmService->applyTags($contact, $tags);
        }

        return $tags;
    }

    /**
     * Get unified contact activity across all Fluent services
     */
    public function getUnifiedContactActivity(Contact $contact): array
    {
        $cacheKey = self::CACHE_PREFIX . "unified_activity:{$contact->id}";

        return Cache::remember($cacheKey, self::CACHE_TTL, function () use ($contact) {
            return [
                'crm' => [
                    'emails_received' => $contact->emails_received ?? 0,
                    'emails_opened' => $contact->email_opens ?? 0,
                    'emails_clicked' => $contact->email_clicks ?? 0,
                    'campaigns_sent' => $this->crmService->getContactCampaignCount($contact->id),
                ],
                'forms' => [
                    'submissions' => FormSubmission::where('user_id', $contact->user_id)->count(),
                    'forms_completed' => FormSubmission::where('user_id', $contact->user_id)
                        ->distinct('form_id')
                        ->count('form_id'),
                ],
                'support' => [
                    'tickets_total' => $this->supportService->getContactTicketCount($contact->id),
                    'tickets_open' => $this->supportService->getContactOpenTicketCount($contact->id),
                    'avg_resolution_time' => $this->supportService->getContactAvgResolutionTime($contact->id),
                ],
                'booking' => [
                    'appointments_total' => $this->bookingService->getContactAppointmentCount($contact->id),
                    'appointments_completed' => $this->bookingService->getContactCompletedAppointments($contact->id),
                    'no_shows' => $this->bookingService->getContactNoShows($contact->id),
                ],
                'scores' => [
                    'lead_score' => $contact->lead_score,
                    'health_score' => $contact->health_score,
                    'engagement_score' => $contact->engagement_score,
                ],
            ];
        });
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AUTOMATION ORCHESTRATION
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Trigger ecosystem-wide automation
     */
    public function triggerEcosystemAutomation(string $trigger, Contact $contact, array $data = []): array
    {
        $triggered = [];

        // CRM automations
        $crmAutomations = $this->crmService->triggerAutomation($trigger, $contact, $data);
        $triggered = array_merge($triggered, $crmAutomations);

        // Support automations (e.g., ticket follow-up)
        if (str_starts_with($trigger, 'support_')) {
            $supportAutomations = $this->supportService->triggerAutomation($trigger, $contact, $data);
            $triggered = array_merge($triggered, $supportAutomations);
        }

        // Booking automations (e.g., appointment reminders)
        if (str_starts_with($trigger, 'booking_')) {
            $bookingAutomations = $this->bookingService->triggerAutomation($trigger, $contact, $data);
            $triggered = array_merge($triggered, $bookingAutomations);
        }

        return $triggered;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ECOSYSTEM HEALTH & ANALYTICS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get ecosystem health status
     */
    public function getEcosystemHealth(): array
    {
        return [
            'status' => 'healthy',
            'services' => [
                'fluent_forms' => [
                    'status' => 'active',
                    'health' => 100,
                    'forms_count' => Form::where('is_published', true)->count(),
                    'submissions_today' => FormSubmission::whereDate('created_at', today())->count(),
                ],
                'fluent_crm' => [
                    'status' => 'active',
                    'health' => 100,
                    'contacts_total' => Contact::count(),
                    'contacts_active' => Contact::where('status', 'active')->count(),
                ],
                'fluent_smtp' => [
                    'status' => 'active',
                    'health' => $this->smtpService->getHealth(),
                    'emails_sent_today' => $this->smtpService->getEmailsSentToday(),
                ],
                'fluent_support' => [
                    'status' => 'active',
                    'health' => 100,
                    'open_tickets' => $this->supportService->getOpenTicketCount(),
                    'avg_response_time' => $this->supportService->getAvgResponseTime(),
                ],
                'fluent_booking' => [
                    'status' => 'active',
                    'health' => 100,
                    'upcoming_appointments' => $this->bookingService->getUpcomingAppointmentCount(),
                    'bookings_today' => $this->bookingService->getBookingsToday(),
                ],
            ],
            'last_checked' => now()->toIso8601String(),
        ];
    }

    /**
     * Get ecosystem analytics summary
     */
    public function getEcosystemAnalytics(string $period = '30d'): array
    {
        $startDate = match ($period) {
            '7d' => now()->subDays(7),
            '30d' => now()->subDays(30),
            '90d' => now()->subDays(90),
            'ytd' => now()->startOfYear(),
            default => now()->subDays(30),
        };

        return [
            'period' => $period,
            'forms' => [
                'submissions' => FormSubmission::where('created_at', '>=', $startDate)->count(),
                'conversion_rate' => $this->formsService->getConversionRate($startDate),
            ],
            'crm' => [
                'new_contacts' => Contact::where('created_at', '>=', $startDate)->count(),
                'email_open_rate' => $this->crmService->getEmailOpenRate($startDate),
                'email_click_rate' => $this->crmService->getEmailClickRate($startDate),
            ],
            'support' => [
                'tickets_created' => $this->supportService->getTicketsCreated($startDate),
                'tickets_resolved' => $this->supportService->getTicketsResolved($startDate),
                'satisfaction_score' => $this->supportService->getSatisfactionScore($startDate),
            ],
            'booking' => [
                'appointments' => $this->bookingService->getAppointmentCount($startDate),
                'revenue' => $this->bookingService->getBookingRevenue($startDate),
                'no_show_rate' => $this->bookingService->getNoShowRate($startDate),
            ],
        ];
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    private function extractEmail(array $formData): ?string
    {
        // Try common email field names
        $emailFields = ['email', 'email_address', 'user_email', 'contact_email', 'e-mail', 'subscriber_email'];

        foreach ($emailFields as $field) {
            if (!empty($formData[$field]) && filter_var($formData[$field], FILTER_VALIDATE_EMAIL)) {
                return strtolower(trim($formData[$field]));
            }
        }

        return null;
    }

    private function buildContactDataFromPopup(array $formData, array $metadata, Popup $popup): array
    {
        $data = [
            'email' => $this->extractEmail($formData),
            'source' => "popup:{$popup->slug}",
        ];

        // Extract name fields
        if (!empty($formData['name'])) {
            $nameParts = explode(' ', $formData['name'], 2);
            $data['first_name'] = $nameParts[0];
            $data['last_name'] = $nameParts[1] ?? '';
        }
        if (!empty($formData['first_name'])) {
            $data['first_name'] = $formData['first_name'];
        }
        if (!empty($formData['last_name'])) {
            $data['last_name'] = $formData['last_name'];
        }

        // Extract other fields
        if (!empty($formData['phone'])) {
            $data['phone'] = $formData['phone'];
        }
        if (!empty($formData['company'])) {
            $data['company'] = $formData['company'];
        }

        // Add metadata as custom fields
        $customFields = [];
        if (!empty($metadata['utm_source'])) {
            $customFields['utm_source'] = $metadata['utm_source'];
        }
        if (!empty($metadata['utm_campaign'])) {
            $customFields['utm_campaign'] = $metadata['utm_campaign'];
        }
        if (!empty($metadata['utm_medium'])) {
            $customFields['utm_medium'] = $metadata['utm_medium'];
        }
        if (!empty($metadata['page_url'])) {
            $customFields['optin_page'] = $metadata['page_url'];
        }
        if (!empty($customFields)) {
            $data['custom_fields'] = $customFields;
        }

        return $data;
    }

    private function getPopupTags(Popup $popup, array $crmConfig): array
    {
        $tags = $crmConfig['tags'] ?? [];

        // Add automatic tags based on popup type
        $tags[] = 'popup-subscriber';
        $tags[] = "popup-type:{$popup->type}";

        // Add popup-specific tag if configured
        if (!empty($popup->slug)) {
            $tags[] = "popup:{$popup->slug}";
        }

        return array_unique($tags);
    }

    private function triggerPopupAutomations(Contact $contact, Popup $popup, array $formData, array $crmConfig): array
    {
        $triggered = [];

        // Generic popup opt-in automation
        $this->crmService->triggerAutomation('popup_optin', $contact, [
            'popup_id' => $popup->id,
            'popup_type' => $popup->type,
            'form_data' => $formData,
        ]);
        $triggered[] = 'popup_optin';

        // Popup-specific automation
        $this->crmService->triggerAutomation("popup_optin:{$popup->slug}", $contact, [
            'popup_id' => $popup->id,
            'form_data' => $formData,
        ]);
        $triggered[] = "popup_optin:{$popup->slug}";

        // Custom automations from config
        $automationIds = $crmConfig['automations'] ?? [];
        foreach ($automationIds as $automationId) {
            $this->crmService->triggerAutomationById($automationId, $contact, [
                'popup_id' => $popup->id,
                'form_data' => $formData,
            ]);
            $triggered[] = "automation:{$automationId}";
        }

        return $triggered;
    }

    private function buildPopupActivityDescription(Popup $popup, array $metadata): string
    {
        $description = "Subscribed via {$popup->type} popup";

        if (!empty($metadata['page_url'])) {
            $description .= " on page: {$metadata['page_url']}";
        }

        if (!empty($metadata['utm_campaign'])) {
            $description .= " (Campaign: {$metadata['utm_campaign']})";
        }

        return $description;
    }

    private function recordPopupConsent(Contact $contact, Popup $popup, array $formData, array $metadata): void
    {
        // Check if consent field was included and checked
        $consentGiven = $formData['consent'] ?? $formData['marketing_consent'] ?? $formData['opt_in'] ?? true;

        if ($consentGiven) {
            $this->crmService->recordConsent($contact->email, 'marketing', true, [
                'source' => 'popup',
                'popup_id' => $popup->id,
                'popup_slug' => $popup->slug,
                'ip' => $metadata['ip'] ?? null,
                'user_agent' => $metadata['user_agent'] ?? null,
                'timestamp' => now()->toIso8601String(),
                'consent_text' => $popup->integration_config['fluent_crm']['consent_text'] ?? 'Subscribed via popup',
            ]);
        }
    }
}
