<?php

declare(strict_types=1);

namespace App\Support\Logging;

use Illuminate\Support\Str;

/**
 * Correlation Context
 *
 * Manages correlation IDs for distributed tracing across services.
 * Ensures all log entries can be traced back to a single request.
 *
 * @version 1.0.0
 */
class CorrelationContext
{
    /**
     * Current correlation ID.
     */
    private static ?string $correlationId = null;

    /**
     * Request ID (unique per HTTP request).
     */
    private static ?string $requestId = null;

    /**
     * Parent span ID for distributed tracing.
     */
    private static ?string $parentSpanId = null;

    /**
     * Current span ID.
     */
    private static ?string $spanId = null;

    /**
     * Additional context data.
     *
     * @var array<string, mixed>
     */
    private static array $contextData = [];

    /**
     * Get or create correlation ID.
     */
    public static function getCorrelationId(): string
    {
        if (self::$correlationId === null) {
            self::$correlationId = self::generateId('cor');
        }

        return self::$correlationId;
    }

    /**
     * Set correlation ID (from incoming request header).
     */
    public static function setCorrelationId(string $correlationId): void
    {
        self::$correlationId = $correlationId;
    }

    /**
     * Get or create request ID.
     */
    public static function getRequestId(): string
    {
        if (self::$requestId === null) {
            self::$requestId = self::generateId('req');
        }

        return self::$requestId;
    }

    /**
     * Set request ID.
     */
    public static function setRequestId(string $requestId): void
    {
        self::$requestId = $requestId;
    }

    /**
     * Get parent span ID.
     */
    public static function getParentSpanId(): ?string
    {
        return self::$parentSpanId;
    }

    /**
     * Set parent span ID (from incoming request header).
     */
    public static function setParentSpanId(?string $parentSpanId): void
    {
        self::$parentSpanId = $parentSpanId;
    }

    /**
     * Get or create current span ID.
     */
    public static function getSpanId(): string
    {
        if (self::$spanId === null) {
            self::$spanId = self::generateId('spn');
        }

        return self::$spanId;
    }

    /**
     * Create a new span (for tracing sub-operations).
     */
    public static function newSpan(): string
    {
        self::$parentSpanId = self::$spanId;
        self::$spanId = self::generateId('spn');

        return self::$spanId;
    }

    /**
     * Add context data.
     *
     * @param array<string, mixed> $data
     */
    public static function addContext(array $data): void
    {
        self::$contextData = array_merge(self::$contextData, $data);
    }

    /**
     * Set a single context value.
     */
    public static function set(string $key, mixed $value): void
    {
        self::$contextData[$key] = $value;
    }

    /**
     * Get a context value.
     */
    public static function get(string $key, mixed $default = null): mixed
    {
        return self::$contextData[$key] ?? $default;
    }

    /**
     * Get all context data for logging.
     *
     * @return array<string, mixed>
     */
    public static function all(): array
    {
        return array_filter([
            'correlation_id' => self::getCorrelationId(),
            'request_id' => self::getRequestId(),
            'span_id' => self::$spanId,
            'parent_span_id' => self::$parentSpanId,
            ...self::$contextData,
        ]);
    }

    /**
     * Get trace headers for propagation.
     *
     * @return array<string, string>
     */
    public static function getTraceHeaders(): array
    {
        return [
            'X-Correlation-ID' => self::getCorrelationId(),
            'X-Request-ID' => self::getRequestId(),
            'X-Span-ID' => self::getSpanId(),
            'X-Parent-Span-ID' => self::$parentSpanId ?? '',
        ];
    }

    /**
     * Initialize from HTTP request headers.
     *
     * @param array<string, string> $headers
     */
    public static function initFromHeaders(array $headers): void
    {
        if (isset($headers['X-Correlation-ID']) || isset($headers['x-correlation-id'])) {
            self::setCorrelationId($headers['X-Correlation-ID'] ?? $headers['x-correlation-id']);
        }

        if (isset($headers['X-Request-ID']) || isset($headers['x-request-id'])) {
            self::setRequestId($headers['X-Request-ID'] ?? $headers['x-request-id']);
        }

        if (isset($headers['X-Parent-Span-ID']) || isset($headers['x-parent-span-id'])) {
            self::setParentSpanId($headers['X-Parent-Span-ID'] ?? $headers['x-parent-span-id']);
        }
    }

    /**
     * Reset context (for new request).
     */
    public static function reset(): void
    {
        self::$correlationId = null;
        self::$requestId = null;
        self::$parentSpanId = null;
        self::$spanId = null;
        self::$contextData = [];
    }

    /**
     * Generate a unique ID.
     */
    private static function generateId(string $prefix): string
    {
        return sprintf(
            '%s_%s_%s',
            $prefix,
            now()->format('YmdHis'),
            Str::random(8),
        );
    }
}
