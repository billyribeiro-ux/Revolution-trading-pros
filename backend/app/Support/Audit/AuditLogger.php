<?php

declare(strict_types=1);

namespace App\Support\Audit;

use App\Support\Logging\CorrelationContext;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * Audit Logger
 *
 * Provides compliance-grade audit logging for sensitive operations.
 * Logs are immutable and include full context for forensic analysis.
 *
 * @version 1.0.0
 */
class AuditLogger
{
    /**
     * Audit event types.
     */
    public const EVENT_CREATE = 'create';
    public const EVENT_READ = 'read';
    public const EVENT_UPDATE = 'update';
    public const EVENT_DELETE = 'delete';
    public const EVENT_ACCESS = 'access';
    public const EVENT_LOGIN = 'login';
    public const EVENT_LOGOUT = 'logout';
    public const EVENT_EXPORT = 'export';
    public const EVENT_IMPORT = 'import';

    /**
     * Log channel for audit events.
     */
    private string $channel = 'audit';

    /**
     * Whether to store in database.
     */
    private bool $storeInDatabase = true;

    /**
     * Create an audit logger instance.
     */
    public static function create(): self
    {
        return new self();
    }

    /**
     * Log a model creation.
     */
    public function logCreate(Model $model, ?string $description = null): void
    {
        $this->log(self::EVENT_CREATE, [
            'model_type' => get_class($model),
            'model_id' => $model->getKey(),
            'attributes' => $this->sanitizeAttributes($model->getAttributes()),
            'description' => $description ?? "Created {$this->getModelName($model)}",
        ]);
    }

    /**
     * Log a model read/view.
     */
    public function logRead(Model $model, ?string $description = null): void
    {
        $this->log(self::EVENT_READ, [
            'model_type' => get_class($model),
            'model_id' => $model->getKey(),
            'description' => $description ?? "Viewed {$this->getModelName($model)}",
        ]);
    }

    /**
     * Log a model update.
     *
     * @param array<string, mixed> $original Original values (before update)
     * @param array<string, mixed> $changes Changed values
     */
    public function logUpdate(
        Model $model,
        array $original,
        array $changes,
        ?string $description = null,
    ): void {
        $this->log(self::EVENT_UPDATE, [
            'model_type' => get_class($model),
            'model_id' => $model->getKey(),
            'original' => $this->sanitizeAttributes($original),
            'changes' => $this->sanitizeAttributes($changes),
            'description' => $description ?? "Updated {$this->getModelName($model)}",
        ]);
    }

    /**
     * Log a model deletion.
     */
    public function logDelete(Model $model, ?string $description = null): void
    {
        $this->log(self::EVENT_DELETE, [
            'model_type' => get_class($model),
            'model_id' => $model->getKey(),
            'attributes' => $this->sanitizeAttributes($model->getAttributes()),
            'description' => $description ?? "Deleted {$this->getModelName($model)}",
        ]);
    }

    /**
     * Log a data access event.
     *
     * @param array<string, mixed> $context
     */
    public function logAccess(string $resource, string $action, array $context = []): void
    {
        $this->log(self::EVENT_ACCESS, array_merge([
            'resource' => $resource,
            'action' => $action,
            'description' => "Accessed {$resource}: {$action}",
        ], $context));
    }

    /**
     * Log a user login.
     *
     * @param array<string, mixed> $context
     */
    public function logLogin(int|string $userId, bool $success, array $context = []): void
    {
        $this->log(self::EVENT_LOGIN, array_merge([
            'target_user_id' => $userId,
            'success' => $success,
            'description' => $success ? 'User logged in' : 'Login attempt failed',
        ], $context));
    }

    /**
     * Log a user logout.
     *
     * @param array<string, mixed> $context
     */
    public function logLogout(int|string $userId, array $context = []): void
    {
        $this->log(self::EVENT_LOGOUT, array_merge([
            'target_user_id' => $userId,
            'description' => 'User logged out',
        ], $context));
    }

    /**
     * Log a data export.
     *
     * @param array<string, mixed> $context
     */
    public function logExport(
        string $dataType,
        int $recordCount,
        string $format,
        array $context = [],
    ): void {
        $this->log(self::EVENT_EXPORT, array_merge([
            'data_type' => $dataType,
            'record_count' => $recordCount,
            'format' => $format,
            'description' => "Exported {$recordCount} {$dataType} records as {$format}",
        ], $context));
    }

    /**
     * Log a data import.
     *
     * @param array<string, mixed> $context
     */
    public function logImport(
        string $dataType,
        int $recordCount,
        string $source,
        array $context = [],
    ): void {
        $this->log(self::EVENT_IMPORT, array_merge([
            'data_type' => $dataType,
            'record_count' => $recordCount,
            'source' => $source,
            'description' => "Imported {$recordCount} {$dataType} records from {$source}",
        ], $context));
    }

    /**
     * Log a custom audit event.
     *
     * @param array<string, mixed> $data
     */
    public function log(string $eventType, array $data = []): void
    {
        $auditEntry = $this->buildAuditEntry($eventType, $data);

        // Log to file
        Log::channel($this->channel)->info($eventType, $auditEntry);

        // Store in database if enabled
        if ($this->storeInDatabase) {
            $this->storeAuditEntry($auditEntry);
        }
    }

    /**
     * Build the audit entry.
     *
     * @param array<string, mixed> $data
     * @return array<string, mixed>
     */
    private function buildAuditEntry(string $eventType, array $data): array
    {
        $user = Auth::user();

        return [
            'event_type' => $eventType,
            'timestamp' => now()->toIso8601String(),
            'correlation_id' => CorrelationContext::getCorrelationId(),
            'request_id' => CorrelationContext::getRequestId(),
            'user_id' => $user?->id,
            'user_email' => $user?->email,
            'ip_address' => request()?->ip(),
            'user_agent' => substr(request()?->userAgent() ?? '', 0, 255),
            'session_id' => session()?->getId(),
            'data' => $data,
            'environment' => config('app.env'),
            'service' => config('app.name'),
        ];
    }

    /**
     * Store audit entry in database.
     *
     * @param array<string, mixed> $entry
     */
    private function storeAuditEntry(array $entry): void
    {
        try {
            DB::table('audit_logs')->insert([
                'id' => (string) \Illuminate\Support\Str::uuid(),
                'event_type' => $entry['event_type'],
                'correlation_id' => $entry['correlation_id'],
                'user_id' => $entry['user_id'],
                'ip_address' => $entry['ip_address'],
                'user_agent' => $entry['user_agent'],
                'data' => json_encode($entry['data']),
                'created_at' => now(),
            ]);
        } catch (\Throwable $e) {
            // Don't let audit failures break the application
            Log::error('Failed to store audit entry', [
                'error' => $e->getMessage(),
                'entry' => $entry,
            ]);
        }
    }

    /**
     * Sanitize attributes to remove sensitive data.
     *
     * @param array<string, mixed> $attributes
     * @return array<string, mixed>
     */
    private function sanitizeAttributes(array $attributes): array
    {
        $sensitiveKeys = [
            'password',
            'password_confirmation',
            'secret',
            'api_key',
            'api_secret',
            'token',
            'access_token',
            'refresh_token',
            'credit_card',
            'card_number',
            'cvv',
            'ssn',
            'social_security',
        ];

        foreach ($attributes as $key => $value) {
            $lowerKey = strtolower($key);

            foreach ($sensitiveKeys as $sensitive) {
                if (str_contains($lowerKey, $sensitive)) {
                    $attributes[$key] = '[REDACTED]';
                    break;
                }
            }

            // Recursively sanitize nested arrays
            if (is_array($value)) {
                $attributes[$key] = $this->sanitizeAttributes($value);
            }
        }

        return $attributes;
    }

    /**
     * Get a human-readable model name.
     */
    private function getModelName(Model $model): string
    {
        $class = get_class($model);
        $parts = explode('\\', $class);

        return end($parts) . " #{$model->getKey()}";
    }

    /**
     * Disable database storage.
     */
    public function withoutDatabase(): self
    {
        $this->storeInDatabase = false;

        return $this;
    }

    /**
     * Set the log channel.
     */
    public function channel(string $channel): self
    {
        $this->channel = $channel;

        return $this;
    }
}
