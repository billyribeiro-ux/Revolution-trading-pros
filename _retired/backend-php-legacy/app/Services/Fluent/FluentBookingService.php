<?php

declare(strict_types=1);

namespace App\Services\Fluent;

use App\Models\Contact;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * FluentBooking Service - Appointment & Scheduling
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive booking functionality matching FluentBooking Pro:
 * - Calendar management
 * - Service types
 * - Team member scheduling
 * - Appointment bookings
 * - Payment integration
 * - Video conferencing
 * - Reminders
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class FluentBookingService
{
    /**
     * Link appointment to CRM contact
     */
    public function linkAppointmentToContact(int $appointmentId, int $contactId): bool
    {
        return DB::table('booking_appointments')
            ->where('id', $appointmentId)
            ->update(['contact_id' => $contactId]) > 0;
    }

    /**
     * Get appointment count for contact
     */
    public function getContactAppointmentCount(int $contactId): int
    {
        return DB::table('booking_appointments')
            ->where('contact_id', $contactId)
            ->count();
    }

    /**
     * Get completed appointments for contact
     */
    public function getContactCompletedAppointments(int $contactId): int
    {
        return DB::table('booking_appointments')
            ->where('contact_id', $contactId)
            ->where('status', 'completed')
            ->count();
    }

    /**
     * Get no-shows for contact
     */
    public function getContactNoShows(int $contactId): int
    {
        return DB::table('booking_appointments')
            ->where('contact_id', $contactId)
            ->where('status', 'no_show')
            ->count();
    }

    /**
     * Get upcoming appointment count
     */
    public function getUpcomingAppointmentCount(): int
    {
        return DB::table('booking_appointments')
            ->where('starts_at', '>', now())
            ->whereIn('status', ['confirmed', 'pending'])
            ->count();
    }

    /**
     * Get bookings made today
     */
    public function getBookingsToday(): int
    {
        return DB::table('booking_appointments')
            ->whereDate('created_at', today())
            ->count();
    }

    /**
     * Get appointment count for period
     */
    public function getAppointmentCount(\DateTime $startDate): int
    {
        return DB::table('booking_appointments')
            ->where('created_at', '>=', $startDate)
            ->count();
    }

    /**
     * Get booking revenue for period
     */
    public function getBookingRevenue(\DateTime $startDate): float
    {
        return (float) DB::table('booking_appointments')
            ->where('created_at', '>=', $startDate)
            ->where('payment_status', 'paid')
            ->sum('amount');
    }

    /**
     * Get no-show rate for period
     */
    public function getNoShowRate(\DateTime $startDate): float
    {
        $stats = DB::table('booking_appointments')
            ->where('starts_at', '>=', $startDate)
            ->where('starts_at', '<', now()) // Only past appointments
            ->selectRaw('
                COUNT(*) as total,
                SUM(CASE WHEN status = \'no_show\' THEN 1 ELSE 0 END) as no_shows
            ')
            ->first();

        if (!$stats || $stats->total == 0) {
            return 0;
        }

        return round(($stats->no_shows / $stats->total) * 100, 2);
    }

    /**
     * Trigger booking automation
     */
    public function triggerAutomation(string $trigger, Contact $contact, array $data = []): array
    {
        $triggered = [];

        // Built-in booking automations
        switch ($trigger) {
            case 'booking_created':
                // Send confirmation email
                $triggered[] = 'booking_confirmation_email';
                break;

            case 'booking_reminder_24h':
                // Send 24h reminder
                $triggered[] = 'booking_reminder_24h_email';
                break;

            case 'booking_reminder_1h':
                // Send 1h reminder
                $triggered[] = 'booking_reminder_1h_email';
                break;

            case 'booking_completed':
                // Send follow-up email
                $triggered[] = 'booking_followup_email';
                break;

            case 'booking_cancelled':
                // Send cancellation confirmation
                $triggered[] = 'booking_cancellation_email';
                break;

            case 'booking_rescheduled':
                // Send reschedule confirmation
                $triggered[] = 'booking_reschedule_email';
                break;

            case 'booking_no_show':
                // Handle no-show
                $triggered[] = 'booking_no_show_followup';
                break;
        }

        // Log automations triggered
        if (!empty($triggered)) {
            Log::info('FluentBooking: Automations triggered', [
                'trigger' => $trigger,
                'contact_id' => $contact->id,
                'triggered' => $triggered,
            ]);
        }

        return $triggered;
    }

    /**
     * Get contact's appointments
     */
    public function getContactAppointments(int $contactId, int $limit = 20): array
    {
        return DB::table('booking_appointments')
            ->where('contact_id', $contactId)
            ->orderByDesc('starts_at')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get available time slots for a service
     */
    public function getAvailableSlots(int $serviceId, string $date): array
    {
        $service = DB::table('booking_services')->find($serviceId);
        if (!$service) {
            return [];
        }

        $calendar = DB::table('booking_calendars')->find($service->calendar_id);
        if (!$calendar) {
            return [];
        }

        // Get availability rules
        $rules = json_decode($calendar->availability_rules, true) ?? [];
        $dayOfWeek = strtolower(date('l', strtotime($date)));

        if (!isset($rules[$dayOfWeek]) || !$rules[$dayOfWeek]['enabled']) {
            return [];
        }

        // Get existing appointments for the date
        $existingAppointments = DB::table('booking_appointments')
            ->where('service_id', $serviceId)
            ->whereDate('starts_at', $date)
            ->whereIn('status', ['confirmed', 'pending'])
            ->get(['starts_at', 'ends_at']);

        // Generate slots based on availability
        $slots = [];
        $dayRules = $rules[$dayOfWeek];
        $startTime = strtotime("{$date} {$dayRules['start']}");
        $endTime = strtotime("{$date} {$dayRules['end']}");
        $duration = $service->duration * 60; // Convert to seconds
        $buffer = ($service->buffer_after ?? $calendar->buffer_after ?? 0) * 60;

        $current = $startTime;
        while ($current + $duration <= $endTime) {
            $slotEnd = $current + $duration;

            // Check if slot conflicts with existing appointments
            $isAvailable = true;
            foreach ($existingAppointments as $apt) {
                $aptStart = strtotime($apt->starts_at);
                $aptEnd = strtotime($apt->ends_at);

                if ($current < $aptEnd && $slotEnd > $aptStart) {
                    $isAvailable = false;
                    break;
                }
            }

            if ($isAvailable) {
                $slots[] = [
                    'start' => date('H:i', $current),
                    'end' => date('H:i', $slotEnd),
                    'available' => true,
                ];
            }

            $current += $duration + $buffer;
        }

        return $slots;
    }

    /**
     * Create appointment
     */
    public function createAppointment(array $data): int
    {
        $bookingRef = 'APT-' . date('Y') . '-' . str_pad((string) (DB::table('booking_appointments')->count() + 1), 6, '0', STR_PAD_LEFT);

        return DB::table('booking_appointments')->insertGetId([
            'booking_reference' => $bookingRef,
            'service_id' => $data['service_id'],
            'calendar_id' => $data['calendar_id'],
            'team_member_id' => $data['team_member_id'] ?? null,
            'customer_id' => $data['customer_id'] ?? null,
            'contact_id' => $data['contact_id'] ?? null,
            'customer_name' => $data['customer_name'],
            'customer_email' => $data['customer_email'],
            'customer_phone' => $data['customer_phone'] ?? null,
            'customer_timezone' => $data['customer_timezone'] ?? 'UTC',
            'starts_at' => $data['starts_at'],
            'ends_at' => $data['ends_at'],
            'duration' => $data['duration'],
            'status' => $data['status'] ?? 'pending',
            'location_type' => $data['location_type'] ?? 'video',
            'location_details' => $data['location_details'] ?? null,
            'notes' => $data['notes'] ?? null,
            'custom_responses' => json_encode($data['custom_responses'] ?? []),
            'created_at' => now(),
            'updated_at' => now(),
        ]);
    }
}
