<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Post;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\RateLimiter;

/**
 * Reading Analytics Controller
 *
 * Tracks blog post reading engagement:
 * - Scroll depth milestones
 * - Read completion
 * - Time on page
 * - Engagement scores
 *
 * Security features:
 * - Rate limiting to prevent abuse
 * - GDPR-compliant IP masking
 * - Admin authorization for analytics access
 *
 * @version 2.0.0 - Production Ready
 */
class ReadingAnalyticsController extends Controller
{
    /**
     * Constructor - apply middleware
     */
    public function __construct()
    {
        // Require authentication for viewing analytics
        $this->middleware('auth:sanctum')->only(['getPostAnalytics', 'getAllAnalytics']);

        // Require admin role for analytics access
        $this->middleware('can:view-analytics')->only(['getPostAnalytics', 'getAllAnalytics']);
    }

    /**
     * Track reading event (public endpoint with rate limiting)
     */
    public function track(Request $request): JsonResponse
    {
        // Rate limiting: 60 requests per minute per IP
        $key = 'reading_analytics:' . $this->maskIpAddress($request->ip());

        if (RateLimiter::tooManyAttempts($key, 60)) {
            return response()->json([
                'status' => 'error',
                'message' => 'Too many requests'
            ], 429);
        }

        RateLimiter::hit($key, 60);

        $validated = $request->validate([
            'event' => 'required|string|in:reading_milestone,reading_completion,reading_leave',
            'postId' => 'required|integer|exists:posts,id',
            'slug' => 'required|string|max:255|regex:/^[a-z0-9]+(?:-[a-z0-9]+)*$/',
            'milestone' => 'nullable|integer|in:25,50,75,100',
            'timeOnPage' => 'nullable|integer|min:0|max:86400',
            'engagementScore' => 'nullable|integer|min:0|max:100',
            'readCompletion' => 'nullable|integer|min:0|max:100',
            'maxScrollDepth' => 'nullable|integer|min:0|max:100',
            'milestonesReached' => 'nullable|array|max:4',
            'milestonesReached.*' => 'integer|in:25,50,75,100',
            'timestamp' => 'nullable|integer|min:0',
            'url' => 'nullable|url|max:2048',
        ]);

        try {
            // Verify post exists and is published
            $post = Post::where('id', $validated['postId'])
                ->where('slug', $validated['slug'])
                ->where('status', 'published')
                ->first();

            if (!$post) {
                return response()->json(['status' => 'error', 'message' => 'Post not found'], 404);
            }

            // Get GDPR-compliant visitor identifier
            $visitorId = $this->getVisitorId($request);

            // Store the event with masked IP
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
     * Get reading analytics for a post (admin only)
     */
    public function getPostAnalytics(int $postId): JsonResponse
    {
        // Verify post exists
        $post = Post::find($postId);
        if (!$post) {
            return response()->json(['error' => 'Post not found'], 404);
        }

        $cacheKey = "reading_analytics:post:{$postId}";

        $analytics = Cache::remember($cacheKey, 300, function () use ($postId, $post) {
            return [
                'postId' => $postId,
                'title' => $post->title,
                'slug' => $post->slug,
                'totalReads' => $this->getTotalReads($postId),
                'avgTimeOnPage' => $this->getAvgTimeOnPage($postId),
                'avgScrollDepth' => $this->getAvgScrollDepth($postId),
                'avgEngagement' => $this->getAvgEngagement($postId),
                'completionRate' => $this->getCompletionRate($postId),
                'milestoneBreakdown' => $this->getMilestoneBreakdown($postId),
                'readsByDay' => $this->getReadsByDay($postId),
                'generatedAt' => now()->toIso8601String(),
            ];
        });

        return response()->json($analytics);
    }

    /**
     * Get analytics summary for all posts (admin only)
     */
    public function getAllAnalytics(Request $request): JsonResponse
    {
        $perPage = min($request->input('per_page', 20), 100);

        $posts = Post::where('status', 'published')
            ->select(['id', 'title', 'slug', 'published_at'])
            ->orderByDesc('published_at')
            ->paginate($perPage);

        $analytics = $posts->through(function ($post) {
            $stats = Cache::get("post_reading_stats:{$post->id}", [
                'reads' => 0,
                'completions' => 0,
            ]);

            return [
                'postId' => $post->id,
                'title' => $post->title,
                'slug' => $post->slug,
                'publishedAt' => $post->published_at,
                'totalReads' => $stats['reads'],
                'completionRate' => $stats['reads'] > 0
                    ? round(($stats['completions'] / $stats['reads']) * 100, 1)
                    : 0,
            ];
        });

        return response()->json($analytics);
    }

    /**
     * Get GDPR-compliant visitor identifier
     * Uses hashed session or anonymized IP fingerprint
     */
    private function getVisitorId(Request $request): string
    {
        // Prefer session-based identification
        if ($request->hasSession()) {
            $sessionId = $request->session()->getId();
            if ($sessionId) {
                return hash('sha256', $sessionId . config('app.key'));
            }
        }

        // Fallback: Anonymized fingerprint (GDPR compliant)
        $maskedIp = $this->maskIpAddress($request->ip());
        $userAgent = substr($request->userAgent() ?? '', 0, 100);

        return hash('sha256', $maskedIp . $userAgent . date('Y-m-d'));
    }

    /**
     * Mask IP address for GDPR compliance
     * IPv4: Zeroes last octet (e.g., 192.168.1.100 -> 192.168.1.0)
     * IPv6: Zeroes last 80 bits
     */
    private function maskIpAddress(?string $ip): string
    {
        if (!$ip) {
            return '0.0.0.0';
        }

        // IPv6
        if (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_IPV6)) {
            $parts = explode(':', $ip);
            // Keep first 3 segments (48 bits), zero the rest
            return implode(':', array_slice($parts, 0, 3)) . '::';
        }

        // IPv4: Zero last octet
        $parts = explode('.', $ip);
        if (count($parts) === 4) {
            $parts[3] = '0';
            return implode('.', $parts);
        }

        return '0.0.0.0';
    }

    /**
     * Store reading event with GDPR-compliant data
     */
    private function storeEvent(array $data, string $visitorId, Request $request): void
    {
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
            // GDPR: Store masked IP only
            'ip_masked' => $this->maskIpAddress($request->ip()),
            // GDPR: Truncate and hash user agent
            'user_agent_hash' => hash('sha256', substr($request->userAgent() ?? '', 0, 200)),
            'created_at' => now(),
        ];

        // Store in Redis with daily key for aggregation
        $redisKey = "reading_events:{$data['postId']}:" . date('Y-m-d');
        $events = Cache::get($redisKey, []);
        $events[] = $eventData;

        // Keep only last 1000 events per day per post to prevent memory issues
        if (count($events) > 1000) {
            $events = array_slice($events, -1000);
        }

        Cache::put($redisKey, $events, 86400 * 7); // Keep for 7 days

        // Log for async batch processing (without PII)
        Log::channel('analytics')->info('Reading event', [
            'event_type' => $eventData['event_type'],
            'post_id' => $eventData['post_id'],
            'milestone' => $eventData['milestone'],
            'engagement_score' => $eventData['engagement_score'],
        ]);
    }

    /**
     * Update cached post analytics with atomic operations
     */
    private function updatePostAnalytics(array $data): void
    {
        $postId = $data['postId'];
        $cacheKey = "post_reading_stats:{$postId}";
        $lockKey = "lock:post_stats:{$postId}";

        // Use atomic lock to prevent race conditions
        Cache::lock($lockKey, 5)->get(function () use ($cacheKey, $data) {
            $stats = Cache::get($cacheKey, [
                'reads' => 0,
                'completions' => 0,
                'totalTimeOnPage' => 0,
                'totalScrollDepth' => 0,
                'totalEngagement' => 0,
                'milestones' => [25 => 0, 50 => 0, 75 => 0, 100 => 0],
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

            // Track milestone achievements
            if ($data['event'] === 'reading_milestone' && isset($data['milestone'])) {
                $milestone = $data['milestone'];
                if (isset($stats['milestones'][$milestone])) {
                    $stats['milestones'][$milestone]++;
                }
            }

            Cache::put($cacheKey, $stats, 86400 * 30); // Keep for 30 days
        });

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
     * Get average time on page (in seconds)
     */
    private function getAvgTimeOnPage(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || ($stats['reads'] ?? 0) === 0) {
            return 0;
        }
        return (int) (($stats['totalTimeOnPage'] ?? 0) / $stats['reads']);
    }

    /**
     * Get average scroll depth (percentage)
     */
    private function getAvgScrollDepth(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || ($stats['reads'] ?? 0) === 0) {
            return 0;
        }
        return (int) (($stats['totalScrollDepth'] ?? 0) / $stats['reads']);
    }

    /**
     * Get average engagement score
     */
    private function getAvgEngagement(int $postId): int
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || ($stats['reads'] ?? 0) === 0) {
            return 0;
        }
        return (int) (($stats['totalEngagement'] ?? 0) / $stats['reads']);
    }

    /**
     * Get completion rate percentage
     */
    private function getCompletionRate(int $postId): float
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        if (!$stats || ($stats['reads'] ?? 0) === 0) {
            return 0;
        }
        return round((($stats['completions'] ?? 0) / $stats['reads']) * 100, 1);
    }

    /**
     * Get milestone breakdown
     */
    private function getMilestoneBreakdown(int $postId): array
    {
        $stats = Cache::get("post_reading_stats:{$postId}");
        return $stats['milestones'] ?? [
            25 => 0,
            50 => 0,
            75 => 0,
            100 => 0,
        ];
    }

    /**
     * Get reads by day for the last 30 days
     */
    private function getReadsByDay(int $postId): array
    {
        $result = [];
        $now = now();

        for ($i = 29; $i >= 0; $i--) {
            $date = $now->copy()->subDays($i)->format('Y-m-d');
            $events = Cache::get("reading_events:{$postId}:{$date}", []);

            // Count unique reads (reading_leave or reading_completion events)
            $reads = collect($events)
                ->filter(fn($e) => in_array($e['event_type'] ?? '', ['reading_leave', 'reading_completion']))
                ->count();

            $result[] = [
                'date' => $date,
                'reads' => $reads,
            ];
        }

        return $result;
    }
}
