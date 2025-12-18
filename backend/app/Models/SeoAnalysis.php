<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\MorphTo;
use Illuminate\Database\Eloquent\Builder;
use InvalidArgumentException;

/**
 * SEO Analysis Model
 * 
 * Stores comprehensive SEO analysis data for any analyzable entity (posts, pages, products, etc.).
 * Tracks metrics including keyword optimization, readability, meta tags, and overall SEO health.
 *
 * @property int $id
 * @property string $analyzable_type
 * @property int $analyzable_id
 * @property string|null $focus_keyword
 * @property int $seo_score Score from 0-100
 * @property array $analysis_results Detailed analysis breakdown
 * @property array $suggestions Actionable improvement suggestions
 * @property float $keyword_density Percentage (0-100)
 * @property int $readability_score Flesch reading ease score (0-100)
 * @property bool $has_meta_title
 * @property bool $has_meta_description
 * @property int|null $word_count
 * @property int|null $heading_count
 * @property int|null $image_count
 * @property int|null $internal_links_count
 * @property int|null $external_links_count
 * @property bool $has_focus_keyword_in_title
 * @property bool $has_focus_keyword_in_meta
 * @property bool $has_focus_keyword_in_url
 * @property \Illuminate\Support\Carbon $created_at
 * @property \Illuminate\Support\Carbon $updated_at
 * @property-read \Illuminate\Database\Eloquent\Model $analyzable
 */
class SeoAnalysis extends Model
{
    use HasFactory;

    /**
     * SEO Score thresholds
     */
    public const SCORE_EXCELLENT = 90;
    public const SCORE_GOOD = 70;
    public const SCORE_NEEDS_IMPROVEMENT = 50;
    public const SCORE_POOR = 0;

    /**
     * SEO Score categories
     */
    public const CATEGORY_EXCELLENT = 'excellent';
    public const CATEGORY_GOOD = 'good';
    public const CATEGORY_NEEDS_IMPROVEMENT = 'needs_improvement';
    public const CATEGORY_POOR = 'poor';

    /**
     * Keyword density thresholds (%)
     */
    public const KEYWORD_DENSITY_OPTIMAL_MIN = 0.5;
    public const KEYWORD_DENSITY_OPTIMAL_MAX = 2.5;
    public const KEYWORD_DENSITY_SPAM_THRESHOLD = 5.0;

    /**
     * Analysis result keys
     */
    public const ANALYSIS_TITLE = 'title';
    public const ANALYSIS_META_DESCRIPTION = 'meta_description';
    public const ANALYSIS_KEYWORD = 'keyword';
    public const ANALYSIS_CONTENT = 'content';
    public const ANALYSIS_READABILITY = 'readability';
    public const ANALYSIS_IMAGES = 'images';
    public const ANALYSIS_LINKS = 'links';
    public const ANALYSIS_TECHNICAL = 'technical';

    protected $fillable = [
        'analyzable_type',
        'analyzable_id',
        'focus_keyword',
        'seo_score',
        'analysis_results',
        'suggestions',
        'keyword_density',
        'readability_score',
        'has_meta_title',
        'has_meta_description',
        'word_count',
        'heading_count',
        'image_count',
        'internal_links_count',
        'external_links_count',
        'has_focus_keyword_in_title',
        'has_focus_keyword_in_meta',
        'has_focus_keyword_in_url',
    ];

    protected $casts = [
        'seo_score' => 'integer',
        'analysis_results' => 'array',
        'suggestions' => 'array',
        'keyword_density' => 'float',
        'readability_score' => 'integer',
        'has_meta_title' => 'boolean',
        'has_meta_description' => 'boolean',
        'word_count' => 'integer',
        'heading_count' => 'integer',
        'image_count' => 'integer',
        'internal_links_count' => 'integer',
        'external_links_count' => 'integer',
        'has_focus_keyword_in_title' => 'boolean',
        'has_focus_keyword_in_meta' => 'boolean',
        'has_focus_keyword_in_url' => 'boolean',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    protected $attributes = [
        'seo_score' => 0,
        'analysis_results' => '[]',
        'suggestions' => '[]',
        'keyword_density' => 0.0,
        'readability_score' => 0,
        'has_meta_title' => false,
        'has_meta_description' => false,
        'word_count' => 0,
        'heading_count' => 0,
        'image_count' => 0,
        'internal_links_count' => 0,
        'external_links_count' => 0,
        'has_focus_keyword_in_title' => false,
        'has_focus_keyword_in_meta' => false,
        'has_focus_keyword_in_url' => false,
    ];

    /**
     * Boot the model
     */
    protected static function boot(): void
    {
        parent::boot();

        static::saving(function (self $model): void {
            $model->validateSeoScore();
            $model->validateKeywordDensity();
            $model->validateReadabilityScore();
            $model->ensureArraysAreValid();
        });
    }

    /**
     * Get the parent analyzable model
     */
    public function analyzable(): MorphTo
    {
        return $this->morphTo();
    }

    /**
     * Validate SEO score is within valid range
     */
    protected function validateSeoScore(): void
    {
        if ($this->seo_score < 0 || $this->seo_score > 100) {
            throw new InvalidArgumentException(
                sprintf('SEO score must be between 0 and 100, got: %d', $this->seo_score)
            );
        }
    }

    /**
     * Validate keyword density is within valid range
     */
    protected function validateKeywordDensity(): void
    {
        if ($this->keyword_density < 0 || $this->keyword_density > 100) {
            throw new InvalidArgumentException(
                sprintf('Keyword density must be between 0 and 100, got: %.2f', $this->keyword_density)
            );
        }
    }

    /**
     * Validate readability score is within valid range
     */
    protected function validateReadabilityScore(): void
    {
        if ($this->readability_score < 0 || $this->readability_score > 100) {
            throw new InvalidArgumentException(
                sprintf('Readability score must be between 0 and 100, got: %d', $this->readability_score)
            );
        }
    }

    /**
     * Ensure array fields are valid
     */
    protected function ensureArraysAreValid(): void
    {
        if (!is_array($this->analysis_results)) {
            $this->analysis_results = [];
        }

        if (!is_array($this->suggestions)) {
            $this->suggestions = [];
        }
    }

    /**
     * Get SEO score category
     */
    public function getScoreCategoryAttribute(): string
    {
        return match(true) {
            $this->seo_score >= self::SCORE_EXCELLENT => self::CATEGORY_EXCELLENT,
            $this->seo_score >= self::SCORE_GOOD => self::CATEGORY_GOOD,
            $this->seo_score >= self::SCORE_NEEDS_IMPROVEMENT => self::CATEGORY_NEEDS_IMPROVEMENT,
            default => self::CATEGORY_POOR,
        };
    }

    /**
     * Get human-readable score category
     */
    public function getScoreCategoryLabelAttribute(): string
    {
        return match($this->score_category) {
            self::CATEGORY_EXCELLENT => 'Excellent',
            self::CATEGORY_GOOD => 'Good',
            self::CATEGORY_NEEDS_IMPROVEMENT => 'Needs Improvement',
            self::CATEGORY_POOR => 'Poor',
        };
    }

    /**
     * Get score category color for UI
     */
    public function getScoreCategoryColorAttribute(): string
    {
        return match($this->score_category) {
            self::CATEGORY_EXCELLENT => '#22c55e', // green-500
            self::CATEGORY_GOOD => '#84cc16', // lime-500
            self::CATEGORY_NEEDS_IMPROVEMENT => '#f59e0b', // amber-500
            self::CATEGORY_POOR => '#ef4444', // red-500
        };
    }

    /**
     * Check if keyword density is optimal
     */
    public function isKeywordDensityOptimal(): bool
    {
        return $this->keyword_density >= self::KEYWORD_DENSITY_OPTIMAL_MIN
            && $this->keyword_density <= self::KEYWORD_DENSITY_OPTIMAL_MAX;
    }

    /**
     * Check if keyword density indicates spam
     */
    public function isKeywordDensitySpammy(): bool
    {
        return $this->keyword_density >= self::KEYWORD_DENSITY_SPAM_THRESHOLD;
    }

    /**
     * Get keyword density status
     */
    public function getKeywordDensityStatusAttribute(): string
    {
        return match(true) {
            $this->isKeywordDensitySpammy() => 'too_high',
            $this->isKeywordDensityOptimal() => 'optimal',
            $this->keyword_density < self::KEYWORD_DENSITY_OPTIMAL_MIN => 'too_low',
            default => 'suboptimal',
        };
    }

    /**
     * Check if readability is good (Flesch score 60+)
     */
    public function isReadabilityGood(): bool
    {
        return $this->readability_score >= 60;
    }

    /**
     * Get readability level description
     */
    public function getReadabilityLevelAttribute(): string
    {
        return match(true) {
            $this->readability_score >= 90 => 'Very Easy',
            $this->readability_score >= 80 => 'Easy',
            $this->readability_score >= 70 => 'Fairly Easy',
            $this->readability_score >= 60 => 'Standard',
            $this->readability_score >= 50 => 'Fairly Difficult',
            $this->readability_score >= 30 => 'Difficult',
            default => 'Very Difficult',
        };
    }

    /**
     * Check if all basic SEO requirements are met
     */
    public function hasBasicSeoRequirements(): bool
    {
        return $this->has_meta_title
            && $this->has_meta_description
            && !empty($this->focus_keyword)
            && $this->word_count >= 300;
    }

    /**
     * Get number of critical issues
     */
    public function getCriticalIssuesCountAttribute(): int
    {
        $count = 0;

        if (!$this->has_meta_title) $count++;
        if (!$this->has_meta_description) $count++;
        if (empty($this->focus_keyword)) $count++;
        if ($this->isKeywordDensitySpammy()) $count++;
        if ($this->word_count < 300) $count++;

        return $count;
    }

    /**
     * Get completion percentage (0-100)
     */
    public function getCompletionPercentageAttribute(): int
    {
        $checks = [
            $this->has_meta_title,
            $this->has_meta_description,
            !empty($this->focus_keyword),
            $this->has_focus_keyword_in_title,
            $this->has_focus_keyword_in_meta,
            $this->isKeywordDensityOptimal(),
            $this->word_count >= 300,
            $this->heading_count > 0,
            $this->image_count > 0,
            $this->isReadabilityGood(),
        ];

        $completed = count(array_filter($checks));
        return (int) round(($completed / count($checks)) * 100);
    }

    /**
     * Get analysis result by key
     */
    public function getAnalysisResult(string $key, mixed $default = null): mixed
    {
        return $this->analysis_results[$key] ?? $default;
    }

    /**
     * Set analysis result
     */
    public function setAnalysisResult(string $key, mixed $value): self
    {
        $results = $this->analysis_results;
        $results[$key] = $value;
        $this->analysis_results = $results;

        return $this;
    }

    /**
     * Add suggestion
     */
    public function addSuggestion(string $suggestion, string $priority = 'medium'): self
    {
        $suggestions = $this->suggestions;
        $suggestions[] = [
            'message' => $suggestion,
            'priority' => $priority,
            'timestamp' => now()->toISOString(),
        ];
        $this->suggestions = $suggestions;

        return $this;
    }

    /**
     * Get suggestions by priority
     */
    public function getSuggestionsByPriority(string $priority): array
    {
        return array_filter(
            $this->suggestions,
            fn($suggestion) => ($suggestion['priority'] ?? 'medium') === $priority
        );
    }

    /**
     * Scope: Filter by analyzable type
     */
    public function scopeForType(Builder $query, string $type): Builder
    {
        return $query->where('analyzable_type', $type);
    }

    /**
     * Scope: Filter by score category
     */
    public function scopeByScoreCategory(Builder $query, string $category): Builder
    {
        return match($category) {
            self::CATEGORY_EXCELLENT => $query->where('seo_score', '>=', self::SCORE_EXCELLENT),
            self::CATEGORY_GOOD => $query->whereBetween('seo_score', [self::SCORE_GOOD, self::SCORE_EXCELLENT - 1]),
            self::CATEGORY_NEEDS_IMPROVEMENT => $query->whereBetween('seo_score', [self::SCORE_NEEDS_IMPROVEMENT, self::SCORE_GOOD - 1]),
            self::CATEGORY_POOR => $query->where('seo_score', '<', self::SCORE_NEEDS_IMPROVEMENT),
            default => $query,
        };
    }

    /**
     * Scope: Filter by excellent scores
     */
    public function scopeExcellent(Builder $query): Builder
    {
        return $query->where('seo_score', '>=', self::SCORE_EXCELLENT);
    }

    /**
     * Scope: Filter by good scores
     */
    public function scopeGood(Builder $query): Builder
    {
        return $query->where('seo_score', '>=', self::SCORE_GOOD);
    }

    /**
     * Scope: Filter by poor scores
     */
    public function scopePoor(Builder $query): Builder
    {
        return $query->where('seo_score', '<', self::SCORE_NEEDS_IMPROVEMENT);
    }

    /**
     * Scope: With focus keyword
     */
    public function scopeWithFocusKeyword(Builder $query): Builder
    {
        return $query->whereNotNull('focus_keyword')->where('focus_keyword', '!=', '');
    }

    /**
     * Scope: Missing meta title
     */
    public function scopeMissingMetaTitle(Builder $query): Builder
    {
        return $query->where('has_meta_title', false);
    }

    /**
     * Scope: Missing meta description
     */
    public function scopeMissingMetaDescription(Builder $query): Builder
    {
        return $query->where('has_meta_description', false);
    }

    /**
     * Scope: With critical issues
     */
    public function scopeWithCriticalIssues(Builder $query): Builder
    {
        return $query->where(function($q) {
            $q->where('has_meta_title', false)
              ->orWhere('has_meta_description', false)
              ->orWhereNull('focus_keyword')
              ->orWhere('focus_keyword', '')
              ->orWhere('word_count', '<', 300);
        });
    }

    /**
     * Scope: Recently analyzed (within hours)
     */
    public function scopeRecentlyAnalyzed(Builder $query, int $hours = 24): Builder
    {
        return $query->where('updated_at', '>=', now()->subHours($hours));
    }

    /**
     * Scope: Order by score
     */
    public function scopeOrderByScore(Builder $query, string $direction = 'desc'): Builder
    {
        return $query->orderBy('seo_score', $direction);
    }

    /**
     * Get summary for dashboard
     */
    public function getSummary(): array
    {
        return [
            'score' => $this->seo_score,
            'category' => $this->score_category,
            'category_label' => $this->score_category_label,
            'category_color' => $this->score_category_color,
            'completion_percentage' => $this->completion_percentage,
            'critical_issues' => $this->critical_issues_count,
            'keyword_density' => $this->keyword_density,
            'keyword_density_status' => $this->keyword_density_status,
            'readability_score' => $this->readability_score,
            'readability_level' => $this->readability_level,
            'word_count' => $this->word_count,
            'has_basic_requirements' => $this->hasBasicSeoRequirements(),
        ];
    }

    /**
     * Export to array for API
     */
    public function toAnalysisArray(): array
    {
        return [
            'id' => $this->id,
            'focus_keyword' => $this->focus_keyword,
            'summary' => $this->getSummary(),
            'metrics' => [
                'word_count' => $this->word_count,
                'heading_count' => $this->heading_count,
                'image_count' => $this->image_count,
                'internal_links' => $this->internal_links_count,
                'external_links' => $this->external_links_count,
            ],
            'checks' => [
                'has_meta_title' => $this->has_meta_title,
                'has_meta_description' => $this->has_meta_description,
                'has_focus_keyword_in_title' => $this->has_focus_keyword_in_title,
                'has_focus_keyword_in_meta' => $this->has_focus_keyword_in_meta,
                'has_focus_keyword_in_url' => $this->has_focus_keyword_in_url,
            ],
            'analysis_results' => $this->analysis_results,
            'suggestions' => $this->suggestions,
            'analyzed_at' => $this->updated_at->toISOString(),
        ];
    }
}