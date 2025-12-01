<?php

declare(strict_types=1);

namespace App\Broadcasting;

use App\Models\User;
use App\Models\Dashboard;
use App\Models\DashboardWidget;

/**
 * DashboardChannel - Authorization for dashboard real-time updates
 *
 * Handles authorization for channels like:
 * - dashboard.{dashboardId}
 * - dashboard.widget.{widgetId}
 *
 * @version 1.0.0
 * @level L8 Principal Engineer
 */
class DashboardChannel
{
    /**
     * Authenticate the user's access to the dashboard channel.
     */
    public function join(User $user, string $dashboardId): bool|array
    {
        // Check if user owns or has access to the dashboard
        $dashboard = Dashboard::find($dashboardId);

        if (!$dashboard) {
            return false;
        }

        // Owner has access
        if ($dashboard->user_id === $user->id) {
            return [
                'id' => $user->id,
                'name' => $user->name,
                'role' => 'owner',
            ];
        }

        // Check shared access
        if ($dashboard->sharedUsers()->where('user_id', $user->id)->exists()) {
            return [
                'id' => $user->id,
                'name' => $user->name,
                'role' => 'viewer',
            ];
        }

        // Admin access
        if ($user->hasRole(['admin', 'super-admin'])) {
            return [
                'id' => $user->id,
                'name' => $user->name,
                'role' => 'admin',
            ];
        }

        return false;
    }

    /**
     * Authenticate the user's access to the widget channel.
     */
    public function joinWidget(User $user, string $widgetId): bool|array
    {
        $widget = DashboardWidget::find($widgetId);

        if (!$widget) {
            return false;
        }

        // Delegate to dashboard authorization
        return $this->join($user, (string) $widget->dashboard_id);
    }
}
