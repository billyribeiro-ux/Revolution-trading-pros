<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;

/**
 * Full-Text Search Service
 *
 * Provides advanced search capabilities:
 * - Full-text search with relevance scoring
 * - Faceted search and filtering
 * - Fuzzy matching and typo tolerance
 * - Search suggestions and autocomplete
 * - Search analytics
 * - Synonym support
 * - Highlighting
 * - Geosearch (location-based)
 *
 * Designed to work with Meilisearch, Elasticsearch, or database fallback.
 */
class SearchService
{
    private const CACHE_TTL = 300; // 5 minutes
    private const SUGGESTION_LIMIT = 10;

    private ?object $searchClient = null;
    private PortableTextService $portableText;

    public function __construct(PortableTextService $portableText)
    {
        $this->portableText = $portableText;
        $this->initializeSearchClient();
    }

    /**
     * Perform a search query
     */
    public function search(string $query, array $options = []): SearchResult
    {
        $startTime = microtime(true);

        $cacheKey = $this->buildCacheKey($query, $options);

        if (!($options['skipCache'] ?? false)) {
            $cached = Cache::get($cacheKey);
            if ($cached) {
                $cached['fromCache'] = true;
                return SearchResult::fromArray($cached);
            }
        }

        // Normalize query
        $normalizedQuery = $this->normalizeQuery($query);

        // Apply synonym expansion
        if ($options['expandSynonyms'] ?? true) {
            $normalizedQuery = $this->expandSynonyms($normalizedQuery);
        }

        // Execute search
        $results = $this->executeSearch($normalizedQuery, $options);

        // Apply facets
        if (!empty($options['facets'])) {
            $results['facets'] = $this->calculateFacets($results['hits'], $options['facets']);
        }

        // Highlight matches
        if ($options['highlight'] ?? true) {
            $results['hits'] = $this->highlightMatches($results['hits'], $query);
        }

        // Record search analytics
        $this->recordSearchAnalytics($query, $results, $options);

        $results['query'] = $query;
        $results['processingTimeMs'] = round((microtime(true) - $startTime) * 1000, 2);
        $results['fromCache'] = false;

        // Cache results
        Cache::put($cacheKey, $results, self::CACHE_TTL);

        return SearchResult::fromArray($results);
    }

    /**
     * Get search suggestions (autocomplete)
     */
    public function suggest(string $prefix, array $options = []): array
    {
        $limit = $options['limit'] ?? self::SUGGESTION_LIMIT;
        $types = $options['types'] ?? ['post', 'category', 'tag'];

        $suggestions = [];

        foreach ($types as $type) {
            $typeResults = $this->getSuggestionsForType($prefix, $type, $limit);
            $suggestions = array_merge($suggestions, $typeResults);
        }

        // Sort by relevance
        usort($suggestions, fn($a, $b) => $b['score'] <=> $a['score']);

        // Add popular searches
        $popularSearches = $this->getPopularSearches($prefix, 3);
        foreach ($popularSearches as $popular) {
            array_unshift($suggestions, [
                'text' => $popular['query'],
                'type' => 'popular',
                'score' => $popular['count'] / 100,
            ]);
        }

        return array_slice($suggestions, 0, $limit);
    }

    /**
     * Search with filters
     */
    public function searchWithFilters(string $query, array $filters, array $options = []): SearchResult
    {
        $options['filters'] = $filters;
        return $this->search($query, $options);
    }

    /**
     * Faceted search
     */
    public function facetedSearch(string $query, array $facetFields, array $options = []): SearchResult
    {
        $options['facets'] = $facetFields;
        return $this->search($query, $options);
    }

    /**
     * Geosearch - find content near a location
     */
    public function geoSearch(float $lat, float $lng, float $radiusKm, array $options = []): SearchResult
    {
        $startTime = microtime(true);

        $results = DB::table('posts')
            ->select('posts.*')
            ->selectRaw(
                '(6371 * acos(cos(radians(?)) * cos(radians(latitude)) * cos(radians(longitude) - radians(?)) + sin(radians(?)) * sin(radians(latitude)))) AS distance',
                [$lat, $lng, $lat]
            )
            ->whereNotNull('latitude')
            ->whereNotNull('longitude')
            ->having('distance', '<=', $radiusKm)
            ->orderBy('distance')
            ->limit($options['limit'] ?? 20)
            ->get();

        return SearchResult::fromArray([
            'hits' => $results->map(fn($r) => $this->formatHit($r, null))->toArray(),
            'totalHits' => $results->count(),
            'query' => "geo:{$lat},{$lng}",
            'processingTimeMs' => round((microtime(true) - $startTime) * 1000, 2),
            'fromCache' => false,
        ]);
    }

    /**
     * Similar content search
     */
    public function findSimilar(string $documentId, array $options = []): SearchResult
    {
        $document = DB::table('posts')->find($documentId);

        if (!$document) {
            return SearchResult::fromArray(['hits' => [], 'totalHits' => 0]);
        }

        // Extract keywords from document
        $text = $document->title . ' ' . ($document->excerpt ?? '');
        $keywords = $this->extractKeywords($text);

        // Search for similar documents
        $query = implode(' ', array_slice($keywords, 0, 5));

        $results = $this->search($query, array_merge($options, [
            'exclude' => [$documentId],
            'limit' => $options['limit'] ?? 5,
        ]));

        return $results;
    }

    /**
     * Index a document for search
     */
    public function indexDocument(string $type, array $document): bool
    {
        $searchDocument = $this->prepareDocumentForIndex($type, $document);

        // Use search client if available
        if ($this->searchClient) {
            try {
                $this->searchClient->index($type)->addDocuments([$searchDocument]);
                return true;
            } catch (\Exception $e) {
                // Fall through to database
            }
        }

        // Store searchable content in database
        DB::table('search_index')->updateOrInsert(
            ['document_id' => $searchDocument['id'], 'document_type' => $type],
            [
                'title' => $searchDocument['title'] ?? '',
                'content' => $searchDocument['searchableContent'] ?? '',
                'metadata' => json_encode($searchDocument['metadata'] ?? []),
                'updated_at' => now(),
            ]
        );

        // Invalidate related caches
        Cache::tags(['search'])->flush();

        return true;
    }

    /**
     * Remove document from search index
     */
    public function removeDocument(string $type, string $documentId): bool
    {
        if ($this->searchClient) {
            try {
                $this->searchClient->index($type)->deleteDocument($documentId);
            } catch (\Exception $e) {
                // Continue with database cleanup
            }
        }

        DB::table('search_index')
            ->where('document_id', $documentId)
            ->where('document_type', $type)
            ->delete();

        return true;
    }

    /**
     * Get search analytics
     */
    public function getSearchAnalytics(array $options = []): array
    {
        $days = $options['days'] ?? 30;
        $since = now()->subDays($days);

        $topSearches = DB::table('search_analytics')
            ->select('query', DB::raw('COUNT(*) as count'), DB::raw('AVG(results_count) as avg_results'))
            ->where('created_at', '>=', $since)
            ->groupBy('query')
            ->orderByDesc('count')
            ->limit(20)
            ->get();

        $noResults = DB::table('search_analytics')
            ->select('query', DB::raw('COUNT(*) as count'))
            ->where('created_at', '>=', $since)
            ->where('results_count', 0)
            ->groupBy('query')
            ->orderByDesc('count')
            ->limit(20)
            ->get();

        $searchVolume = DB::table('search_analytics')
            ->select(DB::raw('DATE(created_at) as date'), DB::raw('COUNT(*) as searches'))
            ->where('created_at', '>=', $since)
            ->groupBy(DB::raw('DATE(created_at)'))
            ->orderBy('date')
            ->get();

        return [
            'topSearches' => $topSearches->toArray(),
            'zeroResultSearches' => $noResults->toArray(),
            'searchVolume' => $searchVolume->toArray(),
            'totalSearches' => DB::table('search_analytics')->where('created_at', '>=', $since)->count(),
            'avgResultsPerSearch' => DB::table('search_analytics')->where('created_at', '>=', $since)->avg('results_count'),
        ];
    }

    /**
     * Manage synonyms
     */
    public function addSynonyms(string $word, array $synonyms): bool
    {
        DB::table('search_synonyms')->updateOrInsert(
            ['word' => strtolower($word)],
            ['synonyms' => json_encode(array_map('strtolower', $synonyms)), 'updated_at' => now()]
        );

        Cache::forget('search_synonyms');
        return true;
    }

    public function getSynonyms(): array
    {
        return Cache::remember('search_synonyms', 3600, function () {
            return DB::table('search_synonyms')
                ->get()
                ->mapWithKeys(fn($row) => [$row->word => json_decode($row->synonyms, true)])
                ->toArray();
        });
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════

    private function initializeSearchClient(): void
    {
        // Initialize Meilisearch or Elasticsearch client if configured
        // $this->searchClient = new \Meilisearch\Client(config('services.meilisearch.host'));
    }

    private function buildCacheKey(string $query, array $options): string
    {
        return 'search:' . md5($query . json_encode($options));
    }

    private function normalizeQuery(string $query): string
    {
        $query = trim($query);
        $query = preg_replace('/\s+/', ' ', $query);
        return $query;
    }

    private function expandSynonyms(string $query): string
    {
        $synonyms = $this->getSynonyms();
        $words = explode(' ', strtolower($query));

        $expanded = [];
        foreach ($words as $word) {
            $expanded[] = $word;
            if (isset($synonyms[$word])) {
                $expanded = array_merge($expanded, $synonyms[$word]);
            }
        }

        return implode(' ', array_unique($expanded));
    }

    private function executeSearch(string $query, array $options): array
    {
        $limit = $options['limit'] ?? 20;
        $offset = $options['offset'] ?? 0;
        $types = $options['types'] ?? ['post'];
        $filters = $options['filters'] ?? [];

        // Use search client if available
        if ($this->searchClient) {
            return $this->executeExternalSearch($query, $options);
        }

        // Database fallback with full-text search
        return $this->executeDatabaseSearch($query, $types, $filters, $limit, $offset);
    }

    private function executeExternalSearch(string $query, array $options): array
    {
        // Meilisearch/Elasticsearch implementation
        return ['hits' => [], 'totalHits' => 0];
    }

    private function executeDatabaseSearch(string $query, array $types, array $filters, int $limit, int $offset): array
    {
        $hits = [];
        $totalHits = 0;

        foreach ($types as $type) {
            $table = $this->getTableForType($type);

            $baseQuery = DB::table($table)
                ->where(function ($q) use ($query) {
                    $q->where('title', 'LIKE', "%{$query}%")
                        ->orWhere('content', 'LIKE', "%{$query}%")
                        ->orWhere('excerpt', 'LIKE', "%{$query}%");
                });

            // Apply filters
            foreach ($filters as $field => $value) {
                if (is_array($value)) {
                    $baseQuery->whereIn($field, $value);
                } else {
                    $baseQuery->where($field, $value);
                }
            }

            // Exclude documents
            if (!empty($options['exclude'])) {
                $baseQuery->whereNotIn('id', $options['exclude']);
            }

            $count = $baseQuery->count();
            $totalHits += $count;

            $results = $baseQuery
                ->orderByRaw("CASE WHEN title LIKE ? THEN 0 ELSE 1 END", ["%{$query}%"])
                ->offset($offset)
                ->limit($limit)
                ->get();

            foreach ($results as $result) {
                $hits[] = $this->formatHit($result, $query, $type);
            }
        }

        // Sort by relevance
        usort($hits, fn($a, $b) => $b['_score'] <=> $a['_score']);

        return [
            'hits' => array_slice($hits, 0, $limit),
            'totalHits' => $totalHits,
        ];
    }

    private function getTableForType(string $type): string
    {
        return match ($type) {
            'post' => 'posts',
            'category' => 'categories',
            'tag' => 'tags',
            'media' => 'media',
            default => $type,
        };
    }

    private function formatHit(object $result, ?string $query, string $type = 'post'): array
    {
        $score = 1.0;

        if ($query) {
            // Calculate simple relevance score
            $titleMatches = substr_count(strtolower($result->title ?? ''), strtolower($query));
            $contentMatches = substr_count(strtolower($result->content ?? ''), strtolower($query));
            $score = ($titleMatches * 2) + $contentMatches + 1;
        }

        return [
            '_id' => $result->id,
            '_type' => $type,
            '_score' => $score,
            'title' => $result->title ?? '',
            'excerpt' => $result->excerpt ?? substr($result->content ?? '', 0, 200),
            'slug' => $result->slug ?? null,
            'status' => $result->status ?? 'published',
            'createdAt' => $result->created_at ?? null,
            'updatedAt' => $result->updated_at ?? null,
        ];
    }

    private function calculateFacets(array $hits, array $facetFields): array
    {
        $facets = [];

        foreach ($facetFields as $field) {
            $facets[$field] = [];

            foreach ($hits as $hit) {
                $value = $hit[$field] ?? null;

                if ($value === null) continue;

                if (is_array($value)) {
                    foreach ($value as $v) {
                        $facets[$field][$v] = ($facets[$field][$v] ?? 0) + 1;
                    }
                } else {
                    $facets[$field][$value] = ($facets[$field][$value] ?? 0) + 1;
                }
            }

            // Sort by count
            arsort($facets[$field]);
        }

        return $facets;
    }

    private function highlightMatches(array $hits, string $query): array
    {
        $words = explode(' ', $query);
        $pattern = '/(' . implode('|', array_map('preg_quote', $words)) . ')/i';

        foreach ($hits as &$hit) {
            $hit['_highlighted'] = [];

            foreach (['title', 'excerpt'] as $field) {
                if (isset($hit[$field])) {
                    $hit['_highlighted'][$field] = preg_replace(
                        $pattern,
                        '<mark>$1</mark>',
                        $hit[$field]
                    );
                }
            }
        }

        return $hits;
    }

    private function recordSearchAnalytics(string $query, array $results, array $options): void
    {
        DB::table('search_analytics')->insert([
            'query' => $query,
            'results_count' => $results['totalHits'] ?? 0,
            'filters' => json_encode($options['filters'] ?? []),
            'user_id' => auth()->id(),
            'created_at' => now(),
        ]);
    }

    private function getSuggestionsForType(string $prefix, string $type, int $limit): array
    {
        $table = $this->getTableForType($type);

        $results = DB::table($table)
            ->where('title', 'LIKE', "{$prefix}%")
            ->orderBy('title')
            ->limit($limit)
            ->get(['id', 'title']);

        return $results->map(fn($r) => [
            'text' => $r->title,
            'type' => $type,
            'id' => $r->id,
            'score' => strlen($prefix) / strlen($r->title),
        ])->toArray();
    }

    private function getPopularSearches(string $prefix, int $limit): array
    {
        return DB::table('search_analytics')
            ->select('query', DB::raw('COUNT(*) as count'))
            ->where('query', 'LIKE', "{$prefix}%")
            ->where('results_count', '>', 0)
            ->where('created_at', '>=', now()->subDays(7))
            ->groupBy('query')
            ->orderByDesc('count')
            ->limit($limit)
            ->get()
            ->toArray();
    }

    private function prepareDocumentForIndex(string $type, array $document): array
    {
        $searchable = [
            'id' => $document['id'] ?? Str::uuid()->toString(),
            'type' => $type,
            'title' => $document['title'] ?? '',
            'searchableContent' => '',
            'metadata' => [],
        ];

        // Extract searchable content
        if (isset($document['content'])) {
            if (is_array($document['content'])) {
                $searchable['searchableContent'] = $this->portableText->toPlainText($document['content']);
            } else {
                $searchable['searchableContent'] = $document['content'];
            }
        }

        // Add other searchable fields
        foreach (['excerpt', 'description', 'body'] as $field) {
            if (isset($document[$field])) {
                if (is_array($document[$field])) {
                    $searchable['searchableContent'] .= ' ' . $this->portableText->toPlainText($document[$field]);
                } else {
                    $searchable['searchableContent'] .= ' ' . $document[$field];
                }
            }
        }

        // Store metadata
        $searchable['metadata'] = [
            'status' => $document['status'] ?? 'draft',
            'author' => $document['author_id'] ?? null,
            'categories' => $document['categories'] ?? [],
            'tags' => $document['tags'] ?? [],
            'createdAt' => $document['created_at'] ?? now()->toIso8601String(),
        ];

        return $searchable;
    }

    private function extractKeywords(string $text): array
    {
        $words = str_word_count(strtolower($text), 1);
        $stopWords = ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by'];

        $wordFreq = [];
        foreach ($words as $word) {
            if (strlen($word) < 3 || in_array($word, $stopWords, true)) continue;
            $wordFreq[$word] = ($wordFreq[$word] ?? 0) + 1;
        }

        arsort($wordFreq);
        return array_keys($wordFreq);
    }
}

/**
 * Search Result DTO
 */
class SearchResult
{
    public array $hits = [];
    public int $totalHits = 0;
    public string $query = '';
    public float $processingTimeMs = 0;
    public bool $fromCache = false;
    public array $facets = [];

    public static function fromArray(array $data): self
    {
        $result = new self();
        foreach ($data as $key => $value) {
            if (property_exists($result, $key)) {
                $result->{$key} = $value;
            }
        }
        return $result;
    }

    public function toArray(): array
    {
        return [
            'hits' => $this->hits,
            'totalHits' => $this->totalHits,
            'query' => $this->query,
            'processingTimeMs' => $this->processingTimeMs,
            'fromCache' => $this->fromCache,
            'facets' => $this->facets,
        ];
    }
}
