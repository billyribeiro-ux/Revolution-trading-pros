<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Media;
use App\Models\MediaVariant;
use App\Models\ImageOptimizationJob;
use App\Models\ImageOptimizationPreset;
use App\Jobs\ProcessImageOptimization;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Intervention\Image\ImageManager;
use Intervention\Image\Drivers\Gd\Driver as GdDriver;
use Intervention\Image\Drivers\Imagick\Driver as ImagickDriver;
use kornrunner\Blurhash\Blurhash;

/**
 * ImageOptimizationService
 *
 * Enterprise-grade image optimization engine providing:
 * - WebP/AVIF conversion
 * - Responsive variant generation
 * - Thumbnail creation
 * - LQIP/BlurHash placeholder generation
 * - Intelligent compression
 * - Metadata extraction/handling
 */
class ImageOptimizationService
{
    protected ImageManager $imageManager;
    protected array $config;

    public function __construct()
    {
        $this->config = config('image-optimization', []);
        $this->initializeImageManager();
    }

    /**
     * Initialize Intervention Image manager
     */
    protected function initializeImageManager(): void
    {
        // Prefer Imagick for better quality and format support
        if (extension_loaded('imagick')) {
            $this->imageManager = new ImageManager(new ImagickDriver());
        } else {
            $this->imageManager = new ImageManager(new GdDriver());
        }
    }

    /**
     * Upload and process a new image
     */
    public function upload(
        UploadedFile $file,
        ?string $collection = null,
        ?string $presetSlug = null,
        array $options = []
    ): Media {
        // Validate file
        $this->validateFile($file);

        // Generate unique filename and path
        $filename = $this->generateFilename($file);
        $path = $this->generatePath($collection);
        $fullPath = $path . '/' . $filename;

        // Store original file
        $disk = $options['disk'] ?? $this->config['storage']['disk'] ?? 'public';
        Storage::disk($disk)->putFileAs($path, $file, $filename);

        // Create media record
        $media = Media::create([
            'filename' => $file->getClientOriginalName(),
            'path' => $fullPath,
            'disk' => $disk,
            'mime_type' => $file->getMimeType(),
            'type' => Media::TYPE_IMAGE,
            'size' => $file->getSize(),
            'collection' => $collection,
            'source' => Media::SOURCE_UPLOAD,
            'processing_status' => Media::STATUS_PENDING,
            'alt_text' => $options['alt_text'] ?? null,
            'title' => $options['title'] ?? pathinfo($file->getClientOriginalName(), PATHINFO_FILENAME),
        ]);

        // Extract metadata
        $this->extractAndUpdateMetadata($media);

        // Queue optimization job
        if ($options['process_immediately'] ?? false) {
            $this->processImage($media, $presetSlug);
        } else {
            $this->queueOptimization($media, $presetSlug, $options['priority'] ?? 5);
        }

        return $media->fresh();
    }

    /**
     * Process image optimization
     */
    public function processImage(Media $media, ?string $presetSlug = null): Media
    {
        $preset = $presetSlug
            ? ImageOptimizationPreset::getBySlug($presetSlug)
            : ImageOptimizationPreset::getDefault();

        if (!$preset) {
            $preset = $this->getDefaultPresetConfig();
        }

        try {
            $media->update(['processing_status' => Media::STATUS_PROCESSING]);
            $originalSize = $media->size;
            $variants = [];
            $totalOptimizedSize = 0;

            // Load source image
            $sourcePath = Storage::disk($media->disk)->path($media->path);
            $image = $this->imageManager->read($sourcePath);

            // 1. Generate WebP version
            if ($this->shouldConvertToWebp($preset)) {
                $webpVariant = $this->generateWebpVariant($media, $image, $preset);
                if ($webpVariant) {
                    $variants[] = $webpVariant;
                    $totalOptimizedSize += $webpVariant->size;
                }
            }

            // 2. Generate AVIF version
            if ($this->shouldConvertToAvif($preset)) {
                $avifVariant = $this->generateAvifVariant($media, $image, $preset);
                if ($avifVariant) {
                    $variants[] = $avifVariant;
                }
            }

            // 3. Generate responsive sizes
            $responsiveVariants = $this->generateResponsiveSizes($media, $image, $preset);
            $variants = array_merge($variants, $responsiveVariants);

            // 4. Generate thumbnail
            $thumbnail = $this->generateThumbnail($media, $image, $preset);
            if ($thumbnail) {
                $variants[] = $thumbnail;
                $media->update([
                    'thumbnail_path' => $thumbnail->path,
                    'thumbnail_url' => $thumbnail->url,
                ]);
            }

            // 5. Generate retina variants
            if ($this->shouldGenerateRetina($preset)) {
                $retinaVariants = $this->generateRetinaVariants($media, $image, $preset);
                $variants = array_merge($variants, $retinaVariants);
            }

            // 6. Generate placeholders
            $placeholders = $this->generatePlaceholders($media, $image, $preset);

            // 7. Compress original if needed
            $compressedSize = $this->compressOriginal($media, $image, $preset);

            // Calculate total savings
            $totalVariantSize = collect($variants)->sum('size');
            $savingsPercent = $originalSize > 0
                ? round((1 - ($compressedSize / $originalSize)) * 100, 2)
                : 0;

            // Update media record
            $media->update([
                'is_optimized' => true,
                'is_processed' => true,
                'processing_status' => Media::STATUS_COMPLETED,
                'optimized_at' => now(),
                'variants' => collect($variants)->map->toVariantArray()->toArray(),
                'metadata' => array_merge($media->metadata ?? [], [
                    'lqip' => $placeholders['lqip'] ?? null,
                    'blurhash' => $placeholders['blurhash'] ?? null,
                    'optimization' => [
                        'original_size' => $originalSize,
                        'optimized_size' => $compressedSize,
                        'savings_percent' => $savingsPercent,
                        'variants_count' => count($variants),
                    ],
                ]),
            ]);

            Log::info('Image optimization completed', [
                'media_id' => $media->id,
                'original_size' => $originalSize,
                'optimized_size' => $compressedSize,
                'savings_percent' => $savingsPercent,
                'variants_count' => count($variants),
            ]);

            return $media->fresh();

        } catch (\Exception $e) {
            $media->markProcessingFailed($e->getMessage());
            Log::error('Image optimization failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);
            throw $e;
        }
    }

    /**
     * Queue optimization job
     */
    public function queueOptimization(
        Media $media,
        ?string $presetSlug = null,
        int $priority = 5,
        array $operations = []
    ): ImageOptimizationJob {
        $preset = $presetSlug ? ImageOptimizationPreset::getBySlug($presetSlug) : null;

        $job = ImageOptimizationJob::create([
            'media_id' => $media->id,
            'preset_id' => $preset?->id,
            'priority' => $priority,
            'operations' => $operations ?: $this->getDefaultOperations(),
            'original_size' => $media->size,
        ]);

        // Dispatch background job
        ProcessImageOptimization::dispatch($job)
            ->onQueue($this->config['queue']['queue_name'] ?? 'image-optimization');

        return $job;
    }

    /**
     * Bulk optimize multiple images
     */
    public function bulkOptimize(
        array $mediaIds,
        ?string $presetSlug = null,
        int $priority = 5
    ): array {
        $jobs = [];

        foreach ($mediaIds as $mediaId) {
            $media = Media::find($mediaId);
            if ($media && $media->isImage() && !$media->is_optimized) {
                $jobs[] = $this->queueOptimization($media, $presetSlug, $priority);
            }
        }

        return $jobs;
    }

    /**
     * Regenerate all variants for a media
     */
    public function regenerateVariants(Media $media, ?string $presetSlug = null): Media
    {
        // Delete existing variants
        MediaVariant::where('media_id', $media->id)->each(function ($variant) {
            $variant->deleteFile();
            $variant->delete();
        });

        // Reset optimization status
        $media->update([
            'is_optimized' => false,
            'processing_status' => Media::STATUS_PENDING,
            'variants' => [],
        ]);

        // Re-process
        return $this->processImage($media, $presetSlug);
    }

    /**
     * Generate WebP variant
     */
    protected function generateWebpVariant(Media $media, $image, $preset): ?MediaVariant
    {
        try {
            $quality = $this->getPresetQuality($preset, 'webp');
            $filename = pathinfo($media->filename, PATHINFO_FILENAME) . '.webp';
            $path = $this->getVariantPath($media, 'webp', $filename);

            // Encode to WebP
            $encoded = $image->toWebp($quality);
            Storage::disk($media->disk)->put($path, $encoded);

            return MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => MediaVariant::TYPE_WEBP,
                'width' => $image->width(),
                'height' => $image->height(),
                'filename' => $filename,
                'path' => $path,
                'disk' => $media->disk,
                'url' => Storage::disk($media->disk)->url($path),
                'mime_type' => 'image/webp',
                'format' => 'webp',
                'size' => Storage::disk($media->disk)->size($path),
                'quality' => $quality,
                'original_size' => $media->size,
            ]);
        } catch (\Exception $e) {
            Log::warning('WebP conversion failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Generate AVIF variant
     */
    protected function generateAvifVariant(Media $media, $image, $preset): ?MediaVariant
    {
        try {
            // Check for AVIF support
            if (!function_exists('imageavif') && !extension_loaded('imagick')) {
                return null;
            }

            $quality = $this->getPresetQuality($preset, 'avif');
            $filename = pathinfo($media->filename, PATHINFO_FILENAME) . '.avif';
            $path = $this->getVariantPath($media, 'avif', $filename);

            $encoded = $image->toAvif($quality);
            Storage::disk($media->disk)->put($path, $encoded);

            return MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => MediaVariant::TYPE_AVIF,
                'width' => $image->width(),
                'height' => $image->height(),
                'filename' => $filename,
                'path' => $path,
                'disk' => $media->disk,
                'url' => Storage::disk($media->disk)->url($path),
                'mime_type' => 'image/avif',
                'format' => 'avif',
                'size' => Storage::disk($media->disk)->size($path),
                'quality' => $quality,
                'original_size' => $media->size,
            ]);
        } catch (\Exception $e) {
            Log::warning('AVIF conversion failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Generate responsive size variants
     */
    protected function generateResponsiveSizes(Media $media, $image, $preset): array
    {
        $variants = [];
        $sizes = $this->getResponsiveSizes($preset);
        $originalWidth = $image->width();

        foreach ($sizes as $sizeName => $width) {
            // Skip if larger than original
            if ($width >= $originalWidth) {
                continue;
            }

            try {
                $resized = clone $image;
                $resized->scaleDown(width: $width);

                $filename = pathinfo($media->filename, PATHINFO_FILENAME) . "-{$sizeName}.webp";
                $path = $this->getVariantPath($media, "responsive/{$sizeName}", $filename);

                $quality = $this->getPresetQuality($preset, 'webp');
                $encoded = $resized->toWebp($quality);
                Storage::disk($media->disk)->put($path, $encoded);

                $variants[] = MediaVariant::create([
                    'media_id' => $media->id,
                    'variant_type' => MediaVariant::TYPE_RESPONSIVE,
                    'size_name' => $sizeName,
                    'width' => $resized->width(),
                    'height' => $resized->height(),
                    'filename' => $filename,
                    'path' => $path,
                    'disk' => $media->disk,
                    'url' => Storage::disk($media->disk)->url($path),
                    'mime_type' => 'image/webp',
                    'format' => 'webp',
                    'size' => Storage::disk($media->disk)->size($path),
                    'quality' => $quality,
                    'original_size' => $media->size,
                ]);
            } catch (\Exception $e) {
                Log::warning("Responsive size {$sizeName} generation failed", [
                    'media_id' => $media->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $variants;
    }

    /**
     * Generate thumbnail
     */
    protected function generateThumbnail(Media $media, $image, $preset): ?MediaVariant
    {
        try {
            $config = $this->config['thumbnails'] ?? [];
            $width = $config['width'] ?? 300;
            $height = $config['height'] ?? 300;

            $thumbnail = clone $image;
            $thumbnail->cover($width, $height);

            $filename = pathinfo($media->filename, PATHINFO_FILENAME) . '-thumb.webp';
            $path = $this->getVariantPath($media, 'thumbnails', $filename);

            $quality = $config['quality'] ?? 80;
            $encoded = $thumbnail->toWebp($quality);
            Storage::disk($media->disk)->put($path, $encoded);

            return MediaVariant::create([
                'media_id' => $media->id,
                'variant_type' => MediaVariant::TYPE_THUMBNAIL,
                'size_name' => 'thumbnail',
                'width' => $thumbnail->width(),
                'height' => $thumbnail->height(),
                'filename' => $filename,
                'path' => $path,
                'disk' => $media->disk,
                'url' => Storage::disk($media->disk)->url($path),
                'mime_type' => 'image/webp',
                'format' => 'webp',
                'size' => Storage::disk($media->disk)->size($path),
                'quality' => $quality,
                'original_size' => $media->size,
            ]);
        } catch (\Exception $e) {
            Log::warning('Thumbnail generation failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
            return null;
        }
    }

    /**
     * Generate retina variants
     */
    protected function generateRetinaVariants(Media $media, $image, $preset): array
    {
        $variants = [];
        $sizes = $this->getResponsiveSizes($preset);
        $originalWidth = $image->width();
        $maxRetinaWidth = $this->config['retina']['max_width'] ?? 3840;

        foreach ($sizes as $sizeName => $width) {
            $retinaWidth = $width * 2;

            // Skip if larger than original or max retina width
            if ($retinaWidth >= $originalWidth || $retinaWidth > $maxRetinaWidth) {
                continue;
            }

            try {
                $resized = clone $image;
                $resized->scaleDown(width: $retinaWidth);

                $filename = pathinfo($media->filename, PATHINFO_FILENAME) . "-{$sizeName}@2x.webp";
                $path = $this->getVariantPath($media, "responsive/{$sizeName}", $filename);

                $quality = $this->getPresetQuality($preset, 'webp');
                $encoded = $resized->toWebp($quality);
                Storage::disk($media->disk)->put($path, $encoded);

                $variants[] = MediaVariant::create([
                    'media_id' => $media->id,
                    'variant_type' => MediaVariant::TYPE_RETINA,
                    'size_name' => $sizeName,
                    'width' => $resized->width(),
                    'height' => $resized->height(),
                    'filename' => $filename,
                    'path' => $path,
                    'disk' => $media->disk,
                    'url' => Storage::disk($media->disk)->url($path),
                    'mime_type' => 'image/webp',
                    'format' => 'webp',
                    'size' => Storage::disk($media->disk)->size($path),
                    'quality' => $quality,
                    'original_size' => $media->size,
                    'is_retina' => true,
                    'pixel_density' => 2,
                ]);
            } catch (\Exception $e) {
                Log::warning("Retina variant {$sizeName}@2x generation failed", [
                    'media_id' => $media->id,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return $variants;
    }

    /**
     * Generate placeholders (LQIP and BlurHash)
     */
    protected function generatePlaceholders(Media $media, $image, $preset): array
    {
        $placeholders = [];

        // Generate LQIP
        if ($this->shouldGenerateLqip($preset)) {
            try {
                $lqipConfig = $this->config['placeholders']['lqip'] ?? [];
                $lqipWidth = $lqipConfig['width'] ?? 32;
                $lqipQuality = $lqipConfig['quality'] ?? 20;

                $lqip = clone $image;
                $lqip->scaleDown(width: $lqipWidth);
                $lqip->blur(10);

                $encoded = $lqip->toJpeg($lqipQuality);
                $placeholders['lqip'] = 'data:image/jpeg;base64,' . base64_encode($encoded->toString());
            } catch (\Exception $e) {
                Log::warning('LQIP generation failed', ['media_id' => $media->id]);
            }
        }

        // Generate BlurHash
        if ($this->shouldGenerateBlurhash($preset)) {
            try {
                $placeholders['blurhash'] = $this->generateBlurhash($image);
            } catch (\Exception $e) {
                Log::warning('BlurHash generation failed', ['media_id' => $media->id]);
            }
        }

        return $placeholders;
    }

    /**
     * Generate BlurHash from image
     */
    protected function generateBlurhash($image): ?string
    {
        try {
            $config = $this->config['placeholders']['blurhash'] ?? [];
            $componentsX = $config['components_x'] ?? 4;
            $componentsY = $config['components_y'] ?? 3;

            // Resize for faster processing
            $small = clone $image;
            $small->scaleDown(width: 100);

            $width = $small->width();
            $height = $small->height();
            $pixels = [];

            // Extract pixels using GD or Imagick
            $gdImage = imagecreatefromstring($small->toJpeg()->toString());
            if (!$gdImage) {
                return null;
            }

            for ($y = 0; $y < $height; $y++) {
                $row = [];
                for ($x = 0; $x < $width; $x++) {
                    $rgb = imagecolorat($gdImage, $x, $y);
                    $row[] = [
                        ($rgb >> 16) & 0xFF,
                        ($rgb >> 8) & 0xFF,
                        $rgb & 0xFF,
                    ];
                }
                $pixels[] = $row;
            }
            imagedestroy($gdImage);

            return Blurhash::encode($pixels, $componentsX, $componentsY);
        } catch (\Exception $e) {
            return null;
        }
    }

    /**
     * Compress original image
     */
    protected function compressOriginal(Media $media, $image, $preset): int
    {
        // Get appropriate quality based on format
        $format = strtolower(pathinfo($media->filename, PATHINFO_EXTENSION));
        $quality = $this->getPresetQuality($preset, $format);

        try {
            $encoded = match ($format) {
                'jpg', 'jpeg' => $image->toJpeg($quality),
                'png' => $image->toPng(),
                'webp' => $image->toWebp($quality),
                default => $image->toJpeg($quality),
            };

            $newSize = strlen($encoded->toString());

            // Only overwrite if smaller
            if ($newSize < $media->size) {
                Storage::disk($media->disk)->put($media->path, $encoded);
                $media->update(['size' => $newSize]);
                return $newSize;
            }
        } catch (\Exception $e) {
            Log::warning('Original compression failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
        }

        return $media->size;
    }

    /**
     * Extract and update metadata
     */
    protected function extractAndUpdateMetadata(Media $media): void
    {
        try {
            $sourcePath = Storage::disk($media->disk)->path($media->path);
            $image = $this->imageManager->read($sourcePath);

            $media->update([
                'width' => $image->width(),
                'height' => $image->height(),
                'aspect_ratio' => $image->height() > 0
                    ? round($image->width() / $image->height(), 4)
                    : null,
            ]);

            // Extract EXIF if available
            if (in_array($media->mime_type, ['image/jpeg', 'image/jpg', 'image/tiff'])) {
                $exif = @exif_read_data($sourcePath);
                if ($exif) {
                    $media->update([
                        'exif' => [
                            'camera' => $exif['Model'] ?? null,
                            'date_taken' => $exif['DateTime'] ?? null,
                            'aperture' => $exif['COMPUTED']['ApertureFNumber'] ?? null,
                            'iso' => $exif['ISOSpeedRatings'] ?? null,
                            'focal_length' => $exif['FocalLength'] ?? null,
                            'exposure_time' => $exif['ExposureTime'] ?? null,
                        ],
                    ]);
                }
            }
        } catch (\Exception $e) {
            Log::warning('Metadata extraction failed', [
                'media_id' => $media->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Validate uploaded file
     */
    protected function validateFile(UploadedFile $file): void
    {
        $limits = $this->config['limits'] ?? [];
        $maxSize = ($limits['max_upload_size_mb'] ?? 50) * 1024 * 1024;

        if ($file->getSize() > $maxSize) {
            throw new \InvalidArgumentException(
                "File size exceeds maximum allowed ({$limits['max_upload_size_mb']}MB)"
            );
        }

        $supportedFormats = $this->config['supported_formats']['input'] ?? [];
        $extension = strtolower($file->getClientOriginalExtension());

        if (!in_array($extension, $supportedFormats)) {
            throw new \InvalidArgumentException(
                "Unsupported file format: {$extension}"
            );
        }
    }

    /**
     * Generate unique filename
     */
    protected function generateFilename(UploadedFile $file): string
    {
        $extension = $file->getClientOriginalExtension();
        return Str::uuid() . '.' . $extension;
    }

    /**
     * Generate storage path
     */
    protected function generatePath(?string $collection = null): string
    {
        $basePath = $this->config['storage']['originals_path'] ?? 'media/originals';
        $datePath = now()->format('Y/m');

        if ($collection) {
            return "{$basePath}/{$collection}/{$datePath}";
        }

        return "{$basePath}/{$datePath}";
    }

    /**
     * Get variant path
     */
    protected function getVariantPath(Media $media, string $type, string $filename): string
    {
        $basePath = $this->config['storage']['variants_path'] ?? 'media/variants';
        return "{$basePath}/{$media->id}/{$type}/{$filename}";
    }

    /**
     * Get responsive sizes from preset or config
     */
    protected function getResponsiveSizes($preset): array
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->getResponsiveSizesArray();
        }

        return $this->config['responsive_sizes'] ?? [
            'xs' => 320,
            'sm' => 640,
            'md' => 768,
            'lg' => 1024,
            'xl' => 1280,
            '2xl' => 1920,
        ];
    }

    /**
     * Get quality for format from preset
     */
    protected function getPresetQuality($preset, string $format): int
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->getQualityForFormat($format);
        }

        if (is_array($preset)) {
            $key = "quality_{$format}";
            return $preset[$key] ?? 85;
        }

        return $this->config['formats'][$format]['quality']
            ?? $this->config['formats']['jpeg']['quality']
            ?? 85;
    }

    /**
     * Check if should convert to WebP
     */
    protected function shouldConvertToWebp($preset): bool
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->convert_to_webp;
        }

        if (is_array($preset)) {
            return $preset['convert_to_webp'] ?? true;
        }

        return $this->config['formats']['webp']['enabled'] ?? true;
    }

    /**
     * Check if should convert to AVIF
     */
    protected function shouldConvertToAvif($preset): bool
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->convert_to_avif;
        }

        if (is_array($preset)) {
            return $preset['convert_to_avif'] ?? false;
        }

        return $this->config['formats']['avif']['enabled'] ?? false;
    }

    /**
     * Check if should generate retina
     */
    protected function shouldGenerateRetina($preset): bool
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->generate_retina;
        }

        if (is_array($preset)) {
            return $preset['generate_retina'] ?? true;
        }

        return $this->config['retina']['enabled'] ?? true;
    }

    /**
     * Check if should generate LQIP
     */
    protected function shouldGenerateLqip($preset): bool
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->generate_lqip;
        }

        if (is_array($preset)) {
            return $preset['generate_lqip'] ?? true;
        }

        return $this->config['placeholders']['lqip']['enabled'] ?? true;
    }

    /**
     * Check if should generate BlurHash
     */
    protected function shouldGenerateBlurhash($preset): bool
    {
        if ($preset instanceof ImageOptimizationPreset) {
            return $preset->generate_blurhash;
        }

        if (is_array($preset)) {
            return $preset['generate_blurhash'] ?? true;
        }

        return $this->config['placeholders']['blurhash']['enabled'] ?? true;
    }

    /**
     * Get default preset config
     */
    protected function getDefaultPresetConfig(): array
    {
        return $this->config['presets']['balanced'] ?? [
            'quality_webp' => 85,
            'quality_avif' => 80,
            'quality_jpeg' => 85,
            'quality_png' => 90,
            'convert_to_webp' => true,
            'convert_to_avif' => false,
            'generate_retina' => true,
            'generate_lqip' => true,
            'generate_blurhash' => true,
        ];
    }

    /**
     * Get default operations
     */
    protected function getDefaultOperations(): array
    {
        return [
            ImageOptimizationJob::OP_CONVERT_WEBP,
            ImageOptimizationJob::OP_GENERATE_THUMBNAIL,
            ImageOptimizationJob::OP_GENERATE_RESPONSIVE,
            ImageOptimizationJob::OP_GENERATE_LQIP,
            ImageOptimizationJob::OP_GENERATE_BLURHASH,
            ImageOptimizationJob::OP_COMPRESS,
        ];
    }

    /**
     * Get optimization statistics
     */
    public function getStatistics(): array
    {
        return [
            'total_images' => Media::images()->count(),
            'optimized_images' => Media::images()->where('is_optimized', true)->count(),
            'pending_optimization' => Media::needsOptimization()->count(),
            'total_storage' => Media::sum('size'),
            'total_variants' => MediaVariant::count(),
            'total_savings_bytes' => MediaVariant::sum('bytes_saved'),
            'job_stats' => ImageOptimizationJob::getStats(),
        ];
    }
}
