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
     * Find similar pages for a 404 URL using fuzzy matching.
     * Uses Levenshtein distance to find similar URLs from resolved 404s or existing pages.
     */
    public function findSimilar(Request $request)
    {
        $request->validate([
            'url' => 'required|string',
        ]);

        $targetUrl = $request->url;
        $targetPath = parse_url($targetUrl, PHP_URL_PATH) ?? $targetUrl;
        $targetSegments = array_filter(explode('/', trim($targetPath, '/')));

        $suggestions = [];

        // Get resolved 404 errors with redirects as potential suggestions
        $resolved404s = Error404::where('is_resolved', true)
            ->whereNotNull('redirect_url')
            ->limit(100)
            ->get();

        foreach ($resolved404s as $error) {
            $resolvedPath = parse_url($error->url, PHP_URL_PATH) ?? $error->url;
            $similarity = $this->calculateSimilarity($targetPath, $resolvedPath);

            if ($similarity >= 0.5) {
                $suggestions[] = [
                    'url' => $error->redirect_url,
                    'original_url' => $error->url,
                    'similarity' => round($similarity * 100),
                    'type' => 'resolved_404'
                ];
            }
        }

        // Extract keywords from URL path for content-based suggestions
        $keywords = array_filter($targetSegments, fn($s) => strlen($s) > 2);

        if (!empty($keywords)) {
            // Search for similar URLs based on path segments
            $query = Error404::where('is_resolved', true);
            foreach (array_slice($keywords, 0, 3) as $keyword) {
                $query->orWhere('redirect_url', 'like', "%{$keyword}%");
            }

            $keywordMatches = $query->limit(10)->get();
            foreach ($keywordMatches as $match) {
                if ($match->redirect_url && !in_array($match->redirect_url, array_column($suggestions, 'url'))) {
                    $suggestions[] = [
                        'url' => $match->redirect_url,
                        'original_url' => $match->url,
                        'similarity' => 40,
                        'type' => 'keyword_match'
                    ];
                }
            }
        }

        // Sort by similarity and limit results
        usort($suggestions, fn($a, $b) => $b['similarity'] <=> $a['similarity']);
        $suggestions = array_slice($suggestions, 0, 5);

        return response()->json([
            'suggestions' => $suggestions,
            'analyzed_url' => $targetUrl,
        ]);
    }

    /**
     * Calculate similarity between two URL paths using Levenshtein distance.
     */
    private function calculateSimilarity(string $path1, string $path2): float
    {
        $path1 = strtolower(trim($path1, '/'));
        $path2 = strtolower(trim($path2, '/'));

        if ($path1 === $path2) {
            return 1.0;
        }

        $maxLen = max(strlen($path1), strlen($path2));
        if ($maxLen === 0) {
            return 1.0;
        }

        $distance = levenshtein($path1, $path2);
        return 1 - ($distance / $maxLen);
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
