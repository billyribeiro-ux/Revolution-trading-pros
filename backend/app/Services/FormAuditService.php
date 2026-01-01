<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormSubmission;
use App\Models\User;
use Illuminate\Support\Facades\DB;

/**
 * Form Audit Service - Comprehensive audit logging for forms
 *
 * Tracks:
 * - Form CRUD operations
 * - Field modifications
 * - Submission access
 * - Data exports
 * - Setting changes
 * - User actions
 * - API access
 *
 * @version 1.0.0
 * @security Critical - GDPR compliance and security auditing
 */
class FormAuditService
{
    /**
     * Audit event types
     */
    public const EVENT_FORM_CREATED = 'form.created';
    public const EVENT_FORM_UPDATED = 'form.updated';
    public const EVENT_FORM_DELETED = 'form.deleted';
    public const EVENT_FORM_PUBLISHED = 'form.published';
    public const EVENT_FORM_UNPUBLISHED = 'form.unpublished';
    public const EVENT_FORM_DUPLICATED = 'form.duplicated';
    public const EVENT_FORM_EXPORTED = 'form.exported';
    public const EVENT_FORM_IMPORTED = 'form.imported';

    public const EVENT_FIELD_ADDED = 'field.added';
    public const EVENT_FIELD_UPDATED = 'field.updated';
    public const EVENT_FIELD_DELETED = 'field.deleted';
    public const EVENT_FIELD_REORDERED = 'field.reordered';

    public const EVENT_SUBMISSION_CREATED = 'submission.created';
    public const EVENT_SUBMISSION_VIEWED = 'submission.viewed';
    public const EVENT_SUBMISSION_UPDATED = 'submission.updated';
    public const EVENT_SUBMISSION_DELETED = 'submission.deleted';
    public const EVENT_SUBMISSION_EXPORTED = 'submission.exported';
    public const EVENT_SUBMISSION_BULK_ACTION = 'submission.bulk_action';

    public const EVENT_DATA_DECRYPTED = 'data.decrypted';
    public const EVENT_DATA_EXPORTED = 'data.exported';
    public const EVENT_DATA_ACCESSED = 'data.accessed';

    public const EVENT_SETTINGS_CHANGED = 'settings.changed';
    public const EVENT_INTEGRATION_ADDED = 'integration.added';
    public const EVENT_INTEGRATION_REMOVED = 'integration.removed';

    public const EVENT_USER_LOGIN = 'user.login';
    public const EVENT_USER_LOGOUT = 'user.logout';
    public const EVENT_API_ACCESS = 'api.access';

    /**
     * Severity levels
     */
    public const SEVERITY_INFO = 'info';
    public const SEVERITY_WARNING = 'warning';
    public const SEVERITY_ERROR = 'error';
    public const SEVERITY_CRITICAL = 'critical';

    /**
     * Log an audit event
     *
     * @param string $event Event type
     * @param array $data Event data
     * @param string $severity Severity level
     * @return int Audit log ID
     */
    public function log(string $event, array $data = [], string $severity = self::SEVERITY_INFO): int
    {
        $user = auth()->user();
        $request = request();

        $auditData = [
            'event' => $event,
            'severity' => $severity,
            'user_id' => $user?->id,
            'user_email' => $user?->email,
            'user_role' => $user?->getRoleNames()->first(),
            'form_id' => $data['form_id'] ?? null,
            'submission_id' => $data['submission_id'] ?? null,
            'ip_address' => $request?->ip(),
            'user_agent' => $request?->userAgent(),
            'request_method' => $request?->method(),
            'request_url' => $request?->fullUrl(),
            'request_id' => $request?->header('X-Request-ID'),
            'session_id' => session()->getId(),
            'data' => json_encode($this->sanitizeData($data)),
            'changes' => isset($data['changes']) ? json_encode($data['changes']) : null,
            'created_at' => now(),
        ];

        return DB::table('form_audit_logs')->insertGetId($auditData);
    }

    /**
     * Log form creation
     */
    public function logFormCreated(Form $form): int
    {
        return $this->log(self::EVENT_FORM_CREATED, [
            'form_id' => $form->id,
            'form_title' => $form->title,
            'form_slug' => $form->slug,
            'field_count' => $form->fields->count(),
        ]);
    }

    /**
     * Log form update with change tracking
     */
    public function logFormUpdated(Form $form, array $originalData): int
    {
        $changes = $this->detectChanges($originalData, $form->toArray());

        return $this->log(self::EVENT_FORM_UPDATED, [
            'form_id' => $form->id,
            'form_title' => $form->title,
            'changes' => $changes,
        ]);
    }

    /**
     * Log form deletion
     */
    public function logFormDeleted(Form $form, bool $permanent = false): int
    {
        return $this->log(
            self::EVENT_FORM_DELETED,
            [
                'form_id' => $form->id,
                'form_title' => $form->title,
                'permanent' => $permanent,
                'had_submissions' => $form->submission_count > 0,
                'submission_count' => $form->submission_count,
            ],
            $permanent ? self::SEVERITY_WARNING : self::SEVERITY_INFO
        );
    }

    /**
     * Log submission view
     */
    public function logSubmissionViewed(FormSubmission $submission): int
    {
        return $this->log(self::EVENT_SUBMISSION_VIEWED, [
            'form_id' => $submission->form_id,
            'submission_id' => $submission->id,
            'submission_uuid' => $submission->submission_id,
        ]);
    }

    /**
     * Log data export
     */
    public function logDataExport(Form $form, string $format, int $recordCount): int
    {
        return $this->log(
            self::EVENT_DATA_EXPORTED,
            [
                'form_id' => $form->id,
                'form_title' => $form->title,
                'export_format' => $format,
                'record_count' => $recordCount,
            ],
            self::SEVERITY_WARNING
        );
    }

    /**
     * Log sensitive data decryption
     */
    public function logDataDecrypted(int $submissionId, string $fieldName, string $reason = ''): int
    {
        return $this->log(
            self::EVENT_DATA_DECRYPTED,
            [
                'submission_id' => $submissionId,
                'field_name' => $fieldName,
                'reason' => $reason,
            ],
            self::SEVERITY_WARNING
        );
    }

    /**
     * Log bulk action
     */
    public function logBulkAction(Form $form, string $action, array $submissionIds): int
    {
        return $this->log(self::EVENT_SUBMISSION_BULK_ACTION, [
            'form_id' => $form->id,
            'action' => $action,
            'submission_count' => count($submissionIds),
            'submission_ids' => $submissionIds,
        ]);
    }

    /**
     * Log API access
     */
    public function logApiAccess(string $endpoint, string $method, int $statusCode): int
    {
        return $this->log(self::EVENT_API_ACCESS, [
            'endpoint' => $endpoint,
            'method' => $method,
            'status_code' => $statusCode,
        ]);
    }

    /**
     * Get audit logs for a form
     *
     * @param int $formId Form ID
     * @param array $filters Filters [event, severity, user_id, date_from, date_to]
     * @param int $limit Limit
     * @return array
     */
    public function getFormLogs(int $formId, array $filters = [], int $limit = 100): array
    {
        $query = DB::table('form_audit_logs')
            ->where('form_id', $formId)
            ->orderBy('created_at', 'desc');

        if (!empty($filters['event'])) {
            $query->where('event', $filters['event']);
        }

        if (!empty($filters['severity'])) {
            $query->where('severity', $filters['severity']);
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

        return $query->limit($limit)->get()->toArray();
    }

    /**
     * Get user activity logs
     */
    public function getUserLogs(int $userId, int $limit = 100): array
    {
        return DB::table('form_audit_logs')
            ->where('user_id', $userId)
            ->orderBy('created_at', 'desc')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get security alerts (critical/warning events)
     */
    public function getSecurityAlerts(int $hours = 24, int $limit = 100): array
    {
        return DB::table('form_audit_logs')
            ->whereIn('severity', [self::SEVERITY_WARNING, self::SEVERITY_CRITICAL])
            ->where('created_at', '>=', now()->subHours($hours))
            ->orderBy('created_at', 'desc')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get audit statistics
     */
    public function getStats(int $days = 30): array
    {
        $since = now()->subDays($days);

        return [
            'total_events' => DB::table('form_audit_logs')
                ->where('created_at', '>=', $since)
                ->count(),

            'events_by_type' => DB::table('form_audit_logs')
                ->select('event', DB::raw('COUNT(*) as count'))
                ->where('created_at', '>=', $since)
                ->groupBy('event')
                ->pluck('count', 'event')
                ->toArray(),

            'events_by_severity' => DB::table('form_audit_logs')
                ->select('severity', DB::raw('COUNT(*) as count'))
                ->where('created_at', '>=', $since)
                ->groupBy('severity')
                ->pluck('count', 'severity')
                ->toArray(),

            'top_users' => DB::table('form_audit_logs')
                ->select('user_email', DB::raw('COUNT(*) as count'))
                ->where('created_at', '>=', $since)
                ->whereNotNull('user_email')
                ->groupBy('user_email')
                ->orderBy('count', 'desc')
                ->limit(10)
                ->get()
                ->toArray(),

            'daily_activity' => DB::table('form_audit_logs')
                ->select(DB::raw('DATE(created_at) as date'), DB::raw('COUNT(*) as count'))
                ->where('created_at', '>=', $since)
                ->groupBy('date')
                ->orderBy('date')
                ->get()
                ->toArray(),
        ];
    }

    /**
     * Generate compliance report (GDPR)
     */
    public function generateComplianceReport(int $formId, string $dateFrom, string $dateTo): array
    {
        $logs = $this->getFormLogs($formId, [
            'date_from' => $dateFrom,
            'date_to' => $dateTo,
        ], 10000);

        $report = [
            'form_id' => $formId,
            'period' => [
                'from' => $dateFrom,
                'to' => $dateTo,
            ],
            'generated_at' => now()->toIso8601String(),
            'generated_by' => auth()->user()?->email,

            'summary' => [
                'total_events' => count($logs),
                'data_access_events' => 0,
                'data_export_events' => 0,
                'data_deletion_events' => 0,
                'decryption_events' => 0,
                'unique_users' => [],
            ],

            'data_access_log' => [],
            'data_export_log' => [],
            'data_deletion_log' => [],
        ];

        foreach ($logs as $log) {
            $report['summary']['unique_users'][$log->user_email ?? 'anonymous'] = true;

            switch ($log->event) {
                case self::EVENT_SUBMISSION_VIEWED:
                case self::EVENT_DATA_ACCESSED:
                    $report['summary']['data_access_events']++;
                    $report['data_access_log'][] = [
                        'timestamp' => $log->created_at,
                        'user' => $log->user_email,
                        'ip' => $log->ip_address,
                        'submission_id' => json_decode($log->data)->submission_id ?? null,
                    ];
                    break;

                case self::EVENT_DATA_EXPORTED:
                case self::EVENT_SUBMISSION_EXPORTED:
                    $report['summary']['data_export_events']++;
                    $report['data_export_log'][] = [
                        'timestamp' => $log->created_at,
                        'user' => $log->user_email,
                        'ip' => $log->ip_address,
                        'format' => json_decode($log->data)->export_format ?? null,
                        'records' => json_decode($log->data)->record_count ?? null,
                    ];
                    break;

                case self::EVENT_SUBMISSION_DELETED:
                    $report['summary']['data_deletion_events']++;
                    $report['data_deletion_log'][] = [
                        'timestamp' => $log->created_at,
                        'user' => $log->user_email,
                        'ip' => $log->ip_address,
                        'submission_id' => json_decode($log->data)->submission_id ?? null,
                    ];
                    break;

                case self::EVENT_DATA_DECRYPTED:
                    $report['summary']['decryption_events']++;
                    break;
            }
        }

        $report['summary']['unique_users'] = count($report['summary']['unique_users']);

        return $report;
    }

    /**
     * Purge old audit logs
     *
     * @param int $retentionDays Days to retain logs
     * @return int Number of records deleted
     */
    public function purgeOldLogs(int $retentionDays = 365): int
    {
        $cutoff = now()->subDays($retentionDays);

        // Keep critical events longer
        $deleted = DB::table('form_audit_logs')
            ->where('created_at', '<', $cutoff)
            ->where('severity', '!=', self::SEVERITY_CRITICAL)
            ->delete();

        // Delete critical events after 2x retention period
        $deleted += DB::table('form_audit_logs')
            ->where('created_at', '<', now()->subDays($retentionDays * 2))
            ->where('severity', self::SEVERITY_CRITICAL)
            ->delete();

        $this->log(self::EVENT_SETTINGS_CHANGED, [
            'action' => 'audit_log_purge',
            'cutoff_date' => $cutoff->toDateString(),
            'records_deleted' => $deleted,
        ]);

        return $deleted;
    }

    /**
     * Detect changes between original and new data
     */
    private function detectChanges(array $original, array $new): array
    {
        $changes = [];
        $ignoredKeys = ['updated_at', 'created_at'];

        foreach ($new as $key => $value) {
            if (in_array($key, $ignoredKeys)) {
                continue;
            }

            $originalValue = $original[$key] ?? null;

            if ($originalValue !== $value) {
                $changes[$key] = [
                    'old' => $this->truncateValue($originalValue),
                    'new' => $this->truncateValue($value),
                ];
            }
        }

        return $changes;
    }

    /**
     * Sanitize data for logging (remove sensitive info)
     */
    private function sanitizeData(array $data): array
    {
        $sensitiveKeys = ['password', 'secret', 'token', 'key', 'credit_card', 'ssn', 'api_key'];

        foreach ($data as $key => $value) {
            $lowerKey = strtolower($key);

            foreach ($sensitiveKeys as $sensitiveKey) {
                if (str_contains($lowerKey, $sensitiveKey)) {
                    $data[$key] = '[REDACTED]';
                    break;
                }
            }

            if (is_array($value)) {
                $data[$key] = $this->sanitizeData($value);
            }
        }

        return $data;
    }

    /**
     * Truncate long values for logging
     */
    private function truncateValue(mixed $value, int $maxLength = 200): mixed
    {
        if (is_string($value) && strlen($value) > $maxLength) {
            return substr($value, 0, $maxLength) . '...';
        }

        if (is_array($value)) {
            return '[ARRAY:' . count($value) . ' items]';
        }

        return $value;
    }
}
