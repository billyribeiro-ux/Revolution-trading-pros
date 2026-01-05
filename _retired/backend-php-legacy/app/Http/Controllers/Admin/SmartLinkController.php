<?php

declare(strict_types=1);

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\SmartLink;
use App\Models\Contact;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\RedirectResponse;
use Illuminate\Support\Str;

/**
 * Smart Link Controller (FluentCRM Pro Action Links)
 *
 * Handles CRUD operations for smart links and click tracking.
 * December 2025 Laravel 12 syntax.
 */
class SmartLinkController extends Controller
{
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'title', 'click_count', 'unique_clicks'
    ];

    public function index(Request $request): JsonResponse
    {
        $query = SmartLink::query();

        if ($request->filled('search')) {
            $search = $request->search;
            $query->where('title', 'like', "%{$search}%");
        }

        if ($request->has('is_active')) {
            $query->where('is_active', $request->boolean('is_active'));
        }

        $sortBy = in_array($request->get('sort_by'), self::ALLOWED_SORT_COLUMNS, true)
            ? $request->get('sort_by')
            : 'created_at';
        $sortOrder = in_array(strtolower($request->get('sort_order', 'desc')), ['asc', 'desc'], true)
            ? $request->get('sort_order', 'desc')
            : 'desc';

        $links = $query->with('creator')
            ->orderBy($sortBy, $sortOrder)
            ->paginate($request->integer('per_page', 25));

        return response()->json($links);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'target_url' => 'nullable|url|max:2000',
            'actions' => 'nullable|array',
            'actions.*.type' => 'required|string',
            'notes' => 'nullable|string|max:1000',
            'is_active' => 'nullable|boolean',
        ]);

        $link = SmartLink::create([
            'id' => Str::uuid(),
            'created_by' => auth()->id(),
            ...$validated,
        ]);

        return response()->json([
            'link' => $link->load('creator'),
            'short_url' => $link->short_url,
        ], 201);
    }

    public function show(string $id): JsonResponse
    {
        $link = SmartLink::with('creator')->findOrFail($id);
        $stats = $link->getStats();

        return response()->json([
            'link' => $link,
            'short_url' => $link->short_url,
            'stats' => $stats,
        ]);
    }

    public function update(Request $request, string $id): JsonResponse
    {
        $link = SmartLink::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'target_url' => 'nullable|url|max:2000',
            'actions' => 'nullable|array',
            'notes' => 'nullable|string|max:1000',
            'is_active' => 'sometimes|boolean',
        ]);

        $link->update($validated);

        return response()->json([
            'link' => $link->fresh('creator'),
            'short_url' => $link->short_url,
        ]);
    }

    public function destroy(string $id): JsonResponse
    {
        $link = SmartLink::findOrFail($id);
        $link->delete();

        return response()->json(['message' => 'Smart link deleted']);
    }

    public function activate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'uuid',
            'is_active' => 'required|boolean',
        ]);

        $updated = SmartLink::whereIn('id', $validated['ids'])
            ->update(['is_active' => $validated['is_active']]);

        return response()->json([
            'message' => "Updated {$updated} links",
            'updated_count' => $updated,
        ]);
    }

    public function getClicks(Request $request, string $id): JsonResponse
    {
        $link = SmartLink::findOrFail($id);

        $clicks = $link->clicks()
            ->with('contact')
            ->orderByDesc('clicked_at')
            ->paginate($request->integer('per_page', 50));

        return response()->json($clicks);
    }

    public function getAnalytics(string $id): JsonResponse
    {
        $link = SmartLink::findOrFail($id);

        $clicksByDay = $link->clicks()
            ->selectRaw('DATE(clicked_at) as date, COUNT(*) as count')
            ->groupBy('date')
            ->orderByDesc('date')
            ->limit(30)
            ->pluck('count', 'date');

        $clicksByCountry = $link->clicks()
            ->whereNotNull('country')
            ->selectRaw('country, COUNT(*) as count')
            ->groupBy('country')
            ->orderByDesc('count')
            ->limit(10)
            ->pluck('count', 'country');

        $clicksByDevice = $link->clicks()
            ->whereNotNull('device')
            ->selectRaw('device, COUNT(*) as count')
            ->groupBy('device')
            ->orderByDesc('count')
            ->pluck('count', 'device');

        $clicksByBrowser = $link->clicks()
            ->whereNotNull('browser')
            ->selectRaw('browser, COUNT(*) as count')
            ->groupBy('browser')
            ->orderByDesc('count')
            ->limit(10)
            ->pluck('count', 'browser');

        return response()->json([
            'total_clicks' => $link->click_count,
            'unique_clicks' => $link->unique_clicks,
            'clicks_by_day' => $clicksByDay,
            'clicks_by_country' => $clicksByCountry,
            'clicks_by_device' => $clicksByDevice,
            'clicks_by_browser' => $clicksByBrowser,
        ]);
    }
}
