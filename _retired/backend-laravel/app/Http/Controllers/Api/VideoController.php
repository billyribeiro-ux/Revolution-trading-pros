<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Video;
use App\Models\VideoAnalytic;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;

class VideoController extends Controller
{
    /**
     * Display a listing of videos.
     */
    public function index(Request $request): JsonResponse
    {
        $query = Video::with('user')->active();

        if ($request->has('platform')) {
            $query->platform($request->platform);
        }

        if ($request->has('search')) {
            $query->where(function ($q) use ($request) {
                $q->where('title', 'like', '%' . $request->search . '%')
                  ->orWhere('description', 'like', '%' . $request->search . '%');
            });
        }

        $videos = $query->latest()->paginate($request->get('per_page', 15));

        return response()->json($videos);
    }

    /**
     * Store a newly created video.
     */
    public function store(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'title' => 'required|string|max:255',
            'description' => 'nullable|string',
            'url' => 'required|url',
            'platform' => 'required|in:youtube,vimeo,wistia,dailymotion,twitch,html5',
            'video_id' => 'nullable|string',
            'thumbnail_url' => 'nullable|url',
            'duration' => 'nullable|integer|min:0',
            'quality' => 'nullable|string',
            'metadata' => 'nullable|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'message' => 'Validation failed',
                'errors' => $validator->errors()
            ], 422);
        }

        $video = Video::create(array_merge(
            $validator->validated(),
            ['user_id' => auth()->id()]
        ));

        return response()->json([
            'message' => 'Video created successfully',
            'video' => $video->load('user')
        ], 201);
    }

    /**
     * Display the specified video.
     */
    public function show(string $id): JsonResponse
    {
        $video = Video::with(['user', 'analytics'])->findOrFail($id);

        return response()->json([
            'video' => $video,
            'stats' => [
                'total_views' => $video->total_views,
                'average_completion_rate' => $video->average_completion_rate,
                'total_watch_time' => $video->total_watch_time,
            ]
        ]);
    }

    /**
     * Update the specified video.
     */
    public function update(Request $request, string $id): JsonResponse
    {
        $video = Video::findOrFail($id);

        $validator = Validator::make($request->all(), [
            'title' => 'sometimes|string|max:255',
            'description' => 'nullable|string',
            'url' => 'sometimes|url',
            'platform' => 'sometimes|in:youtube,vimeo,wistia,dailymotion,twitch,html5',
            'video_id' => 'nullable|string',
            'thumbnail_url' => 'nullable|url',
            'duration' => 'nullable|integer|min:0',
            'quality' => 'nullable|string',
            'is_active' => 'sometimes|boolean',
            'metadata' => 'nullable|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'message' => 'Validation failed',
                'errors' => $validator->errors()
            ], 422);
        }

        $video->update($validator->validated());

        return response()->json([
            'message' => 'Video updated successfully',
            'video' => $video->fresh(['user'])
        ]);
    }

    /**
     * Remove the specified video.
     */
    public function destroy(string $id): JsonResponse
    {
        $video = Video::findOrFail($id);
        $video->delete();

        return response()->json([
            'message' => 'Video deleted successfully'
        ]);
    }

    /**
     * Track video analytics event.
     */
    public function trackEvent(Request $request, string $id): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'session_id' => 'required|string',
            'event_type' => 'required|in:view,play,pause,complete,progress,seek,error,quality_change,speed_change,fullscreen_enter,fullscreen_exit',
            'timestamp_seconds' => 'nullable|integer|min:0',
            'watch_time' => 'nullable|integer|min:0',
            'completion_rate' => 'nullable|numeric|min:0|max:100',
            'interactions' => 'nullable|integer|min:0',
            'quality' => 'nullable|string',
            'buffer_events' => 'nullable|integer|min:0',
            'event_data' => 'nullable|array',
        ]);

        if ($validator->fails()) {
            return response()->json([
                'message' => 'Validation failed',
                'errors' => $validator->errors()
            ], 422);
        }

        $video = Video::findOrFail($id);

        $analytic = VideoAnalytic::create(array_merge(
            $validator->validated(),
            [
                'video_id' => $video->id,
                'user_id' => auth()->id(),
                'ip_address' => $request->ip(),
                'user_agent' => $request->userAgent(),
                'referrer' => $request->header('referer'),
            ]
        ));

        return response()->json([
            'message' => 'Event tracked successfully',
            'analytic' => $analytic
        ], 201);
    }

    /**
     * Get analytics for a video.
     */
    public function analytics(Request $request, string $id): JsonResponse
    {
        $video = Video::findOrFail($id);

        $query = $video->analytics();

        if ($request->has('event_type')) {
            $query->eventType($request->event_type);
        }

        if ($request->has('session_id')) {
            $query->session($request->session_id);
        }

        if ($request->has('from_date')) {
            $query->where('created_at', '>=', $request->from_date);
        }

        if ($request->has('to_date')) {
            $query->where('created_at', '<=', $request->to_date);
        }

        $analytics = $query->latest()->paginate($request->get('per_page', 50));

        // Calculate aggregate stats
        $stats = [
            'total_views' => $video->analytics()->eventType('view')->count(),
            'total_plays' => $video->analytics()->eventType('play')->count(),
            'total_completions' => $video->analytics()->eventType('complete')->count(),
            'average_completion_rate' => $video->analytics()->eventType('complete')->avg('completion_rate') ?? 0,
            'total_watch_time' => $video->analytics()->sum('watch_time'),
            'unique_sessions' => $video->analytics()->distinct('session_id')->count(),
            'total_errors' => $video->analytics()->eventType('error')->count(),
            'average_buffer_events' => $video->analytics()->avg('buffer_events') ?? 0,
        ];

        return response()->json([
            'analytics' => $analytics,
            'stats' => $stats
        ]);
    }

    /**
     * Get video engagement heatmap data.
     */
    public function heatmap(string $id): JsonResponse
    {
        $video = Video::findOrFail($id);

        // Get seek events to build heatmap
        $seekEvents = $video->analytics()
            ->eventType('seek')
            ->get(['timestamp_seconds', 'event_data'])
            ->groupBy('timestamp_seconds')
            ->map(function ($events) {
                return $events->count();
            });

        // Get progress milestones
        $progressEvents = $video->analytics()
            ->eventType('progress')
            ->get(['event_data'])
            ->pluck('event_data')
            ->flatten()
            ->groupBy('percent')
            ->map(function ($events) {
                return $events->count();
            });

        return response()->json([
            'seek_heatmap' => $seekEvents,
            'progress_milestones' => $progressEvents
        ]);
    }
}
