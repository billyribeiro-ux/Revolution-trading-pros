<?php

declare(strict_types=1);

namespace App\Services\Post;

use App\Contracts\PostServiceInterface;
use App\Contracts\RelatedPostStrategyInterface;
use App\Models\Post;
use App\Services\Post\Strategies\CategoryStrategy;
use App\Services\Post\Strategies\HybridStrategy;
use App\Services\Post\Strategies\TagStrategy;
use App\Services\Post\Strategies\TemporalStrategy;
use Illuminate\Contracts\Cache\Repository as CacheRepository;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Psr\Log\LoggerInterface;

class PostService implements PostServiceInterface
{
    private array $strategies = [];
    private CacheRepository $cache;
    private LoggerInterface $logger;
    private bool $telemetryEnabled;
    private bool $cacheEnabled;

    public function __construct(
        ?CacheRepository $cache = null,
        ?LoggerInterface $logger = null
    ) {
        $this->cache = $cache ?? Cache::store();
        $this->logger = $logger ?? Log::channel();
        $this->telemetryEnabled = config('posts.telemetry.enabled', true);
        $this->cacheEnabled = config('posts.related_posts.cache.enabled', true);

        $this->registerStrategies();
    }

    public function getRelatedPosts(Post $post, int $limit = 5, string $algorithm = 'hybrid'): Collection
    {
        return $this->getRelatedPostsWithScores($post, $limit, $algorithm)
            ->pluck('post');
    }

    public function getRelatedPostsWithScores(Post $post, int $limit = 5, string $algorithm = 'hybrid'): Collection
    {
        $startTime = microtime(true);
        
        $limit = $this->normalizeLimit($limit);

        if ($this->cacheEnabled) {
            $cached = $this->getFromCache($post, $limit, $algorithm);
            if ($cached !== null) {
                $this->recordTelemetry('cache_hit', $algorithm, microtime(true) - $startTime);
                return $cached;
            }
        }

        try {
            $strategy = $this->getStrategy($algorithm);

            $candidates = $this->buildRelatedPostsQuery($post, $strategy, $limit)
                ->get();

            $scored = $this->scoreAndRankPosts($post, $candidates, $strategy);

            $minThreshold = config('posts.related_posts.scoring.min_threshold', 0.15);
            $filtered = $scored->filter(fn($item) => $item['score'] >= $minThreshold);

            $results = $filtered->take($limit);

            if ($this->cacheEnabled && $results->isNotEmpty()) {
                $this->storeInCache($post, $limit, $algorithm, $results);
            }

            $duration = microtime(true) - $startTime;
            $this->recordTelemetry('success', $algorithm, $duration, [
                'post_id' => $post->id,
                'results_count' => $results->count(),
                'candidates_count' => $candidates->count(),
            ]);

            return $results;

        } catch (\Throwable $e) {
            $this->logger->error('Failed to get related posts', [
                'post_id' => $post->id,
                'algorithm' => $algorithm,
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            $this->recordTelemetry('error', $algorithm, microtime(true) - $startTime);

            return $this->getFallbackPosts($post, $limit);
        }
    }

    public function warmRelatedPostsCache(Post $post, array $algorithms = ['hybrid']): void
    {
        $limit = config('posts.related_posts.default_limit', 5);

        foreach ($algorithms as $algorithm) {
            if (!isset($this->strategies[$algorithm])) {
                continue;
            }

            $this->getRelatedPostsWithScores($post, $limit, $algorithm);
        }

        $this->logger->info('Warmed related posts cache', [
            'post_id' => $post->id,
            'algorithms' => $algorithms,
        ]);
    }

    public function invalidateRelatedPostsCache(?Post $post = null): void
    {
        $tags = config('posts.related_posts.cache.tags', ['posts', 'related_posts']);

        if ($post === null) {
            $this->cache->tags($tags)->flush();
            
            $this->logger->info('Invalidated all related posts cache');
        } else {
            $this->cache->tags($tags)->flush();
            
            $this->logger->info('Invalidated related posts cache', [
                'post_id' => $post->id,
            ]);
        }
    }

    public function registerStrategy(RelatedPostStrategyInterface $strategy): self
    {
        $this->strategies[$strategy->getIdentifier()] = $strategy;
        return $this;
    }

    private function buildRelatedPostsQuery(
        Post $post,
        RelatedPostStrategyInterface $strategy,
        int $limit
    ): Builder {
        $query = Post::query()
            ->where('id', '!=', $post->id)
            ->where('status', 'published')
            ->whereNotNull('published_at')
            ->limit($limit * 3);

        $eagerLoad = config('posts.related_posts.performance.eager_load', ['author', 'media']);
        if (!empty($eagerLoad)) {
            $query->with($eagerLoad);
        }

        $query = $strategy->applyToQuery($query, $post);

        return $query;
    }

    private function scoreAndRankPosts(
        Post $sourcePost,
        Collection $candidates,
        RelatedPostStrategyInterface $strategy
    ): Collection {
        return $candidates
            ->map(function (Post $candidate) use ($sourcePost, $strategy) {
                return [
                    'post' => $candidate,
                    'score' => $strategy->calculateRelevanceScore($sourcePost, $candidate),
                    'reason' => $strategy->getRelationReason($sourcePost, $candidate),
                ];
            })
            ->sortByDesc('score')
            ->values();
    }

    private function getFallbackPosts(Post $post, int $limit): Collection
    {
        try {
            $posts = Post::query()
                ->where('id', '!=', $post->id)
                ->where('status', 'published')
                ->whereNotNull('published_at')
                ->orderByDesc('published_at')
                ->limit($limit)
                ->get();

            return $posts->map(fn($p) => [
                'post' => $p,
                'score' => 0.5,
                'reason' => 'Recent content',
            ]);
        } catch (\Throwable $e) {
            $this->logger->critical('Fallback posts query failed', [
                'error' => $e->getMessage(),
            ]);
            return collect();
        }
    }

    private function getStrategy(string $algorithm): RelatedPostStrategyInterface
    {
        if (!isset($this->strategies[$algorithm])) {
            throw new \InvalidArgumentException(
                sprintf('Unknown algorithm: %s. Available: %s', 
                    $algorithm, 
                    implode(', ', array_keys($this->strategies))
                )
            );
        }

        return $this->strategies[$algorithm];
    }

    private function normalizeLimit(int $limit): int
    {
        $maxLimit = config('posts.related_posts.max_limit', 20);
        return max(1, min($limit, $maxLimit));
    }

    private function getFromCache(Post $post, int $limit, string $algorithm): ?Collection
    {
        $key = $this->getCacheKey($post, $limit, $algorithm);
        $tags = config('posts.related_posts.cache.tags', ['posts', 'related_posts']);

        $cached = $this->cache->tags($tags)->get($key);

        if ($cached !== null && config('posts.telemetry.track_cache_hits', true)) {
            $this->logger->debug('Cache hit for related posts', [
                'post_id' => $post->id,
                'algorithm' => $algorithm,
            ]);
        }

        return $cached;
    }

    private function storeInCache(Post $post, int $limit, string $algorithm, Collection $results): void
    {
        $key = $this->getCacheKey($post, $limit, $algorithm);
        $tags = config('posts.related_posts.cache.tags', ['posts', 'related_posts']);
        $ttl = config('posts.related_posts.cache.ttl', 3600);

        $this->cache->tags($tags)->put($key, $results, $ttl);
    }

    private function getCacheKey(Post $post, int $limit, string $algorithm): string
    {
        $prefix = config('posts.related_posts.cache.prefix', 'related_posts');
        return sprintf('%s:%d:%d:%s', $prefix, $post->id, $limit, $algorithm);
    }

    private function recordTelemetry(
        string $event,
        string $algorithm,
        float $duration,
        array $context = []
    ): void {
        if (!$this->telemetryEnabled) {
            return;
        }

        $durationMs = $duration * 1000;

        $logContext = array_merge([
            'event' => $event,
            'algorithm' => $algorithm,
            'duration_ms' => round($durationMs, 2),
        ], $context);

        $slowThreshold = config('posts.telemetry.slow_query_threshold', 100);
        if (config('posts.telemetry.log_slow_queries', true) && $durationMs > $slowThreshold) {
            $this->logger->warning('Slow related posts query', $logContext);
        } else {
            $this->logger->debug('Related posts query completed', $logContext);
        }
    }

    private function registerStrategies(): void
    {
        $this->registerStrategy(new CategoryStrategy());
        $this->registerStrategy(new TagStrategy());
        $this->registerStrategy(new TemporalStrategy());
        
        $this->registerStrategy(new HybridStrategy([
            new CategoryStrategy(),
            new TagStrategy(),
            new TemporalStrategy(),
        ]));
    }
}
