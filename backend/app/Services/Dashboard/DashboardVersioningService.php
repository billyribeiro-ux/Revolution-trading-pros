<?php

namespace App\Services\Dashboard;

use App\Models\Dashboard;
use App\Models\DashboardWidget;
use Illuminate\Support\Facades\DB;

class DashboardVersioningService
{
    /**
     * Create version snapshot of dashboard
     */
    public function createVersion(string $dashboardId, ?string $description = null): int
    {
        $dashboard = Dashboard::with('widgets')->findOrFail($dashboardId);

        $versionNumber = $this->getNextVersionNumber($dashboardId);

        $versionId = DB::table('dashboard_versions')->insertGetId([
            'dashboard_id' => $dashboardId,
            'version_number' => $versionNumber,
            'description' => $description ?? "Version {$versionNumber}",
            'dashboard_snapshot' => json_encode([
                'name' => $dashboard->name,
                'description' => $dashboard->description,
                'dashboard_type' => $dashboard->dashboard_type,
                'grid_columns' => $dashboard->grid_columns,
                'grid_row_height' => $dashboard->grid_row_height,
                'grid_gap' => $dashboard->grid_gap,
            ]),
            'widgets_snapshot' => json_encode($dashboard->widgets->map(function ($widget) {
                return [
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
            })->toArray()),
            'created_by' => $dashboard->user_id,
            'created_at' => now(),
        ]);

        return $versionId;
    }

    /**
     * Restore dashboard to specific version
     */
    public function restoreVersion(string $dashboardId, int $versionId): Dashboard
    {
        $version = DB::table('dashboard_versions')
            ->where('id', $versionId)
            ->where('dashboard_id', $dashboardId)
            ->first();

        if (!$version) {
            throw new \Exception('Version not found');
        }

        return DB::transaction(function () use ($dashboardId, $version) {
            // Create backup of current state before restoring
            $this->createVersion($dashboardId, 'Auto-backup before restore');

            $dashboard = Dashboard::findOrFail($dashboardId);
            $dashboardSnapshot = json_decode($version->dashboard_snapshot, true);
            $widgetsSnapshot = json_decode($version->widgets_snapshot, true);

            // Restore dashboard settings
            $dashboard->update([
                'name' => $dashboardSnapshot['name'],
                'description' => $dashboardSnapshot['description'],
                'dashboard_type' => $dashboardSnapshot['dashboard_type'],
                'grid_columns' => $dashboardSnapshot['grid_columns'],
                'grid_row_height' => $dashboardSnapshot['grid_row_height'],
                'grid_gap' => $dashboardSnapshot['grid_gap'],
            ]);

            // Delete current widgets
            $dashboard->widgets()->delete();

            // Restore widgets
            foreach ($widgetsSnapshot as $widgetData) {
                DashboardWidget::create(array_merge($widgetData, [
                    'dashboard_id' => $dashboardId,
                ]));
            }

            return $dashboard->fresh();
        });
    }

    /**
     * Get all versions for dashboard
     */
    public function getVersions(string $dashboardId): array
    {
        $versions = DB::table('dashboard_versions')
            ->where('dashboard_id', $dashboardId)
            ->orderBy('version_number', 'desc')
            ->get();

        return $versions->map(function ($version) {
            return [
                'id' => $version->id,
                'version_number' => $version->version_number,
                'description' => $version->description,
                'created_by' => $version->created_by,
                'created_at' => $version->created_at,
                'widget_count' => count(json_decode($version->widgets_snapshot, true)),
            ];
        })->toArray();
    }

    /**
     * Get specific version details
     */
    public function getVersion(int $versionId): array
    {
        $version = DB::table('dashboard_versions')->where('id', $versionId)->first();

        if (!$version) {
            throw new \Exception('Version not found');
        }

        return [
            'id' => $version->id,
            'dashboard_id' => $version->dashboard_id,
            'version_number' => $version->version_number,
            'description' => $version->description,
            'dashboard_snapshot' => json_decode($version->dashboard_snapshot, true),
            'widgets_snapshot' => json_decode($version->widgets_snapshot, true),
            'created_by' => $version->created_by,
            'created_at' => $version->created_at,
        ];
    }

    /**
     * Compare two versions
     */
    public function compareVersions(int $versionId1, int $versionId2): array
    {
        $version1 = $this->getVersion($versionId1);
        $version2 = $this->getVersion($versionId2);

        $changes = [
            'dashboard_changes' => $this->compareDashboardSettings(
                $version1['dashboard_snapshot'],
                $version2['dashboard_snapshot']
            ),
            'widget_changes' => $this->compareWidgets(
                $version1['widgets_snapshot'],
                $version2['widgets_snapshot']
            ),
        ];

        return $changes;
    }

    /**
     * Compare dashboard settings
     */
    private function compareDashboardSettings(array $settings1, array $settings2): array
    {
        $changes = [];

        foreach ($settings1 as $key => $value) {
            if (!isset($settings2[$key]) || $settings2[$key] !== $value) {
                $changes[$key] = [
                    'old' => $value,
                    'new' => $settings2[$key] ?? null,
                ];
            }
        }

        return $changes;
    }

    /**
     * Compare widgets
     */
    private function compareWidgets(array $widgets1, array $widgets2): array
    {
        return [
            'added' => count($widgets2) - count($widgets1),
            'removed' => count($widgets1) - count($widgets2),
            'modified' => $this->countModifiedWidgets($widgets1, $widgets2),
        ];
    }

    /**
     * Count modified widgets
     */
    private function countModifiedWidgets(array $widgets1, array $widgets2): int
    {
        $modified = 0;

        foreach ($widgets1 as $index => $widget1) {
            if (isset($widgets2[$index])) {
                if (json_encode($widget1) !== json_encode($widgets2[$index])) {
                    $modified++;
                }
            }
        }

        return $modified;
    }

    /**
     * Delete version
     */
    public function deleteVersion(int $versionId): bool
    {
        return DB::table('dashboard_versions')->where('id', $versionId)->delete() > 0;
    }

    /**
     * Get next version number
     */
    private function getNextVersionNumber(string $dashboardId): int
    {
        $lastVersion = DB::table('dashboard_versions')
            ->where('dashboard_id', $dashboardId)
            ->max('version_number');

        return ($lastVersion ?? 0) + 1;
    }

    /**
     * Auto-create version on significant changes
     */
    public function autoVersion(string $dashboardId, string $changeType): void
    {
        $description = match($changeType) {
            'widget_added' => 'Auto-save: Widget added',
            'widget_removed' => 'Auto-save: Widget removed',
            'widget_moved' => 'Auto-save: Widget repositioned',
            'settings_changed' => 'Auto-save: Settings updated',
            default => 'Auto-save',
        };

        $this->createVersion($dashboardId, $description);
    }

    /**
     * Cleanup old versions (keep last N versions)
     */
    public function cleanupOldVersions(string $dashboardId, int $keepLast = 10): int
    {
        $versions = DB::table('dashboard_versions')
            ->where('dashboard_id', $dashboardId)
            ->orderBy('version_number', 'desc')
            ->get();

        if ($versions->count() <= $keepLast) {
            return 0;
        }

        $versionsToDelete = $versions->slice($keepLast)->pluck('id');

        return DB::table('dashboard_versions')
            ->whereIn('id', $versionsToDelete)
            ->delete();
    }

    /**
     * Get version diff summary
     */
    public function getVersionDiff(int $versionId): array
    {
        $version = $this->getVersion($versionId);
        
        // Get previous version
        $previousVersion = DB::table('dashboard_versions')
            ->where('dashboard_id', $version['dashboard_id'])
            ->where('version_number', '<', $version['version_number'])
            ->orderBy('version_number', 'desc')
            ->first();

        if (!$previousVersion) {
            return ['message' => 'First version, no diff available'];
        }

        return $this->compareVersions($previousVersion->id, $versionId);
    }
}
