<?php

declare(strict_types=1);

/**
 * Image Optimization Engine Configuration
 *
 * Enterprise-grade configuration for WebP/AVIF conversion,
 * responsive image generation, and CDN optimization.
 */

return [

    /*
    |--------------------------------------------------------------------------
    | Default Optimization Preset
    |--------------------------------------------------------------------------
    |
    | The default preset to use when no specific preset is specified.
    | This can be overridden per-upload or per-module.
    |
    */

    'default_preset' => env('IMAGE_OPTIMIZATION_PRESET', 'balanced'),

    /*
    |--------------------------------------------------------------------------
    | Storage Configuration
    |--------------------------------------------------------------------------
    |
    | Configure storage disks for originals, optimized versions, and variants.
    |
    */

    'storage' => [
        'disk' => env('IMAGE_STORAGE_DISK', 'public'),
        'originals_path' => 'media/originals',
        'optimized_path' => 'media/optimized',
        'variants_path' => 'media/variants',
        'thumbnails_path' => 'media/thumbnails',
        'temp_path' => 'media/temp',
        'keep_originals' => env('IMAGE_KEEP_ORIGINALS', true),
    ],

    /*
    |--------------------------------------------------------------------------
    | Format Conversion
    |--------------------------------------------------------------------------
    |
    | Configure automatic format conversion settings.
    |
    */

    'formats' => [
        'webp' => [
            'enabled' => env('IMAGE_WEBP_ENABLED', true),
            'quality' => env('IMAGE_WEBP_QUALITY', 85),
            'method' => 6, // 0-6, higher = better compression but slower
            'near_lossless' => true,
        ],
        'avif' => [
            'enabled' => env('IMAGE_AVIF_ENABLED', true),
            'quality' => env('IMAGE_AVIF_QUALITY', 80),
            'speed' => 6, // 0-10, higher = faster but larger files
        ],
        'jpeg' => [
            'quality' => env('IMAGE_JPEG_QUALITY', 85),
            'progressive' => true,
            'optimize' => true,
        ],
        'png' => [
            'quality' => env('IMAGE_PNG_QUALITY', 90),
            'compression' => 9, // 0-9
            'interlace' => true,
        ],
        'gif' => [
            'optimize' => true,
            'preserve_animation' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Responsive Sizes
    |--------------------------------------------------------------------------
    |
    | Define the responsive breakpoints for image variants.
    | Each size generates a separate variant.
    |
    */

    'responsive_sizes' => [
        'xs' => 320,
        'sm' => 640,
        'md' => 768,
        'lg' => 1024,
        'xl' => 1280,
        '2xl' => 1920,
    ],

    /*
    |--------------------------------------------------------------------------
    | Thumbnail Configuration
    |--------------------------------------------------------------------------
    |
    | Settings for thumbnail generation.
    |
    */

    'thumbnails' => [
        'width' => 300,
        'height' => 300,
        'fit' => 'cover', // cover, contain, fill, inside, outside
        'position' => 'center',
        'quality' => 80,
        'format' => 'webp',
    ],

    /*
    |--------------------------------------------------------------------------
    | Retina Support
    |--------------------------------------------------------------------------
    |
    | Enable 2x retina versions for high-DPI displays.
    |
    */

    'retina' => [
        'enabled' => env('IMAGE_RETINA_ENABLED', true),
        'densities' => [2], // Can add 3 for 3x if needed
        'max_width' => 3840, // Limit retina size
    ],

    /*
    |--------------------------------------------------------------------------
    | Placeholder Generation
    |--------------------------------------------------------------------------
    |
    | LQIP (Low Quality Image Placeholder) and BlurHash settings.
    |
    */

    'placeholders' => [
        'lqip' => [
            'enabled' => true,
            'width' => 32,
            'quality' => 20,
            'blur' => 10,
            'format' => 'base64',
        ],
        'blurhash' => [
            'enabled' => true,
            'components_x' => 4,
            'components_y' => 3,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Size Limits
    |--------------------------------------------------------------------------
    |
    | Maximum dimensions and file sizes for uploads and processing.
    |
    */

    'limits' => [
        'max_upload_size_mb' => env('IMAGE_MAX_UPLOAD_MB', 50),
        'max_width' => env('IMAGE_MAX_WIDTH', 8192),
        'max_height' => env('IMAGE_MAX_HEIGHT', 8192),
        'max_pixels' => env('IMAGE_MAX_PIXELS', 50000000), // 50MP
        'min_width' => 10,
        'min_height' => 10,
    ],

    /*
    |--------------------------------------------------------------------------
    | EXIF & Metadata
    |--------------------------------------------------------------------------
    |
    | Configure metadata handling during optimization.
    |
    */

    'metadata' => [
        'strip_exif' => env('IMAGE_STRIP_EXIF', false),
        'preserve_copyright' => true,
        'preserve_icc_profile' => true,
        'extract_gps' => false, // Privacy consideration
        'extract_camera_info' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | Queue Configuration
    |--------------------------------------------------------------------------
    |
    | Configure background processing for optimization jobs.
    |
    */

    'queue' => [
        'connection' => env('IMAGE_QUEUE_CONNECTION', 'database'),
        'queue_name' => env('IMAGE_QUEUE_NAME', 'image-optimization'),
        'timeout' => 300, // 5 minutes per job
        'tries' => 3,
        'retry_after' => 600, // 10 minutes
        'batch_size' => 10,
        'concurrent_jobs' => env('IMAGE_CONCURRENT_JOBS', 3),
    ],

    /*
    |--------------------------------------------------------------------------
    | CDN Integration
    |--------------------------------------------------------------------------
    |
    | Configure CDN for optimized image delivery.
    |
    */

    'cdn' => [
        'enabled' => env('IMAGE_CDN_ENABLED', false),
        'provider' => env('IMAGE_CDN_PROVIDER', 'custom'), // cloudflare, bunny, cloudinary, custom
        'base_url' => env('CDN_URL'),
        'auto_replace_urls' => true,
        'cache_control' => 'public, max-age=31536000, immutable',
    ],

    /*
    |--------------------------------------------------------------------------
    | Content-Based Optimization
    |--------------------------------------------------------------------------
    |
    | Settings for intelligent, content-aware optimization.
    |
    */

    'content_detection' => [
        'enabled' => true,
        'types' => [
            'photo' => ['quality' => 85, 'format' => 'webp'],
            'illustration' => ['quality' => 90, 'format' => 'webp'],
            'graphic' => ['quality' => 95, 'format' => 'png'],
            'text' => ['quality' => 95, 'format' => 'png'],
            'screenshot' => ['quality' => 90, 'format' => 'webp'],
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Lazy Loading
    |--------------------------------------------------------------------------
    |
    | Configure lazy loading behavior for optimized images.
    |
    */

    'lazy_loading' => [
        'enabled' => true,
        'threshold' => '200px', // Load when within 200px of viewport
        'use_intersection_observer' => true,
        'fade_in' => true,
        'fade_duration_ms' => 300,
    ],

    /*
    |--------------------------------------------------------------------------
    | Prefetch Strategy
    |--------------------------------------------------------------------------
    |
    | Configure resource prefetching for critical images.
    |
    */

    'prefetch' => [
        'enabled' => true,
        'hero_images' => true,
        'above_fold_images' => true,
        'max_prefetch' => 5,
    ],

    /*
    |--------------------------------------------------------------------------
    | Analytics
    |--------------------------------------------------------------------------
    |
    | Configure optimization analytics and reporting.
    |
    */

    'analytics' => [
        'enabled' => true,
        'track_savings' => true,
        'track_bandwidth' => true,
        'retention_days' => 90,
        'aggregate_hourly' => false,
        'aggregate_daily' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | Cleanup & Maintenance
    |--------------------------------------------------------------------------
    |
    | Configure automatic cleanup of temporary and orphaned files.
    |
    */

    'cleanup' => [
        'temp_files_after_hours' => 24,
        'orphaned_variants_after_days' => 7,
        'failed_jobs_after_days' => 30,
        'auto_cleanup' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | Presets
    |--------------------------------------------------------------------------
    |
    | Predefined optimization presets for different use cases.
    |
    */

    'presets' => [
        'maximum' => [
            'quality_webp' => 95,
            'quality_avif' => 90,
            'compression_mode' => 'lossless',
            'generate_retina' => true,
            'strip_exif' => false,
        ],
        'balanced' => [
            'quality_webp' => 85,
            'quality_avif' => 80,
            'compression_mode' => 'auto',
            'generate_retina' => true,
            'strip_exif' => false,
        ],
        'performance' => [
            'quality_webp' => 75,
            'quality_avif' => 70,
            'compression_mode' => 'lossy',
            'generate_retina' => false,
            'strip_exif' => true,
        ],
        'thumbnail' => [
            'quality_webp' => 70,
            'quality_avif' => 65,
            'compression_mode' => 'lossy',
            'generate_retina' => false,
            'strip_exif' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Module Integration
    |--------------------------------------------------------------------------
    |
    | Default presets for different modules.
    |
    */

    'modules' => [
        'blog' => 'balanced',
        'products' => 'balanced',
        'courses' => 'balanced',
        'email' => 'performance',
        'funnels' => 'performance',
        'popups' => 'performance',
        'crm' => 'thumbnail',
        'trading_room' => 'balanced',
    ],

    /*
    |--------------------------------------------------------------------------
    | Supported Formats
    |--------------------------------------------------------------------------
    |
    | File formats that can be processed by the optimization engine.
    |
    */

    'supported_formats' => [
        'input' => ['jpeg', 'jpg', 'png', 'gif', 'webp', 'bmp', 'tiff', 'svg', 'heic', 'heif'],
        'output' => ['webp', 'avif', 'jpeg', 'png'],
    ],

    /*
    |--------------------------------------------------------------------------
    | HEIC/HEIF Support (Apple Format)
    |--------------------------------------------------------------------------
    |
    | Auto-convert iPhone HEIC images on upload.
    |
    */

    'heic' => [
        'enabled' => env('IMAGE_HEIC_ENABLED', true),
        'auto_convert' => true,
        'convert_to' => 'jpeg', // jpeg or webp
        'quality' => 90,
        'preserve_exif' => true,
    ],

    /*
    |--------------------------------------------------------------------------
    | SVG Optimization
    |--------------------------------------------------------------------------
    |
    | Settings for SVG minification and optimization.
    |
    */

    'svg' => [
        'enabled' => env('IMAGE_SVG_OPTIMIZATION_ENABLED', true),
        'minify' => true,
        'remove_comments' => true,
        'remove_metadata' => true,
        'remove_empty_attrs' => true,
        'remove_hidden_elements' => true,
        'convert_colors_to_hex' => true,
        'remove_dimensions' => false, // Keep viewBox but remove width/height
        'precision' => 3, // Decimal precision for paths
    ],

    /*
    |--------------------------------------------------------------------------
    | Smart Crop (Face Detection)
    |--------------------------------------------------------------------------
    |
    | Intelligent cropping that detects faces/subjects.
    |
    */

    'smart_crop' => [
        'enabled' => env('IMAGE_SMART_CROP_ENABLED', true),
        'detect_faces' => true,
        'detect_subjects' => true, // Objects of interest
        'fallback_position' => 'center', // If no faces/subjects found
        'face_padding' => 0.2, // 20% padding around detected faces
        'min_face_size' => 20, // Minimum face size in pixels
    ],

    /*
    |--------------------------------------------------------------------------
    | Animated Image Conversion
    |--------------------------------------------------------------------------
    |
    | Convert animated GIFs to more efficient formats.
    |
    */

    'animated' => [
        'enabled' => env('IMAGE_ANIMATED_CONVERSION_ENABLED', true),
        'gif_to_webp' => true,
        'gif_to_avif' => false, // AVIF animation support is limited
        'max_frames' => 500, // Skip conversion for very long animations
        'preserve_loops' => true,
        'optimize_frames' => true, // Remove duplicate frames
    ],

    /*
    |--------------------------------------------------------------------------
    | On-the-fly Transformations
    |--------------------------------------------------------------------------
    |
    | URL-based image transformations (like Cloudinary/Imgix).
    |
    */

    'on_the_fly' => [
        'enabled' => env('IMAGE_ON_THE_FLY_ENABLED', true),
        'cache_transformations' => true,
        'cache_ttl' => 86400 * 30, // 30 days
        'allowed_operations' => ['resize', 'crop', 'fit', 'rotate', 'flip', 'blur', 'grayscale', 'quality'],
        'max_width' => 4096,
        'max_height' => 4096,
        'sign_urls' => env('IMAGE_SIGN_URLS', false), // HMAC signature for security
        'signature_key' => env('IMAGE_SIGNATURE_KEY'),
    ],

    /*
    |--------------------------------------------------------------------------
    | Duplicate Detection
    |--------------------------------------------------------------------------
    |
    | Settings for finding and managing duplicate images.
    |
    */

    'duplicates' => [
        'enabled' => true,
        'detection_method' => 'hash', // hash, perceptual, both
        'hash_algorithm' => 'sha256',
        'perceptual_threshold' => 0.95, // 95% similarity
        'auto_deduplicate' => false, // Require manual confirmation
        'keep_strategy' => 'oldest', // oldest, newest, largest, smallest
    ],

    /*
    |--------------------------------------------------------------------------
    | Cleanup Scheduler
    |--------------------------------------------------------------------------
    |
    | Automatic cleanup of unused and orphaned images.
    |
    */

    'cleanup_scheduler' => [
        'enabled' => env('IMAGE_CLEANUP_ENABLED', true),
        'schedule' => 'daily', // hourly, daily, weekly
        'unused_after_days' => 30, // Mark unused after 30 days
        'delete_unused_after_days' => 90, // Auto-delete after 90 days (if enabled)
        'auto_delete_unused' => false, // Require manual confirmation by default
        'notify_before_delete' => true,
        'notification_days_before' => 7,
    ],

    /*
    |--------------------------------------------------------------------------
    | Real-time Progress (WebSocket)
    |--------------------------------------------------------------------------
    |
    | WebSocket broadcasting for optimization progress.
    |
    */

    'realtime' => [
        'enabled' => env('IMAGE_REALTIME_ENABLED', true),
        'broadcast_channel' => 'image-optimization',
        'broadcast_driver' => env('BROADCAST_DRIVER', 'pusher'),
        'events' => [
            'job_started' => true,
            'job_progress' => true,
            'job_completed' => true,
            'job_failed' => true,
            'batch_progress' => true,
        ],
    ],

    /*
    |--------------------------------------------------------------------------
    | Watermark (Optional)
    |--------------------------------------------------------------------------
    |
    | Configure automatic watermarking for images.
    |
    */

    'watermark' => [
        'enabled' => false,
        'path' => null,
        'position' => 'bottom-right',
        'opacity' => 50,
        'size_percent' => 15,
        'exclude_thumbnails' => true,
    ],

];
