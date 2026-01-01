<?php

declare(strict_types=1);

namespace App\Console\Commands;

use Illuminate\Console\Command;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Cache;
use App\Services\Cache\MultiTierCacheManager;

/**
 * Cache Cleanup Command
 *
 * Cleans up expired cache entries and optimizes cache storage
 *
 * Usage:
 *   php artisan cache:cleanup           - Clean expired L3 entries
 *   php artisan cache:cleanup --all     - Clean all tiers
 *   php artisan cache:cleanup --l3-only - Clean only database cache
 */
class CacheCleanupCommand extends Command
{
    protected $signature = 'cache:cleanup
                            {--all : Clean all cache tiers}
                            {--l3-only : Clean only database (L3) cache}
                            {--dry-run : Show what would be cleaned without actually cleaning}';

    protected $description = 'Clean up expired cache entries and optimize storage';

    public function handle(MultiTierCacheManager $cache): int
    {
        $isDryRun = $this->option('dry-run');

        if ($isDryRun) {
            $this->info('DRY RUN - No changes will be made');
            $this->newLine();
        }

        $totalCleaned = 0;

        // Clean L3 (database) cache
        if (!$this->option('all') || $this->option('l3-only')) {
            $l3Cleaned = $this->cleanL3Cache($isDryRun);
            $totalCleaned += $l3Cleaned;
        }

        // Clean all tiers if requested
        if ($this->option('all')) {
            $this->info('Flushing all cache tiers...');

            if (!$isDryRun) {
                $cache->flush();
                $this->info('✓ All cache tiers flushed');
            } else {
                $this->info('Would flush all cache tiers');
            }
        }

        // Optimize L3 storage
        if (!$this->option('all')) {
            $this->optimizeL3Storage($isDryRun);
        }

        $this->newLine();
        $this->info("Cleanup complete. Cleaned {$totalCleaned} entries.");

        return 0;
    }

    private function cleanL3Cache(bool $isDryRun): int
    {
        $this->info('Cleaning L3 (database) cache...');

        // Count expired entries
        $expiredCount = DB::table('cache_l3')
            ->where('expires_at', '<', now())
            ->count();

        $this->line("  Found {$expiredCount} expired entries");

        if (!$isDryRun && $expiredCount > 0) {
            // Delete in batches to avoid lock issues
            $deleted = 0;
            $batchSize = 1000;

            do {
                $affected = DB::table('cache_l3')
                    ->where('expires_at', '<', now())
                    ->limit($batchSize)
                    ->delete();

                $deleted += $affected;
                $this->output->write(".");
            } while ($affected === $batchSize);

            $this->newLine();
            $this->info("  ✓ Deleted {$deleted} expired entries");

            return $deleted;
        }

        return 0;
    }

    private function optimizeL3Storage(bool $isDryRun): void
    {
        $this->info('Optimizing L3 storage...');

        // Get storage stats
        $stats = DB::table('cache_l3')
            ->selectRaw('COUNT(*) as count, SUM(LENGTH(value)) as total_size')
            ->first();

        $this->line("  Total entries: " . number_format($stats->count ?? 0));
        $this->line("  Total size: " . $this->formatBytes($stats->total_size ?? 0));

        // Find entries not accessed in 7 days
        $staleCount = DB::table('cache_l3')
            ->where('last_accessed_at', '<', now()->subDays(7))
            ->count();

        if ($staleCount > 0) {
            $this->line("  Stale entries (not accessed in 7 days): {$staleCount}");

            if (!$isDryRun && $this->confirm("  Remove stale entries?", false)) {
                $deleted = DB::table('cache_l3')
                    ->where('last_accessed_at', '<', now()->subDays(7))
                    ->delete();

                $this->info("  ✓ Removed {$deleted} stale entries");
            }
        }

        // Run OPTIMIZE TABLE if MySQL
        if (!$isDryRun && config('database.default') === 'mysql') {
            try {
                DB::statement('OPTIMIZE TABLE cache_l3');
                $this->info('  ✓ Table optimized');
            } catch (\Throwable $e) {
                $this->warn("  Could not optimize table: " . $e->getMessage());
            }
        }
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
}
