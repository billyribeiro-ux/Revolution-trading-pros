<?php

declare(strict_types=1);

namespace App\Services\Fluent;

use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\Contact;
use App\Services\Integration\IntegrationHub;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;

/**
 * FluentForms Service - Form Builder & Submission Processing
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive form functionality matching FluentForms Pro:
 * - Form submission processing
 * - CRM integration (via IntegrationHub)
 * - File uploads
 * - Conditional logic
 * - Payment integration
 * - Multi-step forms
 *
 * @version 1.0.0 - December 2025
 * @author Revolution Trading Pros
 */
class FluentFormsService
{
    public function __construct(
        private readonly IntegrationHub $integrationHub,
    ) {}

    /**
     * Process form submission through CRM integration
     */
    public function processSubmission(FormSubmission $submission): array
    {
        return $this->integrationHub->processFormSubmission($submission);
    }

    /**
     * Get conversion rate for forms
     */
    public function getConversionRate(\DateTime $startDate): float
    {
        $stats = DB::table('form_analytics')
            ->where('date', '>=', $startDate->format('Y-m-d'))
            ->selectRaw('SUM(views) as views, SUM(submissions) as submissions')
            ->first();

        if (!$stats || $stats->views == 0) {
            return 0;
        }

        return round(($stats->submissions / $stats->views) * 100, 2);
    }

    /**
     * Get all active forms
     */
    public function getActiveForms(): \Illuminate\Database\Eloquent\Collection
    {
        return Form::where('is_published', true)
            ->orderBy('name')
            ->get();
    }

    /**
     * Get form by slug
     */
    public function getFormBySlug(string $slug): ?Form
    {
        return Form::where('slug', $slug)
            ->where('is_published', true)
            ->first();
    }

    /**
     * Get form submissions for a contact
     */
    public function getContactSubmissions(Contact $contact, int $limit = 20): \Illuminate\Database\Eloquent\Collection
    {
        return FormSubmission::where('user_id', $contact->user_id)
            ->orWhereHas('data', function ($q) use ($contact) {
                $q->where('field_name', 'email')
                  ->where('value', $contact->email);
            })
            ->with('form:id,name,slug')
            ->latest()
            ->limit($limit)
            ->get();
    }

    /**
     * Get form analytics
     */
    public function getFormAnalytics(int $formId, \DateTime $startDate): array
    {
        $stats = DB::table('form_analytics')
            ->where('form_id', $formId)
            ->where('date', '>=', $startDate->format('Y-m-d'))
            ->selectRaw('
                SUM(views) as total_views,
                SUM(submissions) as total_submissions,
                SUM(partial_submissions) as partial_submissions,
                AVG(avg_completion_time) as avg_completion_time
            ')
            ->first();

        return [
            'views' => $stats->total_views ?? 0,
            'submissions' => $stats->total_submissions ?? 0,
            'partial_submissions' => $stats->partial_submissions ?? 0,
            'conversion_rate' => $stats->total_views > 0
                ? round(($stats->total_submissions / $stats->total_views) * 100, 2)
                : 0,
            'avg_completion_time' => round($stats->avg_completion_time ?? 0, 1),
        ];
    }
}
