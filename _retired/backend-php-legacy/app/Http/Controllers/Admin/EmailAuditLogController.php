<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\EmailAuditLog;
use App\Services\Email\AuditService;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\DB;
use Illuminate\Validation\Rule;

/**
 * EmailAuditLogController
 *
 * Enterprise audit log management for email marketing system.
 *
 * Features:
 * - Comprehensive log viewing
 * - Advanced filtering
 * - Statistics and analytics
 * - Export functionality
 * - Retention management
 *
 * @version 1.0.0
 */
class EmailAuditLogController extends Controller
{
    public function __construct(
        private readonly AuditService $auditService
    ) {}

    /**
     * List audit logs with filtering and pagination
     */
    public function index(Request $request): JsonResponse
    {
        $query = EmailAuditLog::query()->latest('created_at');

        // Filter by user
        if ($userId = $request->get('user_id')) {
            $query->where('user_id', $userId);
        }

        // Filter by action
        if ($action = $request->get('action')) {
            $query->where('action', $action);
        }

        // Filter by resource type
        if ($resourceType = $request->get('resource_type')) {
            $query->where('resource_type', $resourceType);
        }

        // Filter by status
        if ($status = $request->get('status')) {
            $query->where('status', $status);
        }

        // Search
        if ($search = $request->get('search')) {
            $query->where(function ($q) use ($search) {
                $q->where('user_email', 'like', "%{$search}%")
                    ->orWhere('resource_name', 'like', "%{$search}%")
                    ->orWhere('ip_address', 'like', "%{$search}%");
            });
        }

        // Date range
        if ($from = $request->get('from_date')) {
            $query->whereDate('created_at', '>=', $from);
        }
        if ($to = $request->get('to_date')) {
            $query->whereDate('created_at', '<=', $to);
        }

        $perPage = min($request->integer('per_page', 50), 100);
        $logs = $query->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $logs->items(),
            'meta' => [
                'current_page' => $logs->currentPage(),
                'last_page' => $logs->lastPage(),
                'per_page' => $logs->perPage(),
                'total' => $logs->total(),
            ],
        ]);
    }

    /**
     * Get audit log statistics
     */
    public function stats(Request $request): JsonResponse
    {
        $period = $request->get('period', 'week');
        $stats = $this->auditService->getStatistics($period);

        // Additional metrics
        $stats['recent_activity'] = EmailAuditLog::latest()
            ->limit(10)
            ->get(['id', 'action', 'resource_type', 'user_email', 'created_at']);

        $stats['top_users'] = EmailAuditLog::whereNotNull('user_id')
            ->where('created_at', '>=', now()->subDays(30))
            ->groupBy('user_id', 'user_email')
            ->selectRaw('user_id, user_email, COUNT(*) as action_count')
            ->orderByDesc('action_count')
            ->limit(10)
            ->get();

        // Use database-agnostic hour extraction
        $driver = \DB::connection()->getDriverName();
        $hourExpr = $driver === 'sqlite'
            ? "CAST(strftime('%H', created_at) AS INTEGER)"
            : "HOUR(created_at)";

        $stats['hourly_distribution'] = EmailAuditLog::where('created_at', '>=', now()->subDays(7))
            ->selectRaw("{$hourExpr} as hour, COUNT(*) as count")
            ->groupBy('hour')
            ->orderBy('hour')
            ->pluck('count', 'hour');

        return response()->json([
            'success' => true,
            'data' => $stats,
        ]);
    }

    /**
     * Get single audit log entry
     */
    public function show(string|int $id): JsonResponse
    {
        $log = EmailAuditLog::findOrFail((int) $id);

        return response()->json([
            'success' => true,
            'data' => $log,
        ]);
    }

    /**
     * Get activity timeline for a specific resource
     */
    public function resourceTimeline(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'resource_type' => 'required|string',
            'resource_id' => 'required|integer',
        ]);

        $timeline = EmailAuditLog::where('resource_type', $validated['resource_type'])
            ->where('resource_id', $validated['resource_id'])
            ->latest('created_at')
            ->limit(100)
            ->get();

        return response()->json([
            'success' => true,
            'data' => $timeline,
        ]);
    }

    /**
     * Get user activity log
     */
    public function userActivity(string|int $userId): JsonResponse
    {
        $logs = EmailAuditLog::where('user_id', $userId)
            ->latest('created_at')
            ->limit(100)
            ->get();

        $summary = [
            'total_actions' => $logs->count(),
            'by_action' => $logs->groupBy('action')->map->count(),
            'by_resource' => $logs->groupBy('resource_type')->map->count(),
            'last_active' => $logs->first()?->created_at,
        ];

        return response()->json([
            'success' => true,
            'data' => [
                'logs' => $logs,
                'summary' => $summary,
            ],
        ]);
    }

    /**
     * Export audit logs
     */
    public function export(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'from_date' => 'nullable|date',
            'to_date' => 'nullable|date',
            'format' => ['nullable', Rule::in(['json', 'csv'])],
        ]);

        $filters = [];
        if (!empty($validated['from_date'])) {
            $filters['from'] = $validated['from_date'];
        }
        if (!empty($validated['to_date'])) {
            $filters['to'] = $validated['to_date'];
        }

        $logs = $this->auditService->export($filters);

        // Log the export action
        $this->auditService->log(
            'export',
            'audit_log',
            null,
            "Exported {$logs->count()} audit logs",
            null,
            null,
            ['count' => $logs->count(), 'filters' => $filters]
        );

        return response()->json([
            'success' => true,
            'data' => $logs,
            'meta' => [
                'total' => $logs->count(),
                'exported_at' => now()->toIso8601String(),
            ],
        ]);
    }

    /**
     * Get available filter options
     */
    public function filterOptions(): JsonResponse
    {
        $actions = EmailAuditLog::distinct()->pluck('action')->filter();
        $resourceTypes = EmailAuditLog::distinct()->pluck('resource_type')->filter();
        $users = EmailAuditLog::whereNotNull('user_id')
            ->distinct()
            ->select('user_id', 'user_email')
            ->get();

        return response()->json([
            'success' => true,
            'data' => [
                'actions' => $actions,
                'resource_types' => $resourceTypes,
                'statuses' => ['success', 'failed', 'warning'],
                'users' => $users,
            ],
        ]);
    }

    /**
     * Clean up old audit logs
     */
    public function cleanup(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'retention_days' => 'nullable|integer|min:30|max:365',
        ]);

        $retentionDays = $validated['retention_days'] ?? 90;
        $deleted = $this->auditService->cleanup($retentionDays);

        return response()->json([
            'success' => true,
            'data' => [
                'deleted' => $deleted,
                'retention_days' => $retentionDays,
            ],
            'message' => "Deleted {$deleted} audit log entries older than {$retentionDays} days",
        ]);
    }

    /**
     * Get security-related events
     */
    public function securityEvents(Request $request): JsonResponse
    {
        $securityActions = [
            'login', 'logout', 'password_change', 'mfa_enable', 'mfa_disable',
            'api_key_create', 'api_key_revoke', 'permission_change', 'export',
        ];

        $query = EmailAuditLog::whereIn('action', $securityActions)
            ->latest('created_at');

        if ($from = $request->get('from_date')) {
            $query->whereDate('created_at', '>=', $from);
        }

        $logs = $query->limit(100)->get();

        // Flag suspicious activity
        $suspiciousPatterns = $this->detectSuspiciousActivity();

        return response()->json([
            'success' => true,
            'data' => [
                'events' => $logs,
                'suspicious_activity' => $suspiciousPatterns,
            ],
        ]);
    }

    /**
     * Detect suspicious activity patterns
     */
    private function detectSuspiciousActivity(): array
    {
        $suspicious = [];

        // High volume of failed actions from single IP
        $failedByIp = EmailAuditLog::where('status', 'failed')
            ->where('created_at', '>=', now()->subHour())
            ->groupBy('ip_address')
            ->selectRaw('ip_address, COUNT(*) as count')
            ->having('count', '>=', 10)
            ->get();

        if ($failedByIp->isNotEmpty()) {
            $suspicious[] = [
                'type' => 'high_failure_rate',
                'description' => 'High volume of failed actions from IP',
                'data' => $failedByIp,
            ];
        }

        // Unusual activity times (database-agnostic)
        $driver = \DB::connection()->getDriverName();
        $hourCondition = $driver === 'sqlite'
            ? "CAST(strftime('%H', created_at) AS INTEGER) BETWEEN 2 AND 5"
            : "HOUR(created_at) BETWEEN 2 AND 5";

        $nightActivity = EmailAuditLog::where('created_at', '>=', now()->subDay())
            ->whereRaw($hourCondition)
            ->count();

        if ($nightActivity > 50) {
            $suspicious[] = [
                'type' => 'unusual_timing',
                'description' => 'Unusual activity during off-hours',
                'data' => ['count' => $nightActivity],
            ];
        }

        // Multiple exports in short time
        $exports = EmailAuditLog::where('action', 'export')
            ->where('created_at', '>=', now()->subHour())
            ->count();

        if ($exports > 5) {
            $suspicious[] = [
                'type' => 'bulk_export',
                'description' => 'Multiple data exports in short period',
                'data' => ['count' => $exports],
            ];
        }

        return $suspicious;
    }
}
