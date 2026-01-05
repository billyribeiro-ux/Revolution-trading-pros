<?php

namespace App\Services\Seo;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Redis;
use Illuminate\Support\Facades\Log;

/**
 * SEO Cache Service
 * 
 * Google L8 Enterprise-Grade Cache Management
 * Implements: Cache Warming, Invalidation, Monitoring, Compression
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class SeoCacheService
{
    private array $config;
    private array $metrics = [
        'hits' => 0,
        'misses' => 0,
        'sets' => 0,
        'deletes' => 0,
    ];

    public function __construct()
    {
        $this->config = config('seo_cache');
    }

    /**
     * Get from cache with metrics tracking.
     */
    public function get(string $key, $default = null)
    {
        $value = Cache::get($key, $default);
        
        if ($value !== $default) {
            $this->metrics['hits']++;
            $this->recordMetric('cache.hit', $key);
        } else {
            $this->metrics['misses']++;
            $this->recordMetric('cache.miss', $key);
        }
        
        return $value;
    }

    /**
     * Remember value in cache with TTL from config.
     */
    public function remember(string $key, string $type, callable $callback)
    {
        $ttl = $this->getTtl($type);
        
        return Cache::remember($key, $ttl, function () use ($callback, $key) {
            $this->metrics['misses']++;
            $this->recordMetric('cache.miss', $key);
            
            $value = $callback();
            
            $this->metrics['sets']++;
            $this->recordMetric('cache.set', $key);
            
            return $value;
        });
    }

    /**
     * Set value in cache.
     */
    public function set(string $key, $value, ?int $ttl = null): bool
    {
        $result = Cache::put($key, $value, $ttl);
        
        if ($result) {
            $this->metrics['sets']++;
            $this->recordMetric('cache.set', $key);
        }
        
        return $result;
    }

    /**
     * Delete from cache.
     */
    public function forget(string $key): bool
    {
        $result = Cache::forget($key);
        
        if ($result) {
            $this->metrics['deletes']++;
            $this->recordMetric('cache.delete', $key);
        }
        
        return $result;
    }

    /**
     * Flush cache by tags.
     */
    public function flushTags(array $tags): bool
    {
        try {
            Cache::tags($tags)->flush();
            $this->recordMetric('cache.flush_tags', implode(',', $tags));
            return true;
        } catch (\Exception $e) {
            Log::error('Cache flush failed', ['tags' => $tags, 'error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Warm cache for popular content.
     */
    public function warmCache(): array
    {
        $warmed = [];
        
        if (!$this->config['warming']['enabled']) {
            return ['status' => 'disabled'];
        }
        
        Log::info('Starting cache warming');
        
        // Warm popular keywords
        if ($this->config['warming']['operations']['popular_keywords']) {
            $warmed['keywords'] = $this->warmPopularKeywords();
        }
        
        // Warm top content
        if ($this->config['warming']['operations']['top_content']) {
            $warmed['content'] = $this->warmTopContent();
        }
        
        // Warm link graph
        if ($this->config['warming']['operations']['link_graph']) {
            $warmed['link_graph'] = $this->warmLinkGraph();
        }
        
        Log::info('Cache warming complete', $warmed);
        
        return $warmed;
    }

    /**
     * Get cache statistics.
     */
    public function getStats(): array
    {
        $redis = Redis::connection('default');
        
        try {
            $info = $redis->info('stats');
            $memory = $redis->info('memory');
            
            $hitRate = $this->calculateHitRate();
            
            return [
                'hits' => $this->metrics['hits'],
                'misses' => $this->metrics['misses'],
                'sets' => $this->metrics['sets'],
                'deletes' => $this->metrics['deletes'],
                'hit_rate' => $hitRate,
                'total_keys' => $redis->dbsize(),
                'used_memory' => $memory['used_memory_human'] ?? 'N/A',
                'used_memory_peak' => $memory['used_memory_peak_human'] ?? 'N/A',
                'evicted_keys' => $info['evicted_keys'] ?? 0,
                'expired_keys' => $info['expired_keys'] ?? 0,
            ];
        } catch (\Exception $e) {
            Log::error('Failed to get cache stats', ['error' => $e->getMessage()]);
            return [
                'error' => 'Failed to retrieve stats',
                'hit_rate' => $this->calculateHitRate(),
            ];
        }
    }

    /**
     * Monitor cache health.
     */
    public function monitorHealth(): array
    {
        $stats = $this->getStats();
        $alerts = [];
        
        // Check hit rate
        if (isset($stats['hit_rate']) && $stats['hit_rate'] < $this->config['monitoring']['alerts']['low_hit_rate_threshold']) {
            $alerts[] = [
                'type' => 'low_hit_rate',
                'severity' => 'warning',
                'message' => "Cache hit rate is {$stats['hit_rate']}%, below threshold",
                'threshold' => $this->config['monitoring']['alerts']['low_hit_rate_threshold'] * 100,
            ];
        }
        
        // Check memory usage
        if (isset($stats['used_memory_peak'])) {
            // Parse memory usage (simplified)
            $alerts[] = [
                'type' => 'memory_usage',
                'severity' => 'info',
                'message' => "Peak memory usage: {$stats['used_memory_peak']}",
            ];
        }
        
        // Check eviction rate
        if (isset($stats['evicted_keys']) && $stats['evicted_keys'] > 1000) {
            $alerts[] = [
                'type' => 'high_eviction',
                'severity' => 'warning',
                'message' => "High eviction rate: {$stats['evicted_keys']} keys evicted",
            ];
        }
        
        return [
            'status' => empty($alerts) ? 'healthy' : 'degraded',
            'stats' => $stats,
            'alerts' => $alerts,
            'timestamp' => now()->toIso8601String(),
        ];
    }

    /**
     * Invalidate cache for content.
     */
    public function invalidateContent(string $contentType, int $contentId): void
    {
        $patterns = [
            "seo:nlp:{$contentType}:{$contentId}:*",
            "seo:entities:{$contentType}:{$contentId}:*",
            "seo:topics:{$contentType}:{$contentId}:*",
            "seo:ai:*:{$contentType}:{$contentId}*",
            "seo:links:suggestions:{$contentType}:{$contentId}",
            "seo:analysis:{$contentType}:{$contentId}:*",
        ];
        
        foreach ($patterns as $pattern) {
            $this->deleteByPattern($pattern);
        }
        
        Log::info('Cache invalidated for content', [
            'type' => $contentType,
            'id' => $contentId,
        ]);
    }

    /**
     * Invalidate cache for keyword.
     */
    public function invalidateKeyword(string $keyword): void
    {
        $keywordHash = md5($keyword);
        
        $patterns = [
            "seo:keywords:difficulty:{$keywordHash}",
            "seo:keywords:serp:{$keywordHash}",
            "seo:serp:keyword:{$keywordHash}",
        ];
        
        foreach ($patterns as $pattern) {
            $this->forget($pattern);
        }
        
        Log::info('Cache invalidated for keyword', ['keyword' => $keyword]);
    }

    /**
     * Invalidate link graph cache.
     */
    public function invalidateLinkGraph(): void
    {
        $patterns = [
            'seo:links:graph:*',
            'seo:links:suggestions:*',
            'seo:links:orphans',
        ];
        
        foreach ($patterns as $pattern) {
            $this->deleteByPattern($pattern);
        }
        
        Log::info('Link graph cache invalidated');
    }

    /**
     * Get TTL for cache type.
     */
    private function getTtl(string $type): int
    {
        return $this->config['ttl'][$type] ?? 3600;
    }

    /**
     * Calculate cache hit rate.
     */
    private function calculateHitRate(): float
    {
        $total = $this->metrics['hits'] + $this->metrics['misses'];
        
        if ($total === 0) {
            return 0.0;
        }
        
        return round(($this->metrics['hits'] / $total) * 100, 2);
    }

    /**
     * Record cache metric.
     */
    private function recordMetric(string $metric, string $key): void
    {
        if (!$this->config['monitoring']['enabled']) {
            return;
        }
        
        // In production, send to monitoring system (Prometheus, DataDog, etc.)
        // For now, just log periodically
        static $logCounter = 0;
        $logCounter++;
        
        if ($logCounter % 100 === 0) {
            Log::debug('Cache metrics', [
                'metric' => $metric,
                'key' => $key,
                'stats' => $this->metrics,
            ]);
        }
    }

    /**
     * Delete keys by pattern.
     */
    private function deleteByPattern(string $pattern): int
    {
        try {
            $redis = Redis::connection('default');
            $keys = $redis->keys($pattern);
            
            if (empty($keys)) {
                return 0;
            }
            
            $deleted = $redis->del($keys);
            $this->metrics['deletes'] += $deleted;
            
            return $deleted;
        } catch (\Exception $e) {
            Log::error('Failed to delete by pattern', [
                'pattern' => $pattern,
                'error' => $e->getMessage(),
            ]);
            return 0;
        }
    }

    /**
     * Warm popular keywords cache.
     */
    private function warmPopularKeywords(): int
    {
        $keywords = \DB::table('seo_keywords')
            ->orderByDesc('search_volume')
            ->limit($this->config['warming']['batch_size'])
            ->pluck('keyword');
        
        $warmed = 0;
        
        foreach ($keywords as $keyword) {
            try {
                // Trigger cache warming by accessing the data
                app(KeywordIntelligenceService::class)->calculateDifficulty($keyword);
                $warmed++;
            } catch (\Exception $e) {
                Log::warning('Failed to warm keyword', [
                    'keyword' => $keyword,
                    'error' => $e->getMessage(),
                ]);
            }
        }
        
        return $warmed;
    }

    /**
     * Warm top content cache.
     */
    private function warmTopContent(): int
    {
        $posts = \DB::table('posts')
            ->orderByDesc('view_count')
            ->limit($this->config['warming']['batch_size'])
            ->get(['id', 'content']);
        
        $warmed = 0;
        
        foreach ($posts as $post) {
            try {
                // Trigger NLP analysis caching
                app(NlpIntelligenceService::class)->analyze('post', $post->id, $post->content);
                $warmed++;
            } catch (\Exception $e) {
                Log::warning('Failed to warm content', [
                    'post_id' => $post->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }
        
        return $warmed;
    }

    /**
     * Warm link graph cache.
     */
    private function warmLinkGraph(): bool
    {
        try {
            app(InternalLinkIntelligenceService::class)->buildLinkGraph();
            return true;
        } catch (\Exception $e) {
            Log::error('Failed to warm link graph', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Compress value for storage.
     */
    private function compress($value): string
    {
        if (!$this->config['serialization']['compression']) {
            return serialize($value);
        }
        
        $serialized = serialize($value);
        $compressed = gzcompress($serialized, $this->config['serialization']['compression_level']);
        
        return base64_encode($compressed);
    }

    /**
     * Decompress value from storage.
     */
    private function decompress(string $value)
    {
        if (!$this->config['serialization']['compression']) {
            return unserialize($value);
        }
        
        $decoded = base64_decode($value);
        $decompressed = gzuncompress($decoded);
        
        return unserialize($decompressed);
    }

    /**
     * Get current metrics.
     */
    public function getMetrics(): array
    {
        return $this->metrics;
    }

    /**
     * Reset metrics.
     */
    public function resetMetrics(): void
    {
        $this->metrics = [
            'hits' => 0,
            'misses' => 0,
            'sets' => 0,
            'deletes' => 0,
        ];
    }
}
