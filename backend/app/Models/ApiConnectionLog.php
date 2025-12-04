<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

/**
 * API Connection Log Model
 *
 * Audit trail for all API connection activities.
 *
 * @property string $id
 * @property string $connection_id
 * @property string $action
 * @property string|null $status
 * @property array|null $details
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property int|null $user_id
 */
class ApiConnectionLog extends Model
{
    use HasFactory, HasUuids;

    protected $fillable = [
        'connection_id',
        'action',
        'status',
        'details',
        'ip_address',
        'user_agent',
        'user_id',
    ];

    protected $casts = [
        'details' => 'array',
    ];

    /**
     * Get the connection this log belongs to
     */
    public function connection(): BelongsTo
    {
        return $this->belongsTo(ApiConnection::class, 'connection_id');
    }

    /**
     * Get the user who performed the action
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    /**
     * Create a log entry
     */
    public static function log(
        string $connectionId,
        string $action,
        ?string $status = null,
        ?array $details = null,
        ?int $userId = null
    ): self {
        return self::create([
            'connection_id' => $connectionId,
            'action' => $action,
            'status' => $status,
            'details' => $details,
            'ip_address' => request()->ip(),
            'user_agent' => request()->userAgent(),
            'user_id' => $userId ?? auth()->id(),
        ]);
    }
}
