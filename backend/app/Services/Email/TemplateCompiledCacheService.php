<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailTemplate;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;
use TijsVerkoyen\CssToInlineStyles\CssToInlineStyles;

/**
 * TemplateCompiledCacheService - Pre-compiled Template Caching
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Provides enterprise-grade template pre-compilation and caching:
 * - Pre-compiles templates on save/update
 * - Multi-layer caching (memory, Redis, file)
 * - CSS inlining at compile time
 * - Variable placeholder optimization
 * - Template versioning support
 * - Cache warming strategies
 *
 * @version 1.0.0
 */
class TemplateCompiledCacheService
{
    /**
     * Cache TTL constants
     */
    private const MEMORY_CACHE_TTL = 300;     // 5 minutes
    private const REDIS_CACHE_TTL = 3600;     // 1 hour
    private const FILE_CACHE_TTL = 86400;     // 24 hours

    /**
     * Cache key prefixes
     */
    private const CACHE_PREFIX = 'compiled_template:';
    private const VERSION_PREFIX = 'template_version:';
    private const STATS_PREFIX = 'template_stats:';

    /**
     * Memory cache for request-level caching
     */
    private array $memoryCache = [];

    /**
     * CSS inliner instance
     */
    private CssToInlineStyles $cssInliner;

    public function __construct()
    {
        $this->cssInliner = new CssToInlineStyles();
    }

    /**
     * Get compiled template (with multi-layer cache)
     */
    public function getCompiled(EmailTemplate $template, string $language = null): array
    {
        $cacheKey = $this->getCacheKey($template, $language);

        // Layer 1: Memory cache (fastest)
        if (isset($this->memoryCache[$cacheKey])) {
            $this->recordCacheHit($template->id, 'memory');
            return $this->memoryCache[$cacheKey];
        }

        // Layer 2: Redis cache
        $compiled = Cache::get($cacheKey);
        if ($compiled !== null) {
            $this->memoryCache[$cacheKey] = $compiled;
            $this->recordCacheHit($template->id, 'redis');
            return $compiled;
        }

        // Layer 3: File cache
        $fileCached = $this->getFromFileCache($cacheKey);
        if ($fileCached !== null) {
            $this->memoryCache[$cacheKey] = $fileCached;
            Cache::put($cacheKey, $fileCached, self::REDIS_CACHE_TTL);
            $this->recordCacheHit($template->id, 'file');
            return $fileCached;
        }

        // Cache miss - compile and cache
        $this->recordCacheMiss($template->id);
        return $this->compileAndCache($template, $language);
    }

    /**
     * Compile and cache template
     */
    public function compileAndCache(EmailTemplate $template, string $language = null): array
    {
        $cacheKey = $this->getCacheKey($template, $language);

        try {
            $compiled = $this->compile($template, $language);

            // Store in all cache layers
            $this->memoryCache[$cacheKey] = $compiled;
            Cache::put($cacheKey, $compiled, self::REDIS_CACHE_TTL);
            $this->saveToFileCache($cacheKey, $compiled);

            // Update version
            $this->updateVersion($template);

            Log::debug('[TemplateCompiledCacheService] Template compiled and cached', [
                'template_id' => $template->id,
                'language' => $language,
            ]);

            return $compiled;
        } catch (\Throwable $e) {
            Log::error('[TemplateCompiledCacheService] Compilation failed', [
                'template_id' => $template->id,
                'error' => $e->getMessage(),
            ]);

            throw $e;
        }
    }

    /**
     * Compile template
     */
    private function compile(EmailTemplate $template, ?string $language): array
    {
        $startTime = microtime(true);

        // Get content for language
        $content = $this->getContentForLanguage($template, $language);

        // Pre-process HTML
        $html = $this->preprocessHtml($content['body_html'] ?? '');

        // Inline CSS
        if ($template->auto_inline_css) {
            $css = $template->inline_css ?? '';
            $css .= $this->extractStyleTags($html);
            $html = $this->cssInliner->convert($html, $css);
        }

        // Optimize variable placeholders
        $html = $this->optimizeVariablePlaceholders($html);

        // Generate text version if not provided
        $text = $content['body_text'] ?? $this->htmlToText($html);

        // Compile subject
        $subject = $this->optimizeVariablePlaceholders($content['subject'] ?? '');

        $duration = (microtime(true) - $startTime) * 1000;

        return [
            'subject' => $subject,
            'html' => $html,
            'text' => $text,
            'preheader' => $template->preheader ?? '',
            'headers' => $template->headers ?? [],
            'compiled_at' => now()->toIso8601String(),
            'version' => $template->version,
            'duration_ms' => round($duration, 2),
            'size_bytes' => strlen($html) + strlen($text),
        ];
    }

    /**
     * Pre-process HTML
     */
    private function preprocessHtml(string $html): string
    {
        // Remove HTML comments
        $html = preg_replace('/<!--(?!\[if).*?-->/s', '', $html);

        // Normalize whitespace
        $html = preg_replace('/\s+/', ' ', $html);

        // Fix self-closing tags for email clients
        $selfClosingTags = ['br', 'hr', 'img', 'input', 'meta', 'link'];
        foreach ($selfClosingTags as $tag) {
            $html = preg_replace("/<{$tag}\s*([^>]*)\/?\s*>/i", "<{$tag} $1 />", $html);
        }

        return trim($html);
    }

    /**
     * Extract style tags from HTML
     */
    private function extractStyleTags(string $html): string
    {
        $css = '';
        preg_match_all('/<style[^>]*>(.*?)<\/style>/is', $html, $matches);

        foreach ($matches[1] as $style) {
            $css .= $style . "\n";
        }

        return $css;
    }

    /**
     * Optimize variable placeholders for faster replacement
     */
    private function optimizeVariablePlaceholders(string $content): string
    {
        // Standardize placeholder format: {{ variable }} -> {{variable}}
        $content = preg_replace('/\{\{\s*(\w+)\s*\}\}/', '{{$1}}', $content);

        // Handle conditional blocks
        $content = preg_replace('/\{%\s*if\s+(\w+)\s*%\}/', '{%if:$1%}', $content);
        $content = preg_replace('/\{%\s*endif\s*%\}/', '{%endif%}', $content);

        return $content;
    }

    /**
     * Convert HTML to plain text
     */
    private function htmlToText(string $html): string
    {
        // Remove style and script tags
        $text = preg_replace('/<(style|script)[^>]*>.*?<\/\1>/is', '', $html);

        // Convert links
        $text = preg_replace('/<a[^>]+href="([^"]*)"[^>]*>([^<]*)<\/a>/i', '$2 [$1]', $text);

        // Convert headers
        $text = preg_replace('/<h[1-6][^>]*>(.*?)<\/h[1-6]>/i', "\n\n== $1 ==\n\n", $text);

        // Convert paragraphs and divs
        $text = preg_replace('/<\/(p|div)>/i', "\n\n", $text);

        // Convert line breaks
        $text = preg_replace('/<br\s*\/?>/i', "\n", $text);

        // Strip remaining tags
        $text = strip_tags($text);

        // Decode entities
        $text = html_entity_decode($text, ENT_QUOTES, 'UTF-8');

        // Normalize whitespace
        $text = preg_replace('/[ \t]+/', ' ', $text);
        $text = preg_replace('/\n{3,}/', "\n\n", $text);

        return trim($text);
    }

    /**
     * Get content for specific language
     */
    private function getContentForLanguage(EmailTemplate $template, ?string $language): array
    {
        if (!$language || $language === $template->primary_language) {
            return [
                'subject' => $template->subject,
                'body_html' => $template->body_html,
                'body_text' => $template->body_text,
            ];
        }

        $translations = $template->translations ?? [];

        return [
            'subject' => $translations[$language]['subject'] ?? $template->subject,
            'body_html' => $translations[$language]['body_html'] ?? $template->body_html,
            'body_text' => $translations[$language]['body_text'] ?? $template->body_text,
        ];
    }

    /**
     * Get cache key for template
     */
    private function getCacheKey(EmailTemplate $template, ?string $language): string
    {
        return self::CACHE_PREFIX . $template->id . ':' . ($language ?? 'default') . ':v' . $template->version;
    }

    /**
     * Get from file cache
     */
    private function getFromFileCache(string $key): ?array
    {
        $path = 'compiled_templates/' . md5($key) . '.json';

        if (!Storage::disk('local')->exists($path)) {
            return null;
        }

        try {
            $content = Storage::disk('local')->get($path);
            $data = json_decode($content, true);

            // Check if expired
            if (isset($data['cached_at'])) {
                $cachedAt = strtotime($data['cached_at']);
                if (time() - $cachedAt > self::FILE_CACHE_TTL) {
                    Storage::disk('local')->delete($path);
                    return null;
                }
            }

            return $data;
        } catch (\Throwable $e) {
            return null;
        }
    }

    /**
     * Save to file cache
     */
    private function saveToFileCache(string $key, array $data): void
    {
        try {
            $path = 'compiled_templates/' . md5($key) . '.json';
            $data['cached_at'] = now()->toIso8601String();
            Storage::disk('local')->put($path, json_encode($data));
        } catch (\Throwable $e) {
            Log::debug('[TemplateCompiledCacheService] Failed to save file cache', [
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Update template version
     */
    private function updateVersion(EmailTemplate $template): void
    {
        Cache::put(
            self::VERSION_PREFIX . $template->id,
            $template->version,
            self::REDIS_CACHE_TTL
        );
    }

    /**
     * Invalidate cache for template
     */
    public function invalidate(EmailTemplate $template): void
    {
        $languages = $template->languages ?? ['default'];
        $languages[] = 'default';

        foreach (array_unique($languages) as $language) {
            $cacheKey = $this->getCacheKey($template, $language);

            unset($this->memoryCache[$cacheKey]);
            Cache::forget($cacheKey);

            $path = 'compiled_templates/' . md5($cacheKey) . '.json';
            Storage::disk('local')->delete($path);
        }

        Cache::forget(self::VERSION_PREFIX . $template->id);

        Log::debug('[TemplateCompiledCacheService] Cache invalidated', [
            'template_id' => $template->id,
        ]);
    }

    /**
     * Warm cache for frequently used templates
     */
    public function warmCache(array $templateIds = [], array $languages = ['en']): array
    {
        $results = [];

        // If no IDs provided, warm top 50 most used templates
        if (empty($templateIds)) {
            $templateIds = EmailTemplate::orderByDesc('usage_count')
                ->limit(50)
                ->pluck('id')
                ->toArray();
        }

        foreach ($templateIds as $templateId) {
            $template = EmailTemplate::find($templateId);
            if (!$template) {
                continue;
            }

            foreach ($languages as $language) {
                try {
                    $this->compileAndCache($template, $language);
                    $results[$templateId][$language] = 'success';
                } catch (\Throwable $e) {
                    $results[$templateId][$language] = 'failed: ' . $e->getMessage();
                }
            }
        }

        Log::info('[TemplateCompiledCacheService] Cache warming completed', [
            'templates' => count($templateIds),
            'languages' => count($languages),
        ]);

        return $results;
    }

    /**
     * Record cache hit
     */
    private function recordCacheHit(int $templateId, string $layer): void
    {
        $key = self::STATS_PREFIX . 'hits:' . date('Y-m-d');
        Cache::increment($key);
        Cache::increment("{$key}:{$layer}");
        Cache::increment(self::STATS_PREFIX . "template:{$templateId}:hits");
    }

    /**
     * Record cache miss
     */
    private function recordCacheMiss(int $templateId): void
    {
        $key = self::STATS_PREFIX . 'misses:' . date('Y-m-d');
        Cache::increment($key);
        Cache::increment(self::STATS_PREFIX . "template:{$templateId}:misses");
    }

    /**
     * Get cache statistics
     */
    public function getStatistics(): array
    {
        $today = date('Y-m-d');

        return [
            'today' => [
                'hits' => Cache::get(self::STATS_PREFIX . 'hits:' . $today, 0),
                'misses' => Cache::get(self::STATS_PREFIX . 'misses:' . $today, 0),
                'hits_by_layer' => [
                    'memory' => Cache::get(self::STATS_PREFIX . "hits:{$today}:memory", 0),
                    'redis' => Cache::get(self::STATS_PREFIX . "hits:{$today}:redis", 0),
                    'file' => Cache::get(self::STATS_PREFIX . "hits:{$today}:file", 0),
                ],
            ],
            'memory_cache_size' => count($this->memoryCache),
        ];
    }

    /**
     * Clear all compiled caches
     */
    public function clearAll(): void
    {
        $this->memoryCache = [];

        // Clear Redis cache
        $keys = Cache::get('compiled_template_keys', []);
        foreach ($keys as $key) {
            Cache::forget($key);
        }

        // Clear file cache
        Storage::disk('local')->deleteDirectory('compiled_templates');

        Log::info('[TemplateCompiledCacheService] All caches cleared');
    }
}
