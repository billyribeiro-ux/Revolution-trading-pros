<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use InvalidArgumentException;

/**
 * Membership Feature Model
 * 
 * Manages individual features/capabilities available in membership plans
 * with detailed specifications, limits, and comparison data.
 *
 * @property int $id
 * @property int $plan_id Associated membership plan
 * @property string $feature_code Unique feature identifier/code
 * @property string $feature_name Display name of feature
 * @property string|null $description Feature description
 * @property string|null $category Feature category (access, storage, support, etc.)
 * @property string $type Feature type (boolean, numeric, text, unlimited)
 * @property mixed $value Feature value (depends on type)
 * @property int|null $limit Numeric limit (for numeric features)
 * @property bool $is_unlimited Whether feature is unlimited
 * @property bool $is_included Whether feature is included in plan
 * @property bool $is_highlighted Whether to highlight in comparison
 * @property int $sort_order Display sort order
 * @property string|null $icon Icon identifier
 * @property string|null $tooltip Tooltip/help text
 * @property array $metadata Additional feature data
 * @property array $restrictions Feature-specific restrictions
 * @property string|null $unit Unit of measurement (GB, users, API calls, etc.)
 * @property float|null $overage_price Price per unit over limit
 * @property bool $allows_overage Whether overage is allowed
 * @property string|null $comparison_text Text for plan comparison tables
 * @property string|null $help_url Help documentation URL
 * @property bool $is_core Core feature (cannot be removed)
 * @property bool $is_addon Available as addon
 * @property float|null $addon_price Price if purchased as addon
 * @property string|null $addon_billing_cycle Addon billing cycle
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\MembershipPlan $plan Parent membership plan
 */
class MembershipFeature extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Feature types
     */
    public const TYPE_BOOLEAN = 'boolean';
    public const TYPE_NUMERIC = 'numeric';
    public const TYPE_TEXT = 'text';
    public const TYPE_UNLIMITED = 'unlimited';
    public const TYPE_PERCENTAGE = 'percentage';
    public const TYPE_ENUM = 'enum';

    /**
     * Valid feature types
     */
    public const VALID_TYPES = [
        self::TYPE_BOOLEAN,
        self::TYPE_NUMERIC,
        self::TYPE_TEXT,
        self::TYPE_UNLIMITED,
        self::TYPE_PERCENTAGE,
        self::TYPE_ENUM,
    ];

    /**
     * Feature categories
     */
    public const CATEGORY_ACCESS = 'access';
    public const CATEGORY_STORAGE = 'storage';
    public const CATEGORY_USERS = 'users';
    public const CATEGORY_SUPPORT = 'support';
    public const CATEGORY_ANALYTICS = 'analytics';
    public const CATEGORY_INTEGRATION = 'integration';
    public const CATEGORY_CUSTOMIZATION = 'customization';
    public const CATEGORY_SECURITY = 'security';
    public const CATEGORY_AUTOMATION = 'automation';
    public const CATEGORY_API = 'api';
    public const CATEGORY_COLLABORATION = 'collaboration';
    public const CATEGORY_REPORTING = 'reporting';

    /**
     * Common feature codes
     */
    public const FEATURE_UNLIMITED_PROJECTS = 'unlimited_projects';
    public const FEATURE_TEAM_MEMBERS = 'team_members';
    public const FEATURE_STORAGE_LIMIT = 'storage_limit';
    public const FEATURE_API_CALLS = 'api_calls';
    public const FEATURE_PRIORITY_SUPPORT = 'priority_support';
    public const FEATURE_CUSTOM_DOMAIN = 'custom_domain';
    public const FEATURE_WHITE_LABEL = 'white_label';
    public const FEATURE_ADVANCED_ANALYTICS = 'advanced_analytics';
    public const FEATURE_SSO = 'sso';
    public const FEATURE_API_ACCESS = 'api_access';

    protected $fillable = [
        'plan_id',
        'feature_code',
        'feature_name',
        'description',
        'category',
        'type',
        'value',
        'limit',
        'is_unlimited',
        'is_included',
        'is_highlighted',
        'sort_order',
        'icon',
        'tooltip',
        'metadata',
        'restrictions',
        'unit',
        'overage_price',
        'allows_overage',
        'comparison_text',
        'help_url',
        'is_core',
        'is_addon',
        'addon_price',
        'addon_billing_cycle',
    ];

    protected $casts = [
        'plan_id' => 'integer',
        'limit' => 'integer',
        'sort_order' => 'integer',
        'overage_price' => 'decimal:2',
        'addon_price' => 'decimal:2',
        'is_unlimited' => 'boolean',
        'is_included' => 'boolean',
        'is_highlighted' => 'boolean',
        'allows_overage' => 'boolean',
        'is_core' => 'boolean',
        'is_addon' => 'boolean',
        'metadata' => 'array',
        'restrictions' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'type' => self::TYPE_BOOLEAN,
        'is_unlimited' => false,
        'is_included' => true,
        'is_highlighted' => false,
        'sort_order' => 0,
        'allows_overage' => false,
        'is_core' => false,
        'is_addon' => false,
        'metadata' => '[]',
        'restrictions' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->validateType();
            $model->normalizeValue();
        });

        static::updating(function (self $model): void {
            $model->validateType();
            
            if ($model->isDirty(['value', 'limit', 'is_unlimited'])) {
                $model->normalizeValue();
            }
        });
    }

    /**
     * Get parent membership plan
     */
    public function plan(): BelongsTo
    {
        return $this->belongsTo(MembershipPlan::class, 'plan_id');
    }

    /**
     * Validate feature type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid feature type: %s', $this->type)
            );
        }
    }

    /**
     * Normalize value based on type
     */
    protected function normalizeValue(): void
    {
        if ($this->is_unlimited) {
            $this->type = self::TYPE_UNLIMITED;
            $this->limit = null;
            $this->value = null;
            return;
        }

        switch ($this->type) {
            case self::TYPE_BOOLEAN:
                $this->value = (bool) $this->value;
                $this->limit = null;
                break;

            case self::TYPE_NUMERIC:
                if ($this->limit !== null) {
                    $this->value = $this->limit;
                }
                break;

            case self::TYPE_UNLIMITED:
                $this->is_unlimited = true;
                $this->limit = null;
                $this->value = null;
                break;

            case self::TYPE_PERCENTAGE:
                if (is_numeric($this->value)) {
                    $this->value = max(0, min(100, (float) $this->value));
                }
                break;
        }
    }

    /**
     * Check if feature is available/enabled
     */
    public function isAvailable(): bool
    {
        if (!$this->is_included) {
            return false;
        }

        if ($this->type === self::TYPE_BOOLEAN) {
            return (bool) $this->value;
        }

        if ($this->type === self::TYPE_UNLIMITED) {
            return true;
        }

        if ($this->type === self::TYPE_NUMERIC) {
            return $this->limit > 0 || $this->is_unlimited;
        }

        return true;
    }

    /**
     * Check if within usage limit
     */
    public function isWithinLimit(int $currentUsage): bool
    {
        if ($this->is_unlimited) {
            return true;
        }

        if ($this->type !== self::TYPE_NUMERIC || $this->limit === null) {
            return true;
        }

        return $currentUsage < $this->limit;
    }

    /**
     * Check if overage is allowed and within limits
     */
    public function canAllowOverage(int $currentUsage): bool
    {
        if ($this->is_unlimited) {
            return false; // No overage needed if unlimited
        }

        if (!$this->allows_overage || $this->overage_price === null) {
            return false;
        }

        return $currentUsage >= ($this->limit ?? 0);
    }

    /**
     * Calculate overage cost
     */
    public function calculateOverageCost(int $currentUsage): float
    {
        if (!$this->canAllowOverage($currentUsage)) {
            return 0.0;
        }

        $overageUnits = $currentUsage - ($this->limit ?? 0);
        return round($overageUnits * $this->overage_price, 2);
    }

    /**
     * Get remaining units
     */
    public function getRemainingUnits(int $currentUsage): ?int
    {
        if ($this->is_unlimited) {
            return null; // Unlimited
        }

        if ($this->type !== self::TYPE_NUMERIC || $this->limit === null) {
            return null;
        }

        return max(0, $this->limit - $currentUsage);
    }

    /**
     * Get usage percentage
     */
    public function getUsagePercentage(int $currentUsage): ?float
    {
        if ($this->is_unlimited || $this->type !== self::TYPE_NUMERIC || $this->limit === null) {
            return null;
        }

        if ($this->limit === 0) {
            return 0.0;
        }

        return round(($currentUsage / $this->limit) * 100, 2);
    }

    /**
     * Get display value for UI
     */
    public function getDisplayValueAttribute(): string
    {
        if ($this->is_unlimited) {
            return 'Unlimited';
        }

        if (!$this->is_included) {
            return 'Not Included';
        }

        return match($this->type) {
            self::TYPE_BOOLEAN => $this->value ? 'Yes' : 'No',
            self::TYPE_NUMERIC => $this->formatNumericValue(),
            self::TYPE_TEXT => (string) $this->value,
            self::TYPE_PERCENTAGE => $this->value . '%',
            self::TYPE_ENUM => ucfirst((string) $this->value),
            default => (string) $this->value,
        };
    }

    /**
     * Format numeric value with unit
     */
    protected function formatNumericValue(): string
    {
        if ($this->limit === null) {
            return '0';
        }

        $formatted = number_format($this->limit);

        if ($this->unit) {
            $formatted .= ' ' . $this->unit;
        }

        return $formatted;
    }

    /**
     * Get comparison text or generate from value
     */
    public function getComparisonDisplayAttribute(): string
    {
        if ($this->comparison_text) {
            return $this->comparison_text;
        }

        return $this->display_value;
    }

    /**
     * Get icon with fallback
     */
    public function getIconAttribute($value): string
    {
        if ($value) {
            return $value;
        }

        // Default icons by category
        return match($this->category) {
            self::CATEGORY_ACCESS => 'key',
            self::CATEGORY_STORAGE => 'database',
            self::CATEGORY_USERS => 'users',
            self::CATEGORY_SUPPORT => 'life-buoy',
            self::CATEGORY_ANALYTICS => 'bar-chart',
            self::CATEGORY_INTEGRATION => 'puzzle',
            self::CATEGORY_CUSTOMIZATION => 'sliders',
            self::CATEGORY_SECURITY => 'shield',
            self::CATEGORY_AUTOMATION => 'zap',
            self::CATEGORY_API => 'code',
            self::CATEGORY_COLLABORATION => 'users',
            self::CATEGORY_REPORTING => 'file-text',
            default => 'check',
        };
    }

    /**
     * Get category label
     */
    public function getCategoryLabelAttribute(): string
    {
        return match($this->category) {
            self::CATEGORY_ACCESS => 'Access & Permissions',
            self::CATEGORY_STORAGE => 'Storage',
            self::CATEGORY_USERS => 'Users & Teams',
            self::CATEGORY_SUPPORT => 'Support',
            self::CATEGORY_ANALYTICS => 'Analytics',
            self::CATEGORY_INTEGRATION => 'Integrations',
            self::CATEGORY_CUSTOMIZATION => 'Customization',
            self::CATEGORY_SECURITY => 'Security',
            self::CATEGORY_AUTOMATION => 'Automation',
            self::CATEGORY_API => 'API Access',
            self::CATEGORY_COLLABORATION => 'Collaboration',
            self::CATEGORY_REPORTING => 'Reporting',
            default => ucfirst($this->category ?? 'General'),
        };
    }

    /**
     * Get metadata value
     */
    public function getMetadataValue(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Set metadata value
     */
    public function setMetadataValue(string $key, mixed $value): self
    {
        $metadata = $this->metadata;
        $metadata[$key] = $value;
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Compare with another feature
     */
    public function compareTo(self $other): int
    {
        // Both unlimited
        if ($this->is_unlimited && $other->is_unlimited) {
            return 0;
        }

        // This is unlimited, other is not
        if ($this->is_unlimited) {
            return 1;
        }

        // Other is unlimited, this is not
        if ($other->is_unlimited) {
            return -1;
        }

        // Compare numeric values
        if ($this->type === self::TYPE_NUMERIC && $other->type === self::TYPE_NUMERIC) {
            return ($this->limit ?? 0) <=> ($other->limit ?? 0);
        }

        // Compare boolean values
        if ($this->type === self::TYPE_BOOLEAN && $other->type === self::TYPE_BOOLEAN) {
            return ($this->value ? 1 : 0) <=> ($other->value ? 1 : 0);
        }

        return 0;
    }

    /**
     * Check if better than another feature
     */
    public function isBetterThan(self $other): bool
    {
        return $this->compareTo($other) > 0;
    }

    /**
     * Scope: By feature code
     */
    public function scopeByCode(Builder $query, string $code): Builder
    {
        return $query->where('feature_code', $code);
    }

    /**
     * Scope: By category
     */
    public function scopeInCategory(Builder $query, string $category): Builder
    {
        return $query->where('category', $category);
    }

    /**
     * Scope: Included features
     */
    public function scopeIncluded(Builder $query): Builder
    {
        return $query->where('is_included', true);
    }

    /**
     * Scope: Highlighted features
     */
    public function scopeHighlighted(Builder $query): Builder
    {
        return $query->where('is_highlighted', true);
    }

    /**
     * Scope: Unlimited features
     */
    public function scopeUnlimited(Builder $query): Builder
    {
        return $query->where('is_unlimited', true);
    }

    /**
     * Scope: Core features
     */
    public function scopeCore(Builder $query): Builder
    {
        return $query->where('is_core', true);
    }

    /**
     * Scope: Available as addon
     */
    public function scopeAddons(Builder $query): Builder
    {
        return $query->where('is_addon', true);
    }

    /**
     * Scope: Order by sort order
     */
    public function scopeOrdered(Builder $query): Builder
    {
        return $query->orderBy('sort_order')->orderBy('feature_name');
    }

    /**
     * Static: Get all unique feature codes
     */
    public static function getAllFeatureCodes(): Collection
    {
        return static::distinct('feature_code')
            ->pluck('feature_code')
            ->sort()
            ->values();
    }

    /**
     * Static: Get features by category for all plans
     */
    public static function getByCategory(string $category): Collection
    {
        return static::inCategory($category)
            ->with('plan')
            ->ordered()
            ->get()
            ->groupBy('plan_id');
    }

    /**
     * Static: Build comparison matrix
     */
    public static function buildComparisonMatrix(array $planIds): array
    {
        $features = static::whereIn('plan_id', $planIds)
            ->ordered()
            ->get()
            ->groupBy('feature_code');

        $matrix = [];

        foreach ($features as $code => $planFeatures) {
            $featureData = [
                'code' => $code,
                'name' => $planFeatures->first()->feature_name,
                'category' => $planFeatures->first()->category,
                'category_label' => $planFeatures->first()->category_label,
                'icon' => $planFeatures->first()->icon,
                'plans' => [],
            ];

            foreach ($planIds as $planId) {
                $feature = $planFeatures->firstWhere('plan_id', $planId);
                $featureData['plans'][$planId] = $feature 
                    ? $feature->comparison_display 
                    : 'Not Available';
            }

            $matrix[] = $featureData;
        }

        return $matrix;
    }

    /**
     * Export to array for API
     */
    public function toFeatureArray(): array
    {
        return [
            'id' => $this->id,
            'code' => $this->feature_code,
            'name' => $this->feature_name,
            'description' => $this->description,
            'category' => $this->category,
            'category_label' => $this->category_label,
            'type' => $this->type,
            'value' => $this->value,
            'limit' => $this->limit,
            'unit' => $this->unit,
            'is_unlimited' => $this->is_unlimited,
            'is_included' => $this->is_included,
            'is_highlighted' => $this->is_highlighted,
            'display_value' => $this->display_value,
            'comparison_display' => $this->comparison_display,
            'icon' => $this->icon,
            'tooltip' => $this->tooltip,
            'help_url' => $this->help_url,
            'is_addon' => $this->is_addon,
            'addon_price' => $this->addon_price,
            'overage_price' => $this->overage_price,
            'allows_overage' => $this->allows_overage,
        ];
    }
}