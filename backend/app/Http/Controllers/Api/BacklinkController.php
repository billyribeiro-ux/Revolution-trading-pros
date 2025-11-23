<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Backlink;
use Illuminate\Http\Request;

class BacklinkController extends Controller
{
    /**
     * Get backlink profile.
     */
    public function index()
    {
        $backlinks = Backlink::all();

        $profile = [
            'total_backlinks' => $backlinks->count(),
            'total_domains' => $backlinks->pluck('source_url')->unique()->count(),
            'follow_backlinks' => $backlinks->where('is_follow', true)->count(),
            'nofollow_backlinks' => $backlinks->where('is_follow', false)->count(),
            'toxic_backlinks' => $backlinks->where('is_toxic', true)->count(),
            'average_domain_authority' => $backlinks->avg('domain_authority'),
            'backlinks' => $backlinks->take(100),
        ];

        return response()->json(['profile' => $profile]);
    }

    /**
     * Get new backlinks.
     */
    public function new()
    {
        $backlinks = Backlink::where('first_seen_at', '>=', now()->subDays(7))
            ->orderBy('first_seen_at', 'desc')
            ->get();

        return response()->json(['backlinks' => $backlinks]);
    }

    /**
     * Get toxic backlinks.
     */
    public function toxic()
    {
        $backlinks = Backlink::where('is_toxic', true)->get();

        return response()->json(['toxic' => $backlinks]);
    }

    /**
     * Disavow backlinks.
     */
    public function disavow(Request $request)
    {
        $request->validate([
            'domains' => 'required|array',
            'domains.*' => 'string',
        ]);

        // Mock implementation - in production, generate disavow file
        return response()->json([
            'message' => 'Disavow file generated',
            'domains' => $request->domains,
        ]);
    }
}
