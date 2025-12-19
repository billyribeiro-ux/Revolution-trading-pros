<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class DashboardPermission extends Model
{
    use HasFactory;

    protected $fillable = [
        'dashboard_id',
        'role',
        'user_id',
        'can_view',
        'can_edit',
        'can_delete',
        'can_share',
    ];

    protected $casts = [
        'can_view' => 'boolean',
        'can_edit' => 'boolean',
        'can_delete' => 'boolean',
        'can_share' => 'boolean',
    ];

    public function dashboard(): BelongsTo
    {
        return $this->belongsTo(Dashboard::class);
    }

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }

    public static function canUserAccess(int $dashboardId, int $userId, string $permission = 'can_view'): bool
    {
        return self::where('dashboard_id', $dashboardId)
            ->where(function ($query) use ($userId) {
                $query->where('user_id', $userId)
                    ->orWhereHas('dashboard.user', function ($q) use ($userId) {
                        $q->where('id', $userId);
                    });
            })
            ->where($permission, true)
            ->exists();
    }
}
