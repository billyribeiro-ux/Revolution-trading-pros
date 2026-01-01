<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailAuditLog;
use App\Models\User;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Str;
use Jenssegers\Agent\Agent;
use Carbon\Carbon;

/**
 * AuditService - Enterprise Email Marketing Audit Logging
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive audit logging service for all email marketing operations.
 *
 * Features:
 * - Automatic change detection
 * - User agent parsing
 * - Geographic tracking
 * - Performance metrics
 * - Summary aggregation
 * - Retention management
 *
 * @version 1.0.0
 */
class AuditService
{
    private ?float $startTime = null;
    private ?string $requestId = null;

    /**
     * Start timing an action
     */
    public function startTiming(): void
    {
        $this->startTime = microtime(true);
        $this->requestId = (string) Str::uuid();
    }

    /**
     * Log an action
     */
    public function log(
        string $action,
        string $resourceType,
        ?int $resourceId = null,
        ?string $resourceName = null,
        ?array $oldValues = null,
        ?array $newValues = null,
        array $metadata = []
    ): EmailAuditLog {
        try {
            $user = Auth::user();
            $request = request();
            $agent = new Agent();
            $agent->setUserAgent($request->userAgent());

            // Calculate duration
            $duration = null;
            if ($this->startTime !== null) {
                $duration = (int) ((microtime(true) - $this->startTime) * 1000);
            }

            // Detect changed fields
            $changedFields = null;
            if ($oldValues !== null && $newValues !== null) {
                $changedFields = $this->detectChanges($oldValues, $newValues);
            }

            // Create audit log entry
            $log = EmailAuditLog::create([
                'uuid' => (string) Str::uuid(),
                'user_id' => $user?->id,
                'user_email' => $user?->email,
                'user_name' => $user?->name,
                'user_role' => $user?->role ?? $user?->roles->first()?->name,
                'action' => $action,
                'resource_type' => $resourceType,
                'resource_id' => $resourceId,
                'resource_name' => $resourceName,
                'old_values' => $oldValues,
                'new_values' => $newValues,
                'changed_fields' => $changedFields,
                'ip_address' => $this->getClientIp($request),
                'user_agent' => $request->userAgent(),
                'request_method' => $request->method(),
                'request_url' => $request->fullUrl(),
                'request_id' => $this->requestId,
                'session_id' => session()->getId(),
                'device_type' => $this->getDeviceType($agent),
                'browser' => $agent->browser() ?: null,
                'os' => $agent->platform() ?: null,
                'country' => $metadata['country'] ?? null,
                'city' => $metadata['city'] ?? null,
                'timezone' => $metadata['timezone'] ?? null,
                'status' => EmailAuditLog::STATUS_SUCCESS,
                'metadata' => $metadata,
                'tags' => $metadata['tags'] ?? null,
                'duration_ms' => $duration,
            ]);

            // Update summary (async in production)
            $this->updateSummary($log);

            // Reset timing
            $this->startTime = null;

            return $log;
        } catch (\Throwable $e) {
            Log::error('[AuditService] Failed to create audit log', [
                'error' => $e->getMessage(),
                'action' => $action,
                'resource_type' => $resourceType,
            ]);

            throw $e;
        }
    }

    /**
     * Log a failed action
     */
    public function logFailure(
        string $action,
        string $resourceType,
        ?int $resourceId = null,
        ?string $resourceName = null,
        string $errorMessage = '',
        array $metadata = []
    ): EmailAuditLog {
        $user = Auth::user();
        $request = request();
        $agent = new Agent();
        $agent->setUserAgent($request->userAgent());

        $duration = null;
        if ($this->startTime !== null) {
            $duration = (int) ((microtime(true) - $this->startTime) * 1000);
        }

        $log = EmailAuditLog::create([
            'uuid' => (string) Str::uuid(),
            'user_id' => $user?->id,
            'user_email' => $user?->email,
            'user_name' => $user?->name,
            'user_role' => $user?->role ?? $user?->roles->first()?->name,
            'action' => $action,
            'resource_type' => $resourceType,
            'resource_id' => $resourceId,
            'resource_name' => $resourceName,
            'ip_address' => $this->getClientIp($request),
            'user_agent' => $request->userAgent(),
            'request_method' => $request->method(),
            'request_url' => $request->fullUrl(),
            'request_id' => $this->requestId,
            'session_id' => session()->getId(),
            'device_type' => $this->getDeviceType($agent),
            'browser' => $agent->browser() ?: null,
            'os' => $agent->platform() ?: null,
            'status' => EmailAuditLog::STATUS_FAILED,
            'error_message' => $errorMessage,
            'metadata' => $metadata,
            'duration_ms' => $duration,
        ]);

        $this->updateSummary($log);
        $this->startTime = null;

        return $log;
    }

    /**
     * Log model changes automatically
     */
    public function logModelChange(string $action, Model $model, ?array $originalAttributes = null): EmailAuditLog
    {
        $resourceType = $this->getResourceTypeFromModel($model);
        $resourceId = $model->getKey();
        $resourceName = $model->name ?? $model->title ?? $model->email ?? "#{$resourceId}";

        $oldValues = $originalAttributes;
        $newValues = $action !== EmailAuditLog::ACTION_DELETE ? $model->getAttributes() : null;

        // Remove sensitive fields
        $sensitiveFields = ['password', 'remember_token', 'api_token', 'secret'];
        if ($oldValues) {
            $oldValues = array_diff_key($oldValues, array_flip($sensitiveFields));
        }
        if ($newValues) {
            $newValues = array_diff_key($newValues, array_flip($sensitiveFields));
        }

        return $this->log($action, $resourceType, $resourceId, $resourceName, $oldValues, $newValues);
    }

    /**
     * Detect changes between old and new values
     */
    private function detectChanges(array $old, array $new): array
    {
        $changed = [];

        foreach ($new as $key => $value) {
            if (!array_key_exists($key, $old) || $old[$key] !== $value) {
                $changed[] = $key;
            }
        }

        // Check for removed keys
        foreach ($old as $key => $value) {
            if (!array_key_exists($key, $new) && !in_array($key, $changed)) {
                $changed[] = $key;
            }
        }

        return $changed;
    }

    /**
     * Get resource type from model class
     */
    private function getResourceTypeFromModel(Model $model): string
    {
        $className = class_basename($model);

        return match ($className) {
            'EmailCampaign' => EmailAuditLog::RESOURCE_CAMPAIGN,
            'EmailTemplate' => EmailAuditLog::RESOURCE_TEMPLATE,
            'NewsletterSubscription' => EmailAuditLog::RESOURCE_SUBSCRIBER,
            'AnalyticsSegment', 'EmailSegment' => EmailAuditLog::RESOURCE_SEGMENT,
            'EmailAutomation', 'EmailAutomationWorkflow' => EmailAuditLog::RESOURCE_AUTOMATION,
            'EmailSequence' => EmailAuditLog::RESOURCE_SEQUENCE,
            'EmailSetting' => EmailAuditLog::RESOURCE_SETTINGS,
            'Webhook' => EmailAuditLog::RESOURCE_WEBHOOK,
            default => strtolower($className),
        };
    }

    /**
     * Get client IP address
     */
    private function getClientIp($request): string
    {
        $forwardedFor = $request->header('X-Forwarded-For');
        if ($forwardedFor) {
            $ips = explode(',', $forwardedFor);
            return trim($ips[0]);
        }

        return $request->ip() ?? '0.0.0.0';
    }

    /**
     * Get device type from agent
     */
    private function getDeviceType(Agent $agent): string
    {
        if ($agent->isTablet()) {
            return 'tablet';
        }
        if ($agent->isMobile()) {
            return 'mobile';
        }
        if ($agent->isDesktop()) {
            return 'desktop';
        }
        if ($agent->isRobot()) {
            return 'bot';
        }

        return 'unknown';
    }

    /**
     * Update summary table
     */
    private function updateSummary(EmailAuditLog $log): void
    {
        try {
            DB::table('email_audit_summaries')
                ->updateOrInsert(
                    [
                        'date' => $log->created_at->toDateString(),
                        'action' => $log->action,
                        'resource_type' => $log->resource_type,
                        'user_id' => $log->user_id,
                    ],
                    [
                        'count' => DB::raw('count + 1'),
                        'success_count' => DB::raw($log->status === 'success' ? 'success_count + 1' : 'success_count'),
                        'failed_count' => DB::raw($log->status === 'failed' ? 'failed_count + 1' : 'failed_count'),
                        'updated_at' => now(),
                    ]
                );
        } catch (\Throwable $e) {
            // Silently fail - summary is non-critical
            Log::debug('[AuditService] Failed to update summary', ['error' => $e->getMessage()]);
        }
    }

    /**
     * Get audit logs with filters
     */
    public function getLogs(array $filters = [], int $perPage = 50): \Illuminate\Contracts\Pagination\LengthAwarePaginator
    {
        $query = EmailAuditLog::query()->latest('created_at');

        if (!empty($filters['user_id'])) {
            $query->byUser($filters['user_id']);
        }

        if (!empty($filters['action'])) {
            $query->byAction($filters['action']);
        }

        if (!empty($filters['resource_type'])) {
            $query->byResourceType($filters['resource_type']);
        }

        if (!empty($filters['status'])) {
            $query->byStatus($filters['status']);
        }

        if (!empty($filters['search'])) {
            $query->search($filters['search']);
        }

        if (!empty($filters['from']) && !empty($filters['to'])) {
            $query->dateRange($filters['from'], $filters['to']);
        }

        return $query->paginate($perPage);
    }

    /**
     * Get audit statistics
     */
    public function getStatistics(string $period = 'week'): array
    {
        $cacheKey = "audit_stats:{$period}";

        return Cache::remember($cacheKey, 300, function () use ($period) {
            $from = match ($period) {
                'day' => now()->startOfDay(),
                'week' => now()->startOfWeek(),
                'month' => now()->startOfMonth(),
                default => now()->startOfWeek(),
            };

            $logs = EmailAuditLog::where('created_at', '>=', $from);

            return [
                'total_actions' => $logs->count(),
                'successful' => (clone $logs)->byStatus('success')->count(),
                'failed' => (clone $logs)->byStatus('failed')->count(),
                'by_action' => (clone $logs)->groupBy('action')
                    ->selectRaw('action, COUNT(*) as count')
                    ->pluck('count', 'action'),
                'by_resource' => (clone $logs)->groupBy('resource_type')
                    ->selectRaw('resource_type, COUNT(*) as count')
                    ->pluck('count', 'resource_type'),
                'by_user' => (clone $logs)->whereNotNull('user_id')
                    ->groupBy('user_id')
                    ->selectRaw('user_id, COUNT(*) as count')
                    ->orderByDesc('count')
                    ->limit(10)
                    ->pluck('count', 'user_id'),
                'avg_duration_ms' => (clone $logs)->whereNotNull('duration_ms')
                    ->avg('duration_ms'),
            ];
        });
    }

    /**
     * Clean up old audit logs
     */
    public function cleanup(int $retentionDays = 90): int
    {
        $cutoff = now()->subDays($retentionDays);

        $deleted = EmailAuditLog::where('created_at', '<', $cutoff)->delete();

        Log::info('[AuditService] Cleaned up old audit logs', [
            'deleted' => $deleted,
            'cutoff_date' => $cutoff->toDateString(),
        ]);

        return $deleted;
    }

    /**
     * Export audit logs
     */
    public function export(array $filters = []): \Illuminate\Support\Collection
    {
        $query = EmailAuditLog::query()->latest('created_at');

        if (!empty($filters['from']) && !empty($filters['to'])) {
            $query->dateRange($filters['from'], $filters['to']);
        }

        return $query->limit(10000)->get()->map(function ($log) {
            return [
                'id' => $log->id,
                'uuid' => $log->uuid,
                'timestamp' => $log->created_at->toIso8601String(),
                'user' => $log->user_email,
                'action' => $log->action,
                'resource_type' => $log->resource_type,
                'resource_name' => $log->resource_name,
                'status' => $log->status,
                'ip_address' => $log->ip_address,
                'description' => $log->description,
            ];
        });
    }
}
