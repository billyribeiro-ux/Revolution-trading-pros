<?php

namespace App\Services\Dashboard;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

class WidgetSharingService
{
    /**
     * Share widget with specific users
     */
    public function shareWidget(string $widgetId, array $userIds, array $permissions = []): array
    {
        $widget = DashboardWidget::findOrFail($widgetId);
        
        $defaultPermissions = [
            'can_view' => true,
            'can_edit' => false,
            'can_delete' => false,
        ];

        $permissions = array_merge($defaultPermissions, $permissions);

        $shares = [];
        foreach ($userIds as $userId) {
            $share = DB::table('widget_shares')->updateOrInsert(
                [
                    'widget_id' => $widgetId,
                    'user_id' => $userId,
                ],
                [
                    'can_view' => $permissions['can_view'],
                    'can_edit' => $permissions['can_edit'],
                    'can_delete' => $permissions['can_delete'],
                    'shared_at' => now(),
                    'updated_at' => now(),
                ]
            );

            $shares[] = [
                'widget_id' => $widgetId,
                'user_id' => $userId,
                'permissions' => $permissions,
            ];
        }

        return $shares;
    }

    /**
     * Generate shareable link for widget
     */
    public function generateShareLink(string $widgetId, ?int $expiresInDays = null): string
    {
        $widget = DashboardWidget::findOrFail($widgetId);
        
        $token = Str::random(32);
        $expiresAt = $expiresInDays ? now()->addDays($expiresInDays) : null;

        DB::table('widget_share_links')->insert([
            'widget_id' => $widgetId,
            'token' => $token,
            'expires_at' => $expiresAt,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        return url("/shared/widget/{$token}");
    }

    /**
     * Revoke widget share for specific user
     */
    public function revokeShare(string $widgetId, int $userId): bool
    {
        return DB::table('widget_shares')
            ->where('widget_id', $widgetId)
            ->where('user_id', $userId)
            ->delete() > 0;
    }

    /**
     * Revoke share link
     */
    public function revokeShareLink(string $token): bool
    {
        return DB::table('widget_share_links')
            ->where('token', $token)
            ->delete() > 0;
    }

    /**
     * Get all users widget is shared with
     */
    public function getSharedUsers(string $widgetId): array
    {
        $shares = DB::table('widget_shares')
            ->join('users', 'widget_shares.user_id', '=', 'users.id')
            ->where('widget_shares.widget_id', $widgetId)
            ->select([
                'users.id',
                'users.name',
                'users.email',
                'widget_shares.can_view',
                'widget_shares.can_edit',
                'widget_shares.can_delete',
                'widget_shares.shared_at',
            ])
            ->get()
            ->toArray();

        return $shares;
    }

    /**
     * Get all share links for widget
     */
    public function getShareLinks(string $widgetId): array
    {
        return DB::table('widget_share_links')
            ->where('widget_id', $widgetId)
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->get()
            ->toArray();
    }

    /**
     * Check if user has access to widget
     */
    public function canUserAccessWidget(string $widgetId, int $userId): bool
    {
        $widget = DashboardWidget::findOrFail($widgetId);

        // Owner always has access
        if ($widget->dashboard->user_id === $userId) {
            return true;
        }

        // Check if shared with user
        $share = DB::table('widget_shares')
            ->where('widget_id', $widgetId)
            ->where('user_id', $userId)
            ->where('can_view', true)
            ->exists();

        return $share;
    }

    /**
     * Copy shared widget to user's dashboard
     */
    public function copyWidgetToDashboard(string $widgetId, string $targetDashboardId): DashboardWidget
    {
        $sourceWidget = DashboardWidget::findOrFail($widgetId);
        $targetDashboard = Dashboard::findOrFail($targetDashboardId);

        $newWidget = $sourceWidget->replicate();
        $newWidget->dashboard_id = $targetDashboard->id;
        $newWidget->save();

        return $newWidget;
    }

    /**
     * Get widget by share token
     */
    public function getWidgetByShareToken(string $token): ?DashboardWidget
    {
        $shareLink = DB::table('widget_share_links')
            ->where('token', $token)
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->first();

        if (!$shareLink) {
            return null;
        }

        return DashboardWidget::find($shareLink->widget_id);
    }

    /**
     * Update share permissions
     */
    public function updateSharePermissions(string $widgetId, int $userId, array $permissions): bool
    {
        return DB::table('widget_shares')
            ->where('widget_id', $widgetId)
            ->where('user_id', $userId)
            ->update([
                'can_view' => $permissions['can_view'] ?? true,
                'can_edit' => $permissions['can_edit'] ?? false,
                'can_delete' => $permissions['can_delete'] ?? false,
                'updated_at' => now(),
            ]) > 0;
    }

    /**
     * Get all widgets shared with user
     */
    public function getSharedWithUser(int $userId): array
    {
        return DB::table('widget_shares')
            ->join('dashboard_widgets', 'widget_shares.widget_id', '=', 'dashboard_widgets.id')
            ->join('dashboards', 'dashboard_widgets.dashboard_id', '=', 'dashboards.id')
            ->join('users', 'dashboards.user_id', '=', 'users.id')
            ->where('widget_shares.user_id', $userId)
            ->select([
                'dashboard_widgets.*',
                'dashboards.name as dashboard_name',
                'users.name as owner_name',
                'widget_shares.can_view',
                'widget_shares.can_edit',
                'widget_shares.can_delete',
                'widget_shares.shared_at',
            ])
            ->get()
            ->toArray();
    }

    /**
     * Bulk share widget with multiple users
     */
    public function bulkShare(string $widgetId, array $userEmails, array $permissions = []): array
    {
        $users = User::whereIn('email', $userEmails)->get();
        $userIds = $users->pluck('id')->toArray();

        return $this->shareWidget($widgetId, $userIds, $permissions);
    }

    /**
     * Get share statistics for widget
     */
    public function getShareStatistics(string $widgetId): array
    {
        $totalShares = DB::table('widget_shares')
            ->where('widget_id', $widgetId)
            ->count();

        $activeLinks = DB::table('widget_share_links')
            ->where('widget_id', $widgetId)
            ->where(function ($query) {
                $query->whereNull('expires_at')
                    ->orWhere('expires_at', '>', now());
            })
            ->count();

        $viewCount = DB::table('widget_views')
            ->where('widget_id', $widgetId)
            ->count();

        return [
            'total_shares' => $totalShares,
            'active_links' => $activeLinks,
            'view_count' => $viewCount,
        ];
    }
}
