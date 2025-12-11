<?php

namespace App\Services;

use App\Models\Popup;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Request;
use Illuminate\Support\Str;

/**
 * Enterprise Popup Audit Service
 *
 * Provides comprehensive audit logging for popup operations:
 * - Automatic change detection and tracking
 * - Version history with rollback capability
 * - Compliance tagging for regulatory requirements
 * - Sensitive data handling
 * - Export functionality for audit reports
 *
 * @version 1.0.0
 */
class PopupAuditService
{
    /**
     * Audit action constants
     */
    public const ACTION_CREATE = 'create';
    public const ACTION_UPDATE = 'update';
    public const ACTION_DELETE = 'delete';
    public const ACTION_ACTIVATE = 'activate';
    public const ACTION_DEACTIVATE = 'deactivate';
    public const ACTION_PAUSE = 'pause';
    public const ACTION_ARCHIVE = 'archive';
    public const ACTION_SCHEDULE = 'schedule';
    public const ACTION_DUPLICATE = 'duplicate';
    public const ACTION_AB_TEST_START = 'ab_test_start';
    public const ACTION_AB_TEST_END = 'ab_test_end';
    public const ACTION_ANALYTICS_VIEW = 'analytics_view';
    public const ACTION_EXPORT = 'export';
    public const ACTION_RESTORE = 'restore';
    public const ACTION_VERSION_ROLLBACK = 'version_rollback';

    /**
     * Compliance tags
     */
    public const COMPLIANCE_GDPR = 'gdpr';
    public const COMPLIANCE_CCPA = 'ccpa';
    public const COMPLIANCE_SOX = 'sox';
    public const COMPLIANCE_HIPAA = 'hipaa';

    /**
     * Log an audit entry for a popup action.
     *
     * @param Popup|int|null $popup
     * @param string $action
     * @param array $oldValues
     * @param array $newValues
     * @param array $metadata
     * @return int|null Audit log ID
     */
    public function log(
        Popup|int|null $popup,
        string $action,
        array $oldValues = [],
        array $newValues = [],
        array $metadata = []
    ): ?int {
        try {
            $user = auth()->user();
            $popupId = $popup instanceof Popup ? $popup->id : $popup;

            // Calculate changed fields
            $changedFields = $this->calculateChangedFields($oldValues, $newValues);

            // Mask sensitive data
            $maskedOldValues = $this->maskSensitiveData($oldValues);
            $maskedNewValues = $this->maskSensitiveData($newValues);

            $auditData = [
                'popup_id' => $popupId,
                'user_id' => $user?->id,
                'user_email' => $user?->email,
                'user_role' => $user?->role ?? 'system',
                'action' => $action,
                'old_values' => !empty($maskedOldValues) ? json_encode($maskedOldValues) : null,
                'new_values' => !empty($maskedNewValues) ? json_encode($maskedNewValues) : null,
                'changed_fields' => !empty($changedFields) ? json_encode($changedFields) : null,
                'ip_address' => Request::ip(),
                'user_agent' => Request::userAgent(),
                'request_id' => Request::header('X-Request-ID') ?? Str::uuid()->toString(),
                'session_id' => session()->getId(),
                'metadata' => !empty($metadata) ? json_encode($metadata) : null,
                'is_sensitive' => $this->containsSensitiveData($newValues),
                'created_at' => now(),
                'updated_at' => now(),
            ];

            $id = DB::table('popup_audit_logs')->insertGetId($auditData);

            // Also log to application logs for redundancy
            Log::channel('audit')->info('Popup audit log', [
                'audit_id' => $id,
                'popup_id' => $popupId,
                'action' => $action,
                'user' => $user?->email,
            ]);

            return $id;
        } catch (\Exception $e) {
            Log::error('Failed to create popup audit log', [
                'error' => $e->getMessage(),
                'popup_id' => $popup instanceof Popup ? $popup->id : $popup,
                'action' => $action,
            ]);
            return null;
        }
    }

    /**
     * Create a version snapshot of a popup.
     *
     * @param Popup $popup
     * @param string|null $changeSummary
     * @return int|null Version ID
     */
    public function createVersion(Popup $popup, ?string $changeSummary = null): ?int
    {
        try {
            // Get the next version number
            $lastVersion = DB::table('popup_versions')
                ->where('popup_id', $popup->id)
                ->max('version_number') ?? 0;

            $versionData = [
                'popup_id' => $popup->id,
                'version_number' => $lastVersion + 1,
                'config' => json_encode($popup->config),
                'design' => $popup->design ? json_encode($popup->design) : null,
                'display_rules' => $popup->display_rules ? json_encode($popup->display_rules) : null,
                'created_by' => auth()->id(),
                'change_summary' => $changeSummary,
                'status_at_version' => $popup->status,
                'created_at' => now(),
                'updated_at' => now(),
            ];

            return DB::table('popup_versions')->insertGetId($versionData);
        } catch (\Exception $e) {
            Log::error('Failed to create popup version', [
                'error' => $e->getMessage(),
                'popup_id' => $popup->id,
            ]);
            return null;
        }
    }

    /**
     * Rollback a popup to a specific version.
     *
     * @param Popup $popup
     * @param int $versionNumber
     * @return bool
     */
    public function rollbackToVersion(Popup $popup, int $versionNumber): bool
    {
        try {
            $version = DB::table('popup_versions')
                ->where('popup_id', $popup->id)
                ->where('version_number', $versionNumber)
                ->first();

            if (!$version) {
                return false;
            }

            // Store current state before rollback
            $oldValues = $popup->toArray();

            // Apply version data
            $popup->config = json_decode($version->config, true);
            if ($version->design) {
                $popup->design = json_decode($version->design, true);
            }
            if ($version->display_rules) {
                $popup->display_rules = json_decode($version->display_rules, true);
            }
            $popup->save();

            // Create a new version with the rollback
            $this->createVersion($popup, "Rollback to version {$versionNumber}");

            // Log the rollback
            $this->log(
                $popup,
                self::ACTION_VERSION_ROLLBACK,
                $oldValues,
                $popup->toArray(),
                ['rolled_back_to_version' => $versionNumber]
            );

            return true;
        } catch (\Exception $e) {
            Log::error('Failed to rollback popup version', [
                'error' => $e->getMessage(),
                'popup_id' => $popup->id,
                'version' => $versionNumber,
            ]);
            return false;
        }
    }

    /**
     * Get version history for a popup.
     *
     * @param int $popupId
     * @param int $limit
     * @return \Illuminate\Support\Collection
     */
    public function getVersionHistory(int $popupId, int $limit = 50)
    {
        return DB::table('popup_versions')
            ->where('popup_id', $popupId)
            ->orderBy('version_number', 'desc')
            ->limit($limit)
            ->get()
            ->map(function ($version) {
                $version->config = json_decode($version->config);
                $version->design = $version->design ? json_decode($version->design) : null;
                $version->display_rules = $version->display_rules ? json_decode($version->display_rules) : null;
                return $version;
            });
    }

    /**
     * Get audit logs for a popup.
     *
     * @param int $popupId
     * @param array $filters
     * @param int $limit
     * @return \Illuminate\Support\Collection
     */
    public function getAuditLogs(int $popupId, array $filters = [], int $limit = 100)
    {
        $query = DB::table('popup_audit_logs')
            ->where('popup_id', $popupId);

        if (!empty($filters['action'])) {
            $query->where('action', $filters['action']);
        }

        if (!empty($filters['user_id'])) {
            $query->where('user_id', $filters['user_id']);
        }

        if (!empty($filters['date_from'])) {
            $query->where('created_at', '>=', $filters['date_from']);
        }

        if (!empty($filters['date_to'])) {
            $query->where('created_at', '<=', $filters['date_to']);
        }

        return $query
            ->orderBy('created_at', 'desc')
            ->limit($limit)
            ->get()
            ->map(function ($log) {
                $log->old_values = $log->old_values ? json_decode($log->old_values) : null;
                $log->new_values = $log->new_values ? json_decode($log->new_values) : null;
                $log->changed_fields = $log->changed_fields ? json_decode($log->changed_fields) : null;
                $log->metadata = $log->metadata ? json_decode($log->metadata) : null;
                return $log;
            });
    }

    /**
     * Get all audit logs with pagination.
     *
     * @param array $filters
     * @param int $perPage
     * @return \Illuminate\Contracts\Pagination\LengthAwarePaginator
     */
    public function getAllAuditLogs(array $filters = [], int $perPage = 25)
    {
        $query = DB::table('popup_audit_logs')
            ->leftJoin('popups', 'popup_audit_logs.popup_id', '=', 'popups.id')
            ->select([
                'popup_audit_logs.*',
                'popups.name as popup_name',
            ]);

        if (!empty($filters['popup_id'])) {
            $query->where('popup_audit_logs.popup_id', $filters['popup_id']);
        }

        if (!empty($filters['action'])) {
            $query->where('popup_audit_logs.action', $filters['action']);
        }

        if (!empty($filters['user_id'])) {
            $query->where('popup_audit_logs.user_id', $filters['user_id']);
        }

        if (!empty($filters['compliance_tag'])) {
            $query->where('popup_audit_logs.compliance_tag', $filters['compliance_tag']);
        }

        return $query
            ->orderBy('popup_audit_logs.created_at', 'desc')
            ->paginate($perPage);
    }

    /**
     * Export audit logs to array format for compliance reports.
     *
     * @param array $filters
     * @return array
     */
    public function exportAuditLogs(array $filters = []): array
    {
        $logs = $this->getAllAuditLogs($filters, 10000)->items();

        return array_map(function ($log) {
            return [
                'id' => $log->id,
                'popup_id' => $log->popup_id,
                'popup_name' => $log->popup_name ?? 'Deleted',
                'action' => $log->action,
                'user_email' => $log->user_email,
                'user_role' => $log->user_role,
                'changed_fields' => $log->changed_fields,
                'ip_address' => $log->ip_address,
                'created_at' => $log->created_at,
            ];
        }, $logs);
    }

    /**
     * Add compliance tag to audit logs.
     *
     * @param int $popupId
     * @param string $complianceTag
     * @return bool
     */
    public function addComplianceTag(int $popupId, string $complianceTag): bool
    {
        return DB::table('popup_audit_logs')
            ->where('popup_id', $popupId)
            ->whereNull('compliance_tag')
            ->update(['compliance_tag' => $complianceTag]) > 0;
    }

    /**
     * Purge expired audit logs (for data retention compliance).
     *
     * @param int $retentionDays
     * @return int Number of deleted records
     */
    public function purgeExpiredLogs(int $retentionDays = 365): int
    {
        $cutoffDate = now()->subDays($retentionDays);

        return DB::table('popup_audit_logs')
            ->where('created_at', '<', $cutoffDate)
            ->where('is_sensitive', false) // Keep sensitive logs longer
            ->delete();
    }

    /**
     * Calculate which fields changed between old and new values.
     *
     * @param array $oldValues
     * @param array $newValues
     * @return array
     */
    protected function calculateChangedFields(array $oldValues, array $newValues): array
    {
        $changed = [];

        foreach ($newValues as $key => $newValue) {
            $oldValue = $oldValues[$key] ?? null;

            if (json_encode($oldValue) !== json_encode($newValue)) {
                $changed[] = $key;
            }
        }

        return $changed;
    }

    /**
     * Mask sensitive data in values.
     *
     * @param array $values
     * @return array
     */
    protected function maskSensitiveData(array $values): array
    {
        $sensitiveKeys = ['password', 'secret', 'token', 'api_key', 'private_key'];

        foreach ($values as $key => $value) {
            foreach ($sensitiveKeys as $sensitiveKey) {
                if (stripos($key, $sensitiveKey) !== false) {
                    $values[$key] = '***MASKED***';
                }
            }

            if (is_array($value)) {
                $values[$key] = $this->maskSensitiveData($value);
            }
        }

        return $values;
    }

    /**
     * Check if values contain sensitive data.
     *
     * @param array $values
     * @return bool
     */
    protected function containsSensitiveData(array $values): bool
    {
        $sensitiveKeys = ['password', 'secret', 'token', 'api_key', 'private_key', 'ssn', 'credit_card'];

        foreach ($values as $key => $value) {
            foreach ($sensitiveKeys as $sensitiveKey) {
                if (stripos($key, $sensitiveKey) !== false) {
                    return true;
                }
            }

            if (is_array($value) && $this->containsSensitiveData($value)) {
                return true;
            }
        }

        return false;
    }
}
