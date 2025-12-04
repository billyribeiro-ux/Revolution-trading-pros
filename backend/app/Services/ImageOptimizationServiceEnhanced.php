<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Media;
use App\Models\MediaVariant;
use App\Events\ImageOptimizationProgress;
use App\Events\ImageOptimizationCompleted;
use App\Events\ImageOptimizationFailed;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Intervention\Image\ImageManager;
use Intervention\Image\Drivers\Imagick\Driver as ImagickDriver;
use Intervention\Image\Drivers\Gd\Driver as GdDriver;

/**
 * Enhanced Image Optimization Service
 *
 * Provides advanced image processing capabilities:
 * - HEIC/HEIF auto-conversion (iPhone support)
 * - Smart crop with face detection
 * - Animated GIF to WebP conversion
 * - SVG optimization
 * - On-the-fly URL transformations
 * - Duplicate detection
 * - Real-time progress via WebSocket
 */
class ImageOptimizationServiceEnhanced
{
    protected ImageManager $imageManager;
    protected array $config;
    protected bool $hasImagick;

    public function __construct()
    {
        $this->config = config('image-optimization', []);
        $this->hasImagick = extension_loaded('imagick');
        $this->initializeImageManager();
    }

    protected function initializeImageManager(): void
    {
        if ($this->hasImagick) {
            $this->imageManager = new ImageManager(new ImagickDriver());
        } else {
            $this->imageManager = new ImageManager(new GdDriver());
        }
    }

    // =========================================================================
    // HEIC/HEIF SUPPORT
    // =========================================================================

    /**
     * Check if file is HEIC/HEIF format
     */
    public function isHeicFormat(UploadedFile|string $file): bool
    {
        $mimeType = $file instanceof UploadedFile
            ? $file->getMimeType()
            : mime_content_type($file);

        return in_array($mimeType, [
            'image/heic',
            'image/heif',
            'image/heic-sequence',
            'image/heif-sequence',
        ]);
    }

    /**
     * Convert HEIC/HEIF to JPEG or WebP
     */
    public function convertHeic(string $sourcePath, ?string $outputFormat = null): string
    {
        $config = $this->config['heic'] ?? [];
        $outputFormat = $outputFormat ?? ($config['convert_to'] ?? 'jpeg');
        $quality = $config['quality'] ?? 90;

        if (!$this->hasImagick) {
            throw new \RuntimeException('Imagick extension required for HEIC conversion');
        }

        $imagick = new \Imagick($sourcePath);

        // Preserve EXIF if configured
        if ($config['preserve_exif'] ?? true) {
            $profiles = $imagick->getImageProfiles('*', false);
        }

        // Generate output path
        $outputPath = pathinfo($sourcePath, PATHINFO_DIRNAME) . '/'
            . pathinfo($sourcePath, PATHINFO_FILENAME) . '.' . $outputFormat;

        // Convert based on target format
        if ($outputFormat === 'webp') {
            $imagick->setImageFormat('webp');
            $imagick->setImageCompressionQuality($quality);
        } else {
            $imagick->setImageFormat('jpeg');
            $imagick->setImageCompressionQuality($quality);
            $imagick->setInterlaceScheme(\Imagick::INTERLACE_PLANE); // Progressive
        }

        // Restore EXIF profiles
        if (isset($profiles) && !empty($profiles)) {
            foreach ($profiles as $name => $profile) {
                if ($name === 'exif' || $name === 'icc') {
                    $imagick->setImageProfile($name, $profile);
                }
            }
        }

        $imagick->writeImage($outputPath);
        $imagick->destroy();

        Log::info('HEIC converted', [
            'source' => $sourcePath,
            'output' => $outputPath,
            'format' => $outputFormat,
        ]);

        return $outputPath;
    }

    /**
     * Process uploaded HEIC file
     */
    public function processHeicUpload(UploadedFile $file): UploadedFile
    {
        if (!$this->isHeicFormat($file)) {
            return $file;
        }

        $config = $this->config['heic'] ?? [];
        if (!($config['enabled'] ?? true) || !($config['auto_convert'] ?? true)) {
            return $file;
        }

        // Store temporarily
        $tempPath = $file->store('temp', 'local');
        $fullTempPath = Storage::disk('local')->path($tempPath);

        // Convert
        $convertedPath = $this->convertHeic($fullTempPath);

        // Clean up original temp file
        Storage::disk('local')->delete($tempPath);

        // Return new uploaded file instance
        return new UploadedFile(
            $convertedPath,
            pathinfo($file->getClientOriginalName(), PATHINFO_FILENAME) . '.' . ($config['convert_to'] ?? 'jpeg'),
            'image/' . ($config['convert_to'] ?? 'jpeg'),
            null,
            true
        );
    }

    // =========================================================================
    // SMART CROP (Face Detection)
    // =========================================================================

    /**
     * Detect faces in image and return bounding boxes
     */
    public function detectFaces(string $imagePath): array
    {
        $config = $this->config['smart_crop'] ?? [];

        if (!($config['enabled'] ?? true) || !($config['detect_faces'] ?? true)) {
            return [];
        }

        if (!$this->hasImagick) {
            return [];
        }

        $faces = [];

        try {
            // Use OpenCV via PHP extension if available
            if (extension_loaded('opencv')) {
                $faces = $this->detectFacesOpenCV($imagePath);
            }
            // Fallback: Use contrast/skin-tone detection heuristics
            else {
                $faces = $this->detectFacesHeuristic($imagePath);
            }
        } catch (\Exception $e) {
            Log::warning('Face detection failed', ['error' => $e->getMessage()]);
        }

        return $faces;
    }

    /**
     * Heuristic face detection based on skin tones and contrast
     */
    protected function detectFacesHeuristic(string $imagePath): array
    {
        $imagick = new \Imagick($imagePath);
        $width = $imagick->getImageWidth();
        $height = $imagick->getImageHeight();

        // Sample the image to find skin-tone regions
        $sampleWidth = min(100, $width);
        $sampleHeight = min(100, $height);

        $imagick->resizeImage($sampleWidth, $sampleHeight, \Imagick::FILTER_BOX, 1);

        $skinRegions = [];
        $iterator = $imagick->getPixelIterator();

        foreach ($iterator as $y => $row) {
            foreach ($row as $x => $pixel) {
                $color = $pixel->getColor(true);
                if ($this->isSkinTone($color)) {
                    $skinRegions[] = [
                        'x' => ($x / $sampleWidth) * $width,
                        'y' => ($y / $sampleHeight) * $height,
                    ];
                }
            }
            $iterator->syncIterator();
        }

        $imagick->destroy();

        // Cluster skin regions to find faces
        if (count($skinRegions) > 10) {
            $centroid = $this->calculateCentroid($skinRegions);
            return [
                [
                    'x' => max(0, $centroid['x'] - $width * 0.15),
                    'y' => max(0, $centroid['y'] - $height * 0.15),
                    'width' => $width * 0.3,
                    'height' => $height * 0.3,
                    'confidence' => 0.6,
                ],
            ];
        }

        return [];
    }

    /**
     * Check if color is skin-tone
     */
    protected function isSkinTone(array $color): bool
    {
        $r = $color['r'] * 255;
        $g = $color['g'] * 255;
        $b = $color['b'] * 255;

        // Common skin tone detection rules
        return $r > 95 && $g > 40 && $b > 20
            && max($r, $g, $b) - min($r, $g, $b) > 15
            && abs($r - $g) > 15
            && $r > $g && $r > $b;
    }

    /**
     * Calculate centroid of points
     */
    protected function calculateCentroid(array $points): array
    {
        $sumX = $sumY = 0;
        $count = count($points);

        foreach ($points as $point) {
            $sumX += $point['x'];
            $sumY += $point['y'];
        }

        return [
            'x' => $sumX / $count,
            'y' => $sumY / $count,
        ];
    }

    /**
     * Smart crop image around detected faces/subjects
     */
    public function smartCrop(string $imagePath, int $targetWidth, int $targetHeight): string
    {
        $config = $this->config['smart_crop'] ?? [];
        $faces = $this->detectFaces($imagePath);

        $imagick = new \Imagick($imagePath);
        $width = $imagick->getImageWidth();
        $height = $imagick->getImageHeight();

        // Calculate crop region
        if (!empty($faces)) {
            // Focus on the largest/most confident face
            $face = $faces[0];
            $padding = $config['face_padding'] ?? 0.2;

            $centerX = $face['x'] + ($face['width'] / 2);
            $centerY = $face['y'] + ($face['height'] / 2);

            // Add padding
            $cropWidth = min($width, $face['width'] * (1 + $padding * 2));
            $cropHeight = min($height, $face['height'] * (1 + $padding * 2));

            // Adjust to target aspect ratio
            $targetRatio = $targetWidth / $targetHeight;
            $currentRatio = $cropWidth / $cropHeight;

            if ($currentRatio > $targetRatio) {
                $cropWidth = $cropHeight * $targetRatio;
            } else {
                $cropHeight = $cropWidth / $targetRatio;
            }

            // Calculate crop position
            $cropX = max(0, min($width - $cropWidth, $centerX - $cropWidth / 2));
            $cropY = max(0, min($height - $cropHeight, $centerY - $cropHeight / 2));
        } else {
            // Fallback to center crop
            $cropX = max(0, ($width - $targetWidth) / 2);
            $cropY = max(0, ($height - $targetHeight) / 2);
            $cropWidth = min($width, $targetWidth);
            $cropHeight = min($height, $targetHeight);
        }

        // Perform crop
        $imagick->cropImage((int) $cropWidth, (int) $cropHeight, (int) $cropX, (int) $cropY);
        $imagick->resizeImage($targetWidth, $targetHeight, \Imagick::FILTER_LANCZOS, 1);

        // Save to temp file
        $outputPath = sys_get_temp_dir() . '/' . Str::uuid() . '.jpg';
        $imagick->writeImage($outputPath);
        $imagick->destroy();

        return $outputPath;
    }

    // =========================================================================
    // ANIMATED GIF TO WEBP
    // =========================================================================

    /**
     * Check if image is animated GIF
     */
    public function isAnimatedGif(string $imagePath): bool
    {
        if (!$this->hasImagick) {
            return false;
        }

        try {
            $imagick = new \Imagick($imagePath);
            $frameCount = $imagick->getNumberImages();
            $imagick->destroy();
            return $frameCount > 1;
        } catch (\Exception $e) {
            return false;
        }
    }

    /**
     * Convert animated GIF to animated WebP
     */
    public function convertAnimatedGifToWebp(string $gifPath, int $quality = 80): ?string
    {
        $config = $this->config['animated'] ?? [];

        if (!($config['enabled'] ?? true) || !($config['gif_to_webp'] ?? true)) {
            return null;
        }

        if (!$this->hasImagick) {
            Log::warning('Imagick required for animated GIF conversion');
            return null;
        }

        try {
            $imagick = new \Imagick($gifPath);
            $frameCount = $imagick->getNumberImages();

            // Check frame limit
            $maxFrames = $config['max_frames'] ?? 500;
            if ($frameCount > $maxFrames) {
                Log::info('Skipping animated conversion - too many frames', [
                    'frames' => $frameCount,
                    'max' => $maxFrames,
                ]);
                $imagick->destroy();
                return null;
            }

            // Coalesce frames for proper rendering
            $imagick = $imagick->coalesceImages();

            // Set WebP format
            $imagick->setFormat('webp');

            // Process each frame
            foreach ($imagick as $frame) {
                $frame->setImageFormat('webp');
                $frame->setImageCompressionQuality($quality);
            }

            // Optimize frames if enabled
            if ($config['optimize_frames'] ?? true) {
                $imagick = $imagick->optimizeImageLayers();
            }

            // Generate output path
            $outputPath = pathinfo($gifPath, PATHINFO_DIRNAME) . '/'
                . pathinfo($gifPath, PATHINFO_FILENAME) . '.webp';

            // Write animated WebP
            $imagick->writeImages($outputPath, true);
            $imagick->destroy();

            Log::info('Animated GIF converted to WebP', [
                'source' => $gifPath,
                'output' => $outputPath,
                'frames' => $frameCount,
            ]);

            return $outputPath;

        } catch (\Exception $e) {
            Log::error('Animated GIF conversion failed', ['error' => $e->getMessage()]);
            return null;
        }
    }

    // =========================================================================
    // SVG OPTIMIZATION
    // =========================================================================

    /**
     * Optimize SVG content
     */
    public function optimizeSvg(string $svgContent): string
    {
        $config = $this->config['svg'] ?? [];

        if (!($config['enabled'] ?? true)) {
            return $svgContent;
        }

        // Remove XML declaration
        $svgContent = preg_replace('/<\?xml[^>]*\?>/', '', $svgContent);

        // Remove comments if configured
        if ($config['remove_comments'] ?? true) {
            $svgContent = preg_replace('/<!--[\s\S]*?-->/', '', $svgContent);
        }

        // Remove metadata if configured
        if ($config['remove_metadata'] ?? true) {
            $svgContent = preg_replace('/<metadata[^>]*>[\s\S]*?<\/metadata>/', '', $svgContent);
        }

        // Remove empty attributes
        if ($config['remove_empty_attrs'] ?? true) {
            $svgContent = preg_replace('/\s+\w+=""/', '', $svgContent);
            $svgContent = preg_replace("/\s+\w+=''/", '', $svgContent);
        }

        // Convert colors to hex shorthand
        if ($config['convert_colors_to_hex'] ?? true) {
            $svgContent = preg_replace_callback(
                '/#([0-9a-fA-F])\1([0-9a-fA-F])\2([0-9a-fA-F])\3/',
                fn($m) => '#' . $m[1] . $m[2] . $m[3],
                $svgContent
            );
        }

        // Round decimal numbers
        $precision = $config['precision'] ?? 3;
        $svgContent = preg_replace_callback(
            '/(\d+\.\d{' . ($precision + 1) . ',})/',
            fn($m) => round((float) $m[1], $precision),
            $svgContent
        );

        // Remove whitespace between tags
        if ($config['minify'] ?? true) {
            $svgContent = preg_replace('/>\s+</', '><', $svgContent);
            $svgContent = preg_replace('/\s+/', ' ', $svgContent);
        }

        // Remove hidden elements
        if ($config['remove_hidden_elements'] ?? true) {
            $svgContent = preg_replace('/<[^>]+display\s*:\s*none[^>]*>[\s\S]*?<\/[^>]+>/', '', $svgContent);
            $svgContent = preg_replace('/<[^>]+visibility\s*:\s*hidden[^>]*>[\s\S]*?<\/[^>]+>/', '', $svgContent);
        }

        return trim($svgContent);
    }

    /**
     * Optimize SVG file
     */
    public function optimizeSvgFile(string $svgPath): string
    {
        $content = file_get_contents($svgPath);
        $optimized = $this->optimizeSvg($content);

        $outputPath = pathinfo($svgPath, PATHINFO_DIRNAME) . '/'
            . pathinfo($svgPath, PATHINFO_FILENAME) . '.optimized.svg';

        file_put_contents($outputPath, $optimized);

        $originalSize = filesize($svgPath);
        $optimizedSize = strlen($optimized);
        $savings = round((1 - $optimizedSize / $originalSize) * 100, 2);

        Log::info('SVG optimized', [
            'original_size' => $originalSize,
            'optimized_size' => $optimizedSize,
            'savings_percent' => $savings,
        ]);

        return $outputPath;
    }

    // =========================================================================
    // ON-THE-FLY TRANSFORMATIONS
    // =========================================================================

    /**
     * Parse transformation string from URL
     * Format: /media/{id}/w:800,h:600,fit:cover,q:80.webp
     */
    public function parseTransformations(string $transformString): array
    {
        $transforms = [];
        $parts = explode(',', $transformString);

        foreach ($parts as $part) {
            if (strpos($part, ':') !== false) {
                [$key, $value] = explode(':', $part, 2);
                $transforms[$key] = is_numeric($value) ? (int) $value : $value;
            }
        }

        return $transforms;
    }

    /**
     * Apply on-the-fly transformations
     */
    public function applyTransformations(Media $media, array $transforms): string
    {
        $config = $this->config['on_the_fly'] ?? [];

        if (!($config['enabled'] ?? true)) {
            throw new \RuntimeException('On-the-fly transformations are disabled');
        }

        // Validate transformations
        $allowed = $config['allowed_operations'] ?? [];
        foreach (array_keys($transforms) as $op) {
            $normalizedOp = $this->normalizeOperationName($op);
            if (!in_array($normalizedOp, $allowed)) {
                throw new \InvalidArgumentException("Operation not allowed: {$op}");
            }
        }

        // Generate cache key
        $cacheKey = 'transform:' . $media->id . ':' . md5(json_encode($transforms));

        // Check cache
        if ($config['cache_transformations'] ?? true) {
            $cached = Cache::get($cacheKey);
            if ($cached && Storage::disk($media->disk)->exists($cached)) {
                return $cached;
            }
        }

        // Load image
        $sourcePath = Storage::disk($media->disk)->path($media->path);
        $image = $this->imageManager->read($sourcePath);

        // Apply transformations
        $width = $transforms['w'] ?? $transforms['width'] ?? null;
        $height = $transforms['h'] ?? $transforms['height'] ?? null;
        $fit = $transforms['fit'] ?? 'contain';
        $quality = $transforms['q'] ?? $transforms['quality'] ?? 85;

        // Validate dimensions
        $maxWidth = $config['max_width'] ?? 4096;
        $maxHeight = $config['max_height'] ?? 4096;

        if ($width && $width > $maxWidth) {
            $width = $maxWidth;
        }
        if ($height && $height > $maxHeight) {
            $height = $maxHeight;
        }

        // Resize
        if ($width || $height) {
            switch ($fit) {
                case 'cover':
                    $image->cover($width ?? $height, $height ?? $width);
                    break;
                case 'contain':
                    $image->scale($width, $height);
                    break;
                case 'fill':
                    $image->resize($width, $height);
                    break;
                default:
                    $image->scale($width, $height);
            }
        }

        // Rotate
        if (isset($transforms['rotate'])) {
            $image->rotate((int) $transforms['rotate']);
        }

        // Flip
        if (isset($transforms['flip'])) {
            if ($transforms['flip'] === 'h' || $transforms['flip'] === 'horizontal') {
                $image->flip();
            } else {
                $image->flop();
            }
        }

        // Blur
        if (isset($transforms['blur'])) {
            $image->blur((int) $transforms['blur']);
        }

        // Grayscale
        if (isset($transforms['grayscale']) || isset($transforms['greyscale'])) {
            $image->greyscale();
        }

        // Determine output format
        $format = $transforms['format'] ?? $transforms['f'] ?? pathinfo($media->filename, PATHINFO_EXTENSION);

        // Generate output path
        $outputFilename = sprintf(
            '%s_%s.%s',
            pathinfo($media->filename, PATHINFO_FILENAME),
            substr(md5(json_encode($transforms)), 0, 8),
            $format
        );
        $outputPath = 'media/transforms/' . $outputFilename;

        // Encode and save
        $encoded = match ($format) {
            'webp' => $image->toWebp($quality),
            'avif' => $image->toAvif($quality),
            'png' => $image->toPng(),
            default => $image->toJpeg($quality),
        };

        Storage::disk($media->disk)->put($outputPath, $encoded);

        // Cache the path
        if ($config['cache_transformations'] ?? true) {
            Cache::put($cacheKey, $outputPath, $config['cache_ttl'] ?? 86400 * 30);
        }

        return $outputPath;
    }

    /**
     * Normalize operation name
     */
    protected function normalizeOperationName(string $op): string
    {
        $map = [
            'w' => 'resize',
            'h' => 'resize',
            'width' => 'resize',
            'height' => 'resize',
            'q' => 'quality',
            'f' => 'format',
            'format' => 'format',
            'greyscale' => 'grayscale',
        ];

        return $map[$op] ?? $op;
    }

    /**
     * Generate signed URL for transformation
     */
    public function generateSignedTransformUrl(Media $media, array $transforms): string
    {
        $config = $this->config['on_the_fly'] ?? [];
        $transformString = $this->buildTransformString($transforms);

        $url = "/media/{$media->id}/{$transformString}";

        if ($config['sign_urls'] ?? false) {
            $signature = hash_hmac(
                'sha256',
                $url,
                $config['signature_key'] ?? config('app.key')
            );
            $url .= '?s=' . substr($signature, 0, 16);
        }

        return $url;
    }

    /**
     * Build transform string from array
     */
    protected function buildTransformString(array $transforms): string
    {
        $parts = [];
        foreach ($transforms as $key => $value) {
            $parts[] = "{$key}:{$value}";
        }
        return implode(',', $parts);
    }

    // =========================================================================
    // DUPLICATE DETECTION
    // =========================================================================

    /**
     * Find duplicate images
     */
    public function findDuplicates(): array
    {
        $config = $this->config['duplicates'] ?? [];

        if (!($config['enabled'] ?? true)) {
            return [];
        }

        $method = $config['detection_method'] ?? 'hash';

        if ($method === 'hash' || $method === 'both') {
            return $this->findDuplicatesByHash();
        }

        return [];
    }

    /**
     * Find duplicates by file hash
     */
    protected function findDuplicatesByHash(): array
    {
        $duplicates = DB::table('media')
            ->select('hash', DB::raw('COUNT(*) as count'), DB::raw('GROUP_CONCAT(id) as ids'))
            ->whereNotNull('hash')
            ->where('type', 'image')
            ->groupBy('hash')
            ->having('count', '>', 1)
            ->get();

        $result = [];

        foreach ($duplicates as $group) {
            $ids = explode(',', $group->ids);
            $media = Media::whereIn('id', $ids)->get();

            $result[] = [
                'hash' => $group->hash,
                'count' => $group->count,
                'total_size' => $media->sum('size'),
                'potential_savings' => $media->sum('size') - $media->first()->size,
                'files' => $media->map(fn($m) => [
                    'id' => $m->id,
                    'filename' => $m->filename,
                    'path' => $m->path,
                    'size' => $m->size,
                    'created_at' => $m->created_at,
                    'usage_count' => $m->usage_count,
                ])->toArray(),
            ];
        }

        return $result;
    }

    /**
     * Merge duplicate images (keep one, update references)
     */
    public function mergeDuplicates(array $idsToMerge, int $keepId): bool
    {
        $config = $this->config['duplicates'] ?? [];
        $keepMedia = Media::findOrFail($keepId);

        DB::beginTransaction();

        try {
            foreach ($idsToMerge as $id) {
                if ($id == $keepId) {
                    continue;
                }

                $media = Media::find($id);
                if (!$media) {
                    continue;
                }

                // Update all references to point to the kept media
                DB::table('post_media')
                    ->where('media_id', $id)
                    ->update(['media_id' => $keepId]);

                DB::table('product_media')
                    ->where('media_id', $id)
                    ->update(['media_id' => $keepId]);

                // Update usage count
                $keepMedia->increment('usage_count', $media->usage_count);

                // Delete the duplicate file and record
                $media->deleteFile();
                $media->delete();
            }

            DB::commit();

            Log::info('Duplicates merged', [
                'kept_id' => $keepId,
                'merged_ids' => $idsToMerge,
            ]);

            return true;

        } catch (\Exception $e) {
            DB::rollBack();
            Log::error('Failed to merge duplicates', ['error' => $e->getMessage()]);
            throw $e;
        }
    }

    // =========================================================================
    // UNUSED IMAGE DETECTION
    // =========================================================================

    /**
     * Find unused images
     */
    public function findUnusedImages(int $daysUnused = 30): array
    {
        $cutoffDate = now()->subDays($daysUnused);

        return Media::where('type', 'image')
            ->where('usage_count', 0)
            ->where('created_at', '<', $cutoffDate)
            ->whereNull('deleted_at')
            ->get()
            ->map(fn($m) => [
                'id' => $m->id,
                'filename' => $m->filename,
                'path' => $m->path,
                'size' => $m->size,
                'created_at' => $m->created_at,
                'days_unused' => now()->diffInDays($m->created_at),
            ])
            ->toArray();
    }

    /**
     * Delete unused images
     */
    public function deleteUnusedImages(array $ids): array
    {
        $deleted = [];
        $failed = [];

        foreach ($ids as $id) {
            try {
                $media = Media::find($id);
                if ($media && $media->usage_count === 0) {
                    $media->deleteFile();
                    $media->delete();
                    $deleted[] = $id;
                }
            } catch (\Exception $e) {
                $failed[] = ['id' => $id, 'error' => $e->getMessage()];
            }
        }

        return [
            'deleted' => $deleted,
            'failed' => $failed,
            'deleted_count' => count($deleted),
            'failed_count' => count($failed),
        ];
    }

    // =========================================================================
    // REAL-TIME PROGRESS (WebSocket)
    // =========================================================================

    /**
     * Broadcast optimization progress
     */
    public function broadcastProgress(Media $media, int $progress, string $operation): void
    {
        $config = $this->config['realtime'] ?? [];

        if (!($config['enabled'] ?? true)) {
            return;
        }

        if (!($config['events']['job_progress'] ?? true)) {
            return;
        }

        event(new ImageOptimizationProgress($media, $progress, $operation));
    }

    /**
     * Broadcast optimization completed
     */
    public function broadcastCompleted(Media $media, array $results): void
    {
        $config = $this->config['realtime'] ?? [];

        if (!($config['enabled'] ?? true)) {
            return;
        }

        if (!($config['events']['job_completed'] ?? true)) {
            return;
        }

        event(new ImageOptimizationCompleted($media, $results));
    }

    /**
     * Broadcast optimization failed
     */
    public function broadcastFailed(Media $media, string $error): void
    {
        $config = $this->config['realtime'] ?? [];

        if (!($config['enabled'] ?? true)) {
            return;
        }

        if (!($config['events']['job_failed'] ?? true)) {
            return;
        }

        event(new ImageOptimizationFailed($media, $error));
    }

    // =========================================================================
    // BULK DOWNLOAD
    // =========================================================================

    /**
     * Create ZIP of optimized images
     */
    public function createBulkDownload(array $mediaIds, string $variant = 'webp'): string
    {
        $zipPath = storage_path('app/temp/') . 'media_export_' . now()->timestamp . '.zip';

        // Ensure temp directory exists
        if (!is_dir(dirname($zipPath))) {
            mkdir(dirname($zipPath), 0755, true);
        }

        $zip = new \ZipArchive();
        $zip->open($zipPath, \ZipArchive::CREATE | \ZipArchive::OVERWRITE);

        foreach ($mediaIds as $id) {
            $media = Media::find($id);
            if (!$media) {
                continue;
            }

            // Get the requested variant or fallback to original
            $variantRecord = MediaVariant::where('media_id', $id)
                ->where('format', $variant)
                ->first();

            if ($variantRecord) {
                $filePath = Storage::disk($media->disk)->path($variantRecord->path);
                $fileName = $variantRecord->filename;
            } else {
                $filePath = Storage::disk($media->disk)->path($media->path);
                $fileName = $media->filename;
            }

            if (file_exists($filePath)) {
                $zip->addFile($filePath, $fileName);
            }
        }

        $zip->close();

        return $zipPath;
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    /**
     * Get comprehensive statistics
     */
    public function getStatistics(): array
    {
        $totalImages = Media::where('type', 'image')->count();
        $optimizedImages = Media::where('type', 'image')->where('is_optimized', true)->count();
        $totalSize = Media::where('type', 'image')->sum('size');

        // Calculate total savings from variants
        $variants = MediaVariant::all();
        $totalSavings = $variants->sum('bytes_saved');

        // Find duplicates
        $duplicateGroups = $this->findDuplicates();
        $duplicateCount = array_sum(array_column($duplicateGroups, 'count')) - count($duplicateGroups);
        $duplicateSavingsPotential = array_sum(array_column($duplicateGroups, 'potential_savings'));

        // Find unused
        $unused = $this->findUnusedImages();
        $unusedSize = array_sum(array_column($unused, 'size'));

        return [
            'total_images' => $totalImages,
            'optimized_images' => $optimizedImages,
            'optimization_percentage' => $totalImages > 0
                ? round(($optimizedImages / $totalImages) * 100, 2)
                : 0,
            'total_storage' => $totalSize,
            'total_storage_human' => $this->formatBytes($totalSize),
            'total_savings' => $totalSavings,
            'total_savings_human' => $this->formatBytes($totalSavings),
            'savings_percentage' => $totalSize > 0
                ? round(($totalSavings / $totalSize) * 100, 2)
                : 0,
            'duplicates' => [
                'groups' => count($duplicateGroups),
                'files' => $duplicateCount,
                'potential_savings' => $duplicateSavingsPotential,
                'potential_savings_human' => $this->formatBytes($duplicateSavingsPotential),
            ],
            'unused' => [
                'count' => count($unused),
                'size' => $unusedSize,
                'size_human' => $this->formatBytes($unusedSize),
            ],
            'formats' => [
                'webp' => MediaVariant::where('format', 'webp')->count(),
                'avif' => MediaVariant::where('format', 'avif')->count(),
                'jpeg' => Media::where('mime_type', 'like', '%jpeg%')->count(),
                'png' => Media::where('mime_type', 'like', '%png%')->count(),
                'gif' => Media::where('mime_type', 'like', '%gif%')->count(),
                'svg' => Media::where('mime_type', 'like', '%svg%')->count(),
                'heic' => Media::where('mime_type', 'like', '%heic%')->count(),
            ],
        ];
    }

    /**
     * Format bytes to human readable
     */
    protected function formatBytes(int $bytes, int $precision = 2): string
    {
        $units = ['B', 'KB', 'MB', 'GB', 'TB'];
        $bytes = max($bytes, 0);
        $pow = floor(($bytes ? log($bytes) : 0) / log(1024));
        $pow = min($pow, count($units) - 1);
        $bytes /= pow(1024, $pow);

        return round($bytes, $precision) . ' ' . $units[$pow];
    }
}
