<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Kalnoy\Nestedset\NodeTrait;
use Spatie\Sluggable\HasSlug;
use Spatie\Sluggable\SlugOptions;
use App\Traits\HasUuid;
use App\Traits\HasMedia;
use App\Traits\Searchable;
use App\Traits\Trackable;
use App\Traits\Translatable;
use App\Contracts\Taxonomizable;
use App\Events\CategoryCreated;
use App\Events\CategoryMoved;
use App\Events\CategoryMerged;
use App\Events\CategoryDeleted;
use App\Enums\CategoryStatus;
use App\Enums\CategoryType;
use App\Services\SEO\SchemaGenerator;
use Carbon\Carbon;

/**
 * Category Model
 * 
 * Enterprise-grade hierarchical category system with nested set implementation,
 * advanced caching, SEO optimization, and comprehensive content management.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $name
 * @property string $slug
 * @property string|null $description
 * @property string|null $short_description
 * @property string|null $long_description
 * @property string $color
 * @property string|null $icon
 * @property string|null $image
 * @property string|null $banner_image
 * @property array|null $images
 * @property int $order
 * @property int $_lft
 * @property int $_rgt
 * @property int|null $parent_id
 * @property int $depth
 * @property string $path
 * @property string $full_path
 * @property bool $is_visible
 * @property bool $is_featured
 * @property bool $is_navigational
 * @property bool $is_searchable
 * @property bool $is_default
 * @property bool $allow_posts
 * @property bool $allow_subcategories
 * @property bool $inherit_parent_settings
 * @property string $status
 * @property string $type
 * @property array|null $settings
 * @property array|null $metadata
 * @property array|null $custom_fields
 * @property string|null $meta_title
 * @property string|null $meta_description
 * @property array|null $meta_keywords
 * @property string|null $canonical_url
 * @property array|null $og_data
 * @property array|null $schema_data
 * @property float $popularity_score
 * @property float $engagement_score
 * @property int $post_count
 * @property int $direct_post_count
 * @property int $total_post_count
 * @property int $published_post_count
 * @property int $view_count
 * @property int $unique_view_count
 * @property int $click_count
 * @property int $search_count
 * @property float $avg_post_rating
 * @property float $avg_time_on_page
 * @property float $bounce_rate
 * @property array|null $access_rules
 * @property array|null $posting_rules
 * @property array|null $moderation_rules
 * @property array|null $display_options
 * @property array|null $layout_settings
 * @property string|null $template
 * @property string|null $redirect_url
 * @property bool $is_redirected
 * @property int $created_by
 * @property int|null $updated_by
 * @property Carbon|null $published_at
 * @property Carbon|null $featured_at
 * @property Carbon|null $archived_at
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\Category|null $parent
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Category[] $children
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Category[] $descendants
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Category[] $ancestors
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Category[] $siblings
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Post[] $posts
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Post[] $publishedPosts
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Post[] $featuredPosts
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Tag[] $tags
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\User[] $subscribers
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\User[] $moderators
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CategoryTranslation[] $translations
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CategoryHistory[] $history
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CategoryMetric[] $metrics
 * 
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category root()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category visible()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category featured()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category navigational()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category withDepth()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category defaultOrder()
 * @method static \Kalnoy\Nestedset\QueryBuilder|Category reversed()
 */
class Category extends Model implements Taxonomizable
{
    use HasFactory;
    use SoftDeletes;
    use NodeTrait; // For nested set management
    use HasSlug;
    use HasUuid;
    use HasMedia;
    use Searchable;
    use Trackable;
    use Translatable;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'categories';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'name',
        'slug',
        'description',
        'short_description',
        'long_description',
        'color',
        'icon',
        'image',
        'banner_image',
        'images',
        'order',
        'parent_id',
        'path',
        'full_path',
        'is_visible',
        'is_featured',
        'is_navigational',
        'is_searchable',
        'is_default',
        'allow_posts',
        'allow_subcategories',
        'inherit_parent_settings',
        'status',
        'type',
        'settings',
        'metadata',
        'custom_fields',
        'meta_title',
        'meta_description',
        'meta_keywords',
        'canonical_url',
        'og_data',
        'schema_data',
        'popularity_score',
        'engagement_score',
        'access_rules',
        'posting_rules',
        'moderation_rules',
        'display_options',
        'layout_settings',
        'template',
        'redirect_url',
        'is_redirected',
        'created_by',
        'updated_by',
        'published_at',
        'featured_at',
        'archived_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'is_visible' => 'boolean',
        'is_featured' => 'boolean',
        'is_navigational' => 'boolean',
        'is_searchable' => 'boolean',
        'is_default' => 'boolean',
        'allow_posts' => 'boolean',
        'allow_subcategories' => 'boolean',
        'inherit_parent_settings' => 'boolean',
        'is_redirected' => 'boolean',
        'order' => 'integer',
        'parent_id' => 'integer',
        'depth' => 'integer',
        'post_count' => 'integer',
        'direct_post_count' => 'integer',
        'total_post_count' => 'integer',
        'published_post_count' => 'integer',
        'view_count' => 'integer',
        'unique_view_count' => 'integer',
        'click_count' => 'integer',
        'search_count' => 'integer',
        'popularity_score' => 'float',
        'engagement_score' => 'float',
        'avg_post_rating' => 'float',
        'avg_time_on_page' => 'float',
        'bounce_rate' => 'float',
        'status' => CategoryStatus::class,
        'type' => CategoryType::class,
        'images' => 'array',
        'settings' => 'array',
        'metadata' => 'array',
        'custom_fields' => 'array',
        'meta_keywords' => 'array',
        'og_data' => 'array',
        'schema_data' => 'array',
        'access_rules' => 'array',
        'posting_rules' => 'array',
        'moderation_rules' => 'array',
        'display_options' => 'array',
        'layout_settings' => 'array',
        'published_at' => 'datetime',
        'featured_at' => 'datetime',
        'archived_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be searchable.
     *
     * @var array<string>
     */
    protected $searchable = [
        'name',
        'slug',
        'description',
        'meta_title',
        'meta_description',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'deleted_at',
        '_lft',
        '_rgt',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'post_count',
        'breadcrumbs',
        'level',
        'is_parent',
        'is_leaf',
        'url',
        'admin_url',
        'full_name',
        'tree_name',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'color' => '#6366f1',
        'is_visible' => true,
        'is_featured' => false,
        'is_navigational' => true,
        'is_searchable' => true,
        'allow_posts' => true,
        'allow_subcategories' => true,
        'inherit_parent_settings' => false,
        'status' => CategoryStatus::ACTIVE,
        'type' => CategoryType::GENERAL,
        'order' => 0,
        'popularity_score' => 0,
        'engagement_score' => 0,
        'view_count' => 0,
    ];

    /**
     * The cache tags for the model.
     *
     * @var array<string>
     */
    protected static array $cacheTags = ['categories'];

    /**
     * Cache duration in seconds.
     *
     * @var int
     */
    protected static int $cacheDuration = 3600;

    /**
     * Boot the model.
     *
     * @return void
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $category) {
            $category->uuid = $category->uuid ?? (string) Str::uuid();
            $category->created_by = $category->created_by ?? auth()->id();
            
            if (is_null($category->order)) {
                $maxOrder = static::when($category->parent_id, function ($query) use ($category) {
                    return $query->where('parent_id', $category->parent_id);
                })->max('order');
                
                $category->order = ($maxOrder ?? -1) + 1;
            }

            $category->updatePaths();
            $category->generateMetaTags();
        });

        static::created(function (self $category) {
            event(new CategoryCreated($category));
            $category->clearAllCaches();
            $category->updateAncestorCounts();
        });

        static::updating(function (self $category) {
            $category->updated_by = auth()->id();
            
            if ($category->isDirty('parent_id')) {
                $category->updatePaths();
                $category->handleParentChange();
            }
            
            if ($category->isDirty(['name', 'description'])) {
                $category->generateMetaTags();
            }
        });

        static::updated(function (self $category) {
            $category->clearAllCaches();
            
            if ($category->wasChanged('parent_id')) {
                event(new CategoryMoved($category));
                $category->updateDescendantPaths();
                $category->updateAncestorCounts();
            }
        });

        static::deleting(function (self $category) {
            if (!$category->canBeDeleted()) {
                throw new \Exception('Category cannot be deleted: ' . $category->getDeletionBlockReason());
            }
            
            $category->handleDeletion();
        });

        static::deleted(function (self $category) {
            event(new CategoryDeleted($category));
            $category->clearAllCaches();
            $category->updateAncestorCounts();
        });
    }

    /**
     * Get slug options.
     */
    public function getSlugOptions(): SlugOptions
    {
        return SlugOptions::create()
            ->generateSlugsFrom('name')
            ->saveSlugsTo('slug')
            ->doNotGenerateSlugsOnUpdate();
    }

    /**
     * Get the parent category.
     */
    public function parent(): BelongsTo
    {
        return $this->belongsTo(Category::class, 'parent_id');
    }

    /**
     * Get the child categories.
     */
    public function children(): HasMany
    {
        return $this->hasMany(Category::class, 'parent_id')
            ->orderBy('order')
            ->orderBy('name');
    }

    /**
     * Get all posts in this category.
     */
    public function posts(): MorphToMany
    {
        return $this->morphedByMany(Post::class, 'categorizable')
            ->withTimestamps()
            ->withPivot(['order', 'is_primary'])
            ->orderByPivot('order');
    }

    /**
     * Get published posts.
     */
    public function publishedPosts(): MorphToMany
    {
        return $this->posts()
            ->where('status', 'published')
            ->where('published_at', '<=', now());
    }

    /**
     * Get featured posts.
     */
    public function featuredPosts(): MorphToMany
    {
        return $this->posts()
            ->where('is_featured', true)
            ->where('status', 'published')
            ->limit(10);
    }

    /**
     * Get all posts including descendants.
     */
    public function allPosts(): Builder
    {
        $categoryIds = $this->descendants()->pluck('id')->push($this->id);
        
        return Post::whereHas('categories', function ($query) use ($categoryIds) {
            $query->whereIn('categories.id', $categoryIds);
        });
    }

    /**
     * Get related tags.
     */
    public function tags(): BelongsToMany
    {
        return $this->belongsToMany(Tag::class, 'category_tags')
            ->withTimestamps()
            ->withPivot(['weight', 'relevance'])
            ->orderByPivot('weight', 'desc');
    }

    /**
     * Get subscribers.
     */
    public function subscribers(): BelongsToMany
    {
        return $this->belongsToMany(User::class, 'category_subscribers')
            ->withTimestamps()
            ->withPivot(['notification_preferences', 'subscribed_at']);
    }

    /**
     * Get moderators.
     */
    public function moderators(): BelongsToMany
    {
        return $this->belongsToMany(User::class, 'category_moderators')
            ->withTimestamps()
            ->withPivot(['permissions', 'assigned_at', 'assigned_by']);
    }

    /**
     * Get translations.
     */
    public function translations(): HasMany
    {
        return $this->hasMany(CategoryTranslation::class);
    }

    /**
     * Get history records.
     */
    public function history(): HasMany
    {
        return $this->hasMany(CategoryHistory::class)
            ->orderBy('created_at', 'desc');
    }

    /**
     * Get metrics.
     */
    public function metrics(): HasMany
    {
        return $this->hasMany(CategoryMetric::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get activity logs.
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Scope for visible categories.
     */
    public function scopeVisible(Builder $query): Builder
    {
        return $query->where('is_visible', true)
            ->where('status', CategoryStatus::ACTIVE);
    }

    /**
     * Scope for featured categories.
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true)
            ->whereNotNull('featured_at')
            ->orderBy('featured_at', 'desc');
    }

    /**
     * Scope for navigational categories.
     */
    public function scopeNavigational(Builder $query): Builder
    {
        return $query->where('is_navigational', true)
            ->visible();
    }

    /**
     * Scope for root categories.
     */
    public function scopeRoot(Builder $query): Builder
    {
        return $query->whereNull('parent_id');
    }

    /**
     * Scope for popular categories.
     */
    public function scopePopular(Builder $query): Builder
    {
        return $query->orderBy('popularity_score', 'desc')
            ->orderBy('post_count', 'desc');
    }

    /**
     * Scope for trending categories.
     */
    public function scopeTrending(Builder $query, int $days = 7): Builder
    {
        return $query->withCount(['posts' => function ($q) use ($days) {
                $q->where('created_at', '>=', now()->subDays($days));
            }])
            ->having('posts_count', '>', 0)
            ->orderBy('posts_count', 'desc');
    }

    /**
     * Get post count attribute.
     */
    protected function postCount(): Attribute
    {
        return Attribute::make(
            get: fn () => Cache::tags(self::$cacheTags)
                ->remember(
                    "category_{$this->id}_post_count",
                    self::$cacheDuration,
                    fn () => $this->calculatePostCount()
                )
        );
    }

    /**
     * Calculate post count.
     */
    protected function calculatePostCount(): int
    {
        return $this->posts()->where('status', 'published')->count();
    }

    /**
     * Get total post count including descendants.
     */
    public function getTotalPostCountAttribute(): int
    {
        return Cache::tags(self::$cacheTags)
            ->remember(
                "category_{$this->id}_total_post_count",
                self::$cacheDuration,
                fn () => $this->calculateTotalPostCount()
            );
    }

    /**
     * Calculate total post count.
     */
    protected function calculateTotalPostCount(): int
    {
        $count = $this->post_count;
        
        foreach ($this->children as $child) {
            $count += $child->total_post_count;
        }
        
        return $count;
    }

    /**
     * Get breadcrumbs.
     */
    public function getBreadcrumbsAttribute(): Collection
    {
        return $this->ancestors->push($this)->map(function ($category) {
            return [
                'id' => $category->id,
                'name' => $category->name,
                'slug' => $category->slug,
                'url' => $category->url,
            ];
        });
    }

    /**
     * Get level (alias for depth).
     */
    public function getLevelAttribute(): int
    {
        return $this->depth;
    }

    /**
     * Check if category is a parent.
     */
    public function getIsParentAttribute(): bool
    {
        return $this->children()->exists();
    }

    /**
     * Check if category is a leaf.
     */
    public function getIsLeafAttribute(): bool
    {
        return !$this->is_parent;
    }

    /**
     * Get URL.
     */
    public function getUrlAttribute(): string
    {
        return route('categories.show', $this->full_path ?: $this->slug);
    }

    /**
     * Get admin URL.
     */
    public function getAdminUrlAttribute(): string
    {
        return route('admin.categories.edit', $this->id);
    }

    /**
     * Get full name with hierarchy.
     */
    public function getFullNameAttribute(): string
    {
        if (!$this->parent_id) {
            return $this->name;
        }

        $names = $this->ancestors->pluck('name')->push($this->name);
        return $names->implode(' › ');
    }

    /**
     * Get tree name with indentation.
     */
    public function getTreeNameAttribute(): string
    {
        $prefix = str_repeat('—', $this->depth);
        return $prefix . ($this->depth > 0 ? ' ' : '') . $this->name;
    }

    /**
     * Update paths.
     */
    protected function updatePaths(): void
    {
        if ($this->parent_id) {
            $parent = static::find($this->parent_id);
            $this->path = ($parent->path ? $parent->path . '/' : '') . $this->id;
            $this->full_path = ($parent->full_path ? $parent->full_path . '/' : '') . $this->slug;
        } else {
            $this->path = (string) $this->id;
            $this->full_path = $this->slug;
        }
    }

    /**
     * Update descendant paths.
     */
    protected function updateDescendantPaths(): void
    {
        $this->descendants()->each(function ($descendant) {
            $descendant->updatePaths();
            $descendant->saveQuietly();
        });
    }

    /**
     * Generate meta tags.
     */
    protected function generateMetaTags(): void
    {
        if (!$this->meta_title) {
            $this->meta_title = $this->name . ' | ' . config('app.name');
        }

        if (!$this->meta_description) {
            $this->meta_description = Str::limit(
                strip_tags($this->description ?? ''),
                160
            );
        }

        if (empty($this->meta_keywords)) {
            $this->meta_keywords = $this->extractKeywords();
        }

        $this->og_data = $this->generateOpenGraphData();
        $this->schema_data = $this->generateSchemaData();
    }

    /**
     * Generate Open Graph data.
     */
    protected function generateOpenGraphData(): array
    {
        return [
            'og:title' => $this->meta_title,
            'og:description' => $this->meta_description,
            'og:type' => 'website',
            'og:url' => $this->url,
            'og:image' => $this->image ?? $this->banner_image,
            'og:site_name' => config('app.name'),
            'article:section' => $this->name,
        ];
    }

    /**
     * Generate Schema.org data.
     */
    protected function generateSchemaData(): array
    {
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => 'CollectionPage',
            'name' => $this->name,
            'description' => $this->description,
            'url' => $this->url,
            'breadcrumb' => $this->generateBreadcrumbSchema(),
        ];

        if ($this->image) {
            $schema['image'] = $this->image;
        }

        return $schema;
    }

    /**
     * Generate breadcrumb schema.
     */
    protected function generateBreadcrumbSchema(): array
    {
        $items = [];
        $position = 1;

        foreach ($this->breadcrumbs as $breadcrumb) {
            $items[] = [
                '@type' => 'ListItem',
                'position' => $position++,
                'name' => $breadcrumb['name'],
                'item' => $breadcrumb['url'],
            ];
        }

        return [
            '@type' => 'BreadcrumbList',
            'itemListElement' => $items,
        ];
    }

    /**
     * Extract keywords from content.
     */
    protected function extractKeywords(): array
    {
        $text = $this->name . ' ' . strip_tags($this->description ?? '');
        $words = str_word_count(strtolower($text), 1);
        $stopWords = ['the', 'is', 'at', 'which', 'on', 'a', 'an', 'as', 'are', 'was', 'were'];
        
        $keywords = array_diff($words, $stopWords);
        $frequency = array_count_values($keywords);
        arsort($frequency);
        
        return array_keys(array_slice($frequency, 0, 10));
    }

    /**
     * Handle parent change.
     */
    protected function handleParentChange(): void
    {
        // Check for circular reference
        if ($this->parent_id) {
            $parent = static::find($this->parent_id);
            if ($parent && $parent->isDescendantOf($this)) {
                throw new \Exception('Cannot move category to its own descendant');
            }
        }

        // Update inherited settings if needed
        if ($this->inherit_parent_settings && $this->parent) {
            $this->inheritParentSettings();
        }
    }

    /**
     * Inherit settings from parent.
     */
    protected function inheritParentSettings(): void
    {
        $parent = $this->parent;
        
        $this->fill([
            'allow_posts' => $parent->allow_posts,
            'allow_subcategories' => $parent->allow_subcategories,
            'is_visible' => $parent->is_visible,
            'is_searchable' => $parent->is_searchable,
            'access_rules' => $parent->access_rules,
            'posting_rules' => $parent->posting_rules,
            'moderation_rules' => $parent->moderation_rules,
            'template' => $parent->template,
        ]);
    }

    /**
     * Check if category can be deleted.
     */
    public function canBeDeleted(): bool
    {
        if ($this->is_default) {
            return false;
        }

        if ($this->posts()->exists()) {
            return false;
        }

        if ($this->children()->exists()) {
            return false;
        }

        return true;
    }

    /**
     * Get deletion block reason.
     */
    public function getDeletionBlockReason(): string
    {
        if ($this->is_default) {
            return 'Default category cannot be deleted';
        }

        if ($this->posts()->exists()) {
            return 'Category has associated posts';
        }

        if ($this->children()->exists()) {
            return 'Category has subcategories';
        }

        return '';
    }

    /**
     * Handle deletion process.
     */
    protected function handleDeletion(): void
    {
        // Move posts to default category
        $defaultCategory = static::where('is_default', true)->first();
        
        if ($defaultCategory && $this->posts()->exists()) {
            $this->posts()->update(['category_id' => $defaultCategory->id]);
        }

        // Handle orphaned children
        if ($this->children()->exists()) {
            $this->children()->update(['parent_id' => $this->parent_id]);
        }
    }

    /**
     * Update ancestor counts.
     */
    protected function updateAncestorCounts(): void
    {
        $this->ancestors->each(function ($ancestor) {
            $ancestor->recalculateCounts();
        });
    }

    /**
     * Recalculate all counts.
     */
    public function recalculateCounts(): void
    {
        $this->direct_post_count = $this->posts()->count();
        $this->published_post_count = $this->publishedPosts()->count();
        $this->total_post_count = $this->calculateTotalPostCount();
        $this->saveQuietly();
        
        $this->clearCache("category_{$this->id}_post_count");
        $this->clearCache("category_{$this->id}_total_post_count");
    }

    /**
     * Clear all caches.
     */
    protected function clearAllCaches(): void
    {
        Cache::tags(self::$cacheTags)->flush();
    }

    /**
     * Clear specific cache.
     */
    protected function clearCache(string $key): void
    {
        Cache::tags(self::$cacheTags)->forget($key);
    }

    /**
     * Merge with another category.
     */
    public function mergeWith(Category $target): void
    {
        DB::transaction(function () use ($target) {
            // Move posts
            $this->posts()->each(function ($post) use ($target) {
                $post->categories()->syncWithoutDetaching([$target->id]);
                $post->categories()->detach($this->id);
            });

            // Move children
            $this->children()->update(['parent_id' => $target->id]);

            // Move subscribers
            $this->subscribers()->each(function ($subscriber) use ($target) {
                $target->subscribers()->syncWithoutDetaching($subscriber->id);
            });

            // Record merge in history
            $target->history()->create([
                'action' => 'merge',
                'data' => [
                    'merged_from' => $this->toArray(),
                    'merged_at' => now(),
                ],
                'user_id' => auth()->id(),
            ]);

            event(new CategoryMerged($this, $target));

            // Delete source category
            $this->delete();
        });
    }

    /**
     * Calculate engagement score.
     */
    public function calculateEngagementScore(): float
    {
        $views = max(1, $this->view_count);
        $clicks = $this->click_count;
        $posts = $this->published_post_count;
        $avgRating = $this->avg_post_rating ?? 0;
        
        $score = (($clicks / $views) * 40) +
                 (min($posts / 10, 1) * 30) +
                 (($avgRating / 5) * 30);
        
        return round(min(100, $score), 2);
    }

    /**
     * Calculate popularity score.
     */
    public function calculatePopularityScore(): float
    {
        $recentViews = $this->metrics()
            ->where('recorded_at', '>=', now()->subDays(30))
            ->sum('views');
        
        $recentPosts = $this->posts()
            ->where('created_at', '>=', now()->subDays(30))
            ->count();
        
        $engagement = $this->calculateEngagementScore();
        
        $score = ($recentViews * 0.3) +
                 ($recentPosts * 10) +
                 ($engagement * 0.4) +
                 ($this->subscriber_count ?? 0);
        
        return round(min(100, $score / 100), 2);
    }

    /**
     * Get navigation tree.
     */
    public static function getNavigationTree(): Collection
    {
        return Cache::tags(self::$cacheTags)
            ->remember('navigation_tree', self::$cacheDuration, function () {
                return static::navigational()
                    ->withDepth()
                    ->defaultOrder()
                    ->get()
                    ->toTree();
            });
    }

    /**
     * Get category tree for admin.
     */
    public static function getAdminTree(): Collection
    {
        return static::withDepth()
            ->defaultOrder()
            ->withCount(['posts', 'children'])
            ->get()
            ->toTree();
    }
}