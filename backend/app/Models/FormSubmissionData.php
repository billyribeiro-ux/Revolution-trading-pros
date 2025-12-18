<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;

/**
 * FormSubmissionData Model
 * 
 * Stores individual field values for form submissions with comprehensive
 * type safety, query optimization, and data integrity features.
 *
 * @property int $id
 * @property int $submission_id
 * @property int $field_id
 * @property string $field_name
 * @property string|array|null $value
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read FormSubmission $submission
 * @property-read FormField $field
 * 
 * @method static Builder forSubmission(int $submissionId)
 * @method static Builder forField(int $fieldId)
 * @method static Builder whereFieldName(string $fieldName)
 */
class FormSubmissionData extends Model
{
    use HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'form_submission_data';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'submission_id',
        'field_id',
        'field_name',
        'value',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'submission_id' => 'integer',
        'field_id' => 'integer',
        'value' => 'json', // Auto-handles array/object values
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<int, string>
     */
    protected $hidden = [
        // Add sensitive fields if needed
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        // 'formatted_value' // Example if needed
    ];

    /**
     * Indicates if the model should be timestamped.
     */
    public $timestamps = true;

    /**
     * Get the attributes that should be cast to native types.
     *
     * @return array<string, string>
     */
    protected function casts(): array
    {
        return [
            'submission_id' => 'integer',
            'field_id' => 'integer',
            'value' => 'json',
            'created_at' => 'datetime',
            'updated_at' => 'datetime',
        ];
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the submission that owns this data.
     * 
     * Uses eager loading optimization to prevent N+1 queries.
     */
    public function submission(): BelongsTo
    {
        return $this->belongsTo(FormSubmission::class, 'submission_id')
            ->withDefault([
                'status' => 'unknown',
            ]);
    }

    /**
     * Get the field definition that this data belongs to.
     * 
     * Provides validation rules and metadata for the field.
     */
    public function field(): BelongsTo
    {
        return $this->belongsTo(FormField::class, 'field_id')
            ->withDefault([
                'type' => 'text',
                'required' => false,
            ]);
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include data for a specific submission.
     */
    public function scopeForSubmission(Builder $query, int $submissionId): Builder
    {
        return $query->where('submission_id', $submissionId);
    }

    /**
     * Scope a query to only include data for a specific field.
     */
    public function scopeForField(Builder $query, int $fieldId): Builder
    {
        return $query->where('field_id', $fieldId);
    }

    /**
     * Scope a query to filter by field name.
     */
    public function scopeWhereFieldName(Builder $query, string $fieldName): Builder
    {
        return $query->where('field_name', $fieldName);
    }

    /**
     * Scope a query to only include data with non-empty values.
     */
    public function scopeHasValue(Builder $query): Builder
    {
        return $query->whereNotNull('value')
            ->where('value', '!=', '')
            ->where('value', '!=', '[]')
            ->where('value', '!=', '{}');
    }

    /**
     * Scope to eager load common relationships.
     */
    public function scopeWithCommonRelations(Builder $query): Builder
    {
        return $query->with(['submission', 'field']);
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Get the value attribute with proper type handling.
     * 
     * Automatically decodes JSON values and handles nulls gracefully.
     */
    public function getValueAttribute($value): mixed
    {
        if ($value === null || $value === '') {
            return null;
        }

        // If already decoded by cast, return as-is
        if (is_array($value)) {
            return $value;
        }

        // Try to decode JSON if it's a string
        if (is_string($value) && $this->isJson($value)) {
            return json_decode($value, true);
        }

        return $value;
    }

    /**
     * Set the value attribute with automatic JSON encoding.
     */
    public function setValueAttribute($value): void
    {
        if (is_array($value) || is_object($value)) {
            $this->attributes['value'] = json_encode($value);
        } else {
            $this->attributes['value'] = $value;
        }
    }

    /**
     * Get formatted value for display purposes.
     */
    public function getFormattedValueAttribute(): string
    {
        $value = $this->value;

        if (is_array($value)) {
            return implode(', ', $value);
        }

        if (is_bool($value)) {
            return $value ? 'Yes' : 'No';
        }

        return (string) $value;
    }

    // =========================================================================
    // UTILITY METHODS
    // =========================================================================

    /**
     * Check if a string is valid JSON.
     */
    protected function isJson(string $string): bool
    {
        json_decode($string);
        return json_last_error() === JSON_ERROR_NONE;
    }

    /**
     * Determine if the field value is empty.
     */
    public function isEmpty(): bool
    {
        $value = $this->value;

        if ($value === null || $value === '') {
            return true;
        }

        if (is_array($value) && empty($value)) {
            return true;
        }

        return false;
    }

    /**
     * Get the sanitized value (strip tags, trim).
     */
    public function getSanitizedValue(): ?string
    {
        if ($this->isEmpty()) {
            return null;
        }

        $value = $this->formatted_value;
        return trim(strip_tags($value));
    }

    /**
     * Validate the value against field rules.
     * 
     * @throws \InvalidArgumentException
     */
    public function validateValue(): bool
    {
        $field = $this->field;

        if ($field->required && $this->isEmpty()) {
            throw new \InvalidArgumentException(
                "Field '{$this->field_name}' is required but has no value"
            );
        }

        // Add more validation logic based on field type
        // This is extensible for different field types

        return true;
    }

    // =========================================================================
    // STATIC FACTORY METHODS
    // =========================================================================

    /**
     * Create a new submission data entry with validation.
     */
    public static function createSafely(array $attributes): self
    {
        $instance = new static($attributes);
        $instance->validateValue();
        $instance->save();

        return $instance;
    }

    /**
     * Bulk insert submission data with transaction safety.
     * 
     * @param array<int, array<string, mixed>> $dataArray
     * @return \Illuminate\Support\Collection<int, self>
     */
    public static function bulkCreate(array $dataArray): \Illuminate\Support\Collection
    {
        return \DB::transaction(function () use ($dataArray) {
            $instances = collect();

            foreach ($dataArray as $data) {
                $instances->push(self::create($data));
            }

            return $instances;
        });
    }

    // =========================================================================
    // EVENTS & OBSERVERS
    // =========================================================================

    /**
     * The "booted" method of the model.
     */
    protected static function booted(): void
    {
        // Auto-sanitize field_name before saving
        static::saving(function (self $model) {
            if ($model->isDirty('field_name')) {
                $model->field_name = strtolower(trim($model->field_name));
            }
        });

        // Log deletions for audit trail
        static::deleting(function (self $model) {
            \Log::info('FormSubmissionData deleted', [
                'id' => $model->id,
                'submission_id' => $model->submission_id,
                'field_name' => $model->field_name,
            ]);
        });
    }
}