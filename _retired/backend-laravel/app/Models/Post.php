<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphOne;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Post Model
 * 
 * Comprehensive blog/CMS post with advanced SEO, analytics, versioning,
 * and content management capabilities.
 *
 * @property int $id
 * @property int $author_id Post author
 * @property string $title Post title
 * @property string $slug URL-friendly slug
 * @property string|null $excerpt Short summary
 * @property array $content_blocks Structured content blocks
 * @property string|null $content_html Rendered HTML content (cached)
 * @property string|null $featured_image Main featured image
 * @property string|null $og_image Open Graph image
 * @property string|null $twitter_card Twitter card type
 * @property string $status Post status (draft, published, scheduled, archived, private)
 * @property \Illuminate\Support\Carbon|null $published_at Publication timestamp
 * @property \Illuminate\Support\Carbon|null $scheduled_at Scheduled publication time
 * @property bool $auto_publish Auto-publish when scheduled_at reached
 * @property string|null $meta_title SEO title
 * @property string|null $meta_description SEO description
 * @property bool $indexable Allow search engine indexing
 * @property string|null $canonical_url Canonical URL
 * @property array $schema_markup Schema.org structured data
 * @property array $categories Post categories
 * @property array $tags Post tags
 * @property array $keywords SEO keywords
 * @property int|null $reading_time Estimated reading time (minutes)
 * @property array $related_posts Related post IDs
 * @property array $custom_fields Custom metadata
 * @property int $view_count Total views
 * @property int $unique_view_count Unique visitor views
 * @property int $comment_count Total comments
 * @property int $share_count Social shares
 * @property int $like_count Likes/reactions
 * @property float $engagement_rate Engagement percentage
 * @property float $ctr Click-through rate
 * @property float $avg_time_on_page Average time spent (seconds)
 * @property float $bounce_rate Bounce rate percentage
 * @property int $seo_score SEO quality score (0-100)
 * @property int $word_count Total word count
 * @property int $readability_score Readability score (0-100)
 * @property bool $is_featured Featured post flag
 * @property bool $is_pinned Pinned to top
 * @property bool $is_evergreen Evergreen content
 * @property bool $allow_comments Comments enabled
 * @property string $visibility Visibility level (public, private, password, members)
 * @property string|null $password Password for protected posts
 * @property int $version Content version number
 * @property \Illuminate\Support\Carbon|null $last_edited_at Last edit timestamp
 * @property int|null $last_edited_by User ID of last editor
 * @property string|null $locale Content locale (en, es, etc.)
 * @property int|null $parent_id Parent post (for translations)
 * @property string|null $template Template/layout name
 * @property array $content_warnings Content warnings/advisories
 * @property int $revision_count Number of revisions
 * @property \Illuminate\Support\Carbon|null $last_seo_check_at Last SEO analysis date
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\User $author Post author
 * @property-read \App\Models\User|null $lastEditor Last editor
 * @property-read \Illuminate\Database\Eloquent\Collection $media Attached media
 * @property-read \Illuminate\Database\Eloquent\Collection $comments Post comments
 * @property-read \Illuminate\Database\Eloquent\Collection $revisions Post revisions
 * @property-read \App\Models\SeoAnalysis|null $seoAnalysis SEO analysis data
 */
class Post extends Model
{
    use HasFactory;

    /**
     * Post statuses
     */
    public const STATUS_DRAFT = 'draft';
    public const STATUS_PUBLISHED = 'published';
    public const STATUS_SCHEDULED = 'scheduled';
    public const STATUS_ARCHIVED = 'archived';
    public const STATUS_PRIVATE = 'private';
    public const STATUS_PENDING_REVIEW = 'pending_review';
    public const STATUS_REJECTED = 'rejected';

    /**
     * Visibility levels
     */
    public const VISIBILITY_PUBLIC = 'public';
    public const VISIBILITY_PRIVATE = 'private';
    public const VISIBILITY_PASSWORD = 'password';
    public const VISIBILITY_MEMBERS = 'members';

    /**
     * Twitter card types
     */
    public const TWITTER_CARD_SUMMARY = 'summary';
    public const TWITTER_CARD_SUMMARY_LARGE = 'summary_large_image';
    public const TWITTER_CARD_APP = 'app';
    public const TWITTER_CARD_PLAYER = 'player';

    /**
     * Valid statuses
     */
    public const VALID_STATUSES = [
        self::STATUS_DRAFT,
        self::STATUS_PUBLISHED,
        self::STATUS_SCHEDULED,
        self::STATUS_ARCHIVED,
        self::STATUS_PRIVATE,
        self::STATUS_PENDING_REVIEW,
        self::STATUS_REJECTED,
    ];

    /**
     * Valid visibility levels
     */
    public const VALID_VISIBILITIES = [
        self::VISIBILITY_PUBLIC,
        self::VISIBILITY_PRIVATE,
        self::VISIBILITY_PASSWORD,
        self::VISIBILITY_MEMBERS,
    ];

    /**
     * Reading speed (words per minute)
     */
    public const READING_SPEED_WPM = 200;

    /**
     * Cache TTL
     */
    public const CACHE_TTL = 3600; // 1 hour

    protected $fillable = [
        'author_id',
        'title',
        'slug',
        'excerpt',
        'content_blocks',
        'content_html',
        'featured_image',
        'featured_image_alt',
        'featured_image_title',
        'featured_image_caption',
        'featured_image_description',
        'featured_media_id',
        'og_image',
        'twitter_card',
        'status',
        'published_at',
        'scheduled_at',
        'auto_publish',
        'meta_title',
        'meta_description',
        'indexable',
        'canonical_url',
        'schema_markup',
        'categories',
        'tags',
        'keywords',
        'reading_time',
        'related_posts',
        'custom_fields',
        'view_count',
        'unique_view_count',
        'comment_count',
        'share_count',
        'like_count',
        'engagement_rate',
        'ctr',
        'avg_time_on_page',
        'bounce_rate',
        'seo_score',
        'word_count',
        'readability_score',
        'is_featured',
        'is_pinned',
        'is_evergreen',
        'allow_comments',
        'visibility',
        'password',
        'version',
        'last_edited_at',
        'last_edited_by',
        'locale',
        'parent_id',
        'template',
        'content_warnings',
        'revision_count',
        'last_seo_check_at',
    ];

    protected $casts = [
        'content_blocks' => 'array',
        'schema_markup' => 'array',
        'categories' => 'array',
        'tags' => 'array',
        'keywords' => 'array',
        'related_posts' => 'array',
        'custom_fields' => 'array',
        'content_warnings' => 'array',
        'indexable' => 'boolean',
        'is_featured' => 'boolean',
        'is_pinned' => 'boolean',
        'is_evergreen' => 'boolean',
        'allow_comments' => 'boolean',
        'auto_publish' => 'boolean',
        'published_at' => 'datetime',
        'scheduled_at' => 'datetime',
        'last_edited_at' => 'datetime',
        'last_seo_check_at' => 'datetime',
        'view_count' => 'integer',
        'unique_view_count' => 'integer',
        'comment_count' => 'integer',
        'share_count' => 'integer',
        'like_count' => 'integer',
        'word_count' => 'integer',
        'reading_time' => 'integer',
        'version' => 'integer',
        'revision_count' => 'integer',
        'engagement_rate' => 'decimal:2',
        'ctr' => 'decimal:2',
        'avg_time_on_page' => 'decimal:2',
        'bounce_rate' => 'decimal:2',
        'seo_score' => 'decimal:2',
        'readability_score' => 'decimal:2',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => self::STATUS_DRAFT,
        'visibility' => self::VISIBILITY_PUBLIC,
        'twitter_card' => self::TWITTER_CARD_SUMMARY_LARGE,
        'indexable' => true,
        'is_featured' => false,
        'is_pinned' => false,
        'is_evergreen' => false,
        'allow_comments' => true,
        'auto_publish' => false,
        'view_count' => 0,
        'unique_view_count' => 0,
        'comment_count' => 0,
        'share_count' => 0,
        'like_count' => 0,
        'word_count' => 0,
        'version' => 1,
        'revision_count' => 0,
        'engagement_rate' => 0.0,
        'ctr' => 0.0,
        'avg_time_on_page' => 0.0,
        'bounce_rate' => 0.0,
        'seo_score' => 0.0,
        'readability_score' => 0.0,
        'locale' => 'en',
        'content_blocks' => '[]',
        'schema_markup' => '[]',
        'categories' => '[]',
        'tags' => '[]',
        'keywords' => '[]',
        'related_posts' => '[]',
        'custom_fields' => '[]',
        'content_warnings' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->generateSlugIfMissing();
            $model->validateStatus();
            $model->calculateWordCount();
            $model->calculateReadingTime();
            $model->renderContentHtml();
            
            if ($model->status === self::STATUS_PUBLISHED && !$model->published_at) {
                $model->published_at = now();
            }
        });

        static::updating(function (self $model): void {
            $model->validateStatus();
            
            if ($model->isDirty('content_blocks')) {
                $model->calculateWordCount();
                $model->calculateReadingTime();
                $model->renderContentHtml();
                $model->incrementVersion();
            }
            
            if ($model->isDirty('status') && $model->status === self::STATUS_PUBLISHED && !$model->published_at) {
                $model->published_at = now();
            }
            
            $model->last_edited_at = now();
            $model->last_edited_by = auth()->id();
        });

        static::saved(function (self $model): void {
            $model->clearPostCache();
            $model->updateSchemaMarkup();
        });

        static::deleted(function (self $model): void {
            $model->clearPostCache();
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
     * Get post author
     */
    public function author(): BelongsTo
    {
        return $this->belongsTo(User::class, 'author_id');
    }

    /**
     * Get last editor
     */
    public function lastEditor(): BelongsTo
    {
        return $this->belongsTo(User::class, 'last_edited_by');
    }

    /**
     * Get parent post (for translations)
     */
    public function parent(): BelongsTo
    {
        return $this->belongsTo(Post::class, 'parent_id');
    }

    /**
     * Get child posts (translations)
     */
    public function translations(): HasMany
    {
        return $this->hasMany(Post::class, 'parent_id');
    }

    /**
     * Get attached media
     */
    public function media(): BelongsToMany
    {
        return $this->belongsToMany(Media::class, 'post_media')
            ->withPivot(['order', 'caption', 'alt_text'])
            ->withTimestamps()
            ->orderBy('post_media.order');
    }

    /**
     * Get post comments
     */
    public function comments(): HasMany
    {
        return $this->hasMany(Comment::class)->whereNull('parent_id');
    }

    /**
     * Get all comments (including replies)
     */
    public function allComments(): HasMany
    {
        return $this->hasMany(Comment::class);
    }

    /**
     * Get post revisions
     */
    public function revisions(): HasMany
    {
        return $this->hasMany(PostRevision::class)->orderByDesc('created_at');
    }

    /**
     * Get SEO analysis
     */
    public function seoAnalysis(): MorphOne
    {
        return $this->morphOne(SeoAnalysis::class, 'analyzable');
    }

    /**
     * Get analytics events
     */
    public function analytics(): MorphMany
    {
        return $this->morphMany(AnalyticsEvent::class, 'trackable');
    }

    /**
     * Generate slug from title if missing
     */
    protected function generateSlugIfMissing(): void
    {
        if (empty($this->slug)) {
            $this->slug = Str::slug($this->title);
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
     * Validate post status
     */
    protected function validateStatus(): void
    {
        if (!in_array($this->status, self::VALID_STATUSES, true)) {
            throw new InvalidArgumentException(
                sprintf('Invalid post status: %s', $this->status)
            );
        }
    }

    /**
     * Calculate word count from content blocks
     */
    protected function calculateWordCount(): void
    {
        $text = '';
        
        // Handle content_blocks if present
        if (is_array($this->content_blocks)) {
            foreach ($this->content_blocks as $block) {
                if (isset($block['content'])) {
                    $content = is_string($block['content']) ? $block['content'] : '';
                    $text .= ' ' . strip_tags($content);
                }
            }
        }
        
        // Also count words from main content field
        if (!empty($this->content)) {
            $text .= ' ' . strip_tags($this->content);
        }
        
        $this->word_count = str_word_count($text);
    }

    /**
     * Calculate estimated reading time
     */
    protected function calculateReadingTime(): void
    {
        $this->reading_time = (int) ceil($this->word_count / self::READING_SPEED_WPM);
    }

    /**
     * Render content blocks to HTML (cached)
     */
    protected function renderContentHtml(): void
    {
        $html = '';
        
        if (is_array($this->content_blocks)) {
            foreach ($this->content_blocks as $block) {
                $html .= $this->renderBlock($block);
            }
        } elseif (!empty($this->content)) {
            $html = $this->content;
        }
        
        $this->content_html = $html;
    }
        
        

    /**
     * Render a single content block
     */
    protected function renderBlock(array $block): string
    {
        $type = $block['type'] ?? 'paragraph';
        $content = $block['content'] ?? '';

        // Sanitize content to prevent XSS attacks
        $safeContent = e($content);
        $safeLevel = (int) ($block['level'] ?? 2);
        $safeAlt = e($block['alt'] ?? '');
        $safeLanguage = e($block['language'] ?? 'plaintext');

        // For image src, validate it's a proper URL
        $safeSrc = filter_var($content, FILTER_VALIDATE_URL) ? e($content) : '';

        return match($type) {
            'heading' => "<h{$safeLevel}>{$safeContent}</h{$safeLevel}>",
            'paragraph' => "<p>{$safeContent}</p>",
            'image' => $safeSrc ? "<img src=\"{$safeSrc}\" alt=\"{$safeAlt}\" loading=\"lazy\">" : '',
            'code' => "<pre><code class=\"language-{$safeLanguage}\">{$safeContent}</code></pre>",
            'quote' => "<blockquote>{$safeContent}</blockquote>",
            'list' => $this->renderList($block),
            default => "<div>{$safeContent}</div>",
        };
    }

    /**
     * Render list block
     */
    protected function renderList(array $block): string
    {
        $tag = $block['ordered'] ? 'ol' : 'ul';
        // Escape each list item to prevent XSS
        $items = array_map(fn($item) => "<li>" . e($item) . "</li>", $block['items'] ?? []);
        return "<{$tag}>" . implode('', $items) . "</{$tag}>";
    }

    /**
     * Increment version number
     */
    protected function incrementVersion(): void
    {
        $this->version++;
        $this->revision_count++;
    }

    /**
     * Update Schema.org structured data
     */
    protected function updateSchemaMarkup(): void
    {
        $schema = [
            '@context' => 'https://schema.org',
            '@type' => 'BlogPosting',
            'headline' => $this->title,
            'description' => $this->excerpt ?? $this->meta_description,
            'datePublished' => $this->published_at?->toIso8601String(),
            'dateModified' => $this->updated_at->toIso8601String(),
            'author' => [
                '@type' => 'Person',
                'name' => $this->author->name ?? 'Unknown',
            ],
            'publisher' => [
                '@type' => 'Organization',
                'name' => config('app.name'),
                'logo' => [
                    '@type' => 'ImageObject',
                    'url' => url('/logo.png'),
                ],
            ],
            'mainEntityOfPage' => [
                '@type' => 'WebPage',
                '@id' => $this->getPublicUrl(),
            ],
        ];

        if ($this->featured_image) {
            $schema['image'] = url($this->featured_image);
        }

        if ($this->word_count) {
            $schema['wordCount'] = $this->word_count;
        }

        if (!empty($this->keywords)) {
            $schema['keywords'] = implode(', ', $this->keywords);
        }

        $this->updateQuietly(['schema_markup' => $schema]);
    }

    /**
     * Clear post cache
     */
    protected function clearPostCache(): void
    {
        Cache::tags(['posts'])->flush();
        Cache::forget("post:slug:{$this->slug}");
        Cache::forget("post:id:{$this->id}");
    }

    /**
     * Check if post is published
     */
    public function isPublished(): bool
    {
        return $this->status === self::STATUS_PUBLISHED
            && $this->published_at
            && $this->published_at->isPast();
    }

    /**
     * Check if post is draft
     */
    public function isDraft(): bool
    {
        return $this->status === self::STATUS_DRAFT;
    }

    /**
     * Check if post is scheduled
     */
    public function isScheduled(): bool
    {
        return $this->status === self::STATUS_SCHEDULED
            && $this->scheduled_at
            && $this->scheduled_at->isFuture();
    }

    /**
     * Check if post is archived
     */
    public function isArchived(): bool
    {
        return $this->status === self::STATUS_ARCHIVED;
    }

    /**
     * Check if post is password protected
     */
    public function isPasswordProtected(): bool
    {
        return $this->visibility === self::VISIBILITY_PASSWORD && !empty($this->password);
    }

    /**
     * Check if user can view post
     */
    public function canBeViewedBy(?User $user = null): bool
    {
        if (!$this->isPublished()) {
            return $user && ($user->id === $this->author_id || $user->isAdmin());
        }

        return match($this->visibility) {
            self::VISIBILITY_PUBLIC => true,
            self::VISIBILITY_PRIVATE => $user && ($user->id === $this->author_id || $user->isAdmin()),
            self::VISIBILITY_MEMBERS => $user !== null,
            self::VISIBILITY_PASSWORD => false, // Requires password check
            default => false,
        };
    }

    /**
     * Verify password for protected post
     */
    public function verifyPassword(string $password): bool
    {
        return $this->isPasswordProtected() && hash_equals($this->password, $password);
    }

    /**
     * Publish the post
     */
    public function publish(): self
    {
        $this->update([
            'status' => self::STATUS_PUBLISHED,
            'published_at' => now(),
        ]);

        return $this;
    }

    /**
     * Unpublish the post
     */
    public function unpublish(): self
    {
        $this->update([
            'status' => self::STATUS_DRAFT,
            'published_at' => null,
        ]);

        return $this;
    }

    /**
     * Archive the post
     */
    public function archive(): self
    {
        $this->update(['status' => self::STATUS_ARCHIVED]);
        return $this;
    }

    /**
     * Schedule publication
     */
    public function schedule(\DateTimeInterface $date): self
    {
        $this->update([
            'status' => self::STATUS_SCHEDULED,
            'scheduled_at' => $date,
            'auto_publish' => true,
        ]);

        return $this;
    }

    /**
     * Increment view count
     */
    public function incrementViews(bool $unique = false): self
    {
        $this->increment('view_count');
        
        if ($unique) {
            $this->increment('unique_view_count');
        }

        return $this;
    }

    /**
     * Increment share count
     */
    public function incrementShares(): self
    {
        $this->increment('share_count');
        return $this;
    }

    /**
     * Increment like count
     */
    public function incrementLikes(): self
    {
        $this->increment('like_count');
        return $this;
    }

    /**
     * Decrement like count
     */
    public function decrementLikes(): self
    {
        $this->decrement('like_count');
        return $this;
    }

    /**
     * Update engagement metrics
     */
    public function updateEngagementMetrics(array $data): self
    {
        $updates = [];

        if (isset($data['avg_time_on_page'])) {
            $updates['avg_time_on_page'] = $data['avg_time_on_page'];
        }

        if (isset($data['bounce_rate'])) {
            $updates['bounce_rate'] = $data['bounce_rate'];
        }

        if (isset($data['ctr'])) {
            $updates['ctr'] = $data['ctr'];
        }

        // Calculate engagement rate
        if ($this->view_count > 0) {
            $engagementActions = $this->comment_count + $this->share_count + $this->like_count;
            $updates['engagement_rate'] = round(($engagementActions / $this->view_count) * 100, 2);
        }

        if (!empty($updates)) {
            $this->update($updates);
        }

        return $this;
    }

    /**
     * Create revision snapshot
     */
    public function createRevision(?string $reason = null): void
    {
        PostRevision::create([
            'post_id' => $this->id,
            'title' => $this->title,
            'content_blocks' => $this->content_blocks,
            'version' => $this->version,
            'created_by' => auth()->id(),
            'reason' => $reason,
        ]);
    }

    /**
     * Restore from revision
     */
    public function restoreFromRevision(PostRevision $revision): self
    {
        $this->update([
            'title' => $revision->title,
            'content_blocks' => $revision->content_blocks,
        ]);

        return $this;
    }

    /**
     * Get public URL
     */
    public function getPublicUrl(): string
    {
        return url("/blog/{$this->slug}");
    }

    /**
     * Get edit URL
     */
    public function getEditUrl(): string
    {
        return url("/admin/posts/{$this->id}/edit");
    }

    /**
     * Get reading time formatted
     */
    public function getReadingTimeFormattedAttribute(): string
    {
        $minutes = $this->reading_time;
        
        if ($minutes < 1) {
            return '< 1 min read';
        }
        
        return $minutes === 1 ? '1 min read' : "{$minutes} mins read";
    }

    /**
     * Get excerpt or generate from content
     */
    public function getExcerptOrGeneratedAttribute(): string
    {
        if ($this->excerpt) {
            return $this->excerpt;
        }

        return Str::limit(strip_tags($this->content_html), 160);
    }

    /**
     * Get performance score
     */
    public function getPerformanceScoreAttribute(): float
    {
        $scores = [
            $this->seo_score,
            $this->readability_score,
            min($this->engagement_rate * 10, 100), // Scale engagement
        ];

        $scores = array_filter($scores, fn($score) => $score > 0);

        return $scores ? round(array_sum($scores) / count($scores), 2) : 0.0;
    }

    /**
     * Get status color for UI
     */
    public function getStatusColorAttribute(): string
    {
        return match($this->status) {
            self::STATUS_PUBLISHED => '#22c55e', // green-500
            self::STATUS_DRAFT => '#6b7280', // gray-500
            self::STATUS_SCHEDULED => '#3b82f6', // blue-500
            self::STATUS_PENDING_REVIEW => '#f59e0b', // amber-500
            self::STATUS_ARCHIVED => '#9ca3af', // gray-400
            self::STATUS_REJECTED => '#ef4444', // red-500
            default => '#6b7280',
        };
    }

    /**
     * Get custom field value
     */
    public function getCustomField(string $key, mixed $default = null): mixed
    {
        return $this->custom_fields[$key] ?? $default;
    }

    /**
     * Set custom field value
     */
    public function setCustomField(string $key, mixed $value): self
    {
        $fields = $this->custom_fields;
        $fields[$key] = $value;
        $this->custom_fields = $fields;

        return $this;
    }

    /**
     * Scope: Published posts only
     */
    public function scopePublished(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_PUBLISHED)
            ->whereNotNull('published_at')
            ->where('published_at', '<=', now());
    }

    /**
     * Scope: Draft posts
     */
    public function scopeDraft(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_DRAFT);
    }

    /**
     * Scope: Scheduled posts
     */
    public function scopeScheduled(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_SCHEDULED)
            ->whereNotNull('scheduled_at')
            ->where('scheduled_at', '>', now());
    }

    /**
     * Scope: Posts ready to auto-publish
     */
    public function scopeReadyToPublish(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_SCHEDULED)
            ->where('auto_publish', true)
            ->whereNotNull('scheduled_at')
            ->where('scheduled_at', '<=', now());
    }

    /**
     * Scope: Featured posts
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true);
    }

    /**
     * Scope: Pinned posts
     */
    public function scopePinned(Builder $query): Builder
    {
        return $query->where('is_pinned', true);
    }

    /**
     * Scope: Evergreen content
     */
    public function scopeEvergreen(Builder $query): Builder
    {
        return $query->where('is_evergreen', true);
    }

    /**
     * Scope: Filter by author
     */
    public function scopeByAuthor(Builder $query, int $authorId): Builder
    {
        return $query->where('author_id', $authorId);
    }

    /**
     * Scope: Filter by category
     */
    public function scopeInCategory(Builder $query, string $category): Builder
    {
        return $query->whereJsonContains('categories', $category);
    }

    /**
     * Scope: Filter by tag
     */
    public function scopeWithTag(Builder $query, string $tag): Builder
    {
        return $query->whereJsonContains('tags', $tag);
    }

    /**
     * Scope: Filter by locale
     */
    public function scopeInLocale(Builder $query, string $locale): Builder
    {
        return $query->where('locale', $locale);
    }

    /**
     * Scope: Popular posts (by views)
     */
    public function scopePopular(Builder $query, int $minViews = 100): Builder
    {
        return $query->where('view_count', '>=', $minViews);
    }

    /**
     * Scope: Trending posts (recent engagement)
     */
    public function scopeTrending(Builder $query, int $days = 7): Builder
    {
        return $query->where('published_at', '>=', now()->subDays($days))
            ->orderByDesc('view_count');
    }

    /**
     * Scope: High engagement posts
     */
    public function scopeHighEngagement(Builder $query, float $minRate = 5.0): Builder
    {
        return $query->where('engagement_rate', '>=', $minRate);
    }

    /**
     * Scope: Recently published
     */
    public function scopeRecent(Builder $query, int $days = 30): Builder
    {
        return $query->where('published_at', '>=', now()->subDays($days));
    }

    /**
     * Scope: Needs SEO attention
     */
    public function scopeNeedsSeoAttention(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->whereNull('meta_title')
              ->orWhereNull('meta_description')
              ->orWhere('seo_score', '<', 70)
              ->orWhere(function($sq) {
                  $sq->whereNull('last_seo_check_at')
                    ->orWhere('last_seo_check_at', '<', now()->subDays(30));
              });
        });
    }

    /**
     * Scope: Order by popularity
     */
    public function scopeOrderByPopularity(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('view_count', $direction);
    }

    /**
     * Scope: Order by engagement
     */
    public function scopeOrderByEngagement(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('engagement_rate', $direction);
    }

    /**
     * Scope: Order by publication date
     */
    public function scopeOrderByPublished(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('published_at', $direction);
    }

    /**
     * Static: Search posts
     */
    public static function search(string $query): Builder
    {
        return static::where(function($q) use ($query) {
            $q->where('title', 'LIKE', "%{$query}%")
              ->orWhere('excerpt', 'LIKE', "%{$query}%")
              ->orWhere('content_html', 'LIKE', "%{$query}%")
              ->orWhereJsonContains('tags', $query)
              ->orWhereJsonContains('keywords', $query);
        });
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total' => static::count(),
            'published' => static::published()->count(),
            'draft' => static::draft()->count(),
            'scheduled' => static::scheduled()->count(),
            'featured' => static::featured()->count(),
            'total_views' => static::sum('view_count'),
            'total_comments' => static::sum('comment_count'),
            'total_shares' => static::sum('share_count'),
            'avg_engagement_rate' => round(static::where('engagement_rate', '>', 0)->avg('engagement_rate'), 2),
            'avg_seo_score' => round(static::where('seo_score', '>', 0)->avg('seo_score'), 2),
            'avg_readability' => round(static::where('readability_score', '>', 0)->avg('readability_score'), 2),
        ];
    }

    /**
     * Static: Get trending posts
     */
    public static function getTrending(int $days = 7, int $limit = 10): Collection
    {
        return static::published()
            ->trending($days)
            ->limit($limit)
            ->get();
    }

    /**
     * Static: Get posts needing attention
     */
    public static function getNeedingAttention(): Collection
    {
        return static::needsSeoAttention()
            ->published()
            ->orderByDesc('view_count')
            ->get();
    }

    /**
     * Static: Process scheduled posts
     */
    public static function processScheduled(): int
    {
        $posts = static::readyToPublish()->get();
        
        foreach ($posts as $post) {
            $post->publish();
        }

        return $posts->count();
    }

    /**
     * Export to array for API
     */
    public function toPostArray(bool $includeContent = true): array
    {
        $data = [
            'id' => $this->id,
            'title' => $this->title,
            'slug' => $this->slug,
            'excerpt' => $this->excerpt_or_generated,
            'status' => $this->status,
            'status_color' => $this->status_color,
            'featured_image' => $this->featured_image ? url($this->featured_image) : null,
            'author' => [
                'id' => $this->author->id,
                'name' => $this->author->name,
                'avatar' => $this->author->avatar ?? null,
            ],
            'categories' => $this->categories,
            'tags' => $this->tags,
            'reading_time' => $this->reading_time_formatted,
            'word_count' => $this->word_count,
            'is_featured' => $this->is_featured,
            'is_pinned' => $this->is_pinned,
            'metrics' => [
                'views' => $this->view_count,
                'comments' => $this->comment_count,
                'shares' => $this->share_count,
                'likes' => $this->like_count,
                'engagement_rate' => $this->engagement_rate,
            ],
            'seo' => [
                'score' => $this->seo_score,
                'readability_score' => $this->readability_score,
                'performance_score' => $this->performance_score,
            ],
            'url' => $this->getPublicUrl(),
            'published_at' => $this->published_at?->toIso8601String(),
            'created_at' => $this->created_at->toIso8601String(),
            'updated_at' => $this->updated_at->toIso8601String(),
        ];

        if ($includeContent) {
            $data['content_blocks'] = $this->content_blocks;
            $data['content_html'] = $this->content_html;
        }

        return $data;
    }
}