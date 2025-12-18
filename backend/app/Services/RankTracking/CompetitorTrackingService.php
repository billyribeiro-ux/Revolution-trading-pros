<?php

namespace App\Services\RankTracking;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

/**
 * Service for tracking and analyzing competitor rankings.
 * 
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
class CompetitorTrackingService
{
    private const CACHE_TTL = 3600; // 1 hour

    /**
     * Get competitors for a keyword.
     */
    public function getCompetitors(string $keyword, string $url, array $serpResults = []): array
    {
        $targetDomain = parse_url($url, PHP_URL_HOST);
        $competitors = [];

        foreach ($serpResults as $result) {
            $resultDomain = $result['domain'] ?? parse_url($result['url'] ?? '', PHP_URL_HOST);
            
            // Skip the target domain
            if ($resultDomain === $targetDomain) {
                continue;
            }

            $competitors[] = [
                'domain' => $resultDomain,
                'url' => $result['url'] ?? '',
                'position' => $result['position'] ?? 0,
                'title' => $result['title'] ?? '',
            ];
        }

        return array_slice($competitors, 0, 10);
    }

    /**
     * Analyze competitor gaps.
     */
    public function analyzeGaps(string $targetUrl, array $competitorUrls, array $keywords = []): array
    {
        $gaps = [];

        foreach ($keywords as $keyword) {
            $gaps[$keyword] = [
                'keyword' => $keyword,
                'your_position' => null,
                'competitors' => [],
                'opportunity_score' => 0,
            ];
        }

        return $gaps;
    }

    /**
     * Get competitor domain metrics.
     */
    public function getDomainMetrics(string $domain): array
    {
        $cacheKey = "competitor_metrics:{$domain}";

        return Cache::remember($cacheKey, self::CACHE_TTL * 24, function () use ($domain) {
            // This would integrate with domain authority APIs
            return [
                'domain' => $domain,
                'estimated_authority' => 50,
                'estimated_traffic' => 10000,
                'last_updated' => now()->toISOString(),
            ];
        });
    }

    /**
     * Track competitor position changes.
     */
    public function trackPositionChanges(string $keyword, array $currentResults, array $previousResults = []): array
    {
        $changes = [];

        foreach ($currentResults as $current) {
            $domain = $current['domain'] ?? '';
            $currentPos = $current['position'] ?? null;
            
            // Find in previous results
            $previousPos = null;
            foreach ($previousResults as $prev) {
                if (($prev['domain'] ?? '') === $domain) {
                    $previousPos = $prev['position'] ?? null;
                    break;
                }
            }

            if ($previousPos !== null && $currentPos !== null) {
                $change = $previousPos - $currentPos;
                if ($change !== 0) {
                    $changes[] = [
                        'domain' => $domain,
                        'previous_position' => $previousPos,
                        'current_position' => $currentPos,
                        'change' => $change,
                        'direction' => $change > 0 ? 'up' : 'down',
                    ];
                }
            }
        }

        return $changes;
    }
}
