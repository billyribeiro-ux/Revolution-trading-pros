<?php

declare(strict_types=1);

namespace App\Services;

use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;
use Illuminate\Http\UploadedFile;
use Exception;

/**
 * Cloudflare Images Service
 * ⚡ LIGHTNING STACK - ICT 11+ Principal Engineer Implementation
 *
 * High-performance image delivery with global edge caching.
 * - Automatic WebP/AVIF conversion
 * - On-the-fly resizing
 * - Global CDN delivery (<20ms)
 * - Zero egress fees
 *
 * @version 3.0.0 - Lightning Stack Edition
 * @see https://developers.cloudflare.com/images/
 */
class CloudflareImagesService
{
    /**
     * Cloudflare API base URL
     */
    private const API_BASE = 'https://api.cloudflare.com/client/v4';

    /**
     * Image delivery base URL
     */
    private const DELIVERY_BASE = 'https://imagedelivery.net';

    /**
     * Cloudflare account ID
     */
    private string $accountId;

    /**
     * Cloudflare API token
     */
    private string $apiToken;

    /**
     * Account hash for image delivery
     */
    private string $accountHash;

    /**
     * Whether the service is enabled
     */
    private bool $enabled;

    /**
     * Cache TTL for image metadata (24 hours)
     */
    private const CACHE_TTL = 86400;

    /**
     * Predefined image variants
     */
    public const VARIANTS = [
        'thumbnail' => 'w=150,h=150,fit=cover',
        'small' => 'w=320,h=240,fit=contain',
        'medium' => 'w=640,h=480,fit=contain',
        'large' => 'w=1280,h=960,fit=contain',
        'xlarge' => 'w=1920,h=1440,fit=contain',
        'hero' => 'w=1920,h=1080,fit=cover',
        'og' => 'w=1200,h=630,fit=cover', // Open Graph
        'avatar' => 'w=200,h=200,fit=cover',
        'card' => 'w=400,h=300,fit=cover',
        'public' => '', // Original with optimization
    ];

    /**
     * Constructor
     */
    public function __construct()
    {
        $this->accountId = config('services.cloudflare.account_id', '');
        $this->apiToken = config('services.cloudflare.images_token', '');
        $this->accountHash = config('services.cloudflare.images_hash', '');
        $this->enabled = config('services.cloudflare.images_enabled', false);
    }

    /**
     * Check if the service is properly configured
     */
    public function isConfigured(): bool
    {
        return $this->enabled
            && !empty($this->accountId)
            && !empty($this->apiToken)
            && !empty($this->accountHash);
    }

    /**
     * Upload an image to Cloudflare Images
     *
     * @param UploadedFile|string $image File or URL
     * @param array $metadata Optional metadata
     * @return array{id: string, url: string, variants: array}
     * @throws Exception
     */
    public function upload(UploadedFile|string $image, array $metadata = []): array
    {
        if (!$this->isConfigured()) {
            throw new Exception('Cloudflare Images service is not configured');
        }

        $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v1";

        try {
            if ($image instanceof UploadedFile) {
                $response = Http::withToken($this->apiToken)
                    ->attach('file', file_get_contents($image->path()), $image->getClientOriginalName())
                    ->post($endpoint, [
                        'metadata' => json_encode($metadata),
                        'requireSignedURLs' => false,
                    ]);
            } else {
                // URL upload
                $response = Http::withToken($this->apiToken)
                    ->post($endpoint, [
                        'url' => $image,
                        'metadata' => json_encode($metadata),
                        'requireSignedURLs' => false,
                    ]);
            }

            if (!$response->successful()) {
                Log::error('Cloudflare Images upload failed', [
                    'status' => $response->status(),
                    'body' => $response->body(),
                ]);
                throw new Exception('Failed to upload image to Cloudflare: ' . $response->body());
            }

            $result = $response->json('result');

            return [
                'id' => $result['id'],
                'url' => $this->getDeliveryUrl($result['id']),
                'variants' => $this->generateVariantUrls($result['id']),
                'filename' => $result['filename'] ?? null,
                'uploaded' => $result['uploaded'] ?? now()->toIso8601String(),
                'metadata' => $result['meta'] ?? $metadata,
            ];
        } catch (Exception $e) {
            Log::error('Cloudflare Images upload exception', [
                'error' => $e->getMessage(),
            ]);
            throw $e;
        }
    }

    /**
     * Delete an image from Cloudflare Images
     *
     * @param string $imageId
     * @return bool
     */
    public function delete(string $imageId): bool
    {
        if (!$this->isConfigured()) {
            return false;
        }

        $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v1/{$imageId}";

        try {
            $response = Http::withToken($this->apiToken)->delete($endpoint);

            if ($response->successful()) {
                Cache::forget("cf_image:{$imageId}");
                return true;
            }

            Log::warning('Cloudflare Images delete failed', [
                'imageId' => $imageId,
                'status' => $response->status(),
            ]);

            return false;
        } catch (Exception $e) {
            Log::error('Cloudflare Images delete exception', [
                'imageId' => $imageId,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get image details
     *
     * @param string $imageId
     * @return array|null
     */
    public function getDetails(string $imageId): ?array
    {
        return Cache::remember("cf_image:{$imageId}", self::CACHE_TTL, function () use ($imageId) {
            if (!$this->isConfigured()) {
                return null;
            }

            $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v1/{$imageId}";

            try {
                $response = Http::withToken($this->apiToken)->get($endpoint);

                if ($response->successful()) {
                    return $response->json('result');
                }

                return null;
            } catch (Exception $e) {
                Log::error('Cloudflare Images get details exception', [
                    'imageId' => $imageId,
                    'error' => $e->getMessage(),
                ]);
                return null;
            }
        });
    }

    /**
     * Get the delivery URL for an image
     *
     * @param string $imageId
     * @param string $variant Variant name or custom options
     * @return string
     */
    public function getDeliveryUrl(string $imageId, string $variant = 'public'): string
    {
        if (!$this->isConfigured()) {
            return '';
        }

        $variantPath = self::VARIANTS[$variant] ?? $variant;

        if (empty($variantPath)) {
            return self::DELIVERY_BASE . "/{$this->accountHash}/{$imageId}/public";
        }

        return self::DELIVERY_BASE . "/{$this->accountHash}/{$imageId}/{$variantPath}";
    }

    /**
     * Generate URLs for all predefined variants
     *
     * @param string $imageId
     * @return array<string, string>
     */
    public function generateVariantUrls(string $imageId): array
    {
        $urls = [];

        foreach (array_keys(self::VARIANTS) as $variantName) {
            $urls[$variantName] = $this->getDeliveryUrl($imageId, $variantName);
        }

        return $urls;
    }

    /**
     * Get a responsive image srcset
     *
     * @param string $imageId
     * @param array $widths Array of widths to generate
     * @return string srcset attribute value
     */
    public function getSrcset(string $imageId, array $widths = [320, 640, 960, 1280, 1920]): string
    {
        if (!$this->isConfigured()) {
            return '';
        }

        $srcset = [];

        foreach ($widths as $width) {
            $url = self::DELIVERY_BASE . "/{$this->accountHash}/{$imageId}/w={$width},fit=contain";
            $srcset[] = "{$url} {$width}w";
        }

        return implode(', ', $srcset);
    }

    /**
     * Get optimized image HTML with srcset
     *
     * @param string $imageId
     * @param string $alt Alt text
     * @param string $class CSS classes
     * @param array $widths Responsive widths
     * @return string HTML img tag
     */
    public function getResponsiveImageHtml(
        string $imageId,
        string $alt = '',
        string $class = '',
        array $widths = [320, 640, 960, 1280, 1920]
    ): string {
        if (!$this->isConfigured()) {
            return '';
        }

        $src = $this->getDeliveryUrl($imageId, 'medium');
        $srcset = $this->getSrcset($imageId, $widths);
        $sizes = '(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw';

        $classAttr = $class ? " class=\"{$class}\"" : '';
        $altAttr = htmlspecialchars($alt, ENT_QUOTES);

        return <<<HTML
<img
    src="{$src}"
    srcset="{$srcset}"
    sizes="{$sizes}"
    alt="{$altAttr}"
    loading="lazy"
    decoding="async"{$classAttr}
>
HTML;
    }

    /**
     * List all images with pagination
     *
     * @param int $page
     * @param int $perPage
     * @return array
     */
    public function list(int $page = 1, int $perPage = 100): array
    {
        if (!$this->isConfigured()) {
            return ['images' => [], 'total' => 0];
        }

        $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v1";

        try {
            $response = Http::withToken($this->apiToken)
                ->get($endpoint, [
                    'page' => $page,
                    'per_page' => $perPage,
                ]);

            if ($response->successful()) {
                $result = $response->json();
                return [
                    'images' => $result['result']['images'] ?? [],
                    'total' => $result['result_info']['total_count'] ?? 0,
                ];
            }

            return ['images' => [], 'total' => 0];
        } catch (Exception $e) {
            Log::error('Cloudflare Images list exception', [
                'error' => $e->getMessage(),
            ]);
            return ['images' => [], 'total' => 0];
        }
    }

    /**
     * Get usage statistics
     *
     * @return array
     */
    public function getUsageStats(): array
    {
        if (!$this->isConfigured()) {
            return [];
        }

        $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v1/stats";

        try {
            $response = Http::withToken($this->apiToken)->get($endpoint);

            if ($response->successful()) {
                return $response->json('result') ?? [];
            }

            return [];
        } catch (Exception $e) {
            Log::error('Cloudflare Images stats exception', [
                'error' => $e->getMessage(),
            ]);
            return [];
        }
    }

    /**
     * Create a direct upload URL (for client-side uploads)
     *
     * @param array $metadata
     * @param int $expirySeconds
     * @return array{uploadURL: string, id: string}|null
     */
    public function createDirectUploadUrl(array $metadata = [], int $expirySeconds = 1800): ?array
    {
        if (!$this->isConfigured()) {
            return null;
        }

        $endpoint = self::API_BASE . "/accounts/{$this->accountId}/images/v2/direct_upload";

        try {
            $response = Http::withToken($this->apiToken)
                ->post($endpoint, [
                    'metadata' => $metadata,
                    'expiry' => now()->addSeconds($expirySeconds)->toIso8601String(),
                    'requireSignedURLs' => false,
                ]);

            if ($response->successful()) {
                $result = $response->json('result');
                return [
                    'uploadURL' => $result['uploadURL'],
                    'id' => $result['id'],
                ];
            }

            return null;
        } catch (Exception $e) {
            Log::error('Cloudflare Images direct upload URL exception', [
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Batch upload images
     *
     * @param array $images Array of UploadedFile or URLs
     * @param array $metadata Shared metadata for all images
     * @return array Results for each image
     */
    public function batchUpload(array $images, array $metadata = []): array
    {
        $results = [];

        foreach ($images as $index => $image) {
            try {
                $results[$index] = [
                    'success' => true,
                    'data' => $this->upload($image, $metadata),
                ];
            } catch (Exception $e) {
                $results[$index] = [
                    'success' => false,
                    'error' => $e->getMessage(),
                ];
            }
        }

        return $results;
    }

    /**
     * Migrate an existing image URL to Cloudflare Images
     *
     * @param string $url Original image URL
     * @param array $metadata
     * @return array|null
     */
    public function migrateFromUrl(string $url, array $metadata = []): ?array
    {
        try {
            return $this->upload($url, array_merge($metadata, [
                'migrated_from' => $url,
                'migrated_at' => now()->toIso8601String(),
            ]));
        } catch (Exception $e) {
            Log::error('Cloudflare Images migration failed', [
                'url' => $url,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }
}
