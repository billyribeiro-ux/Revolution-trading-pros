<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\Relations\MorphOne;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Product Model
 * 
 * Manages digital/physical products with comprehensive SEO, pricing, inventory,
 * and customer relationship tracking.
 *
 * @property int $id
 * @property string $name Product name
 * @property string $slug URL-friendly slug
 * @property string $sku Unique product identifier
 * @property string $type Product type (digital, physical, service, subscription, bundle)
 * @property string $description Short description
 * @property string|null $long_description Detailed description
 * @property float $price Base price
 * @property float|null $sale_price Sale/promotional price
 * @property float|null $cost Cost of goods sold
 * @property string|null $currency Currency code (USD, EUR, etc.)
 * @property bool $is_active Product is active/published
 * @property bool $is_featured Featured product
 * @property bool $is_new New arrival
 * @property bool $is_on_sale On sale status
 * @property array $metadata Additional product data
 * @property string|null $thumbnail Main thumbnail image
 * @property array $images Product image gallery
 * @property array $videos Product video URLs
 * @property array $downloads Downloadable files (for digital products)
 * @property string|null $category Primary category
 * @property array $tags Product tags
 * @property int $stock_quantity Available inventory
 * @property int|null $low_stock_threshold Alert threshold
 * @property bool $track_inventory Whether to track stock
 * @property bool $allow_backorder Allow purchase when out of stock
 * @property string|null $availability In stock, out of stock, preorder
 * @property float|null $weight Product weight
 * @property array|null $dimensions Length, width, height
 * @property bool $requires_shipping Physical shipping required
 * @property string|null $shipping_class Shipping classification
 * @property int $view_count Total views
 * @property int $purchase_count Total purchases
 * @property int $wishlist_count Times added to wishlist
 * @property float|null $average_rating Average customer rating (0-5)
 * @property int $review_count Total reviews
 * @property int $sort_order Display sort order
 * @property \Illuminate\Support\Carbon|null $available_from Product availability start
 * @property \Illuminate\Support\Carbon|null $available_until Product availability end
 * @property \Illuminate\Support\Carbon|null $sale_starts_at Sale period start
 * @property \Illuminate\Support\Carbon|null $sale_ends_at Sale period end
 * @property string|null $meta_title SEO title
 * @property string|null $meta_description SEO description
 * @property string|null $meta_keywords SEO keywords
 * @property bool $indexable Allow search engine indexing
 * @property string|null $canonical_url Canonical URL
 * @property array $structured_data Schema.org structured data
 * @property string|null $created_by User who created product
 * @property string|null $updated_by User who last updated product
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Collection $users Customers who purchased
 * @property-read \Illuminate\Database\Eloquent\Collection $reviews Product reviews
 * @property-read \App\Models\SeoAnalysis|null $seoAnalysis SEO analysis data
 */
class Product extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Product types
     */
    public const TYPE_DIGITAL = 'digital';
    public const TYPE_PHYSICAL = 'physical';
    public const TYPE_SERVICE = 'service';
    public const TYPE_SUBSCRIPTION = 'subscription';
    public const TYPE_BUNDLE = 'bundle';
    public const TYPE_COURSE = 'course';
    public const TYPE_MEMBERSHIP = 'membership';

    /**
     * Availability statuses
     */
    public const AVAILABILITY_IN_STOCK = 'in_stock';
    public const AVAILABILITY_OUT_OF_STOCK = 'out_of_stock';
    public const AVAILABILITY_PREORDER = 'preorder';
    public const AVAILABILITY_DISCONTINUED = 'discontinued';
    public const AVAILABILITY_BACKORDER = 'backorder';

    /**
     * Valid product types
     */
    public const VALID_TYPES = [
        self::TYPE_DIGITAL,
        self::TYPE_PHYSICAL,
        self::TYPE_SERVICE,
        self::TYPE_SUBSCRIPTION,
        self::TYPE_BUNDLE,
        self::TYPE_COURSE,
        self::TYPE_MEMBERSHIP,
    ];

    /**
     * Valid availability statuses
     */
    public const VALID_AVAILABILITIES = [
        self::AVAILABILITY_IN_STOCK,
        self::AVAILABILITY_OUT_OF_STOCK,
        self::AVAILABILITY_PREORDER,
        self::AVAILABILITY_DISCONTINUED,
        self::AVAILABILITY_BACKORDER,
    ];

    /**
     * Low stock threshold default
     */
    public const DEFAULT_LOW_STOCK_THRESHOLD = 10;

    /**
     * Cache duration
     */
    public const CACHE_TTL = 3600; // 1 hour

    protected $fillable = [
        'name',
        'slug',
        'sku',
        'type',
        'description',
        'long_description',
        'price',
        'sale_price',
        'cost',
        'currency',
        'is_active',
        'is_featured',
        'is_new',
        'is_on_sale',
        'metadata',
        'thumbnail',
        'images',
        'videos',
        'downloads',
        'category',
        'tags',
        'stock_quantity',
        'low_stock_threshold',
        'track_inventory',
        'allow_backorder',
        'availability',
        'weight',
        'dimensions',
        'requires_shipping',
        'shipping_class',
        'view_count',
        'purchase_count',
        'wishlist_count',
        'average_rating',
        'review_count',
        'sort_order',
        'available_from',
        'available_until',
        'sale_starts_at',
        'sale_ends_at',
        'meta_title',
        'meta_description',
        'meta_keywords',
        'indexable',
        'canonical_url',
        'structured_data',
        'created_by',
        'updated_by',
    ];

    protected $casts = [
        'metadata' => 'array',
        'images' => 'array',
        'videos' => 'array',
        'downloads' => 'array',
        'tags' => 'array',
        'dimensions' => 'array',
        'structured_data' => 'array',
        'is_active' => 'boolean',
        'is_featured' => 'boolean',
        'is_new' => 'boolean',
        'is_on_sale' => 'boolean',
        'track_inventory' => 'boolean',
        'allow_backorder' => 'boolean',
        'requires_shipping' => 'boolean',
        'indexable' => 'boolean',
        'price' => 'decimal:2',
        'sale_price' => 'decimal:2',
        'cost' => 'decimal:2',
        'weight' => 'decimal:2',
        'average_rating' => 'decimal:2',
        'stock_quantity' => 'integer',
        'low_stock_threshold' => 'integer',
        'view_count' => 'integer',
        'purchase_count' => 'integer',
        'wishlist_count' => 'integer',
        'review_count' => 'integer',
        'sort_order' => 'integer',
        'available_from' => 'datetime',
        'available_until' => 'datetime',
        'sale_starts_at' => 'datetime',
        'sale_ends_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'is_active' => true,
        'is_featured' => false,
        'is_new' => false,
        'is_on_sale' => false,
        'track_inventory' => true,
        'allow_backorder' => false,
        'requires_shipping' => false,
        'indexable' => true,
        'stock_quantity' => 0,
        'view_count' => 0,
        'purchase_count' => 0,
        'wishlist_count' => 0,
        'review_count' => 0,
        'sort_order' => 0,
        'currency' => 'USD',
        'availability' => self::AVAILABILITY_IN_STOCK,
        'metadata' => '[]',
        'images' => '[]',
        'videos' => '[]',
        'downloads' => '[]',
        'tags' => '[]',
        'structured_data' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->generateSlugIfMissing();
            $model->generateSkuIfMissing();
            $model->validateType();
            $model->normalizeData();
            $model->calculateAvailability();
            $model->setDefaultLowStockThreshold();
        });

        static::updating(function (self $model): void {
            $model->validateType();
            $model->normalizeData();
            
            if ($model->isDirty(['stock_quantity', 'track_inventory'])) {
                $model->calculateAvailability();
            }
            
            if ($model->isDirty(['price', 'sale_price', 'sale_starts_at', 'sale_ends_at'])) {
                $model->updateSaleStatus();
            }
        });

        static::saved(function (self $model): void {
            $model->clearProductCache();
            $model->updateStructuredData();
        });

        static::deleted(function (self $model): void {
            $model->clearProductCache();
        });
    }

    /**
     * Get route key name (use slug for URLs)
     */
    public function getRouteKeyName(): string
    {
        return 'slug';
    }

    /**
     * Get users who purchased this product
     */
    public function users(): BelongsToMany
    {
        return $this->belongsToMany(User::class, 'user_products')
            ->withPivot(['purchased_at', 'order_id', 'quantity', 'price_paid', 'status'])
            ->withTimestamps();
    }

    /**
     * Get product reviews
     */
    public function reviews(): HasMany
    {
        return $this->hasMany(ProductReview::class);
    }

    /**
     * Get SEO analysis
     */
    public function seoAnalysis(): MorphOne
    {
        return $this->morphOne(SeoAnalysis::class, 'analyzable');
    }

    /**
     * Get related products
     */
    public function relatedProducts(): BelongsToMany
    {
        return $this->belongsToMany(Product::class, 'related_products', 'product_id', 'related_product_id')
            ->withTimestamps();
    }

    /**
     * Get product variants (e.g., sizes, colors)
     */
    public function variants(): HasMany
    {
        return $this->hasMany(ProductVariant::class);
    }

    /**
     * Generate slug from name if missing
     */
    protected function generateSlugIfMissing(): void
    {
        if (empty($this->slug)) {
            $this->slug = Str::slug($this->name);
        }

        // Ensure uniqueness
        $originalSlug = $this->slug;
        $counter = 1;

        while (static::where('slug', $this->slug)
            ->where('id', '!=', $this->id ?? 0)
            ->exists()) {
            $this->slug = $originalSlug . '-' . $counter++;
        }
    }

    /**
     * Generate SKU if missing
     */
    protected function generateSkuIfMissing(): void
    {
        if (empty($this->sku)) {
            $prefix = strtoupper(substr($this->type, 0, 3));
            $random = strtoupper(Str::random(8));
            $this->sku = "{$prefix}-{$random}";
        }
    }

    /**
     * Validate product type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid product type: %s', $this->type)
            );
        }
    }

    /**
     * Normalize product data
     */
    protected function normalizeData(): void
    {
        // Ensure arrays are properly formatted
        foreach (['images', 'videos', 'downloads', 'tags', 'metadata'] as $field) {
            if (!is_array($this->{$field})) {
                $this->{$field} = [];
            }
        }

        // Normalize price
        if ($this->price < 0) {
            $this->price = 0;
        }

        // Normalize sale price
        if ($this->sale_price !== null && $this->sale_price >= $this->price) {
            $this->sale_price = null;
        }
    }

    /**
     * Calculate availability status
     */
    protected function calculateAvailability(): void
    {
        if (!$this->track_inventory) {
            $this->availability = self::AVAILABILITY_IN_STOCK;
            return;
        }

        if ($this->stock_quantity <= 0) {
            $this->availability = $this->allow_backorder 
                ? self::AVAILABILITY_BACKORDER 
                : self::AVAILABILITY_OUT_OF_STOCK;
        } else {
            $this->availability = self::AVAILABILITY_IN_STOCK;
        }
    }

    /**
     * Update sale status based on dates
     */
    protected function updateSaleStatus(): void
    {
        $now = now();
        $isInSalePeriod = true;

        if ($this->sale_starts_at && $now->isBefore($this->sale_starts_at)) {
            $isInSalePeriod = false;
        }

        if ($this->sale_ends_at && $now->isAfter($this->sale_ends_at)) {
            $isInSalePeriod = false;
        }

        $this->is_on_sale = $this->sale_price && $isInSalePeriod;
    }

    /**
     * Set default low stock threshold
     */
    protected function setDefaultLowStockThreshold(): void
    {
        if ($this->low_stock_threshold === null && $this->track_inventory) {
            $this->low_stock_threshold = self::DEFAULT_LOW_STOCK_THRESHOLD;
        }
    }

    /**
     * Update structured data for SEO
     */
    protected function updateStructuredData(): void
    {
        $structuredData = [
            '@context' => 'https://schema.org',
            '@type' => 'Product',
            'name' => $this->name,
            'description' => $this->description,
            'sku' => $this->sku,
            'image' => $this->thumbnail ? url($this->thumbnail) : null,
            'offers' => [
                '@type' => 'Offer',
                'price' => $this->getCurrentPrice(),
                'priceCurrency' => $this->currency,
                'availability' => $this->getSchemaAvailability(),
                'url' => $this->getPublicUrl(),
            ],
        ];

        if ($this->average_rating && $this->review_count > 0) {
            $structuredData['aggregateRating'] = [
                '@type' => 'AggregateRating',
                'ratingValue' => $this->average_rating,
                'reviewCount' => $this->review_count,
                'bestRating' => 5,
                'worstRating' => 1,
            ];
        }

        $this->updateQuietly(['structured_data' => $structuredData]);
    }

    /**
     * Get Schema.org availability string
     */
    protected function getSchemaAvailability(): string
    {
        return match($this->availability) {
            self::AVAILABILITY_IN_STOCK => 'https://schema.org/InStock',
            self::AVAILABILITY_OUT_OF_STOCK => 'https://schema.org/OutOfStock',
            self::AVAILABILITY_PREORDER => 'https://schema.org/PreOrder',
            self::AVAILABILITY_BACKORDER => 'https://schema.org/BackOrder',
            self::AVAILABILITY_DISCONTINUED => 'https://schema.org/Discontinued',
            default => 'https://schema.org/InStock',
        };
    }

    /**
     * Clear product cache
     */
    protected function clearProductCache(): void
    {
        Cache::tags(['products'])->flush();
        Cache::forget("product:slug:{$this->slug}");
        Cache::forget("product:sku:{$this->sku}");
    }

    /**
     * Get current effective price
     */
    public function getCurrentPrice(): float
    {
        if ($this->is_on_sale && $this->sale_price) {
            return (float) $this->sale_price;
        }

        return (float) $this->price;
    }

    /**
     * Get discount amount
     */
    public function getDiscountAmountAttribute(): ?float
    {
        if (!$this->is_on_sale || !$this->sale_price) {
            return null;
        }

        return round($this->price - $this->sale_price, 2);
    }

    /**
     * Get discount percentage
     */
    public function getDiscountPercentageAttribute(): ?float
    {
        if (!$this->is_on_sale || !$this->sale_price || $this->price <= 0) {
            return null;
        }

        return round((($this->price - $this->sale_price) / $this->price) * 100, 2);
    }

    /**
     * Get profit margin
     */
    public function getProfitMarginAttribute(): ?float
    {
        if (!$this->cost || $this->price <= 0) {
            return null;
        }

        return round((($this->price - $this->cost) / $this->price) * 100, 2);
    }

    /**
     * Get formatted price
     */
    public function getFormattedPriceAttribute(): string
    {
        return $this->formatMoney($this->getCurrentPrice());
    }

    /**
     * Get formatted original price
     */
    public function getFormattedOriginalPriceAttribute(): string
    {
        return $this->formatMoney($this->price);
    }

    /**
     * Format money with currency
     */
    protected function formatMoney(float $amount): string
    {
        return match($this->currency) {
            'USD' => '$' . number_format($amount, 2),
            'EUR' => '€' . number_format($amount, 2),
            'GBP' => '£' . number_format($amount, 2),
            default => $this->currency . ' ' . number_format($amount, 2),
        };
    }

    /**
     * Check if product is in stock
     */
    public function isInStock(): bool
    {
        if (!$this->track_inventory) {
            return true;
        }

        return $this->stock_quantity > 0 || $this->allow_backorder;
    }

    /**
     * Check if product is low stock
     */
    public function isLowStock(): bool
    {
        if (!$this->track_inventory || !$this->low_stock_threshold) {
            return false;
        }

        return $this->stock_quantity > 0 
            && $this->stock_quantity <= $this->low_stock_threshold;
    }

    /**
     * Check if product is out of stock
     */
    public function isOutOfStock(): bool
    {
        if (!$this->track_inventory) {
            return false;
        }

        return $this->stock_quantity <= 0 && !$this->allow_backorder;
    }

    /**
     * Check if product is available for purchase
     */
    public function isAvailable(): bool
    {
        if (!$this->is_active) {
            return false;
        }

        $now = now();

        if ($this->available_from && $now->isBefore($this->available_from)) {
            return false;
        }

        if ($this->available_until && $now->isAfter($this->available_until)) {
            return false;
        }

        return $this->isInStock();
    }

    /**
     * Check if product is digital
     */
    public function isDigital(): bool
    {
        return in_array($this->type, [
            self::TYPE_DIGITAL,
            self::TYPE_COURSE,
            self::TYPE_MEMBERSHIP,
            self::TYPE_SUBSCRIPTION,
        ], true);
    }

    /**
     * Check if product is physical
     */
    public function isPhysical(): bool
    {
        return $this->type === self::TYPE_PHYSICAL;
    }

    /**
     * Check if product can be purchased
     */
    public function canBePurchased(): bool
    {
        return $this->is_active && $this->isAvailable();
    }

    /**
     * Increment view count
     */
    public function incrementViews(): self
    {
        $this->increment('view_count');
        return $this;
    }

    /**
     * Increment purchase count
     */
    public function incrementPurchases(int $quantity = 1): self
    {
        $this->increment('purchase_count', $quantity);
        
        if ($this->track_inventory) {
            $this->decrement('stock_quantity', $quantity);
            $this->calculateAvailability();
            $this->save();
        }

        return $this;
    }

    /**
     * Increment wishlist count
     */
    public function incrementWishlist(): self
    {
        $this->increment('wishlist_count');
        return $this;
    }

    /**
     * Decrement wishlist count
     */
    public function decrementWishlist(): self
    {
        $this->decrement('wishlist_count');
        return $this;
    }

    /**
     * Update rating
     */
    public function updateRating(float $newRating): self
    {
        $totalRating = ($this->average_rating * $this->review_count) + $newRating;
        $this->review_count++;
        $this->average_rating = round($totalRating / $this->review_count, 2);
        $this->save();

        return $this;
    }

    /**
     * Restock product
     */
    public function restock(int $quantity): self
    {
        if ($this->track_inventory) {
            $this->increment('stock_quantity', $quantity);
            $this->calculateAvailability();
            $this->save();
        }

        return $this;
    }

    /**
     * Get public URL
     */
    public function getPublicUrl(): string
    {
        return url("/products/{$this->slug}");
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
     * Get stock status label
     */
    public function getStockStatusLabelAttribute(): string
    {
        return match($this->availability) {
            self::AVAILABILITY_IN_STOCK => 'In Stock',
            self::AVAILABILITY_OUT_OF_STOCK => 'Out of Stock',
            self::AVAILABILITY_PREORDER => 'Pre-Order',
            self::AVAILABILITY_BACKORDER => 'Backorder',
            self::AVAILABILITY_DISCONTINUED => 'Discontinued',
            default => 'Unknown',
        };
    }

    /**
     * Get stock status color
     */
    public function getStockStatusColorAttribute(): string
    {
        return match($this->availability) {
            self::AVAILABILITY_IN_STOCK => '#22c55e', // green-500
            self::AVAILABILITY_OUT_OF_STOCK => '#ef4444', // red-500
            self::AVAILABILITY_PREORDER => '#3b82f6', // blue-500
            self::AVAILABILITY_BACKORDER => '#f59e0b', // amber-500
            self::AVAILABILITY_DISCONTINUED => '#6b7280', // gray-500
            default => '#9ca3af', // gray-400
        };
    }

    /**
     * Get conversion rate (purchases / views)
     */
    public function getConversionRateAttribute(): float
    {
        if ($this->view_count === 0) {
            return 0.0;
        }

        return round(($this->purchase_count / $this->view_count) * 100, 2);
    }

    /**
     * Get revenue generated
     */
    public function getTotalRevenueAttribute(): float
    {
        return round($this->purchase_count * $this->price, 2);
    }

    /**
     * Scope: Active products only
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    /**
     * Scope: Inactive products
     */
    public function scopeInactive(Builder $query): Builder
    {
        return $query->where('is_active', false);
    }

    /**
     * Scope: Featured products
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true);
    }

    /**
     * Scope: New products
     */
    public function scopeNew(Builder $query): Builder
    {
        return $query->where('is_new', true);
    }

    /**
     * Scope: On sale products
     */
    public function scopeOnSale(Builder $query): Builder
    {
        return $query->where('is_on_sale', true);
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Digital products
     */
    public function scopeDigital(Builder $query): Builder
    {
        return $query->whereIn('type', [
            self::TYPE_DIGITAL,
            self::TYPE_COURSE,
            self::TYPE_MEMBERSHIP,
            self::TYPE_SUBSCRIPTION,
        ]);
    }

    /**
     * Scope: Physical products
     */
    public function scopePhysical(Builder $query): Builder
    {
        return $query->where('type', self::TYPE_PHYSICAL);
    }

    /**
     * Scope: In stock products
     */
    public function scopeInStock(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->where('track_inventory', false)
              ->orWhere('stock_quantity', '>', 0)
              ->orWhere('allow_backorder', true);
        });
    }

    /**
     * Scope: Low stock products
     */
    public function scopeLowStock(Builder $query): Builder
    {
        return $query->where('track_inventory', true)
            ->whereColumn('stock_quantity', '<=', 'low_stock_threshold')
            ->where('stock_quantity', '>', 0);
    }

    /**
     * Scope: Out of stock products
     */
    public function scopeOutOfStock(Builder $query): Builder
    {
        return $query->where('track_inventory', true)
            ->where('stock_quantity', '<=', 0)
            ->where('allow_backorder', false);
    }

    /**
     * Scope: Filter by category
     */
    public function scopeInCategory(Builder $query, string $category): Builder
    {
        return $query->where('category', $category);
    }

    /**
     * Scope: Filter by tag
     */
    public function scopeWithTag(Builder $query, string $tag): Builder
    {
        return $query->whereJsonContains('tags', $tag);
    }

    /**
     * Scope: Price range
     */
    public function scopePriceBetween(Builder $query, float $min, float $max): Builder
    {
        return $query->whereBetween('price', [$min, $max]);
    }

    /**
     * Scope: Popular products (by purchases)
     */
    public function scopePopular(Builder $query, int $minPurchases = 10): Builder
    {
        return $query->where('purchase_count', '>=', $minPurchases);
    }

    /**
     * Scope: Trending products (recent purchases)
     */
    public function scopeTrending(Builder $query, int $days = 7): Builder
    {
        return $query->whereHas('users', function($q) use ($days) {
            $q->where('user_products.created_at', '>=', now()->subDays($days));
        });
    }

    /**
     * Scope: Highly rated products
     */
    public function scopeHighlyRated(Builder $query, float $minRating = 4.0): Builder
    {
        return $query->where('average_rating', '>=', $minRating)
            ->where('review_count', '>', 0);
    }

    /**
     * Scope: Order by popularity
     */
    public function scopeOrderByPopularity(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('purchase_count', $direction);
    }

    /**
     * Scope: Order by rating
     */
    public function scopeOrderByRating(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('average_rating', $direction);
    }

    /**
     * Scope: Order by price
     */
    public function scopeOrderByPrice(Builder $query, string $direction = 'asc'): Builder
    {
        return $query->orderBy('price', $direction);
    }

    /**
     * Scope: Order by sort order
     */
    public function scopeOrderBySortOrder(Builder $query, string $direction = 'asc'): Builder
    {
        return $query->orderBy('sort_order', $direction);
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total' => static::count(),
            'active' => static::active()->count(),
            'inactive' => static::inactive()->count(),
            'featured' => static::featured()->count(),
            'on_sale' => static::onSale()->count(),
            'in_stock' => static::inStock()->count(),
            'low_stock' => static::lowStock()->count(),
            'out_of_stock' => static::outOfStock()->count(),
            'digital' => static::digital()->count(),
            'physical' => static::physical()->count(),
            'total_views' => static::sum('view_count'),
            'total_purchases' => static::sum('purchase_count'),
            'total_revenue' => static::sum(\DB::raw('purchase_count * price')),
            'avg_rating' => round(static::where('review_count', '>', 0)->avg('average_rating'), 2),
        ];
    }

    /**
     * Static: Get best sellers
     */
    public static function getBestSellers(int $limit = 10): Collection
    {
        return static::active()
            ->orderByPopularity()
            ->limit($limit)
            ->get();
    }

    /**
     * Static: Get low stock alert products
     */
    public static function getLowStockProducts(): Collection
    {
        return static::active()
            ->lowStock()
            ->orderBy('stock_quantity', 'asc')
            ->get();
    }

    /**
     * Static: Search products
     */
    public static function search(string $query): Builder
    {
        return static::where(function($q) use ($query) {
            $q->where('name', 'LIKE', "%{$query}%")
              ->orWhere('description', 'LIKE', "%{$query}%")
              ->orWhere('sku', 'LIKE', "%{$query}%")
              ->orWhereJsonContains('tags', $query);
        });
    }

    /**
     * Export to array for API
     */
    public function toProductArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'slug' => $this->slug,
            'sku' => $this->sku,
            'type' => $this->type,
            'description' => $this->description,
            'price' => $this->price,
            'sale_price' => $this->sale_price,
            'current_price' => $this->getCurrentPrice(),
            'formatted_price' => $this->formatted_price,
            'discount_percentage' => $this->discount_percentage,
            'is_on_sale' => $this->is_on_sale,
            'is_featured' => $this->is_featured,
            'is_new' => $this->is_new,
            'is_available' => $this->isAvailable(),
            'thumbnail' => $this->thumbnail ? url($this->thumbnail) : null,
            'images' => $this->images,
            'category' => $this->category,
            'tags' => $this->tags,
            'stock_status' => $this->stock_status_label,
            'stock_quantity' => $this->track_inventory ? $this->stock_quantity : null,
            'average_rating' => $this->average_rating,
            'review_count' => $this->review_count,
            'view_count' => $this->view_count,
            'purchase_count' => $this->purchase_count,
            'url' => $this->getPublicUrl(),
            'created_at' => $this->created_at->toISOString(),
        ];
    }
}