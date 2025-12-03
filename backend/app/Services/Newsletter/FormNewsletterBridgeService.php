<?php

declare(strict_types=1);

namespace App\Services\Newsletter;

use App\Models\Form;
use App\Models\FormField;
use App\Models\FormSubmission;
use App\Models\NewsletterSubscription;
use App\Models\NewsletterCategory;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * FormNewsletterBridgeService
 *
 * Enterprise-grade service that bridges form submissions to newsletter subscriptions.
 * Handles automatic subscription, category assignment, and tracking.
 *
 * @level ICT11 Principal Engineer
 * @version 1.0.0
 */
class FormNewsletterBridgeService
{
    /**
     * Newsletter-specific field types
     */
    public const FIELD_NEWSLETTER_SUBSCRIBE = 'newsletter_subscribe';
    public const FIELD_NEWSLETTER_CATEGORIES = 'newsletter_categories';
    public const FIELD_NEWSLETTER_FREQUENCY = 'newsletter_frequency';

    /**
     * Default frequency options
     */
    public const FREQUENCY_OPTIONS = [
        'daily' => 'Daily Digest',
        'weekly' => 'Weekly Summary',
        'biweekly' => 'Bi-Weekly Updates',
        'monthly' => 'Monthly Newsletter',
    ];

    /**
     * Process a form submission for newsletter subscription
     */
    public function processSubmission(FormSubmission $submission): ?NewsletterSubscription
    {
        $form = $submission->form;

        if (!$form || !$this->formHasNewsletterFields($form)) {
            return null;
        }

        $submissionData = $this->getSubmissionData($submission);

        // Check if user opted in to newsletter
        if (!$this->hasNewsletterOptIn($form, $submissionData)) {
            Log::info('FormNewsletterBridge: User did not opt-in to newsletter', [
                'submission_id' => $submission->id,
                'form_id' => $form->id,
            ]);
            return null;
        }

        // Extract email from submission
        $email = $this->extractEmail($form, $submissionData);

        if (!$email) {
            Log::warning('FormNewsletterBridge: No email found in submission', [
                'submission_id' => $submission->id,
                'form_id' => $form->id,
            ]);
            return null;
        }

        return $this->createOrUpdateSubscription($submission, $email, $submissionData);
    }

    /**
     * Check if a form has newsletter-related fields
     */
    public function formHasNewsletterFields(Form $form): bool
    {
        return $form->fields()
            ->whereIn('field_type', [
                self::FIELD_NEWSLETTER_SUBSCRIBE,
                self::FIELD_NEWSLETTER_CATEGORIES,
            ])
            ->exists();
    }

    /**
     * Get newsletter fields from a form
     */
    public function getNewsletterFields(Form $form): array
    {
        return $form->fields()
            ->whereIn('field_type', [
                self::FIELD_NEWSLETTER_SUBSCRIBE,
                self::FIELD_NEWSLETTER_CATEGORIES,
                self::FIELD_NEWSLETTER_FREQUENCY,
            ])
            ->get()
            ->toArray();
    }

    /**
     * Check if user opted in to newsletter
     */
    private function hasNewsletterOptIn(Form $form, array $data): bool
    {
        $subscribeField = $form->fields()
            ->where('field_type', self::FIELD_NEWSLETTER_SUBSCRIBE)
            ->first();

        if (!$subscribeField) {
            // If no explicit subscribe field, check form settings
            return $form->settings['auto_subscribe_newsletter'] ?? false;
        }

        $fieldValue = $data[$subscribeField->name] ?? null;

        // Handle various truthy values
        return in_array($fieldValue, [true, 1, '1', 'yes', 'on', 'true'], true);
    }

    /**
     * Extract email from submission data
     */
    private function extractEmail(Form $form, array $data): ?string
    {
        // First, look for email field type
        $emailField = $form->fields()
            ->where('field_type', FormField::TYPE_EMAIL)
            ->first();

        if ($emailField && isset($data[$emailField->name])) {
            return $this->validateEmail($data[$emailField->name]);
        }

        // Fallback: look for common email field names
        $emailFieldNames = ['email', 'email_address', 'user_email', 'contact_email', 'e-mail'];

        foreach ($emailFieldNames as $name) {
            if (isset($data[$name]) && $this->validateEmail($data[$name])) {
                return $data[$name];
            }
        }

        return null;
    }

    /**
     * Validate and sanitize email
     */
    private function validateEmail(?string $email): ?string
    {
        if (!$email) {
            return null;
        }

        $email = trim(strtolower($email));

        return filter_var($email, FILTER_VALIDATE_EMAIL) ? $email : null;
    }

    /**
     * Get submission data as array
     */
    private function getSubmissionData(FormSubmission $submission): array
    {
        $data = [];

        foreach ($submission->data as $fieldData) {
            $data[$fieldData->field_name] = $fieldData->value;
        }

        return $data;
    }

    /**
     * Create or update newsletter subscription
     */
    private function createOrUpdateSubscription(
        FormSubmission $submission,
        string $email,
        array $data
    ): NewsletterSubscription {
        $form = $submission->form;

        // Extract subscriber details
        $name = $this->extractName($form, $data);
        $categories = $this->extractCategories($form, $data);
        $frequency = $this->extractFrequency($form, $data);
        $interests = $this->extractInterests($form, $data);

        return DB::transaction(function () use (
            $submission,
            $email,
            $name,
            $categories,
            $frequency,
            $interests,
            $form
        ) {
            $subscription = NewsletterSubscription::updateOrCreate(
                ['email' => $email],
                [
                    'name' => $name['full'] ?? null,
                    'first_name' => $name['first'] ?? null,
                    'last_name' => $name['last'] ?? null,
                    'status' => NewsletterSubscription::STATUS_PENDING,
                    'source' => 'form',
                    'referring_url' => $submission->referrer,
                    'ip_address' => $submission->ip_address,
                    'user_agent' => $submission->user_agent,
                    'verification_token' => Str::random(64),
                    'unsubscribe_token' => Str::random(64),
                    'double_opt_in' => true,
                    'consent_given_at' => now(),
                    'consent_ip' => $submission->ip_address,
                    'consent_method' => 'form_submission',
                ]
            );

            // Update preferences
            $preferences = $subscription->preferences ?? [];
            $preferences['frequency'] = $frequency;
            $preferences['topics'] = $categories;
            $subscription->preferences = $preferences;

            // Update segments/categories
            $segments = $subscription->segments ?? [];
            $segments = array_unique(array_merge($segments, $categories));
            $subscription->segments = $segments;

            // Update interests
            if (!empty($interests)) {
                $currentInterests = $subscription->interests ?? [];
                $subscription->interests = array_unique(array_merge($currentInterests, $interests));
            }

            // Track source metadata
            $metadata = $subscription->metadata ?? [];
            $metadata['form_id'] = $form->id;
            $metadata['form_name'] = $form->name;
            $metadata['submission_id'] = $submission->id;
            $metadata['subscribed_via_form_at'] = now()->toIso8601String();
            $subscription->metadata = $metadata;

            // Update UTM parameters if available
            if ($form->settings['track_utm'] ?? true) {
                $submission->refresh();
                $utmData = $submission->metadata['utm'] ?? [];
                if (!empty($utmData)) {
                    $subscription->utm_source = $utmData['source'] ?? $subscription->utm_source;
                    $subscription->utm_medium = $utmData['medium'] ?? $subscription->utm_medium;
                    $subscription->utm_campaign = $utmData['campaign'] ?? $subscription->utm_campaign;
                }
            }

            $subscription->save();

            // Link submission to subscription
            $submission->update([
                'newsletter_subscription_id' => $subscription->id,
            ]);

            // Update category subscriber counts
            $this->updateCategorySubscriberCounts($categories);

            Log::info('FormNewsletterBridge: Created/updated subscription', [
                'subscription_id' => $subscription->id,
                'email' => $email,
                'form_id' => $form->id,
                'categories' => $categories,
            ]);

            return $subscription;
        });
    }

    /**
     * Extract name from submission data
     */
    private function extractName(Form $form, array $data): array
    {
        $name = [
            'full' => null,
            'first' => null,
            'last' => null,
        ];

        // Look for first_name field
        $firstNameFields = ['first_name', 'firstname', 'fname', 'given_name'];
        foreach ($firstNameFields as $field) {
            if (!empty($data[$field])) {
                $name['first'] = trim($data[$field]);
                break;
            }
        }

        // Look for last_name field
        $lastNameFields = ['last_name', 'lastname', 'lname', 'family_name', 'surname'];
        foreach ($lastNameFields as $field) {
            if (!empty($data[$field])) {
                $name['last'] = trim($data[$field]);
                break;
            }
        }

        // Look for full name field
        $nameFields = ['name', 'full_name', 'fullname', 'your_name'];
        foreach ($nameFields as $field) {
            if (!empty($data[$field])) {
                $name['full'] = trim($data[$field]);

                // If we don't have first/last, try to split
                if (!$name['first'] && !$name['last']) {
                    $parts = explode(' ', $name['full'], 2);
                    $name['first'] = $parts[0] ?? null;
                    $name['last'] = $parts[1] ?? null;
                }
                break;
            }
        }

        // Build full name if not set
        if (!$name['full'] && ($name['first'] || $name['last'])) {
            $name['full'] = trim(($name['first'] ?? '') . ' ' . ($name['last'] ?? ''));
        }

        return $name;
    }

    /**
     * Extract selected newsletter categories
     */
    private function extractCategories(Form $form, array $data): array
    {
        $categoryField = $form->fields()
            ->where('field_type', self::FIELD_NEWSLETTER_CATEGORIES)
            ->first();

        if (!$categoryField) {
            // Return default categories from form settings
            return $form->settings['default_newsletter_categories'] ?? [];
        }

        $value = $data[$categoryField->name] ?? [];

        // Handle string (single selection) or array (multiple)
        if (is_string($value)) {
            return [$value];
        }

        return is_array($value) ? $value : [];
    }

    /**
     * Extract newsletter frequency preference
     */
    private function extractFrequency(Form $form, array $data): string
    {
        $frequencyField = $form->fields()
            ->where('field_type', self::FIELD_NEWSLETTER_FREQUENCY)
            ->first();

        if (!$frequencyField) {
            return $form->settings['default_newsletter_frequency'] ?? 'weekly';
        }

        $value = $data[$frequencyField->name] ?? 'weekly';

        // Validate frequency value
        return array_key_exists($value, self::FREQUENCY_OPTIONS) ? $value : 'weekly';
    }

    /**
     * Extract interests from form data
     */
    private function extractInterests(Form $form, array $data): array
    {
        $interests = [];

        // Look for interest-related fields
        $interestFields = $form->fields()
            ->where(function ($query) {
                $query->where('name', 'like', '%interest%')
                    ->orWhere('name', 'like', '%topic%')
                    ->orWhere('name', 'like', '%preference%');
            })
            ->whereIn('field_type', [
                FormField::TYPE_CHECKBOX,
                FormField::TYPE_MULTISELECT,
                FormField::TYPE_SELECT,
            ])
            ->get();

        foreach ($interestFields as $field) {
            $value = $data[$field->name] ?? null;
            if ($value) {
                if (is_array($value)) {
                    $interests = array_merge($interests, $value);
                } else {
                    $interests[] = $value;
                }
            }
        }

        return array_unique($interests);
    }

    /**
     * Update subscriber counts for categories
     */
    private function updateCategorySubscriberCounts(array $categorySlugs): void
    {
        if (empty($categorySlugs)) {
            return;
        }

        NewsletterCategory::whereIn('slug', $categorySlugs)
            ->each(function (NewsletterCategory $category) {
                $category->updateSubscriberCount();
            });
    }

    /**
     * Get available newsletter categories for forms
     */
    public function getAvailableCategories(): array
    {
        return NewsletterCategory::where('is_active', true)
            ->orderBy('sort_order')
            ->get()
            ->map(fn ($cat) => [
                'value' => $cat->slug,
                'label' => $cat->name,
                'description' => $cat->description,
            ])
            ->toArray();
    }

    /**
     * Create newsletter form field configuration
     */
    public function createNewsletterFieldConfig(string $type, array $options = []): array
    {
        return match ($type) {
            self::FIELD_NEWSLETTER_SUBSCRIBE => [
                'field_type' => self::FIELD_NEWSLETTER_SUBSCRIBE,
                'label' => $options['label'] ?? 'Subscribe to our newsletter',
                'name' => $options['name'] ?? 'newsletter_subscribe',
                'help_text' => $options['help_text'] ?? 'Receive updates and exclusive content',
                'default_value' => $options['default_checked'] ?? false,
                'required' => false,
                'options' => [
                    'checkbox_label' => $options['checkbox_label'] ?? 'Yes, I want to receive newsletters',
                    'show_privacy_link' => $options['show_privacy_link'] ?? true,
                    'privacy_url' => $options['privacy_url'] ?? '/privacy-policy',
                ],
            ],
            self::FIELD_NEWSLETTER_CATEGORIES => [
                'field_type' => self::FIELD_NEWSLETTER_CATEGORIES,
                'label' => $options['label'] ?? 'Newsletter Topics',
                'name' => $options['name'] ?? 'newsletter_categories',
                'help_text' => $options['help_text'] ?? 'Select the topics you\'re interested in',
                'required' => false,
                'options' => $options['categories'] ?? $this->getAvailableCategories(),
                'attributes' => [
                    'multiple' => true,
                    'min_selections' => $options['min_selections'] ?? 0,
                    'max_selections' => $options['max_selections'] ?? 10,
                ],
            ],
            self::FIELD_NEWSLETTER_FREQUENCY => [
                'field_type' => self::FIELD_NEWSLETTER_FREQUENCY,
                'label' => $options['label'] ?? 'Email Frequency',
                'name' => $options['name'] ?? 'newsletter_frequency',
                'help_text' => $options['help_text'] ?? 'How often would you like to hear from us?',
                'default_value' => $options['default'] ?? 'weekly',
                'required' => false,
                'options' => array_map(
                    fn ($value, $label) => ['value' => $value, 'label' => $label],
                    array_keys(self::FREQUENCY_OPTIONS),
                    array_values(self::FREQUENCY_OPTIONS)
                ),
            ],
            default => throw new \InvalidArgumentException("Unknown newsletter field type: {$type}"),
        };
    }

    /**
     * Get analytics for form-based newsletter signups
     */
    public function getFormSignupAnalytics(int $formId = null, string $period = 'month'): array
    {
        $query = NewsletterSubscription::where('source', 'form');

        if ($formId) {
            $query->whereJsonContains('metadata->form_id', $formId);
        }

        $startDate = match ($period) {
            'day' => now()->startOfDay(),
            'week' => now()->startOfWeek(),
            'month' => now()->startOfMonth(),
            'year' => now()->startOfYear(),
            default => now()->startOfMonth(),
        };

        $subscriptions = $query->where('created_at', '>=', $startDate)->get();

        // Group by category
        $byCategory = [];
        foreach ($subscriptions as $sub) {
            foreach ($sub->segments ?? [] as $segment) {
                $byCategory[$segment] = ($byCategory[$segment] ?? 0) + 1;
            }
        }

        return [
            'total_signups' => $subscriptions->count(),
            'verified' => $subscriptions->where('status', 'active')->count(),
            'pending' => $subscriptions->where('status', 'pending')->count(),
            'by_category' => $byCategory,
            'by_frequency' => $subscriptions->groupBy(fn ($s) => $s->preferences['frequency'] ?? 'unknown')
                ->map->count()
                ->toArray(),
            'period' => $period,
            'start_date' => $startDate->toDateString(),
        ];
    }
}
