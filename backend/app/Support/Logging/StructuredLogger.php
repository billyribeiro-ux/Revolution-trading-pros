<?php

declare(strict_types=1);

namespace App\Support\Logging;

use Illuminate\Support\Facades\Log;
use Psr\Log\LoggerInterface;
use Psr\Log\LogLevel;
use Stringable;

/**
 * Structured Logger
 *
 * Provides structured logging with automatic correlation ID injection,
 * consistent formatting, and metric integration.
 *
 * @version 1.0.0
 */
class StructuredLogger implements LoggerInterface
{
    /**
     * The logger channel.
     */
    private readonly string $channel;

    /**
     * Additional static context.
     *
     * @var array<string, mixed>
     */
    private array $staticContext = [];

    public function __construct(
        string $channel = 'default',
        array $staticContext = [],
    ) {
        $this->channel = $channel;
        $this->staticContext = $staticContext;
    }

    /**
     * Create a logger for a specific domain.
     */
    public static function forDomain(string $domain): self
    {
        return new self($domain, ['domain' => $domain]);
    }

    /**
     * Create a logger for email processing.
     */
    public static function email(): self
    {
        return self::forDomain('email');
    }

    /**
     * Create a logger for webhook processing.
     */
    public static function webhook(): self
    {
        return self::forDomain('webhook');
    }

    /**
     * Create a logger for security events.
     */
    public static function security(): self
    {
        return self::forDomain('security');
    }

    /**
     * Log an emergency message.
     */
    public function emergency(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::EMERGENCY, $message, $context);
    }

    /**
     * Log an alert message.
     */
    public function alert(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::ALERT, $message, $context);
    }

    /**
     * Log a critical message.
     */
    public function critical(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::CRITICAL, $message, $context);
    }

    /**
     * Log an error message.
     */
    public function error(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::ERROR, $message, $context);
    }

    /**
     * Log a warning message.
     */
    public function warning(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::WARNING, $message, $context);
    }

    /**
     * Log a notice message.
     */
    public function notice(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::NOTICE, $message, $context);
    }

    /**
     * Log an info message.
     */
    public function info(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::INFO, $message, $context);
    }

    /**
     * Log a debug message.
     */
    public function debug(Stringable|string $message, array $context = []): void
    {
        $this->log(LogLevel::DEBUG, $message, $context);
    }

    /**
     * Log with arbitrary level.
     *
     * @param array<string, mixed> $context
     */
    public function log($level, Stringable|string $message, array $context = []): void
    {
        $structuredContext = $this->buildContext($context);

        Log::channel($this->channel)->log($level, (string) $message, $structuredContext);
    }

    /**
     * Log an operation start.
     *
     * @param array<string, mixed> $context
     */
    public function operationStart(string $operation, array $context = []): void
    {
        $spanId = CorrelationContext::newSpan();

        $this->info("Starting: {$operation}", array_merge($context, [
            'operation' => $operation,
            'operation_phase' => 'start',
            'span_id' => $spanId,
        ]));
    }

    /**
     * Log an operation completion.
     *
     * @param array<string, mixed> $context
     */
    public function operationComplete(
        string $operation,
        float $durationMs,
        array $context = [],
    ): void {
        $this->info("Completed: {$operation}", array_merge($context, [
            'operation' => $operation,
            'operation_phase' => 'complete',
            'duration_ms' => round($durationMs, 2),
        ]));
    }

    /**
     * Log an operation failure.
     *
     * @param array<string, mixed> $context
     */
    public function operationFailed(
        string $operation,
        string $error,
        ?float $durationMs = null,
        array $context = [],
    ): void {
        $this->error("Failed: {$operation}", array_merge($context, [
            'operation' => $operation,
            'operation_phase' => 'failed',
            'error' => $error,
            'duration_ms' => $durationMs ? round($durationMs, 2) : null,
        ]));
    }

    /**
     * Log a webhook received.
     *
     * @param array<string, mixed> $context
     */
    public function webhookReceived(
        string $provider,
        string $type,
        array $context = [],
    ): void {
        $this->info("Webhook received: {$provider}/{$type}", array_merge($context, [
            'webhook_provider' => $provider,
            'webhook_type' => $type,
            'event_type' => 'webhook.received',
        ]));
    }

    /**
     * Log an email processed.
     *
     * @param array<string, mixed> $context
     */
    public function emailProcessed(
        string $messageId,
        string $from,
        array $context = [],
    ): void {
        $this->info("Email processed: {$messageId}", array_merge($context, [
            'message_id' => $messageId,
            'from_email' => $from,
            'event_type' => 'email.processed',
        ]));
    }

    /**
     * Log a security event.
     *
     * @param array<string, mixed> $context
     */
    public function securityEvent(
        string $event,
        string $severity,
        array $context = [],
    ): void {
        $level = match ($severity) {
            'critical' => LogLevel::CRITICAL,
            'high' => LogLevel::ERROR,
            'medium' => LogLevel::WARNING,
            'low' => LogLevel::NOTICE,
            default => LogLevel::INFO,
        };

        $this->log($level, "Security: {$event}", array_merge($context, [
            'security_event' => $event,
            'severity' => $severity,
            'event_type' => 'security',
        ]));
    }

    /**
     * Build the full context with correlation IDs.
     *
     * @param array<string, mixed> $context
     * @return array<string, mixed>
     */
    private function buildContext(array $context): array
    {
        return array_merge(
            CorrelationContext::all(),
            $this->staticContext,
            [
                'timestamp' => now()->toIso8601String(),
                'environment' => config('app.env'),
                'service' => config('app.name'),
            ],
            $context,
        );
    }

    /**
     * Add static context to all log entries.
     *
     * @param array<string, mixed> $context
     */
    public function withContext(array $context): self
    {
        $clone = clone $this;
        $clone->staticContext = array_merge($this->staticContext, $context);

        return $clone;
    }
}
