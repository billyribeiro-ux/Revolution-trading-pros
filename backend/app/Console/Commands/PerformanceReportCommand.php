<?php

declare(strict_types=1);

namespace App\Console\Commands;

use Illuminate\Console\Command;
use App\Providers\PerformanceServiceProvider;

/**
 * Performance Report Command
 *
 * Generates comprehensive performance diagnostics
 */
class PerformanceReportCommand extends Command
{
    protected $signature = 'performance:report
                            {--json : Output as JSON}';

    protected $description = 'Generate comprehensive performance diagnostics report';

    public function handle(PerformanceServiceProvider $provider): int
    {
        $this->info('Generating performance report...');
        $this->newLine();

        $diagnostics = $provider->getDiagnostics();

        if ($this->option('json')) {
            $this->line(json_encode($diagnostics, JSON_PRETTY_PRINT));
            return 0;
        }

        // OPcache Status
        $this->info('═══════════════════════════════════════════════════════════════');
        $this->info('                    PERFORMANCE DIAGNOSTICS                     ');
        $this->info('═══════════════════════════════════════════════════════════════');
        $this->newLine();

        // OPcache
        $opcache = $diagnostics['opcache'];
        $this->info('🔧 OPcache Status');
        if ($opcache['enabled']) {
            $this->table(
                ['Metric', 'Value'],
                [
                    ['Status', '✓ Enabled'],
                    ['Memory Used', $this->formatBytes($opcache['memory_usage']['used'])],
                    ['Memory Free', $this->formatBytes($opcache['memory_usage']['free'])],
                    ['Memory Usage', $opcache['memory_usage']['usage_pct'] . '%'],
                    ['Cache Hits', number_format($opcache['statistics']['hits'])],
                    ['Hit Rate', $opcache['statistics']['hit_rate'] . '%'],
                    ['Cached Scripts', number_format($opcache['statistics']['cached_scripts'])],
                    ['JIT Enabled', ($opcache['jit']['enabled'] ?? false) ? '✓ Yes' : '✗ No'],
                ]
            );
        } else {
            $this->warn('  OPcache is DISABLED - enable for 3-5x performance improvement');
        }
        $this->newLine();

        // Memory
        $memory = $diagnostics['memory'];
        $this->info('💾 Memory Usage');
        $this->table(
            ['Metric', 'Value'],
            [
                ['Current Usage', $this->formatBytes($memory['current'])],
                ['Peak Usage', $this->formatBytes($memory['peak'])],
                ['Memory Limit', $memory['limit']],
            ]
        );
        $this->newLine();

        // Queue Health
        $queues = $diagnostics['queue_health'];
        $this->info('📋 Queue Health');
        $queueRows = [];
        foreach ($queues as $name => $data) {
            $status = match ($data['status']) {
                'healthy' => '<fg=green>✓ Healthy</>',
                'warning' => '<fg=yellow>⚠ Warning</>',
                'critical' => '<fg=red>✗ Critical</>',
                default => '? Unknown',
            };
            $queueRows[] = [$name, $data['size'], $status];
        }
        $this->table(['Queue', 'Size', 'Status'], $queueRows);
        $this->newLine();

        // Connection Pool
        $pool = $diagnostics['connection_pool'];
        $this->info('🔗 Connection Pool');
        $this->line("  Active HTTP Clients: {$pool['active_clients']}");
        if (!empty($pool['circuit_breakers'])) {
            $this->line("  Circuit Breakers:");
            foreach ($pool['circuit_breakers'] as $service => $state) {
                $icon = $state['open'] ? '🔴' : '🟢';
                $this->line("    {$icon} {$service}: " . ($state['open'] ? 'OPEN' : 'CLOSED'));
            }
        }
        $this->newLine();

        // Recommendations
        $this->info('💡 Recommendations');
        $recommendations = $this->getRecommendations($diagnostics);
        foreach ($recommendations as $rec) {
            $icon = match ($rec['priority']) {
                'high' => '🔴',
                'medium' => '🟡',
                default => '🟢',
            };
            $this->line("  {$icon} {$rec['message']}");
        }
        $this->newLine();

        return 0;
    }

    private function getRecommendations(array $diagnostics): array
    {
        $recommendations = [];

        // OPcache recommendations
        if (!($diagnostics['opcache']['enabled'] ?? false)) {
            $recommendations[] = [
                'priority' => 'high',
                'message' => 'Enable OPcache for 3-5x faster PHP execution',
            ];
        } elseif (!($diagnostics['opcache']['jit']['enabled'] ?? false)) {
            $recommendations[] = [
                'priority' => 'medium',
                'message' => 'Enable JIT compilation for additional 10-30% performance boost',
            ];
        }

        // Memory recommendations
        $memoryLimit = $this->parseBytes($diagnostics['memory']['limit']);
        $peakUsage = $diagnostics['memory']['peak'];
        if ($peakUsage > $memoryLimit * 0.8) {
            $recommendations[] = [
                'priority' => 'medium',
                'message' => 'Memory usage is above 80% of limit - consider increasing memory_limit',
            ];
        }

        // Queue recommendations
        foreach ($diagnostics['queue_health'] as $queue => $data) {
            if ($data['status'] === 'critical') {
                $recommendations[] = [
                    'priority' => 'high',
                    'message' => "Queue '{$queue}' has {$data['size']} pending jobs - add more workers",
                ];
            }
        }

        if (empty($recommendations)) {
            $recommendations[] = [
                'priority' => 'low',
                'message' => 'All performance metrics look good!',
            ];
        }

        return $recommendations;
    }

    private function formatBytes(int $bytes): string
    {
        $units = ['B', 'KB', 'MB', 'GB'];
        $i = 0;
        while ($bytes >= 1024 && $i < count($units) - 1) {
            $bytes /= 1024;
            $i++;
        }
        return round($bytes, 2) . ' ' . $units[$i];
    }

    private function parseBytes(string $value): int
    {
        $value = trim($value);
        $unit = strtoupper(substr($value, -1));
        $bytes = (int) $value;

        return match ($unit) {
            'G' => $bytes * 1024 * 1024 * 1024,
            'M' => $bytes * 1024 * 1024,
            'K' => $bytes * 1024,
            default => $bytes,
        };
    }
}
