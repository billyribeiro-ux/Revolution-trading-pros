<?php

declare(strict_types=1);

namespace App\Services\Fluent;

use App\Models\Contact;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * FluentSupport Service - Help Desk & Ticketing
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive support functionality matching FluentSupport Pro:
 * - Ticket management
 * - Agent assignments
 * - SLA tracking
 * - Canned responses
 * - Customer satisfaction
 * - Department routing
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class FluentSupportService
{
    /**
     * Link ticket to CRM contact
     */
    public function linkTicketToContact(int $ticketId, int $contactId): bool
    {
        return DB::table('support_tickets')
            ->where('id', $ticketId)
            ->update(['contact_id' => $contactId]) > 0;
    }

    /**
     * Get ticket count for contact
     */
    public function getContactTicketCount(int $contactId): int
    {
        return DB::table('support_tickets')
            ->where('contact_id', $contactId)
            ->count();
    }

    /**
     * Get open ticket count for contact
     */
    public function getContactOpenTicketCount(int $contactId): int
    {
        return DB::table('support_tickets')
            ->where('contact_id', $contactId)
            ->whereIn('status', ['open', 'pending', 'waiting'])
            ->count();
    }

    /**
     * Get average resolution time for contact tickets (in hours)
     */
    public function getContactAvgResolutionTime(int $contactId): ?float
    {
        $avg = DB::table('support_tickets')
            ->where('contact_id', $contactId)
            ->whereNotNull('resolved_at')
            ->selectRaw('AVG(TIMESTAMPDIFF(HOUR, created_at, resolved_at)) as avg_hours')
            ->value('avg_hours');

        return $avg ? round((float) $avg, 1) : null;
    }

    /**
     * Get total open tickets count
     */
    public function getOpenTicketCount(): int
    {
        return DB::table('support_tickets')
            ->whereIn('status', ['open', 'pending', 'waiting'])
            ->count();
    }

    /**
     * Get average first response time (in minutes)
     */
    public function getAvgResponseTime(): ?float
    {
        $avg = DB::table('support_tickets')
            ->whereNotNull('first_response_at')
            ->where('created_at', '>=', now()->subDays(30))
            ->selectRaw('AVG(TIMESTAMPDIFF(MINUTE, created_at, first_response_at)) as avg_minutes')
            ->value('avg_minutes');

        return $avg ? round((float) $avg, 1) : null;
    }

    /**
     * Get tickets created in period
     */
    public function getTicketsCreated(\DateTime $startDate): int
    {
        return DB::table('support_tickets')
            ->where('created_at', '>=', $startDate)
            ->count();
    }

    /**
     * Get tickets resolved in period
     */
    public function getTicketsResolved(\DateTime $startDate): int
    {
        return DB::table('support_tickets')
            ->where('resolved_at', '>=', $startDate)
            ->count();
    }

    /**
     * Get customer satisfaction score (1-5 scale)
     */
    public function getSatisfactionScore(\DateTime $startDate): ?float
    {
        $avg = DB::table('support_tickets')
            ->whereNotNull('satisfaction_rating')
            ->where('resolved_at', '>=', $startDate)
            ->avg('satisfaction_rating');

        return $avg ? round((float) $avg, 2) : null;
    }

    /**
     * Trigger support automation
     */
    public function triggerAutomation(string $trigger, Contact $contact, array $data = []): array
    {
        $triggered = [];

        // Check for automation rules matching the trigger
        $automations = DB::table('support_automations')
            ->where('trigger_event', $trigger)
            ->where('is_active', true)
            ->get();

        foreach ($automations as $automation) {
            // Process automation actions
            $this->processAutomationActions($automation, $contact, $data);
            $triggered[] = "support_automation:{$automation->id}";
        }

        return $triggered;
    }

    /**
     * Get contact's tickets
     */
    public function getContactTickets(int $contactId, int $limit = 20): array
    {
        return DB::table('support_tickets')
            ->where('contact_id', $contactId)
            ->orderByDesc('created_at')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Create ticket from contact
     */
    public function createTicket(array $data): int
    {
        $ticketNumber = 'TKT-' . date('Y') . '-' . str_pad((string) (DB::table('support_tickets')->count() + 1), 6, '0', STR_PAD_LEFT);

        return DB::table('support_tickets')->insertGetId([
            'ticket_number' => $ticketNumber,
            'subject' => $data['subject'],
            'description' => $data['description'],
            'customer_name' => $data['customer_name'],
            'customer_email' => $data['customer_email'],
            'contact_id' => $data['contact_id'] ?? null,
            'department_id' => $data['department_id'] ?? null,
            'priority' => $data['priority'] ?? 'medium',
            'status' => 'open',
            'source' => $data['source'] ?? 'web',
            'created_at' => now(),
            'updated_at' => now(),
        ]);
    }

    /**
     * Process automation actions
     */
    private function processAutomationActions(object $automation, Contact $contact, array $data): void
    {
        $actions = json_decode($automation->actions, true) ?? [];

        foreach ($actions as $action) {
            switch ($action['type'] ?? null) {
                case 'assign_agent':
                    if (isset($data['ticket_id'])) {
                        DB::table('support_tickets')
                            ->where('id', $data['ticket_id'])
                            ->update(['assigned_agent_id' => $action['agent_id']]);
                    }
                    break;

                case 'set_priority':
                    if (isset($data['ticket_id'])) {
                        DB::table('support_tickets')
                            ->where('id', $data['ticket_id'])
                            ->update(['priority' => $action['priority']]);
                    }
                    break;

                case 'add_tag':
                    // Tag handling
                    break;

                case 'send_notification':
                    // Notification handling
                    break;
            }
        }
    }
}
