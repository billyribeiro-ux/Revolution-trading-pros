<?php

namespace App\Services\Dashboard;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use App\Models\DashboardPermission;
use App\Models\DashboardActivityLog;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;

class DashboardService
{
    public function __construct(
        private WidgetDataProviderService $widgetDataProvider,
        private DashboardCacheService $cacheService
    ) {}

    /**
     * Get or create default dashboard for user
     */
    public function getOrCreateUserDashboard(int $userId, string $type = 'user'): Dashboard
    {
        $dashboard = Dashboard::where('user_id', $userId)
            ->where('dashboard_type', $type)
            ->where('is_default', true)
            ->first();

        if (!$dashboard) {
            $dashboard = $this->createDefaultDashboard($userId, $type);
        }

        return $dashboard;
    }

    /**
     * Create default dashboard with preset widgets
     */
    public function createDefaultDashboard(int $userId, string $type = 'user'): Dashboard
    {
        return DB::transaction(function () use ($userId, $type) {
            $dashboard = Dashboard::create([
                'user_id' => $userId,
                'dashboard_type' => $type,
                'name' => $type === 'admin' ? 'Admin Dashboard' : 'My Dashboard',
                'description' => 'Default dashboard',
                'is_default' => true,
                'is_public' => false,
                'grid_columns' => 12,
                'grid_row_height' => 80,
                'grid_gap' => 16,
            ]);

            // Create default widgets based on type
            $widgets = $type === 'admin' 
                ? $this->getAdminDefaultWidgets() 
                : $this->getUserDefaultWidgets();

            foreach ($widgets as $widgetConfig) {
                DashboardWidget::create(array_merge($widgetConfig, [
                    'dashboard_id' => $dashboard->id,
                ]));
            }

            DashboardActivityLog::logActivity(
                $userId,
                'Dashboard',
                $dashboard->id,
                'created',
                "Created default {$type} dashboard"
            );

            return $dashboard;
        });
    }

    /**
     * Get dashboard with all widgets and cached data
     */
    public function getDashboardWithData(string $dashboardId, int $userId): array
    {
        $dashboard = Dashboard::with('widgets')->findOrFail($dashboardId);

        // Check permissions
        if (!$this->canUserAccessDashboard($dashboard, $userId)) {
            throw new \Exception('Unauthorized access to dashboard');
        }

        $widgetsWithData = $dashboard->widgets->map(function ($widget) {
            return [
                'id' => $widget->id,
                'widget_type' => $widget->widget_type,
                'title' => $widget->title,
                'position_x' => $widget->position_x,
                'position_y' => $widget->position_y,
                'width' => $widget->width,
                'height' => $widget->height,
                'config' => $widget->config,
                'refresh_interval' => $widget->refresh_interval,
                'is_visible' => $widget->is_visible,
                'data' => $this->widgetDataProvider->getWidgetData($widget),
            ];
        });

        return [
            'dashboard' => $dashboard,
            'widgets' => $widgetsWithData,
        ];
    }

    /**
     * Update widget position and size
     */
    public function updateWidgetLayout(string $widgetId, array $layout): DashboardWidget
    {
        $widget = DashboardWidget::findOrFail($widgetId);
        
        $widget->update([
            'position_x' => $layout['x'] ?? $widget->position_x,
            'position_y' => $layout['y'] ?? $widget->position_y,
            'width' => $layout['width'] ?? $widget->width,
            'height' => $layout['height'] ?? $widget->height,
        ]);

        $this->cacheService->invalidateWidgetCache($widgetId);

        return $widget;
    }

    /**
     * Add widget to dashboard
     */
    public function addWidget(string $dashboardId, array $widgetData): DashboardWidget
    {
        $dashboard = Dashboard::findOrFail($dashboardId);

        $widget = DashboardWidget::create(array_merge($widgetData, [
            'dashboard_id' => $dashboard->id,
        ]));

        DashboardActivityLog::logActivity(
            $dashboard->user_id,
            'DashboardWidget',
            $widget->id,
            'created',
            "Added widget: {$widget->title}"
        );

        return $widget;
    }

    /**
     * Remove widget from dashboard
     */
    public function removeWidget(string $widgetId): bool
    {
        $widget = DashboardWidget::findOrFail($widgetId);
        
        DashboardActivityLog::logActivity(
            $widget->dashboard->user_id,
            'DashboardWidget',
            $widget->id,
            'deleted',
            "Removed widget: {$widget->title}"
        );

        $this->cacheService->invalidateWidgetCache($widgetId);
        
        return $widget->delete();
    }

    /**
     * Check if user can access dashboard
     */
    private function canUserAccessDashboard(Dashboard $dashboard, int $userId): bool
    {
        // Owner always has access
        if ($dashboard->user_id === $userId) {
            return true;
        }

        // Check if public
        if ($dashboard->is_public) {
            return true;
        }

        // Check shared_with
        if (in_array($userId, $dashboard->shared_with ?? [])) {
            return true;
        }

        // Check permissions table
        return DashboardPermission::canUserAccess($dashboard->id, $userId, 'can_view');
    }

    /**
     * Get default admin widgets configuration
     */
    private function getAdminDefaultWidgets(): array
    {
        return [
            [
                'widget_type' => 'system_health',
                'title' => 'System Health',
                'position_x' => 0,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['show_all_services' => true],
                'refresh_interval' => 60,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'revenue_mrr',
                'title' => 'Monthly Recurring Revenue',
                'position_x' => 6,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'user_growth',
                'title' => 'User Growth',
                'position_x' => 0,
                'position_y' => 4,
                'width' => 4,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'subscription_churn',
                'title' => 'Churn Rate',
                'position_x' => 4,
                'position_y' => 4,
                'width' => 4,
                'height' => 4,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'recent_activity',
                'title' => 'Recent Activity',
                'position_x' => 8,
                'position_y' => 4,
                'width' => 4,
                'height' => 8,
                'config' => ['limit' => 20],
                'refresh_interval' => 60,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'email_performance',
                'title' => 'Email Performance',
                'position_x' => 0,
                'position_y' => 8,
                'width' => 4,
                'height' => 4,
                'config' => ['period' => '7d'],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'crm_pipeline',
                'title' => 'CRM Pipeline',
                'position_x' => 4,
                'position_y' => 8,
                'width' => 4,
                'height' => 4,
                'config' => [],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
        ];
    }

    /**
     * Get default user widgets configuration
     */
    private function getUserDefaultWidgets(): array
    {
        return [
            [
                'widget_type' => 'subscription_status',
                'title' => 'My Subscription',
                'position_x' => 0,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => [],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'recent_courses',
                'title' => 'Recent Courses',
                'position_x' => 6,
                'position_y' => 0,
                'width' => 6,
                'height' => 4,
                'config' => ['limit' => 5],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'trading_performance',
                'title' => 'Trading Performance',
                'position_x' => 0,
                'position_y' => 4,
                'width' => 8,
                'height' => 6,
                'config' => ['period' => '30d'],
                'refresh_interval' => 300,
                'is_visible' => true,
            ],
            [
                'widget_type' => 'notifications',
                'title' => 'Notifications',
                'position_x' => 8,
                'position_y' => 4,
                'width' => 4,
                'height' => 6,
                'config' => ['limit' => 10],
                'refresh_interval' => 60,
                'is_visible' => true,
            ],
        ];
    }
}
