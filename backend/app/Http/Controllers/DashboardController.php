<?php

namespace App\Http\Controllers;

use App\Services\Dashboard\DashboardService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;

class DashboardController extends Controller
{
    public function __construct(
        private DashboardService $dashboardService
    ) {}

    public function index(Request $request): JsonResponse
    {
        $userId = $request->user()->id;
        $type = $request->query('type', 'user');

        $dashboard = $this->dashboardService->getOrCreateUserDashboard($userId, $type);
        $data = $this->dashboardService->getDashboardWithData($dashboard->id, $userId);

        return response()->json($data);
    }

    public function show(Request $request, string $id): JsonResponse
    {
        $userId = $request->user()->id;
        $data = $this->dashboardService->getDashboardWithData($id, $userId);

        return response()->json($data);
    }

    public function updateWidgetLayout(Request $request, string $widgetId): JsonResponse
    {
        $validated = $request->validate([
            'x' => 'required|integer|min:0',
            'y' => 'required|integer|min:0',
            'width' => 'required|integer|min:1',
            'height' => 'required|integer|min:1',
        ]);

        $widget = $this->dashboardService->updateWidgetLayout($widgetId, $validated);

        return response()->json($widget);
    }

    public function addWidget(Request $request, string $dashboardId): JsonResponse
    {
        $validated = $request->validate([
            'widget_type' => 'required|string',
            'title' => 'required|string',
            'position_x' => 'required|integer',
            'position_y' => 'required|integer',
            'width' => 'required|integer',
            'height' => 'required|integer',
            'config' => 'nullable|array',
        ]);

        $widget = $this->dashboardService->addWidget($dashboardId, $validated);

        return response()->json($widget, 201);
    }

    public function removeWidget(string $widgetId): JsonResponse
    {
        $this->dashboardService->removeWidget($widgetId);

        return response()->json(['message' => 'Widget removed successfully']);
    }
}
