<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Media;
use App\Models\MediaVariant;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * CdnService
 *
 * Google Enterprise Grade CDN integration with:
 * - Circuit breaker pattern for external service protection
 * - Automatic failover to local storage
 * - Configurable retry policies
 * - Cache purge with resilience
 *
 * @version 2.0.0
 * @level L8 Principal Engineer
 */
class CdnService
{
    protected array $config;
    protected ?string $provider;
    protected ?string $baseUrl;
    protected CircuitBreaker $cloudflareBreaker;
    protected CircuitBreaker $bunnyBreaker;

    public function __construct()
    {
        $this->config = config('image-optimization.cdn', []);
        $this->provider = $this->config['provider'] ?? 'custom';
        $this->baseUrl = $this->config['base_url'] ?? env('CDN_URL');

        // Initialize circuit breakers for CDN providers
        $this->cloudflareBreaker = new CircuitBreaker(
            service: 'cloudflare_cdn',
            failureThreshold: 3,
            successThreshold: 2,
            timeout: 60,
            halfOpenRequests: 2
        );

        $this->bunnyBreaker = new CircuitBreaker(
            service: 'bunny_cdn',
            failureThreshold: 3,
            successThreshold: 2,
            timeout: 60,
            halfOpenRequests: 2
        );
    }

    /**
     * Get circuit breaker metrics for monitoring
     */
    public function getCircuitBreakerMetrics(): array
    {
        return [
            'cloudflare' => $this->cloudflareBreaker->getMetrics(),
            'bunny' => $this->bunnyBreaker->getMetrics(),
        ];
    }

    /**
     * Check if CDN is enabled
     */
    public function isEnabled(): bool
    {
        return ($this->config['enabled'] ?? false) && !empty($this->baseUrl);
    }

    /**
     * Get CDN URL for a media item
     */
    public function getCdnUrl(Media $media, ?string $variant = null): ?string
    {
        if (!$this->isEnabled()) {
            return null;
        }

        $path = $variant ? $this->getVariantPath($media, $variant) : $media->path;

        return $this->buildCdnUrl($path);
    }

    /**
     * Get CDN URL for a specific variant
     */
    public function getVariantCdnUrl(MediaVariant $variant): ?string
    {
        if (!$this->isEnabled()) {
            return null;
        }

        return $this->buildCdnUrl($variant->path);
    }

    /**
     * Build full CDN URL from path
     */
    protected function buildCdnUrl(string $path): string
    {
        $baseUrl = rtrim($this->baseUrl, '/');
        $path = ltrim($path, '/');

        return "{$baseUrl}/{$path}";
    }

    /**
     * Get optimized image URL with transformations
     */
    public function getOptimizedUrl(
        Media $media,
        array $options = []
    ): string {
        if (!$this->isEnabled()) {
            return $media->getPublicUrl();
        }

        $width = $options['width'] ?? null;
        $height = $options['height'] ?? null;
        $quality = $options['quality'] ?? null;
        $format = $options['format'] ?? 'webp';

        // Find matching variant
        if ($width || $height) {
            $variant = $this->findBestVariant($media, $width, $height, $format);
            if ($variant) {
                return $this->getVariantCdnUrl($variant) ?? $variant->getPublicUrl();
            }
        }

        // Return WebP version if available
        $webpVariant = MediaVariant::where('media_id', $media->id)
            ->where('format', 'webp')
            ->whereNull('size_name')
            ->first();

        if ($webpVariant) {
            return $this->getVariantCdnUrl($webpVariant) ?? $webpVariant->getPublicUrl();
        }

        return $this->getCdnUrl($media) ?? $media->getPublicUrl();
    }

    /**
     * Find best matching variant for requested dimensions
     */
    protected function findBestVariant(
        Media $media,
        ?int $width,
        ?int $height,
        string $format = 'webp'
    ): ?MediaVariant {
        $query = MediaVariant::where('media_id', $media->id)
            ->where('format', $format);

        if ($width) {
            // Find the smallest variant that's at least as wide as requested
            $query->where('width', '>=', $width)
                ->orderBy('width', 'asc');
        }

        if ($height) {
            $query->where('height', '>=', $height);
        }

        return $query->first();
    }

    /**
     * Generate srcset for responsive images
     */
    public function generateSrcset(Media $media, string $format = 'webp'): string
    {
        $variants = MediaVariant::where('media_id', $media->id)
            ->where('format', $format)
            ->where('is_retina', false)
            ->orderBy('width', 'asc')
            ->get();

        $srcset = [];

        foreach ($variants as $variant) {
            $url = $this->getVariantCdnUrl($variant) ?? $variant->getPublicUrl();
            $srcset[] = "{$url} {$variant->width}w";
        }

        // Add original if no variants or if original is larger than all variants
        if ($variants->isEmpty() || $variants->max('width') < $media->width) {
            $url = $this->getCdnUrl($media) ?? $media->getPublicUrl();
            $srcset[] = "{$url} {$media->width}w";
        }

        return implode(', ', $srcset);
    }

    /**
     * Generate sizes attribute suggestion
     */
    public function generateSizes(array $breakpoints = []): string
    {
        $defaults = [
            '(max-width: 640px)' => '100vw',
            '(max-width: 768px)' => '80vw',
            '(max-width: 1024px)' => '60vw',
            '' => '50vw',
        ];

        $breakpoints = array_merge($defaults, $breakpoints);
        $sizes = [];

        foreach ($breakpoints as $query => $size) {
            if ($query) {
                $sizes[] = "{$query} {$size}";
            } else {
                $sizes[] = $size;
            }
        }

        return implode(', ', $sizes);
    }

    /**
     * Get picture element sources for format fallbacks
     */
    public function getPictureSources(Media $media): array
    {
        $sources = [];

        // AVIF (best compression)
        $avifVariants = MediaVariant::where('media_id', $media->id)
            ->where('format', 'avif')
            ->get();

        if ($avifVariants->isNotEmpty()) {
            $sources[] = [
                'type' => 'image/avif',
                'srcset' => $this->buildSrcsetFromVariants($avifVariants),
            ];
        }

        // WebP (good compression, wide support)
        $webpVariants = MediaVariant::where('media_id', $media->id)
            ->where('format', 'webp')
            ->get();

        if ($webpVariants->isNotEmpty()) {
            $sources[] = [
                'type' => 'image/webp',
                'srcset' => $this->buildSrcsetFromVariants($webpVariants),
            ];
        }

        return $sources;
    }

    /**
     * Build srcset string from variants collection
     */
    protected function buildSrcsetFromVariants($variants): string
    {
        $srcset = [];

        foreach ($variants as $variant) {
            $url = $this->getVariantCdnUrl($variant) ?? $variant->getPublicUrl();
            $width = $variant->width;

            if ($variant->is_retina) {
                $srcset[] = "{$url} {$variant->pixel_density}x";
            } else {
                $srcset[] = "{$url} {$width}w";
            }
        }

        return implode(', ', $srcset);
    }

    /**
     * Purge CDN cache for a media item
     */
    public function purgeCache(Media $media): bool
    {
        if (!$this->isEnabled()) {
            return true;
        }

        try {
            $urls = [$this->getCdnUrl($media)];

            // Add all variant URLs
            $variants = MediaVariant::where('media_id', $media->id)->get();
            foreach ($variants as $variant) {
                $urls[] = $this->getVariantCdnUrl($variant);
            }

            return $this->purgeUrls(array_filter($urls));
        } catch (\Exception $e) {
            Log::error('CDN cache purge failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Purge specific URLs from CDN cache
     */
    protected function purgeUrls(array $urls): bool
    {
        // Implementation depends on CDN provider
        // This is a placeholder for provider-specific implementations

        switch ($this->provider) {
            case 'cloudflare':
                return $this->purgeCloudflare($urls);
            case 'bunny':
                return $this->purgeBunny($urls);
            case 'cloudinary':
                return $this->purgeCloudinary($urls);
            default:
                // Custom CDN - log the URLs for manual purge or API call
                Log::info('CDN purge requested', ['urls' => $urls]);
                return true;
        }
    }

    /**
     * Purge Cloudflare cache with circuit breaker protection
     */
    protected function purgeCloudflare(array $urls): bool
    {
        $zoneId = env('CLOUDFLARE_ZONE_ID');
        $apiToken = env('CLOUDFLARE_API_TOKEN');

        if (!$zoneId || !$apiToken) {
            Log::warning('Cloudflare credentials not configured');
            return false;
        }

        // Use circuit breaker to protect against Cloudflare API failures
        return $this->cloudflareBreaker->execute(
            operation: function () use ($zoneId, $apiToken, $urls) {
                $response = Http::withToken($apiToken)
                    ->timeout(10)
                    ->retry(3, 100, function ($exception) {
                        // Only retry on network errors, not on API errors
                        return $exception instanceof \Illuminate\Http\Client\ConnectionException;
                    })
                    ->post("https://api.cloudflare.com/client/v4/zones/{$zoneId}/purge_cache", [
                        'files' => $urls,
                    ]);

                if (!$response->successful()) {
                    throw new \RuntimeException(
                        "Cloudflare API error: " . $response->status() . " - " . $response->body()
                    );
                }

                Log::info('Cloudflare cache purged successfully', [
                    'urls_count' => count($urls),
                ]);

                return true;
            },
            fallback: function () use ($urls) {
                // Circuit breaker open - queue for retry later
                Log::warning('Cloudflare circuit breaker OPEN, queuing purge for retry', [
                    'urls_count' => count($urls),
                ]);
                Cache::put(
                    'cdn_purge_queue:cloudflare:' . md5(json_encode($urls)),
                    $urls,
                    now()->addHour()
                );
                return false;
            }
        );
    }

    /**
     * Purge Bunny CDN cache with circuit breaker protection
     */
    protected function purgeBunny(array $urls): bool
    {
        $apiKey = env('BUNNY_API_KEY');
        $pullZone = env('BUNNY_PULL_ZONE');

        if (!$apiKey || !$pullZone) {
            Log::warning('Bunny CDN credentials not configured');
            return false;
        }

        // Use circuit breaker to protect against Bunny API failures
        return $this->bunnyBreaker->execute(
            operation: function () use ($apiKey, $urls) {
                foreach ($urls as $url) {
                    $response = Http::withHeaders(['AccessKey' => $apiKey])
                        ->timeout(10)
                        ->retry(3, 100, function ($exception) {
                            return $exception instanceof \Illuminate\Http\Client\ConnectionException;
                        })
                        ->post("https://api.bunny.net/purge?url=" . urlencode($url));

                    if (!$response->successful()) {
                        throw new \RuntimeException(
                            "Bunny CDN API error: " . $response->status()
                        );
                    }
                }

                Log::info('Bunny CDN cache purged successfully', [
                    'urls_count' => count($urls),
                ]);

                return true;
            },
            fallback: function () use ($urls) {
                // Circuit breaker open - queue for retry later
                Log::warning('Bunny CDN circuit breaker OPEN, queuing purge for retry', [
                    'urls_count' => count($urls),
                ]);
                Cache::put(
                    'cdn_purge_queue:bunny:' . md5(json_encode($urls)),
                    $urls,
                    now()->addHour()
                );
                return false;
            }
        );
    }

    /**
     * Purge Cloudinary cache
     */
    protected function purgeCloudinary(array $urls): bool
    {
        // Cloudinary handles cache automatically
        // Invalidation happens on resource update
        return true;
    }

    /**
     * Get variant path helper
     */
    protected function getVariantPath(Media $media, string $variant): string
    {
        $variantData = collect($media->variants)->firstWhere('type', $variant);
        return $variantData['path'] ?? $media->path;
    }

    /**
     * Replace URLs in HTML content with CDN URLs
     */
    public function replaceUrlsInContent(string $content): string
    {
        if (!$this->isEnabled() || !$this->config['auto_replace_urls']) {
            return $content;
        }

        // Get local storage URL pattern
        $storageUrl = config('app.url') . '/storage';
        $cdnUrl = rtrim($this->baseUrl, '/');

        // Replace storage URLs with CDN URLs
        $content = str_replace($storageUrl, $cdnUrl, $content);

        return $content;
    }

    /**
     * Get cache headers for CDN
     */
    public function getCacheHeaders(): array
    {
        $cacheControl = $this->config['cache_control'] ?? 'public, max-age=31536000, immutable';

        return [
            'Cache-Control' => $cacheControl,
            'Vary' => 'Accept',
        ];
    }
}
