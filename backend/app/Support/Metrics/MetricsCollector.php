<?php

declare(strict_types=1);

namespace App\Support\Metrics;

use Illuminate\Contracts\Cache\Repository as CacheRepository;
use Illuminate\Support\Collection;

/**
 * Metrics Collector
 *
 * Collects and exposes application metrics in Prometheus format.
 * Uses cache for atomic counter operations and metric aggregation.
 *
 * @version 1.0.0
 */
class MetricsCollector
{
    /**
     * Metric namespace prefix.
     */
    private const NAMESPACE = 'rtp';

    /**
     * Cache TTL for metrics in seconds.
     */
    private const CACHE_TTL = 86400; // 24 hours

    public function __construct(
        private readonly CacheRepository $cache,
    ) {}

    /**
     * Increment a counter metric.
     *
     * @param array<string, string> $labels
     */
    public function incrementCounter(string $name, array $labels = [], float $value = 1): void
    {
        $key = $this->buildKey('counter', $name, $labels);
        $this->cache->increment($key, (int) $value);

        // Register metric if not exists
        $this->registerMetric('counter', $name, $labels);
    }

    /**
     * Set a gauge metric value.
     *
     * @param array<string, string> $labels
     */
    public function setGauge(string $name, float $value, array $labels = []): void
    {
        $key = $this->buildKey('gauge', $name, $labels);
        $this->cache->put($key, $value, now()->addSeconds(self::CACHE_TTL));

        // Register metric if not exists
        $this->registerMetric('gauge', $name, $labels);
    }

    /**
     * Increment a gauge metric.
     *
     * @param array<string, string> $labels
     */
    public function incrementGauge(string $name, float $value = 1, array $labels = []): void
    {
        $key = $this->buildKey('gauge', $name, $labels);
        $current = (float) $this->cache->get($key, 0);
        $this->cache->put($key, $current + $value, now()->addSeconds(self::CACHE_TTL));

        $this->registerMetric('gauge', $name, $labels);
    }

    /**
     * Decrement a gauge metric.
     *
     * @param array<string, string> $labels
     */
    public function decrementGauge(string $name, float $value = 1, array $labels = []): void
    {
        $this->incrementGauge($name, -$value, $labels);
    }

    /**
     * Record a histogram observation.
     *
     * @param array<string, string> $labels
     * @param array<float> $buckets
     */
    public function observeHistogram(
        string $name,
        float $value,
        array $labels = [],
        array $buckets = [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10],
    ): void {
        $baseKey = $this->buildKey('histogram', $name, $labels);

        // Increment sum
        $sumKey = "{$baseKey}:sum";
        $sum = (float) $this->cache->get($sumKey, 0);
        $this->cache->put($sumKey, $sum + $value, now()->addSeconds(self::CACHE_TTL));

        // Increment count
        $countKey = "{$baseKey}:count";
        $this->cache->increment($countKey);

        // Increment bucket counters
        foreach ($buckets as $bucket) {
            if ($value <= $bucket) {
                $bucketKey = "{$baseKey}:bucket:{$bucket}";
                $this->cache->increment($bucketKey);
            }
        }

        // +Inf bucket
        $infKey = "{$baseKey}:bucket:inf";
        $this->cache->increment($infKey);

        $this->registerMetric('histogram', $name, $labels, $buckets);
    }

    /**
     * Record request duration.
     *
     * @param array<string, string> $labels
     */
    public function recordRequestDuration(float $durationSeconds, array $labels = []): void
    {
        $this->observeHistogram(
            'http_request_duration_seconds',
            $durationSeconds,
            $labels,
            [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10],
        );
    }

    /**
     * Record email processing.
     *
     * @param array<string, string> $labels
     */
    public function recordEmailProcessed(string $provider, string $status): void
    {
        $this->incrementCounter('email_processed_total', [
            'provider' => $provider,
            'status' => $status,
        ]);
    }

    /**
     * Record email processing duration.
     */
    public function recordEmailProcessingDuration(float $durationSeconds, string $provider): void
    {
        $this->observeHistogram(
            'email_processing_duration_seconds',
            $durationSeconds,
            ['provider' => $provider],
            [0.01, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10, 30],
        );
    }

    /**
     * Record attachment processed.
     */
    public function recordAttachmentProcessed(string $status, int $sizeBytes): void
    {
        $this->incrementCounter('attachment_processed_total', ['status' => $status]);
        $this->observeHistogram(
            'attachment_size_bytes',
            $sizeBytes,
            [],
            [1024, 10240, 102400, 1048576, 10485760, 26214400], // 1KB to 25MB
        );
    }

    /**
     * Record circuit breaker state change.
     */
    public function recordCircuitBreakerState(string $circuit, string $state): void
    {
        // Set gauge to represent state (0=closed, 1=half-open, 2=open)
        $stateValue = match ($state) {
            'closed' => 0,
            'half_open' => 1,
            'open' => 2,
            default => -1,
        };

        $this->setGauge('circuit_breaker_state', $stateValue, ['circuit' => $circuit]);
        $this->incrementCounter('circuit_breaker_transitions_total', [
            'circuit' => $circuit,
            'to_state' => $state,
        ]);
    }

    /**
     * Record spam detection.
     */
    public function recordSpamDetection(bool $isSpam, string $provider): void
    {
        $this->incrementCounter('spam_detection_total', [
            'provider' => $provider,
            'is_spam' => $isSpam ? 'true' : 'false',
        ]);
    }

    /**
     * Record webhook received.
     */
    public function recordWebhookReceived(string $provider): void
    {
        $this->incrementCounter('webhook_received_total', ['provider' => $provider]);
    }

    /**
     * Record webhook error.
     */
    public function recordWebhookError(string $provider, string $errorType): void
    {
        $this->incrementCounter('webhook_errors_total', [
            'provider' => $provider,
            'error_type' => $errorType,
        ]);
    }

    /**
     * Set active conversations gauge.
     */
    public function setActiveConversations(int $count): void
    {
        $this->setGauge('active_conversations', $count);
    }

    /**
     * Set unread messages gauge.
     */
    public function setUnreadMessages(int $count): void
    {
        $this->setGauge('unread_messages', $count);
    }

    /**
     * Export metrics in Prometheus format.
     */
    public function export(): string
    {
        $output = [];
        $registered = $this->getRegisteredMetrics();

        foreach ($registered as $metric) {
            $type = $metric['type'];
            $name = self::NAMESPACE . '_' . $metric['name'];

            // Add TYPE and HELP
            $output[] = "# HELP {$name} {$this->getMetricHelp($metric['name'])}";
            $output[] = "# TYPE {$name} {$type}";

            if ($type === 'histogram') {
                $this->exportHistogram($output, $metric);
            } else {
                $this->exportSimpleMetric($output, $metric);
            }
        }

        return implode("\n", $output) . "\n";
    }

    /**
     * Get metric help text.
     */
    private function getMetricHelp(string $name): string
    {
        $helps = [
            'http_request_duration_seconds' => 'HTTP request duration in seconds',
            'email_processed_total' => 'Total number of emails processed',
            'email_processing_duration_seconds' => 'Email processing duration in seconds',
            'attachment_processed_total' => 'Total number of attachments processed',
            'attachment_size_bytes' => 'Attachment size in bytes',
            'circuit_breaker_state' => 'Circuit breaker state (0=closed, 1=half-open, 2=open)',
            'circuit_breaker_transitions_total' => 'Total circuit breaker state transitions',
            'spam_detection_total' => 'Total spam detection results',
            'webhook_received_total' => 'Total webhooks received',
            'webhook_errors_total' => 'Total webhook processing errors',
            'active_conversations' => 'Number of active conversations',
            'unread_messages' => 'Number of unread messages',
        ];

        return $helps[$name] ?? $name;
    }

    /**
     * Build cache key for metric.
     *
     * @param array<string, string> $labels
     */
    private function buildKey(string $type, string $name, array $labels): string
    {
        $labelString = $this->labelsToString($labels);

        return "metrics:{$type}:{$name}:{$labelString}";
    }

    /**
     * Convert labels to string for cache key.
     *
     * @param array<string, string> $labels
     */
    private function labelsToString(array $labels): string
    {
        if (empty($labels)) {
            return 'no_labels';
        }

        ksort($labels);
        $parts = [];

        foreach ($labels as $key => $value) {
            $parts[] = "{$key}={$value}";
        }

        return md5(implode(',', $parts));
    }

    /**
     * Register a metric.
     *
     * @param array<string, string> $labels
     * @param array<float> $buckets
     */
    private function registerMetric(
        string $type,
        string $name,
        array $labels,
        array $buckets = [],
    ): void {
        $registered = $this->getRegisteredMetrics();
        $key = "{$type}:{$name}";

        if (!isset($registered[$key])) {
            $registered[$key] = [
                'type' => $type,
                'name' => $name,
                'label_names' => array_keys($labels),
                'buckets' => $buckets,
                'label_values' => [],
            ];
        }

        // Track label combinations
        if (!empty($labels)) {
            $labelHash = $this->labelsToString($labels);
            $registered[$key]['label_values'][$labelHash] = $labels;
        }

        $this->cache->put(
            'metrics:registered',
            $registered,
            now()->addSeconds(self::CACHE_TTL),
        );
    }

    /**
     * Get registered metrics.
     *
     * @return array<string, array<string, mixed>>
     */
    private function getRegisteredMetrics(): array
    {
        return $this->cache->get('metrics:registered', []);
    }

    /**
     * Export a simple counter or gauge metric.
     *
     * @param array<string> $output
     * @param array<string, mixed> $metric
     */
    private function exportSimpleMetric(array &$output, array $metric): void
    {
        $name = self::NAMESPACE . '_' . $metric['name'];
        $type = $metric['type'];

        foreach ($metric['label_values'] as $labelHash => $labels) {
            $key = $this->buildKey($type, $metric['name'], $labels);
            $value = $this->cache->get($key, 0);
            $labelString = $this->formatLabels($labels);

            $output[] = "{$name}{$labelString} {$value}";
        }

        // Handle no-label metrics
        if (empty($metric['label_values'])) {
            $key = $this->buildKey($type, $metric['name'], []);
            $value = $this->cache->get($key, 0);
            $output[] = "{$name} {$value}";
        }
    }

    /**
     * Export a histogram metric.
     *
     * @param array<string> $output
     * @param array<string, mixed> $metric
     */
    private function exportHistogram(array &$output, array $metric): void
    {
        $name = self::NAMESPACE . '_' . $metric['name'];
        $buckets = $metric['buckets'] ?? [];

        foreach ($metric['label_values'] as $labelHash => $labels) {
            $baseKey = $this->buildKey('histogram', $metric['name'], $labels);
            $labelString = $this->formatLabels($labels);
            $labelStringWithComma = empty($labels) ? '' : $labelString . ',';

            // Export bucket values
            foreach ($buckets as $bucket) {
                $bucketKey = "{$baseKey}:bucket:{$bucket}";
                $value = $this->cache->get($bucketKey, 0);
                $le = $bucket === INF ? '+Inf' : $bucket;
                $output[] = "{$name}_bucket{" . ltrim($labelStringWithComma, '{') . "le=\"{$le}\"} {$value}";
            }

            // +Inf bucket
            $infKey = "{$baseKey}:bucket:inf";
            $infValue = $this->cache->get($infKey, 0);
            $output[] = "{$name}_bucket{" . ltrim($labelStringWithComma, '{') . "le=\"+Inf\"} {$infValue}";

            // Sum
            $sumKey = "{$baseKey}:sum";
            $sum = $this->cache->get($sumKey, 0);
            $output[] = "{$name}_sum{$labelString} {$sum}";

            // Count
            $countKey = "{$baseKey}:count";
            $count = $this->cache->get($countKey, 0);
            $output[] = "{$name}_count{$labelString} {$count}";
        }
    }

    /**
     * Format labels for Prometheus output.
     *
     * @param array<string, string> $labels
     */
    private function formatLabels(array $labels): string
    {
        if (empty($labels)) {
            return '';
        }

        $parts = [];
        foreach ($labels as $key => $value) {
            $escapedValue = addslashes($value);
            $parts[] = "{$key}=\"{$escapedValue}\"";
        }

        return '{' . implode(',', $parts) . '}';
    }
}
