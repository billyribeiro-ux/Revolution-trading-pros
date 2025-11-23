<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Str;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;

/**
 * Media Model
 * 
 * Manages media files (images, videos, documents) with optimization,
 * CDN support, metadata extraction, and advanced organization features.
 *
 * @property int $id
 * @property string $filename Original filename
 * @property string $disk Storage disk (local, s3, cloudinary, etc.)
 * @property string $path File path on disk
 * @property string $url Public URL
 * @property string|null $cdn_url CDN URL if available
 * @property string $mime_type MIME type
 * @property string $type Media type (image, video, audio, document, other)
 * @property int $size File size in bytes
 * @property string|null $alt_text Alternative text for accessibility
 * @property string|null $title Media title
 * @property string|null $caption Media caption
 * @property string|null $description Detailed description
 * @property array $metadata File metadata (dimensions, duration, etc.)
 * @property array $exif EXIF data for images
 * @property int|null $width Image/video width
 * @property int|null $height Image/video height
 * @property float|null $aspect_ratio Aspect ratio (width/height)
 * @property int|null $duration Media duration in seconds (for video/audio)
 * @property string|null $hash File hash (SHA-256)
 * @property bool $is_optimized Whether media has been optimized
 * @property bool $is_processed Whether processing is complete
 * @property string|null $processing_status Processing status (pending, processing, completed, failed)
 * @property array $variants Generated variants (thumbnails, formats, sizes)
 * @property string|null $thumbnail_path Thumbnail path
 * @property string|null $thumbnail_url Thumbnail URL
 * @property array $tags Media tags for organization
 * @property string|null $collection Collection/folder name
 * @property string|null $category Media category
 * @property bool $is_public Public accessibility
 * @property bool $is_featured Featured media flag
 * @property int $download_count Download counter
 * @property int $view_count View counter
 * @property int $usage_count Usage counter (in posts, products, etc.)
 * @property \Illuminate\Support\Carbon|null $last_accessed_at Last access timestamp
 * @property string|null $source Media source (upload, url, import, api, etc.)
 * @property string|null $source_url Original source URL
 * @property string|null $license License information
 * @property string|null $copyright Copyright information
 * @property string|null $credit Credit/attribution
 * @property array $seo SEO data (title, description, keywords)
 * @property int|null $uploaded_by User who uploaded
 * @property string|null $ip_address Upload IP address
 * @property array $processing_log Processing history
 * @property \Illuminate\Support\Carbon|null $optimized_at Optimization timestamp
 * @property \Illuminate\Support\Carbon|null $expires_at Expiration timestamp (for temporary files)
 * @property \Illuminate\Support\Carbon|null $deleted_at Soft delete timestamp
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \App\Models\User|null $uploader User who uploaded
 * @property-read \Illuminate\Database\Eloquent\Collection $posts Posts using this media
 */
class Media extends Model
{
    use HasFactory, SoftDeletes;

    /**
     * Media types
     */
    public const TYPE_IMAGE = 'image';
    public const TYPE_VIDEO = 'video';
    public const TYPE_AUDIO = 'audio';
    public const TYPE_DOCUMENT = 'document';
    public const TYPE_ARCHIVE = 'archive';
    public const TYPE_OTHER = 'other';

    /**
     * Processing statuses
     */
    public const STATUS_PENDING = 'pending';
    public const STATUS_PROCESSING = 'processing';
    public const STATUS_COMPLETED = 'completed';
    public const STATUS_FAILED = 'failed';

    /**
     * Media sources
     */
    public const SOURCE_UPLOAD = 'upload';
    public const SOURCE_URL = 'url';
    public const SOURCE_IMPORT = 'import';
    public const SOURCE_API = 'api';
    public const SOURCE_GENERATED = 'generated';

    /**
     * Image variant sizes
     */
    public const SIZE_THUMBNAIL = 'thumbnail';
    public const SIZE_SMALL = 'small';
    public const SIZE_MEDIUM = 'medium';
    public const SIZE_LARGE = 'large';
    public const SIZE_XLARGE = 'xlarge';
    public const SIZE_ORIGINAL = 'original';

    /**
     * Valid media types
     */
    public const VALID_TYPES = [
        self::TYPE_IMAGE,
        self::TYPE_VIDEO,
        self::TYPE_AUDIO,
        self::TYPE_DOCUMENT,
        self::TYPE_ARCHIVE,
        self::TYPE_OTHER,
    ];

    /**
     * Image MIME types
     */
    public const IMAGE_MIME_TYPES = [
        'image/jpeg',
        'image/jpg',
        'image/png',
        'image/gif',
        'image/webp',
        'image/svg+xml',
        'image/bmp',
        'image/tiff',
    ];

    /**
     * Video MIME types
     */
    public const VIDEO_MIME_TYPES = [
        'video/mp4',
        'video/mpeg',
        'video/quicktime',
        'video/x-msvideo',
        'video/webm',
        'video/ogg',
    ];

    /**
     * Audio MIME types
     */
    public const AUDIO_MIME_TYPES = [
        'audio/mpeg',
        'audio/mp3',
        'audio/wav',
        'audio/ogg',
        'audio/webm',
        'audio/aac',
    ];

    /**
     * Document MIME types
     */
    public const DOCUMENT_MIME_TYPES = [
        'application/pdf',
        'application/msword',
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
        'application/vnd.ms-excel',
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
        'application/vnd.ms-powerpoint',
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
        'text/plain',
        'text/csv',
    ];

    /**
     * Default storage disk
     */
    public const DEFAULT_DISK = 'public';

    /**
     * Cache TTL
     */
    public const CACHE_TTL = 86400; // 24 hours

    protected $fillable = [
        'filename',
        'disk',
        'path',
        'url',
        'cdn_url',
        'mime_type',
        'type',
        'size',
        'alt_text',
        'title',
        'caption',
        'description',
        'metadata',
        'exif',
        'width',
        'height',
        'aspect_ratio',
        'duration',
        'hash',
        'is_optimized',
        'is_processed',
        'processing_status',
        'variants',
        'thumbnail_path',
        'thumbnail_url',
        'tags',
        'collection',
        'category',
        'is_public',
        'is_featured',
        'download_count',
        'view_count',
        'usage_count',
        'last_accessed_at',
        'source',
        'source_url',
        'license',
        'copyright',
        'credit',
        'seo',
        'uploaded_by',
        'ip_address',
        'processing_log',
        'optimized_at',
        'expires_at',
    ];

    protected $casts = [
        'metadata' => 'array',
        'exif' => 'array',
        'variants' => 'array',
        'tags' => 'array',
        'seo' => 'array',
        'processing_log' => 'array',
        'size' => 'integer',
        'width' => 'integer',
        'height' => 'integer',
        'duration' => 'integer',
        'download_count' => 'integer',
        'view_count' => 'integer',
        'usage_count' => 'integer',
        'aspect_ratio' => 'decimal:4',
        'is_optimized' => 'boolean',
        'is_processed' => 'boolean',
        'is_public' => 'boolean',
        'is_featured' => 'boolean',
        'last_accessed_at' => 'datetime',
        'optimized_at' => 'datetime',
        'expires_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    protected $attributes = [
        'disk' => self::DEFAULT_DISK,
        'type' => self::TYPE_OTHER,
        'processing_status' => self::STATUS_PENDING,
        'is_optimized' => false,
        'is_processed' => false,
        'is_public' => true,
        'is_featured' => false,
        'download_count' => 0,
        'view_count' => 0,
        'usage_count' => 0,
        'source' => self::SOURCE_UPLOAD,
        'metadata' => '[]',
        'exif' => '[]',
        'variants' => '[]',
        'tags' => '[]',
        'seo' => '[]',
        'processing_log' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->detectMediaType();
            $model->generateHash();
            $model->extractMetadata();
            $model->generateUrl();
            
            if (!$model->uploaded_by) {
                $model->uploaded_by = auth()->id();
            }
            
            if (!$model->ip_address) {
                $model->ip_address = request()->ip();
            }
        });

        static::updating(function (self $model): void {
            if ($model->isDirty('path')) {
                $model->generateUrl();
            }
        });

        static::saved(function (self $model): void {
            $model->clearMediaCache();
        });

        static::deleting(function (self $model): void {
            // Delete physical file
            if ($model->exists()) {
                $model->deleteFile();
            }
        });

        static::deleted(function (self $model): void {
            $model->clearMediaCache();
        });
    }

    /**
     * Get user who uploaded the media
     */
    public function uploader(): BelongsTo
    {
        return $this->belongsTo(User::class, 'uploaded_by');
    }

    /**
     * Get posts using this media
     */
    public function posts(): BelongsToMany
    {
        return $this->belongsToMany(Post::class, 'post_media')
            ->withPivot(['order', 'caption', 'alt_text'])
            ->withTimestamps();
    }

    /**
     * Get products using this media
     */
    public function products(): BelongsToMany
    {
        return $this->belongsToMany(Product::class, 'product_media')
            ->withPivot(['order', 'is_primary'])
            ->withTimestamps();
    }

    /**
     * Get any model using this media (polymorphic)
     */
    public function mediables(): MorphToMany
    {
        return $this->morphedByMany(Model::class, 'mediable');
    }

    /**
     * Detect media type from MIME type
     */
    protected function detectMediaType(): void
    {
        if ($this->type && $this->type !== self::TYPE_OTHER) {
            return;
        }

        $this->type = match(true) {
            in_array($this->mime_type, self::IMAGE_MIME_TYPES, true) => self::TYPE_IMAGE,
            in_array($this->mime_type, self::VIDEO_MIME_TYPES, true) => self::TYPE_VIDEO,
            in_array($this->mime_type, self::AUDIO_MIME_TYPES, true) => self::TYPE_AUDIO,
            in_array($this->mime_type, self::DOCUMENT_MIME_TYPES, true) => self::TYPE_DOCUMENT,
            str_contains($this->mime_type, 'zip') || str_contains($this->mime_type, 'archive') => self::TYPE_ARCHIVE,
            default => self::TYPE_OTHER,
        };
    }

    /**
     * Generate file hash
     */
    protected function generateHash(): void
    {
        if ($this->hash || !$this->fileExists()) {
            return;
        }

        try {
            $contents = Storage::disk($this->disk)->get($this->path);
            $this->hash = hash('sha256', $contents);
        } catch (\Exception $e) {
            // Log error but don't fail
            logger()->error('Failed to generate media hash', [
                'media_id' => $this->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Extract metadata from file
     */
    protected function extractMetadata(): void
    {
        if (!$this->fileExists()) {
            return;
        }

        try {
            if ($this->isImage()) {
                $this->extractImageMetadata();
            } elseif ($this->isVideo()) {
                $this->extractVideoMetadata();
            }
        } catch (\Exception $e) {
            logger()->error('Failed to extract media metadata', [
                'media_id' => $this->id,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Extract image metadata
     */
    protected function extractImageMetadata(): void
    {
        $fullPath = Storage::disk($this->disk)->path($this->path);

        if (!file_exists($fullPath)) {
            return;
        }

        // Get dimensions
        $imageInfo = getimagesize($fullPath);
        if ($imageInfo) {
            $this->width = $imageInfo[0];
            $this->height = $imageInfo[1];
            $this->aspect_ratio = $this->height > 0 ? round($this->width / $this->height, 4) : null;
        }

        // Get EXIF data
        if (function_exists('exif_read_data') && in_array($this->mime_type, ['image/jpeg', 'image/jpg', 'image/tiff'])) {
            $exif = @exif_read_data($fullPath);
            if ($exif) {
                $this->exif = [
                    'camera' => $exif['Model'] ?? null,
                    'date_taken' => $exif['DateTime'] ?? null,
                    'aperture' => $exif['COMPUTED']['ApertureFNumber'] ?? null,
                    'iso' => $exif['ISOSpeedRatings'] ?? null,
                    'focal_length' => $exif['FocalLength'] ?? null,
                    'exposure_time' => $exif['ExposureTime'] ?? null,
                ];
            }
        }
    }

    /**
     * Extract video metadata
     */
    protected function extractVideoMetadata(): void
    {
        // Implement with FFmpeg or similar
        // This is a placeholder for video metadata extraction
        // You would use packages like pbmedia/laravel-ffmpeg
    }

    /**
     * Generate public URL
     */
    protected function generateUrl(): void
    {
        if ($this->url) {
            return;
        }

        $this->url = Storage::disk($this->disk)->url($this->path);
    }

    /**
     * Clear media cache
     */
    protected function clearMediaCache(): void
    {
        Cache::tags(['media'])->flush();
        Cache::forget("media:id:{$this->id}");
        Cache::forget("media:hash:{$this->hash}");
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
            // Delete main file
            if ($this->fileExists()) {
                Storage::disk($this->disk)->delete($this->path);
            }

            // Delete thumbnail
            if ($this->thumbnail_path && Storage::disk($this->disk)->exists($this->thumbnail_path)) {
                Storage::disk($this->disk)->delete($this->thumbnail_path);
            }

            // Delete variants
            foreach ($this->variants as $variant) {
                if (isset($variant['path']) && Storage::disk($this->disk)->exists($variant['path'])) {
                    Storage::disk($this->disk)->delete($variant['path']);
                }
            }

            return true;
        } catch (\Exception $e) {
            logger()->error('Failed to delete media file', [
                'media_id' => $this->id,
                'error' => $e->getMessage(),
            ]);
            return false;
        }
    }

    /**
     * Get full storage path
     */
    public function getFullPath(): string
    {
        return Storage::disk($this->disk)->path($this->path);
    }

    /**
     * Get public URL (with CDN if available)
     */
    public function getPublicUrl(): string
    {
        return $this->cdn_url ?? $this->url;
    }

    /**
     * Get variant URL
     */
    public function getVariantUrl(string $size): ?string
    {
        $variant = collect($this->variants)->firstWhere('size', $size);
        return $variant['url'] ?? null;
    }

    /**
     * Get thumbnail URL with fallback
     */
    public function getThumbnailUrlWithFallback(): string
    {
        return $this->thumbnail_url 
            ?? $this->getVariantUrl(self::SIZE_THUMBNAIL) 
            ?? $this->url;
    }

    /**
     * Check if media is image
     */
    public function isImage(): bool
    {
        return $this->type === self::TYPE_IMAGE;
    }

    /**
     * Check if media is video
     */
    public function isVideo(): bool
    {
        return $this->type === self::TYPE_VIDEO;
    }

    /**
     * Check if media is audio
     */
    public function isAudio(): bool
    {
        return $this->type === self::TYPE_AUDIO;
    }

    /**
     * Check if media is document
     */
    public function isDocument(): bool
    {
        return $this->type === self::TYPE_DOCUMENT;
    }

    /**
     * Check if media is expired
     */
    public function isExpired(): bool
    {
        return $this->expires_at && $this->expires_at->isPast();
    }

    /**
     * Check if media is optimized
     */
    public function needsOptimization(): bool
    {
        return !$this->is_optimized && $this->isImage();
    }

    /**
     * Get human-readable file size
     */
    public function getHumanReadableSizeAttribute(): string
    {
        $bytes = $this->size;

        if ($bytes >= 1073741824) {
            return number_format($bytes / 1073741824, 2) . ' GB';
        }

        if ($bytes >= 1048576) {
            return number_format($bytes / 1048576, 2) . ' MB';
        }

        if ($bytes >= 1024) {
            return number_format($bytes / 1024, 2) . ' KB';
        }

        return $bytes . ' bytes';
    }

    /**
     * Get file extension
     */
    public function getExtensionAttribute(): string
    {
        return pathinfo($this->filename, PATHINFO_EXTENSION);
    }

    /**
     * Get filename without extension
     */
    public function getBaseNameAttribute(): string
    {
        return pathinfo($this->filename, PATHINFO_FILENAME);
    }

    /**
     * Increment download count
     */
    public function recordDownload(): self
    {
        $this->increment('download_count');
        $this->update(['last_accessed_at' => now()]);
        return $this;
    }

    /**
     * Increment view count
     */
    public function recordView(): self
    {
        $this->increment('view_count');
        $this->update(['last_accessed_at' => now()]);
        return $this;
    }

    /**
     * Increment usage count
     */
    public function recordUsage(): self
    {
        $this->increment('usage_count');
        return $this;
    }

    /**
     * Mark as optimized
     */
    public function markAsOptimized(): self
    {
        $this->update([
            'is_optimized' => true,
            'optimized_at' => now(),
        ]);
        return $this;
    }

    /**
     * Mark as processed
     */
    public function markAsProcessed(): self
    {
        $this->update([
            'is_processed' => true,
            'processing_status' => self::STATUS_COMPLETED,
        ]);
        return $this;
    }

    /**
     * Mark processing as failed
     */
    public function markProcessingFailed(string $error): self
    {
        $log = $this->processing_log;
        $log[] = [
            'status' => 'failed',
            'error' => $error,
            'timestamp' => now()->toISOString(),
        ];

        $this->update([
            'processing_status' => self::STATUS_FAILED,
            'processing_log' => $log,
        ]);

        return $this;
    }

    /**
     * Add tag
     */
    public function addTag(string $tag): self
    {
        $tags = $this->tags;
        
        if (!in_array($tag, $tags, true)) {
            $tags[] = $tag;
            $this->tags = $tags;
            $this->save();
        }

        return $this;
    }

    /**
     * Remove tag
     */
    public function removeTag(string $tag): self
    {
        $tags = array_filter($this->tags, fn($t) => $t !== $tag);
        $this->tags = array_values($tags);
        $this->save();

        return $this;
    }

    /**
     * Check if has tag
     */
    public function hasTag(string $tag): bool
    {
        return in_array($tag, $this->tags, true);
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Images only
     */
    public function scopeImages(Builder $query): Builder
    {
        return $query->where('type', self::TYPE_IMAGE);
    }

    /**
     * Scope: Videos only
     */
    public function scopeVideos(Builder $query): Builder
    {
        return $query->where('type', self::TYPE_VIDEO);
    }

    /**
     * Scope: Documents only
     */
    public function scopeDocuments(Builder $query): Builder
    {
        return $query->where('type', self::TYPE_DOCUMENT);
    }

    /**
     * Scope: Public media
     */
    public function scopePublic(Builder $query): Builder
    {
        return $query->where('is_public', true);
    }

    /**
     * Scope: Featured media
     */
    public function scopeFeatured(Builder $query): Builder
    {
        return $query->where('is_featured', true);
    }

    /**
     * Scope: In collection
     */
    public function scopeInCollection(Builder $query, string $collection): Builder
    {
        return $query->where('collection', $collection);
    }

    /**
     * Scope: With tag
     */
    public function scopeWithTag(Builder $query, string $tag): Builder
    {
        return $query->whereJsonContains('tags', $tag);
    }

    /**
     * Scope: Uploaded by user
     */
    public function scopeUploadedBy(Builder $query, int $userId): Builder
    {
        return $query->where('uploaded_by', $userId);
    }

    /**
     * Scope: Unoptimized images
     */
    public function scopeNeedsOptimization(Builder $query): Builder
    {
        return $query->where('type', self::TYPE_IMAGE)
            ->where('is_optimized', false);
    }

    /**
     * Scope: Expired media
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->whereNotNull('expires_at')
            ->where('expires_at', '<=', now());
    }

    /**
     * Scope: Unused media
     */
    public function scopeUnused(Builder $query): Builder
    {
        return $query->where('usage_count', 0);
    }

    /**
     * Scope: Recently uploaded
     */
    public function scopeRecent(Builder $query, int $days = 7): Builder
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    /**
     * Scope: Large files
     */
    public function scopeLargeFiles(Builder $query, int $minSizeMB = 10): Builder
    {
        return $query->where('size', '>=', $minSizeMB * 1048576);
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total_files' => static::count(),
            'total_size' => static::sum('size'),
            'total_images' => static::images()->count(),
            'total_videos' => static::videos()->count(),
            'total_documents' => static::documents()->count(),
            'total_downloads' => static::sum('download_count'),
            'total_views' => static::sum('view_count'),
            'needs_optimization' => static::needsOptimization()->count(),
            'unused_files' => static::unused()->count(),
        ];
    }

    /**
     * Static: Get storage usage by type
     */
    public static function getStorageUsageByType(): Collection
    {
        return static::selectRaw('type, COUNT(*) as count, SUM(size) as total_size')
            ->groupBy('type')
            ->get()
            ->mapWithKeys(fn($item) => [
                $item->type => [
                    'count' => $item->count,
                    'size' => $item->total_size,
                    'human_size' => self::formatBytes($item->total_size),
                ],
            ]);
    }

    /**
     * Static: Format bytes to human-readable
     */
    public static function formatBytes(int $bytes): string
    {
        if ($bytes >= 1073741824) {
            return number_format($bytes / 1073741824, 2) . ' GB';
        }

        if ($bytes >= 1048576) {
            return number_format($bytes / 1048576, 2) . ' MB';
        }

        if ($bytes >= 1024) {
            return number_format($bytes / 1024, 2) . ' KB';
        }

        return $bytes . ' bytes';
    }

    /**
     * Static: Clean up expired files
     */
    public static function cleanupExpired(bool $dryRun = true): array
    {
        $expired = static::expired()->get();

        if ($dryRun) {
            return [
                'would_delete' => $expired->count(),
                'files' => $expired->pluck('filename'),
            ];
        }

        $deleted = 0;
        foreach ($expired as $media) {
            if ($media->delete()) {
                $deleted++;
            }
        }

        return [
            'deleted' => $deleted,
            'failed' => $expired->count() - $deleted,
        ];
    }

    /**
     * Export to array for API
     */
    public function toMediaArray(): array
    {
        return [
            'id' => $this->id,
            'filename' => $this->filename,
            'url' => $this->getPublicUrl(),
            'thumbnail_url' => $this->getThumbnailUrlWithFallback(),
            'type' => $this->type,
            'mime_type' => $this->mime_type,
            'size' => $this->size,
            'human_size' => $this->human_readable_size,
            'alt_text' => $this->alt_text,
            'title' => $this->title,
            'caption' => $this->caption,
            'width' => $this->width,
            'height' => $this->height,
            'aspect_ratio' => $this->aspect_ratio,
            'duration' => $this->duration,
            'tags' => $this->tags,
            'collection' => $this->collection,
            'is_featured' => $this->is_featured,
            'uploaded_at' => $this->created_at->toIso8601String(),
        ];
    }
}