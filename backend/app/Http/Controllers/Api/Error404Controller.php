<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Error404;
use Illuminate\Http\Request;

class Error404Controller extends Controller
{
    /**
     * List all 404 errors.
     */
    public function index(Request $request)
    {
        $query = Error404::query();

        if ($request->has('search')) {
            $query->where('url', 'like', "%{$request->search}%");
        }

        if ($request->has('is_resolved')) {
            $query->where('is_resolved', $request->boolean('is_resolved'));
        }

        $errors = $query->orderBy('last_seen_at', 'desc')->paginate(50);

        return response()->json($errors);
    }

    /**
     * Get a single 404 error.
     */
    public function show(int $id)
    {
        $error = Error404::findOrFail($id);

        return response()->json(['error' => $error]);
    }

    /**
     * Resolve a 404 error.
     */
    public function resolve(int $id)
    {
        $error = Error404::findOrFail($id);
        $error->resolve();

        return response()->json(['message' => '404 error resolved']);
    }

    /**
     * Bulk delete 404 errors.
     */
    public function bulkDelete(Request $request)
    {
        $request->validate([
            'resolved_only' => 'required|boolean',
            'older_than_days' => 'nullable|integer|min:1',
        ]);

        $query = Error404::query();

        if ($request->resolved_only) {
            $query->where('is_resolved', true);
        }

        if ($request->older_than_days) {
            $query->where('last_seen_at', '<', now()->subDays($request->older_than_days));
        }

        $deleted = $query->delete();

        return response()->json(['deleted' => $deleted]);
    }

    /**
     * Find similar pages for a 404 URL.
     */
    public function findSimilar(Request $request)
    {
        $request->validate([
            'url' => 'required|string',
        ]);

        // Mock implementation - in production, use fuzzy matching or ML
        return response()->json([
            'suggestions' => [],
        ]);
    }

    /**
     * Track a 404 error (called by middleware).
     */
    public function track(Request $request)
    {
        $request->validate([
            'url' => 'required|string',
            'referrer' => 'nullable|string',
        ]);

        $error = Error404::firstOrNew(['url' => $request->url]);

        if ($error->exists) {
            $error->incrementHits();
        } else {
            $error->fill([
                'referrer' => $request->referrer,
                'first_seen_at' => now(),
                'last_seen_at' => now(),
            ]);
            $error->save();
        }

        return response()->json(['message' => '404 tracked']);
    }
}
