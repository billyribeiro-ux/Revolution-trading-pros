<?php

declare(strict_types=1);

namespace App\Http\Resources;

use Carbon\Carbon;
use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;
use App\Services\ContentAnalyzer;
use App\Services\SEOAnalyzer;

/**
 * Post API Resource
 * 
 * Enterprise-grade blog post resource with SEO optimization, content analytics,
 * engagement tracking, and advanced content management features.
 * 
 * @property-read int $id
 * @property-read string $title
 * @property-read string $slug
 * @property-read ?string $excerpt
 * @property-read array $content_blocks
 * @property-read ?string $featured_image
 * @property-read string $status
 * @property-read ?Carbon $published_at
 * @property-read int $reading_time
 * @property-read int $view_count
 * @property-read array $categories
 * @property-read array $tags
 * @property-read ?string $meta_title
 * @property-read ?string $meta_description
 * @property-read Carbon $created_at
 * @property-read Carbon $updated_at
 * @property-read \App\Models\User $author
 * @property-read \Illuminate\Support\Collection $comments
 * @property-read \Illuminate\Support\Collection $reactions
 * @property-read \Illuminate\Support\Collection $bookmarks
 * @property-read \Illuminate\Support\Collection $revisions
 * 
 * @mixin \App\Models\Post
 */
class PostResource extends JsonResource
{
    /**
     * Cache TTL in seconds (10 minutes for published, 1 minute for drafts)
     */
    protected const CACHE_TTL_PUBLISHED = 600;
    protected const CACHE_TTL_DRAFT = 60;

    /**
     * Content analyzer service
     */
    protected static ?ContentAnalyzer $contentAnalyzer = null;

    /**
     * SEO analyzer service
     */
    protected static ?SEOAnalyzer $seoAnalyzer = null;

    /**
     * Transform the resource into an array.
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    public function toArray(Request $request): array
    {
        return [
            'id' => $this->id,
            'title' => $this->title,
            'slug' => $this->slug,
            'excerpt' => $this->excerpt,
            'content_html' => $this->when(
                $request->input('include_content', false),
                $this->content_html
            ),
            'featured_image' => $this->featured_image,
            'status' => $this->status,
            'published_at' => $this->published_at?->toIso8601String(),
            'created_at' => $this->created_at?->toIso8601String(),
            'updated_at' => $this->updated_at?->toIso8601String(),
            'author' => [
                'id' => $this->author_id,
                'name' => $this->author?->name,
            ],
            'meta' => [
                'reading_time' => $this->reading_time,
                'view_count' => $this->view_count ?? 0,
                'word_count' => $this->word_count,
            ],
            'seo' => [
                'meta_title' => $this->meta_title,
                'meta_description' => $this->meta_description,
            ],
        ];
    }

    /**
     * Get computed/calculated fields for the post.
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    protected function getComputedFields(Request $request): array
    {
        $content = strip_tags($this->content_html ?? '');
        
        return [
            'word_count' => $this->word_count ?? str_word_count($content),
            'reading_time' => $this->reading_time ?? ceil(str_word_count($content) / 200),
            'view_count' => $this->view_count ?? 0,
        ];
    }

    /**
     * Get content structure
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    protected function getContentStructure(Request $request): array
    {
        return [
            'excerpt' => $this->excerpt ?? $this->generateExcerpt(),
            'summary' => $this->summary ?? $this->generateSummary(),
            'table_of_contents' => $this->generateTableOfContents(),
            'highlights' => $this->content_highlights ?? [],
            'key_takeaways' => $this->key_takeaways ?? [],
            'formatted_content' => $this->when(
                $request->input('format') === 'html',
                fn() => $this->getHTMLContent()
            ),
            'markdown_content' => $this->when(
                $request->input('format') === 'markdown',
                fn() => $this->getMarkdownContent()
            ),
            'amp_content' => $this->when(
                $request->input('format') === 'amp',
                fn() => $this->getAMPContent()
            ),
        ];
    }

    /**
     * Get media structure
     *
     * @return array<string, mixed>
     */
    protected function getMediaStructure(): array
    {
        return [
            'featured_image' => $this->getFeaturedImageStructure(),
            'gallery' => $this->when(
                $this->relationLoaded('gallery'),
                fn() => MediaResource::collection($this->gallery)
            ),
            'videos' => $this->when(
                $this->relationLoaded('videos'),
                fn() => VideoResource::collection($this->videos)
            ),
            'attachments' => $this->when(
                $this->relationLoaded('attachments'),
                fn() => AttachmentResource::collection($this->attachments)
            ),
            'thumbnail_sizes' => $this->getThumbnailSizes(),
        ];
    }

    /**
     * Get featured image structure with responsive sizes
     *
     * @return array<string, mixed>|null
     */
    protected function getFeaturedImageStructure(): ?array
    {
        if (!$this->featured_image) {
            return null;
        }

        return [
            'url' => $this->featured_image,
            'alt' => $this->featured_image_alt ?? $this->title,
            'caption' => $this->featured_image_caption,
            'credits' => $this->featured_image_credits,
            'sizes' => [
                'thumbnail' => $this->getFeaturedImageUrl('thumbnail'),
                'medium' => $this->getFeaturedImageUrl('medium'),
                'large' => $this->getFeaturedImageUrl('large'),
                'full' => $this->featured_image,
            ],
            'srcset' => $this->generateSrcSet(),
            'dimensions' => [
                'width' => $this->featured_image_width,
                'height' => $this->featured_image_height,
                'aspect_ratio' => $this->calculateAspectRatio(),
            ],
        ];
    }

    /**
     * Get date structure with multiple formats
     *
     * @return array<string, mixed>
     */
    protected function getDateStructure(): array
    {
        return [
            'published_at' => $this->formatDate($this->published_at),
            'created_at' => $this->formatDate($this->created_at),
            'updated_at' => $this->formatDate($this->updated_at),
            'modified_at' => $this->formatDate($this->modified_at ?? $this->updated_at),
            'scheduled_for' => $this->formatDate($this->scheduled_for),
            'archived_at' => $this->formatDate($this->archived_at),
            'last_viewed_at' => $this->formatDate($this->last_viewed_at),
            'is_recently_updated' => $this->isRecentlyUpdated(),
            'is_new' => $this->isNew(),
            'age_in_days' => $this->getAgeInDays(),
        ];
    }

    /**
     * Get SEO structure with advanced metrics
     *
     * @param array $computedData
     * @return array<string, mixed>
     */
    protected function getSEOStructure(array $computedData): array
    {
        return [
            'meta_title' => $this->meta_title ?? $this->generateMetaTitle(),
            'meta_description' => $this->meta_description ?? $this->generateMetaDescription(),
            'canonical_url' => $this->canonical_url ?? $this->getUrl(),
            'og_data' => $this->getOpenGraphData(),
            'twitter_card' => $this->getTwitterCardData(),
            'schema_markup' => $this->getSchemaMarkup(),
            'keywords' => $this->meta_keywords ?? [],
            'focus_keyword' => $this->focus_keyword,
            'keyword_density' => $computedData['keyword_density'],
            'seo_score' => $computedData['seo_score'],
            'readability_score' => $computedData['readability_score'],
            'suggestions' => $this->getSEOSuggestions(),
            'internal_links' => $computedData['internal_links'],
            'external_links' => $computedData['external_links'],
            'sitemap_priority' => $this->sitemap_priority ?? 0.5,
            'sitemap_frequency' => $this->sitemap_frequency ?? 'weekly',
            'robots' => $this->robots_directives ?? 'index,follow',
        ];
    }

    /**
     * Get author structure with extended information
     *
     * @return array<string, mixed>|null
     */
    protected function getAuthorStructure(): ?array
    {
        if (!$this->relationLoaded('author') && !$this->author) {
            return null;
        }

        $author = $this->author;

        return [
            'id' => $author->id,
            'name' => $author->name,
            'slug' => $author->slug ?? Str::slug($author->name),
            'bio' => $author->bio,
            'avatar' => $author->avatar_url,
            'social_links' => $author->social_links ?? [],
            'post_count' => $author->posts_count ?? 0,
            'expertise' => $author->expertise ?? [],
            'verified' => $author->is_verified ?? false,
            'url' => $author->profile_url ?? route('authors.show', $author->slug ?? $author->id),
        ];
    }

    /**
     * Get user interactions
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    protected function getUserInteractions(Request $request): array
    {
        $user = $request->user();
        
        return [
            'likes' => [
                'count' => $this->likes_count ?? 0,
                'user_liked' => $user ? $this->isLikedBy($user) : false,
            ],
            'bookmarks' => [
                'count' => $this->bookmarks_count ?? 0,
                'user_bookmarked' => $user ? $this->isBookmarkedBy($user) : false,
            ],
            'shares' => [
                'count' => $this->shares_count ?? 0,
                'platforms' => $this->share_platforms ?? [],
            ],
            'reactions' => $this->when(
                $this->relationLoaded('reactions'),
                fn() => ReactionResource::collection($this->reactions)
            ),
            'user_rating' => $user ? $this->getUserRating($user) : null,
            'average_rating' => $this->average_rating ?? null,
            'rating_count' => $this->ratings_count ?? 0,
        ];
    }

    /**
     * Get comments structure
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    protected function getCommentsStructure(Request $request): array
    {
        return [
            'count' => $this->comments_count ?? 0,
            'enabled' => $this->comments_enabled ?? true,
            'moderated' => $this->comments_moderated ?? false,
            'recent' => $this->when(
                $request->input('include_comments', false) && $this->relationLoaded('comments'),
                fn() => CommentResource::collection($this->comments->take(5))
            ),
            'featured' => $this->when(
                $this->relationLoaded('featuredComments'),
                fn() => CommentResource::collection($this->featuredComments)
            ),
            'requires_approval' => $this->comments_require_approval ?? false,
            'close_after_days' => $this->comments_close_after_days ?? null,
            'is_closed' => $this->areCommentsClosed(),
        ];
    }

    /**
     * Get processed content blocks with enhancements
     *
     * @return array
     */
    protected function getProcessedContentBlocks(): array
    {
        if (!$this->content_blocks) {
            return [];
        }

        return collect($this->content_blocks)->map(function ($block, $index) {
            return array_merge($block, [
                'id' => $block['id'] ?? Str::uuid()->toString(),
                'order' => $index,
                'estimated_read_time' => $this->estimateBlockReadTime($block),
                'word_count' => $this->countBlockWords($block),
                'has_media' => $this->blockHasMedia($block),
                'accessibility' => $this->getBlockAccessibility($block),
            ]);
        })->toArray();
    }

    /**
     * Get social sharing structure
     *
     * @return array<string, mixed>
     */
    protected function getSocialStructure(): array
    {
        $url = urlencode($this->getUrl());
        $title = urlencode($this->title);
        $excerpt = urlencode($this->excerpt ?? '');

        return [
            'share_urls' => [
                'twitter' => "https://twitter.com/intent/tweet?url={$url}&text={$title}",
                'facebook' => "https://www.facebook.com/sharer/sharer.php?u={$url}",
                'linkedin' => "https://www.linkedin.com/sharing/share-offsite/?url={$url}",
                'reddit' => "https://reddit.com/submit?url={$url}&title={$title}",
                'whatsapp' => "https://wa.me/?text={$title}%20{$url}",
                'telegram' => "https://t.me/share/url?url={$url}&text={$title}",
                'email' => "mailto:?subject={$title}&body={$excerpt}%20{$url}",
            ],
            'share_count' => $this->total_shares ?? 0,
            'trending_platforms' => $this->trending_platforms ?? [],
        ];
    }

    /**
     * Get permissions for the current user
     *
     * @param Request $request
     * @return array<string, bool>
     */
    protected function getPermissions(Request $request): array
    {
        $user = $request->user();
        
        if (!$user) {
            return [
                'can_view' => $this->isPublished(),
                'can_comment' => false,
                'can_share' => true,
            ];
        }

        return [
            'can_view' => $user->can('view', $this->resource),
            'can_edit' => $user->can('update', $this->resource),
            'can_delete' => $user->can('delete', $this->resource),
            'can_publish' => $user->can('publish', $this->resource),
            'can_comment' => $user->can('comment', $this->resource),
            'can_moderate_comments' => $user->can('moderate-comments', $this->resource),
            'can_feature' => $user->can('feature', $this->resource),
            'can_archive' => $user->can('archive', $this->resource),
            'can_view_analytics' => $user->can('view-analytics', $this->resource),
            'can_manage_seo' => $user->can('manage-seo', $this->resource),
        ];
    }

    /**
     * Get monetization data
     *
     * @return array<string, mixed>
     */
    protected function getMonetizationData(): array
    {
        return [
            'type' => $this->monetization_type ?? 'free',
            'price' => $this->price ? $this->formatCurrency($this->price) : null,
            'subscription_required' => $this->subscription_required ?? false,
            'subscription_tiers' => $this->subscription_tiers ?? [],
            'revenue' => [
                'total' => $this->total_revenue ?? 0,
                'this_month' => $this->monthly_revenue ?? 0,
                'average_per_view' => $this->revenue_per_view ?? 0,
            ],
            'affiliate_links' => $this->affiliate_links_count ?? 0,
            'sponsored' => $this->is_sponsored ?? false,
            'sponsor' => $this->sponsor_info ?? null,
        ];
    }

    /**
     * Get metadata
     *
     * @param Request $request
     * @return array<string, mixed>
     */
    protected function getMetadata(Request $request): array
    {
        return [
            'version' => $this->version ?? 1,
            'locale' => $this->locale ?? app()->getLocale(),
            'translations' => $this->when(
                $this->relationLoaded('translations'),
                fn() => $this->translations->pluck('locale')
            ),
            'source' => $this->source ?? 'original',
            'license' => $this->license ?? 'all-rights-reserved',
            'disclaimer' => $this->disclaimer,
            'ai_generated' => $this->ai_generated ?? false,
            'fact_checked' => $this->fact_checked ?? false,
            'fact_checked_at' => $this->formatDate($this->fact_checked_at),
            'editorial_notes' => $this->when(
                $request->user()?->can('view-editorial-notes', $this->resource),
                fn() => $this->editorial_notes
            ),
        ];
    }

    /**
     * Format date with multiple formats
     *
     * @param Carbon|string|null $date
     * @return array<string, mixed>|null
     */
    protected function formatDate($date): ?array
    {
        if (!$date) {
            return null;
        }

        $carbon = $date instanceof Carbon ? $date : Carbon::parse($date);

        return [
            'iso' => $carbon->toIso8601String(),
            'formatted' => $carbon->format('F j, Y'),
            'formatted_time' => $carbon->format('F j, Y g:i A'),
            'relative' => $carbon->diffForHumans(),
            'timestamp' => $carbon->timestamp,
            'day_of_week' => $carbon->format('l'),
            'time_ago' => $this->getTimeAgo($carbon),
        ];
    }

    /**
     * Generate cache key
     *
     * @param Request $request
     * @return string
     */
    protected function getCacheKey(Request $request): string
    {
        $userId = $request->user()?->id ?? 'guest';
        $locale = app()->getLocale();
        
        return sprintf(
            'post_resource_%d_%s_%s_%s',
            $this->id,
            $userId,
            $locale,
            md5(serialize($request->all()))
        );
    }

    /**
     * Calculate engagement score
     *
     * @return float
     */
    protected function calculateEngagementScore(): float
    {
        $views = max(1, $this->view_count ?? 1);
        $interactions = ($this->likes_count ?? 0) + 
                       ($this->comments_count ?? 0) * 2 + 
                       ($this->shares_count ?? 0) * 3;
        
        return round(($interactions / $views) * 100, 2);
    }

    /**
     * Calculate trending score
     *
     * @return float
     */
    protected function calculateTrendingScore(): float
    {
        $recency = max(0, 30 - $this->created_at->diffInDays());
        $engagement = $this->calculateEngagementScore();
        $velocity = $this->calculateViewVelocity();
        
        return round(($recency * 0.3) + ($engagement * 0.4) + ($velocity * 0.3), 2);
    }

    /**
     * Add additional data to the resource
     *
     * @param array<string, mixed> $data
     * @return static
     */
    public function additional(array $data): static
    {
        $this->additional = $data;
        return $this;
    }

    /**
     * Customize the response
     *
     * @param Request $request
     * @param \Illuminate\Http\JsonResponse $response
     * @return void
     */
    public function withResponse($request, $response): void
    {
        $response->header('X-Content-Version', '3.0');
        $response->header('X-Content-Type', 'BlogPost');
        
        if ($request->isMethod('GET')) {
            $response->header('Cache-Control', 'public, max-age=300');
            $response->header('ETag', md5($response->getContent()));
            $response->header('Last-Modified', $this->updated_at->toRfc7231String());
        }
    }

    // Additional helper methods would follow...
    // These would include all the calculation methods referenced above
}