<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Carbon;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * Form Model - Enterprise-grade form management system
 * 
 * Comprehensive form builder with field management, submission tracking,
 * validation, notifications, and analytics capabilities.
 *
 * @property int $id
 * @property string $name Form name
 * @property string $slug Unique URL slug
 * @property string|null $description Form description
 * @property array|null $settings Additional form settings
 * @property bool $active Active status
 * @property bool $accept_submissions Accepting submissions flag
 * @property string|null $submit_button_text Submit button label
 * @property string|null $success_message Success message text
 * @property string|null $error_message Error message text
 * @property string|null $redirect_url Post-submission redirect URL
 * @property array|null $notification_settings Email notification config
 * @property array|null $anti_spam_settings Anti-spam configuration
 * @property int $submission_limit Maximum submissions allowed
 * @property Carbon|null $expires_at Form expiration timestamp
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read Collection<int, FormField> $fields
 * @property-read Collection<int, FormSubmission> $submissions
 * @property-read int $fields_count
 * @property-read int $submissions_count
 * @property-read bool $is_accepting
 * @property-read bool $is_expired
 * @property-read int|null $submissions_remaining
 * @property-read float $completion_rate
 * 
 * @method static Builder active()
 * @method static Builder inactive()
 * @method static Builder accepting()
 * @method static Builder notExpired()
 * @method static Builder expired()
 * @method static Builder bySlug(string $slug)
 * @method static Builder withRelations()
 * @method static Builder popular(int $minSubmissions = 10)
 */
class Form extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * The table associated with the model.
     */
    protected $table = 'forms';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'name',
        'slug',
        'description',
        'settings',
        'active',
        'accept_submissions',
        'submit_button_text',
        'success_message',
        'error_message',
        'redirect_url',
        'notification_settings',
        'anti_spam_settings',
        'submission_limit',
        'expires_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'settings' => 'array',
        'notification_settings' => 'array',
        'anti_spam_settings' => 'array',
        'active' => 'boolean',
        'accept_submissions' => 'boolean',
        'submission_limit' => 'integer',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'active' => true,
        'accept_submissions' => true,
        'submit_button_text' => 'Submit',
        'success_message' => 'Thank you for your submission!',
        'error_message' => 'Please correct the errors and try again.',
        'submission_limit' => 0,
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'is_accepting',
        'is_expired',
        'submissions_remaining',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const STATUS_ACTIVE = 'active';
    public const STATUS_INACTIVE = 'inactive';

    public const ANTI_SPAM_HONEYPOT = 'honeypot';
    public const ANTI_SPAM_RECAPTCHA = 'recaptcha';
    public const ANTI_SPAM_TIME_THRESHOLD = 'time_threshold';
    public const ANTI_SPAM_RATE_LIMIT = 'rate_limit';

    /**
     * Default form settings.
     *
     * @var array<string, mixed>
     */
    public const DEFAULT_SETTINGS = [
        'show_labels' => true,
        'show_placeholders' => true,
        'show_required_asterisk' => true,
        'ajax_submit' => true,
        'store_ip' => true,
        'store_user_agent' => true,
    ];

    /**
     * Default anti-spam settings.
     *
     * @var array<string, mixed>
     */
    public const DEFAULT_ANTI_SPAM = [
        'enabled' => true,
        'honeypot' => true,
        'time_threshold' => 3, // seconds
        'rate_limit' => 5, // submissions per hour
    ];

    // =========================================================================
    // BOOT & LIFECYCLE EVENTS
    // =========================================================================

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        // Auto-generate slug on creation
        static::creating(function (Form $form) {
            if (empty($form->slug)) {
                $form->slug = $form->generateUniqueSlug($form->name);
            }

            // Apply default settings
            if (empty($form->settings)) {
                $form->settings = self::DEFAULT_SETTINGS;
            }

            if (empty($form->anti_spam_settings)) {
                $form->anti_spam_settings = self::DEFAULT_ANTI_SPAM;
            }
        });

        // Update slug if name changes
        static::updating(function (Form $form) {
            if ($form->isDirty('name') && $form->getSetting('auto_update_slug', true)) {
                $form->slug = $form->generateUniqueSlug($form->name);
            }
        });

        // Cascade delete or soft delete relationships
        static::deleting(function (Form $form) {
            if ($form->isForceDeleting()) {
                $form->fields()->forceDelete();
                $form->submissions()->forceDelete();
            } else {
                $form->fields()->delete();
            }
            
            Log::info('Form deleted', [
                'id' => $form->id,
                'name' => $form->name,
                'slug' => $form->slug,
                'force' => $form->isForceDeleting(),
            ]);

            // Clear cache
            Cache::forget("form_stats_{$form->id}");
            Cache::forget("form_public_config_{$form->slug}");
        });

        // Clear cache on updates
        static::saved(function (Form $form) {
            Cache::forget("form_stats_{$form->id}");
            Cache::forget("form_public_config_{$form->slug}");
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get all fields for this form.
     */
    public function fields(): HasMany
    {
        return $this->hasMany(FormField::class)->orderBy('order');
    }

    /**
     * Get all submissions for this form.
     */
    public function submissions(): HasMany
    {
        return $this->hasMany(FormSubmission::class)->latest();
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include active forms.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('active', true);
    }

    /**
     * Scope a query to only include inactive forms.
     */
    public function scopeInactive(Builder $query): Builder
    {
        return $query->where('active', false);
    }

    /**
     * Scope a query to forms accepting submissions.
     */
    public function scopeAccepting(Builder $query): Builder
    {
        return $query->where('active', true)
            ->where('accept_submissions', true)
            ->where(function (Builder $q) {
                $q->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            });
    }

    /**
     * Scope a query to forms not expired.
     */
    public function scopeNotExpired(Builder $query): Builder
    {
        return $query->where(function (Builder $q) {
            $q->whereNull('expires_at')
                ->orWhere('expires_at', '>', now());
        });
    }

    /**
     * Scope a query to expired forms.
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->whereNotNull('expires_at')
            ->where('expires_at', '<=', now());
    }

    /**
     * Scope a query by slug.
     */
    public function scopeBySlug(Builder $query, string $slug): Builder
    {
        return $query->where('slug', $slug);
    }

    /**
     * Scope to eager load common relationships.
     */
    public function scopeWithRelations(Builder $query): Builder
    {
        return $query->with(['fields' => function ($q) {
            $q->active()->ordered();
        }]);
    }

    /**
     * Scope to popular forms (high submission count).
     */
    public function scopePopular(Builder $query, int $minSubmissions = 10): Builder
    {
        return $query->withCount('submissions')
            ->having('submissions_count', '>=', $minSubmissions)
            ->orderBy('submissions_count', 'desc');
    }

    /**
     * Scope to forms created recently.
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope to search forms by name or description.
     */
    public function scopeSearch(Builder $query, string $searchTerm): Builder
    {
        return $query->where(function (Builder $q) use ($searchTerm) {
            $q->where('name', 'LIKE', "%{$searchTerm}%")
                ->orWhere('description', 'LIKE', "%{$searchTerm}%")
                ->orWhere('slug', 'LIKE', "%{$searchTerm}%");
        });
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Check if form is accepting submissions.
     */
    public function getIsAcceptingAttribute(): bool
    {
        return $this->canAcceptSubmissions();
    }

    /**
     * Check if form is expired.
     */
    public function getIsExpiredAttribute(): bool
    {
        return $this->isExpired();
    }

    /**
     * Get remaining submissions count.
     */
    public function getSubmissionsRemainingAttribute(): ?int
    {
        if ($this->submission_limit <= 0) {
            return null;
        }

        $used = $this->submissions()->count();
        $remaining = $this->submission_limit - $used;
        
        return max(0, $remaining);
    }

    /**
     * Get form completion rate.
     */
    public function getCompletionRateAttribute(): float
    {
        $total = $this->submissions()->count();
        if ($total === 0) {
            return 0.0;
        }

        $completed = $this->submissions()
            ->whereHas('data', function ($q) {
                $q->whereNotNull('value')
                    ->where('value', '!=', '');
            })
            ->count();

        return round(($completed / $total) * 100, 2);
    }

    /**
     * Ensure slug is unique and URL-safe.
     */
    public function setSlugAttribute(?string $value): void
    {
        $this->attributes['slug'] = $value ? Str::slug($value) : null;
    }

    // =========================================================================
    // STATUS & VALIDATION METHODS
    // =========================================================================

    /**
     * Check if form is expired.
     */
    public function isExpired(): bool
    {
        return $this->expires_at && $this->expires_at->isPast();
    }

    /**
     * Check if submission limit reached.
     */
    public function hasReachedSubmissionLimit(): bool
    {
        if ($this->submission_limit <= 0) {
            return false;
        }

        return $this->submissions()->count() >= $this->submission_limit;
    }

    /**
     * Check if form can accept submissions.
     */
    public function canAcceptSubmissions(): bool
    {
        return $this->active 
            && $this->accept_submissions 
            && !$this->isExpired()
            && !$this->hasReachedSubmissionLimit();
    }

    /**
     * Check if form is published.
     */
    public function isPublished(): bool
    {
        return $this->active;
    }

    /**
     * Check if form has fields.
     */
    public function hasFields(): bool
    {
        return $this->fields()->exists();
    }

    /**
     * Check if form has submissions.
     */
    public function hasSubmissions(): bool
    {
        return $this->submissions()->exists();
    }

    // =========================================================================
    // ACTIVATION & STATUS MANAGEMENT
    // =========================================================================

    /**
     * Activate the form.
     */
    public function activate(): bool
    {
        $this->active = true;
        return $this->save();
    }

    /**
     * Deactivate the form.
     */
    public function deactivate(): bool
    {
        $this->active = false;
        return $this->save();
    }

    /**
     * Toggle active status.
     */
    public function toggleActive(): bool
    {
        $this->active = !$this->active;
        return $this->save();
    }

    /**
     * Enable submissions.
     */
    public function enableSubmissions(): bool
    {
        $this->accept_submissions = true;
        return $this->save();
    }

    /**
     * Disable submissions.
     */
    public function disableSubmissions(): bool
    {
        $this->accept_submissions = false;
        return $this->save();
    }

    /**
     * Toggle submission acceptance.
     */
    public function toggleSubmissions(): bool
    {
        $this->accept_submissions = !$this->accept_submissions;
        return $this->save();
    }

    /**
     * Set expiration date.
     */
    public function setExpiration(Carbon $date): bool
    {
        $this->expires_at = $date;
        return $this->save();
    }

    /**
     * Remove expiration date.
     */
    public function removeExpiration(): bool
    {
        $this->expires_at = null;
        return $this->save();
    }

    /**
     * Extend expiration by days.
     */
    public function extendExpiration(int $days): bool
    {
        if ($this->expires_at) {
            $this->expires_at = $this->expires_at->addDays($days);
        } else {
            $this->expires_at = now()->addDays($days);
        }
        return $this->save();
    }

    // =========================================================================
    // FIELD MANAGEMENT METHODS
    // =========================================================================

    /**
     * Get active fields.
     */
    public function getActiveFields(): Collection
    {
        return $this->fields()->active()->ordered()->get();
    }

    /**
     * Get input fields (non-layout).
     */
    public function getInputFields(): Collection
    {
        return $this->fields()->active()->inputFields()->ordered()->get();
    }

    /**
     * Get required fields.
     */
    public function getRequiredFields(): Collection
    {
        return $this->fields()->active()->required()->ordered()->get();
    }

    /**
     * Get field by name.
     */
    public function getFieldByName(string $name): ?FormField
    {
        return $this->fields()->where('name', $name)->first();
    }

    /**
     * Check if field exists.
     */
    public function hasField(string $name): bool
    {
        return $this->fields()->where('name', $name)->exists();
    }

    /**
     * Add field to form.
     */
    public function addField(array $fieldData): FormField
    {
        $fieldData['order'] = $this->fields()->max('order') + 1;
        return $this->fields()->create($fieldData);
    }

    /**
     * Remove field from form.
     */
    public function removeField(int $fieldId): bool
    {
        return $this->fields()->where('id', $fieldId)->delete() > 0;
    }

    /**
     * Reorder fields.
     *
     * @param array<int, int> $orderMap [field_id => new_order]
     */
    public function reorderFields(array $orderMap): int
    {
        return FormField::bulkReorder($orderMap);
    }

    // =========================================================================
    // VALIDATION METHODS
    // =========================================================================

    /**
     * Get validation rules for all fields.
     */
    public function getValidationRules(): array
    {
        $rules = [];
        
        foreach ($this->getInputFields() as $field) {
            $rules = array_merge($rules, $field->getValidationArray());
        }

        return $rules;
    }

    /**
     * Validate submission data against form rules.
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function validateSubmissionData(array $data): bool
    {
        $validator = \Validator::make($data, $this->getValidationRules());

        if ($validator->fails()) {
            throw new \Illuminate\Validation\ValidationException($validator);
        }

        return true;
    }

    /**
     * Validate conditional logic for fields.
     */
    public function evaluateConditionalFields(array $data): Collection
    {
        return $this->getActiveFields()->filter(function (FormField $field) use ($data) {
            return $field->evaluateConditionalLogic($data);
        });
    }

    // =========================================================================
    // SETTINGS MANAGEMENT
    // =========================================================================

    /**
     * Get setting value by key.
     */
    public function getSetting(string $key, mixed $default = null): mixed
    {
        return $this->settings[$key] ?? $default;
    }

    /**
     * Set setting value.
     */
    public function setSetting(string $key, mixed $value): bool
    {
        $settings = $this->settings ?? [];
        $settings[$key] = $value;
        $this->settings = $settings;
        return $this->save();
    }

    /**
     * Remove setting.
     */
    public function removeSetting(string $key): bool
    {
        $settings = $this->settings ?? [];
        unset($settings[$key]);
        $this->settings = $settings;
        return $this->save();
    }

    /**
     * Bulk update settings.
     */
    public function updateSettings(array $settings): bool
    {
        $this->settings = array_merge($this->settings ?? [], $settings);
        return $this->save();
    }

    /**
     * Reset settings to defaults.
     */
    public function resetSettings(): bool
    {
        $this->settings = self::DEFAULT_SETTINGS;
        return $this->save();
    }

    // =========================================================================
    // NOTIFICATION MANAGEMENT
    // =========================================================================

    /**
     * Get notification emails.
     */
    public function getNotificationEmails(): array
    {
        return $this->notification_settings['emails'] ?? [];
    }

    /**
     * Add notification email.
     */
    public function addNotificationEmail(string $email): bool
    {
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new \InvalidArgumentException("Invalid email address: {$email}");
        }

        $settings = $this->notification_settings ?? [];
        $emails = $settings['emails'] ?? [];
        
        if (!in_array($email, $emails, true)) {
            $emails[] = $email;
            $settings['emails'] = $emails;
            $this->notification_settings = $settings;
            return $this->save();
        }

        return true;
    }

    /**
     * Remove notification email.
     */
    public function removeNotificationEmail(string $email): bool
    {
        $settings = $this->notification_settings ?? [];
        $emails = $settings['emails'] ?? [];
        
        $emails = array_values(array_filter($emails, fn($e) => $e !== $email));
        $settings['emails'] = $emails;
        $this->notification_settings = $settings;
        
        return $this->save();
    }

    /**
     * Check if should notify on submission.
     */
    public function shouldNotifyOnSubmission(): bool
    {
        return ($this->notification_settings['enabled'] ?? false) 
            && !empty($this->getNotificationEmails());
    }

    /**
     * Enable notifications.
     */
    public function enableNotifications(): bool
    {
        $settings = $this->notification_settings ?? [];
        $settings['enabled'] = true;
        $this->notification_settings = $settings;
        return $this->save();
    }

    /**
     * Disable notifications.
     */
    public function disableNotifications(): bool
    {
        $settings = $this->notification_settings ?? [];
        $settings['enabled'] = false;
        $this->notification_settings = $settings;
        return $this->save();
    }

    // =========================================================================
    // ANTI-SPAM METHODS
    // =========================================================================

    /**
     * Check if anti-spam is enabled.
     */
    public function isAntiSpamEnabled(): bool
    {
        return $this->anti_spam_settings['enabled'] ?? true;
    }

    /**
     * Check if honeypot is enabled.
     */
    public function hasHoneypot(): bool
    {
        return $this->anti_spam_settings['honeypot'] ?? true;
    }

    /**
     * Get time threshold for submissions (seconds).
     */
    public function getTimeThreshold(): int
    {
        return $this->anti_spam_settings['time_threshold'] ?? 3;
    }

    /**
     * Get rate limit per hour.
     */
    public function getRateLimit(): int
    {
        return $this->anti_spam_settings['rate_limit'] ?? 5;
    }

    /**
     * Update anti-spam settings.
     */
    public function updateAntiSpamSettings(array $settings): bool
    {
        $this->anti_spam_settings = array_merge(
            $this->anti_spam_settings ?? self::DEFAULT_ANTI_SPAM,
            $settings
        );
        return $this->save();
    }

    // =========================================================================
    // ANALYTICS & STATISTICS
    // =========================================================================

    /**
     * Get comprehensive submission statistics.
     */
    public function getSubmissionStats(): array
    {
        return Cache::remember("form_stats_{$this->id}", 300, function () {
            return [
                'total' => $this->submissions()->count(),
                'unread' => $this->submissions()->unread()->count(),
                'read' => $this->submissions()->read()->count(),
                'starred' => $this->submissions()->starred()->count(),
                'archived' => $this->submissions()->archived()->count(),
                'spam' => $this->submissions()->spam()->count(),
                'recent_24h' => $this->submissions()->where('created_at', '>=', now()->subDay())->count(),
                'recent_7d' => $this->submissions()->where('created_at', '>=', now()->subWeek())->count(),
                'recent_30d' => $this->submissions()->where('created_at', '>=', now()->subMonth())->count(),
                'completion_rate' => $this->completion_rate,
                'avg_per_day' => $this->getAverageSubmissionsPerDay(),
            ];
        });
    }

    /**
     * Get average submissions per day.
     */
    public function getAverageSubmissionsPerDay(): float
    {
        $days = max(1, $this->created_at->diffInDays(now()));
        $total = $this->submissions()->count();
        return round($total / $days, 2);
    }

    /**
     * Get submission trend data.
     */
    public function getSubmissionTrend(int $days = 30): array
    {
        $submissions = $this->submissions()
            ->where('created_at', '>=', now()->subDays($days))
            ->selectRaw('DATE(created_at) as date, COUNT(*) as count')
            ->groupBy('date')
            ->orderBy('date')
            ->get();

        return $submissions->pluck('count', 'date')->toArray();
    }

    /**
     * Get field response rates.
     */
    public function getFieldResponseRates(): array
    {
        $totalSubmissions = $this->submissions()->count();
        if ($totalSubmissions === 0) {
            return [];
        }

        $rates = [];
        foreach ($this->getInputFields() as $field) {
            $responses = $field->submissions()
                ->whereNotNull('value')
                ->where('value', '!=', '')
                ->count();
            
            $rates[$field->name] = [
                'label' => $field->label,
                'responses' => $responses,
                'rate' => round(($responses / $totalSubmissions) * 100, 2),
            ];
        }

        return $rates;
    }

    // =========================================================================
    // EXPORT & RENDERING
    // =========================================================================

    /**
     * Get form as public configuration (for frontend).
     */
    public function toPublicConfig(): array
    {
        return Cache::remember("form_public_config_{$this->slug}", 600, function () {
            return [
                'id' => $this->id,
                'name' => $this->name,
                'slug' => $this->slug,
                'description' => $this->description,
                'submit_button_text' => $this->submit_button_text,
                'success_message' => $this->success_message,
                'error_message' => $this->error_message,
                'redirect_url' => $this->redirect_url,
                'fields' => $this->getActiveFields()->map->toFormConfig(),
                'settings' => $this->settings,
                'is_accepting' => $this->is_accepting,
                'expires_at' => $this->expires_at?->toIso8601String(),
                'submissions_remaining' => $this->submissions_remaining,
                'anti_spam' => [
                    'honeypot' => $this->hasHoneypot(),
                    'time_threshold' => $this->getTimeThreshold(),
                ],
            ];
        });
    }

    /**
     * Export all submissions.
     */
    public function exportSubmissions(string $format = 'csv'): array
    {
        $submissions = $this->submissions()->with('data')->get();
        $exported = [];

        foreach ($submissions as $submission) {
            $exported[] = $submission->toExportArray();
        }

        return $exported;
    }

    /**
     * Get form summary for dashboard.
     */
    public function toSummary(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'slug' => $this->slug,
            'active' => $this->active,
            'is_accepting' => $this->is_accepting,
            'fields_count' => $this->fields()->count(),
            'submissions_count' => $this->submissions()->count(),
            'unread_count' => $this->submissions()->unread()->count(),
            'expires_at' => $this->expires_at?->toDateString(),
            'created_at' => $this->created_at->toDateString(),
        ];
    }

    // =========================================================================
    // DUPLICATION & FACTORY METHODS
    // =========================================================================

    /**
     * Duplicate form with all fields.
     */
    public function duplicate(string $newName): self
    {
        return \DB::transaction(function () use ($newName) {
            $duplicate = $this->replicate(['slug']);
            $duplicate->name = $newName;
            $duplicate->slug = $this->generateUniqueSlug($newName);
            $duplicate->save();

            foreach ($this->fields as $field) {
                $field->duplicate($duplicate->id);
            }

            return $duplicate->load('fields');
        });
    }

    /**
     * Create form with fields in one transaction.
     */
    public static function createWithFields(string $name, array $fieldsData, array $formData = []): self
    {
        return \DB::transaction(function () use ($name, $fieldsData, $formData) {
            $formData['name'] = $name;
            $form = self::create($formData);

            foreach ($fieldsData as $index => $fieldData) {
                $fieldData['order'] = $index;
                $form->fields()->create($fieldData);
            }

            return $form->load('fields');
        });
    }

    /**
     * Generate unique slug.
     */
    protected function generateUniqueSlug(string $name): string
    {
        $slug = Str::slug($name);
        $originalSlug = $slug;
        $counter = 1;

        while (self::where('slug', $slug)->where('id', '!=', $this->id ?? 0)->exists()) {
            $slug = $originalSlug . '-' . $counter;
            $counter++;
        }

        return $slug;
    }

    // =========================================================================
    // BULK OPERATIONS
    // =========================================================================

    /**
     * Bulk activate forms.
     *
     * @param array<int> $ids
     */
    public static function bulkActivate(array $ids): int
    {
        return self::whereIn('id', $ids)->update(['active' => true]);
    }

    /**
     * Bulk deactivate forms.
     *
     * @param array<int> $ids
     */
    public static function bulkDeactivate(array $ids): int
    {
        return self::whereIn('id', $ids)->update(['active' => false]);
    }

    /**
     * Bulk delete forms.
     *
     * @param array<int> $ids
     */
    public static function bulkDelete(array $ids): int
    {
        return self::whereIn('id', $ids)->delete();
    }

    /**
     * Clean up expired forms.
     */
    public static function cleanupExpired(int $daysAfterExpiration = 30): int
    {
        return self::whereNotNull('expires_at')
            ->where('expires_at', '<', now()->subDays($daysAfterExpiration))
            ->delete();
    }
}