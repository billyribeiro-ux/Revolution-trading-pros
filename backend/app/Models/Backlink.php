<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Str;
use Carbon\Carbon;
use App\Traits\HasUuid;
use App\Traits\Trackable;
use App\Traits\Searchable;
use App\Contracts\SEOAnalyzable;
use App\Events\BacklinkDiscovered;
use App\Events\BacklinkLost;
use App\Events\BacklinkStatusChanged;
use App\Services\SEO\DomainAnalyzer;
use App\Services\SEO\ToxicityAnalyzer;
use App\Services\SEO\BacklinkValueCalculator;
use App\Enums\BacklinkStatus;
use App\Enums\BacklinkType;
use App\Enums\ToxicityLevel;

/**
 * Backlink Model
 * 
 * Enterprise-grade backlink tracking and analysis system with comprehensive
 * SEO metrics, toxicity detection, competitor analysis, and value scoring.
 * 
 * @property int $id
 * @property string $uuid
 * @property string $source_url
 * @property string $source_domain
 * @property string $target_url
 * @property string $target_domain
 * @property string|null $anchor_text
 * @property string|null $anchor_type
 * @property string|null $link_context
 * @property int $domain_authority
 * @property int $page_authority
 * @property int $domain_rating
 * @property int $url_rating
 * @property float $trust_flow
 * @property float $citation_flow
 * @property int $referring_domains
 * @property int $referring_ips
 * @property int $referring_subnets
 * @property float $spam_score
 * @property float $toxicity_score
 * @property string $toxicity_level
 * @property array $toxicity_reasons
 * @property bool $is_follow
 * @property bool $is_sponsored
 * @property bool $is_ugc
 * @property bool $is_toxic
 * @property bool $is_image_link
 * @property bool $is_redirect
 * @property bool $is_canonical
 * @property bool $is_alternate
 * @property bool $is_mobile_alternate
 * @property bool $is_amp
 * @property string $link_type
 * @property string $link_position
 * @property string $status
 * @property array $http_status_history
 * @property int $http_status_code
 * @property float $link_value
 * @property float $estimated_traffic
 * @property float $estimated_value_usd
 * @property string|null $language
 * @property string|null $country_code
 * @property string|null $industry
 * @property string|null $competitor_domain
 * @property array|null $keywords
 * @property array|null $topics
 * @property int $outbound_links_count
 * @property int $inbound_links_count
 * @property int $external_links_count
 * @property int $position_on_page
 * @property float $page_rank
 * @property float $semantic_relevance
 * @property float $topical_trust_flow
 * @property array|null $social_metrics
 * @property array|null $page_metrics
 * @property array|null $technical_metrics
 * @property Carbon|null $first_seen_at
 * @property Carbon|null $last_seen_at
 * @property Carbon|null $last_checked_at
 * @property Carbon|null $lost_at
 * @property Carbon|null $disavowed_at
 * @property Carbon|null $approved_at
 * @property string|null $lost_reason
 * @property int $check_frequency_days
 * @property int $consecutive_failures
 * @property array|null $metadata
 * @property array|null $screenshot_url
 * @property array|null $wayback_url
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\Domain $sourceDomain
 * @property-read \App\Models\Domain $targetDomain
 * @property-read \App\Models\Page $sourcePage
 * @property-read \App\Models\Page $targetPage
 * @property-read \App\Models\Competitor $competitor
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\BacklinkHistory[] $history
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\BacklinkMetric[] $metrics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Note[] $notes
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Tag[] $tags
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Alert[] $alerts
 */
class Backlink extends Model implements SEOAnalyzable
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Trackable;
    use Searchable;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'backlinks';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'source_url',
        'source_domain',
        'target_url',
        'target_domain',
        'anchor_text',
        'anchor_type',
        'link_context',
        'domain_authority',
        'page_authority',
        'domain_rating',
        'url_rating',
        'trust_flow',
        'citation_flow',
        'referring_domains',
        'referring_ips',
        'referring_subnets',
        'spam_score',
        'toxicity_score',
        'toxicity_level',
        'toxicity_reasons',
        'is_follow',
        'is_sponsored',
        'is_ugc',
        'is_toxic',
        'is_image_link',
        'is_redirect',
        'is_canonical',
        'is_alternate',
        'is_mobile_alternate',
        'is_amp',
        'link_type',
        'link_position',
        'status',
        'http_status_history',
        'http_status_code',
        'link_value',
        'estimated_traffic',
        'estimated_value_usd',
        'language',
        'country_code',
        'industry',
        'competitor_domain',
        'keywords',
        'topics',
        'outbound_links_count',
        'inbound_links_count',
        'external_links_count',
        'position_on_page',
        'page_rank',
        'semantic_relevance',
        'topical_trust_flow',
        'social_metrics',
        'page_metrics',
        'technical_metrics',
        'first_seen_at',
        'last_seen_at',
        'last_checked_at',
        'lost_at',
        'disavowed_at',
        'approved_at',
        'lost_reason',
        'check_frequency_days',
        'consecutive_failures',
        'metadata',
        'screenshot_url',
        'wayback_url',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'domain_authority' => 'integer',
        'page_authority' => 'integer',
        'domain_rating' => 'integer',
        'url_rating' => 'integer',
        'trust_flow' => 'float',
        'citation_flow' => 'float',
        'referring_domains' => 'integer',
        'referring_ips' => 'integer',
        'referring_subnets' => 'integer',
        'spam_score' => 'float',
        'toxicity_score' => 'float',
        'toxicity_level' => ToxicityLevel::class,
        'toxicity_reasons' => 'array',
        'is_follow' => 'boolean',
        'is_sponsored' => 'boolean',
        'is_ugc' => 'boolean',
        'is_toxic' => 'boolean',
        'is_image_link' => 'boolean',
        'is_redirect' => 'boolean',
        'is_canonical' => 'boolean',
        'is_alternate' => 'boolean',
        'is_mobile_alternate' => 'boolean',
        'is_amp' => 'boolean',
        'link_type' => BacklinkType::class,
        'status' => BacklinkStatus::class,
        'http_status_history' => 'array',
        'http_status_code' => 'integer',
        'link_value' => 'float',
        'estimated_traffic' => 'float',
        'estimated_value_usd' => 'float',
        'keywords' => 'array',
        'topics' => 'array',
        'outbound_links_count' => 'integer',
        'inbound_links_count' => 'integer',
        'external_links_count' => 'integer',
        'position_on_page' => 'integer',
        'page_rank' => 'float',
        'semantic_relevance' => 'float',
        'topical_trust_flow' => 'float',
        'social_metrics' => 'array',
        'page_metrics' => 'array',
        'technical_metrics' => 'array',
        'first_seen_at' => 'datetime',
        'last_seen_at' => 'datetime',
        'last_checked_at' => 'datetime',
        'lost_at' => 'datetime',
        'disavowed_at' => 'datetime',
        'approved_at' => 'datetime',
        'check_frequency_days' => 'integer',
        'consecutive_failures' => 'integer',
        'metadata' => 'array',
        'screenshot_url' => 'array',
        'wayback_url' => 'array',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be searchable.
     *
     * @var array<string>
     */
    protected $searchable = [
        'source_url',
        'target_url',
        'anchor_text',
        'source_domain',
        'target_domain',
        'link_context',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'deleted_at',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'quality_score',
        'is_active',
        'days_active',
        'value_rating',
        'rel_attributes',
        'monitoring_status',
    ];

    /**
     * Default values for attributes.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'domain_authority' => 0,
        'page_authority' => 0,
        'spam_score' => 0,
        'toxicity_score' => 0,
        'is_follow' => true,
        'is_toxic' => false,
        'status' => BacklinkStatus::ACTIVE,
        'check_frequency_days' => 7,
        'consecutive_failures' => 0,
        'link_value' => 0,
    ];

    /**
     * The model's default values for attributes.
     *
     * @return void
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $backlink) {
            $backlink->uuid = $backlink->uuid ?? (string) Str::uuid();
            $backlink->source_domain = $backlink->source_domain ?? parse_url($backlink->source_url, PHP_URL_HOST);
            $backlink->target_domain = $backlink->target_domain ?? parse_url($backlink->target_url, PHP_URL_HOST);
            $backlink->first_seen_at = $backlink->first_seen_at ?? now();
            $backlink->last_seen_at = $backlink->last_seen_at ?? now();
            $backlink->analyzeAnchorType();
            $backlink->detectLinkType();
        });

        static::created(function (self $backlink) {
            event(new BacklinkDiscovered($backlink));
            $backlink->analyzeBacklinkQuality();
            $backlink->checkToxicity();
            $backlink->calculateValue();
        });

        static::updating(function (self $backlink) {
            if ($backlink->isDirty('status')) {
                $backlink->handleStatusChange();
            }
            
            if ($backlink->isDirty(['domain_authority', 'page_authority', 'spam_score'])) {
                $backlink->recalculateValue();
            }
        });

        static::updated(function (self $backlink) {
            if ($backlink->wasChanged('status')) {
                event(new BacklinkStatusChanged($backlink));
            }
            
            if ($backlink->status === BacklinkStatus::LOST && $backlink->wasChanged('status')) {
                event(new BacklinkLost($backlink));
            }
        });
    }

    /**
     * Get the source domain relationship.
     */
    public function sourceDomain(): BelongsTo
    {
        return $this->belongsTo(Domain::class, 'source_domain', 'domain');
    }

    /**
     * Get the target domain relationship.
     */
    public function targetDomain(): BelongsTo
    {
        return $this->belongsTo(Domain::class, 'target_domain', 'domain');
    }

    /**
     * Get the source page relationship.
     */
    public function sourcePage(): BelongsTo
    {
        return $this->belongsTo(Page::class, 'source_url', 'url');
    }

    /**
     * Get the target page relationship.
     */
    public function targetPage(): BelongsTo
    {
        return $this->belongsTo(Page::class, 'target_url', 'url');
    }

    /**
     * Get the competitor relationship.
     */
    public function competitor(): BelongsTo
    {
        return $this->belongsTo(Competitor::class, 'competitor_domain', 'domain');
    }

    /**
     * Get the backlink history.
     */
    public function history(): HasMany
    {
        return $this->hasMany(BacklinkHistory::class)->orderBy('created_at', 'desc');
    }

    /**
     * Get the backlink metrics over time.
     */
    public function metrics(): HasMany
    {
        return $this->hasMany(BacklinkMetric::class)->orderBy('recorded_at', 'desc');
    }

    /**
     * Get the notes for this backlink.
     */
    public function notes(): MorphMany
    {
        return $this->morphMany(Note::class, 'notable');
    }

    /**
     * Get the tags for this backlink.
     */
    public function tags(): BelongsToMany
    {
        return $this->belongsToMany(Tag::class, 'backlink_tags')
            ->withTimestamps()
            ->withPivot(['added_by', 'reason']);
    }

    /**
     * Get the alerts for this backlink.
     */
    public function alerts(): MorphMany
    {
        return $this->morphMany(Alert::class, 'alertable');
    }

    /**
     * Scope for active backlinks.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('status', BacklinkStatus::ACTIVE)
            ->whereNull('lost_at');
    }

    /**
     * Scope for lost backlinks.
     */
    public function scopeLost(Builder $query): Builder
    {
        return $query->where('status', BacklinkStatus::LOST)
            ->orWhereNotNull('lost_at');
    }

    /**
     * Scope for toxic backlinks.
     */
    public function scopeToxic(Builder $query): Builder
    {
        return $query->where('is_toxic', true)
            ->orWhere('toxicity_score', '>=', 70);
    }

    /**
     * Scope for high-quality backlinks.
     */
    public function scopeHighQuality(Builder $query): Builder
    {
        return $query->where('domain_authority', '>=', 50)
            ->where('spam_score', '<=', 30)
            ->where('is_follow', true)
            ->where('is_toxic', false);
    }

    /**
     * Scope for dofollow backlinks.
     */
    public function scopeDofollow(Builder $query): Builder
    {
        return $query->where('is_follow', true);
    }

    /**
     * Scope for nofollow backlinks.
     */
    public function scopeNofollow(Builder $query): Builder
    {
        return $query->where('is_follow', false);
    }

    /**
     * Scope for competitor backlinks.
     */
    public function scopeCompetitor(Builder $query, ?string $domain = null): Builder
    {
        return $query->when($domain, function ($q) use ($domain) {
            $q->where('competitor_domain', $domain);
        }, function ($q) {
            $q->whereNotNull('competitor_domain');
        });
    }

    /**
     * Scope for backlinks from a specific domain.
     */
    public function scopeFromDomain(Builder $query, string $domain): Builder
    {
        return $query->where('source_domain', $domain);
    }

    /**
     * Scope for backlinks to a specific URL.
     */
    public function scopeToUrl(Builder $query, string $url): Builder
    {
        return $query->where('target_url', $url);
    }

    /**
     * Scope for recently discovered backlinks.
     */
    public function scopeRecentlyDiscovered(Builder $query, int $days = 7): Builder
    {
        return $query->where('first_seen_at', '>=', now()->subDays($days));
    }

    /**
     * Scope for backlinks that need checking.
     */
    public function scopeNeedsChecking(Builder $query): Builder
    {
        return $query->where(function ($q) {
            $q->whereNull('last_checked_at')
                ->orWhereRaw('DATE_ADD(last_checked_at, INTERVAL check_frequency_days DAY) <= NOW()');
        });
    }

    /**
     * Get the quality score attribute.
     */
    public function getQualityScoreAttribute(): float
    {
        return Cache::remember(
            "backlink_quality_{$this->id}",
            3600,
            fn() => $this->calculateQualityScore()
        );
    }

    /**
     * Calculate the quality score.
     */
    public function calculateQualityScore(): float
    {
        $score = 0;
        
        // Domain Authority (30% weight)
        $score += ($this->domain_authority / 100) * 30;
        
        // Page Authority (20% weight)
        $score += ($this->page_authority / 100) * 20;
        
        // Trust Flow (15% weight)
        $score += ($this->trust_flow / 100) * 15;
        
        // Spam Score (negative 20% weight)
        $score -= ($this->spam_score / 100) * 20;
        
        // Follow status (10% weight)
        $score += $this->is_follow ? 10 : 0;
        
        // Toxicity (negative 15% weight)
        $score -= ($this->toxicity_score / 100) * 15;
        
        // Semantic relevance (10% weight)
        $score += ($this->semantic_relevance / 100) * 10;
        
        return max(0, min(100, $score));
    }

    /**
     * Get whether the backlink is active.
     */
    public function getIsActiveAttribute(): bool
    {
        return $this->status === BacklinkStatus::ACTIVE && 
               $this->lost_at === null &&
               $this->http_status_code >= 200 && 
               $this->http_status_code < 400;
    }

    /**
     * Get the number of days the backlink has been active.
     */
    public function getDaysActiveAttribute(): int
    {
        if (!$this->first_seen_at) {
            return 0;
        }

        $endDate = $this->lost_at ?? now();
        return $this->first_seen_at->diffInDays($endDate);
    }

    /**
     * Get the value rating.
     */
    public function getValueRatingAttribute(): string
    {
        return match(true) {
            $this->link_value >= 90 => 'Exceptional',
            $this->link_value >= 70 => 'High',
            $this->link_value >= 50 => 'Good',
            $this->link_value >= 30 => 'Moderate',
            $this->link_value >= 10 => 'Low',
            default => 'Minimal',
        };
    }

    /**
     * Get the rel attributes.
     */
    public function getRelAttributesAttribute(): array
    {
        $attributes = [];
        
        if (!$this->is_follow) $attributes[] = 'nofollow';
        if ($this->is_sponsored) $attributes[] = 'sponsored';
        if ($this->is_ugc) $attributes[] = 'ugc';
        
        return $attributes;
    }

    /**
     * Get the monitoring status.
     */
    public function getMonitoringStatusAttribute(): string
    {
        if ($this->consecutive_failures >= 5) {
            return 'Critical';
        }
        
        if ($this->consecutive_failures >= 3) {
            return 'Warning';
        }
        
        if (!$this->last_checked_at) {
            return 'Pending';
        }
        
        $daysSinceCheck = $this->last_checked_at->diffInDays(now());
        
        return match(true) {
            $daysSinceCheck > $this->check_frequency_days * 2 => 'Overdue',
            $daysSinceCheck > $this->check_frequency_days => 'Due',
            default => 'Active',
        };
    }

    /**
     * Analyze the anchor text type.
     */
    public function analyzeAnchorType(): void
    {
        if (!$this->anchor_text) {
            $this->anchor_type = 'empty';
            return;
        }

        $anchor = strtolower($this->anchor_text);
        $targetDomain = parse_url($this->target_url, PHP_URL_HOST);
        
        $this->anchor_type = match(true) {
            $anchor === $targetDomain => 'exact_domain',
            str_contains($anchor, $targetDomain) => 'partial_domain',
            preg_match('/^https?:\/\//', $anchor) => 'naked_url',
            in_array($anchor, ['click here', 'read more', 'learn more', 'here']) => 'generic',
            str_contains($this->target_url, $anchor) => 'exact_match',
            default => 'branded',
        };
    }

    /**
     * Detect the link type.
     */
    public function detectLinkType(): void
    {
        $this->link_type = match(true) {
            $this->is_image_link => BacklinkType::IMAGE,
            $this->is_redirect => BacklinkType::REDIRECT,
            $this->is_canonical => BacklinkType::CANONICAL,
            $this->is_alternate => BacklinkType::ALTERNATE,
            str_contains($this->source_url, '/comments/') => BacklinkType::COMMENT,
            str_contains($this->source_url, '/forum/') => BacklinkType::FORUM,
            preg_match('/\/(blog|news|article)\//', $this->source_url) => BacklinkType::EDITORIAL,
            default => BacklinkType::CONTEXTUAL,
        };
    }

    /**
     * Check the backlink status.
     */
    public function checkStatus(): void
    {
        try {
            $response = Http::timeout(10)
                ->withHeaders(['User-Agent' => config('app.crawler_user_agent')])
                ->get($this->source_url);

            $this->http_status_code = $response->status();
            $this->last_checked_at = now();
            
            // Store status history
            $history = $this->http_status_history ?? [];
            array_unshift($history, [
                'code' => $response->status(),
                'checked_at' => now()->toIso8601String(),
            ]);
            $this->http_status_history = array_slice($history, 0, 10);

            if ($response->successful()) {
                $html = $response->body();
                $linkExists = $this->verifyLinkInContent($html);
                
                if ($linkExists) {
                    $this->status = BacklinkStatus::ACTIVE;
                    $this->last_seen_at = now();
                    $this->consecutive_failures = 0;
                } else {
                    $this->handleLinkNotFound();
                }
            } else {
                $this->handleHttpError($response->status());
            }
        } catch (\Exception $e) {
            $this->handleCheckError($e);
        }
    }

    /**
     * Verify if the link exists in the content.
     */
    protected function verifyLinkInContent(string $html): bool
    {
        $dom = new \DOMDocument();
        @$dom->loadHTML($html);
        $xpath = new \DOMXPath($dom);
        
        // Check for exact link
        $query = "//a[@href='{$this->target_url}']";
        $nodes = $xpath->query($query);
        
        if ($nodes->length > 0) {
            $this->updateLinkContext($nodes->item(0));
            return true;
        }
        
        // Check for redirected or modified URLs
        $parsedTarget = parse_url($this->target_url);
        $query = "//a[contains(@href, '{$parsedTarget['host']}')]";
        $nodes = $xpath->query($query);
        
        foreach ($nodes as $node) {
            if ($this->isUrlMatch($node->getAttribute('href'), $this->target_url)) {
                $this->updateLinkContext($node);
                return true;
            }
        }
        
        return false;
    }

    /**
     * Update link context from DOM node.
     */
    protected function updateLinkContext(\DOMNode $node): void
    {
        $this->anchor_text = trim($node->textContent);
        
        // Get rel attributes
        $rel = $node->getAttribute('rel');
        $this->is_follow = !str_contains($rel, 'nofollow');
        $this->is_sponsored = str_contains($rel, 'sponsored');
        $this->is_ugc = str_contains($rel, 'ugc');
        
        // Get surrounding context
        $parent = $node->parentNode;
        $context = '';
        foreach ($parent->childNodes as $child) {
            $context .= $child->textContent . ' ';
        }
        $this->link_context = Str::limit(trim($context), 500);
    }

    /**
     * Handle link not found scenario.
     */
    protected function handleLinkNotFound(): void
    {
        $this->consecutive_failures++;
        
        if ($this->consecutive_failures >= 3) {
            $this->status = BacklinkStatus::LOST;
            $this->lost_at = now();
            $this->lost_reason = 'Link not found in content after ' . $this->consecutive_failures . ' checks';
        } else {
            $this->status = BacklinkStatus::NOT_FOUND;
        }
    }

    /**
     * Check toxicity using multiple signals.
     */
    public function checkToxicity(): void
    {
        $analyzer = app(ToxicityAnalyzer::class);
        $result = $analyzer->analyze($this);
        
        $this->toxicity_score = $result['score'];
        $this->toxicity_level = $result['level'];
        $this->toxicity_reasons = $result['reasons'];
        $this->is_toxic = $result['is_toxic'];
        
        if ($this->is_toxic) {
            $this->createToxicityAlert();
        }
    }

    /**
     * Calculate the backlink value.
     */
    public function calculateValue(): void
    {
        $calculator = app(BacklinkValueCalculator::class);
        
        $this->link_value = $calculator->calculate($this);
        $this->estimated_traffic = $calculator->estimateTraffic($this);
        $this->estimated_value_usd = $calculator->estimateMonetaryValue($this);
    }

    /**
     * Analyze overall backlink quality.
     */
    public function analyzeBacklinkQuality(): void
    {
        dispatch(function () {
            $this->fetchDomainMetrics();
            $this->analyzeSemanticRelevance();
            $this->checkToxicity();
            $this->calculateValue();
            $this->save();
        })->afterResponse();
    }

    /**
     * Fetch domain metrics from external APIs.
     */
    public function fetchDomainMetrics(): void
    {
        $analyzer = app(DomainAnalyzer::class);
        $metrics = $analyzer->analyze($this->source_domain);
        
        $this->fill([
            'domain_authority' => $metrics['domain_authority'] ?? $this->domain_authority,
            'domain_rating' => $metrics['domain_rating'] ?? $this->domain_rating,
            'trust_flow' => $metrics['trust_flow'] ?? $this->trust_flow,
            'citation_flow' => $metrics['citation_flow'] ?? $this->citation_flow,
            'referring_domains' => $metrics['referring_domains'] ?? $this->referring_domains,
            'spam_score' => $metrics['spam_score'] ?? $this->spam_score,
        ]);
    }

    /**
     * Create a toxicity alert.
     */
    protected function createToxicityAlert(): void
    {
        $this->alerts()->create([
            'type' => 'toxicity_detected',
            'severity' => 'high',
            'title' => 'Toxic Backlink Detected',
            'message' => "Backlink from {$this->source_domain} has been identified as toxic with a score of {$this->toxicity_score}",
            'data' => [
                'toxicity_score' => $this->toxicity_score,
                'reasons' => $this->toxicity_reasons,
            ],
        ]);
    }

    /**
     * Disavow the backlink.
     */
    public function disavow(): void
    {
        $this->disavowed_at = now();
        $this->status = BacklinkStatus::DISAVOWED;
        $this->save();
        
        // Add to disavow file
        dispatch(new UpdateDisavowFile($this));
    }

    /**
     * Create a competitor gap opportunity.
     */
    public function createCompetitorOpportunity(): void
    {
        CompetitorOpportunity::create([
            'backlink_id' => $this->id,
            'competitor_domain' => $this->competitor_domain,
            'source_domain' => $this->source_domain,
            'opportunity_score' => $this->calculateOpportunityScore(),
            'estimated_difficulty' => $this->estimateAcquisitionDifficulty(),
            'estimated_cost' => $this->estimateAcquisitionCost(),
            'priority' => $this->calculateOpportunityPriority(),
        ]);
    }

    /**
     * Get anchor text distribution for the target domain.
     */
    public static function getAnchorTextDistribution(string $targetDomain): array
    {
        return static::where('target_domain', $targetDomain)
            ->active()
            ->get()
            ->groupBy('anchor_type')
            ->map(function ($group, $type) {
                return [
                    'type' => $type,
                    'count' => $group->count(),
                    'percentage' => ($group->count() / static::where('target_domain', $group->first()->target_domain)->count()) * 100,
                ];
            })
            ->values()
            ->toArray();
    }

    /**
     * Get the link growth trend.
     */
    public static function getLinkGrowthTrend(string $targetDomain, int $days = 30): array
    {
        return static::where('target_domain', $targetDomain)
            ->where('first_seen_at', '>=', now()->subDays($days))
            ->selectRaw('DATE(first_seen_at) as date, COUNT(*) as count')
            ->groupBy('date')
            ->orderBy('date')
            ->get()
            ->toArray();
    }
}