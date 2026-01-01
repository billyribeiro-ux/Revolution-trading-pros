<?php

namespace App\Services\Dashboard;

use App\Models\Dashboard;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Mail;
use Illuminate\Support\Facades\Storage;
use Carbon\Carbon;

class ScheduledReportService
{
    public function __construct(
        private DashboardExportService $exportService
    ) {}

    /**
     * Create scheduled report
     */
    public function createScheduledReport(array $data): array
    {
        $reportId = DB::table('scheduled_reports')->insertGetId([
            'dashboard_id' => $data['dashboard_id'],
            'user_id' => $data['user_id'],
            'name' => $data['name'],
            'frequency' => $data['frequency'], // daily, weekly, monthly
            'recipients' => json_encode($data['recipients']),
            'format' => $data['format'] ?? 'json', // json, pdf, csv
            'include_data' => $data['include_data'] ?? true,
            'filters' => json_encode($data['filters'] ?? []),
            'next_run_at' => $this->calculateNextRun($data['frequency']),
            'is_active' => true,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        return $this->getScheduledReport($reportId);
    }

    /**
     * Get scheduled report
     */
    public function getScheduledReport(int $reportId): array
    {
        $report = DB::table('scheduled_reports')->where('id', $reportId)->first();
        
        if (!$report) {
            throw new \Exception('Scheduled report not found');
        }

        return [
            'id' => $report->id,
            'dashboard_id' => $report->dashboard_id,
            'user_id' => $report->user_id,
            'name' => $report->name,
            'frequency' => $report->frequency,
            'recipients' => json_decode($report->recipients, true),
            'format' => $report->format,
            'include_data' => $report->include_data,
            'filters' => json_decode($report->filters, true),
            'next_run_at' => $report->next_run_at,
            'last_run_at' => $report->last_run_at,
            'is_active' => $report->is_active,
            'created_at' => $report->created_at,
        ];
    }

    /**
     * Execute scheduled report
     */
    public function executeReport(int $reportId): void
    {
        $report = $this->getScheduledReport($reportId);

        if (!$report['is_active']) {
            return;
        }

        try {
            // Export dashboard
            $export = $this->exportService->exportDashboard(
                $report['dashboard_id'],
                $report['include_data']
            );

            // Apply filters if any
            if (!empty($report['filters'])) {
                $export = $this->applyFilters($export, $report['filters']);
            }

            // Generate report file
            $filename = $this->generateReportFile($export, $report['format'], $report['name']);

            // Send to recipients
            $this->sendReport($report, $filename);

            // Update last run
            DB::table('scheduled_reports')
                ->where('id', $reportId)
                ->update([
                    'last_run_at' => now(),
                    'next_run_at' => $this->calculateNextRun($report['frequency']),
                    'updated_at' => now(),
                ]);

            // Log execution
            DB::table('report_executions')->insert([
                'scheduled_report_id' => $reportId,
                'status' => 'success',
                'filename' => $filename,
                'executed_at' => now(),
            ]);

        } catch (\Exception $e) {
            // Log failure
            DB::table('report_executions')->insert([
                'scheduled_report_id' => $reportId,
                'status' => 'failed',
                'error_message' => $e->getMessage(),
                'executed_at' => now(),
            ]);

            throw $e;
        }
    }

    /**
     * Send report via email
     */
    private function sendReport(array $report, string $filename): void
    {
        $filePath = Storage::disk('local')->path("reports/{$filename}");

        foreach ($report['recipients'] as $recipient) {
            Mail::send('emails.scheduled-report', [
                'report_name' => $report['name'],
                'frequency' => $report['frequency'],
            ], function ($message) use ($recipient, $report, $filePath) {
                $message->to($recipient)
                    ->subject("Scheduled Report: {$report['name']}")
                    ->attach($filePath);
            });
        }
    }

    /**
     * Generate report file
     */
    private function generateReportFile(array $export, string $format, string $name): string
    {
        $timestamp = now()->format('Y-m-d_His');
        $filename = sprintf('%s_%s.%s', 
            preg_replace('/[^A-Za-z0-9_\-]/', '_', $name),
            $timestamp,
            $format
        );

        $content = match($format) {
            'json' => json_encode($export, JSON_PRETTY_PRINT),
            'csv' => $this->convertToCSV($export),
            'pdf' => $this->convertToPDF($export),
            default => json_encode($export, JSON_PRETTY_PRINT),
        };

        Storage::disk('local')->put("reports/{$filename}", $content);

        return $filename;
    }

    /**
     * Convert export to CSV
     */
    private function convertToCSV(array $export): string
    {
        $csv = [];
        
        // Header
        $csv[] = "Dashboard Report: {$export['dashboard']['name']}";
        $csv[] = "Generated: {$export['exported_at']}";
        $csv[] = "";
        
        // Widgets summary
        $csv[] = "Widget,Type,Position,Size";
        foreach ($export['widgets'] as $widget) {
            $csv[] = sprintf('"%s","%s","%d,%d","%dx%d"',
                $widget['title'],
                $widget['widget_type'],
                $widget['position_x'],
                $widget['position_y'],
                $widget['width'],
                $widget['height']
            );
        }

        return implode("\n", $csv);
    }

    /**
     * Convert export to PDF (placeholder)
     */
    private function convertToPDF(array $export): string
    {
        // This would integrate with a PDF library like TCPDF or DomPDF
        // For now, return JSON
        return json_encode($export, JSON_PRETTY_PRINT);
    }

    /**
     * Apply filters to export
     */
    private function applyFilters(array $export, array $filters): array
    {
        // Filter widgets by type
        if (isset($filters['widget_types'])) {
            $export['widgets'] = array_filter($export['widgets'], function ($widget) use ($filters) {
                return in_array($widget['widget_type'], $filters['widget_types']);
            });
        }

        // Filter by date range
        if (isset($filters['date_from']) || isset($filters['date_to'])) {
            // Apply date filtering to widget data
        }

        return $export;
    }

    /**
     * Calculate next run time
     */
    private function calculateNextRun(string $frequency): Carbon
    {
        return match($frequency) {
            'daily' => now()->addDay()->startOfDay()->addHours(8), // 8 AM next day
            'weekly' => now()->addWeek()->startOfWeek()->addHours(8), // Monday 8 AM
            'monthly' => now()->addMonth()->startOfMonth()->addHours(8), // 1st of month 8 AM
            default => now()->addDay(),
        };
    }

    /**
     * Update scheduled report
     */
    public function updateScheduledReport(int $reportId, array $data): array
    {
        $updates = [];

        if (isset($data['name'])) $updates['name'] = $data['name'];
        if (isset($data['frequency'])) {
            $updates['frequency'] = $data['frequency'];
            $updates['next_run_at'] = $this->calculateNextRun($data['frequency']);
        }
        if (isset($data['recipients'])) $updates['recipients'] = json_encode($data['recipients']);
        if (isset($data['format'])) $updates['format'] = $data['format'];
        if (isset($data['include_data'])) $updates['include_data'] = $data['include_data'];
        if (isset($data['filters'])) $updates['filters'] = json_encode($data['filters']);
        if (isset($data['is_active'])) $updates['is_active'] = $data['is_active'];

        $updates['updated_at'] = now();

        DB::table('scheduled_reports')->where('id', $reportId)->update($updates);

        return $this->getScheduledReport($reportId);
    }

    /**
     * Delete scheduled report
     */
    public function deleteScheduledReport(int $reportId): bool
    {
        return DB::table('scheduled_reports')->where('id', $reportId)->delete() > 0;
    }

    /**
     * Get all scheduled reports for user
     */
    public function getUserScheduledReports(int $userId): array
    {
        $reports = DB::table('scheduled_reports')
            ->where('user_id', $userId)
            ->orderBy('created_at', 'desc')
            ->get();

        return $reports->map(function ($report) {
            return [
                'id' => $report->id,
                'dashboard_id' => $report->dashboard_id,
                'name' => $report->name,
                'frequency' => $report->frequency,
                'recipients' => json_decode($report->recipients, true),
                'format' => $report->format,
                'next_run_at' => $report->next_run_at,
                'last_run_at' => $report->last_run_at,
                'is_active' => $report->is_active,
            ];
        })->toArray();
    }

    /**
     * Get report execution history
     */
    public function getReportExecutions(int $reportId, int $limit = 10): array
    {
        return DB::table('report_executions')
            ->where('scheduled_report_id', $reportId)
            ->orderBy('executed_at', 'desc')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Run all due reports
     */
    public function runDueReports(): array
    {
        $dueReports = DB::table('scheduled_reports')
            ->where('is_active', true)
            ->where('next_run_at', '<=', now())
            ->get();

        $results = [];

        foreach ($dueReports as $report) {
            try {
                $this->executeReport($report->id);
                $results[] = ['id' => $report->id, 'status' => 'success'];
            } catch (\Exception $e) {
                $results[] = ['id' => $report->id, 'status' => 'failed', 'error' => $e->getMessage()];
            }
        }

        return $results;
    }
}
