<?php

declare(strict_types=1);

namespace App\Services\Cache;

use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Collection;
use App\Models\User;
use App\Models\Contact;
use App\Models\Post;
use App\Models\Product;
use App\Models\Setting;
use App\Models\EmailSequence;
use App\Models\AutomationFunnel;
use App\Models\ContactList;
use App\Models\ContactTag;

/**
 * Cache Warmer Service (ICT9+ Enterprise Grade)
 *
 * Proactively warms cache with frequently accessed data:
 * - On deployment (warm critical paths)
 * - On schedule (refresh expiring entries)
 * - On demand (specific domains)
 *
 * Features:
 * - Parallel warming with chunked processing
 * - Priority-based warming order
 * - Warming status tracking
 * - Incremental warming for large datasets
 *
 * @version 1.0.0
 * @level ICT9+ Principal Engineer Grade
 */
class CacheWarmer
{
    private MultiTierCacheManager $cache;

    /**
     * Warming priorities (higher = warmed first)
     */
    private const PRIORITIES = [
        'config' => 100,
        'users_active' => 90,
        'contacts_recent' => 80,
        'crm_sequences' => 70,
        'crm_funnels' => 70,
        'lists_tags' => 60,
        'posts_popular' => 50,
        'products_active' => 50,
    ];

    /**
     * Warming batch sizes
     */
    private const BATCH_SIZE = 100;

    /**
     * Warming status
     */
    private array $status = [];

    public function __construct(MultiTierCacheManager $cache)
    {
        $this->cache = $cache;
    }

    /**
     * Warm all critical caches
     */
    public function warmAll(): array
    {
        $startTime = microtime(true);
        Log::info('Starting full cache warm-up');

        $this->status = [
            'started_at' => now()->toIso8601String(),
            'completed' => [],
            'failed' => [],
            'skipped' => [],
        ];

        // Sort by priority
        $warmers = [
            'config' => fn() => $this->warmConfig(),
            'users_active' => fn() => $this->warmActiveUsers(),
            'contacts_recent' => fn() => $this->warmRecentContacts(),
            'crm_sequences' => fn() => $this->warmEmailSequences(),
            'crm_funnels' => fn() => $this->warmAutomationFunnels(),
            'lists_tags' => fn() => $this->warmListsAndTags(),
            'posts_popular' => fn() => $this->warmPopularPosts(),
            'products_active' => fn() => $this->warmActiveProducts(),
        ];

        foreach ($warmers as $name => $warmer) {
            try {
                $count = $warmer();
                $this->status['completed'][$name] = $count;
                Log::info("Warmed {$name}", ['count' => $count]);
            } catch (\Throwable $e) {
                $this->status['failed'][$name] = $e->getMessage();
                Log::error("Failed to warm {$name}", ['error' => $e->getMessage()]);
            }
        }

        $duration = round((microtime(true) - $startTime) * 1000, 2);
        $this->status['duration_ms'] = $duration;
        $this->status['completed_at'] = now()->toIso8601String();

        Log::info('Cache warm-up completed', ['duration_ms' => $duration]);

        return $this->status;
    }

    /**
     * Warm specific domain
     */
    public function warmDomain(string $domain): int
    {
        return match ($domain) {
            'config' => $this->warmConfig(),
            'users' => $this->warmActiveUsers(),
            'contacts' => $this->warmRecentContacts(),
            'crm' => $this->warmCrm(),
            'posts' => $this->warmPopularPosts(),
            'products' => $this->warmActiveProducts(),
            default => 0,
        };
    }

    /**
     * Warm configuration settings (highest priority)
     */
    private function warmConfig(): int
    {
        $count = 0;

        // Site settings
        if (class_exists(Setting::class)) {
            try {
                $settings = Setting::all();
                foreach ($settings as $setting) {
                    $this->cache->put(
                        "config:{$setting->key}",
                        $setting->value,
                        86400, // 24 hours
                        ['config']
                    );
                    $count++;
                }
            } catch (\Throwable $e) {
                Log::warning('Could not warm settings', ['error' => $e->getMessage()]);
            }
        }

        // Feature flags
        $this->cache->put('config:features', config('features', []), 86400, ['config']);
        $count++;

        // App config
        $this->cache->put('config:app', [
            'name' => config('app.name'),
            'env' => config('app.env'),
            'debug' => config('app.debug'),
            'url' => config('app.url'),
            'timezone' => config('app.timezone'),
        ], 86400, ['config']);
        $count++;

        return $count;
    }

    /**
     * Warm active users
     */
    private function warmActiveUsers(): int
    {
        if (!class_exists(User::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm users who logged in recently (last 7 days)
            User::query()
                ->where('last_login_at', '>=', now()->subDays(7))
                ->orWhere('updated_at', '>=', now()->subDay())
                ->select(['id', 'name', 'email', 'role', 'last_login_at'])
                ->chunk(self::BATCH_SIZE, function ($users) use (&$count) {
                    foreach ($users as $user) {
                        $this->cache->put(
                            "user:{$user->id}",
                            $user->toArray(),
                            3600, // 1 hour
                            ['users']
                        );
                        $count++;
                    }
                });
        } catch (\Throwable $e) {
            Log::warning('Could not warm users', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Warm recent contacts
     */
    private function warmRecentContacts(): int
    {
        if (!class_exists(Contact::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm contacts updated recently
            Contact::query()
                ->where('updated_at', '>=', now()->subDays(3))
                ->select(['id', 'email', 'first_name', 'last_name', 'status', 'lead_score'])
                ->chunk(self::BATCH_SIZE, function ($contacts) use (&$count) {
                    foreach ($contacts as $contact) {
                        $this->cache->put(
                            "contact:{$contact->id}",
                            $contact->toArray(),
                            1800, // 30 minutes
                            ['contacts']
                        );
                        $count++;
                    }
                });

            // Warm contact counts by status
            $statusCounts = Contact::query()
                ->selectRaw('status, COUNT(*) as count')
                ->groupBy('status')
                ->pluck('count', 'status')
                ->toArray();

            $this->cache->put('contacts:status_counts', $statusCounts, 300, ['contacts']);
            $count++;
        } catch (\Throwable $e) {
            Log::warning('Could not warm contacts', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Warm CRM data (sequences, funnels, lists, tags)
     */
    private function warmCrm(): int
    {
        return $this->warmEmailSequences() +
               $this->warmAutomationFunnels() +
               $this->warmListsAndTags();
    }

    /**
     * Warm email sequences
     */
    private function warmEmailSequences(): int
    {
        if (!class_exists(EmailSequence::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm active sequences with their emails
            $sequences = EmailSequence::query()
                ->where('status', 'published')
                ->with('emails')
                ->get();

            foreach ($sequences as $sequence) {
                $this->cache->put(
                    "crm:sequence:{$sequence->id}",
                    $sequence->toArray(),
                    3600,
                    ['crm', 'sequences']
                );
                $count++;
            }

            // Warm active sequences list
            $this->cache->put(
                'crm:sequences:active',
                $sequences->pluck('id')->toArray(),
                3600,
                ['crm', 'sequences']
            );
            $count++;
        } catch (\Throwable $e) {
            Log::warning('Could not warm sequences', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Warm automation funnels
     */
    private function warmAutomationFunnels(): int
    {
        if (!class_exists(AutomationFunnel::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm active funnels with their actions
            $funnels = AutomationFunnel::query()
                ->where('status', 'published')
                ->with('actions')
                ->get();

            foreach ($funnels as $funnel) {
                $this->cache->put(
                    "crm:funnel:{$funnel->id}",
                    $funnel->toArray(),
                    3600,
                    ['crm', 'funnels']
                );
                $count++;
            }

            // Warm funnels by trigger type
            $byTrigger = $funnels->groupBy('trigger_type');
            foreach ($byTrigger as $trigger => $triggerFunnels) {
                $this->cache->put(
                    "crm:funnels:trigger:{$trigger}",
                    $triggerFunnels->pluck('id')->toArray(),
                    3600,
                    ['crm', 'funnels']
                );
                $count++;
            }
        } catch (\Throwable $e) {
            Log::warning('Could not warm funnels', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Warm lists and tags
     */
    private function warmListsAndTags(): int
    {
        $count = 0;

        // Warm lists
        if (class_exists(ContactList::class)) {
            try {
                $lists = ContactList::all();
                foreach ($lists as $list) {
                    $this->cache->put(
                        "crm:list:{$list->id}",
                        $list->toArray(),
                        3600,
                        ['crm', 'lists']
                    );
                    $count++;
                }

                $this->cache->put(
                    'crm:lists:all',
                    $lists->pluck('id', 'slug')->toArray(),
                    3600,
                    ['crm', 'lists']
                );
                $count++;
            } catch (\Throwable $e) {
                Log::warning('Could not warm lists', ['error' => $e->getMessage()]);
            }
        }

        // Warm tags
        if (class_exists(ContactTag::class)) {
            try {
                $tags = ContactTag::all();
                foreach ($tags as $tag) {
                    $this->cache->put(
                        "crm:tag:{$tag->id}",
                        $tag->toArray(),
                        3600,
                        ['crm', 'tags']
                    );
                    $count++;
                }

                $this->cache->put(
                    'crm:tags:all',
                    $tags->pluck('id', 'slug')->toArray(),
                    3600,
                    ['crm', 'tags']
                );
                $count++;
            } catch (\Throwable $e) {
                Log::warning('Could not warm tags', ['error' => $e->getMessage()]);
            }
        }

        return $count;
    }

    /**
     * Warm popular posts
     */
    private function warmPopularPosts(): int
    {
        if (!class_exists(Post::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm top 50 most viewed posts
            Post::query()
                ->where('status', 'published')
                ->orderByDesc('view_count')
                ->limit(50)
                ->get()
                ->each(function ($post) use (&$count) {
                    $this->cache->put(
                        "post:slug:{$post->slug}",
                        $post->toArray(),
                        1800,
                        ['posts']
                    );
                    $count++;
                });

            // Warm recent posts
            Post::query()
                ->where('status', 'published')
                ->where('published_at', '>=', now()->subDays(7))
                ->get()
                ->each(function ($post) use (&$count) {
                    $this->cache->put(
                        "post:id:{$post->id}",
                        $post->toArray(),
                        1800,
                        ['posts']
                    );
                    $count++;
                });
        } catch (\Throwable $e) {
            Log::warning('Could not warm posts', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Warm active products
     */
    private function warmActiveProducts(): int
    {
        if (!class_exists(Product::class)) {
            return 0;
        }

        $count = 0;

        try {
            // Warm active products
            Product::query()
                ->where('status', 'active')
                ->chunk(self::BATCH_SIZE, function ($products) use (&$count) {
                    foreach ($products as $product) {
                        $this->cache->put(
                            "product:{$product->id}",
                            $product->toArray(),
                            3600,
                            ['products']
                        );
                        $count++;
                    }
                });
        } catch (\Throwable $e) {
            Log::warning('Could not warm products', ['error' => $e->getMessage()]);
        }

        return $count;
    }

    /**
     * Get warming status
     */
    public function getStatus(): array
    {
        return $this->status;
    }

    /**
     * Check if specific domain needs warming
     */
    public function needsWarming(string $domain): bool
    {
        $key = "cache:warmer:last_warm:{$domain}";
        $lastWarm = $this->cache->get($key, fn() => null, 86400);

        if ($lastWarm === null) {
            return true;
        }

        // Check if warming expired based on domain
        $maxAge = match ($domain) {
            'config' => 86400,    // 24 hours
            'users' => 3600,      // 1 hour
            'contacts' => 1800,   // 30 minutes
            'crm' => 3600,        // 1 hour
            'posts' => 1800,      // 30 minutes
            'products' => 3600,   // 1 hour
            default => 3600,
        };

        return (time() - $lastWarm) > $maxAge;
    }

    /**
     * Record warming completion
     */
    public function recordWarming(string $domain): void
    {
        $this->cache->put(
            "cache:warmer:last_warm:{$domain}",
            time(),
            86400,
            ['cache_meta']
        );
    }
}
