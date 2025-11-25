<?php

declare(strict_types=1);

namespace Database\Seeders;

use App\Models\ImageOptimizationPreset;
use Illuminate\Database\Seeder;

/**
 * ImageOptimizationSeeder
 *
 * Seeds default optimization presets for the image optimization engine.
 */
class ImageOptimizationSeeder extends Seeder
{
    public function run(): void
    {
        $presets = [
            [
                'name' => 'Maximum Quality',
                'slug' => 'maximum',
                'description' => 'Best quality with minimal compression. Ideal for photography portfolios and high-end product images.',
                'quality_webp' => 95,
                'quality_avif' => 90,
                'quality_jpeg' => 95,
                'quality_png' => 95,
                'compression_mode' => 'lossless',
                'convert_to_webp' => true,
                'convert_to_avif' => true,
                'generate_retina' => true,
                'strip_exif' => false,
                'generate_lqip' => true,
                'generate_blurhash' => true,
                'responsive_sizes' => ['sm' => 640, 'md' => 1024, 'lg' => 1920, 'xl' => 2560],
                'is_default' => false,
            ],
            [
                'name' => 'Balanced',
                'slug' => 'balanced',
                'description' => 'Optimal balance between quality and file size. Recommended for most use cases.',
                'quality_webp' => 85,
                'quality_avif' => 80,
                'quality_jpeg' => 85,
                'quality_png' => 90,
                'compression_mode' => 'auto',
                'convert_to_webp' => true,
                'convert_to_avif' => false,
                'generate_retina' => true,
                'strip_exif' => false,
                'generate_lqip' => true,
                'generate_blurhash' => true,
                'responsive_sizes' => ['xs' => 320, 'sm' => 640, 'md' => 768, 'lg' => 1024, 'xl' => 1280],
                'is_default' => true,
            ],
            [
                'name' => 'Performance',
                'slug' => 'performance',
                'description' => 'Aggressive compression for maximum performance. Best for blogs and content sites.',
                'quality_webp' => 75,
                'quality_avif' => 70,
                'quality_jpeg' => 75,
                'quality_png' => 80,
                'compression_mode' => 'lossy',
                'convert_to_webp' => true,
                'convert_to_avif' => false,
                'generate_retina' => false,
                'strip_exif' => true,
                'generate_lqip' => true,
                'generate_blurhash' => true,
                'responsive_sizes' => ['sm' => 640, 'md' => 1024, 'lg' => 1280],
                'is_default' => false,
            ],
            [
                'name' => 'Thumbnail',
                'slug' => 'thumbnail',
                'description' => 'Optimized for small preview images and thumbnails.',
                'quality_webp' => 70,
                'quality_avif' => 65,
                'quality_jpeg' => 70,
                'quality_png' => 75,
                'compression_mode' => 'lossy',
                'convert_to_webp' => true,
                'convert_to_avif' => false,
                'generate_retina' => false,
                'strip_exif' => true,
                'generate_lqip' => false,
                'generate_blurhash' => true,
                'responsive_sizes' => ['xs' => 150, 'sm' => 300],
                'max_width' => 600,
                'max_height' => 600,
                'is_default' => false,
            ],
            [
                'name' => 'Email Marketing',
                'slug' => 'email',
                'description' => 'Optimized for email campaigns with strict file size limits.',
                'quality_webp' => 70,
                'quality_avif' => 65,
                'quality_jpeg' => 70,
                'quality_png' => 75,
                'compression_mode' => 'lossy',
                'convert_to_webp' => false, // Email clients may not support WebP
                'convert_to_avif' => false,
                'generate_retina' => false,
                'strip_exif' => true,
                'generate_lqip' => false,
                'generate_blurhash' => false,
                'responsive_sizes' => ['sm' => 600],
                'max_width' => 800,
                'is_default' => false,
            ],
            [
                'name' => 'Social Media',
                'slug' => 'social',
                'description' => 'Optimized for social media sharing with proper dimensions.',
                'quality_webp' => 85,
                'quality_avif' => 80,
                'quality_jpeg' => 85,
                'quality_png' => 90,
                'compression_mode' => 'auto',
                'convert_to_webp' => false, // Keep JPEG for compatibility
                'convert_to_avif' => false,
                'generate_retina' => true,
                'strip_exif' => false,
                'generate_lqip' => false,
                'generate_blurhash' => false,
                'responsive_sizes' => ['og' => 1200, 'twitter' => 1024],
                'is_default' => false,
            ],
        ];

        foreach ($presets as $preset) {
            ImageOptimizationPreset::updateOrCreate(
                ['slug' => $preset['slug']],
                $preset
            );
        }

        $this->command->info('Image optimization presets seeded successfully.');
    }
}
