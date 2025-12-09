<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\TradingRoom;
use App\Models\RoomTrader;
use App\Models\RoomDailyVideo;
use App\Models\RoomLearningContent;
use App\Models\RoomArchive;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

/**
 * Trading Room API Controller
 *
 * Handles all CRUD operations for:
 * - Trading Rooms & Alert Services
 * - Room Traders
 * - Daily Videos
 * - Learning Content
 * - Video Archives
 *
 * @version 1.0.0 - December 2025
 */
class TradingRoomController extends Controller
{
    // ═══════════════════════════════════════════════════════════════════════════
    // TRADING ROOMS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * List all trading rooms and alert services
     */
    public function index(Request $request): JsonResponse
    {
        $query = TradingRoom::query();

        // Filter by type
        if ($request->has('type')) {
            $query->where('type', $request->type);
        }

        // Filter active only
        if ($request->boolean('active_only', true)) {
            $query->active();
        }

        // Include relationships
        if ($request->boolean('with_traders')) {
            $query->with('traders');
        }

        if ($request->boolean('with_counts')) {
            $query->withCount(['dailyVideos', 'learningContent', 'archives']);
        }

        $rooms = $query->ordered()->get();

        return response()->json([
            'success' => true,
            'data' => $rooms,
        ]);
    }

    /**
     * Get a single trading room by slug
     */
    public function show(string $slug): JsonResponse
    {
        $room = TradingRoom::where('slug', $slug)
            ->with(['traders' => fn($q) => $q->active()->ordered()])
            ->withCount(['dailyVideos', 'learningContent', 'archives'])
            ->firstOrFail();

        return response()->json([
            'success' => true,
            'data' => $room,
        ]);
    }

    /**
     * Create a new trading room
     */
    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:trading_rooms',
            'type' => 'required|in:trading_room,alert_service',
            'description' => 'nullable|string',
            'short_description' => 'nullable|string|max:255',
            'icon' => 'nullable|string|max:100',
            'color' => 'nullable|string|max:20',
            'image_url' => 'nullable|url',
            'logo_url' => 'nullable|url',
            'is_active' => 'boolean',
            'is_featured' => 'boolean',
            'sort_order' => 'integer',
            'features' => 'nullable|array',
            'schedule' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        // Generate slug if not provided
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $room = TradingRoom::create($validated);

        return response()->json([
            'success' => true,
            'message' => 'Trading room created successfully',
            'data' => $room,
        ], 201);
    }

    /**
     * Update a trading room
     */
    public function update(Request $request, int $id): JsonResponse
    {
        $room = TradingRoom::findOrFail($id);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'slug' => ['string', 'max:255', Rule::unique('trading_rooms')->ignore($id)],
            'type' => 'in:trading_room,alert_service',
            'description' => 'nullable|string',
            'short_description' => 'nullable|string|max:255',
            'icon' => 'nullable|string|max:100',
            'color' => 'nullable|string|max:20',
            'image_url' => 'nullable|url',
            'logo_url' => 'nullable|url',
            'is_active' => 'boolean',
            'is_featured' => 'boolean',
            'sort_order' => 'integer',
            'features' => 'nullable|array',
            'schedule' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        $room->update($validated);

        return response()->json([
            'success' => true,
            'message' => 'Trading room updated successfully',
            'data' => $room->fresh(),
        ]);
    }

    /**
     * Delete a trading room
     */
    public function destroy(int $id): JsonResponse
    {
        $room = TradingRoom::findOrFail($id);
        $room->delete();

        return response()->json([
            'success' => true,
            'message' => 'Trading room deleted successfully',
        ]);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TRADERS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * List all traders
     */
    public function listTraders(Request $request): JsonResponse
    {
        $query = RoomTrader::query();

        // Filter by room
        if ($request->has('room_slug')) {
            $query->whereHas('tradingRooms', fn($q) => $q->where('slug', $request->room_slug));
        }

        if ($request->boolean('active_only', true)) {
            $query->active();
        }

        $traders = $query->ordered()->withCount('dailyVideos')->get();

        return response()->json([
            'success' => true,
            'data' => $traders,
        ]);
    }

    /**
     * Create a new trader
     */
    public function storeTrader(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'nullable|string|max:255|unique:room_traders',
            'title' => 'nullable|string|max:255',
            'bio' => 'nullable|string',
            'photo_url' => 'nullable|url',
            'email' => 'nullable|email',
            'social_links' => 'nullable|array',
            'specialties' => 'nullable|array',
            'is_active' => 'boolean',
            'sort_order' => 'integer',
            'trading_room_ids' => 'nullable|array',
            'trading_room_ids.*' => 'exists:trading_rooms,id',
        ]);

        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $roomIds = $validated['trading_room_ids'] ?? [];
        unset($validated['trading_room_ids']);

        $trader = RoomTrader::create($validated);

        // Attach to trading rooms
        if (!empty($roomIds)) {
            $trader->tradingRooms()->attach($roomIds);
        }

        return response()->json([
            'success' => true,
            'message' => 'Trader created successfully',
            'data' => $trader->load('tradingRooms'),
        ], 201);
    }

    /**
     * Update a trader
     */
    public function updateTrader(Request $request, int $id): JsonResponse
    {
        $trader = RoomTrader::findOrFail($id);

        $validated = $request->validate([
            'name' => 'string|max:255',
            'slug' => ['string', 'max:255', Rule::unique('room_traders')->ignore($id)],
            'title' => 'nullable|string|max:255',
            'bio' => 'nullable|string',
            'photo_url' => 'nullable|url',
            'email' => 'nullable|email',
            'social_links' => 'nullable|array',
            'specialties' => 'nullable|array',
            'is_active' => 'boolean',
            'sort_order' => 'integer',
            'trading_room_ids' => 'nullable|array',
            'trading_room_ids.*' => 'exists:trading_rooms,id',
        ]);

        $roomIds = $validated['trading_room_ids'] ?? null;
        unset($validated['trading_room_ids']);

        $trader->update($validated);

        // Sync trading rooms if provided
        if ($roomIds !== null) {
            $trader->tradingRooms()->sync($roomIds);
        }

        return response()->json([
            'success' => true,
            'message' => 'Trader updated successfully',
            'data' => $trader->fresh()->load('tradingRooms'),
        ]);
    }

    /**
     * Delete a trader
     */
    public function destroyTrader(int $id): JsonResponse
    {
        $trader = RoomTrader::findOrFail($id);
        $trader->delete();

        return response()->json([
            'success' => true,
            'message' => 'Trader deleted successfully',
        ]);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // DAILY VIDEOS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * List daily videos for a room
     */
    public function listVideos(Request $request, string $roomSlug): JsonResponse
    {
        $room = TradingRoom::where('slug', $roomSlug)->firstOrFail();

        $query = RoomDailyVideo::where('trading_room_id', $room->id)
            ->with('trader');

        // Search
        if ($request->has('search')) {
            $query->search($request->search);
        }

        // Filter by trader
        if ($request->has('trader_id')) {
            $query->byTrader($request->trader_id);
        }

        // Filter published only (default)
        if ($request->boolean('published_only', true)) {
            $query->published();
        }

        // Pagination
        $perPage = $request->integer('per_page', 12);
        $videos = $query->latest()->paginate($perPage);

        return response()->json([
            'success' => true,
            'data' => $videos,
        ]);
    }

    /**
     * Create a new daily video
     */
    public function storeVideo(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'trading_room_id' => 'required|exists:trading_rooms,id',
            'trader_id' => 'nullable|exists:room_traders,id',
            'title' => 'required|string|max:255',
            'description' => 'nullable|string',
            'video_url' => 'required|url',
            'video_platform' => 'required|in:vimeo,youtube,bunny,wistia,direct',
            'video_id' => 'nullable|string|max:255',
            'thumbnail_url' => 'nullable|url',
            'duration' => 'nullable|integer',
            'video_date' => 'required|date',
            'is_featured' => 'boolean',
            'is_published' => 'boolean',
            'tags' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        // Auto-extract video ID if not provided
        if (empty($validated['video_id'])) {
            $validated['video_id'] = RoomDailyVideo::extractVideoId(
                $validated['video_url'],
                $validated['video_platform']
            );
        }

        $video = RoomDailyVideo::create($validated);

        return response()->json([
            'success' => true,
            'message' => 'Video created successfully',
            'data' => $video->load(['tradingRoom', 'trader']),
        ], 201);
    }

    /**
     * Update a daily video
     */
    public function updateVideo(Request $request, int $id): JsonResponse
    {
        $video = RoomDailyVideo::findOrFail($id);

        $validated = $request->validate([
            'trading_room_id' => 'exists:trading_rooms,id',
            'trader_id' => 'nullable|exists:room_traders,id',
            'title' => 'string|max:255',
            'description' => 'nullable|string',
            'video_url' => 'url',
            'video_platform' => 'in:vimeo,youtube,bunny,wistia,direct',
            'video_id' => 'nullable|string|max:255',
            'thumbnail_url' => 'nullable|url',
            'duration' => 'nullable|integer',
            'video_date' => 'date',
            'is_featured' => 'boolean',
            'is_published' => 'boolean',
            'tags' => 'nullable|array',
            'metadata' => 'nullable|array',
        ]);

        $video->update($validated);

        return response()->json([
            'success' => true,
            'message' => 'Video updated successfully',
            'data' => $video->fresh()->load(['tradingRoom', 'trader']),
        ]);
    }

    /**
     * Delete a daily video
     */
    public function destroyVideo(int $id): JsonResponse
    {
        $video = RoomDailyVideo::findOrFail($id);
        $video->delete();

        return response()->json([
            'success' => true,
            'message' => 'Video deleted successfully',
        ]);
    }

    /**
     * Bulk create videos
     */
    public function bulkStoreVideos(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'videos' => 'required|array|min:1',
            'videos.*.trading_room_id' => 'required|exists:trading_rooms,id',
            'videos.*.trader_id' => 'nullable|exists:room_traders,id',
            'videos.*.title' => 'required|string|max:255',
            'videos.*.description' => 'nullable|string',
            'videos.*.video_url' => 'required|url',
            'videos.*.video_platform' => 'required|in:vimeo,youtube,bunny,wistia,direct',
            'videos.*.video_date' => 'required|date',
            'videos.*.is_published' => 'boolean',
        ]);

        $created = [];
        foreach ($validated['videos'] as $videoData) {
            $videoData['video_id'] = RoomDailyVideo::extractVideoId(
                $videoData['video_url'],
                $videoData['video_platform']
            );
            $created[] = RoomDailyVideo::create($videoData);
        }

        return response()->json([
            'success' => true,
            'message' => count($created) . ' videos created successfully',
            'data' => $created,
        ], 201);
    }
}
