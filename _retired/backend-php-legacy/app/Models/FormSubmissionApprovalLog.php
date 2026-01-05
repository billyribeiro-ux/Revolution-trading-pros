<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Factories\HasFactory;

/**
 * FormSubmissionApprovalLog Model - FluentForms 6.1.8 (December 2025)
 *
 * Tracks approval workflow history for form submissions.
 * Provides audit trail for GDPR compliance and workflow analysis.
 *
 * @property int $id
 * @property int $submission_id
 * @property int $admin_id
 * @property string $status
 * @property string|null $previous_status
 * @property string|null $note
 * @property string|null $admin_name
 * @property string|null $admin_email
 * @property string|null $ip_address
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 *
 * @property-read FormSubmission $submission
 * @property-read User $admin
 */
class FormSubmissionApprovalLog extends Model
{
    use HasFactory;

    protected $table = 'form_submission_approval_logs';

    protected $fillable = [
        'submission_id',
        'admin_id',
        'status',
        'previous_status',
        'note',
        'admin_name',
        'admin_email',
        'ip_address',
    ];

    protected $casts = [
        'submission_id' => 'integer',
        'admin_id' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    // =========================================================================
    // CONSTANTS
    // =========================================================================

    public const STATUS_PENDING = 'pending';
    public const STATUS_APPROVED = 'approved';
    public const STATUS_REJECTED = 'rejected';
    public const STATUS_NEEDS_REVISION = 'needs_revision';
    public const STATUS_ON_HOLD = 'on_hold';

    public const STATUSES = [
        self::STATUS_PENDING,
        self::STATUS_APPROVED,
        self::STATUS_REJECTED,
        self::STATUS_NEEDS_REVISION,
        self::STATUS_ON_HOLD,
    ];

    public const STATUS_LABELS = [
        self::STATUS_PENDING => 'Pending Review',
        self::STATUS_APPROVED => 'Approved',
        self::STATUS_REJECTED => 'Rejected',
        self::STATUS_NEEDS_REVISION => 'Needs Revision',
        self::STATUS_ON_HOLD => 'On Hold',
    ];

    // =========================================================================
    // RELATIONSHIPS
    // =========================================================================

    public function submission(): BelongsTo
    {
        return $this->belongsTo(FormSubmission::class, 'submission_id');
    }

    public function admin(): BelongsTo
    {
        return $this->belongsTo(User::class, 'admin_id');
    }

    // =========================================================================
    // ACCESSORS
    // =========================================================================

    public function getStatusLabelAttribute(): string
    {
        return self::STATUS_LABELS[$this->status] ?? ucfirst($this->status);
    }

    public function getFormattedDateAttribute(): string
    {
        return $this->created_at->format('M d, Y \a\t g:i A');
    }

    // =========================================================================
    // SCOPES
    // =========================================================================

    public function scopeForSubmission($query, int $submissionId)
    {
        return $query->where('submission_id', $submissionId);
    }

    public function scopeByAdmin($query, int $adminId)
    {
        return $query->where('admin_id', $adminId);
    }

    public function scopeByStatus($query, string $status)
    {
        return $query->where('status', $status);
    }

    public function scopeRecent($query, int $days = 30)
    {
        return $query->where('created_at', '>=', now()->subDays($days));
    }

    // =========================================================================
    // STATIC METHODS
    // =========================================================================

    public static function logAction(
        int $submissionId,
        int $adminId,
        string $status,
        ?string $previousStatus = null,
        ?string $note = null
    ): self {
        $admin = User::find($adminId);

        return self::create([
            'submission_id' => $submissionId,
            'admin_id' => $adminId,
            'status' => $status,
            'previous_status' => $previousStatus,
            'note' => $note,
            'admin_name' => $admin?->name,
            'admin_email' => $admin?->email,
            'ip_address' => request()?->ip(),
        ]);
    }
}
