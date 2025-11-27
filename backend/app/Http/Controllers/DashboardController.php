<?php

namespace App\Http\Controllers;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use App\Services\Dashboard\DashboardService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Gate;

class DashboardController extends Controller
{
    public function __construct(
        private DashboardService $dashboardService
    ) {}

    /**
     * Get user's dashboard
     */
    public function index(Request $request): JsonResponse
    {
        $user = $request->user();
        $type = $request->query('type', 'user');

        // Admin dashboard requires admin role
        if ($type === 'admin' && !$user->hasAnyRole(['admin', 'super-admin'])) {
            abort(403, 'Access denied to admin dashboard');
        }

        $dashboard = $this->dashboardService->getOrCreateUserDashboard($user->id, $type);
        $data = $this->dashboardService->getDashboardWithData($dashboard->id, $user->id);

        return response()->json($data);
    }

    /**
     * Get a specific dashboard
     */
    public function show(Request $request, string $id): JsonResponse
    {
        $user = $request->user();

        // Verify user owns this dashboard or is admin
        $dashboard = Dashboard::findOrFail($id);
        $this->authorizeDashboard($user, $dashboard);

        $data = $this->dashboardService->getDashboardWithData($id, $user->id);

        return response()->json($data);
    }

    /**
     * Update widget layout position
     */
    public function updateWidgetLayout(Request $request, string $widgetId): JsonResponse
    {
        $user = $request->user();

        // Verify user owns this widget's dashboard
        $widget = DashboardWidget::with('dashboard')->findOrFail($widgetId);
        $this->authorizeDashboard($user, $widget->dashboard);

        $validated = $request->validate([
            'x' => 'required|integer|min:0|max:12',
            'y' => 'required|integer|min:0|max:100',
            'width' => 'required|integer|min:1|max:12',
            'height' => 'required|integer|min:1|max:20',
        ]);

        $widget = $this->dashboardService->updateWidgetLayout($widgetId, $validated);

        return response()->json($widget);
    }

    /**
     * Add a widget to dashboard
     */
    public function addWidget(Request $request, string $dashboardId): JsonResponse
    {
        $user = $request->user();

        // Verify user owns this dashboard
        $dashboard = Dashboard::findOrFail($dashboardId);
        $this->authorizeDashboard($user, $dashboard);

        // Validate allowed widget types
        $allowedWidgetTypes = [
            'stats_overview', 'revenue_chart', 'users_chart', 'recent_orders',
            'recent_users', 'top_products', 'active_subscriptions', 'form_submissions',
            'email_stats', 'traffic_sources', 'popular_content', 'quick_actions',
            'system_health', 'notifications', 'calendar', 'notes', 'tasks',
            'seo_overview', 'analytics_realtime'
        ];

        $validated = $request->validate([
            'widget_type' => ['required', 'string', 'in:' . implode(',', $allowedWidgetTypes)],
            'title' => 'required|string|max:100',
            'position_x' => 'required|integer|min:0|max:12',
            'position_y' => 'required|integer|min:0|max:100',
            'width' => 'required|integer|min:1|max:12',
            'height' => 'required|integer|min:1|max:20',
            'config' => 'nullable|array',
        ]);

        // Limit widgets per dashboard to prevent abuse
        $widgetCount = $dashboard->widgets()->count();
        if ($widgetCount >= 20) {
            return response()->json([
                'message' => 'Maximum widget limit reached (20)',
            ], 422);
        }

        $widget = $this->dashboardService->addWidget($dashboardId, $validated);

        return response()->json($widget, 201);
    }

    /**
     * Remove a widget from dashboard
     */
    public function removeWidget(Request $request, string $widgetId): JsonResponse
    {
        $user = $request->user();

        // Verify user owns this widget's dashboard
        $widget = DashboardWidget::with('dashboard')->findOrFail($widgetId);
        $this->authorizeDashboard($user, $widget->dashboard);

        $this->dashboardService->removeWidget($widgetId);

        return response()->json(['message' => 'Widget removed successfully']);
    }

    /**
     * Authorize dashboard access
     */
    private function authorizeDashboard($user, Dashboard $dashboard): void
    {
        // Super-admin can access any dashboard
        if ($user->hasRole('super-admin')) {
            return;
        }

        // Admin can access admin dashboards
        if ($dashboard->type === 'admin' && $user->hasRole('admin')) {
            return;
        }

        // User can only access their own dashboards
        if ($dashboard->user_id !== $user->id) {
            abort(403, 'You do not have permission to access this dashboard');
        }
    }
}
