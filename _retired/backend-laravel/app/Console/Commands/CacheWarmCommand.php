<?php

declare(strict_types=1);

namespace App\Console\Commands;

use Illuminate\Console\Command;
use App\Services\Cache\CacheWarmer;
use App\Services\Cache\MultiTierCacheManager;

/**
 * Cache Warm Command
 *
 * Warms cache with frequently accessed data
 *
 * Usage:
 *   php artisan cache:warm           - Warm all caches
 *   php artisan cache:warm --domain=crm  - Warm specific domain
 */
class CacheWarmCommand extends Command
{
    protected $signature = 'cache:warm
                            {--domain= : Specific domain to warm (config, users, contacts, crm, posts, products)}
                            {--force : Force warming even if recently warmed}';

    protected $description = 'Warm cache with frequently accessed data';

    public function handle(MultiTierCacheManager $cache): int
    {
        $warmer = new CacheWarmer($cache);

        $domain = $this->option('domain');

        if ($domain) {
            $this->info("Warming {$domain} cache...");

            if (!$this->option('force') && !$warmer->needsWarming($domain)) {
                $this->info("{$domain} cache is still fresh. Use --force to warm anyway.");
                return 0;
            }

            $count = $warmer->warmDomain($domain);
            $warmer->recordWarming($domain);

            $this->info("Warmed {$count} entries for {$domain}");
            return 0;
        }

        $this->info('Starting full cache warm-up...');
        $this->newLine();

        $status = $warmer->warmAll();

        // Display results
        $this->info('Completed:');
        foreach ($status['completed'] as $domain => $count) {
            $this->line("  ✓ {$domain}: {$count} entries");
        }

        if (!empty($status['failed'])) {
            $this->newLine();
            $this->warn('Failed:');
            foreach ($status['failed'] as $domain => $error) {
                $this->line("  ✗ {$domain}: {$error}");
            }
        }

        $this->newLine();
        $this->info("Total duration: {$status['duration_ms']}ms");

        return empty($status['failed']) ? 0 : 1;
    }
}
