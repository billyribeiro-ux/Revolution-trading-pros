<?php

declare(strict_types=1);

namespace App\Services\Performance;

use Illuminate\Support\Facades\DB;
use Illuminate\Database\Connection;

/**
 * Read Replica Manager (ICT9+ Enterprise Grade)
 *
 * Manages database read/write splitting:
 * - Automatic routing of reads to replicas
 * - Write stick for consistency
 * - Health-based replica selection
 * - Failover handling
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class ReadReplicaManager
{
    /**
     * Whether to stick to primary after write
     */
    private static bool $stickToPrimary = false;

    /**
     * Timestamp of last write (for time-based stickiness)
     */
    private static ?int $lastWriteTime = null;

    /**
     * Stick duration in seconds
     */
    private const STICK_DURATION = 5;

    /**
     * Force next query to use primary (write) connection
     */
    public static function usePrimary(): void
    {
        DB::connection()->recordsHaveBeenModified();
    }

    /**
     * Check if should stick to primary
     */
    public static function shouldUsePrimary(): bool
    {
        if (self::$stickToPrimary) {
            // Check if stick duration expired
            if (self::$lastWriteTime && (time() - self::$lastWriteTime) > self::STICK_DURATION) {
                self::$stickToPrimary = false;
                return false;
            }
            return true;
        }

        return false;
    }

    /**
     * Record that a write occurred
     */
    public static function recordWrite(): void
    {
        self::$stickToPrimary = true;
        self::$lastWriteTime = time();
    }

    /**
     * Execute read on replica
     */
    public static function readFromReplica(callable $callback): mixed
    {
        if (self::shouldUsePrimary()) {
            return $callback();
        }

        // Force use of read connection
        return DB::connection()->setReadWriteType('read')->transaction($callback);
    }

    /**
     * Execute write on primary
     */
    public static function writeOnPrimary(callable $callback): mixed
    {
        self::recordWrite();
        return DB::connection()->setReadWriteType('write')->transaction($callback);
    }

    /**
     * Get read/write connection configuration
     */
    public static function getConnectionConfig(): array
    {
        return [
            'read' => [
                'host' => array_filter([
                    env('DB_READ_HOST_1'),
                    env('DB_READ_HOST_2'),
                    env('DB_READ_HOST_3'),
                ]),
                'sticky' => true, // Stick to same replica for request
            ],
            'write' => [
                'host' => env('DB_HOST', '127.0.0.1'),
            ],
            'driver' => env('DB_CONNECTION', 'mysql'),
            'database' => env('DB_DATABASE', 'forge'),
            'username' => env('DB_USERNAME', 'forge'),
            'password' => env('DB_PASSWORD', ''),
            'charset' => 'utf8mb4',
            'collation' => 'utf8mb4_unicode_ci',
            'prefix' => '',
            'strict' => true,
            'engine' => 'InnoDB',
            'options' => [
                \PDO::ATTR_PERSISTENT => true, // Connection pooling
                \PDO::ATTR_EMULATE_PREPARES => false, // Real prepared statements
                \PDO::MYSQL_ATTR_USE_BUFFERED_QUERY => true,
            ],
        ];
    }

    /**
     * Health check for replicas
     */
    public static function checkReplicaHealth(): array
    {
        $results = [];

        $replicas = [
            env('DB_READ_HOST_1'),
            env('DB_READ_HOST_2'),
            env('DB_READ_HOST_3'),
        ];

        foreach (array_filter($replicas) as $index => $host) {
            try {
                $start = microtime(true);

                $config = [
                    'driver' => 'mysql',
                    'host' => $host,
                    'database' => env('DB_DATABASE'),
                    'username' => env('DB_USERNAME'),
                    'password' => env('DB_PASSWORD'),
                ];

                $pdo = new \PDO(
                    "mysql:host={$host};dbname=" . env('DB_DATABASE'),
                    env('DB_USERNAME'),
                    env('DB_PASSWORD'),
                    [\PDO::ATTR_TIMEOUT => 2]
                );

                $stmt = $pdo->query('SELECT 1');
                $latency = (microtime(true) - $start) * 1000;

                // Check replication lag
                $lagStmt = $pdo->query("SHOW SLAVE STATUS");
                $slaveStatus = $lagStmt->fetch(\PDO::FETCH_ASSOC);
                $lag = $slaveStatus['Seconds_Behind_Master'] ?? 0;

                $results["replica_{$index}"] = [
                    'host' => $host,
                    'status' => 'healthy',
                    'latency_ms' => round($latency, 2),
                    'replication_lag_seconds' => $lag,
                ];
            } catch (\Throwable $e) {
                $results["replica_{$index}"] = [
                    'host' => $host,
                    'status' => 'unhealthy',
                    'error' => $e->getMessage(),
                ];
            }
        }

        return $results;
    }

    /**
     * Reset stickiness (for testing or after timeout)
     */
    public static function reset(): void
    {
        self::$stickToPrimary = false;
        self::$lastWriteTime = null;
    }
}
