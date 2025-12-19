<?php

namespace App\Services\Dashboard;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Storage;
use Carbon\Carbon;

class DashboardExportService
{
    /**
     * Export dashboard to JSON format
     */
    public function exportDashboard(string $dashboardId, bool $includeData = false): array
    {
        $dashboard = Dashboard::with('widgets')->findOrFail($dashboardId);

        $export = [
            'version' => '1.0.0',
            'exported_at' => now()->toIso8601String(),
            'dashboard' => [
                'name' => $dashboard->name,
                'description' => $dashboard->description,
                'dashboard_type' => $dashboard->dashboard_type,
                'grid_columns' => $dashboard->grid_columns,
                'grid_row_height' => $dashboard->grid_row_height,
                'grid_gap' => $dashboard->grid_gap,
            ],
            'widgets' => $dashboard->widgets->map(function ($widget) use ($includeData) {
                $widgetData = [
                    'widget_type' => $widget->widget_type,
                    'title' => $widget->title,
                    'position_x' => $widget->position_x,
                    'position_y' => $widget->position_y,
                    'width' => $widget->width,
                    'height' => $widget->height,
                    'config' => $widget->config,
                    'refresh_interval' => $widget->refresh_interval,
                    'is_visible' => $widget->is_visible,
                ];

                if ($includeData) {
                    $widgetData['data'] = $widget->data;
                }

                return $widgetData;
            })->toArray(),
        ];

        return $export;
    }

    /**
     * Export dashboard to JSON file
     */
    public function exportToFile(string $dashboardId, bool $includeData = false): string
    {
        $export = $this->exportDashboard($dashboardId, $includeData);
        $dashboard = Dashboard::findOrFail($dashboardId);
        
        $filename = sprintf(
            'dashboard_%s_%s.json',
            $dashboard->name,
            now()->format('Y-m-d_His')
        );

        $filename = preg_replace('/[^A-Za-z0-9_\-.]/', '_', $filename);
        
        Storage::disk('local')->put(
            "exports/dashboards/{$filename}",
            json_encode($export, JSON_PRETTY_PRINT)
        );

        return $filename;
    }

    /**
     * Import dashboard from JSON
     */
    public function importDashboard(array $data, int $userId, bool $overwrite = false): Dashboard
    {
        return DB::transaction(function () use ($data, $userId, $overwrite) {
            // Validate import data
            $this->validateImportData($data);

            // Check if dashboard exists
            $existingDashboard = Dashboard::where('user_id', $userId)
                ->where('name', $data['dashboard']['name'])
                ->first();

            if ($existingDashboard && !$overwrite) {
                throw new \Exception('Dashboard with this name already exists. Use overwrite option.');
            }

            // Create or update dashboard
            $dashboard = $existingDashboard ?: new Dashboard();
            $dashboard->fill([
                'user_id' => $userId,
                'name' => $data['dashboard']['name'],
                'description' => $data['dashboard']['description'] ?? null,
                'dashboard_type' => $data['dashboard']['dashboard_type'] ?? 'custom',
                'grid_columns' => $data['dashboard']['grid_columns'] ?? 12,
                'grid_row_height' => $data['dashboard']['grid_row_height'] ?? 80,
                'grid_gap' => $data['dashboard']['grid_gap'] ?? 16,
                'is_default' => false,
                'is_public' => false,
            ]);
            $dashboard->save();

            // Delete existing widgets if overwriting
            if ($existingDashboard) {
                $dashboard->widgets()->delete();
            }

            // Import widgets
            foreach ($data['widgets'] as $widgetData) {
                DashboardWidget::create([
                    'dashboard_id' => $dashboard->id,
                    'widget_type' => $widgetData['widget_type'],
                    'title' => $widgetData['title'],
                    'position_x' => $widgetData['position_x'],
                    'position_y' => $widgetData['position_y'],
                    'width' => $widgetData['width'],
                    'height' => $widgetData['height'],
                    'config' => $widgetData['config'] ?? [],
                    'refresh_interval' => $widgetData['refresh_interval'] ?? 300,
                    'is_visible' => $widgetData['is_visible'] ?? true,
                ]);
            }

            return $dashboard;
        });
    }

    /**
     * Import dashboard from JSON file
     */
    public function importFromFile(string $filename, int $userId, bool $overwrite = false): Dashboard
    {
        $path = "exports/dashboards/{$filename}";
        
        if (!Storage::disk('local')->exists($path)) {
            throw new \Exception('Import file not found');
        }

        $json = Storage::disk('local')->get($path);
        $data = json_decode($json, true);

        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \Exception('Invalid JSON format');
        }

        return $this->importDashboard($data, $userId, $overwrite);
    }

    /**
     * Validate import data structure
     */
    private function validateImportData(array $data): void
    {
        if (!isset($data['version'])) {
            throw new \Exception('Missing version in import data');
        }

        if (!isset($data['dashboard'])) {
            throw new \Exception('Missing dashboard configuration');
        }

        if (!isset($data['widgets']) || !is_array($data['widgets'])) {
            throw new \Exception('Missing or invalid widgets data');
        }

        // Validate required dashboard fields
        $requiredDashboardFields = ['name'];
        foreach ($requiredDashboardFields as $field) {
            if (!isset($data['dashboard'][$field])) {
                throw new \Exception("Missing required dashboard field: {$field}");
            }
        }

        // Validate widget structure
        $requiredWidgetFields = ['widget_type', 'title', 'position_x', 'position_y', 'width', 'height'];
        foreach ($data['widgets'] as $index => $widget) {
            foreach ($requiredWidgetFields as $field) {
                if (!isset($widget[$field])) {
                    throw new \Exception("Missing required field '{$field}' in widget {$index}");
                }
            }
        }
    }

    /**
     * Export multiple dashboards
     */
    public function exportMultipleDashboards(array $dashboardIds, bool $includeData = false): array
    {
        $exports = [];

        foreach ($dashboardIds as $dashboardId) {
            $exports[] = $this->exportDashboard($dashboardId, $includeData);
        }

        return [
            'version' => '1.0.0',
            'exported_at' => now()->toIso8601String(),
            'dashboards' => $exports,
        ];
    }

    /**
     * Clone dashboard for another user
     */
    public function cloneDashboard(string $dashboardId, int $targetUserId, ?string $newName = null): Dashboard
    {
        $export = $this->exportDashboard($dashboardId, false);
        
        if ($newName) {
            $export['dashboard']['name'] = $newName;
        } else {
            $export['dashboard']['name'] .= ' (Copy)';
        }

        return $this->importDashboard($export, $targetUserId, false);
    }

    /**
     * Get export file URL
     */
    public function getExportUrl(string $filename): string
    {
        return Storage::disk('local')->url("exports/dashboards/{$filename}");
    }

    /**
     * List all export files for user
     */
    public function listUserExports(int $userId): array
    {
        $files = Storage::disk('local')->files('exports/dashboards');
        
        return collect($files)->map(function ($file) {
            return [
                'filename' => basename($file),
                'size' => Storage::disk('local')->size($file),
                'created_at' => Carbon::createFromTimestamp(
                    Storage::disk('local')->lastModified($file)
                )->toIso8601String(),
            ];
        })->toArray();
    }

    /**
     * Delete export file
     */
    public function deleteExport(string $filename): bool
    {
        $path = "exports/dashboards/{$filename}";
        
        if (Storage::disk('local')->exists($path)) {
            return Storage::disk('local')->delete($path);
        }

        return false;
    }
}
