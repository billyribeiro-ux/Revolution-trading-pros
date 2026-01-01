<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;

/**
 * NewsletterCategory Model
 *
 * Manages newsletter categories/topics for segmented email campaigns.
 * Enables subscribers to choose specific content areas of interest.
 *
 * @property int $id
 * @property string $name Category display name
 * @property string $slug URL-safe unique identifier
 * @property string|null $description Category description
 * @property string|null $icon Icon class or emoji
 * @property string|null $color Hex color code
 * @property int $sort_order Display order
 * @property bool $is_active Active status
 * @property bool $is_default Auto-select for new subscribers
 * @property int $subscriber_count Cached subscriber count
 * @property int $email_count Total emails sent to this category
 * @property float $avg_open_rate Average open rate
 * @property float $avg_click_rate Average click rate
 * @property array|null $settings Additional settings
 * @property array|null $metadata Custom metadata
 * @property \Carbon\Carbon|null $last_sent_at Last email sent timestamp
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 * @property \Carbon\Carbon|null $deleted_at
 *
 * @level ICT11 Principal Engineer
 */
class NewsletterCategory extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'newsletter_categories';

    protected $fillable = [
        'name',
        'slug',
        'description',
        'icon',
        'color',
        'sort_order',
        'is_active',
        'is_default',
        'subscriber_count',
        'email_count',
        'avg_open_rate',
        'avg_click_rate',
        'settings',
        'metadata',
        'last_sent_at',
    ];

    protected $casts = [
        'sort_order' => 'integer',
        'is_active' => 'boolean',
        'is_default' => 'boolean',
        'subscriber_count' => 'integer',
        'email_count' => 'integer',
        'avg_open_rate' => 'float',
        'avg_click_rate' => 'float',
        'settings' => 'array',
        'metadata' => 'array',
        'last_sent_at' => 'datetime',
    ];

    protected $attributes = [
        'sort_order' => 0,
        'is_active' => true,
        'is_default' => false,
        'subscriber_count' => 0,
        'email_count' => 0,
        'avg_open_rate' => 0.0,
        'avg_click_rate' => 0.0,
    ];

    // =========================================================================
    // BOOT
    // =========================================================================

    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $category) {
            if (empty($category->slug)) {
                $category->slug = Str::slug($category->name);
            }

            // Ensure unique slug
            $originalSlug = $category->slug;
            $counter = 1;
            while (static::where('slug', $category->slug)->exists()) {
                $category->slug = $originalSlug . '-' . $counter++;
            }
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get subscribers in this category
     */
    public function subscribers(): Builder
    {
        return NewsletterSubscription::whereJsonContains('segments', $this->slug);
    }

    /**
     * Get campaigns sent to this category
     */
    public function campaigns(): Builder
    {
        return EmailCampaign::whereJsonContains('target_segments', $this->slug);
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    public function scopeDefault(Builder $query): Builder
    {
        return $query->where('is_default', true);
    }

    public function scopeOrdered(Builder $query): Builder
    {
        return $query->orderBy('sort_order')->orderBy('name');
    }

    public function scopePopular(Builder $query): Builder
    {
        return $query->orderByDesc('subscriber_count');
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Update subscriber count from database
     */
    public function updateSubscriberCount(): void
    {
        $this->subscriber_count = NewsletterSubscription::query()
            ->where('status', NewsletterSubscription::STATUS_ACTIVE)
            ->whereJsonContains('segments', $this->slug)
            ->count();

        $this->save();
    }

    /**
     * Update performance metrics
     */
    public function updatePerformanceMetrics(): void
    {
        $subscribers = NewsletterSubscription::query()
            ->where('status', NewsletterSubscription::STATUS_ACTIVE)
            ->whereJsonContains('segments', $this->slug)
            ->get();

        if ($subscribers->isEmpty()) {
            return;
        }

        $this->avg_open_rate = $subscribers->avg('open_rate') ?? 0;
        $this->avg_click_rate = $subscribers->avg('click_rate') ?? 0;
        $this->save();
    }

    /**
     * Get category statistics
     */
    public function getStatistics(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'slug' => $this->slug,
            'subscriber_count' => $this->subscriber_count,
            'email_count' => $this->email_count,
            'avg_open_rate' => round($this->avg_open_rate, 2),
            'avg_click_rate' => round($this->avg_click_rate, 2),
            'last_sent_at' => $this->last_sent_at?->toIso8601String(),
            'engagement_score' => $this->calculateEngagementScore(),
        ];
    }

    /**
     * Calculate engagement score for category
     */
    public function calculateEngagementScore(): float
    {
        // Weighted score: 40% open rate, 50% click rate, 10% recency
        $openScore = min($this->avg_open_rate * 2, 100); // Normalize to 0-100
        $clickScore = min($this->avg_click_rate * 5, 100); // Normalize to 0-100

        $recencyScore = 0;
        if ($this->last_sent_at) {
            $daysSinceSent = now()->diffInDays($this->last_sent_at);
            $recencyScore = max(0, 100 - ($daysSinceSent * 3)); // Decay over ~33 days
        }

        return round(
            ($openScore * 0.4) + ($clickScore * 0.5) + ($recencyScore * 0.1),
            2
        );
    }

    /**
     * Add subscriber to category
     */
    public function addSubscriber(NewsletterSubscription $subscription): void
    {
        $segments = $subscription->segments ?? [];

        if (!in_array($this->slug, $segments)) {
            $segments[] = $this->slug;
            $subscription->segments = $segments;
            $subscription->save();

            $this->increment('subscriber_count');
        }
    }

    /**
     * Remove subscriber from category
     */
    public function removeSubscriber(NewsletterSubscription $subscription): void
    {
        $segments = $subscription->segments ?? [];
        $key = array_search($this->slug, $segments);

        if ($key !== false) {
            unset($segments[$key]);
            $subscription->segments = array_values($segments);
            $subscription->save();

            $this->decrement('subscriber_count');
        }
    }

    /**
     * Get for dropdown/select
     */
    public static function getForSelect(): array
    {
        return static::active()
            ->ordered()
            ->get()
            ->map(fn ($cat) => [
                'value' => $cat->slug,
                'label' => $cat->name,
                'description' => $cat->description,
                'icon' => $cat->icon,
                'color' => $cat->color,
                'subscriber_count' => $cat->subscriber_count,
            ])
            ->toArray();
    }

    /**
     * Seed default categories
     */
    public static function seedDefaults(): void
    {
        $defaults = [
            [
                'name' => 'Market Updates',
                'slug' => 'market-updates',
                'description' => 'Daily market analysis and trading insights',
                'icon' => '📈',
                'color' => '#4F46E5',
                'sort_order' => 1,
                'is_default' => true,
            ],
            [
                'name' => 'Trading Strategies',
                'slug' => 'trading-strategies',
                'description' => 'Learn new trading techniques and strategies',
                'icon' => '🎯',
                'color' => '#10B981',
                'sort_order' => 2,
            ],
            [
                'name' => 'Product Updates',
                'slug' => 'product-updates',
                'description' => 'New features and platform improvements',
                'icon' => '🚀',
                'color' => '#F59E0B',
                'sort_order' => 3,
            ],
            [
                'name' => 'Educational Content',
                'slug' => 'educational',
                'description' => 'Trading tutorials and learning resources',
                'icon' => '📚',
                'color' => '#8B5CF6',
                'sort_order' => 4,
            ],
            [
                'name' => 'Promotions & Offers',
                'slug' => 'promotions',
                'description' => 'Special deals and exclusive offers',
                'icon' => '🎁',
                'color' => '#EC4899',
                'sort_order' => 5,
            ],
        ];

        foreach ($defaults as $category) {
            static::firstOrCreate(
                ['slug' => $category['slug']],
                $category
            );
        }
    }
}
