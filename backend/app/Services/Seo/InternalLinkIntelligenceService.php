<?php

namespace App\Services\Seo;

use App\Models\SeoInternalLinkSuggestion;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * Internal Link Intelligence Service
 * 
 * Google L8 Enterprise-Grade Internal Linking Engine
 * Implements: Link Graph, PageRank, Semantic Matching, Orphan Detection
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class InternalLinkIntelligenceService
{
    private const CACHE_PREFIX = 'seo:links';
    private const CACHE_TTL = 3600; // 1 hour

    /**
     * Build complete site link graph.
     */
    public function buildLinkGraph(): array
    {
        $cacheKey = self::CACHE_PREFIX . ":graph:full";
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () {
            $graph = [];
            
            // Get all content with links
            $posts = DB::table('posts')->select('id', 'title', 'slug', 'content')->get();
            
            foreach ($posts as $post) {
                $outboundLinks = $this->extractInternalLinks($post->content);
                $inboundLinks = $this->findInboundLinks('post', $post->id);
                
                $graph[$post->id] = [
                    'type' => 'post',
                    'id' => $post->id,
                    'title' => $post->title,
                    'slug' => $post->slug,
                    'outbound' => $outboundLinks,
                    'inbound' => $inboundLinks,
                    'outbound_count' => count($outboundLinks),
                    'inbound_count' => count($inboundLinks),
                    'is_orphan' => count($inboundLinks) === 0,
                    'is_hub' => count($outboundLinks) > 10,
                ];
                
                // Store in database
                DB::table('seo_link_graph')->updateOrInsert(
                    ['content_type' => 'post', 'content_id' => $post->id],
                    [
                        'outbound_links' => json_encode($outboundLinks),
                        'inbound_links' => json_encode($inboundLinks),
                        'outbound_count' => count($outboundLinks),
                        'inbound_count' => count($inboundLinks),
                        'pagerank_score' => 0, // Will be calculated separately
                        'is_orphan' => count($inboundLinks) === 0,
                        'is_hub' => count($outboundLinks) > 10,
                        'calculated_at' => now(),
                        'updated_at' => now(),
                    ]
                );
            }
            
            return $graph;
        });
    }

    /**
     * Calculate PageRank scores for all content.
     */
    public function calculatePageRank(int $iterations = 10, float $dampingFactor = 0.85): array
    {
        $graph = $this->buildLinkGraph();
        $pageRanks = [];
        
        // Initialize all pages with equal PageRank
        $initialRank = 1.0 / count($graph);
        foreach ($graph as $id => $node) {
            $pageRanks[$id] = $initialRank;
        }
        
        // Iterate PageRank calculation
        for ($i = 0; $i < $iterations; $i++) {
            $newRanks = [];
            
            foreach ($graph as $id => $node) {
                $rank = (1 - $dampingFactor);
                
                // Add rank from inbound links
                foreach ($node['inbound'] as $inboundLink) {
                    $inboundId = $inboundLink['id'];
                    if (isset($graph[$inboundId])) {
                        $outboundCount = $graph[$inboundId]['outbound_count'];
                        if ($outboundCount > 0) {
                            $rank += $dampingFactor * ($pageRanks[$inboundId] / $outboundCount);
                        }
                    }
                }
                
                $newRanks[$id] = $rank;
            }
            
            $pageRanks = $newRanks;
        }
        
        // Store PageRank scores
        foreach ($pageRanks as $id => $rank) {
            DB::table('seo_link_graph')
                ->where('content_type', 'post')
                ->where('content_id', $id)
                ->update(['pagerank_score' => $rank]);
        }
        
        return $pageRanks;
    }

    /**
     * Generate link suggestions for content.
     */
    public function generateSuggestions(
        string $contentType,
        int $contentId,
        string $content,
        int $limit = 10
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":suggestions:{$contentType}:{$contentId}";
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use (
            $contentType, $contentId, $content, $limit
        ) {
            // Get content embeddings/keywords for semantic matching
            $contentKeywords = $this->extractKeywords($content);
            
            // Find similar content
            $similarContent = $this->findSimilarContent($contentType, $contentId, $contentKeywords);
            
            // Generate suggestions
            $suggestions = [];
            
            foreach ($similarContent as $target) {
                // Check if link already exists
                if ($this->linkExists($contentType, $contentId, $target['type'], $target['id'])) {
                    continue;
                }
                
                // Find best anchor text
                $anchorText = $this->suggestAnchorText($content, $target['title'], $contentKeywords);
                
                // Find context for placement
                $context = $this->findLinkContext($content, $target['title'], $contentKeywords);
                
                $suggestion = SeoInternalLinkSuggestion::create([
                    'source_content_type' => $contentType,
                    'source_content_id' => $contentId,
                    'target_content_type' => $target['type'],
                    'target_content_id' => $target['id'],
                    'suggested_anchor_text' => $anchorText,
                    'context_snippet' => $context,
                    'relevance_score' => $target['relevance_score'],
                    'priority' => $this->calculatePriority($target['relevance_score']),
                    'reasoning' => $this->generateReasoning($target),
                    'status' => 'pending',
                ]);
                
                $suggestions[] = $suggestion;
                
                if (count($suggestions) >= $limit) {
                    break;
                }
            }
            
            return $suggestions;
        });
    }

    /**
     * Detect orphan pages (no inbound links).
     */
    public function detectOrphans(): array
    {
        $cacheKey = self::CACHE_PREFIX . ":orphans";
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () {
            return DB::table('seo_link_graph')
                ->where('is_orphan', true)
                ->get()
                ->toArray();
        });
    }

    /**
     * Detect hub pages (many outbound links).
     */
    public function detectHubs(int $threshold = 10): array
    {
        return DB::table('seo_link_graph')
            ->where('is_hub', true)
            ->where('outbound_count', '>=', $threshold)
            ->orderByDesc('outbound_count')
            ->get()
            ->toArray();
    }

    /**
     * Get link suggestions for orphan pages.
     */
    public function suggestLinksForOrphans(): array
    {
        $orphans = $this->detectOrphans();
        $suggestions = [];
        
        foreach ($orphans as $orphan) {
            // Find content to link TO this orphan
            $potentialSources = $this->findPotentialLinkSources(
                $orphan->content_type,
                $orphan->content_id
            );
            
            foreach ($potentialSources as $source) {
                $suggestions[] = [
                    'orphan' => $orphan,
                    'source' => $source,
                    'reasoning' => 'Link to orphan page to improve discoverability',
                ];
            }
        }
        
        return $suggestions;
    }

    /**
     * Extract internal links from content.
     */
    private function extractInternalLinks(string $content): array
    {
        $links = [];
        $domain = parse_url(config('app.url'), PHP_URL_HOST);
        
        // Extract all links
        preg_match_all('/<a\s+(?:[^>]*?\s+)?href="([^"]*)"[^>]*>(.*?)<\/a>/i', $content, $matches, PREG_SET_ORDER);
        
        foreach ($matches as $match) {
            $url = $match[1];
            $anchorText = strip_tags($match[2]);
            
            // Check if internal link
            $linkDomain = parse_url($url, PHP_URL_HOST);
            if (!$linkDomain || $linkDomain === $domain || strpos($url, '/') === 0) {
                // Try to resolve to content
                $resolved = $this->resolveUrlToContent($url);
                if ($resolved) {
                    $links[] = [
                        'type' => $resolved['type'],
                        'id' => $resolved['id'],
                        'url' => $url,
                        'anchor_text' => $anchorText,
                    ];
                }
            }
        }
        
        return $links;
    }

    /**
     * Find inbound links to content.
     */
    private function findInboundLinks(string $contentType, int $contentId): array
    {
        // Query link graph for inbound links
        $inbound = DB::table('seo_link_graph')
            ->whereRaw("JSON_CONTAINS(outbound_links, ?)", [json_encode(['id' => $contentId, 'type' => $contentType])])
            ->get();
        
        $links = [];
        foreach ($inbound as $source) {
            $links[] = [
                'type' => $source->content_type,
                'id' => $source->content_id,
            ];
        }
        
        return $links;
    }

    /**
     * Resolve URL to content.
     */
    private function resolveUrlToContent(string $url): ?array
    {
        // Extract slug from URL
        $path = parse_url($url, PHP_URL_PATH);
        $slug = basename($path);
        
        // Try to find post
        $post = DB::table('posts')->where('slug', $slug)->first();
        if ($post) {
            return ['type' => 'post', 'id' => $post->id];
        }
        
        // Try other content types...
        
        return null;
    }

    /**
     * Extract keywords from content.
     */
    private function extractKeywords(string $content): array
    {
        // Simple keyword extraction (in production, use NLP)
        $text = strip_tags($content);
        $words = str_word_count(strtolower($text), 1);
        $wordFreq = array_count_values($words);
        
        // Remove stop words
        $stopWords = ['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with'];
        $wordFreq = array_diff_key($wordFreq, array_flip($stopWords));
        
        arsort($wordFreq);
        
        return array_slice(array_keys($wordFreq), 0, 20);
    }

    /**
     * Find similar content based on keywords.
     */
    private function findSimilarContent(
        string $contentType,
        int $contentId,
        array $keywords
    ): array {
        $similar = [];
        
        // Get all other content
        $allContent = DB::table('posts')
            ->where('id', '!=', $contentId)
            ->select('id', 'title', 'slug', 'content')
            ->get();
        
        foreach ($allContent as $content) {
            $contentKeywords = $this->extractKeywords($content->content);
            $commonKeywords = array_intersect($keywords, $contentKeywords);
            
            if (count($commonKeywords) > 0) {
                $relevanceScore = (count($commonKeywords) / count($keywords)) * 100;
                
                $similar[] = [
                    'type' => 'post',
                    'id' => $content->id,
                    'title' => $content->title,
                    'slug' => $content->slug,
                    'relevance_score' => (int) $relevanceScore,
                    'common_keywords' => $commonKeywords,
                ];
            }
        }
        
        // Sort by relevance
        usort($similar, fn($a, $b) => $b['relevance_score'] <=> $a['relevance_score']);
        
        return $similar;
    }

    /**
     * Check if link already exists.
     */
    private function linkExists(
        string $sourceType,
        int $sourceId,
        string $targetType,
        int $targetId
    ): bool {
        $graph = DB::table('seo_link_graph')
            ->where('content_type', $sourceType)
            ->where('content_id', $sourceId)
            ->first();
        
        if (!$graph) {
            return false;
        }
        
        $outboundLinks = json_decode($graph->outbound_links, true) ?? [];
        
        foreach ($outboundLinks as $link) {
            if ($link['type'] === $targetType && $link['id'] === $targetId) {
                return true;
            }
        }
        
        return false;
    }

    /**
     * Suggest anchor text for link.
     */
    private function suggestAnchorText(
        string $sourceContent,
        string $targetTitle,
        array $keywords
    ): string {
        // Use target title as default
        $anchorText = $targetTitle;
        
        // Try to find a natural phrase in source content
        foreach ($keywords as $keyword) {
            if (stripos($targetTitle, $keyword) !== false && stripos($sourceContent, $keyword) !== false) {
                $anchorText = $keyword;
                break;
            }
        }
        
        return $anchorText;
    }

    /**
     * Find context for link placement.
     */
    private function findLinkContext(
        string $sourceContent,
        string $targetTitle,
        array $keywords
    ): string {
        $text = strip_tags($sourceContent);
        
        // Find paragraph with relevant keywords
        $paragraphs = explode("\n\n", $text);
        
        foreach ($paragraphs as $paragraph) {
            foreach ($keywords as $keyword) {
                if (stripos($paragraph, $keyword) !== false) {
                    return substr($paragraph, 0, 200) . '...';
                }
            }
        }
        
        return substr($text, 0, 200) . '...';
    }

    /**
     * Calculate suggestion priority.
     */
    private function calculatePriority(int $relevanceScore): string
    {
        if ($relevanceScore >= 80) return 'high';
        if ($relevanceScore >= 50) return 'medium';
        return 'low';
    }

    /**
     * Generate reasoning for suggestion.
     */
    private function generateReasoning(array $target): string
    {
        $commonCount = count($target['common_keywords']);
        return "High semantic relevance ({$target['relevance_score']}%) with {$commonCount} common topics";
    }

    /**
     * Find potential sources to link to orphan.
     */
    private function findPotentialLinkSources(string $orphanType, int $orphanId): array
    {
        // Get orphan content
        $orphan = DB::table('posts')->where('id', $orphanId)->first();
        if (!$orphan) {
            return [];
        }
        
        $orphanKeywords = $this->extractKeywords($orphan->content);
        
        // Find content with similar keywords
        return $this->findSimilarContent($orphanType, $orphanId, $orphanKeywords);
    }
}
