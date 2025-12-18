<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

/**
 * ImageOptimizationPreset Model
 *
 * Defines optimization settings for different use cases.
 *
 * @property int $id
 * @property string $name
 * @property string $slug
 * @property string|null $description
 * @property int $quality_webp
 * @property int $quality_avif
 * @property int $quality_jpeg
 * @property int $quality_png
 * @property string $compression_mode
 * @property bool $preserve_transparency
 * @property bool $progressive_jpeg
 * @property bool $interlaced_png
 * @property bool $convert_to_webp
 * @property bool $convert_to_avif
 * @property bool $keep_original_format
 * @property array|null $responsive_sizes
 * @property bool $generate_retina
 * @property int|null $max_width
 * @property int|null $max_height
 * @property bool $strip_exif
 * @property bool $preserve_icc_profile
 * @property bool $preserve_copyright
 * @property bool $generate_lqip
 * @property bool $generate_blurhash
 * @property int $lqip_quality
 * @property int $lqip_width
 * @property string $content_type
 * @property bool $is_default
 * @property bool $is_active
 */
class ImageOptimizationPreset extends Model
{
    use HasFactory;

    // Compression modes
    public const MODE_LOSSLESS = 'lossless';
    public const MODE_LOSSY = 'lossy';
    public const MODE_AUTO = 'auto';

    // Content types
    public const CONTENT_AUTO = 'auto';
    public const CONTENT_PHOTO = 'photo';
    public const CONTENT_ILLUSTRATION = 'illustration';
    public const CONTENT_GRAPHIC = 'graphic';
    public const CONTENT_TEXT = 'text';
    public const CONTENT_MIXED = 'mixed';

    protected $fillable = [
        'name',
        'slug',
        'description',
        'quality_webp',
        'quality_avif',
        'quality_jpeg',
        'quality_png',
        'compression_mode',
        'preserve_transparency',
        'progressive_jpeg',
        'interlaced_png',
        'convert_to_webp',
        'convert_to_avif',
        'keep_original_format',
        'responsive_sizes',
        'generate_retina',
        'max_width',
        'max_height',
        'strip_exif',
        'preserve_icc_profile',
        'preserve_copyright',
        'generate_lqip',
        'generate_blurhash',
        'lqip_quality',
        'lqip_width',
        'content_type',
        'is_default',
        'is_active',
    ];

    protected $casts = [
        'quality_webp' => 'integer',
        'quality_avif' => 'integer',
        'quality_jpeg' => 'integer',
        'quality_png' => 'integer',
        'responsive_sizes' => 'array',
        'max_width' => 'integer',
        'max_height' => 'integer',
        'lqip_quality' => 'integer',
        'lqip_width' => 'integer',
        'preserve_transparency' => 'boolean',
        'progressive_jpeg' => 'boolean',
        'interlaced_png' => 'boolean',
        'convert_to_webp' => 'boolean',
        'convert_to_avif' => 'boolean',
        'keep_original_format' => 'boolean',
        'generate_retina' => 'boolean',
        'strip_exif' => 'boolean',
        'preserve_icc_profile' => 'boolean',
        'preserve_copyright' => 'boolean',
        'generate_lqip' => 'boolean',
        'generate_blurhash' => 'boolean',
        'is_default' => 'boolean',
        'is_active' => 'boolean',
    ];

    protected $attributes = [
        'quality_webp' => 85,
        'quality_avif' => 80,
        'quality_jpeg' => 85,
        'quality_png' => 90,
        'compression_mode' => self::MODE_AUTO,
        'preserve_transparency' => true,
        'progressive_jpeg' => true,
        'interlaced_png' => true,
        'convert_to_webp' => true,
        'convert_to_avif' => false,
        'keep_original_format' => true,
        'generate_retina' => true,
        'strip_exif' => false,
        'preserve_icc_profile' => true,
        'preserve_copyright' => true,
        'generate_lqip' => true,
        'generate_blurhash' => true,
        'lqip_quality' => 20,
        'lqip_width' => 32,
        'content_type' => self::CONTENT_AUTO,
        'is_default' => false,
        'is_active' => true,
    ];

    /**
     * Get optimization jobs using this preset
     */
    public function optimizationJobs(): HasMany
    {
        return $this->hasMany(ImageOptimizationJob::class, 'preset_id');
    }

    /**
     * Get default preset
     */
    public static function getDefault(): ?self
    {
        return static::where('is_default', true)->where('is_active', true)->first();
    }

    /**
     * Get by slug
     */
    public static function getBySlug(string $slug): ?self
    {
        return static::where('slug', $slug)->where('is_active', true)->first();
    }

    /**
     * Set as default
     */
    public function setAsDefault(): self
    {
        static::where('is_default', true)->update(['is_default' => false]);
        $this->update(['is_default' => true]);
        return $this;
    }

    /**
     * Get quality for format
     */
    public function getQualityForFormat(string $format): int
    {
        return match ($format) {
            'webp' => $this->quality_webp,
            'avif' => $this->quality_avif,
            'jpeg', 'jpg' => $this->quality_jpeg,
            'png' => $this->quality_png,
            default => 85,
        };
    }

    /**
     * Get responsive sizes array
     */
    public function getResponsiveSizesArray(): array
    {
        return $this->responsive_sizes ?? config('image-optimization.responsive_sizes', []);
    }

    /**
     * Scope: Active presets
     */
    public function scopeActive($query)
    {
        return $query->where('is_active', true);
    }

    /**
     * Export to array
     */
    public function toPresetArray(): array
    {
        return [
            'id' => $this->id,
            'name' => $this->name,
            'slug' => $this->slug,
            'description' => $this->description,
            'quality' => [
                'webp' => $this->quality_webp,
                'avif' => $this->quality_avif,
                'jpeg' => $this->quality_jpeg,
                'png' => $this->quality_png,
            ],
            'compression_mode' => $this->compression_mode,
            'convert_to_webp' => $this->convert_to_webp,
            'convert_to_avif' => $this->convert_to_avif,
            'generate_retina' => $this->generate_retina,
            'responsive_sizes' => $this->getResponsiveSizesArray(),
            'is_default' => $this->is_default,
        ];
    }
}
