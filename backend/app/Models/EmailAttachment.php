<?php

declare(strict_types=1);

namespace App\Models;

use App\Enums\AttachmentScanStatus;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Carbon;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;

/**
 * EmailAttachment Model
 *
 * Represents an attachment on an email message with secure storage
 * and virus scanning capabilities.
 *
 * @property string $id UUID primary key
 * @property string $message_id Foreign key to email_messages
 * @property string $filename Stored filename (sanitized)
 * @property string $original_filename Original filename from email
 * @property string $content_type MIME type
 * @property string|null $content_id Content-ID for inline attachments
 * @property string $disposition attachment or inline
 * @property int $size_bytes File size in bytes
 * @property string|null $checksum_md5 MD5 hash for deduplication
 * @property string|null $checksum_sha256 SHA-256 hash for verification
 * @property string $storage_disk Storage disk name
 * @property string $storage_path Full storage path
 * @property string $storage_key Unique storage key
 * @property bool $is_inline Inline image flag
 * @property bool $is_encrypted Encryption status
 * @property AttachmentScanStatus $scan_status Virus scan status
 * @property Carbon|null $scanned_at Scan timestamp
 * @property string|null $scan_result Scan result details
 * @property array|null $metadata Additional metadata
 * @property Carbon|null $expires_at Expiration timestamp
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 *
 * @property-read EmailMessage $message
 * @property-read string $download_url
 * @property-read string $human_readable_size
 * @property-read bool $is_image
 * @property-read bool $is_document
 * @property-read bool $is_safe
 *
 * @method static Builder|self images()
 * @method static Builder|self documents()
 * @method static Builder|self clean()
 * @method static Builder|self infected()
 * @method static Builder|self pendingScan()
 * @method static Builder|self inline()
 * @method static Builder|self regular()
 *
 * @version 1.0.0
 */
class EmailAttachment extends Model
{
    use HasFactory;
    use HasUuids;
    use SoftDeletes;

    /**
     * The table associated with the model.
     */
    protected $table = 'email_attachments';

    /**
     * The primary key type.
     */
    protected $keyType = 'string';

    /**
     * Indicates if the IDs are auto-incrementing.
     */
    public $incrementing = false;

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'message_id',
        'filename',
        'original_filename',
        'content_type',
        'content_id',
        'disposition',
        'size_bytes',
        'checksum_md5',
        'checksum_sha256',
        'storage_disk',
        'storage_path',
        'storage_key',
        'is_inline',
        'is_encrypted',
        'scan_status',
        'scanned_at',
        'scan_result',
        'metadata',
        'expires_at',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'scan_status' => AttachmentScanStatus::class,
        'metadata' => 'array',
        'is_inline' => 'boolean',
        'is_encrypted' => 'boolean',
        'size_bytes' => 'integer',
        'scanned_at' => 'datetime',
        'expires_at' => 'datetime',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'human_readable_size',
        'is_image',
        'is_document',
        'is_safe',
        'file_extension',
    ];

    /**
     * The attributes that should be hidden.
     *
     * @var array<string>
     */
    protected $hidden = [
        'storage_path',
        'storage_key',
        'checksum_md5',
        'checksum_sha256',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'disposition' => 'attachment',
        'storage_disk' => 'r2',
        'is_inline' => false,
        'is_encrypted' => false,
        'scan_status' => AttachmentScanStatus::PENDING,
    ];

    /**
     * Image MIME types.
     *
     * @var array<string>
     */
    protected const IMAGE_TYPES = [
        'image/jpeg',
        'image/png',
        'image/gif',
        'image/webp',
        'image/svg+xml',
        'image/bmp',
        'image/tiff',
    ];

    /**
     * Document MIME types.
     *
     * @var array<string>
     */
    protected const DOCUMENT_TYPES = [
        'application/pdf',
        'application/msword',
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
        'application/vnd.ms-excel',
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
        'application/vnd.ms-powerpoint',
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
        'text/plain',
        'text/csv',
        'application/rtf',
    ];

    /**
     * Dangerous file extensions that should be blocked.
     *
     * @var array<string>
     */
    protected const BLOCKED_EXTENSIONS = [
        'exe', 'bat', 'cmd', 'com', 'pif', 'scr', 'vbs', 'vbe',
        'js', 'jse', 'ws', 'wsf', 'wsc', 'wsh', 'ps1', 'ps1xml',
        'ps2', 'ps2xml', 'psc1', 'psc2', 'msc', 'msi', 'msp', 'mst',
        'reg', 'dll', 'cpl', 'jar', 'hta', 'msh', 'msh1', 'msh2',
    ];

    /**
     * Maximum file size in bytes (25 MB).
     */
    protected const MAX_FILE_SIZE = 26214400;

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $attachment): void {
            $attachment->storage_key = $attachment->storage_key ?? $attachment->generateStorageKey();
            $attachment->filename = $attachment->filename ?? $attachment->sanitizeFilename($attachment->original_filename);
        });

        static::deleted(function (self $attachment): void {
            // Soft delete - don't actually remove file yet
            // Hard delete in scheduled cleanup job
        });

        static::forceDeleted(function (self $attachment): void {
            $attachment->deleteFile();
        });
    }

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    /**
     * Get the message this attachment belongs to.
     */
    public function message(): BelongsTo
    {
        return $this->belongsTo(EmailMessage::class, 'message_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    /**
     * Scope to image attachments.
     */
    public function scopeImages(Builder $query): Builder
    {
        return $query->whereIn('content_type', self::IMAGE_TYPES);
    }

    /**
     * Scope to document attachments.
     */
    public function scopeDocuments(Builder $query): Builder
    {
        return $query->whereIn('content_type', self::DOCUMENT_TYPES);
    }

    /**
     * Scope to clean (virus-free) attachments.
     */
    public function scopeClean(Builder $query): Builder
    {
        return $query->where('scan_status', AttachmentScanStatus::CLEAN);
    }

    /**
     * Scope to infected attachments.
     */
    public function scopeInfected(Builder $query): Builder
    {
        return $query->where('scan_status', AttachmentScanStatus::INFECTED);
    }

    /**
     * Scope to pending scan attachments.
     */
    public function scopePendingScan(Builder $query): Builder
    {
        return $query->where('scan_status', AttachmentScanStatus::PENDING);
    }

    /**
     * Scope to inline attachments.
     */
    public function scopeInline(Builder $query): Builder
    {
        return $query->where('is_inline', true);
    }

    /**
     * Scope to regular (non-inline) attachments.
     */
    public function scopeRegular(Builder $query): Builder
    {
        return $query->where('is_inline', false);
    }

    /**
     * Scope to safe downloadable attachments.
     */
    public function scopeSafe(Builder $query): Builder
    {
        return $query->whereIn('scan_status', [
            AttachmentScanStatus::CLEAN,
            AttachmentScanStatus::SKIPPED,
        ]);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    /**
     * Get human-readable file size.
     */
    protected function humanReadableSize(): Attribute
    {
        return Attribute::make(
            get: function (): string {
                $bytes = $this->size_bytes;

                if ($bytes < 1024) {
                    return $bytes . ' B';
                }

                $units = ['KB', 'MB', 'GB'];
                $i = 0;
                $bytes = $bytes / 1024;

                while ($bytes >= 1024 && $i < count($units) - 1) {
                    $bytes /= 1024;
                    $i++;
                }

                return round($bytes, 2) . ' ' . $units[$i];
            }
        );
    }

    /**
     * Check if attachment is an image.
     */
    protected function isImage(): Attribute
    {
        return Attribute::make(
            get: fn (): bool => in_array($this->content_type, self::IMAGE_TYPES, true)
        );
    }

    /**
     * Check if attachment is a document.
     */
    protected function isDocument(): Attribute
    {
        return Attribute::make(
            get: fn (): bool => in_array($this->content_type, self::DOCUMENT_TYPES, true)
        );
    }

    /**
     * Check if attachment is safe to download.
     */
    protected function isSafe(): Attribute
    {
        return Attribute::make(
            get: fn (): bool => $this->scan_status->isSafeToDownload()
        );
    }

    /**
     * Get file extension from filename.
     */
    protected function fileExtension(): Attribute
    {
        return Attribute::make(
            get: fn (): string => strtolower(pathinfo($this->original_filename, PATHINFO_EXTENSION))
        );
    }

    // =========================================================================
    // BUSINESS LOGIC
    // =========================================================================

    /**
     * Get temporary download URL.
     */
    public function getDownloadUrl(int $expirationMinutes = 60): ?string
    {
        if (!$this->is_safe) {
            return null;
        }

        return Storage::disk($this->storage_disk)->temporaryUrl(
            $this->storage_path,
            now()->addMinutes($expirationMinutes),
            [
                'ResponseContentDisposition' => "attachment; filename=\"{$this->original_filename}\"",
            ]
        );
    }

    /**
     * Get temporary view URL (for inline display).
     */
    public function getViewUrl(int $expirationMinutes = 60): ?string
    {
        if (!$this->is_safe) {
            return null;
        }

        return Storage::disk($this->storage_disk)->temporaryUrl(
            $this->storage_path,
            now()->addMinutes($expirationMinutes),
            [
                'ResponseContentDisposition' => "inline; filename=\"{$this->original_filename}\"",
                'ResponseContentType' => $this->content_type,
            ]
        );
    }

    /**
     * Get file contents.
     */
    public function getContents(): ?string
    {
        if (!$this->is_safe) {
            return null;
        }

        return Storage::disk($this->storage_disk)->get($this->storage_path);
    }

    /**
     * Check if file exists in storage.
     */
    public function existsInStorage(): bool
    {
        return Storage::disk($this->storage_disk)->exists($this->storage_path);
    }

    /**
     * Mark as scanned with result.
     */
    public function markAsScanned(AttachmentScanStatus $status, ?string $result = null): self
    {
        $this->update([
            'scan_status' => $status,
            'scanned_at' => now(),
            'scan_result' => $result,
        ]);

        return $this;
    }

    /**
     * Mark as clean (no virus found).
     */
    public function markAsClean(): self
    {
        return $this->markAsScanned(AttachmentScanStatus::CLEAN, 'No threats detected');
    }

    /**
     * Mark as infected.
     */
    public function markAsInfected(string $threatName): self
    {
        return $this->markAsScanned(AttachmentScanStatus::INFECTED, $threatName);
    }

    /**
     * Skip virus scanning.
     */
    public function skipScan(string $reason): self
    {
        return $this->markAsScanned(AttachmentScanStatus::SKIPPED, $reason);
    }

    /**
     * Check if file extension is blocked.
     */
    public function hasBlockedExtension(): bool
    {
        return in_array($this->file_extension, self::BLOCKED_EXTENSIONS, true);
    }

    /**
     * Check if file size exceeds maximum.
     */
    public function exceedsMaxSize(): bool
    {
        return $this->size_bytes > self::MAX_FILE_SIZE;
    }

    /**
     * Verify file integrity using checksum.
     */
    public function verifyIntegrity(): bool
    {
        if (!$this->existsInStorage() || empty($this->checksum_sha256)) {
            return false;
        }

        $contents = Storage::disk($this->storage_disk)->get($this->storage_path);
        $currentHash = hash('sha256', $contents);

        return hash_equals($this->checksum_sha256, $currentHash);
    }

    /**
     * Delete the actual file from storage.
     */
    public function deleteFile(): bool
    {
        if ($this->existsInStorage()) {
            return Storage::disk($this->storage_disk)->delete($this->storage_path);
        }

        return true;
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Generate unique storage key.
     */
    protected function generateStorageKey(): string
    {
        return Str::uuid()->toString();
    }

    /**
     * Sanitize filename for safe storage.
     */
    protected function sanitizeFilename(string $filename): string
    {
        // Remove path components
        $filename = basename($filename);

        // Remove null bytes and directory traversal attempts
        $filename = str_replace(["\0", '..', '/', '\\'], '', $filename);

        // Replace potentially problematic characters
        $filename = preg_replace('/[^\w\-\.]/', '_', $filename);

        // Ensure filename is not empty
        if (empty($filename) || $filename === '.') {
            $filename = 'attachment_' . time();
        }

        // Limit length
        $extension = pathinfo($filename, PATHINFO_EXTENSION);
        $basename = pathinfo($filename, PATHINFO_FILENAME);

        if (strlen($basename) > 200) {
            $basename = substr($basename, 0, 200);
        }

        return $extension ? "{$basename}.{$extension}" : $basename;
    }

    /**
     * Create attachment from Postmark webhook payload.
     *
     * @param array{
     *     Name: string,
     *     Content: string,
     *     ContentType: string,
     *     ContentLength: int,
     *     ContentID?: string
     * } $data
     */
    public static function createFromPostmarkPayload(
        string $messageId,
        array $data,
        string $storagePath
    ): self {
        $originalFilename = $data['Name'] ?? 'attachment';
        $contentType = $data['ContentType'] ?? 'application/octet-stream';
        $contentId = $data['ContentID'] ?? null;
        $sizeBytes = $data['ContentLength'] ?? 0;

        // Decode base64 content to calculate checksums
        $content = base64_decode($data['Content'] ?? '', true);
        $actualSize = $content !== false ? strlen($content) : $sizeBytes;

        return self::create([
            'message_id' => $messageId,
            'original_filename' => $originalFilename,
            'content_type' => $contentType,
            'content_id' => $contentId,
            'disposition' => $contentId ? 'inline' : 'attachment',
            'size_bytes' => $actualSize,
            'checksum_md5' => $content !== false ? md5($content) : null,
            'checksum_sha256' => $content !== false ? hash('sha256', $content) : null,
            'storage_path' => $storagePath,
            'is_inline' => !empty($contentId),
        ]);
    }

    /**
     * Get MIME type category.
     */
    public function getMimeCategory(): string
    {
        if ($this->is_image) {
            return 'image';
        }

        if ($this->is_document) {
            return 'document';
        }

        $type = explode('/', $this->content_type)[0] ?? 'other';

        return match ($type) {
            'audio' => 'audio',
            'video' => 'video',
            'text' => 'text',
            default => 'other',
        };
    }

    /**
     * Get icon name for file type.
     */
    public function getIcon(): string
    {
        if ($this->is_image) {
            return 'image';
        }

        return match ($this->file_extension) {
            'pdf' => 'file-pdf',
            'doc', 'docx' => 'file-word',
            'xls', 'xlsx' => 'file-excel',
            'ppt', 'pptx' => 'file-powerpoint',
            'zip', 'rar', '7z', 'tar', 'gz' => 'file-archive',
            'txt', 'md', 'rtf' => 'file-text',
            'mp3', 'wav', 'ogg', 'flac' => 'file-audio',
            'mp4', 'avi', 'mov', 'mkv' => 'file-video',
            default => 'file',
        };
    }
}
