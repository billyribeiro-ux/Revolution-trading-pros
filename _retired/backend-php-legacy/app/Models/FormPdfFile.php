<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Carbon;

/**
 * FormPdfFile Model - Generated PDF Files
 *
 * Tracks generated PDF files for form submissions.
 *
 * @property int $id
 * @property int $form_id
 * @property int $submission_id
 * @property int|null $template_id
 * @property string $filename
 * @property string $path
 * @property int|null $file_size
 * @property string $mime_type
 * @property string $status
 * @property Carbon $generated_at
 * @property Carbon|null $sent_at
 * @property Carbon|null $downloaded_at
 * @property int $download_count
 * @property Carbon $created_at
 * @property Carbon $updated_at
 */
class FormPdfFile extends Model
{
    use HasFactory;

    protected $table = 'form_pdf_files';

    protected $fillable = [
        'form_id',
        'submission_id',
        'template_id',
        'filename',
        'path',
        'file_size',
        'mime_type',
        'status',
        'generated_at',
        'sent_at',
        'downloaded_at',
        'download_count',
    ];

    protected $casts = [
        'form_id' => 'integer',
        'submission_id' => 'integer',
        'template_id' => 'integer',
        'file_size' => 'integer',
        'download_count' => 'integer',
        'generated_at' => 'datetime',
        'sent_at' => 'datetime',
        'downloaded_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'status' => 'generated',
        'mime_type' => 'application/pdf',
        'download_count' => 0,
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const STATUS_GENERATED = 'generated';
    public const STATUS_SENT = 'sent';
    public const STATUS_DOWNLOADED = 'downloaded';

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function form(): BelongsTo
    {
        return $this->belongsTo(Form::class);
    }

    public function submission(): BelongsTo
    {
        return $this->belongsTo(FormSubmission::class, 'submission_id');
    }

    public function template(): BelongsTo
    {
        return $this->belongsTo(FormPdfTemplate::class, 'template_id');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeForSubmission($query, int $submissionId)
    {
        return $query->where('submission_id', $submissionId);
    }

    public function scopeForForm($query, int $formId)
    {
        return $query->where('form_id', $formId);
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getUrlAttribute(): string
    {
        return Storage::url($this->path);
    }

    public function getFileSizeFormattedAttribute(): string
    {
        if (!$this->file_size) {
            return 'Unknown';
        }

        $units = ['B', 'KB', 'MB', 'GB'];
        $bytes = $this->file_size;
        $unitIndex = 0;

        while ($bytes >= 1024 && $unitIndex < count($units) - 1) {
            $bytes /= 1024;
            $unitIndex++;
        }

        return round($bytes, 2) . ' ' . $units[$unitIndex];
    }

    // =========================================================================
    // METHODS
    // =========================================================================

    /**
     * Check if file exists in storage.
     */
    public function exists(): bool
    {
        return Storage::exists($this->path);
    }

    /**
     * Get file contents.
     */
    public function getContents(): ?string
    {
        if (!$this->exists()) {
            return null;
        }

        return Storage::get($this->path);
    }

    /**
     * Record a download.
     */
    public function recordDownload(): void
    {
        $this->increment('download_count');
        $this->update([
            'status' => self::STATUS_DOWNLOADED,
            'downloaded_at' => now(),
        ]);
    }

    /**
     * Mark as sent.
     */
    public function markAsSent(): void
    {
        $this->update([
            'status' => self::STATUS_SENT,
            'sent_at' => now(),
        ]);
    }

    /**
     * Delete the file from storage.
     */
    public function deleteFile(): bool
    {
        if ($this->exists()) {
            return Storage::delete($this->path);
        }

        return true;
    }

    /**
     * Boot method - cleanup on delete.
     */
    protected static function boot()
    {
        parent::boot();

        static::deleting(function ($pdfFile) {
            $pdfFile->deleteFile();
        });
    }
}
