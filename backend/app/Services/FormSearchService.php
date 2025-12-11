<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormField;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Form Search Service - Full-text search with relevance scoring
 *
 * Features:
 * - Full-text search across forms
 * - Multi-field search (title, description, fields)
 * - Relevance scoring
 * - Advanced filtering
 * - Search suggestions/autocomplete
 * - Typo tolerance
 * - Search analytics
 * - Search result highlighting
 *
 * @version 1.0.0
 */
class FormSearchService
{
    /**
     * Field weights for relevance scoring
     */
    private const FIELD_WEIGHTS = [
        'title' => 10,
        'description' => 5,
        'field_labels' => 3,
        'slug' => 2,
        'tags' => 4,
    ];

    /**
     * Minimum characters for search
     */
    private const MIN_QUERY_LENGTH = 2;

    /**
     * Maximum results
     */
    private const MAX_RESULTS = 100;

    /**
     * Search forms with full-text search
     *
     * @param string $query Search query
     * @param array $filters Additional filters
     * @param array $options Search options
     * @return array Search results with metadata
     */
    public function search(string $query, array $filters = [], array $options = []): array
    {
        $query = trim($query);

        if (strlen($query) < self::MIN_QUERY_LENGTH) {
            return [
                'results' => [],
                'total' => 0,
                'query' => $query,
                'suggestions' => [],
            ];
        }

        $limit = min($options['limit'] ?? 20, self::MAX_RESULTS);
        $offset = $options['offset'] ?? 0;

        // Prepare search terms
        $terms = $this->prepareSearchTerms($query);

        // Build base query
        $dbQuery = Form::query()
            ->select('forms.*')
            ->selectRaw($this->buildRelevanceScore($terms))
            ->leftJoin('form_fields', 'forms.id', '=', 'form_fields.form_id');

        // Apply text search
        $dbQuery->where(function ($q) use ($terms) {
            foreach ($terms as $term) {
                $q->where(function ($innerQ) use ($term) {
                    $innerQ->where('forms.title', 'like', "%{$term}%")
                        ->orWhere('forms.description', 'like', "%{$term}%")
                        ->orWhere('forms.slug', 'like', "%{$term}%")
                        ->orWhere('form_fields.label', 'like', "%{$term}%");
                });
            }
        });

        // Apply filters
        $this->applyFilters($dbQuery, $filters);

        // Get total count before pagination
        $total = (clone $dbQuery)->distinct('forms.id')->count('forms.id');

        // Get results with relevance ordering
        $results = $dbQuery->groupBy('forms.id')
            ->orderByRaw('relevance_score DESC')
            ->orderBy('forms.updated_at', 'desc')
            ->offset($offset)
            ->limit($limit)
            ->get();

        // Highlight matches
        if ($options['highlight'] ?? true) {
            $results = $this->highlightMatches($results, $terms);
        }

        // Track search
        $this->trackSearch($query, $total);

        // Get suggestions if few results
        $suggestions = [];
        if ($total < 3) {
            $suggestions = $this->getSuggestions($query);
        }

        return [
            'results' => $results,
            'total' => $total,
            'query' => $query,
            'terms' => $terms,
            'suggestions' => $suggestions,
            'facets' => $this->getFacets($dbQuery, $filters),
        ];
    }

    /**
     * Get search suggestions/autocomplete
     */
    public function autocomplete(string $query, int $limit = 10): array
    {
        if (strlen($query) < 2) {
            return [];
        }

        $suggestions = [];

        // Search in form titles
        $titleMatches = Form::where('title', 'like', "{$query}%")
            ->orWhere('title', 'like', "% {$query}%")
            ->limit($limit)
            ->pluck('title')
            ->toArray();

        foreach ($titleMatches as $title) {
            $suggestions[] = [
                'type' => 'form',
                'text' => $title,
                'highlight' => $this->highlightText($title, [$query]),
            ];
        }

        // Search in popular searches
        $popularSearches = DB::table('form_search_analytics')
            ->where('query', 'like', "{$query}%")
            ->where('results_count', '>', 0)
            ->orderBy('search_count', 'desc')
            ->limit(5)
            ->pluck('query')
            ->toArray();

        foreach ($popularSearches as $popular) {
            if (!in_array($popular, array_column($suggestions, 'text'))) {
                $suggestions[] = [
                    'type' => 'popular',
                    'text' => $popular,
                    'highlight' => $this->highlightText($popular, [$query]),
                ];
            }
        }

        return array_slice($suggestions, 0, $limit);
    }

    /**
     * Get similar forms
     */
    public function findSimilar(Form $form, int $limit = 5): array
    {
        // Extract keywords from form
        $keywords = $this->extractKeywords($form);

        if (empty($keywords)) {
            return [];
        }

        $query = Form::where('id', '!=', $form->id)
            ->where('status', 'published')
            ->where(function ($q) use ($keywords) {
                foreach ($keywords as $keyword) {
                    $q->orWhere('title', 'like', "%{$keyword}%")
                        ->orWhere('description', 'like', "%{$keyword}%");
                }
            })
            ->limit($limit)
            ->get();

        return $query->toArray();
    }

    /**
     * Get trending searches
     */
    public function getTrendingSearches(int $limit = 10, int $days = 7): array
    {
        return DB::table('form_search_analytics')
            ->where('created_at', '>=', now()->subDays($days))
            ->where('results_count', '>', 0)
            ->select('query')
            ->selectRaw('SUM(search_count) as total_searches')
            ->groupBy('query')
            ->orderBy('total_searches', 'desc')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    /**
     * Get search analytics
     */
    public function getSearchAnalytics(int $days = 30): array
    {
        $since = now()->subDays($days);

        return [
            'total_searches' => DB::table('form_search_analytics')
                ->where('created_at', '>=', $since)
                ->sum('search_count'),

            'unique_queries' => DB::table('form_search_analytics')
                ->where('created_at', '>=', $since)
                ->distinct('query')
                ->count('query'),

            'zero_result_rate' => $this->getZeroResultRate($since),

            'top_queries' => DB::table('form_search_analytics')
                ->where('created_at', '>=', $since)
                ->select('query', 'results_count')
                ->selectRaw('SUM(search_count) as searches')
                ->groupBy('query', 'results_count')
                ->orderBy('searches', 'desc')
                ->limit(20)
                ->get()
                ->toArray(),

            'daily_searches' => DB::table('form_search_analytics')
                ->where('created_at', '>=', $since)
                ->selectRaw('DATE(created_at) as date, SUM(search_count) as searches')
                ->groupBy('date')
                ->orderBy('date')
                ->get()
                ->toArray(),
        ];
    }

    /**
     * Prepare search terms (tokenize, clean, expand)
     */
    private function prepareSearchTerms(string $query): array
    {
        // Lowercase and split
        $terms = preg_split('/\s+/', strtolower($query));

        // Remove short terms and duplicates
        $terms = array_filter($terms, fn($t) => strlen($t) >= 2);
        $terms = array_unique($terms);

        // Add fuzzy variations for typo tolerance
        $expanded = [];
        foreach ($terms as $term) {
            $expanded[] = $term;
            // Add common typo fixes
            if (strlen($term) > 3) {
                $expanded = array_merge($expanded, $this->generateTypoVariations($term));
            }
        }

        return array_unique($expanded);
    }

    /**
     * Generate typo variations for a term
     */
    private function generateTypoVariations(string $term): array
    {
        $variations = [];

        // Common character swaps
        $swaps = ['a' => 'e', 'e' => 'a', 'i' => 'y', 'y' => 'i', 'o' => 'u', 's' => 'z'];

        foreach ($swaps as $from => $to) {
            if (str_contains($term, $from)) {
                $variations[] = str_replace($from, $to, $term);
            }
        }

        // Limit variations
        return array_slice($variations, 0, 3);
    }

    /**
     * Build relevance score SQL
     */
    private function buildRelevanceScore(array $terms): string
    {
        $cases = [];

        foreach ($terms as $term) {
            $term = addslashes($term);

            // Title exact match
            $cases[] = "CASE WHEN forms.title LIKE '{$term}' THEN " . self::FIELD_WEIGHTS['title'] * 2 . " ELSE 0 END";

            // Title contains
            $cases[] = "CASE WHEN forms.title LIKE '%{$term}%' THEN " . self::FIELD_WEIGHTS['title'] . " ELSE 0 END";

            // Description contains
            $cases[] = "CASE WHEN forms.description LIKE '%{$term}%' THEN " . self::FIELD_WEIGHTS['description'] . " ELSE 0 END";

            // Slug contains
            $cases[] = "CASE WHEN forms.slug LIKE '%{$term}%' THEN " . self::FIELD_WEIGHTS['slug'] . " ELSE 0 END";

            // Field label contains
            $cases[] = "CASE WHEN form_fields.label LIKE '%{$term}%' THEN " . self::FIELD_WEIGHTS['field_labels'] . " ELSE 0 END";
        }

        return '(' . implode(' + ', $cases) . ') as relevance_score';
    }

    /**
     * Apply search filters
     */
    private function applyFilters($query, array $filters): void
    {
        if (!empty($filters['status'])) {
            $query->where('forms.status', $filters['status']);
        }

        if (!empty($filters['user_id'])) {
            $query->where('forms.user_id', $filters['user_id']);
        }

        if (!empty($filters['created_after'])) {
            $query->where('forms.created_at', '>=', $filters['created_after']);
        }

        if (!empty($filters['created_before'])) {
            $query->where('forms.created_at', '<=', $filters['created_before']);
        }

        if (!empty($filters['has_submissions'])) {
            $query->where('forms.submission_count', '>', 0);
        }
    }

    /**
     * Highlight search matches in results
     */
    private function highlightMatches($results, array $terms): mixed
    {
        return $results->map(function ($form) use ($terms) {
            $form->title_highlighted = $this->highlightText($form->title, $terms);
            $form->description_highlighted = $this->highlightText($form->description ?? '', $terms);
            return $form;
        });
    }

    /**
     * Highlight text with search terms
     */
    private function highlightText(string $text, array $terms): string
    {
        foreach ($terms as $term) {
            $text = preg_replace(
                '/(' . preg_quote($term, '/') . ')/i',
                '<mark>$1</mark>',
                $text
            );
        }

        return $text;
    }

    /**
     * Get search facets
     */
    private function getFacets($baseQuery, array $appliedFilters): array
    {
        return [
            'status' => DB::table('forms')
                ->select('status')
                ->selectRaw('COUNT(*) as count')
                ->groupBy('status')
                ->pluck('count', 'status')
                ->toArray(),
        ];
    }

    /**
     * Get search suggestions
     */
    private function getSuggestions(string $query): array
    {
        // Get forms with similar titles
        $suggestions = Form::where('status', 'published')
            ->where('title', 'like', substr($query, 0, 3) . '%')
            ->limit(5)
            ->pluck('title')
            ->toArray();

        return $suggestions;
    }

    /**
     * Extract keywords from form
     */
    private function extractKeywords(Form $form): array
    {
        $text = $form->title . ' ' . ($form->description ?? '');

        // Simple keyword extraction
        $words = preg_split('/\s+/', strtolower($text));
        $words = array_filter($words, fn($w) => strlen($w) > 3);

        // Remove common words
        $stopWords = ['form', 'the', 'and', 'for', 'with', 'your', 'this', 'that', 'from'];
        $words = array_diff($words, $stopWords);

        return array_slice(array_unique($words), 0, 10);
    }

    /**
     * Track search query
     */
    private function trackSearch(string $query, int $resultsCount): void
    {
        DB::table('form_search_analytics')->updateOrInsert(
            ['query' => strtolower($query), 'date' => now()->toDateString()],
            [
                'results_count' => $resultsCount,
                'search_count' => DB::raw('search_count + 1'),
                'updated_at' => now(),
            ]
        );
    }

    /**
     * Get zero result rate
     */
    private function getZeroResultRate($since): float
    {
        $total = DB::table('form_search_analytics')
            ->where('created_at', '>=', $since)
            ->sum('search_count');

        $zeroResults = DB::table('form_search_analytics')
            ->where('created_at', '>=', $since)
            ->where('results_count', 0)
            ->sum('search_count');

        return $total > 0 ? round(($zeroResults / $total) * 100, 2) : 0;
    }
}
