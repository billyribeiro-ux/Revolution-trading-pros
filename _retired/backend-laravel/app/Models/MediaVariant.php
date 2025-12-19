<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Support\Facades\Storage;

/**
 * MediaVariant Model
 *
 * Represents generated image variants (thumbnails, responsive sizes, WebP/AVIF conversions).
 *
 * @property int $id
 * @property int $media_id
 * @property string $variant_type
 * @property string|null $size_name
 * @property int $width
 * @property int $height
 * @property string $filename
 * @property string $path
 * @property string $disk
 * @property string|null $url
 * @property string|null $cdn_url
 * @property string $mime_type
 * @property string $format
 * @property int $size
 * @property int|null $quality
 * @property float|null $compression_ratio
 * @property int|null $original_size
 * @property int|null $bytes_saved
 * @property string|null $lqip_data
 * @property string|null $blurhash
 * @property bool $is_retina
 * @property int $pixel_density
 */
class MediaVariant extends Model
{
    use HasFactory;

    // Variant types
    public const TYPE_THUMBNAIL = 'thumbnail';
    public const TYPE_RESPONSIVE = 'responsive';
    public const TYPE_WEBP = 'webp';
    public const TYPE_AVIF = 'avif';
    public const TYPE_RETINA = 'retina';
    public const TYPE_PLACEHOLDER = 'placeholder';

    // Size names
    public const SIZE_XS = 'xs';
    public const SIZE_SM = 'sm';
    public const SIZE_MD = 'md';
    public const SIZE_LG = 'lg';
    public const SIZE_XL = 'xl';
    public const SIZE_2XL = '2xl';

    protected $fillable = [
        'media_id',
        'variant_type',
        'size_name',
        'width',
        'height',
        'filename',
        'path',
        'disk',
        'url',
        'cdn_url',
        'mime_type',
        'format',
        'size',
        'quality',
        'compression_ratio',
        'original_size',
        'bytes_saved',
        'lqip_data',
        'blurhash',
        'is_retina',
        'pixel_density',
    ];

    protected $casts = [
        'width' => 'integer',
        'height' => 'integer',
        'size' => 'integer',
        'quality' => 'integer',
        'compression_ratio' => 'decimal:2',
        'original_size' => 'integer',
        'bytes_saved' => 'integer',
        'is_retina' => 'boolean',
        'pixel_density' => 'integer',
    ];

    protected $attributes = [
        'disk' => 'public',
        'is_retina' => false,
        'pixel_density' => 1,
    ];

    /**
     * Get the parent media
     */
    public function media(): BelongsTo
    {
        return $this->belongsTo(Media::class);
    }

    /**
     * Get public URL
     */
    public function getPublicUrl(): string
    {
        return $this->cdn_url ?? $this->url ?? Storage::disk($this->disk)->url($this->path);
    }

    /**
     * Check if file exists
     */
    public function fileExists(): bool
    {
        return Storage::disk($this->disk)->exists($this->path);
    }

    /**
     * Delete physical file
     */
    public function deleteFile(): bool
    {
        try {
            if ($this->fileExists()) {
                Storage::disk($this->disk)->delete($this->path);
            }
            return true;
        } catch (\Exception $e) {
            logger()->error('Failed to delete variant file', [
                'variant_id' => $this->id,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get savings percentage
     */
    public function getSavingsPercentAttribute(): ?float
    {
        if (!$this->original_size || $this->original_size === 0) {
            return null;
        }
        return round((1 - ($this->size / $this->original_size)) * 100, 2);
    }

    /**
     * Get aspect ratio
     */
    public function getAspectRatioAttribute(): float
    {
        return $this->height > 0 ? round($this->width / $this->height, 4) : 0;
    }

    /**
     * Get human-readable size
     */
    public function getHumanSizeAttribute(): string
    {
        return Media::formatBytes($this->size);
    }

    /**
     * Scope: By variant type
     */
    public function scopeOfType($query, string $type)
    {
        return $query->where('variant_type', $type);
    }

    /**
     * Scope: By size name
     */
    public function scopeOfSize($query, string $size)
    {
        return $query->where('size_name', $size);
    }

    /**
     * Scope: By format
     */
    public function scopeOfFormat($query, string $format)
    {
        return $query->where('format', $format);
    }

    /**
     * Scope: Retina variants
     */
    public function scopeRetina($query)
    {
        return $query->where('is_retina', true);
    }

    /**
     * Scope: Non-retina variants
     */
    public function scopeStandard($query)
    {
        return $query->where('is_retina', false);
    }

    /**
     * Export to array
     */
    public function toVariantArray(): array
    {
        return [
            'type' => $this->variant_type,
            'size' => $this->size_name,
            'width' => $this->width,
            'height' => $this->height,
            'url' => $this->getPublicUrl(),
            'format' => $this->format,
            'file_size' => $this->size,
            'is_retina' => $this->is_retina,
            'savings_percent' => $this->savings_percent,
            'lqip' => $this->lqip_data,
            'blurhash' => $this->blurhash,
        ];
    }
}
