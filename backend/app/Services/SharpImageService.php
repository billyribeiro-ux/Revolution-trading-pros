<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Media;
use App\Models\MediaVariant;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;

/**
 * SharpImageService
 *
 * High-performance image processing service that delegates to Node.js Sharp service.
 * Provides 10-15x faster image processing compared to PHP-based solutions.
 *
 * Features:
 * - WebP/AVIF conversion
 * - Responsive image generation
 * - BlurHash placeholders
 * - LQIP generation
 * - Cloudflare R2 integration
 */
class SharpImageService
{
    protected string $serviceUrl;
    protected bool $enabled;
    protected string $disk;
    protected int $timeout;

    public function __construct()
    {
        $this->serviceUrl = config('services.sharp.url', 'http://localhost:3001');
        $this->enabled = config('services.sharp.enabled', true);
        $this->disk = config('filesystems.default_media_disk', 'r2');
        $this->timeout = config('services.sharp.timeout', 60);
    }

    /**
     * Check if Sharp service is available
     */
    public function isAvailable(): bool
    {
        if (!$this->enabled) {
            return false;
        }

        try {
            $response = Http::timeout(5)->get("{$this->serviceUrl}/health");
            return $response->successful() && $response->json('status') === 'healthy';
        } catch (\Exception $e) {
            Log::warning('Sharp service unavailable', ['error' => $e->getMessage()]);
            return false;
        }
    }

    /**
     * Get Sharp service capabilities
     */
    public function getCapabilities(): array
    {
        try {
            $response = Http::timeout(5)->get("{$this->serviceUrl}/capabilities");
            return $response->json() ?? [];
        } catch (\Exception $e) {
            return [];
        }
    }

    /**
     * Upload and process image through Sharp service
     */
    public function upload(
        UploadedFile $file,
        ?string $collection = null,
        string $preset = 'balanced',
        array $options = []
    ): array {
        $startTime = microtime(true);

        // Prepare options
        $processOptions = array_merge([
            'preset' => $preset,
            'generateWebp' => 'true',
            'generateAvif' => 'true',
            'generateResponsive' => 'true',
            'generateThumbnail' => 'true',
            'generateBlurhash' => 'true',
            'generateRetina' => $options['retina'] ?? 'false',
            'collection' => $collection ?? 'general',
            'altText' => $options['alt_text'] ?? '',
            'title' => $options['title'] ?? pathinfo($file->getClientOriginalName(), PATHINFO_FILENAME),
        ], $options);

        try {
            // Send to Sharp service
            $response = Http::timeout($this->timeout)
                ->attach('image', $file->getContent(), $file->getClientOriginalName())
                ->post("{$this->serviceUrl}/upload", $processOptions);

            if (!$response->successful()) {
                throw new \Exception('Sharp service returned error: ' . $response->body());
            }

            $result = $response->json();

            // Create media record in database
            $media = $this->createMediaRecord($result, $collection, $options);

            Log::info('Image processed via Sharp service', [
                'media_id' => $media->id,
                'processing_time_ms' => $result['processingTime'] ?? 0,
                'total_time_ms' => round((microtime(true) - $startTime) * 1000),
                'variants_count' => count($result['variants'] ?? []),
            ]);

            return [
                'success' => true,
                'media' => $media,
                'result' => $result,
                'processing_time' => $result['processingTime'] ?? 0,
            ];

        } catch (\Exception $e) {
            Log::error('Sharp upload failed', [
                'error' => $e->getMessage(),
                'file' => $file->getClientOriginalName(),
            ]);

            throw $e;
        }
    }

    /**
     * Process existing image from URL
     */
    public function processFromUrl(string $url, ?string $collection = null, array $options = []): array
    {
        try {
            $response = Http::timeout($this->timeout)
                ->post("{$this->serviceUrl}/process-url", [
                    'url' => $url,
                    'options' => array_merge([
                        'preset' => $options['preset'] ?? 'balanced',
                        'generateWebp' => true,
                        'generateAvif' => true,
                        'generateResponsive' => true,
                        'generateThumbnail' => true,
                        'generateBlurhash' => true,
                        'collection' => $collection ?? 'general',
                    ], $options),
                ]);

            if (!$response->successful()) {
                throw new \Exception('Sharp service returned error: ' . $response->body());
            }

            $result = $response->json();
            $media = $this->createMediaRecord($result, $collection, $options);

            return [
                'success' => true,
                'media' => $media,
                'result' => $result,
            ];

        } catch (\Exception $e) {
            Log::error('Sharp URL processing failed', ['error' => $e->getMessage(), 'url' => $url]);
            throw $e;
        }
    }

    /**
     * Optimize existing media
     */
    public function optimize(Media $media, string $preset = 'balanced'): array
    {
        try {
            $response = Http::timeout($this->timeout)
                ->post("{$this->serviceUrl}/optimize", [
                    'key' => $media->path,
                    'options' => [
                        'preset' => $preset,
                        'generateWebp' => true,
                        'generateAvif' => true,
                        'generateResponsive' => true,
                        'generateThumbnail' => true,
                        'generateBlurhash' => true,
                    ],
                ]);

            if (!$response->successful()) {
                throw new \Exception('Sharp optimization failed: ' . $response->body());
            }

            $result = $response->json();

            // Update media record with new variants
            $this->updateMediaWithVariants($media, $result);

            return [
                'success' => true,
                'media' => $media->fresh(),
                'result' => $result,
            ];

        } catch (\Exception $e) {
            Log::error('Sharp optimization failed', ['error' => $e->getMessage(), 'media_id' => $media->id]);
            throw $e;
        }
    }

    /**
     * Generate single variant
     */
    public function generateVariant(Media $media, array $options): ?MediaVariant
    {
        try {
            $response = Http::timeout($this->timeout)
                ->post("{$this->serviceUrl}/generate-variant", [
                    'key' => $media->path,
                    'width' => $options['width'] ?? null,
                    'height' => $options['height'] ?? null,
                    'format' => $options['format'] ?? 'webp',
                    'quality' => $options['quality'] ?? 85,
                ]);

            if (!$response->successful()) {
                return null;
            }

            $result = $response->json('variant');

            return MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => $options['type'] ?? 'custom',
                'size_name' => $options['size_name'] ?? 'custom',
                'width' => $result['width'],
                'height' => $result['height'],
                'filename' => basename($result['key']),
                'path' => $result['key'],
                'disk' => $this->disk,
                'url' => $result['url'],
                'mime_type' => "image/{$result['format']}",
                'format' => $result['format'],
                'size' => $result['size'],
                'quality' => $options['quality'] ?? 85,
            ]);

        } catch (\Exception $e) {
            Log::error('Variant generation failed', ['error' => $e->getMessage()]);
            return null;
        }
    }

    /**
     * Generate BlurHash for an image
     */
    public function generateBlurhash(Media $media): ?string
    {
        try {
            $response = Http::timeout(30)
                ->post("{$this->serviceUrl}/blurhash", [
                    'key' => $media->path,
                ]);

            if ($response->successful()) {
                $blurhash = $response->json('blurhash');
                $media->update(['metadata->blurhash' => $blurhash]);
                return $blurhash;
            }

            return null;

        } catch (\Exception $e) {
            return null;
        }
    }

    /**
     * Create media record from Sharp result
     */
    protected function createMediaRecord(array $result, ?string $collection, array $options): Media
    {
        $original = $result['original'];

        $media = Media::create([
            'filename' => $original['filename'],
            'path' => $original['key'],
            'disk' => $this->disk,
            'url' => $original['url'],
            'mime_type' => "image/{$original['format']}",
            'type' => Media::TYPE_IMAGE,
            'size' => $original['size'],
            'width' => $original['width'],
            'height' => $original['height'],
            'aspect_ratio' => $original['height'] > 0
                ? round($original['width'] / $original['height'], 4)
                : null,
            'collection' => $collection,
            'source' => Media::SOURCE_UPLOAD,
            'processing_status' => Media::STATUS_COMPLETED,
            'is_optimized' => true,
            'is_processed' => true,
            'optimized_at' => now(),
            'alt_text' => $options['alt_text'] ?? null,
            'title' => $options['title'] ?? pathinfo($original['filename'], PATHINFO_FILENAME),
            'metadata' => [
                'blurhash' => $result['blurhash'] ?? null,
                'lqip' => $result['lqip'] ?? null,
                'sharp_id' => $result['id'],
                'optimization' => $result['stats'] ?? [],
            ],
            'thumbnail_url' => $result['thumbnail']['url'] ?? null,
            'thumbnail_path' => $result['thumbnail']['key'] ?? null,
            'uploaded_by' => auth()->id(),
        ]);

        // Create variant records
        foreach ($result['variants'] ?? [] as $variant) {
            MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => $variant['type'],
                'size_name' => $variant['sizeName'] ?? $variant['type'],
                'width' => $variant['width'],
                'height' => $variant['height'],
                'filename' => basename($variant['key']),
                'path' => $variant['key'],
                'disk' => $this->disk,
                'url' => $variant['url'],
                'mime_type' => $this->getMimeType($variant['key']),
                'format' => pathinfo($variant['key'], PATHINFO_EXTENSION),
                'size' => $variant['size'],
                'original_size' => $original['size'],
                'bytes_saved' => max(0, $original['size'] - $variant['size']),
                'is_retina' => ($variant['pixelDensity'] ?? 1) > 1,
                'pixel_density' => $variant['pixelDensity'] ?? 1,
            ]);
        }

        // Update variants JSON on media
        $media->update([
            'variants' => collect($result['variants'] ?? [])->map(fn($v) => [
                'type' => $v['type'],
                'sizeName' => $v['sizeName'] ?? $v['type'],
                'url' => $v['url'],
                'width' => $v['width'],
                'height' => $v['height'],
                'size' => $v['size'],
            ])->toArray(),
        ]);

        return $media;
    }

    /**
     * Update existing media with new variants
     */
    protected function updateMediaWithVariants(Media $media, array $result): void
    {
        // Delete existing variants
        MediaVariant::where('media_id', $media->id)->delete();

        // Create new variants
        foreach ($result['variants'] ?? [] as $variant) {
            MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => $variant['type'],
                'size_name' => $variant['sizeName'] ?? $variant['type'],
                'width' => $variant['width'],
                'height' => $variant['height'],
                'filename' => basename($variant['key']),
                'path' => $variant['key'],
                'disk' => $this->disk,
                'url' => $variant['url'],
                'mime_type' => $this->getMimeType($variant['key']),
                'format' => pathinfo($variant['key'], PATHINFO_EXTENSION),
                'size' => $variant['size'],
            ]);
        }

        // Update media record
        $media->update([
            'is_optimized' => true,
            'is_processed' => true,
            'processing_status' => Media::STATUS_COMPLETED,
            'optimized_at' => now(),
            'metadata' => array_merge($media->metadata ?? [], [
                'blurhash' => $result['blurhash'] ?? null,
                'lqip' => $result['lqip'] ?? null,
                'optimization' => $result['stats'] ?? [],
            ]),
            'thumbnail_url' => $result['thumbnail']['url'] ?? null,
            'thumbnail_path' => $result['thumbnail']['key'] ?? null,
            'variants' => collect($result['variants'] ?? [])->map(fn($v) => [
                'type' => $v['type'],
                'sizeName' => $v['sizeName'] ?? $v['type'],
                'url' => $v['url'],
                'width' => $v['width'],
                'height' => $v['height'],
                'size' => $v['size'],
            ])->toArray(),
        ]);
    }

    /**
     * Get MIME type from filename
     */
    protected function getMimeType(string $filename): string
    {
        $ext = strtolower(pathinfo($filename, PATHINFO_EXTENSION));

        return match ($ext) {
            'webp' => 'image/webp',
            'avif' => 'image/avif',
            'jpg', 'jpeg' => 'image/jpeg',
            'png' => 'image/png',
            'gif' => 'image/gif',
            'svg' => 'image/svg+xml',
            default => 'application/octet-stream',
        };
    }

    /**
     * Get responsive image srcset
     */
    public function getSrcset(Media $media, string $format = 'webp'): string
    {
        $variants = MediaVariant::where('media_id', $media->id)
            ->where('variant_type', 'responsive')
            ->where('format', $format)
            ->orderBy('width')
            ->get();

        return $variants->map(fn($v) => "{$v->url} {$v->width}w")->implode(', ');
    }

    /**
     * Get picture element sources for responsive images
     */
    public function getPictureSources(Media $media): array
    {
        $sources = [];

        // AVIF source (best compression)
        $avifVariants = MediaVariant::where('media_id', $media->id)
            ->where('format', 'avif')
            ->orderBy('width')
            ->get();

        if ($avifVariants->isNotEmpty()) {
            $sources[] = [
                'type' => 'image/avif',
                'srcset' => $avifVariants->map(fn($v) => "{$v->url} {$v->width}w")->implode(', '),
            ];
        }

        // WebP source
        $webpVariants = MediaVariant::where('media_id', $media->id)
            ->where('format', 'webp')
            ->orderBy('width')
            ->get();

        if ($webpVariants->isNotEmpty()) {
            $sources[] = [
                'type' => 'image/webp',
                'srcset' => $webpVariants->map(fn($v) => "{$v->url} {$v->width}w")->implode(', '),
            ];
        }

        return $sources;
    }
}
