<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use App\Models\Media;
use Illuminate\Support\Facades\DB;
use Intervention\Image\ImageManager;
use Intervention\Image\Drivers\Gd\Driver;

/**
 * Image Hotspot & Crop Service
 *
 * Implements Sanity-style image focal points and crop regions.
 *
 * Features:
 * - Hotspot (focal point) for smart cropping
 * - Named crop regions (square, 16:9, portrait, etc.)
 * - Responsive image URL generation
 * - BlurHash/LQIP generation
 */
class ImageHotspotService
{
    private ImageManager $imageManager;

    public function __construct()
    {
        $this->imageManager = new ImageManager(new Driver());
    }

    /**
     * Set image hotspot (focal point)
     */
    public function setHotspot(int $mediaId, float $x, float $y, ?string $name = null): array
    {
        // Validate coordinates are 0-1
        if ($x < 0 || $x > 1 || $y < 0 || $y > 1) {
            throw new \InvalidArgumentException("Hotspot coordinates must be between 0 and 1");
        }

        // Update media record
        DB::table('media')
            ->where('id', $mediaId)
            ->update([
                'hotspot_x' => $x,
                'hotspot_y' => $y,
                'updated_at' => now(),
            ]);

        // Also store in hotspots table for named hotspots
        if ($name) {
            DB::table('image_hotspots')->updateOrInsert(
                ['media_id' => $mediaId, 'name' => $name],
                [
                    'x' => $x,
                    'y' => $y,
                    'is_default' => false,
                    'updated_at' => now(),
                ]
            );
        } else {
            // Update or create default hotspot
            DB::table('image_hotspots')->updateOrInsert(
                ['media_id' => $mediaId, 'is_default' => true],
                [
                    'x' => $x,
                    'y' => $y,
                    'name' => 'default',
                    'updated_at' => now(),
                ]
            );
        }

        return $this->getHotspot($mediaId, $name);
    }

    /**
     * Get hotspot for media
     */
    public function getHotspot(int $mediaId, ?string $name = null): ?array
    {
        if ($name) {
            $hotspot = DB::table('image_hotspots')
                ->where('media_id', $mediaId)
                ->where('name', $name)
                ->first();
        } else {
            $hotspot = DB::table('image_hotspots')
                ->where('media_id', $mediaId)
                ->where('is_default', true)
                ->first();

            if (!$hotspot) {
                // Fallback to media table
                $media = DB::table('media')
                    ->where('id', $mediaId)
                    ->first(['hotspot_x', 'hotspot_y']);

                if ($media && $media->hotspot_x !== null) {
                    return [
                        'x' => (float) $media->hotspot_x,
                        'y' => (float) $media->hotspot_y,
                        'name' => 'default',
                        'isDefault' => true,
                    ];
                }

                // Center as default
                return [
                    'x' => 0.5,
                    'y' => 0.5,
                    'name' => 'center',
                    'isDefault' => true,
                ];
            }
        }

        if (!$hotspot) {
            return null;
        }

        return [
            'x' => (float) $hotspot->x,
            'y' => (float) $hotspot->y,
            'name' => $hotspot->name,
            'isDefault' => (bool) $hotspot->is_default,
        ];
    }

    /**
     * Set crop region
     */
    public function setCrop(
        int $mediaId,
        string $name,
        float $top,
        float $left,
        float $bottom,
        float $right,
        ?string $aspectRatio = null
    ): array {
        // Validate coordinates
        foreach ([$top, $left, $bottom, $right] as $coord) {
            if ($coord < 0 || $coord > 1) {
                throw new \InvalidArgumentException("Crop coordinates must be between 0 and 1");
            }
        }

        if ($right <= $left || $bottom <= $top) {
            throw new \InvalidArgumentException("Invalid crop region");
        }

        DB::table('image_crops')->updateOrInsert(
            ['media_id' => $mediaId, 'name' => $name],
            [
                'aspect_ratio' => $aspectRatio,
                'top' => $top,
                'left' => $left,
                'bottom' => $bottom,
                'right' => $right,
                'updated_at' => now(),
            ]
        );

        // Update media crop_data
        $allCrops = DB::table('image_crops')
            ->where('media_id', $mediaId)
            ->get()
            ->keyBy('name')
            ->map(fn($c) => [
                'top' => (float) $c->top,
                'left' => (float) $c->left,
                'bottom' => (float) $c->bottom,
                'right' => (float) $c->right,
                'aspectRatio' => $c->aspect_ratio,
            ])
            ->toArray();

        DB::table('media')
            ->where('id', $mediaId)
            ->update([
                'crop_data' => json_encode($allCrops),
                'updated_at' => now(),
            ]);

        return $this->getCrop($mediaId, $name);
    }

    /**
     * Get crop region
     */
    public function getCrop(int $mediaId, string $name): ?array
    {
        $crop = DB::table('image_crops')
            ->where('media_id', $mediaId)
            ->where('name', $name)
            ->first();

        if (!$crop) {
            return null;
        }

        return [
            'name' => $crop->name,
            'aspectRatio' => $crop->aspect_ratio,
            'top' => (float) $crop->top,
            'left' => (float) $crop->left,
            'bottom' => (float) $crop->bottom,
            'right' => (float) $crop->right,
        ];
    }

    /**
     * Get all crops for media
     */
    public function getAllCrops(int $mediaId): array
    {
        return DB::table('image_crops')
            ->where('media_id', $mediaId)
            ->get()
            ->map(fn($c) => [
                'name' => $c->name,
                'aspectRatio' => $c->aspect_ratio,
                'top' => (float) $c->top,
                'left' => (float) $c->left,
                'bottom' => (float) $c->bottom,
                'right' => (float) $c->right,
            ])
            ->toArray();
    }

    /**
     * Create common crop presets
     */
    public function createPresets(int $mediaId): array
    {
        $presets = [
            'square' => ['aspectRatio' => '1:1', 'top' => 0, 'left' => 0, 'bottom' => 1, 'right' => 1],
            'landscape' => ['aspectRatio' => '16:9', 'top' => 0.1, 'left' => 0, 'bottom' => 0.9, 'right' => 1],
            'portrait' => ['aspectRatio' => '3:4', 'top' => 0, 'left' => 0.1, 'bottom' => 1, 'right' => 0.9],
            'wide' => ['aspectRatio' => '21:9', 'top' => 0.2, 'left' => 0, 'bottom' => 0.8, 'right' => 1],
            'thumbnail' => ['aspectRatio' => '4:3', 'top' => 0.05, 'left' => 0.05, 'bottom' => 0.95, 'right' => 0.95],
        ];

        $created = [];

        foreach ($presets as $name => $config) {
            $this->setCrop(
                $mediaId,
                $name,
                $config['top'],
                $config['left'],
                $config['bottom'],
                $config['right'],
                $config['aspectRatio']
            );
            $created[] = $name;
        }

        return $created;
    }

    /**
     * Generate image URL with transformations
     */
    public function buildUrl(
        int $mediaId,
        array $options = []
    ): string {
        $media = DB::table('media')->find($mediaId);

        if (!$media) {
            throw new \InvalidArgumentException("Media not found: {$mediaId}");
        }

        $baseUrl = $media->url ?? $media->path;
        $params = [];

        // Width/Height
        if (isset($options['width'])) {
            $params['w'] = (int) $options['width'];
        }
        if (isset($options['height'])) {
            $params['h'] = (int) $options['height'];
        }

        // Fit mode
        if (isset($options['fit'])) {
            $params['fit'] = $options['fit']; // crop, clip, fill, max, min
        }

        // Apply crop
        if (isset($options['crop'])) {
            $crop = $this->getCrop($mediaId, $options['crop']);
            if ($crop) {
                $params['rect'] = sprintf(
                    '%s,%s,%s,%s',
                    $crop['left'],
                    $crop['top'],
                    $crop['right'] - $crop['left'],
                    $crop['bottom'] - $crop['top']
                );
            }
        }

        // Apply hotspot for smart crop
        if (($options['fit'] ?? null) === 'crop' && !isset($params['rect'])) {
            $hotspot = $this->getHotspot($mediaId, $options['hotspot'] ?? null);
            if ($hotspot) {
                $params['fp-x'] = $hotspot['x'];
                $params['fp-y'] = $hotspot['y'];
            }
        }

        // Format
        if (isset($options['format'])) {
            $params['fm'] = $options['format']; // webp, avif, jpg, png
        } else {
            // Auto format
            $params['auto'] = 'format';
        }

        // Quality
        if (isset($options['quality'])) {
            $params['q'] = (int) $options['quality'];
        }

        // Blur
        if (isset($options['blur'])) {
            $params['blur'] = (int) $options['blur'];
        }

        // DPR
        if (isset($options['dpr'])) {
            $params['dpr'] = (float) $options['dpr'];
        }

        // Build URL
        if (!empty($params)) {
            $query = http_build_query($params);
            return $baseUrl . (str_contains($baseUrl, '?') ? '&' : '?') . $query;
        }

        return $baseUrl;
    }

    /**
     * Generate BlurHash for image
     */
    public function generateBlurHash(int $mediaId): ?string
    {
        $media = Media::find($mediaId);

        if (!$media || !$media->path) {
            return null;
        }

        try {
            $path = storage_path('app/public/' . $media->path);

            if (!file_exists($path)) {
                return null;
            }

            // Load and resize image for blurhash calculation
            $image = $this->imageManager->read($path);
            $image->scale(width: 32);

            $width = $image->width();
            $height = $image->height();

            // Simple blurhash approximation (4x3 components)
            $components = [];
            for ($y = 0; $y < 3; $y++) {
                for ($x = 0; $x < 4; $x++) {
                    $sampleX = (int) (($x + 0.5) * $width / 4);
                    $sampleY = (int) (($y + 0.5) * $height / 3);
                    $color = $image->pickColor($sampleX, $sampleY);
                    $components[] = [
                        $color->red()->toInt(),
                        $color->green()->toInt(),
                        $color->blue()->toInt(),
                    ];
                }
            }

            // Encode as simple base64 color array
            $blurhash = base64_encode(json_encode([
                'w' => 4,
                'h' => 3,
                'c' => $components,
            ]));

            // Update media
            DB::table('media')
                ->where('id', $mediaId)
                ->update([
                    'blurhash' => $blurhash,
                    'updated_at' => now(),
                ]);

            return $blurhash;

        } catch (\Throwable $e) {
            return null;
        }
    }

    /**
     * Generate LQIP (Low Quality Image Placeholder)
     */
    public function generateLqip(int $mediaId): ?string
    {
        $media = Media::find($mediaId);

        if (!$media || !$media->path) {
            return null;
        }

        try {
            $path = storage_path('app/public/' . $media->path);

            if (!file_exists($path)) {
                return null;
            }

            // Create tiny version
            $image = $this->imageManager->read($path);
            $image->scale(width: 20);
            $image->blur(5);

            // Encode as base64 data URI
            $encoded = $image->toJpeg(quality: 20);
            $lqip = 'data:image/jpeg;base64,' . base64_encode($encoded->toString());

            // Update media
            DB::table('media')
                ->where('id', $mediaId)
                ->update([
                    'lqip' => $lqip,
                    'updated_at' => now(),
                ]);

            return $lqip;

        } catch (\Throwable $e) {
            return null;
        }
    }

    /**
     * Get responsive image srcset
     */
    public function getSrcSet(
        int $mediaId,
        array $widths = [320, 640, 960, 1280, 1920],
        array $options = []
    ): string {
        $srcset = [];

        foreach ($widths as $width) {
            $url = $this->buildUrl($mediaId, array_merge($options, ['width' => $width]));
            $srcset[] = "{$url} {$width}w";
        }

        return implode(', ', $srcset);
    }

    /**
     * Get picture element sources
     */
    public function getPictureSources(
        int $mediaId,
        array $breakpoints = []
    ): array {
        $defaults = [
            ['media' => '(max-width: 640px)', 'width' => 640],
            ['media' => '(max-width: 1024px)', 'width' => 1024],
            ['media' => '(min-width: 1025px)', 'width' => 1920],
        ];

        $breakpoints = $breakpoints ?: $defaults;
        $sources = [];

        foreach ($breakpoints as $bp) {
            $sources[] = [
                'media' => $bp['media'],
                'srcset' => $this->getSrcSet($mediaId, [$bp['width'], $bp['width'] * 2]),
                'type' => 'image/webp',
            ];
        }

        return $sources;
    }

    /**
     * Crop image using hotspot
     */
    public function smartCrop(
        int $mediaId,
        int $targetWidth,
        int $targetHeight
    ): array {
        $media = Media::find($mediaId);
        $hotspot = $this->getHotspot($mediaId);

        $sourceWidth = $media->width ?? 1000;
        $sourceHeight = $media->height ?? 1000;

        $targetRatio = $targetWidth / $targetHeight;
        $sourceRatio = $sourceWidth / $sourceHeight;

        if ($sourceRatio > $targetRatio) {
            // Source is wider - crop horizontally
            $newWidth = (int) ($sourceHeight * $targetRatio);
            $newHeight = $sourceHeight;

            // Center on hotspot
            $centerX = $hotspot['x'] * $sourceWidth;
            $left = max(0, $centerX - ($newWidth / 2));
            $left = min($left, $sourceWidth - $newWidth);

            return [
                'x' => (int) $left,
                'y' => 0,
                'width' => $newWidth,
                'height' => $newHeight,
            ];
        } else {
            // Source is taller - crop vertically
            $newWidth = $sourceWidth;
            $newHeight = (int) ($sourceWidth / $targetRatio);

            // Center on hotspot
            $centerY = $hotspot['y'] * $sourceHeight;
            $top = max(0, $centerY - ($newHeight / 2));
            $top = min($top, $sourceHeight - $newHeight);

            return [
                'x' => 0,
                'y' => (int) $top,
                'width' => $newWidth,
                'height' => $newHeight,
            ];
        }
    }
}
