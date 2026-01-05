<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\Request;

/**
 * Revolution Trading Pros - AuditLog Model
 * Enterprise-grade audit logging for compliance and security
 *
 * @property int $id
 * @property int|null $user_id
 * @property int|null $user_session_id
 * @property string $action
 * @property string|null $entity_type
 * @property int|null $entity_id
 * @property string $ip_address
 * @property string|null $user_agent
 * @property array|null $old_values
 * @property array|null $new_values
 * @property array|null $metadata
 * @property string $severity
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */
class AuditLog extends Model
{
    use HasFactory;

    /**
     * The attributes that are mass assignable.
     */
    protected $fillable = [
        'user_id',
        'user_session_id',
        'action',
        'entity_type',
        'entity_id',
        'ip_address',
        'user_agent',
        'old_values',
        'new_values',
        'metadata',
        'severity',
    ];

    /**
     * The attributes that should be cast.
     */
    protected $casts = [
        'old_values' => 'array',
        'new_values' => 'array',
        'metadata' => 'array',
    ];

    /**
     * Get the user that performed the action.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Get the session associated with this log.
     */
    public function session(): BelongsTo
    {
        return $this->belongsTo(UserSession::class, 'user_session_id');
    }

    /**
     * Create a new audit log entry.
     */
    public static function log(
        string $action,
        ?string $entityType = null,
        ?int $entityId = null,
        ?array $oldValues = null,
        ?array $newValues = null,
        array $metadata = [],
        string $severity = 'info'
    ): self {
        $user = Auth::user();
        $sessionId = Request::header('X-Session-ID');
        $session = $sessionId ? UserSession::where('session_id', $sessionId)->first() : null;

        return self::create([
            'user_id' => $user?->id,
            'user_session_id' => $session?->id,
            'action' => $action,
            'entity_type' => $entityType,
            'entity_id' => $entityId,
            'ip_address' => Request::ip(),
            'user_agent' => Request::userAgent(),
            'old_values' => $oldValues,
            'new_values' => $newValues,
            'metadata' => $metadata,
            'severity' => $severity,
        ]);
    }

    /**
     * Log an authentication event.
     */
    public static function logAuth(string $action, ?User $user = null, array $metadata = []): self
    {
        return self::create([
            'user_id' => $user?->id ?? Auth::id(),
            'action' => $action,
            'entity_type' => 'User',
            'entity_id' => $user?->id ?? Auth::id(),
            'ip_address' => Request::ip(),
            'user_agent' => Request::userAgent(),
            'metadata' => $metadata,
            'severity' => in_array($action, ['login_failed', 'session_revoked', 'password_change']) ? 'warning' : 'info',
        ]);
    }

    /**
     * Log a model change.
     */
    public static function logModelChange(
        Model $model,
        string $action,
        ?array $oldValues = null,
        ?array $newValues = null
    ): self {
        return self::log(
            $action,
            get_class($model),
            $model->id,
            $oldValues,
            $newValues
        );
    }

    /**
     * Scope to filter by action.
     */
    public function scopeAction($query, string $action)
    {
        return $query->where('action', $action);
    }

    /**
     * Scope to filter by entity.
     */
    public function scopeForEntity($query, string $entityType, ?int $entityId = null)
    {
        $query->where('entity_type', $entityType);

        if ($entityId) {
            $query->where('entity_id', $entityId);
        }

        return $query;
    }

    /**
     * Scope to filter by user.
     */
    public function scopeForUser($query, int $userId)
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope to filter by severity.
     */
    public function scopeSeverity($query, string $severity)
    {
        return $query->where('severity', $severity);
    }

    /**
     * Scope to filter by date range.
     */
    public function scopeDateRange($query, $start, $end)
    {
        return $query->whereBetween('created_at', [$start, $end]);
    }
}
