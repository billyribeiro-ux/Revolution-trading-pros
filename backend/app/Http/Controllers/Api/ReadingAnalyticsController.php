<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;

/**
 * Reading Analytics Controller
 *
 * Tracks blog post reading engagement:
 * - Scroll depth milestones
 * - Read completion
 * - Time on page
 * - Engagement scores
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class ReadingAnalyticsController extends Controller
{
    /**
     * Track reading event
     */
    public function track(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'event' => 'required|string|in:reading_milestone,reading_completion,reading_leave',
            'postId' => 'required|integer',
            'slug' => 'required|string|max:255',
            'milestone' => 'nullable|integer|min:0|max:100',
            'timeOnPage' => 'nullable|integer|min:0',
            'engagementScore' => 'nullable|integer|min:0|max:100',
            'readCompletion' => 'nullable|integer|min:0|max:100',
            'maxScrollDepth' => 'nullable|integer|min:0|max:100',
            'milestonesReached' => 'nullable|array',
            'timestamp' => 'nullable|integer',
            'url' => 'nullable|url|max:2048',
        ]);

        try {
            // Get session/visitor identifier
            $visitorId = $this->getVisitorId($request);

            // Store the event
            $this->storeEvent($validated, $visitorId, $request);

            // Update post analytics cache
            $this->updatePostAnalytics($validated);

            return response()->json(['status' => 'ok']);
        } catch (\Exception $e) {
            Log::error('Reading analytics tracking failed', [
                'error' => $e->getMessage(),
                'event' => $validated['event'] ?? 'unknown',
            ]);

            return response()->json(['status' => 'error'], 500);
        }
    }

    /**
     * Get reading analytics for a post (admin)
     */
    public function getPostAnalytics(int $postId): JsonResponse
    {
        $cacheKey = "reading_analytics:post:{$postId}";

        $analytics = Cache::remember($cacheKey, 300, function () use ($postId) {
            return [
                'postId' => $postId,
                'totalReads' => $this->getTotalReads($postId),
                'avgTimeOnPage' => $this->getAvgTimeOnPage($postId),
                'avgScrollDepth' => $this->getAvgScrollDepth($postId),
                'avgEngagement' => $this->getAvgEngagement($postId),
                'completionRate' => $this->getCompletionRate($postId),
                'milestoneBreakdown' => $this->getMilestoneBreakdown($postId),
                'readsByDay' => $this->getReadsByDay($postId),
            ];
        });

        return response()->json($analytics);
    }

    /**
     * Get visitor identifier
     */
    private function getVisitorId(Request $request): string
    {
        // Try to get from session or create fingerprint
        $sessionId = $request->session()->getId();

        if ($sessionId) {
            return hash('sha256', $sessionId);
        }

        // Fallback: IP + User Agent hash
        return hash('sha256', $request->ip() . $request->userAgent());
    }

    /**
     * Store reading event
     */
    private function storeEvent(array $data, string $visitorId, Request $request): void
    {
        // For high-volume sites, consider using a queue
        // For now, we'll use Redis for real-time and batch to DB

        $eventData = [
            'event_type' => $data['event'],
            'post_id' => $data['postId'],
            'slug' => $data['slug'],
            'visitor_id' => $visitorId,
            'milestone' => $data['milestone'] ?? null,
            'time_on_page' => $data['timeOnPage'] ?? null,
            'engagement_score' => $data['engagementScore'] ?? null,
            'read_completion' => $data['readCompletion'] ?? null,
            'max_scroll_depth' => $data['maxScrollDepth'] ?? null,
            'milestones_reached' => isset($data['milestonesReached'])
                ? json_encode($data['milestonesReached'])
                : null,
            'ip_address' => $request->ip(),
            'user_agent' => substr($request->userAgent() ?? '', 0, 500),
            'created_at' => now(),
        ];

        // Store in Redis for real-time analytics
        $redisKey = "reading_events:{$data['postId']}:" . date('Y-m-d');
        Cache::put($redisKey, $eventData, 86400);

        // Log for batch processing
        Log::channel('analytics')->info('Reading event', $eventData);
    }

    /**
     * Update cached post analytics
     */
    private function updatePostAnalytics(array $data): void
    {
        $postId = $data['postId'];
        $cacheKey = "post_reading_stats:{$postId}";

        $stats = Cache::get($cacheKey, [
            'reads' => 0,
            'completions' => 0,
            'totalTimeOnPage' => 0,
            'totalScrollDepth' => 0,
            'totalEngagement' => 0,
        ]);

        // Update based on event type
        if ($data['event'] === 'reading_leave' || $data['event'] === 'reading_completion') {
            $stats['reads']++;
            $stats['totalTimeOnPage'] += $data['timeOnPage'] ?? 0;
            $stats['totalScrollDepth'] += $data['maxScrollDepth'] ?? 0;
            $stats['totalEngagement'] += $data['engagementScore'] ?? 0;

            if ($data['event'] === 'reading_completion') {
                $stats['completions']++;
            }
        }

        Cache::put($cacheKey, $stats, 3600);

        // Invalidate detailed analytics cache
        Cache::forget("reading_analytics:post:{$postId}");
    }

    /**
     * Get total reads for a post
     */
    private function getTotalReads(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        return $stats['reads'] ?? 0;
    }

    /**
     * Get average time on page
     */
    private function getAvgTimeOnPage(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || $stats['reads'] === 0) {
            return 0;
        }
        return (int) ($stats['totalTimeOnPage'] / $stats['reads']);
    }

    /**
     * Get average scroll depth
     */
    private function getAvgScrollDepth(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || $stats['reads'] === 0) {
            return 0;
        }
        return (int) ($stats['totalScrollDepth'] / $stats['reads']);
    }

    /**
     * Get average engagement score
     */
    private function getAvgEngagement(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || $stats['reads'] === 0) {
            return 0;
        }
        return (int) ($stats['totalEngagement'] / $stats['reads']);
    }

    /**
     * Get completion rate percentage
     */
    private function getCompletionRate(int $postId): float
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || $stats['reads'] === 0) {
            return 0;
        }
        return round(($stats['completions'] / $stats['reads']) * 100, 1);
    }

    /**
     * Get milestone breakdown (placeholder)
     */
    private function getMilestoneBreakdown(int $postId): array
    {
        // This would query from a proper database table
        return [
            '25%' => 0,
            '50%' => 0,
            '75%' => 0,
            '100%' => 0,
        ];
    }

    /**
     * Get reads by day (placeholder)
     */
    private function getReadsByDay(int $postId): array
    {
        // This would query from a proper database table
        return [];
    }
}
