<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Casts\Attribute;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\MorphMany;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;
use Carbon\Carbon;
use App\Traits\HasUuid;
use App\Traits\Immutable;
use App\Traits\HasMetadata;
use App\Contracts\Analyzable;
use App\Events\RankHistoryRecorded;
use App\Events\SignificantRankChange;
use App\Events\SERPFeatureDetected;
use App\Events\CompetitorChangeDetected;
use App\Enums\SearchEngine;
use App\Enums\DeviceType;
use App\Enums\RankChangeType;
use App\Enums\SERPFeature;
use App\Enums\SnapshotType;
use App\Services\SERP\HistoryAnalyzer;
use App\Services\SERP\TrendCalculator;
use App\Services\SERP\VolatilityAnalyzer;

/**
 * RankHistory Model
 * 
 * Enterprise-grade SERP history tracking with comprehensive analytics,
 * competitor monitoring, SERP feature evolution, and trend analysis.
 * 
 * @property int $id
 * @property string $uuid
 * @property int $rank_tracking_id
 * @property int|null $position
 * @property int|null $previous_position
 * @property int $position_change
 * @property float $position_change_percentage
 * @property string $change_type
 * @property Carbon $date
 * @property Carbon $checked_at
 * @property string $search_engine
 * @property string $search_engine_url
 * @property string $device_type
 * @property string $location
 * @property string|null $location_code
 * @property float|null $latitude
 * @property float|null $longitude
 * @property string|null $language
 * @property string|null $ranking_url
 * @property string|null $canonical_url
 * @property string|null $displayed_url
 * @property string|null $title
 * @property string|null $description
 * @property int|null $title_length
 * @property int|null $description_length
 * @property float|null $title_pixel_width
 * @property float|null $description_pixel_width
 * @property array|null $breadcrumbs
 * @property array|null $rich_snippets
 * @property array|null $schema_types
 * @property array|null $structured_data
 * @property array|null $serp_features
 * @property array|null $serp_features_above
 * @property array|null $serp_features_below
 * @property bool $has_featured_snippet
 * @property bool $owns_featured_snippet
 * @property array|null $featured_snippet_data
 * @property bool $has_knowledge_panel
 * @property array|null $knowledge_panel_data
 * @property bool $has_local_pack
 * @property int|null $local_pack_position
 * @property array|null $local_pack_data
 * @property bool $has_people_also_ask
 * @property array|null $people_also_ask_data
 * @property bool $has_related_searches
 * @property array|null $related_searches_data
 * @property bool $has_image_pack
 * @property array|null $image_pack_data
 * @property bool $has_video_carousel
 * @property array|null $video_carousel_data
 * @property bool $has_shopping_results
 * @property array|null $shopping_results_data
 * @property bool $has_ads_top
 * @property bool $has_ads_bottom
 * @property bool $has_ads_sidebar
 * @property int $ads_count_top
 * @property int $ads_count_bottom
 * @property int $ads_count_total
 * @property array|null $ads_data
 * @property int $organic_results_count
 * @property int $total_results_count
 * @property string|null $total_results_string
 * @property float|null $search_time_seconds
 * @property array|null $competitors
 * @property array|null $top_10_domains
 * @property array|null $position_distribution
 * @property float|null $average_position_top_10
 * @property float|null $domain_diversity_score
 * @property float|null $serp_volatility_score
 * @property float|null $click_potential_score
 * @property float|null $visibility_score
 * @property float|null $estimated_traffic
 * @property float|null $estimated_clicks
 * @property float|null $estimated_impressions
 * @property float|null $estimated_ctr
 * @property float|null $estimated_value
 * @property float|null $pixel_rank
 * @property float|null $pixel_height
 * @property float|null $above_fold_percentage
 * @property bool $is_mobile_friendly
 * @property bool $has_amp
 * @property bool $has_https
 * @property bool $has_www
 * @property array|null $technical_signals
 * @property array|null $user_signals
 * @property array|null $content_signals
 * @property array|null $link_signals
 * @property float|null $page_speed_score
 * @property float|null $core_web_vitals_score
 * @property array|null $lighthouse_metrics
 * @property string|null $snapshot_type
 * @property string|null $snapshot_url
 * @property string|null $screenshot_url
 * @property string|null $html_snapshot_path
 * @property int|null $html_snapshot_size
 * @property string|null $json_snapshot_path
 * @property array|null $raw_serp_data
 * @property array|null $parsed_serp_data
 * @property string|null $serp_hash
 * @property bool $is_personalized
 * @property array|null $personalization_factors
 * @property array|null $ab_test_variations
 * @property array|null $experimental_features
 * @property array|null $api_response_data
 * @property string|null $api_provider
 * @property float|null $api_cost
 * @property int|null $api_credits_used
 * @property float|null $response_time_ms
 * @property array|null $headers
 * @property array|null $cookies
 * @property string|null $user_agent
 * @property string|null $ip_address
 * @property string|null $proxy_used
 * @property array|null $error_data
 * @property bool $is_valid
 * @property bool $is_complete
 * @property bool $requires_recheck
 * @property array|null $quality_scores
 * @property array|null $relevance_scores
 * @property array|null $authority_scores
 * @property array|null $engagement_metrics
 * @property array|null $social_signals
 * @property array|null $brand_signals
 * @property array|null $semantic_analysis
 * @property array|null $entity_analysis
 * @property array|null $topic_modeling
 * @property array|null $sentiment_analysis
 * @property array|null $intent_classification
 * @property array|null $query_classification
 * @property array|null $weather_data
 * @property array|null $event_data
 * @property array|null $news_data
 * @property array|null $trend_data
 * @property array|null $seasonality_data
 * @property array|null $annotations
 * @property array|null $tags
 * @property array|null $metadata
 * @property array|null $custom_metrics
 * @property int|null $created_by
 * @property string|null $source
 * @property string|null $import_batch_id
 * @property Carbon $created_at
 * @property Carbon $updated_at
 * @property Carbon|null $deleted_at
 * 
 * @property-read \App\Models\RankTracking $rankTracking
 * @property-read \App\Models\User|null $creator
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\CompetitorHistory[] $competitorHistories
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\SERPFeatureHistory[] $featureHistories
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankHistoryAnalysis[] $analyses
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\RankHistoryNote[] $notes
 * @property-read \Illuminate\Database\Eloquent\Collection|\App\Models\Activity[] $activities
 */
class RankHistory extends Model implements Analyzable
{
    use HasFactory;
    use SoftDeletes;
    use HasUuid;
    use Immutable;
    use HasMetadata;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'rank_histories';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'uuid',
        'rank_tracking_id',
        'position',
        'previous_position',
        'position_change',
        'position_change_percentage',
        'change_type',
        'date',
        'checked_at',
        'search_engine',
        'search_engine_url',
        'device_type',
        'location',
        'location_code',
        'latitude',
        'longitude',
        'language',
        'ranking_url',
        'canonical_url',
        'displayed_url',
        'title',
        'description',
        'title_length',
        'description_length',
        'title_pixel_width',
        'description_pixel_width',
        'breadcrumbs',
        'rich_snippets',
        'schema_types',
        'structured_data',
        'serp_features',
        'serp_features_above',
        'serp_features_below',
        'has_featured_snippet',
        'owns_featured_snippet',
        'featured_snippet_data',
        'has_knowledge_panel',
        'knowledge_panel_data',
        'has_local_pack',
        'local_pack_position',
        'local_pack_data',
        'has_people_also_ask',
        'people_also_ask_data',
        'has_related_searches',
        'related_searches_data',
        'has_image_pack',
        'image_pack_data',
        'has_video_carousel',
        'video_carousel_data',
        'has_shopping_results',
        'shopping_results_data',
        'has_ads_top',
        'has_ads_bottom',
        'has_ads_sidebar',
        'ads_count_top',
        'ads_count_bottom',
        'ads_count_total',
        'ads_data',
        'organic_results_count',
        'total_results_count',
        'total_results_string',
        'search_time_seconds',
        'competitors',
        'top_10_domains',
        'position_distribution',
        'average_position_top_10',
        'domain_diversity_score',
        'serp_volatility_score',
        'click_potential_score',
        'visibility_score',
        'estimated_traffic',
        'estimated_clicks',
        'estimated_impressions',
        'estimated_ctr',
        'estimated_value',
        'pixel_rank',
        'pixel_height',
        'above_fold_percentage',
        'is_mobile_friendly',
        'has_amp',
        'has_https',
        'has_www',
        'technical_signals',
        'user_signals',
        'content_signals',
        'link_signals',
        'page_speed_score',
        'core_web_vitals_score',
        'lighthouse_metrics',
        'snapshot_type',
        'snapshot_url',
        'screenshot_url',
        'html_snapshot_path',
        'html_snapshot_size',
        'json_snapshot_path',
        'raw_serp_data',
        'parsed_serp_data',
        'serp_hash',
        'is_personalized',
        'personalization_factors',
        'ab_test_variations',
        'experimental_features',
        'api_response_data',
        'api_provider',
        'api_cost',
        'api_credits_used',
        'response_time_ms',
        'headers',
        'cookies',
        'user_agent',
        'ip_address',
        'proxy_used',
        'error_data',
        'is_valid',
        'is_complete',
        'requires_recheck',
        'quality_scores',
        'relevance_scores',
        'authority_scores',
        'engagement_metrics',
        'social_signals',
        'brand_signals',
        'semantic_analysis',
        'entity_analysis',
        'topic_modeling',
        'sentiment_analysis',
        'intent_classification',
        'query_classification',
        'weather_data',
        'event_data',
        'news_data',
        'trend_data',
        'seasonality_data',
        'annotations',
        'tags',
        'metadata',
        'custom_metrics',
        'created_by',
        'source',
        'import_batch_id',
    ];

    /**
     * The attributes that should be cast.
     *
     * @var array<string, string>
     */
    protected $casts = [
        'position' => 'integer',
        'previous_position' => 'integer',
        'position_change' => 'integer',
        'position_change_percentage' => 'float',
        'change_type' => RankChangeType::class,
        'date' => 'date',
        'checked_at' => 'datetime',
        'search_engine' => SearchEngine::class,
        'device_type' => DeviceType::class,
        'snapshot_type' => SnapshotType::class,
        'latitude' => 'float',
        'longitude' => 'float',
        'title_length' => 'integer',
        'description_length' => 'integer',
        'title_pixel_width' => 'float',
        'description_pixel_width' => 'float',
        'local_pack_position' => 'integer',
        'ads_count_top' => 'integer',
        'ads_count_bottom' => 'integer',
        'ads_count_total' => 'integer',
        'organic_results_count' => 'integer',
        'total_results_count' => 'integer',
        'search_time_seconds' => 'float',
        'average_position_top_10' => 'float',
        'domain_diversity_score' => 'float',
        'serp_volatility_score' => 'float',
        'click_potential_score' => 'float',
        'visibility_score' => 'float',
        'estimated_traffic' => 'float',
        'estimated_clicks' => 'float',
        'estimated_impressions' => 'float',
        'estimated_ctr' => 'float',
        'estimated_value' => 'decimal:2',
        'pixel_rank' => 'float',
        'pixel_height' => 'float',
        'above_fold_percentage' => 'float',
        'page_speed_score' => 'float',
        'core_web_vitals_score' => 'float',
        'html_snapshot_size' => 'integer',
        'api_cost' => 'decimal:4',
        'api_credits_used' => 'integer',
        'response_time_ms' => 'float',
        'has_featured_snippet' => 'boolean',
        'owns_featured_snippet' => 'boolean',
        'has_knowledge_panel' => 'boolean',
        'has_local_pack' => 'boolean',
        'has_people_also_ask' => 'boolean',
        'has_related_searches' => 'boolean',
        'has_image_pack' => 'boolean',
        'has_video_carousel' => 'boolean',
        'has_shopping_results' => 'boolean',
        'has_ads_top' => 'boolean',
        'has_ads_bottom' => 'boolean',
        'has_ads_sidebar' => 'boolean',
        'is_mobile_friendly' => 'boolean',
        'has_amp' => 'boolean',
        'has_https' => 'boolean',
        'has_www' => 'boolean',
        'is_personalized' => 'boolean',
        'is_valid' => 'boolean',
        'is_complete' => 'boolean',
        'requires_recheck' => 'boolean',
        'breadcrumbs' => 'array',
        'rich_snippets' => 'array',
        'schema_types' => 'array',
        'structured_data' => 'array',
        'serp_features' => 'array',
        'serp_features_above' => 'array',
        'serp_features_below' => 'array',
        'featured_snippet_data' => 'array',
        'knowledge_panel_data' => 'array',
        'local_pack_data' => 'array',
        'people_also_ask_data' => 'array',
        'related_searches_data' => 'array',
        'image_pack_data' => 'array',
        'video_carousel_data' => 'array',
        'shopping_results_data' => 'array',
        'ads_data' => 'array',
        'competitors' => 'array',
        'top_10_domains' => 'array',
        'position_distribution' => 'array',
        'technical_signals' => 'array',
        'user_signals' => 'array',
        'content_signals' => 'array',
        'link_signals' => 'array',
        'lighthouse_metrics' => 'array',
        'raw_serp_data' => 'array',
        'parsed_serp_data' => 'array',
        'personalization_factors' => 'array',
        'ab_test_variations' => 'array',
        'experimental_features' => 'array',
        'api_response_data' => 'array',
        'headers' => 'array',
        'cookies' => 'array',
        'error_data' => 'array',
        'quality_scores' => 'array',
        'relevance_scores' => 'array',
        'authority_scores' => 'array',
        'engagement_metrics' => 'array',
        'social_signals' => 'array',
        'brand_signals' => 'array',
        'semantic_analysis' => 'array',
        'entity_analysis' => 'array',
        'topic_modeling' => 'array',
        'sentiment_analysis' => 'array',
        'intent_classification' => 'array',
        'query_classification' => 'array',
        'weather_data' => 'array',
        'event_data' => 'array',
        'news_data' => 'array',
        'trend_data' => 'array',
        'seasonality_data' => 'array',
        'annotations' => 'array',
        'tags' => 'array',
        'metadata' => 'array',
        'custom_metrics' => 'array',
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
        'raw_serp_data',
        'api_response_data',
        'cookies',
        'ip_address',
        'proxy_used',
        'deleted_at',
    ];

    /**
     * The attributes that should be appended.
     *
     * @var array<string>
     */
    protected $appends = [
        'is_ranking',
        'is_first_page',
        'is_top_3',
        'position_label',
        'change_label',
        'change_icon',
        'serp_feature_count',
        'competitor_count',
        'quality_score',
        'days_ago',
        'formatted_date',
    ];

    /**
     * Default attribute values.
     *
     * @var array<string, mixed>
     */
    protected $attributes = [
        'position_change' => 0,
        'position_change_percentage' => 0,
        'change_type' => RankChangeType::STABLE,
        'search_engine' => SearchEngine::GOOGLE,
        'device_type' => DeviceType::DESKTOP,
        'snapshot_type' => SnapshotType::API,
        'ads_count_top' => 0,
        'ads_count_bottom' => 0,
        'ads_count_total' => 0,
        'organic_results_count' => 10,
        'total_results_count' => 0,
        'search_time_seconds' => 0,
        'visibility_score' => 0,
        'estimated_traffic' => 0,
        'estimated_clicks' => 0,
        'estimated_impressions' => 0,
        'estimated_ctr' => 0,
        'estimated_value' => 0,
        'is_valid' => true,
        'is_complete' => true,
        'requires_recheck' => false,
        'has_featured_snippet' => false,
        'owns_featured_snippet' => false,
        'has_knowledge_panel' => false,
        'has_local_pack' => false,
        'has_people_also_ask' => false,
        'has_related_searches' => false,
        'has_image_pack' => false,
        'has_video_carousel' => false,
        'has_shopping_results' => false,
        'has_ads_top' => false,
        'has_ads_bottom' => false,
        'has_ads_sidebar' => false,
        'is_mobile_friendly' => false,
        'has_amp' => false,
        'has_https' => true,
        'has_www' => false,
        'is_personalized' => false,
    ];

    /**
     * Cache configuration.
     */
    protected static array $cacheConfig = [
        'tags' => ['rank_history'],
        'ttl' => 3600,
    ];

    /**
     * Boot the model.
     */
    protected static function boot(): void
    {
        parent::boot();

        static::creating(function (self $history) {
            $history->uuid = $history->uuid ?? (string) Str::uuid();
            $history->checked_at = $history->checked_at ?? now();
            $history->created_by = $history->created_by ?? auth()->id();
            
            $history->calculateChanges();
            $history->analyzeSerp();
            $history->estimateMetrics();
        });

        static::created(function (self $history) {
            event(new RankHistoryRecorded($history));
            
            $history->detectSignificantChanges();
            $history->analyzeSerpFeatures();
            $history->detectCompetitorChanges();
        });
    }

    /**
     * Get the rank tracking record.
     */
    public function rankTracking(): BelongsTo
    {
        return $this->belongsTo(RankTracking::class);
    }

    /**
     * Get the creator.
     */
    public function creator(): BelongsTo
    {
        return $this->belongsTo(User::class, 'created_by');
    }

    /**
     * Get competitor histories.
     */
    public function competitorHistories(): HasMany
    {
        return $this->hasMany(CompetitorHistory::class);
    }

    /**
     * Get SERP feature histories.
     */
    public function featureHistories(): HasMany
    {
        return $this->hasMany(SERPFeatureHistory::class);
    }

    /**
     * Get analyses.
     */
    public function analyses(): HasMany
    {
        return $this->hasMany(RankHistoryAnalysis::class);
    }

    /**
     * Get notes.
     */
    public function notes(): HasMany
    {
        return $this->hasMany(RankHistoryNote::class);
    }

    /**
     * Get activity logs.
     */
    public function activities(): MorphMany
    {
        return $this->morphMany(Activity::class, 'subject');
    }

    /**
     * Scope for ranking positions.
     */
    public function scopeRanking(Builder $query): Builder
    {
        return $query->whereNotNull('position')
            ->where('position', '>', 0);
    }

    /**
     * Scope for first page results.
     */
    public function scopeFirstPage(Builder $query): Builder
    {
        return $query->ranking()
            ->where('position', '<=', 10);
    }

    /**
     * Scope for top 3 positions.
     */
    public function scopeTop3(Builder $query): Builder
    {
        return $query->ranking()
            ->where('position', '<=', 3);
    }

    /**
     * Scope for specific date range.
     */
    public function scopeDateRange(Builder $query, Carbon $start, Carbon $end): Builder
    {
        return $query->whereBetween('date', [$start->toDateString(), $end->toDateString()]);
    }

    /**
     * Scope for improved positions.
     */
    public function scopeImproved(Builder $query): Builder
    {
        return $query->where('change_type', RankChangeType::IMPROVED);
    }

    /**
     * Scope for declined positions.
     */
    public function scopeDeclined(Builder $query): Builder
    {
        return $query->where('change_type', RankChangeType::DECLINED);
    }

    /**
     * Scope for with SERP features.
     */
    public function scopeWithSerpFeatures(Builder $query): Builder
    {
        return $query->where(function ($q) {
            $q->where('has_featured_snippet', true)
                ->orWhere('has_knowledge_panel', true)
                ->orWhere('has_local_pack', true)
                ->orWhere('has_people_also_ask', true)
                ->orWhere('has_image_pack', true)
                ->orWhere('has_video_carousel', true);
        });
    }

    /**
     * Get ranking status.
     */
    protected function isRanking(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->position !== null && $this->position > 0
        );
    }

    /**
     * Get first page status.
     */
    protected function isFirstPage(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_ranking && $this->position <= 10
        );
    }

    /**
     * Get top 3 status.
     */
    protected function isTop3(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->is_ranking && $this->position <= 3
        );
    }

    /**
     * Get position label.
     */
    protected function positionLabel(): Attribute
    {
        return Attribute::make(
            get: function () {
                if (!$this->is_ranking) {
                    return 'Not Ranking';
                }
                
                if ($this->position === 1) {
                    return '#1';
                } elseif ($this->position <= 3) {
                    return "Top 3 (#{$this->position})";
                } elseif ($this->position <= 10) {
                    return "Page 1 (#{$this->position})";
                } else {
                    $page = ceil($this->position / 10);
                    return "Page {$page} (#{$this->position})";
                }
            }
        );
    }

    /**
     * Get change label.
     */
    protected function changeLabel(): Attribute
    {
        return Attribute::make(
            get: function () {
                if ($this->position_change === 0) {
                    return 'No change';
                }
                
                $direction = $this->position_change > 0 ? '↓' : '↑';
                $amount = abs($this->position_change);
                
                return "{$direction} {$amount}";
            }
        );
    }

    /**
     * Get change icon.
     */
    protected function changeIcon(): Attribute
    {
        return Attribute::make(
            get: function () {
                return match($this->change_type) {
                    RankChangeType::IMPROVED => '↑',
                    RankChangeType::DECLINED => '↓',
                    RankChangeType::NEW_ENTRY => '★',
                    RankChangeType::LOST => '✗',
                    RankChangeType::RECOVERED => '↺',
                    default => '—',
                };
            }
        );
    }

    /**
     * Get SERP feature count.
     */
    protected function serpFeatureCount(): Attribute
    {
        return Attribute::make(
            get: fn () => count($this->serp_features ?? [])
        );
    }

    /**
     * Get competitor count.
     */
    protected function competitorCount(): Attribute
    {
        return Attribute::make(
            get: fn () => count($this->competitors ?? [])
        );
    }

    /**
     * Get quality score.
     */
    protected function qualityScore(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->calculateQualityScore()
        );
    }

    /**
     * Get days ago.
     */
    protected function daysAgo(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->date->diffInDays(now())
        );
    }

    /**
     * Get formatted date.
     */
    protected function formattedDate(): Attribute
    {
        return Attribute::make(
            get: fn () => $this->date->format('M d, Y')
        );
    }

    /**
     * Calculate position changes.
     */
    protected function calculateChanges(): void
    {
        if ($this->previous_position === null) {
            $previousHistory = self::where('rank_tracking_id', $this->rank_tracking_id)
                ->where('date', '<', $this->date)
                ->orderBy('date', 'desc')
                ->first();
            
            $this->previous_position = $previousHistory?->position;
        }
        
        if ($this->previous_position !== null && $this->position !== null) {
            $this->position_change = $this->position - $this->previous_position;
            $this->position_change_percentage = $this->previous_position > 0
                ? round((abs($this->position_change) / $this->previous_position) * 100, 2)
                : 0;
            
            $this->change_type = match(true) {
                $this->position_change < 0 => RankChangeType::IMPROVED,
                $this->position_change > 0 => RankChangeType::DECLINED,
                default => RankChangeType::STABLE,
            };
        } elseif ($this->previous_position === null && $this->position !== null) {
            $this->change_type = RankChangeType::NEW_ENTRY;
        } elseif ($this->previous_position !== null && $this->position === null) {
            $this->change_type = RankChangeType::LOST;
        }
    }

    /**
     * Analyze SERP data.
     */
    protected function analyzeSerp(): void
    {
        if (empty($this->raw_serp_data)) {
            return;
        }
        
        $analyzer = app(SERPAnalyzer::class);
        $analysis = $analyzer->analyze($this->raw_serp_data);
        
        $this->fill([
            'serp_features' => $analysis['features'],
            'competitors' => $analysis['competitors'],
            'top_10_domains' => $analysis['top_10_domains'],
            'domain_diversity_score' => $analysis['diversity_score'],
            'serp_volatility_score' => $analysis['volatility_score'],
            'average_position_top_10' => $analysis['avg_position'],
        ]);
    }

    /**
     * Estimate traffic and value metrics.
     */
    protected function estimateMetrics(): void
    {
        if (!$this->is_ranking || !$this->rankTracking) {
            return;
        }
        
        $searchVolume = $this->rankTracking->search_volume ?? 0;
        $cpc = $this->rankTracking->cpc ?? 0;
        
        // CTR estimation based on position
        $ctr = $this->estimateCTR();
        
        $this->estimated_impressions = $searchVolume;
        $this->estimated_ctr = $ctr;
        $this->estimated_clicks = round($searchVolume * ($ctr / 100), 2);
        $this->estimated_traffic = $this->estimated_clicks;
        $this->estimated_value = round($this->estimated_clicks * $cpc, 2);
        
        // Calculate visibility score
        $this->visibility_score = $this->calculateVisibilityScore();
        
        // Calculate click potential
        $this->click_potential_score = $this->calculateClickPotential();
    }

    /**
     * Estimate CTR based on position and SERP features.
     */
    protected function estimateCTR(): float
    {
        if (!$this->is_ranking) {
            return 0;
        }
        
        // Base CTR curve
        $baseCTR = match(true) {
            $this->position === 1 => 28.5,
            $this->position === 2 => 15.7,
            $this->position === 3 => 11.0,
            $this->position === 4 => 8.0,
            $this->position === 5 => 7.2,
            $this->position <= 10 => 4.5 - (($this->position - 6) * 0.5),
            default => max(0.5, 20 / $this->position),
        };
        
        // Adjust for SERP features
        $adjustment = 1.0;
        
        if ($this->has_ads_top && $this->position <= 4) {
            $adjustment *= 0.85; // Reduce CTR due to ads
        }
        
        if ($this->owns_featured_snippet) {
            $adjustment *= 1.5; // Increase for featured snippet
        } elseif ($this->has_featured_snippet && $this->position === 1) {
            $adjustment *= 0.7; // Reduce if featured snippet exists but not owned
        }
        
        if ($this->has_local_pack && $this->position > 3) {
            $adjustment *= 0.9; // Reduce if local pack pushes down
        }
        
        return round($baseCTR * $adjustment, 2);
    }

    /**
     * Calculate visibility score.
     */
    protected function calculateVisibilityScore(): float
    {
        if (!$this->is_ranking) {
            return 0;
        }
        
        $baseScore = max(0, 100 - (($this->position - 1) * 5));
        
        // Bonus for SERP features
        if ($this->owns_featured_snippet) $baseScore += 20;
        if ($this->position <= 3) $baseScore += 10;
        if ($this->has_rich_snippets) $baseScore += 5;
        
        // Penalty for competition
        if ($this->ads_count_top > 2) $baseScore -= 10;
        if ($this->serp_feature_count > 5) $baseScore -= 5;
        
        return min(100, max(0, $baseScore));
    }

    /**
     * Calculate click potential score.
     */
    protected function calculateClickPotential(): float
    {
        $potential = 100;
        
        // Reduce potential based on position
        if ($this->is_ranking) {
            $potential -= min(50, $this->position * 2);
        } else {
            return 0;
        }
        
        // Adjust for pixel rank
        if ($this->pixel_rank) {
            $pixelPenalty = min(30, $this->pixel_rank / 100);
            $potential -= $pixelPenalty;
        }
        
        // Adjust for above fold percentage
        if ($this->above_fold_percentage !== null) {
            $potential *= ($this->above_fold_percentage / 100);
        }
        
        return max(0, min(100, $potential));
    }

    /**
     * Calculate overall quality score.
     */
    protected function calculateQualityScore(): float
    {
        $score = 0;
        $factors = 0;
        
        // Data completeness (25%)
        if ($this->is_complete) {
            $score += 25;
            $factors++;
        }
        
        // Position quality (25%)
        if ($this->is_ranking) {
            $positionScore = max(0, 25 - ($this->position * 0.5));
            $score += $positionScore;
            $factors++;
        }
        
        // SERP features (25%)
        if ($this->serp_feature_count > 0) {
            $featureScore = min(25, $this->serp_feature_count * 5);
            $score += $featureScore;
            $factors++;
        }
        
        // Technical signals (25%)
        $technicalScore = 0;
        if ($this->has_https) $technicalScore += 6.25;
        if ($this->is_mobile_friendly) $technicalScore += 6.25;
        if ($this->page_speed_score > 50) $technicalScore += 6.25;
        if ($this->core_web_vitals_score > 50) $technicalScore += 6.25;
        $score += $technicalScore;
        if ($technicalScore > 0) $factors++;
        
        return $factors > 0 ? round($score / $factors, 2) : 0;
    }

    /**
     * Detect significant changes.
     */
    protected function detectSignificantChanges(): void
    {
        if (abs($this->position_change) >= 5) {
            event(new SignificantRankChange($this));
        }
        
        if ($this->change_type === RankChangeType::NEW_ENTRY && $this->position <= 10) {
            $this->createAnnotation('First page entry', 'success');
        }
        
        if ($this->change_type === RankChangeType::LOST && $this->previous_position <= 10) {
            $this->createAnnotation('Lost first page ranking', 'warning');
        }
    }

    /**
     * Analyze SERP features.
     */
    protected function analyzeSerpFeatures(): void
    {
        foreach ($this->serp_features ?? [] as $feature) {
            event(new SERPFeatureDetected($this, $feature));
            
            $this->featureHistories()->create([
                'feature_type' => $feature,
                'is_owned' => $this->checkFeatureOwnership($feature),
                'data' => $this->getFeatureData($feature),
                'detected_at' => $this->checked_at,
            ]);
        }
    }

    /**
     * Check if feature is owned.
     */
    protected function checkFeatureOwnership(string $feature): bool
    {
        return match($feature) {
            SERPFeature::FEATURED_SNIPPET => $this->owns_featured_snippet,
            SERPFeature::LOCAL_PACK => $this->local_pack_position !== null,
            default => false,
        };
    }

    /**
     * Get feature data.
     */
    protected function getFeatureData(string $feature): ?array
    {
        return match($feature) {
            SERPFeature::FEATURED_SNIPPET => $this->featured_snippet_data,
            SERPFeature::KNOWLEDGE_PANEL => $this->knowledge_panel_data,
            SERPFeature::LOCAL_PACK => $this->local_pack_data,
            SERPFeature::PEOPLE_ALSO_ASK => $this->people_also_ask_data,
            SERPFeature::IMAGE_PACK => $this->image_pack_data,
            SERPFeature::VIDEO_CAROUSEL => $this->video_carousel_data,
            default => null,
        };
    }

    /**
     * Detect competitor changes.
     */
    protected function detectCompetitorChanges(): void
    {
        if (empty($this->competitors)) {
            return;
        }
        
        $previousHistory = self::where('rank_tracking_id', $this->rank_tracking_id)
            ->where('date', '<', $this->date)
            ->orderBy('date', 'desc')
            ->first();
        
        if (!$previousHistory || empty($previousHistory->competitors)) {
            return;
        }
        
        $newCompetitors = array_diff(
            array_column($this->competitors, 'domain'),
            array_column($previousHistory->competitors, 'domain')
        );
        
        if (!empty($newCompetitors)) {
            event(new CompetitorChangeDetected($this, $newCompetitors));
        }
    }

    /**
     * Create annotation.
     */
    public function createAnnotation(string $note, string $type = 'info'): void
    {
        $this->notes()->create([
            'note' => $note,
            'type' => $type,
            'created_by' => auth()->id(),
            'created_at' => now(),
        ]);
    }

    /**
     * Get trend for period.
     */
    public function getTrendForPeriod(int $days = 30): array
    {
        $calculator = app(TrendCalculator::class);
        return $calculator->calculate($this->rank_tracking_id, $this->date->subDays($days), $this->date);
    }

    /**
     * Get volatility analysis.
     */
    public function getVolatilityAnalysis(): array
    {
        $analyzer = app(VolatilityAnalyzer::class);
        return $analyzer->analyze($this->rank_tracking_id, $this->date);
    }

    /**
     * Export to array with formatted data.
     */
    public function toExportArray(): array
    {
        return [
            'date' => $this->date->format('Y-m-d'),
            'position' => $this->position,
            'change' => $this->position_change,
            'url' => $this->ranking_url,
            'title' => $this->title,
            'description' => $this->description,
            'serp_features' => implode(', ', $this->serp_features ?? []),
            'visibility_score' => $this->visibility_score,
            'estimated_traffic' => $this->estimated_traffic,
            'estimated_value' => $this->estimated_value,
        ];
    }
}