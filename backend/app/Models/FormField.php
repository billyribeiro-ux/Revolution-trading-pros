<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Carbon;
use Illuminate\Support\Str;

/**
 * FormField Model - Enterprise-grade form field management
 * 
 * Manages form field definitions with comprehensive validation, conditional logic,
 * and dynamic rendering capabilities. Supports 20+ field types with extensibility.
 *
 * @property int $id
 * @property int $form_id
 * @property string $field_type Field type (text, email, select, etc.)
 * @property string $label Display label
 * @property string $name Unique field identifier
 * @property string|null $placeholder Placeholder text
 * @property string|null $help_text Help/description text
 * @property string|null $default_value Default field value
 * @property array|null $options Field options for select/radio/checkbox
 * @property array|null $validation Custom validation rules
 * @property array|null $conditional_logic Conditional display rules
 * @property array|null $attributes HTML attributes
 * @property bool $required Required field flag
 * @property int $order Display order
 * @property int $width Width percentage (1-100)
 * @property bool $active Active status
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * 
 * @property-read Form $form
 * @property-read \Illuminate\Support\Collection<int, FormSubmissionData> $submissions
 * @property-read string $field_type_label
 * @property-read string $validation_rules_string
 * 
 * @method static Builder active()
 * @method static Builder required()
 * @method static Builder optional()
 * @method static Builder forForm(int $formId)
 * @method static Builder ofType(string $type)
 * @method static Builder ordered()
 * @method static Builder searchable()
 */
class FormField extends Model
{
    use HasFactory;

    /**
     * The table associated with the model.
     */
    protected $table = 'form_fields';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [
        'form_id',
        'field_type',
        'label',
        'name',
        'placeholder',
        'help_text',
        'default_value',
        'options',
        'validation',
        'conditional_logic',
        'attributes',
        'required',
        'order',
        'width',
        'active',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'form_id' => 'integer',
        'options' => 'array',
        'validation' => 'array',
        'conditional_logic' => 'array',
        'attributes' => 'array',
        'required' => 'boolean',
        'active' => 'boolean',
        'order' => 'integer',
        'width' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * The model's default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'required' => false,
        'active' => true,
        'order' => 0,
        'width' => 100,
    ];

    /**
     * The accessors to append to the model's array form.
     *
     * @var array<int, string>
     */
    protected $appends = [
        'field_type_label',
        'validation_rules_string',
    ];

    // =========================================================================
    // CONSTANTS - FIELD TYPES
    // =========================================================================

    // Input Types
    public const TYPE_TEXT = 'text';
    public const TYPE_EMAIL = 'email';
    public const TYPE_PHONE = 'phone';
    public const TYPE_NUMBER = 'number';
    public const TYPE_TEXTAREA = 'textarea';
    public const TYPE_PASSWORD = 'password';
    public const TYPE_URL = 'url';
    public const TYPE_COLOR = 'color';
    public const TYPE_RANGE = 'range';
    public const TYPE_HIDDEN = 'hidden';

    // Selection Types
    public const TYPE_SELECT = 'select';
    public const TYPE_RADIO = 'radio';
    public const TYPE_CHECKBOX = 'checkbox';
    public const TYPE_MULTISELECT = 'multiselect';

    // Date/Time Types
    public const TYPE_DATE = 'date';
    public const TYPE_TIME = 'time';
    public const TYPE_DATETIME = 'datetime';

    // File Types
    public const TYPE_FILE = 'file';
    public const TYPE_IMAGE = 'image';

    // Advanced Types
    public const TYPE_RATING = 'rating';
    public const TYPE_SIGNATURE = 'signature';
    public const TYPE_WYSIWYG = 'wysiwyg';
    public const TYPE_CODE = 'code';

    // Layout Types
    public const TYPE_HEADING = 'heading';
    public const TYPE_PARAGRAPH = 'paragraph';
    public const TYPE_DIVIDER = 'divider';
    public const TYPE_SPACER = 'spacer';

    /**
     * Available field types.
     *
     * @var array<int, string>
     */
    public const FIELD_TYPES = [
        // Input Types
        self::TYPE_TEXT,
        self::TYPE_EMAIL,
        self::TYPE_PHONE,
        self::TYPE_NUMBER,
        self::TYPE_TEXTAREA,
        self::TYPE_PASSWORD,
        self::TYPE_URL,
        self::TYPE_COLOR,
        self::TYPE_RANGE,
        self::TYPE_HIDDEN,
        
        // Selection Types
        self::TYPE_SELECT,
        self::TYPE_RADIO,
        self::TYPE_CHECKBOX,
        self::TYPE_MULTISELECT,
        
        // Date/Time Types
        self::TYPE_DATE,
        self::TYPE_TIME,
        self::TYPE_DATETIME,
        
        // File Types
        self::TYPE_FILE,
        self::TYPE_IMAGE,
        
        // Advanced Types
        self::TYPE_RATING,
        self::TYPE_SIGNATURE,
        self::TYPE_WYSIWYG,
        self::TYPE_CODE,
        
        // Layout Types
        self::TYPE_HEADING,
        self::TYPE_PARAGRAPH,
        self::TYPE_DIVIDER,
        self::TYPE_SPACER,
    ];

    /**
     * Field type labels for display.
     *
     * @var array<string, string>
     */
    public const FIELD_TYPE_LABELS = [
        self::TYPE_TEXT => 'Text Input',
        self::TYPE_EMAIL => 'Email Address',
        self::TYPE_PHONE => 'Phone Number',
        self::TYPE_NUMBER => 'Number',
        self::TYPE_TEXTAREA => 'Text Area',
        self::TYPE_PASSWORD => 'Password',
        self::TYPE_URL => 'Website URL',
        self::TYPE_COLOR => 'Color Picker',
        self::TYPE_RANGE => 'Range Slider',
        self::TYPE_HIDDEN => 'Hidden Field',
        self::TYPE_SELECT => 'Dropdown Select',
        self::TYPE_RADIO => 'Radio Buttons',
        self::TYPE_CHECKBOX => 'Checkboxes',
        self::TYPE_MULTISELECT => 'Multi-Select',
        self::TYPE_DATE => 'Date Picker',
        self::TYPE_TIME => 'Time Picker',
        self::TYPE_DATETIME => 'Date & Time',
        self::TYPE_FILE => 'File Upload',
        self::TYPE_IMAGE => 'Image Upload',
        self::TYPE_RATING => 'Star Rating',
        self::TYPE_SIGNATURE => 'Signature Pad',
        self::TYPE_WYSIWYG => 'Rich Text Editor',
        self::TYPE_CODE => 'Code Editor',
        self::TYPE_HEADING => 'Heading',
        self::TYPE_PARAGRAPH => 'Paragraph Text',
        self::TYPE_DIVIDER => 'Divider Line',
        self::TYPE_SPACER => 'Spacer',
    ];

    /**
     * Field types that require options.
     *
     * @var array<int, string>
     */
    public const TYPES_REQUIRING_OPTIONS = [
        self::TYPE_SELECT,
        self::TYPE_RADIO,
        self::TYPE_CHECKBOX,
        self::TYPE_MULTISELECT,
    ];

    /**
     * Field types that are layout elements (non-input).
     *
     * @var array<int, string>
     */
    public const LAYOUT_TYPES = [
        self::TYPE_HEADING,
        self::TYPE_PARAGRAPH,
        self::TYPE_DIVIDER,
        self::TYPE_SPACER,
    ];

    /**
     * Field types that accept file uploads.
     *
     * @var array<int, string>
     */
    public const FILE_TYPES = [
        self::TYPE_FILE,
        self::TYPE_IMAGE,
        self::TYPE_SIGNATURE,
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

        // Auto-generate field name from label if not provided
        static::creating(function (FormField $field) {
            if (empty($field->name) && !empty($field->label)) {
                $field->name = Str::slug($field->label, '_');
            }

            // Validate field type
            if (!self::isValidFieldType($field->field_type)) {
                throw new \InvalidArgumentException(
                    "Invalid field type: {$field->field_type}"
                );
            }

            // Validate options exist for selection types
            if ($field->requiresOptions() && empty($field->options)) {
                throw new \InvalidArgumentException(
                    "Field type '{$field->field_type}' requires options"
                );
            }
        });

        // Validate on update as well
        static::updating(function (FormField $field) {
            if ($field->isDirty('field_type')) {
                if (!self::isValidFieldType($field->field_type)) {
                    throw new \InvalidArgumentException(
                        "Invalid field type: {$field->field_type}"
                    );
                }
            }

            if ($field->isDirty(['field_type', 'options'])) {
                if ($field->requiresOptions() && empty($field->options)) {
                    throw new \InvalidArgumentException(
                        "Field type '{$field->field_type}' requires options"
                    );
                }
            }
        });

        // Log deletions
        static::deleting(function (FormField $field) {
            \Log::info('FormField deleted', [
                'id' => $field->id,
                'form_id' => $field->form_id,
                'name' => $field->name,
                'type' => $field->field_type,
            ]);
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the form that owns the field.
     */
    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class)
            ->withDefault([
                'name' => 'Unknown Form',
                'active' => false,
            ]);
    }

    /**
     * Get all submission data entries for this field.
     */
    public function submissions(): HasMany
    {
        return $this->hasMany(FormSubmissionData::class, 'field_id');
    }

    // =========================================================================
    // QUERY SCOPES
    // =========================================================================

    /**
     * Scope a query to only include active fields.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('active', true);
    }

    /**
     * Scope a query to only include required fields.
     */
    public function scopeRequired(Builder $query): Builder
    {
        return $query->where('required', true);
    }

    /**
     * Scope a query to only include optional fields.
     */
    public function scopeOptional(Builder $query): Builder
    {
        return $query->where('required', false);
    }

    /**
     * Scope a query to fields for a specific form.
     */
    public function scopeForForm(Builder $query, int $formId): Builder
    {
        return $query->where('form_id', $formId);
    }

    /**
     * Scope a query to fields of a specific type.
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('field_type', $type);
    }

    /**
     * Scope a query to order fields by their order column.
     */
    public function scopeOrdered(Builder $query): Builder
    {
        return $query->orderBy('order', 'asc')->orderBy('id', 'asc');
    }

    /**
     * Scope to exclude layout-only fields.
     */
    public function scopeInputFields(Builder $query): Builder
    {
        return $query->whereNotIn('field_type', self::LAYOUT_TYPES);
    }

    /**
     * Scope to only layout fields.
     */
    public function scopeLayoutFields(Builder $query): Builder
    {
        return $query->whereIn('field_type', self::LAYOUT_TYPES);
    }

    /**
     * Scope to searchable fields (text-based inputs).
     */
    public function scopeSearchable(Builder $query): Builder
    {
        return $query->whereIn('field_type', [
            self::TYPE_TEXT,
            self::TYPE_EMAIL,
            self::TYPE_PHONE,
            self::TYPE_TEXTAREA,
            self::TYPE_URL,
        ]);
    }

    // =========================================================================
    // ACCESSORS & MUTATORS
    // =========================================================================

    /**
     * Get human-readable field type label.
     */
    public function getFieldTypeLabelAttribute(): string
    {
        return self::FIELD_TYPE_LABELS[$this->field_type] 
            ?? ucfirst(str_replace('_', ' ', $this->field_type));
    }

    /**
     * Get validation rules as a readable string.
     */
    public function getValidationRulesStringAttribute(): string
    {
        $rules = $this->getValidationRules();
        return empty($rules) ? 'None' : implode(', ', $rules);
    }

    /**
     * Auto-generate field name from label if empty.
     */
    public function setLabelAttribute(?string $value): void
    {
        $this->attributes['label'] = $value;

        // Auto-set name if it's empty and label is provided
        if (empty($this->attributes['name']) && !empty($value)) {
            $this->attributes['name'] = Str::slug($value, '_');
        }
    }

    /**
     * Ensure width is within valid range (1-100).
     */
    public function setWidthAttribute(?int $value): void
    {
        $this->attributes['width'] = max(1, min(100, $value ?? 100));
    }

    // =========================================================================
    // VALIDATION METHODS
    // =========================================================================

    /**
     * Check if field type is valid.
     */
    public static function isValidFieldType(string $type): bool
    {
        return in_array($type, self::FIELD_TYPES, true);
    }

    /**
     * Get comprehensive validation rules array.
     */
    public function getValidationRules(): array
    {
        $rules = [];

        // Add required rule
        if ($this->required && !$this->isLayoutType()) {
            $rules[] = 'required';
        } else {
            $rules[] = 'nullable';
        }

        // Add custom validation rules
        if (!empty($this->validation) && is_array($this->validation)) {
            $rules = array_merge($rules, $this->validation);
        }

        // Add type-specific validations
        $rules = array_merge($rules, $this->getTypeSpecificRules());

        return array_values(array_unique($rules));
    }

    /**
     * Get type-specific validation rules.
     *
     * @return array<int, string>
     */
    protected function getTypeSpecificRules(): array
    {
        return match ($this->field_type) {
            self::TYPE_EMAIL => ['email:rfc,dns'],
            self::TYPE_URL => ['url', 'active_url'],
            self::TYPE_NUMBER, self::TYPE_RANGE => ['numeric'],
            self::TYPE_PHONE => ['regex:/^[\d\s\-\+\(\)]+$/'],
            self::TYPE_DATE => ['date'],
            self::TYPE_TIME => ['date_format:H:i'],
            self::TYPE_DATETIME => ['date'],
            self::TYPE_FILE => ['file', 'max:10240'], // 10MB max
            self::TYPE_IMAGE => ['image', 'mimes:jpeg,png,jpg,gif,webp', 'max:5120'], // 5MB max
            self::TYPE_COLOR => ['regex:/^#[0-9A-Fa-f]{6}$/'],
            default => [],
        };
    }

    /**
     * Get validation rules as Laravel validation array format.
     */
    public function getValidationArray(): array
    {
        return [$this->name => $this->getValidationRules()];
    }

    /**
     * Validate a value against this field's rules.
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function validateValue(mixed $value): bool
    {
        $validator = \Validator::make(
            [$this->name => $value],
            $this->getValidationArray(),
            [],
            [$this->name => $this->label]
        );

        if ($validator->fails()) {
            throw new \Illuminate\Validation\ValidationException($validator);
        }

        return true;
    }

    // =========================================================================
    // TYPE CHECKING METHODS
    // =========================================================================

    /**
     * Check if field requires options (select, radio, checkbox).
     */
    public function requiresOptions(): bool
    {
        return in_array($this->field_type, self::TYPES_REQUIRING_OPTIONS, true);
    }

    /**
     * Check if field is a layout element.
     */
    public function isLayoutType(): bool
    {
        return in_array($this->field_type, self::LAYOUT_TYPES, true);
    }

    /**
     * Check if field accepts file uploads.
     */
    public function isFileType(): bool
    {
        return in_array($this->field_type, self::FILE_TYPES, true);
    }

    /**
     * Check if field is a text input type.
     */
    public function isTextInputType(): bool
    {
        return in_array($this->field_type, [
            self::TYPE_TEXT,
            self::TYPE_EMAIL,
            self::TYPE_PHONE,
            self::TYPE_NUMBER,
            self::TYPE_PASSWORD,
            self::TYPE_URL,
        ], true);
    }

    /**
     * Check if field is a selection type.
     */
    public function isSelectionType(): bool
    {
        return in_array($this->field_type, [
            self::TYPE_SELECT,
            self::TYPE_RADIO,
            self::TYPE_CHECKBOX,
            self::TYPE_MULTISELECT,
        ], true);
    }

    /**
     * Check if field allows multiple values.
     */
    public function allowsMultipleValues(): bool
    {
        return in_array($this->field_type, [
            self::TYPE_CHECKBOX,
            self::TYPE_MULTISELECT,
        ], true);
    }

    // =========================================================================
    // OPTIONS MANAGEMENT
    // =========================================================================

    /**
     * Get formatted options array.
     *
     * @return array<string, string>
     */
    public function getFormattedOptions(): array
    {
        if (empty($this->options)) {
            return [];
        }

        // If options are already key-value pairs
        if ($this->isAssociativeArray($this->options)) {
            return $this->options;
        }

        // If options are a simple list, create key-value pairs
        return array_combine($this->options, $this->options);
    }

    /**
     * Check if array is associative.
     */
    protected function isAssociativeArray(array $array): bool
    {
        if (empty($array)) {
            return false;
        }
        return array_keys($array) !== range(0, count($array) - 1);
    }

    /**
     * Add an option to the field.
     */
    public function addOption(string $value, ?string $label = null): bool
    {
        $options = $this->options ?? [];
        $options[$value] = $label ?? $value;
        $this->options = $options;
        return $this->save();
    }

    /**
     * Remove an option from the field.
     */
    public function removeOption(string $value): bool
    {
        $options = $this->options ?? [];
        unset($options[$value]);
        $this->options = $options;
        return $this->save();
    }

    /**
     * Check if option exists.
     */
    public function hasOption(string $value): bool
    {
        $options = $this->options ?? [];
        return isset($options[$value]) || in_array($value, $options, true);
    }

    // =========================================================================
    // CONDITIONAL LOGIC
    // =========================================================================

    /**
     * Check if field has conditional logic.
     */
    public function hasConditionalLogic(): bool
    {
        return !empty($this->conditional_logic);
    }

    /**
     * Evaluate conditional logic against provided values.
     *
     * @param array<string, mixed> $formValues
     */
    public function evaluateConditionalLogic(array $formValues): bool
    {
        if (!$this->hasConditionalLogic()) {
            return true; // Show by default if no conditions
        }

        // Implement conditional logic evaluation
        // Format: ['field' => 'field_name', 'operator' => '==', 'value' => 'expected']
        foreach ($this->conditional_logic as $condition) {
            $fieldName = $condition['field'] ?? null;
            $operator = $condition['operator'] ?? '==';
            $expectedValue = $condition['value'] ?? null;

            if (!isset($formValues[$fieldName])) {
                return false;
            }

            $actualValue = $formValues[$fieldName];

            $result = match ($operator) {
                '==' => $actualValue == $expectedValue,
                '!=' => $actualValue != $expectedValue,
                '>' => $actualValue > $expectedValue,
                '<' => $actualValue < $expectedValue,
                '>=' => $actualValue >= $expectedValue,
                '<=' => $actualValue <= $expectedValue,
                'contains' => str_contains((string) $actualValue, (string) $expectedValue),
                'not_contains' => !str_contains((string) $actualValue, (string) $expectedValue),
                'empty' => empty($actualValue),
                'not_empty' => !empty($actualValue),
                default => false,
            };

            if (!$result) {
                return false;
            }
        }

        return true;
    }

    // =========================================================================
    // RENDERING & EXPORT
    // =========================================================================

    /**
     * Get field as form configuration array.
     */
    public function toFormConfig(): array
    {
        return [
            'id' => $this->id,
            'type' => $this->field_type,
            'name' => $this->name,
            'label' => $this->label,
            'placeholder' => $this->placeholder,
            'helpText' => $this->help_text,
            'defaultValue' => $this->default_value,
            'options' => $this->getFormattedOptions(),
            'required' => $this->required,
            'validation' => $this->getValidationRules(),
            'attributes' => $this->attributes ?? [],
            'width' => $this->width,
            'order' => $this->order,
            'conditionalLogic' => $this->conditional_logic,
        ];
    }

    /**
     * Get HTML attributes as string.
     */
    public function getAttributesString(): string
    {
        if (empty($this->attributes)) {
            return '';
        }

        $attrs = [];
        foreach ($this->attributes as $key => $value) {
            if (is_bool($value)) {
                if ($value) {
                    $attrs[] = $key;
                }
            } else {
                $attrs[] = sprintf('%s="%s"', $key, htmlspecialchars((string) $value));
            }
        }

        return implode(' ', $attrs);
    }

    // =========================================================================
    // STATIC FACTORY METHODS
    // =========================================================================

    /**
     * Create a text field.
     */
    public static function createTextField(int $formId, string $label, bool $required = false): self
    {
        return self::create([
            'form_id' => $formId,
            'field_type' => self::TYPE_TEXT,
            'label' => $label,
            'required' => $required,
        ]);
    }

    /**
     * Create an email field.
     */
    public static function createEmailField(int $formId, string $label = 'Email', bool $required = true): self
    {
        return self::create([
            'form_id' => $formId,
            'field_type' => self::TYPE_EMAIL,
            'label' => $label,
            'required' => $required,
        ]);
    }

    /**
     * Create a select field with options.
     */
    public static function createSelectField(int $formId, string $label, array $options, bool $required = false): self
    {
        return self::create([
            'form_id' => $formId,
            'field_type' => self::TYPE_SELECT,
            'label' => $label,
            'options' => $options,
            'required' => $required,
        ]);
    }

    /**
     * Bulk reorder fields.
     *
     * @param array<int, int> $orderMap [field_id => new_order]
     */
    public static function bulkReorder(array $orderMap): int
    {
        $updated = 0;
        foreach ($orderMap as $fieldId => $newOrder) {
            $updated += self::where('id', $fieldId)->update(['order' => $newOrder]);
        }
        return $updated;
    }

    /**
     * Duplicate a field within the same or different form.
     */
    public function duplicate(?int $targetFormId = null): self
    {
        $duplicate = $this->replicate();
        $duplicate->form_id = $targetFormId ?? $this->form_id;
        $duplicate->name = $this->name . '_copy_' . time();
        $duplicate->order = self::where('form_id', $duplicate->form_id)->max('order') + 1;
        $duplicate->save();

        return $duplicate;
    }
}