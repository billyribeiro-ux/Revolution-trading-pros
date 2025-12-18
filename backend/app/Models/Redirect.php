<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use InvalidArgumentException;
use Carbon\Carbon;

/**
 * Redirect Model
 * 
 * Manages URL redirects for SEO, site restructuring, and link preservation.
 * Tracks redirect performance, validates redirect chains, and prevents loops.
 *
 * @property int $id
 * @property string $from_path Source path (e.g., /old-page)
 * @property string $to_path Destination path (e.g., /new-page)
 * @property int $status_code HTTP status code (301, 302, 307, 308)
 * @property string $type Redirect type (permanent, temporary, seo, marketing, maintenance)
 * @property bool $is_active Whether redirect is active
 * @property int $hit_count Total redirect hits
 * @property \Illuminate\Support\Carbon|null $last_hit_at Last redirect timestamp
 * @property string|null $description Human-readable description
 * @property array $metadata Additional redirect data
 * @property bool $preserve_query_string Whether to preserve query parameters
 * @property bool $case_sensitive Whether path matching is case-sensitive
 * @property string|null $regex_pattern Optional regex pattern for matching
 * @property int $priority Priority for matching order (higher = first)
 * @property \Illuminate\Support\Carbon|null $expires_at Optional expiration date
 * @property string|null $created_by User who created redirect
 * @property string|null $updated_by User who last updated redirect
 * @property int|null $parent_redirect_id Parent redirect (for chains)
 * @property int $chain_length Length of redirect chain
 * @property bool $has_loop Whether redirect creates a loop
 * @property float|null $avg_response_time Average response time in ms
 * @property int $conversion_count Conversions attributed to this redirect
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 */
class Redirect extends Model
{
    use HasFactory;

    /**
     * HTTP status codes
     */
    public const STATUS_301_PERMANENT = 301;
    public const STATUS_302_FOUND = 302;
    public const STATUS_307_TEMPORARY = 307;
    public const STATUS_308_PERMANENT = 308;

    /**
     * Redirect types
     */
    public const TYPE_PERMANENT = 'permanent';
    public const TYPE_TEMPORARY = 'temporary';
    public const TYPE_SEO = 'seo';
    public const TYPE_MARKETING = 'marketing';
    public const TYPE_MAINTENANCE = 'maintenance';
    public const TYPE_ALIAS = 'alias';
    public const TYPE_CANONICAL = 'canonical';
    public const TYPE_WILDCARD = 'wildcard';

    /**
     * Valid status codes
     */
    public const VALID_STATUS_CODES = [
        self::STATUS_301_PERMANENT,
        self::STATUS_302_FOUND,
        self::STATUS_307_TEMPORARY,
        self::STATUS_308_PERMANENT,
    ];

    /**
     * Valid redirect types
     */
    public const VALID_TYPES = [
        self::TYPE_PERMANENT,
        self::TYPE_TEMPORARY,
        self::TYPE_SEO,
        self::TYPE_MARKETING,
        self::TYPE_MAINTENANCE,
        self::TYPE_ALIAS,
        self::TYPE_CANONICAL,
        self::TYPE_WILDCARD,
    ];

    /**
     * Redirect type to status code mapping
     */
    protected const TYPE_STATUS_MAP = [
        self::TYPE_PERMANENT => self::STATUS_301_PERMANENT,
        self::TYPE_TEMPORARY => self::STATUS_302_FOUND,
        self::TYPE_SEO => self::STATUS_301_PERMANENT,
        self::TYPE_MARKETING => self::STATUS_302_FOUND,
        self::TYPE_MAINTENANCE => self::STATUS_307_TEMPORARY,
        self::TYPE_ALIAS => self::STATUS_301_PERMANENT,
        self::TYPE_CANONICAL => self::STATUS_301_PERMANENT,
        self::TYPE_WILDCARD => self::STATUS_301_PERMANENT,
    ];

    /**
     * Maximum chain length before warning
     */
    public const MAX_SAFE_CHAIN_LENGTH = 3;

    /**
     * Cache duration for redirect lookups
     */
    public const CACHE_TTL = 3600; // 1 hour

    protected $fillable = [
        'from_path',
        'to_path',
        'status_code',
        'type',
        'is_active',
        'hit_count',
        'last_hit_at',
        'description',
        'metadata',
        'preserve_query_string',
        'case_sensitive',
        'regex_pattern',
        'priority',
        'expires_at',
        'created_by',
        'updated_by',
        'parent_redirect_id',
        'chain_length',
        'has_loop',
        'avg_response_time',
        'conversion_count',
    ];

    protected $casts = [
        'is_active' => 'boolean',
        'hit_count' => 'integer',
        'status_code' => 'integer',
        'last_hit_at' => 'datetime',
        'metadata' => 'array',
        'preserve_query_string' => 'boolean',
        'case_sensitive' => 'boolean',
        'priority' => 'integer',
        'expires_at' => 'datetime',
        'parent_redirect_id' => 'integer',
        'chain_length' => 'integer',
        'has_loop' => 'boolean',
        'avg_response_time' => 'float',
        'conversion_count' => 'integer',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'status_code' => self::STATUS_301_PERMANENT,
        'type' => self::TYPE_PERMANENT,
        'is_active' => true,
        'hit_count' => 0,
        'preserve_query_string' => true,
        'case_sensitive' => false,
        'priority' => 0,
        'chain_length' => 0,
        'has_loop' => false,
        'conversion_count' => 0,
        'metadata' => '[]',
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $model): void {
            $model->normalizePaths();
            $model->validateStatusCode();
            $model->validateType();
            $model->autoSetStatusCode();
            $model->detectRedirectLoop();
            $model->calculateChainLength();
        });

        static::updating(function (self $model): void {
            $model->normalizePaths();
            $model->validateStatusCode();
            $model->validateType();
            
            if ($model->isDirty(['from_path', 'to_path'])) {
                $model->detectRedirectLoop();
                $model->calculateChainLength();
            }
        });

        static::saved(function (self $model): void {
            $model->clearRedirectCache();
        });

        static::deleted(function (self $model): void {
            $model->clearRedirectCache();
        });
    }

    /**
     * Normalize paths to standard format
     */
    protected function normalizePaths(): void
    {
        // Ensure paths start with /
        if (!str_starts_with($this->from_path, '/')) {
            $this->from_path = '/' . $this->from_path;
        }

        if (!str_starts_with($this->to_path, '/') && !$this->isExternalUrl($this->to_path)) {
            $this->to_path = '/' . $this->to_path;
        }

        // Remove trailing slashes (except root)
        if ($this->from_path !== '/') {
            $this->from_path = rtrim($this->from_path, '/');
        }

        if ($this->to_path !== '/' && !$this->isExternalUrl($this->to_path)) {
            $this->to_path = rtrim($this->to_path, '/');
        }

        // Convert to lowercase if not case-sensitive
        if (!$this->case_sensitive) {
            $this->from_path = strtolower($this->from_path);
        }
    }

    /**
     * Validate status code
     */
    protected function validateStatusCode(): void
    {
        if (!in_array($this->status_code, self::VALID_STATUS_CODES, true)) {
            throw new InvalidArgumentException(
                sprintf(
                    'Invalid redirect status code: %d. Valid codes: %s',
                    $this->status_code,
                    implode(', ', self::VALID_STATUS_CODES)
                )
            );
        }
    }

    /**
     * Validate redirect type
     */
    protected function validateType(): void
    {
        if (!in_array($this->type, self::VALID_TYPES, true)) {
            throw new InvalidArgumentException(
                sprintf(
                    'Invalid redirect type: %s. Valid types: %s',
                    $this->type,
                    implode(', ', self::VALID_TYPES)
                )
            );
        }
    }

    /**
     * Auto-set status code based on type if not explicitly set
     */
    protected function autoSetStatusCode(): void
    {
        if ($this->isDirty('type') && !$this->isDirty('status_code')) {
            $this->status_code = self::TYPE_STATUS_MAP[$this->type] ?? self::STATUS_301_PERMANENT;
        }
    }

    /**
     * Detect if redirect creates a loop
     */
    protected function detectRedirectLoop(): void
    {
        if ($this->isExternalUrl($this->to_path)) {
            $this->has_loop = false;
            return;
        }

        $visited = [$this->from_path];
        $current = $this->to_path;
        $maxIterations = 10;
        $iterations = 0;

        while ($iterations < $maxIterations) {
            if (in_array($current, $visited, true)) {
                $this->has_loop = true;
                return;
            }

            $nextRedirect = static::where('from_path', $current)
                ->where('is_active', true)
                ->where('id', '!=', $this->id ?? 0)
                ->first();

            if (!$nextRedirect) {
                break;
            }

            $visited[] = $current;
            $current = $nextRedirect->to_path;
            $iterations++;
        }

        $this->has_loop = false;
    }

    /**
     * Calculate redirect chain length
     */
    protected function calculateChainLength(): void
    {
        if ($this->isExternalUrl($this->to_path)) {
            $this->chain_length = 0;
            return;
        }

        $length = 0;
        $current = $this->to_path;
        $maxDepth = 10;

        while ($length < $maxDepth) {
            $nextRedirect = static::where('from_path', $current)
                ->where('is_active', true)
                ->where('id', '!=', $this->id ?? 0)
                ->first();

            if (!$nextRedirect) {
                break;
            }

            $length++;
            $current = $nextRedirect->to_path;

            if ($this->isExternalUrl($current)) {
                break;
            }
        }

        $this->chain_length = $length;
    }

    /**
     * Check if URL is external
     */
    protected function isExternalUrl(string $url): bool
    {
        return str_starts_with($url, 'http://') || str_starts_with($url, 'https://');
    }

    /**
     * Clear redirect cache
     */
    protected function clearRedirectCache(): void
    {
        Cache::tags(['redirects'])->flush();
        Cache::forget("redirect:path:{$this->from_path}");
    }

    /**
     * Increment hit counter and update last hit timestamp
     */
    public function incrementHits(float $responseTime = null): self
    {
        $this->increment('hit_count');
        
        $updates = ['last_hit_at' => now()];
        
        // Update average response time
        if ($responseTime !== null) {
            $currentAvg = $this->avg_response_time ?? 0;
            $totalHits = $this->hit_count + 1;
            $updates['avg_response_time'] = (($currentAvg * $this->hit_count) + $responseTime) / $totalHits;
        }
        
        $this->updateQuietly($updates);

        return $this;
    }

    /**
     * Record a conversion
     */
    public function recordConversion(): self
    {
        $this->increment('conversion_count');
        $this->setMetadataValue('last_conversion_at', now()->toISOString());

        return $this;
    }

    /**
     * Activate redirect
     */
    public function activate(): self
    {
        $this->update(['is_active' => true]);
        return $this;
    }

    /**
     * Deactivate redirect
     */
    public function deactivate(): self
    {
        $this->update(['is_active' => false]);
        return $this;
    }

    /**
     * Check if redirect is expired
     */
    public function isExpired(): bool
    {
        return $this->expires_at && $this->expires_at->isPast();
    }

    /**
     * Check if redirect is permanent (301 or 308)
     */
    public function isPermanent(): bool
    {
        return in_array($this->status_code, [self::STATUS_301_PERMANENT, self::STATUS_308_PERMANENT], true);
    }

    /**
     * Check if redirect is temporary (302 or 307)
     */
    public function isTemporary(): bool
    {
        return in_array($this->status_code, [self::STATUS_302_FOUND, self::STATUS_307_TEMPORARY], true);
    }

    /**
     * Check if redirect chain is too long
     */
    public function hasLongChain(): bool
    {
        return $this->chain_length > self::MAX_SAFE_CHAIN_LENGTH;
    }

    /**
     * Check if redirect is popular (high traffic)
     */
    public function isPopular(int $threshold = 1000): bool
    {
        return $this->hit_count >= $threshold;
    }

    /**
     * Check if redirect is stale (not used recently)
     */
    public function isStale(int $days = 90): bool
    {
        if (!$this->last_hit_at) {
            return $this->created_at->addDays($days)->isPast();
        }

        return $this->last_hit_at->addDays($days)->isPast();
    }

    /**
     * Get conversion rate
     */
    public function getConversionRateAttribute(): float
    {
        if ($this->hit_count === 0) {
            return 0.0;
        }

        return round(($this->conversion_count / $this->hit_count) * 100, 2);
    }

    /**
     * Get destination URL with preserved query string
     */
    public function getDestinationUrl(string $queryString = null): string
    {
        $destination = $this->to_path;

        if ($this->preserve_query_string && $queryString) {
            $separator = str_contains($destination, '?') ? '&' : '?';
            $destination .= $separator . $queryString;
        }

        return $destination;
    }

    /**
     * Get human-readable status name
     */
    public function getStatusNameAttribute(): string
    {
        return match($this->status_code) {
            self::STATUS_301_PERMANENT => '301 Moved Permanently',
            self::STATUS_302_FOUND => '302 Found',
            self::STATUS_307_TEMPORARY => '307 Temporary Redirect',
            self::STATUS_308_PERMANENT => '308 Permanent Redirect',
            default => 'Unknown',
        };
    }

    /**
     * Get metadata value
     */
    public function getMetadataValue(string $key, mixed $default = null): mixed
    {
        return $this->metadata[$key] ?? $default;
    }

    /**
     * Set metadata value
     */
    public function setMetadataValue(string $key, mixed $value): self
    {
        $metadata = $this->metadata;
        $metadata[$key] = $value;
        $this->metadata = $metadata;

        return $this;
    }

    /**
     * Get health status
     */
    public function getHealthStatusAttribute(): string
    {
        if ($this->has_loop) {
            return 'critical';
        }

        if ($this->hasLongChain()) {
            return 'warning';
        }

        if ($this->isExpired()) {
            return 'expired';
        }

        if (!$this->is_active) {
            return 'inactive';
        }

        return 'healthy';
    }

    /**
     * Get color for health status
     */
    public function getHealthStatusColorAttribute(): string
    {
        return match($this->health_status) {
            'critical' => '#dc2626', // red-600
            'warning' => '#f59e0b', // amber-500
            'expired' => '#6b7280', // gray-500
            'inactive' => '#9ca3af', // gray-400
            default => '#22c55e', // green-500
        };
    }

    /**
     * Scope: Filter by type
     */
    public function scopeOfType(Builder $query, string $type): Builder
    {
        return $query->where('type', $type);
    }

    /**
     * Scope: Active redirects only
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where(function($q) {
                $q->whereNull('expires_at')
                  ->orWhere('expires_at', '>', now());
            });
    }

    /**
     * Scope: Inactive redirects
     */
    public function scopeInactive(Builder $query): Builder
    {
        return $query->where('is_active', false);
    }

    /**
     * Scope: Expired redirects
     */
    public function scopeExpired(Builder $query): Builder
    {
        return $query->whereNotNull('expires_at')
            ->where('expires_at', '<=', now());
    }

    /**
     * Scope: Permanent redirects (301, 308)
     */
    public function scopePermanent(Builder $query): Builder
    {
        return $query->whereIn('status_code', [self::STATUS_301_PERMANENT, self::STATUS_308_PERMANENT]);
    }

    /**
     * Scope: Temporary redirects (302, 307)
     */
    public function scopeTemporary(Builder $query): Builder
    {
        return $query->whereIn('status_code', [self::STATUS_302_FOUND, self::STATUS_307_TEMPORARY]);
    }

    /**
     * Scope: With redirect loops
     */
    public function scopeWithLoops(Builder $query): Builder
    {
        return $query->where('has_loop', true);
    }

    /**
     * Scope: With long chains
     */
    public function scopeWithLongChains(Builder $query, int $minLength = null): Builder
    {
        $minLength = $minLength ?? self::MAX_SAFE_CHAIN_LENGTH;
        return $query->where('chain_length', '>', $minLength);
    }

    /**
     * Scope: Popular redirects
     */
    public function scopePopular(Builder $query, int $minHits = 1000): Builder
    {
        return $query->where('hit_count', '>=', $minHits);
    }

    /**
     * Scope: Stale redirects
     */
    public function scopeStale(Builder $query, int $days = 90): Builder
    {
        return $query->where(function($q) use ($days) {
            $q->where(function($sq) use ($days) {
                $sq->whereNull('last_hit_at')
                   ->where('created_at', '<', now()->subDays($days));
            })->orWhere('last_hit_at', '<', now()->subDays($days));
        });
    }

    /**
     * Scope: Recently used
     */
    public function scopeRecentlyUsed(Builder $query, int $hours = 24): Builder
    {
        return $query->where('last_hit_at', '>=', now()->subHours($hours));
    }

    /**
     * Scope: Never used
     */
    public function scopeNeverUsed(Builder $query): Builder
    {
        return $query->where('hit_count', 0);
    }

    /**
     * Scope: Order by priority
     */
    public function scopeOrderByPriority(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('priority', $direction);
    }

    /**
     * Scope: Order by hits
     */
    public function scopeOrderByHits(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('hit_count', $direction);
    }

    /**
     * Static: Find redirect by path
     */
    public static function findByPath(string $path, bool $useCache = true): ?self
    {
        // Normalize path
        if (!str_starts_with($path, '/')) {
            $path = '/' . $path;
        }
        
        $path = rtrim($path, '/') ?: '/';

        if ($useCache) {
            return Cache::tags(['redirects'])->remember(
                "redirect:path:{$path}",
                self::CACHE_TTL,
                fn() => static::active()
                    ->where('from_path', $path)
                    ->orderByPriority()
                    ->first()
            );
        }

        return static::active()
            ->where('from_path', $path)
            ->orderByPriority()
            ->first();
    }

    /**
     * Static: Resolve final destination (follow chain)
     */
    public static function resolveFinalDestination(string $path, int $maxDepth = 10): ?string
    {
        $visited = [];
        $current = $path;
        $depth = 0;

        while ($depth < $maxDepth) {
            if (in_array($current, $visited, true)) {
                // Loop detected
                return null;
            }

            $redirect = static::findByPath($current);

            if (!$redirect) {
                return $current;
            }

            $visited[] = $current;
            $current = $redirect->to_path;
            $depth++;

            if ($redirect->isExternalUrl($current)) {
                return $current;
            }
        }

        return $current;
    }

    /**
     * Static: Get dashboard statistics
     */
    public static function getDashboardStats(): array
    {
        return [
            'total' => static::count(),
            'active' => static::active()->count(),
            'inactive' => static::inactive()->count(),
            'expired' => static::expired()->count(),
            'permanent' => static::permanent()->count(),
            'temporary' => static::temporary()->count(),
            'with_loops' => static::withLoops()->count(),
            'long_chains' => static::withLongChains()->count(),
            'stale' => static::stale()->count(),
            'never_used' => static::neverUsed()->count(),
            'total_hits' => static::sum('hit_count'),
            'total_conversions' => static::sum('conversion_count'),
            'avg_response_time' => round(static::whereNotNull('avg_response_time')->avg('avg_response_time'), 2),
        ];
    }

    /**
     * Static: Get top redirects by hits
     */
    public static function getTopRedirects(int $limit = 10): Collection
    {
        return static::active()
            ->orderByHits()
            ->limit($limit)
            ->get()
            ->map(fn($redirect) => [
                'id' => $redirect->id,
                'from_path' => $redirect->from_path,
                'to_path' => $redirect->to_path,
                'hit_count' => $redirect->hit_count,
                'conversion_count' => $redirect->conversion_count,
                'conversion_rate' => $redirect->conversion_rate,
                'last_hit_at' => $redirect->last_hit_at?->toISOString(),
            ]);
    }

    /**
     * Static: Get redirects needing attention
     */
    public static function getNeedingAttention(): Collection
    {
        return static::where(function($query) {
            $query->where('has_loop', true)
                  ->orWhere('chain_length', '>', self::MAX_SAFE_CHAIN_LENGTH)
                  ->orWhere(function($q) {
                      $q->whereNotNull('expires_at')
                        ->where('expires_at', '<', now()->addDays(7));
                  });
        })->get();
    }

    /**
     * Static: Bulk import redirects
     */
    public static function bulkImport(array $redirects, bool $skipExisting = true): array
    {
        $created = 0;
        $skipped = 0;
        $errors = [];

        foreach ($redirects as $redirect) {
            try {
                if ($skipExisting && static::where('from_path', $redirect['from_path'])->exists()) {
                    $skipped++;
                    continue;
                }

                static::create($redirect);
                $created++;
            } catch (\Exception $e) {
                $errors[] = [
                    'redirect' => $redirect,
                    'error' => $e->getMessage(),
                ];
            }
        }

        return [
            'created' => $created,
            'skipped' => $skipped,
            'errors' => $errors,
        ];
    }

    /**
     * Static: Clean up stale redirects
     */
    public static function cleanupStale(int $days = 180, bool $dryRun = true): array
    {
        $staleRedirects = static::stale($days)->neverUsed()->get();

        if ($dryRun) {
            return [
                'would_delete' => $staleRedirects->count(),
                'redirects' => $staleRedirects->pluck('from_path'),
            ];
        }

        $deleted = $staleRedirects->each->delete();

        return [
            'deleted' => $deleted->count(),
            'redirects' => $deleted->pluck('from_path'),
        ];
    }

    /**
     * Export to array for API
     */
    public function toRedirectArray(): array
    {
        return [
            'id' => $this->id,
            'from_path' => $this->from_path,
            'to_path' => $this->to_path,
            'status_code' => $this->status_code,
            'status_name' => $this->status_name,
            'type' => $this->type,
            'is_active' => $this->is_active,
            'is_permanent' => $this->isPermanent(),
            'hit_count' => $this->hit_count,
            'conversion_count' => $this->conversion_count,
            'conversion_rate' => $this->conversion_rate,
            'chain_length' => $this->chain_length,
            'has_loop' => $this->has_loop,
            'health_status' => $this->health_status,
            'health_status_color' => $this->health_status_color,
            'avg_response_time' => $this->avg_response_time,
            'last_hit_at' => $this->last_hit_at?->toISOString(),
            'expires_at' => $this->expires_at?->toISOString(),
            'created_at' => $this->created_at->toISOString(),
        ];
    }
}