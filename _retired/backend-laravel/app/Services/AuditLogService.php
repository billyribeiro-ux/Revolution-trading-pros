<?php

declare(strict_types=1);

namespace App\Services;

use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Illuminate\Http\Request;

/**
 * AuditLogService - Enterprise Audit Logging System
 * ==================================================
 *
 * Google L11 Principal Engineer standard audit logging for:
 * - Security events (auth, access control)
 * - Data modifications (CRUD operations)
 * - System events (errors, performance)
 * - Compliance (GDPR, SOC2, PCI-DSS)
 *
 * Features:
 * - Structured JSON logging
 * - Correlation IDs for request tracing
 * - PII redaction
 * - Tamper-evident logging
 *
 * @version 1.0.0
 * @security Critical
 */
class AuditLogService
{
    // Audit event types
    public const EVENT_AUTH_LOGIN = 'auth.login';
    public const EVENT_AUTH_LOGOUT = 'auth.logout';
    public const EVENT_AUTH_FAILED = 'auth.failed';
    public const EVENT_AUTH_MFA = 'auth.mfa';
    public const EVENT_AUTH_PASSWORD_CHANGE = 'auth.password_change';
    public const EVENT_AUTH_PASSWORD_RESET = 'auth.password_reset';

    public const EVENT_USER_CREATE = 'user.create';
    public const EVENT_USER_UPDATE = 'user.update';
    public const EVENT_USER_DELETE = 'user.delete';
    public const EVENT_USER_ROLE_CHANGE = 'user.role_change';

    public const EVENT_DATA_CREATE = 'data.create';
    public const EVENT_DATA_UPDATE = 'data.update';
    public const EVENT_DATA_DELETE = 'data.delete';
    public const EVENT_DATA_EXPORT = 'data.export';

    public const EVENT_ACCESS_GRANTED = 'access.granted';
    public const EVENT_ACCESS_DENIED = 'access.denied';

    public const EVENT_PAYMENT_INITIATED = 'payment.initiated';
    public const EVENT_PAYMENT_COMPLETED = 'payment.completed';
    public const EVENT_PAYMENT_FAILED = 'payment.failed';

    public const EVENT_SECURITY_SUSPICIOUS = 'security.suspicious';
    public const EVENT_SECURITY_BREACH_ATTEMPT = 'security.breach_attempt';

    // PII fields to redact
    private const PII_FIELDS = [
        'password', 'password_confirmation', 'current_password', 'new_password',
        'credit_card', 'card_number', 'cvv', 'cvc', 'ssn', 'social_security',
        'token', 'api_key', 'secret', 'access_token', 'refresh_token',
        'mfa_secret', 'backup_codes', 'private_key',
    ];

    private ?string $correlationId = null;
    private ?Request $request = null;

    /**
     * Set the current request context
     */
    public function setRequest(Request $request): self
    {
        $this->request = $request;
        $this->correlationId = $request->header('X-Correlation-ID') ?? (string) Str::uuid();
        return $this;
    }

    /**
     * Get or generate correlation ID
     */
    public function getCorrelationId(): string
    {
        return $this->correlationId ?? (string) Str::uuid();
    }

    /**
     * Log an audit event
     */
    public function log(
        string $eventType,
        string $description,
        array $data = [],
        ?int $userId = null,
        string $severity = 'info'
    ): void {
        $auditEntry = [
            'audit_id' => (string) Str::uuid(),
            'correlation_id' => $this->getCorrelationId(),
            'timestamp' => now()->toIso8601String(),
            'event_type' => $eventType,
            'description' => $description,
            'severity' => $severity,
            'user_id' => $userId ?? auth()->id(),
            'ip_address' => $this->request?->ip() ?? 'unknown',
            'user_agent' => $this->request?->userAgent() ?? 'unknown',
            'request_method' => $this->request?->method() ?? 'unknown',
            'request_path' => $this->request?->path() ?? 'unknown',
            'data' => $this->redactPII($data),
            'checksum' => null, // Will be set below
        ];

        // Generate tamper-evident checksum
        $auditEntry['checksum'] = $this->generateChecksum($auditEntry);

        // Log to file (structured JSON for log aggregation)
        Log::channel('audit')->{$severity}(json_encode($auditEntry));

        // Store in database for querying (async in production)
        $this->storeAuditLog($auditEntry);
    }

    /**
     * Log authentication event
     */
    public function logAuth(string $action, bool $success, array $data = []): void
    {
        $eventType = match ($action) {
            'login' => $success ? self::EVENT_AUTH_LOGIN : self::EVENT_AUTH_FAILED,
            'logout' => self::EVENT_AUTH_LOGOUT,
            'mfa' => self::EVENT_AUTH_MFA,
            'password_change' => self::EVENT_AUTH_PASSWORD_CHANGE,
            'password_reset' => self::EVENT_AUTH_PASSWORD_RESET,
            default => 'auth.unknown',
        };

        $this->log(
            $eventType,
            "Authentication event: {$action} - " . ($success ? 'success' : 'failed'),
            $data,
            null,
            $success ? 'info' : 'warning'
        );
    }

    /**
     * Log data access/modification event
     */
    public function logDataAccess(
        string $action,
        string $resourceType,
        string|int $resourceId,
        array $changes = []
    ): void {
        $eventType = match ($action) {
            'create' => self::EVENT_DATA_CREATE,
            'update' => self::EVENT_DATA_UPDATE,
            'delete' => self::EVENT_DATA_DELETE,
            'export' => self::EVENT_DATA_EXPORT,
            default => 'data.access',
        };

        $this->log(
            $eventType,
            "Data {$action}: {$resourceType} #{$resourceId}",
            [
                'resource_type' => $resourceType,
                'resource_id' => $resourceId,
                'changes' => $changes,
            ]
        );
    }

    /**
     * Log security event
     */
    public function logSecurity(string $type, string $description, array $data = []): void
    {
        $eventType = match ($type) {
            'suspicious' => self::EVENT_SECURITY_SUSPICIOUS,
            'breach_attempt' => self::EVENT_SECURITY_BREACH_ATTEMPT,
            'access_denied' => self::EVENT_ACCESS_DENIED,
            default => 'security.event',
        };

        $this->log(
            $eventType,
            $description,
            $data,
            null,
            $type === 'breach_attempt' ? 'critical' : 'warning'
        );
    }

    /**
     * Log payment event
     */
    public function logPayment(string $action, string $orderId, float $amount, array $data = []): void
    {
        $eventType = match ($action) {
            'initiated' => self::EVENT_PAYMENT_INITIATED,
            'completed' => self::EVENT_PAYMENT_COMPLETED,
            'failed' => self::EVENT_PAYMENT_FAILED,
            default => 'payment.event',
        };

        $this->log(
            $eventType,
            "Payment {$action}: Order #{$orderId} - \${$amount}",
            array_merge($data, [
                'order_id' => $orderId,
                'amount' => $amount,
            ]),
            null,
            $action === 'failed' ? 'error' : 'info'
        );
    }

    /**
     * Redact PII from data
     */
    private function redactPII(array $data): array
    {
        array_walk_recursive($data, function (&$value, $key) {
            if (is_string($key)) {
                $lowercaseKey = strtolower($key);
                foreach (self::PII_FIELDS as $piiField) {
                    if (str_contains($lowercaseKey, $piiField)) {
                        $value = '[REDACTED]';
                        return;
                    }
                }
            }

            // Redact email addresses partially
            if (is_string($value) && filter_var($value, FILTER_VALIDATE_EMAIL)) {
                $parts = explode('@', $value);
                $value = substr($parts[0], 0, 2) . '***@' . $parts[1];
            }
        });

        return $data;
    }

    /**
     * Generate tamper-evident checksum
     */
    private function generateChecksum(array $data): string
    {
        unset($data['checksum']);
        $secret = config('app.key');
        return hash_hmac('sha256', json_encode($data), $secret);
    }

    /**
     * Store audit log in database
     */
    private function storeAuditLog(array $entry): void
    {
        try {
            DB::table('audit_logs')->insert([
                'id' => $entry['audit_id'],
                'correlation_id' => $entry['correlation_id'],
                'event_type' => $entry['event_type'],
                'description' => $entry['description'],
                'severity' => $entry['severity'],
                'user_id' => $entry['user_id'],
                'ip_address' => $entry['ip_address'],
                'user_agent' => substr($entry['user_agent'], 0, 500),
                'request_method' => $entry['request_method'],
                'request_path' => $entry['request_path'],
                'data' => json_encode($entry['data']),
                'checksum' => $entry['checksum'],
                'created_at' => now(),
            ]);
        } catch (\Exception $e) {
            // Don't let audit logging failures break the application
            Log::error('Failed to store audit log', [
                'error' => $e->getMessage(),
                'audit_id' => $entry['audit_id'],
            ]);
        }
    }

    /**
     * Verify audit log integrity
     */
    public function verifyIntegrity(string $auditId): bool
    {
        $entry = DB::table('audit_logs')->where('id', $auditId)->first();

        if (!$entry) {
            return false;
        }

        $data = [
            'audit_id' => $entry->id,
            'correlation_id' => $entry->correlation_id,
            'timestamp' => $entry->created_at,
            'event_type' => $entry->event_type,
            'description' => $entry->description,
            'severity' => $entry->severity,
            'user_id' => $entry->user_id,
            'ip_address' => $entry->ip_address,
            'user_agent' => $entry->user_agent,
            'request_method' => $entry->request_method,
            'request_path' => $entry->request_path,
            'data' => json_decode($entry->data, true),
        ];

        $expectedChecksum = $this->generateChecksum($data);

        return hash_equals($entry->checksum, $expectedChecksum);
    }
}
