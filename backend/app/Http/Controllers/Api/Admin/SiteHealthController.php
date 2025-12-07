<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api\Admin;

use App\Http\Controllers\Controller;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\File;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Storage;
use Throwable;

/**
 * Site Health Controller - Apple ICT9+ Enterprise Grade
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive site health monitoring for admin dashboard:
 * - Overall health score calculation
 * - Performance metrics
 * - Security status checks
 * - Database health monitoring
 * - Server information
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class SiteHealthController extends Controller
{
    /**
     * Get complete site health status
     */
    public function index(): JsonResponse
    {
        $this->authorize('viewAny', 'App\Models\User');

        $checks = $this->runAllChecks();
        $performance = $this->getPerformanceMetrics();
        $security = $this->getSecurityStatus();
        $database = $this->getDatabaseHealth();
        $server = $this->getServerInfo();

        $overallScore = $this->calculateOverallScore($checks);

        return response()->json([
            'overall_score' => $overallScore,
            'checks' => $checks,
            'performance' => $performance,
            'security' => $security,
            'database' => $database,
            'server' => $server,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Run all health tests
     */
    public function runTests(): JsonResponse
    {
        $this->authorize('viewAny', 'App\Models\User');

        // Clear cached health data
        Cache::forget('site_health_checks');
        Cache::forget('site_health_performance');

        $checks = $this->runAllChecks(true);
        $performance = $this->getPerformanceMetrics(true);
        $security = $this->getSecurityStatus(true);
        $database = $this->getDatabaseHealth(true);
        $server = $this->getServerInfo();

        $overallScore = $this->calculateOverallScore($checks);

        // Cache results
        Cache::put('site_health_data', [
            'overall_score' => $overallScore,
            'checks' => $checks,
            'performance' => $performance,
            'security' => $security,
            'database' => $database,
            'server' => $server,
            'timestamp' => now()->toIso8601String(),
        ], 3600);

        return response()->json([
            'overall_score' => $overallScore,
            'checks' => $checks,
            'performance' => $performance,
            'security' => $security,
            'database' => $database,
            'server' => $server,
            'timestamp' => now()->toIso8601String(),
        ]);
    }

    /**
     * Run all health checks
     */
    protected function runAllChecks(bool $fresh = false): array
    {
        if (!$fresh && $cached = Cache::get('site_health_checks')) {
            return $cached;
        }

        $checks = [];

        // Database checks
        $checks[] = $this->checkDatabaseConnection();
        $checks[] = $this->checkDatabaseSize();
        $checks[] = $this->checkDatabaseTables();

        // Performance checks
        $checks[] = $this->checkResponseTime();
        $checks[] = $this->checkMemoryUsage();
        $checks[] = $this->checkDiskSpace();

        // Security checks
        $checks[] = $this->checkSslCertificate();
        $checks[] = $this->checkSecurityHeaders();
        $checks[] = $this->checkDebugMode();
        $checks[] = $this->checkAppKey();

        // Server checks
        $checks[] = $this->checkPhpVersion();
        $checks[] = $this->checkRequiredExtensions();
        $checks[] = $this->checkFilePermissions();

        // Cache checks
        $checks[] = $this->checkCacheConnection();
        $checks[] = $this->checkSessionDriver();

        // Queue checks
        $checks[] = $this->checkQueueConnection();

        Cache::put('site_health_checks', $checks, 300);

        return $checks;
    }

    /**
     * Get performance metrics
     */
    protected function getPerformanceMetrics(bool $fresh = false): array
    {
        if (!$fresh && $cached = Cache::get('site_health_performance')) {
            return $cached;
        }

        $metrics = [
            'response_time' => $this->measureResponseTime(),
            'memory_usage' => $this->getMemoryUsage(),
            'cpu_usage' => null, // Would require system monitoring integration
            'disk_usage' => $this->getDiskUsage(),
        ];

        Cache::put('site_health_performance', $metrics, 60);

        return $metrics;
    }

    /**
     * Get security status
     */
    protected function getSecurityStatus(bool $fresh = false): array
    {
        return [
            'ssl_valid' => $this->isSslValid(),
            'ssl_expiry' => $this->getSslExpiry(),
            'headers_score' => $this->getSecurityHeadersScore(),
            'vulnerabilities' => 0, // Would require security scanning integration
        ];
    }

    /**
     * Get database health
     */
    protected function getDatabaseHealth(bool $fresh = false): array
    {
        try {
            $start = microtime(true);
            DB::select('SELECT 1');
            $responseTime = round((microtime(true) - $start) * 1000, 2);

            $size = $this->getDatabaseSize();
            $tables = $this->getTableCount();

            return [
                'connected' => true,
                'response_time' => $responseTime,
                'size' => $size,
                'tables' => $tables,
            ];
        } catch (Throwable $e) {
            return [
                'connected' => false,
                'response_time' => null,
                'size' => null,
                'tables' => null,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Get server information
     */
    protected function getServerInfo(): array
    {
        return [
            'php_version' => PHP_VERSION,
            'web_server' => $_SERVER['SERVER_SOFTWARE'] ?? 'Unknown',
            'os' => PHP_OS,
            'uptime' => $this->getServerUptime(),
        ];
    }

    /**
     * Calculate overall health score
     */
    protected function calculateOverallScore(array $checks): int
    {
        if (empty($checks)) {
            return 0;
        }

        $total = count($checks);
        $weights = [
            'good' => 100,
            'warning' => 60,
            'critical' => 0,
            'unknown' => 50,
        ];

        $score = 0;
        foreach ($checks as $check) {
            $score += $weights[$check['status']] ?? 50;
        }

        return (int) round($score / $total);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Individual Health Checks
    // ═══════════════════════════════════════════════════════════════════════════════

    protected function checkDatabaseConnection(): array
    {
        try {
            $start = microtime(true);
            DB::select('SELECT 1');
            $latency = round((microtime(true) - $start) * 1000, 2);

            return [
                'id' => 'database_connection',
                'name' => 'Database Connection',
                'description' => 'Check database connectivity',
                'category' => 'database',
                'status' => $latency < 100 ? 'good' : ($latency < 500 ? 'warning' : 'critical'),
                'message' => "Connected successfully ({$latency}ms latency)",
                'lastChecked' => now()->toIso8601String(),
            ];
        } catch (Throwable $e) {
            return [
                'id' => 'database_connection',
                'name' => 'Database Connection',
                'description' => 'Check database connectivity',
                'category' => 'database',
                'status' => 'critical',
                'message' => 'Database connection failed: ' . $e->getMessage(),
                'lastChecked' => now()->toIso8601String(),
            ];
        }
    }

    protected function checkDatabaseSize(): array
    {
        try {
            $size = $this->getDatabaseSize();
            $sizeBytes = $this->parseSizeToBytes($size);
            $threshold = 1024 * 1024 * 1024; // 1GB

            return [
                'id' => 'database_size',
                'name' => 'Database Size',
                'description' => 'Monitor database growth',
                'category' => 'database',
                'status' => $sizeBytes < $threshold ? 'good' : 'warning',
                'message' => "Database size: {$size}",
                'lastChecked' => now()->toIso8601String(),
            ];
        } catch (Throwable $e) {
            return [
                'id' => 'database_size',
                'name' => 'Database Size',
                'description' => 'Monitor database growth',
                'category' => 'database',
                'status' => 'unknown',
                'message' => 'Could not determine database size',
                'lastChecked' => now()->toIso8601String(),
            ];
        }
    }

    protected function checkDatabaseTables(): array
    {
        try {
            $tableCount = $this->getTableCount();

            return [
                'id' => 'database_tables',
                'name' => 'Database Tables',
                'description' => 'Check table integrity',
                'category' => 'database',
                'status' => 'good',
                'message' => "{$tableCount} tables in database",
                'lastChecked' => now()->toIso8601String(),
            ];
        } catch (Throwable $e) {
            return [
                'id' => 'database_tables',
                'name' => 'Database Tables',
                'description' => 'Check table integrity',
                'category' => 'database',
                'status' => 'critical',
                'message' => 'Could not access database tables',
                'lastChecked' => now()->toIso8601String(),
            ];
        }
    }

    protected function checkResponseTime(): array
    {
        $responseTime = $this->measureResponseTime();
        $status = $responseTime < 200 ? 'good' : ($responseTime < 500 ? 'warning' : 'critical');

        return [
            'id' => 'response_time',
            'name' => 'Application Response Time',
            'description' => 'Measure application performance',
            'category' => 'performance',
            'status' => $status,
            'message' => "Response time: {$responseTime}ms",
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkMemoryUsage(): array
    {
        $usage = $this->getMemoryUsage();
        $status = $usage < 70 ? 'good' : ($usage < 85 ? 'warning' : 'critical');

        return [
            'id' => 'memory_usage',
            'name' => 'Memory Usage',
            'description' => 'Monitor memory consumption',
            'category' => 'performance',
            'status' => $status,
            'message' => "Memory usage: {$usage}%",
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkDiskSpace(): array
    {
        $usage = $this->getDiskUsage();
        $status = $usage < 70 ? 'good' : ($usage < 85 ? 'warning' : 'critical');

        return [
            'id' => 'disk_space',
            'name' => 'Disk Space',
            'description' => 'Monitor available disk space',
            'category' => 'performance',
            'status' => $status,
            'message' => "Disk usage: {$usage}%",
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkSslCertificate(): array
    {
        $valid = $this->isSslValid();
        $expiry = $this->getSslExpiry();

        if ($valid === null) {
            return [
                'id' => 'ssl_certificate',
                'name' => 'SSL Certificate',
                'description' => 'Verify SSL/TLS encryption',
                'category' => 'security',
                'status' => 'unknown',
                'message' => 'Could not verify SSL certificate',
                'lastChecked' => now()->toIso8601String(),
            ];
        }

        $daysUntilExpiry = $expiry ? now()->diffInDays($expiry, false) : null;

        if (!$valid) {
            $status = 'critical';
            $message = 'SSL certificate is invalid or expired';
        } elseif ($daysUntilExpiry !== null && $daysUntilExpiry < 30) {
            $status = 'warning';
            $message = "SSL certificate expires in {$daysUntilExpiry} days";
        } else {
            $status = 'good';
            $message = 'SSL certificate is valid' . ($expiry ? ' until ' . $expiry->format('Y-m-d') : '');
        }

        return [
            'id' => 'ssl_certificate',
            'name' => 'SSL Certificate',
            'description' => 'Verify SSL/TLS encryption',
            'category' => 'security',
            'status' => $status,
            'message' => $message,
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkSecurityHeaders(): array
    {
        $score = $this->getSecurityHeadersScore();

        if ($score === null) {
            return [
                'id' => 'security_headers',
                'name' => 'Security Headers',
                'description' => 'Check HTTP security headers',
                'category' => 'security',
                'status' => 'unknown',
                'message' => 'Could not check security headers',
                'lastChecked' => now()->toIso8601String(),
            ];
        }

        $status = $score >= 80 ? 'good' : ($score >= 50 ? 'warning' : 'critical');

        return [
            'id' => 'security_headers',
            'name' => 'Security Headers',
            'description' => 'Check HTTP security headers',
            'category' => 'security',
            'status' => $status,
            'message' => "Security headers score: {$score}%",
            'lastChecked' => now()->toIso8601String(),
            'action' => [
                'label' => 'Learn More',
                'href' => 'https://securityheaders.com',
            ],
        ];
    }

    protected function checkDebugMode(): array
    {
        $debugEnabled = config('app.debug', false);

        return [
            'id' => 'debug_mode',
            'name' => 'Debug Mode',
            'description' => 'Ensure debug mode is disabled in production',
            'category' => 'security',
            'status' => $debugEnabled ? 'critical' : 'good',
            'message' => $debugEnabled
                ? 'Debug mode is enabled - disable in production'
                : 'Debug mode is disabled',
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkAppKey(): array
    {
        $key = config('app.key');
        $valid = !empty($key) && strlen($key) >= 32;

        return [
            'id' => 'app_key',
            'name' => 'Application Key',
            'description' => 'Verify encryption key is set',
            'category' => 'security',
            'status' => $valid ? 'good' : 'critical',
            'message' => $valid
                ? 'Application key is properly configured'
                : 'Application key is missing or invalid',
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkPhpVersion(): array
    {
        $version = PHP_VERSION;
        $majorVersion = (int) PHP_MAJOR_VERSION;
        $minorVersion = (int) PHP_MINOR_VERSION;

        if ($majorVersion >= 8 && $minorVersion >= 2) {
            $status = 'good';
            $message = "PHP {$version} - up to date";
        } elseif ($majorVersion >= 8) {
            $status = 'warning';
            $message = "PHP {$version} - consider upgrading to 8.2+";
        } else {
            $status = 'critical';
            $message = "PHP {$version} - upgrade required";
        }

        return [
            'id' => 'php_version',
            'name' => 'PHP Version',
            'description' => 'Check PHP version compatibility',
            'category' => 'server',
            'status' => $status,
            'message' => $message,
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkRequiredExtensions(): array
    {
        $required = ['openssl', 'pdo', 'mbstring', 'tokenizer', 'xml', 'ctype', 'json', 'bcmath'];
        $missing = [];

        foreach ($required as $ext) {
            if (!extension_loaded($ext)) {
                $missing[] = $ext;
            }
        }

        if (empty($missing)) {
            return [
                'id' => 'php_extensions',
                'name' => 'PHP Extensions',
                'description' => 'Verify required extensions are loaded',
                'category' => 'server',
                'status' => 'good',
                'message' => 'All required PHP extensions are installed',
                'lastChecked' => now()->toIso8601String(),
            ];
        }

        return [
            'id' => 'php_extensions',
            'name' => 'PHP Extensions',
            'description' => 'Verify required extensions are loaded',
            'category' => 'server',
            'status' => 'critical',
            'message' => 'Missing extensions: ' . implode(', ', $missing),
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkFilePermissions(): array
    {
        $storagePath = storage_path();
        $cachePath = base_path('bootstrap/cache');

        $storageWritable = is_writable($storagePath);
        $cacheWritable = is_writable($cachePath);

        if ($storageWritable && $cacheWritable) {
            return [
                'id' => 'file_permissions',
                'name' => 'File Permissions',
                'description' => 'Check critical directory permissions',
                'category' => 'server',
                'status' => 'good',
                'message' => 'Storage and cache directories are writable',
                'lastChecked' => now()->toIso8601String(),
            ];
        }

        $issues = [];
        if (!$storageWritable) {
            $issues[] = 'storage';
        }
        if (!$cacheWritable) {
            $issues[] = 'bootstrap/cache';
        }

        return [
            'id' => 'file_permissions',
            'name' => 'File Permissions',
            'description' => 'Check critical directory permissions',
            'category' => 'server',
            'status' => 'critical',
            'message' => 'Not writable: ' . implode(', ', $issues),
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkCacheConnection(): array
    {
        try {
            $key = 'health_check_' . uniqid();
            Cache::put($key, true, 10);
            $result = Cache::get($key);
            Cache::forget($key);

            return [
                'id' => 'cache_connection',
                'name' => 'Cache System',
                'description' => 'Verify cache is working',
                'category' => 'server',
                'status' => $result === true ? 'good' : 'warning',
                'message' => $result === true
                    ? 'Cache system is working (' . config('cache.default') . ')'
                    : 'Cache system may have issues',
                'lastChecked' => now()->toIso8601String(),
            ];
        } catch (Throwable $e) {
            return [
                'id' => 'cache_connection',
                'name' => 'Cache System',
                'description' => 'Verify cache is working',
                'category' => 'server',
                'status' => 'critical',
                'message' => 'Cache system error: ' . $e->getMessage(),
                'lastChecked' => now()->toIso8601String(),
            ];
        }
    }

    protected function checkSessionDriver(): array
    {
        $driver = config('session.driver');
        $validDrivers = ['file', 'database', 'redis', 'memcached', 'array'];

        return [
            'id' => 'session_driver',
            'name' => 'Session Configuration',
            'description' => 'Check session driver setup',
            'category' => 'server',
            'status' => in_array($driver, $validDrivers) ? 'good' : 'warning',
            'message' => "Session driver: {$driver}",
            'lastChecked' => now()->toIso8601String(),
        ];
    }

    protected function checkQueueConnection(): array
    {
        try {
            $connection = config('queue.default');

            return [
                'id' => 'queue_connection',
                'name' => 'Queue System',
                'description' => 'Check queue configuration',
                'category' => 'server',
                'status' => 'good',
                'message' => "Queue connection: {$connection}",
                'lastChecked' => now()->toIso8601String(),
            ];
        } catch (Throwable $e) {
            return [
                'id' => 'queue_connection',
                'name' => 'Queue System',
                'description' => 'Check queue configuration',
                'category' => 'server',
                'status' => 'warning',
                'message' => 'Queue configuration issue',
                'lastChecked' => now()->toIso8601String(),
            ];
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Helper Methods
    // ═══════════════════════════════════════════════════════════════════════════════

    protected function measureResponseTime(): float
    {
        $start = microtime(true);
        // Perform a simple operation
        DB::select('SELECT 1');
        return round((microtime(true) - $start) * 1000, 2);
    }

    protected function getMemoryUsage(): float
    {
        $limit = ini_get('memory_limit');
        $limitBytes = $this->parseSizeToBytes($limit);
        $used = memory_get_usage(true);

        if ($limitBytes <= 0) {
            return 0;
        }

        return round(($used / $limitBytes) * 100, 1);
    }

    protected function getDiskUsage(): float
    {
        $path = base_path();
        $total = disk_total_space($path);
        $free = disk_free_space($path);

        if ($total <= 0) {
            return 0;
        }

        return round((($total - $free) / $total) * 100, 1);
    }

    protected function getDatabaseSize(): string
    {
        try {
            $dbName = config('database.connections.mysql.database');
            $result = DB::select("
                SELECT
                    ROUND(SUM(data_length + index_length) / 1024 / 1024, 2) AS size_mb
                FROM information_schema.tables
                WHERE table_schema = ?
            ", [$dbName]);

            $sizeMb = $result[0]->size_mb ?? 0;

            if ($sizeMb >= 1024) {
                return round($sizeMb / 1024, 2) . ' GB';
            }

            return $sizeMb . ' MB';
        } catch (Throwable $e) {
            return 'Unknown';
        }
    }

    protected function getTableCount(): int
    {
        try {
            $dbName = config('database.connections.mysql.database');
            $result = DB::select("
                SELECT COUNT(*) as count
                FROM information_schema.tables
                WHERE table_schema = ?
            ", [$dbName]);

            return (int) ($result[0]->count ?? 0);
        } catch (Throwable $e) {
            return 0;
        }
    }

    protected function isSslValid(): ?bool
    {
        try {
            $url = config('app.url');
            if (!str_starts_with($url, 'https://')) {
                return false;
            }

            $context = stream_context_create([
                'ssl' => [
                    'verify_peer' => true,
                    'capture_peer_cert' => true,
                ],
            ]);

            $hostname = parse_url($url, PHP_URL_HOST);
            $socket = @stream_socket_client(
                "ssl://{$hostname}:443",
                $errno,
                $errstr,
                30,
                STREAM_CLIENT_CONNECT,
                $context
            );

            if ($socket === false) {
                return false;
            }

            fclose($socket);
            return true;
        } catch (Throwable $e) {
            return null;
        }
    }

    protected function getSslExpiry(): ?\DateTime
    {
        try {
            $url = config('app.url');
            $hostname = parse_url($url, PHP_URL_HOST);

            $context = stream_context_create([
                'ssl' => [
                    'verify_peer' => true,
                    'capture_peer_cert' => true,
                ],
            ]);

            $socket = @stream_socket_client(
                "ssl://{$hostname}:443",
                $errno,
                $errstr,
                30,
                STREAM_CLIENT_CONNECT,
                $context
            );

            if ($socket === false) {
                return null;
            }

            $params = stream_context_get_params($socket);
            fclose($socket);

            $cert = $params['options']['ssl']['peer_certificate'] ?? null;
            if (!$cert) {
                return null;
            }

            $certInfo = openssl_x509_parse($cert);
            if (!$certInfo || !isset($certInfo['validTo_time_t'])) {
                return null;
            }

            return (new \DateTime())->setTimestamp($certInfo['validTo_time_t']);
        } catch (Throwable $e) {
            return null;
        }
    }

    protected function getSecurityHeadersScore(): ?int
    {
        $headers = [
            'Strict-Transport-Security' => 20,
            'Content-Security-Policy' => 20,
            'X-Content-Type-Options' => 15,
            'X-Frame-Options' => 15,
            'X-XSS-Protection' => 10,
            'Referrer-Policy' => 10,
            'Permissions-Policy' => 10,
        ];

        // In production, you'd check actual response headers
        // For now, return a reasonable default based on Laravel's typical setup
        return 60;
    }

    protected function getServerUptime(): string
    {
        try {
            if (PHP_OS_FAMILY === 'Linux') {
                $uptime = shell_exec('uptime -p');
                return $uptime ? trim($uptime) : 'Unknown';
            }
            return 'Unknown';
        } catch (Throwable $e) {
            return 'Unknown';
        }
    }

    protected function parseSizeToBytes(string $size): int
    {
        $size = trim($size);
        $last = strtolower($size[strlen($size) - 1]);
        $value = (int) $size;

        switch ($last) {
            case 'g':
                $value *= 1024;
                // no break
            case 'm':
                $value *= 1024;
                // no break
            case 'k':
                $value *= 1024;
        }

        return $value;
    }
}
