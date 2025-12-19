<?php

declare(strict_types=1);

namespace App\Console\Commands;

use Illuminate\Console\Command;
use App\Services\Cache\CacheMetrics;

/**
 * Cache Metrics Command
 *
 * Displays cache metrics and statistics
 *
 * Usage:
 *   php artisan cache:metrics           - Show all metrics
 *   php artisan cache:metrics --tier=l2 - Show specific tier metrics
 *   php artisan cache:metrics --reset   - Reset all metrics
 */
class CacheMetricsCommand extends Command
{
    protected $signature = 'cache:metrics
                            {--tier= : Specific tier to show (l1, l2, l3)}
                            {--reset : Reset all metrics}
                            {--json : Output as JSON}';

    protected $description = 'Display cache metrics and statistics';

    public function handle(CacheMetrics $metrics): int
    {
        if ($this->option('reset')) {
            $metrics->reset();
            $this->info('Cache metrics reset successfully.');
            return 0;
        }

        $tier = $this->option('tier');

        if ($tier) {
            $data = $metrics->getTierMetrics($tier);
            $this->displayTierMetrics($tier, $data);
            return 0;
        }

        $allMetrics = $metrics->getMetrics();

        if ($this->option('json')) {
            $this->line(json_encode($allMetrics, JSON_PRETTY_PRINT));
            return 0;
        }

        $this->displayAllMetrics($allMetrics);

        return 0;
    }

    private function displayTierMetrics(string $tier, array $data): void
    {
        $this->info(strtoupper($tier) . ' Cache Metrics');
        $this->newLine();

        $this->table(
            ['Metric', 'Value'],
            [
                ['Hits', number_format($data['hits'])],
                ['Misses', number_format($data['misses'])],
                ['Total Requests', number_format($data['total'])],
                ['Hit Rate', $data['hit_rate'] . '%'],
                ['Avg Response', $data['avg_response_ms'] . 'ms'],
                ['P95 Response', $data['p95_response_ms'] . 'ms'],
                ['P99 Response', $data['p99_response_ms'] . 'ms'],
                ['Total Size', $this->formatBytes($data['total_size_bytes'])],
            ]
        );
    }

    private function displayAllMetrics(array $data): void
    {
        // Overall metrics
        $this->info('═══════════════════════════════════════════════════════════════');
        $this->info('                    CACHE METRICS DASHBOARD                     ');
        $this->info('═══════════════════════════════════════════════════════════════');
        $this->newLine();

        // Overall stats
        $overall = $data['overall'];
        $this->info('📊 Overall Statistics');
        $this->table(
            ['Metric', 'Value'],
            [
                ['Total Requests', number_format($overall['total_requests'])],
                ['Total Hits', number_format($overall['total_hits'])],
                ['Total Misses', number_format($overall['total_misses'])],
                ['Overall Hit Rate', $this->colorizeRate($overall['overall_hit_rate']) . '%'],
                ['L1 Efficiency', $overall['l1_efficiency'] . '%'],
                ['Cache Savings', $overall['cache_savings_pct'] . '%'],
            ]
        );
        $this->newLine();

        // Tier breakdown
        $this->info('📈 Cache Tiers');
        $this->table(
            ['Tier', 'Hits', 'Misses', 'Hit Rate', 'Avg Response'],
            [
                ['L1 (Memory)', number_format($data['l1']['hits']), number_format($data['l1']['misses']), $data['l1']['hit_rate'] . '%', $data['l1']['avg_response_ms'] . 'ms'],
                ['L2 (Redis)', number_format($data['l2']['hits']), number_format($data['l2']['misses']), $data['l2']['hit_rate'] . '%', $data['l2']['avg_response_ms'] . 'ms'],
                ['L3 (Database)', number_format($data['l3']['hits']), number_format($data['l3']['misses']), $data['l3']['hit_rate'] . '%', $data['l3']['avg_response_ms'] . 'ms'],
            ]
        );
        $this->newLine();

        // Redis metrics
        if (isset($data['redis']) && !isset($data['redis']['error'])) {
            $redis = $data['redis'];
            $this->info('🔴 Redis Status');
            $this->table(
                ['Metric', 'Value'],
                [
                    ['Memory Used', $redis['used_memory']],
                    ['Memory Usage', $redis['memory_usage_pct'] . '%'],
                    ['Connected Clients', $redis['connected_clients']],
                    ['Keys in DB', number_format($redis['db_size'])],
                    ['Evicted Keys', number_format($redis['evicted_keys'])],
                    ['Uptime', $this->formatDuration($redis['uptime_seconds'])],
                ]
            );
            $this->newLine();
        }

        // Database cache metrics
        if (isset($data['database']) && !isset($data['database']['error'])) {
            $db = $data['database'];
            $this->info('💾 Database Cache (L3)');
            $this->table(
                ['Metric', 'Value'],
                [
                    ['Total Entries', number_format($db['total_entries'])],
                    ['Expired Entries', number_format($db['expired_entries'])],
                    ['Total Size', $this->formatBytes($db['total_size_bytes'])],
                ]
            );
            $this->newLine();
        }

        // Alerts
        if (!empty($data['alerts'])) {
            $this->warn('⚠️  Alerts');
            foreach ($data['alerts'] as $alert) {
                $icon = $alert['level'] === 'critical' ? '🔴' : '🟡';
                $this->line("  {$icon} {$alert['message']}");
            }
            $this->newLine();
        }

        $this->info("Last updated: {$data['timestamp']}");
    }

    private function colorizeRate(float $rate): string
    {
        if ($rate >= 90) {
            return "<fg=green>{$rate}</>";
        } elseif ($rate >= 70) {
            return "<fg=yellow>{$rate}</>";
        } else {
            return "<fg=red>{$rate}</>";
        }
    }

    private function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB', 'TB'];
        $i = 0;
        while ($bytes >= 1024 && $i < count($units) - 1) {
            $bytes /= 1024;
            $i++;
        }
        return round($bytes, 2) . ' ' . $units[$i];
    }

    private function formatDuration(int $seconds): string
    {
        if ($seconds < 60) {
            return "{$seconds}s";
        } elseif ($seconds < 3600) {
            return round($seconds / 60) . 'm';
        } elseif ($seconds < 86400) {
            return round($seconds / 3600, 1) . 'h';
        } else {
            return round($seconds / 86400, 1) . 'd';
        }
    }
}
