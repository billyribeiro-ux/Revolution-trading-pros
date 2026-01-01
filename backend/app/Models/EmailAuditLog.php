<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Str;

/**
 * EmailAuditLog Model
 *
 * Enterprise audit logging for email marketing system with comprehensive
 * tracking of all admin actions for compliance and debugging.
 *
 * @property int $id
 * @property string $uuid
 * @property int|null $user_id
 * @property string|null $user_email
 * @property string|null $user_name
 * @property string|null $user_role
 * @property string $action
 * @property string $resource_type
 * @property int|null $resource_id
 * @property string|null $resource_name
 * @property array|null $old_values
 * @property array|null $new_values
 * @property array|null $changed_fields
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property string|null $request_method
 * @property string|null $request_url
 * @property string|null $request_id
 * @property string|null $session_id
 * @property string|null $device_type
 * @property string|null $browser
 * @property string|null $os
 * @property string|null $country
 * @property string|null $city
 * @property string|null $timezone
 * @property string $status
 * @property string|null $error_message
 * @property array|null $metadata
 * @property array|null $tags
 * @property int|null $duration_ms
 * @property \Carbon\Carbon $created_at
 */
class EmailAuditLog extends Model
{
    public $timestamps = false;

    protected $fillable = [
        'uuid',
        'user_id',
        'user_email',
        'user_name',
        'user_role',
        'action',
        'resource_type',
        'resource_id',
        'resource_name',
        'old_values',
        'new_values',
        'changed_fields',
        'ip_address',
        'user_agent',
        'request_method',
        'request_url',
        'request_id',
        'session_id',
        'device_type',
        'browser',
        'os',
        'country',
        'city',
        'timezone',
        'status',
        'error_message',
        'metadata',
        'tags',
        'duration_ms',
        'created_at',
    ];

    protected $casts = [
        'old_values' => 'array',
        'new_values' => 'array',
        'changed_fields' => 'array',
        'metadata' => 'array',
        'tags' => 'array',
        'created_at' => 'datetime',
    ];

    /**
     * Action types
     */
    public const ACTION_CREATE = 'create';
    public const ACTION_UPDATE = 'update';
    public const ACTION_DELETE = 'delete';
    public const ACTION_SEND = 'send';
    public const ACTION_SCHEDULE = 'schedule';
    public const ACTION_CANCEL = 'cancel';
    public const ACTION_DUPLICATE = 'duplicate';
    public const ACTION_PUBLISH = 'publish';
    public const ACTION_UNPUBLISH = 'unpublish';
    public const ACTION_APPROVE = 'approve';
    public const ACTION_REJECT = 'reject';
    public const ACTION_IMPORT = 'import';
    public const ACTION_EXPORT = 'export';
    public const ACTION_TEST = 'test';
    public const ACTION_PREVIEW = 'preview';
    public const ACTION_SUBSCRIBE = 'subscribe';
    public const ACTION_UNSUBSCRIBE = 'unsubscribe';
    public const ACTION_BLOCK = 'block';
    public const ACTION_UNBLOCK = 'unblock';
    public const ACTION_SETTINGS_CHANGE = 'settings_change';
    public const ACTION_LOGIN = 'login';
    public const ACTION_LOGOUT = 'logout';

    /**
     * Resource types
     */
    public const RESOURCE_CAMPAIGN = 'campaign';
    public const RESOURCE_TEMPLATE = 'template';
    public const RESOURCE_SUBSCRIBER = 'subscriber';
    public const RESOURCE_SEGMENT = 'segment';
    public const RESOURCE_AUTOMATION = 'automation';
    public const RESOURCE_SEQUENCE = 'sequence';
    public const RESOURCE_SETTINGS = 'settings';
    public const RESOURCE_WEBHOOK = 'webhook';
    public const RESOURCE_API_KEY = 'api_key';

    /**
     * Status types
     */
    public const STATUS_SUCCESS = 'success';
    public const STATUS_FAILED = 'failed';
    public const STATUS_PENDING = 'pending';

    /**
     * Boot method
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $log) {
            $log->uuid = $log->uuid ?? (string) Str::uuid();
            $log->created_at = $log->created_at ?? now();
        });
    }

    /**
     * Get the user that performed the action
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Scope: By action
     */
    public function scopeByAction(Builder $query, string $action): Builder
    {
        return $query->where('action', $action);
    }

    /**
     * Scope: By resource type
     */
    public function scopeByResourceType(Builder $query, string $type): Builder
    {
        return $query->where('resource_type', $type);
    }

    /**
     * Scope: By user
     */
    public function scopeByUser(Builder $query, int $userId): Builder
    {
        return $query->where('user_id', $userId);
    }

    /**
     * Scope: By status
     */
    public function scopeByStatus(Builder $query, string $status): Builder
    {
        return $query->where('status', $status);
    }

    /**
     * Scope: Failed actions
     */
    public function scopeFailed(Builder $query): Builder
    {
        return $query->where('status', self::STATUS_FAILED);
    }

    /**
     * Scope: Date range
     */
    public function scopeDateRange(Builder $query, $from, $to): Builder
    {
        return $query->whereBetween('created_at', [$from, $to]);
    }

    /**
     * Scope: Today
     */
    public function scopeToday(Builder $query): Builder
    {
        return $query->whereDate('created_at', today());
    }

    /**
     * Scope: This week
     */
    public function scopeThisWeek(Builder $query): Builder
    {
        return $query->whereBetween('created_at', [now()->startOfWeek(), now()->endOfWeek()]);
    }

    /**
     * Scope: Search
     */
    public function scopeSearch(Builder $query, string $term): Builder
    {
        return $query->where(function ($q) use ($term) {
            $q->where('resource_name', 'like', "%{$term}%")
              ->orWhere('user_email', 'like', "%{$term}%")
              ->orWhere('user_name', 'like', "%{$term}%")
              ->orWhere('ip_address', 'like', "%{$term}%");
        });
    }

    /**
     * Get human-readable action description
     */
    public function getDescriptionAttribute(): string
    {
        $user = $this->user_name ?? $this->user_email ?? 'System';
        $resource = $this->resource_name ?? "#{$this->resource_id}";

        return match ($this->action) {
            self::ACTION_CREATE => "{$user} created {$this->resource_type} '{$resource}'",
            self::ACTION_UPDATE => "{$user} updated {$this->resource_type} '{$resource}'",
            self::ACTION_DELETE => "{$user} deleted {$this->resource_type} '{$resource}'",
            self::ACTION_SEND => "{$user} sent {$this->resource_type} '{$resource}'",
            self::ACTION_SCHEDULE => "{$user} scheduled {$this->resource_type} '{$resource}'",
            self::ACTION_CANCEL => "{$user} cancelled {$this->resource_type} '{$resource}'",
            self::ACTION_DUPLICATE => "{$user} duplicated {$this->resource_type} '{$resource}'",
            self::ACTION_PUBLISH => "{$user} published {$this->resource_type} '{$resource}'",
            self::ACTION_SUBSCRIBE => "{$user} subscribed to newsletter",
            self::ACTION_UNSUBSCRIBE => "{$user} unsubscribed from newsletter",
            default => "{$user} performed {$this->action} on {$this->resource_type}",
        };
    }

    /**
     * Get changes summary
     */
    public function getChangesSummaryAttribute(): array
    {
        if (empty($this->changed_fields)) {
            return [];
        }

        $summary = [];
        foreach ($this->changed_fields as $field) {
            $old = $this->old_values[$field] ?? null;
            $new = $this->new_values[$field] ?? null;
            $summary[$field] = [
                'old' => $old,
                'new' => $new,
            ];
        }

        return $summary;
    }
}
