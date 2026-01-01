<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Carbon\Carbon;
use App\Traits\HasUuid;
use App\Traits\Trackable;
use App\Traits\HasAnalytics;
use App\Contracts\Analyzable;
use App\Events\RankingImproved;
use App\Events\RankingDropped;
use App\Events\RankingLost;
use App\Events\RankingRecovered;
use App\Events\FeaturedSnippetGained;
use App\Events\FeaturedSnippetLost;
use App\Enums\SearchEngine;
use App\Enums\DeviceType;
use App\Enums\RankingStatus;
use App\Enums\SERPFeature;
use App\Enums\TrackingFrequency;
use App\Enums\AlertType;
use App\Services\SEO\SERPAnalyzer;
use App\Services\SEO\RankChecker;
use App\Services\SEO\CompetitorAnalyzer;
use App\Services\SEO\KeywordAnalyzer;
use App\Exceptions\RankTrackingException;

/**
 * RankTracking Model
 * 
 * Enterprise-grade SERP rank tracking system with competitor analysis,
 * SERP features monitoring, volatility tracking, and comprehensive analytics.
 * 
 * @property int $id
 * @property string $uuid
 * @property int $project_id
 * @property int $domain_id
 * @property string $keyword
 * @property string $keyword_slug
 * @property string $url
 * @property string $domain
 * @property string $path
 * @property string $protocol
 * @property int|null $current_position
 * @property int|null $previous_position
 * @property int|null $best_position
 * @property int|null $worst_position
 * @property float|null $average_position
 * @property int $position_change
 * @property float $position_change_percentage
 * @property string $search_engine
 * @property string $search_engine_domain
 * @property string $location
 * @property string|null $location_code
 * @property float|null $latitude
 * @property float|null $longitude
 * @property string|null $language
 * @property string $device
 * @property string|null $user_agent
 * @property bool $is_mobile_friendly
 * @property int $search_volume
 * @property float $keyword_difficulty
 * @property float $cpc
 * @property float $competition
 * @property array $serp_features
 * @property bool $has_featured_snippet
 * @property bool $has_knowledge_panel
 * @property bool $has_local_pack
 * @property bool $has_image_pack
 * @property bool $has_video_carousel
 * @property bool $has_people_also_ask
 * @property bool $has_shopping_results
 * @property bool $has_ads_top
 * @property bool $has_ads_bottom
 * @property int $ads_count
 * @property int $organic_results_count
 * @property array|null $featured_snippet_data
 * @property array|null $knowledge_panel_data
 * @property array|null $local_pack_data
 * @property array|null $people_also_ask_data
 * @property string|null $ranking_url
 * @property string|null $canonical_url
 * @property string|null $title
 * @property string|null $description
 * @property int|null $title_length
 * @property int|null $description_length
 * @property array|null $breadcrumbs
 * @property array|null $rich_snippets
 * @property array|null $schema_markup
 * @property float|null $page_speed_score
 * @property float|null $core_web_vitals_score
 * @property string $status
 * @property string $tracking_frequency
 * @property bool $is_active
 * @property bool $is_tracked
 * @property bool $is_favorite
 * @property bool $is_branded
 * @property bool $is_competitor
 * @property string|null $competitor_domain
 * @property int|null $competitor_position
 * @property array|null $top_competitors
 * @property array|null $serp_competitors
 * @property float $visibility_score
 * @property float $share_of_voice
 * @property float $estimated_traffic
 * @property float $estimated_value
 * @property float $click_through_rate
 * @property int $impressions
 * @property int $clicks
 * @property float $volatility_score
 * @property float $stability_score
 * @property int $days_in_top_3
 * @property int $days_in_top_10
 * @property int $days_in_top_100
 * @property int $days_not_ranking
 * @property Carbon|null $first_seen_at
 * @property Carbon|null $last_seen_at
 * @property Carbon|null $last_checked_at
 * @property Carbon|null $next_check_at
 * @property Carbon|null $lost_ranking_at
 * @property Carbon|null $recovered_ranking_at
 * @property int $check_count
 * @property int $consecutive_failures
 * @property string|null $last_error
 * @property array|null $error_details
 * @property array|null $alerts_config
 * @property bool $alert_on_position_change
 * @property int|null $alert_threshold
 * @property array|null $notification_channels
 * @property array|null $tags
 * @property array|null $categories
 * @property array|null $metadata
 * @property array|null $custom_fields
 * @property int|null $group_id
 * @property int|null $campaign_id
 * @property int $created_by
 * @property int|null $updated_by
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\Project $project
 * @property-read \App\Models\Domain $domainRecord
 * @property-read \App\Models\Keyword $keywordRecord
 * @property-read \App\Models\RankTrackingGroup $group
 * @property-read \App\Models\Campaign $campaign
 * @property-read \App\Models\User $creator
 * @property-read \App\Models\User|null $updater
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankHistory[] $histories
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\SERPSnapshot[] $serpSnapshots
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CompetitorRank[] $competitorRanks
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankAlert[] $alerts
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankAnnotation[] $annotations
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankMetric[] $metrics
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Tag[] $tagRecords
 */
class RankTracking extends Model implements Analyzable
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Trackable;
    use HasAnalytics;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'rank_trackings';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'project_id',
        'domain_id',
        'keyword',
        'keyword_slug',
        'url',
        'domain',
        'path',
        'protocol',
        'current_position',
        'previous_position',
        'best_position',
        'worst_position',
        'average_position',
        'position_change',
        'position_change_percentage',
        'search_engine',
        'search_engine_domain',
        'location',
        'location_code',
        'latitude',
        'longitude',
        'language',
        'device',
        'user_agent',
        'is_mobile_friendly',
        'search_volume',
        'keyword_difficulty',
        'cpc',
        'competition',
        'serp_features',
        'has_featured_snippet',
        'has_knowledge_panel',
        'has_local_pack',
        'has_image_pack',
        'has_video_carousel',
        'has_people_also_ask',
        'has_shopping_results',
        'has_ads_top',
        'has_ads_bottom',
        'ads_count',
        'organic_results_count',
        'featured_snippet_data',
        'knowledge_panel_data',
        'local_pack_data',
        'people_also_ask_data',
        'ranking_url',
        'canonical_url',
        'title',
        'description',
        'title_length',
        'description_length',
        'breadcrumbs',
        'rich_snippets',
        'schema_markup',
        'page_speed_score',
        'core_web_vitals_score',
        'status',
        'tracking_frequency',
        'is_active',
        'is_tracked',
        'is_favorite',
        'is_branded',
        'is_competitor',
        'competitor_domain',
        'competitor_position',
        'top_competitors',
        'serp_competitors',
        'visibility_score',
        'share_of_voice',
        'estimated_traffic',
        'estimated_value',
        'click_through_rate',
        'impressions',
        'clicks',
        'volatility_score',
        'stability_score',
        'days_in_top_3',
        'days_in_top_10',
        'days_in_top_100',
        'days_not_ranking',
        'first_seen_at',
        'last_seen_at',
        'last_checked_at',
        'next_check_at',
        'lost_ranking_at',
        'recovered_ranking_at',
        'check_count',
        'consecutive_failures',
        'last_error',
        'error_details',
        'alerts_config',
        'alert_on_position_change',
        'alert_threshold',
        'notification_channels',
        'tags',
        'categories',
        'metadata',
        'custom_fields',
        'group_id',
        'campaign_id',
        'created_by',
        'updated_by',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'current_position' => 'integer',
        'previous_position' => 'integer',
        'best_position' => 'integer',
        'worst_position' => 'integer',
        'average_position' => 'float',
        'position_change' => 'integer',
        'position_change_percentage' => 'float',
        'search_volume' => 'integer',
        'keyword_difficulty' => 'float',
        'cpc' => 'decimal:2',
        'competition' => 'float',
        'latitude' => 'float',
        'longitude' => 'float',
        'ads_count' => 'integer',
        'organic_results_count' => 'integer',
        'title_length' => 'integer',
        'description_length' => 'integer',
        'page_speed_score' => 'float',
        'core_web_vitals_score' => 'float',
        'competitor_position' => 'integer',
        'visibility_score' => 'float',
        'share_of_voice' => 'float',
        'estimated_traffic' => 'float',
        'estimated_value' => 'decimal:2',
        'click_through_rate' => 'float',
        'impressions' => 'integer',
        'clicks' => 'integer',
        'volatility_score' => 'float',
        'stability_score' => 'float',
        'days_in_top_3' => 'integer',
        'days_in_top_10' => 'integer',
        'days_in_top_100' => 'integer',
        'days_not_ranking' => 'integer',
        'check_count' => 'integer',
        'consecutive_failures' => 'integer',
        'alert_threshold' => 'integer',
        'is_mobile_friendly' => 'boolean',
        'has_featured_snippet' => 'boolean',
        'has_knowledge_panel' => 'boolean',
        'has_local_pack' => 'boolean',
        'has_image_pack' => 'boolean',
        'has_video_carousel' => 'boolean',
        'has_people_also_ask' => 'boolean',
        'has_shopping_results' => 'boolean',
        'has_ads_top' => 'boolean',
        'has_ads_bottom' => 'boolean',
        'is_active' => 'boolean',
        'is_tracked' => 'boolean',
        'is_favorite' => 'boolean',
        'is_branded' => 'boolean',
        'is_competitor' => 'boolean',
        'alert_on_position_change' => 'boolean',
        'search_engine' => SearchEngine::class,
        'device' => DeviceType::class,
        'status' => RankingStatus::class,
        'tracking_frequency' => TrackingFrequency::class,
        'serp_features' => 'array',
        'featured_snippet_data' => 'array',
        'knowledge_panel_data' => 'array',
        'local_pack_data' => 'array',
        'people_also_ask_data' => 'array',
        'breadcrumbs' => 'array',
        'rich_snippets' => 'array',
        'schema_markup' => 'array',
        'top_competitors' => 'array',
        'serp_competitors' => 'array',
        'error_details' => 'array',
        'alerts_config' => 'array',
        'notification_channels' => 'array',
        'tags' => 'array',
        'categories' => 'array',
        'metadata' => 'array',
        'custom_fields' => 'array',
        'first_seen_at' => 'datetime',
        'last_seen_at' => 'datetime',
        'last_checked_at' => 'datetime',
        'next_check_at' => 'datetime',
        'lost_ranking_at' => 'datetime',
        'recovered_ranking_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
        'deleted_at' => 'datetime',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'deleted_at',
        'error_details',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_ranking',
        'is_improving',
        'is_declining',
        'is_stable',
        'is_top_3',
        'is_top_10',
        'is_first_page',
        'position_trend',
        'visibility_change',
        'days_since_last_check',
        'performance_score',
        'opportunity_score',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'search_engine' => SearchEngine::GOOGLE,
        'device' => DeviceType::DESKTOP,
        'status' => RankingStatus::ACTIVE,
        'tracking_frequency' => TrackingFrequency::DAILY,
        'is_active' => true,
        'is_tracked' => true,
        'is_favorite' => false,
        'is_branded' => false,
        'is_competitor' => false,
        'position_change' => 0,
        'position_change_percentage' => 0,
        'search_volume' => 0,
        'keyword_difficulty' => 0,
        'cpc' => 0,
        'competition' => 0,
        'visibility_score' => 0,
        'share_of_voice' => 0,
        'estimated_traffic' => 0,
        'estimated_value' => 0,
        'click_through_rate' => 0,
        'impressions' => 0,
        'clicks' => 0,
        'volatility_score' => 0,
        'stability_score' => 100,
        'days_in_top_3' => 0,
        'days_in_top_10' => 0,
        'days_in_top_100' => 0,
        'days_not_ranking' => 0,
        'check_count' => 0,
        'consecutive_failures' => 0,
        'alert_on_position_change' => true,
        'alert_threshold' => 3,
        'ads_count' => 0,
        'organic_results_count' => 10,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'tags' => ['rank_tracking'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $rankTracking) {
            $rankTracking->uuid = $rankTracking->uuid ?? (string) Str::uuid();
            $rankTracking->keyword_slug = $rankTracking->keyword_slug ?? Str::slug($rankTracking->keyword);
            $rankTracking->created_by = $rankTracking->created_by ?? auth()->id();
            
            $rankTracking->parseUrl();
            $rankTracking->detectBrandedKeyword();
            $rankTracking->scheduleNextCheck();
        });

        static::created(function (self $rankTracking) {
            $rankTracking->checkInitialRanking();
            $rankTracking->analyzeKeyword();
        });

        static::updating(function (self $rankTracking) {
            $rankTracking->updated_by = auth()->id();
            
            if ($rankTracking->isDirty('current_position')) {
                $rankTracking->handlePositionChange();
            }
        });

        static::updated(function (self $rankTracking) {
            $rankTracking->clearCache();
            
            if ($rankTracking->wasChanged('current_position')) {
                $rankTracking->firePositionEvents();
                $rankTracking->checkAlerts();
            }
        });
    }

    /**
     * Get the project.
     */
    public function project(): BelongsTo
    {
        return $this->belongsTo(Project::class);
    }

    /**
     * Get the domain record.
     */
    public function domainRecord(): BelongsTo
    {
        return $this->belongsTo(Domain::class, 'domain_id');
    }

    /**
     * Get the keyword record.
     */
    public function keywordRecord(): BelongsTo
    {
        return $this->belongsTo(Keyword::class, 'keyword', 'keyword');
    }

    /**
     * Get the group.
     */
    public function group(): BelongsTo
    {
        return $this->belongsTo(RankTrackingGroup::class, 'group_id');
    }

    /**
     * Get the campaign.
     */
    public function campaign(): BelongsTo
    {
        return $this->belongsTo(Campaign::class);
    }

    /**
     * Get the creator.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get the updater.
     */
    public function updater(): BelongsTo
    {
        return $this->belongsTo(User::class, 'updated_by');
    }

    /**
     * Get ranking history.
     */
    public function histories(): HasMany
    {
        return $this->hasMany(RankHistory::class)
            ->orderBy('checked_at', 'desc');
    }

    /**
     * Get SERP snapshots.
     */
    public function serpSnapshots(): HasMany
    {
        return $this->hasMany(SERPSnapshot::class)
            ->orderBy('captured_at', 'desc');
    }

    /**
     * Get competitor ranks.
     */
    public function competitorRanks(): HasMany
    {
        return $this->hasMany(CompetitorRank::class)
            ->orderBy('position');
    }

    /**
     * Get alerts.
     */
    public function alerts(): HasMany
    {
        return $this->hasMany(RankAlert::class)
            ->orderBy('triggered_at', 'desc');
    }

    /**
     * Get annotations.
     */
    public function annotations(): HasMany
    {
        return $this->hasMany(RankAnnotation::class)
            ->orderBy('annotated_at', 'desc');
    }

    /**
     * Get metrics.
     */
    public function metrics(): HasMany
    {
        return $this->hasMany(RankMetric::class)
            ->orderBy('recorded_at', 'desc');
    }

    /**
     * Get tags.
     */
    public function tagRecords(): BelongsToMany
    {
        return $this->belongsToMany(Tag::class, 'rank_tracking_tags')
            ->withTimestamps();
    }

    /**
     * Get activities.
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Scope for active tracking.
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true)
            ->where('is_tracked', true);
    }

    /**
     * Scope for ranking keywords.
     */
    public function scopeRanking(Builder $query): Builder
    {
        return $query->whereNotNull('current_position')
            ->where('current_position', '>', 0);
    }

    /**
     * Scope for top positions.
     */
    public function scopeTopPositions(Builder $query, int $limit = 10): Builder
    {
        return $query->ranking()
            ->where('current_position', '<=', $limit);
    }

    /**
     * Scope for improved rankings.
     */
    public function scopeImproved(Builder $query): Builder
    {
        return $query->whereNotNull('previous_position')
            ->whereColumn('current_position', '<', 'previous_position');
    }

    /**
     * Scope for declined rankings.
     */
    public function scopeDeclined(Builder $query): Builder
    {
        return $query->whereNotNull('previous_position')
            ->whereColumn('current_position', '>', 'previous_position');
    }

    /**
     * Scope for specific search engine.
     */
    public function scopeForSearchEngine(Builder $query, string $engine): Builder
    {
        return $query->where('search_engine', $engine);
    }

    /**
     * Scope for specific device.
     */
    public function scopeForDevice(Builder $query, string $device): Builder
    {
        return $query->where('device', $device);
    }

    /**
     * Scope for specific location.
     */
    public function scopeForLocation(Builder $query, string $location): Builder
    {
        return $query->where('location', $location);
    }

    /**
     * Scope for keywords needing check.
     */
    public function scopeNeedsCheck(Builder $query): Builder
    {
        return $query->active()
            ->where(function ($q) {
                $q->whereNull('last_checked_at')
                    ->orWhere('next_check_at', '<=', now());
            });
    }

    /**
     * Get ranking status.
     */
    protected function isRanking(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->current_position !== null && $this->current_position > 0
        );
    }

    /**
     * Get improvement status.
     */
    protected function isImproving(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->position_change < 0 && $this->previous_position !== null
        );
    }

    /**
     * Get decline status.
     */
    protected function isDeclining(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->position_change > 0 && $this->previous_position !== null
        );
    }

    /**
     * Get stability status.
     */
    protected function isStable(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->position_change === 0 && $this->previous_position !== null
        );
    }

    /**
     * Get top 3 status.
     */
    protected function isTop3(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_ranking && $this->current_position <= 3
        );
    }

    /**
     * Get top 10 status.
     */
    protected function isTop10(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_ranking && $this->current_position <= 10
        );
    }

    /**
     * Get first page status.
     */
    protected function isFirstPage(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_ranking && $this->current_position <= 10
        );
    }

    /**
     * Get position trend.
     */
    protected function positionTrend(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculatePositionTrend()
        );
    }

    /**
     * Get visibility change.
     */
    protected function visibilityChange(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateVisibilityChange()
        );
    }

    /**
     * Get days since last check.
     */
    protected function daysSinceLastCheck(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->last_checked_at 
                ? $this->last_checked_at->diffInDays(now()) 
                : null
        );
    }

    /**
     * Get performance score.
     */
    protected function performanceScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculatePerformanceScore()
        );
    }

    /**
     * Get opportunity score.
     */
    protected function opportunityScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateOpportunityScore()
        );
    }

    /**
     * Update position.
     */
    public function updatePosition(?int $newPosition, array $serpData = []): void
    {
        $oldPosition = $this->current_position;
        
        $this->update([
            'previous_position' => $oldPosition,
            'current_position' => $newPosition,
            'position_change' => $newPosition && $oldPosition 
                ? $newPosition - $oldPosition 
                : 0,
            'position_change_percentage' => $this->calculateChangePercentage($oldPosition, $newPosition),
            'last_checked_at' => now(),
            'last_seen_at' => $newPosition ? now() : $this->last_seen_at,
            'lost_ranking_at' => !$newPosition && $oldPosition ? now() : null,
            'recovered_ranking_at' => $newPosition && !$oldPosition ? now() : null,
            'check_count' => $this->check_count + 1,
            'consecutive_failures' => $newPosition === null 
                ? $this->consecutive_failures + 1 
                : 0,
        ]);

        // Update best/worst positions
        $this->updateBestWorstPositions($newPosition);
        
        // Create history record
        $this->createHistoryRecord($newPosition, $serpData);
        
        // Update SERP features
        if (!empty($serpData)) {
            $this->updateSERPFeatures($serpData);
        }
        
        // Update metrics
        $this->updateMetrics();
    }

    /**
     * Calculate change percentage.
     */
    protected function calculateChangePercentage(?int $oldPosition, ?int $newPosition): float
    {
        if (!$oldPosition || !$newPosition) {
            return 0;
        }
        
        return round((($oldPosition - $newPosition) / $oldPosition) * 100, 2);
    }

    /**
     * Update best/worst positions.
     */
    protected function updateBestWorstPositions(?int $position): void
    {
        if ($position === null) {
            return;
        }
        
        if ($this->best_position === null || $position < $this->best_position) {
            $this->best_position = $position;
        }
        
        if ($this->worst_position === null || $position > $this->worst_position) {
            $this->worst_position = $position;
        }
        
        // Update average position
        $avgPosition = $this->histories()
            ->whereNotNull('position')
            ->avg('position');
        
        $this->average_position = round($avgPosition, 2);
        $this->save();
    }

    /**
     * Create history record.
     */
    protected function createHistoryRecord(?int $position, array $serpData): void
    {
        $this->histories()->create([
            'position' => $position,
            'date' => now()->toDateString(),
            'checked_at' => now(),
            'serp_data' => $serpData,
            'ranking_url' => $serpData['ranking_url'] ?? null,
            'title' => $serpData['title'] ?? null,
            'description' => $serpData['description'] ?? null,
            'serp_features' => $serpData['features'] ?? [],
            'competitors' => $serpData['competitors'] ?? [],
        ]);
    }

    /**
     * Update SERP features.
     */
    protected function updateSERPFeatures(array $serpData): void
    {
        $this->update([
            'serp_features' => $serpData['features'] ?? [],
            'has_featured_snippet' => in_array(SERPFeature::FEATURED_SNIPPET, $serpData['features'] ?? []),
            'has_knowledge_panel' => in_array(SERPFeature::KNOWLEDGE_PANEL, $serpData['features'] ?? []),
            'has_local_pack' => in_array(SERPFeature::LOCAL_PACK, $serpData['features'] ?? []),
            'has_image_pack' => in_array(SERPFeature::IMAGE_PACK, $serpData['features'] ?? []),
            'has_video_carousel' => in_array(SERPFeature::VIDEO_CAROUSEL, $serpData['features'] ?? []),
            'has_people_also_ask' => in_array(SERPFeature::PEOPLE_ALSO_ASK, $serpData['features'] ?? []),
            'has_shopping_results' => in_array(SERPFeature::SHOPPING, $serpData['features'] ?? []),
            'featured_snippet_data' => $serpData['featured_snippet'] ?? null,
            'knowledge_panel_data' => $serpData['knowledge_panel'] ?? null,
            'local_pack_data' => $serpData['local_pack'] ?? null,
            'people_also_ask_data' => $serpData['people_also_ask'] ?? null,
            'ranking_url' => $serpData['ranking_url'] ?? null,
            'title' => $serpData['title'] ?? null,
            'description' => $serpData['description'] ?? null,
            'top_competitors' => $serpData['top_competitors'] ?? [],
            'ads_count' => $serpData['ads_count'] ?? 0,
            'organic_results_count' => $serpData['organic_count'] ?? 10,
        ]);
        
        // Create SERP snapshot
        $this->serpSnapshots()->create([
            'serp_html' => $serpData['html'] ?? null,
            'serp_data' => $serpData,
            'captured_at' => now(),
        ]);
    }

    /**
     * Handle position change.
     */
    protected function handlePositionChange(): void
    {
        $oldPosition = $this->getOriginal('current_position');
        $newPosition = $this->current_position;
        
        // Update position tracking days
        if ($newPosition) {
            if ($newPosition <= 3) {
                $this->days_in_top_3++;
            }
            if ($newPosition <= 10) {
                $this->days_in_top_10++;
            }
            if ($newPosition <= 100) {
                $this->days_in_top_100++;
            }
            $this->days_not_ranking = 0;
        } else {
            $this->days_not_ranking++;
        }
        
        // Calculate volatility
        $this->updateVolatilityScore();
    }

    /**
     * Fire position events.
     */
    protected function firePositionEvents(): void
    {
        $oldPosition = $this->getOriginal('current_position');
        $newPosition = $this->current_position;
        
        if (!$oldPosition && $newPosition) {
            event(new RankingRecovered($this));
        } elseif ($oldPosition && !$newPosition) {
            event(new RankingLost($this));
        } elseif ($oldPosition && $newPosition) {
            if ($newPosition < $oldPosition) {
                event(new RankingImproved($this, $oldPosition, $newPosition));
            } elseif ($newPosition > $oldPosition) {
                event(new RankingDropped($this, $oldPosition, $newPosition));
            }
        }
        
        // Check for featured snippet changes
        if ($this->wasChanged('has_featured_snippet')) {
            if ($this->has_featured_snippet) {
                event(new FeaturedSnippetGained($this));
            } else {
                event(new FeaturedSnippetLost($this));
            }
        }
    }

    /**
     * Check alerts.
     */
    protected function checkAlerts(): void
    {
        if (!$this->alert_on_position_change) {
            return;
        }
        
        $change = abs($this->position_change);
        
        if ($change >= $this->alert_threshold) {
            $this->alerts()->create([
                'type' => $this->position_change > 0 
                    ? AlertType::POSITION_DROP 
                    : AlertType::POSITION_GAIN,
                'severity' => $change >= 10 ? 'high' : 'medium',
                'message' => "Position changed by {$change} places",
                'old_value' => $this->previous_position,
                'new_value' => $this->current_position,
                'triggered_at' => now(),
            ]);
            
            $this->sendAlertNotifications();
        }
    }

    /**
     * Send alert notifications.
     */
    protected function sendAlertNotifications(): void
    {
        if (empty($this->notification_channels)) {
            return;
        }
        
        // Send notifications through configured channels
        foreach ($this->notification_channels as $channel) {
            // Implementation for each channel (email, slack, webhook, etc.)
        }
    }

    /**
     * Parse URL components.
     */
    protected function parseUrl(): void
    {
        $parsed = parse_url($this->url);
        
        $this->domain = $parsed['host'] ?? '';
        $this->path = $parsed['path'] ?? '/';
        $this->protocol = $parsed['scheme'] ?? 'https';
    }

    /**
     * Detect branded keyword.
     */
    protected function detectBrandedKeyword(): void
    {
        if (!$this->domain) {
            return;
        }
        
        $brandTerms = explode('.', $this->domain)[0];
        $this->is_branded = str_contains(
            strtolower($this->keyword),
            strtolower($brandTerms)
        );
    }

    /**
     * Schedule next check.
     */
    protected function scheduleNextCheck(): void
    {
        $this->next_check_at = match($this->tracking_frequency) {
            TrackingFrequency::HOURLY => now()->addHour(),
            TrackingFrequency::DAILY => now()->addDay(),
            TrackingFrequency::WEEKLY => now()->addWeek(),
            TrackingFrequency::MONTHLY => now()->addMonth(),
            default => now()->addDay(),
        };
    }

    /**
     * Check initial ranking.
     */
    protected function checkInitialRanking(): void
    {
        dispatch(function () {
            $checker = app(RankChecker::class);
            $result = $checker->check($this);
            
            if ($result['success']) {
                $this->updatePosition($result['position'], $result['serp_data']);
            }
        })->afterResponse();
    }

    /**
     * Analyze keyword.
     */
    protected function analyzeKeyword(): void
    {
        $analyzer = app(KeywordAnalyzer::class);
        $data = $analyzer->analyze($this->keyword);
        
        $this->update([
            'search_volume' => $data['search_volume'] ?? 0,
            'keyword_difficulty' => $data['difficulty'] ?? 0,
            'cpc' => $data['cpc'] ?? 0,
            'competition' => $data['competition'] ?? 0,
        ]);
    }

    /**
     * Calculate position trend.
     */
    protected function calculatePositionTrend(): string
    {
        $recentHistory = $this->histories()
            ->limit(7)
            ->pluck('position')
            ->reverse();
        
        if ($recentHistory->count() < 3) {
            return 'insufficient_data';
        }
        
        $trend = [];
        for ($i = 1; $i < $recentHistory->count(); $i++) {
            $trend[] = $recentHistory[$i] - $recentHistory[$i - 1];
        }
        
        $avgTrend = array_sum($trend) / count($trend);
        
        return match(true) {
            $avgTrend < -1 => 'improving',
            $avgTrend > 1 => 'declining',
            default => 'stable',
        };
    }

    /**
     * Calculate visibility change.
     */
    protected function calculateVisibilityChange(): float
    {
        $oldVisibility = $this->calculateVisibilityForPosition($this->previous_position);
        $newVisibility = $this->calculateVisibilityForPosition($this->current_position);
        
        return round($newVisibility - $oldVisibility, 2);
    }

    /**
     * Calculate visibility for position.
     */
    protected function calculateVisibilityForPosition(?int $position): float
    {
        if (!$position || $position > 20) {
            return 0;
        }
        
        // CTR curve based on position
        $ctrCurve = [
            1 => 28.5, 2 => 15.7, 3 => 11.0, 4 => 8.0, 5 => 7.2,
            6 => 5.1, 7 => 4.0, 8 => 3.2, 9 => 2.8, 10 => 2.5,
        ];
        
        $ctr = $ctrCurve[$position] ?? (2.0 / $position);
        
        return ($this->search_volume * $ctr) / 100;
    }

    /**
     * Update volatility score.
     */
    protected function updateVolatilityScore(): void
    {
        $recentChanges = $this->histories()
            ->limit(30)
            ->get()
            ->map(fn ($h, $i) => $i > 0 ? abs($h->position - $this->histories[$i-1]->position) : 0)
            ->filter()
            ->avg();
        
        $this->volatility_score = min(100, $recentChanges * 10);
        $this->stability_score = 100 - $this->volatility_score;
    }

    /**
     * Calculate performance score.
     */
    protected function calculatePerformanceScore(): float
    {
        $score = 0;
        
        // Position score (40%)
        if ($this->is_ranking) {
            $positionScore = max(0, 100 - ($this->current_position - 1) * 2);
            $score += $positionScore * 0.4;
        }
        
        // Visibility score (30%)
        $visibilityScore = min(100, $this->visibility_score);
        $score += $visibilityScore * 0.3;
        
        // Stability score (20%)
        $score += $this->stability_score * 0.2;
        
        // SERP features (10%)
        $featuresScore = count($this->serp_features ?? []) * 10;
        $score += min(10, $featuresScore);
        
        return round($score, 2);
    }

    /**
     * Calculate opportunity score.
     */
    protected function calculateOpportunityScore(): float
    {
        $score = 0;
        
        // Search volume opportunity (30%)
        $volumeScore = min(100, ($this->search_volume / 1000) * 10);
        $score += $volumeScore * 0.3;
        
        // Position improvement potential (40%)
        if ($this->is_ranking && $this->current_position > 1) {
            $improvementPotential = (100 - $this->current_position) / 100;
            $score += ($improvementPotential * 100) * 0.4;
        } elseif (!$this->is_ranking) {
            $score += 40; // Maximum potential if not ranking
        }
        
        // Competition factor (20%)
        $competitionScore = 100 - ($this->keyword_difficulty ?? 50);
        $score += $competitionScore * 0.2;
        
        // CPC value (10%)
        $cpcScore = min(100, $this->cpc * 10);
        $score += $cpcScore * 0.1;
        
        return round($score, 2);
    }

    /**
     * Update metrics.
     */
    protected function updateMetrics(): void
    {
        // Calculate estimated traffic
        $ctr = $this->calculateCTR();
        $this->estimated_traffic = round($this->search_volume * ($ctr / 100), 2);
        
        // Calculate estimated value
        $this->estimated_value = round($this->estimated_traffic * $this->cpc, 2);
        
        // Calculate share of voice
        $this->share_of_voice = $this->calculateShareOfVoice();
        
        // Update visibility score
        $this->visibility_score = $this->calculateVisibilityForPosition($this->current_position);
        
        $this->save();
        
        // Record metrics
        $this->metrics()->create([
            'position' => $this->current_position,
            'visibility' => $this->visibility_score,
            'traffic' => $this->estimated_traffic,
            'value' => $this->estimated_value,
            'share_of_voice' => $this->share_of_voice,
            'recorded_at' => now(),
        ]);
    }

    /**
     * Calculate CTR based on position.
     */
    protected function calculateCTR(): float
    {
        if (!$this->is_ranking) {
            return 0;
        }
        
        // Advanced CTR model considering SERP features
        $baseCTR = match(true) {
            $this->current_position === 1 => 28.5,
            $this->current_position === 2 => 15.7,
            $this->current_position === 3 => 11.0,
            $this->current_position <= 5 => 7.5,
            $this->current_position <= 10 => 3.5,
            $this->current_position <= 20 => 1.5,
            default => 0.5,
        };
        
        // Adjust for SERP features
        if ($this->has_featured_snippet && $this->current_position === 1) {
            $baseCTR += 10;
        }
        
        if ($this->has_ads_top && $this->current_position <= 3) {
            $baseCTR *= 0.8; // Reduce CTR due to ads
        }
        
        return $baseCTR;
    }

    /**
     * Calculate share of voice.
     */
    protected function calculateShareOfVoice(): float
    {
        if (!$this->is_ranking) {
            return 0;
        }
        
        $totalVisibility = DB::table('rank_trackings')
            ->where('keyword', $this->keyword)
            ->where('search_engine', $this->search_engine)
            ->where('location', $this->location)
            ->whereNotNull('current_position')
            ->where('current_position', '<=', 10)
            ->sum('visibility_score');
        
        if ($totalVisibility === 0) {
            return 0;
        }
        
        return round(($this->visibility_score / $totalVisibility) * 100, 2);
    }

    /**
     * Get ranking history for period.
     */
    public function getHistoryForPeriod(Carbon $start, Carbon $end): Collection
    {
        return Cache::tags(self::$cacheConfig['tags'])
            ->remember(
                "rank_history_{$this->id}_{$start->format('Y-m-d')}_{$end->format('Y-m-d')}",
                self::$cacheConfig['ttl'],
                fn () => $this->histories()
                    ->whereBetween('checked_at', [$start, $end])
                    ->get()
            );
    }

    /**
     * Get competitor analysis.
     */
    public function getCompetitorAnalysis(): array
    {
        $analyzer = app(CompetitorAnalyzer::class);
        return $analyzer->analyze($this);
    }

    /**
     * Clear cache.
     */
    protected function clearCache(): void
    {
        Cache::tags(self::$cacheConfig['tags'])->flush();
    }
}