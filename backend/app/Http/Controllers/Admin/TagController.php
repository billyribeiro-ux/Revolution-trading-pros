<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\Tag;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

class TagController extends Controller
{
    /**
     * Display a listing of tags
     */
    public function index(Request $request)
    {
        $query = Tag::query();

        // Search
        if ($request->has('search')) {
            $search = $request->search;
            $query->where(function ($q) use ($search) {
                $q->where('name', 'like', "%{$search}%")
                  ->orWhere('slug', 'like', "%{$search}%");
            });
        }

        // Filter by visibility
        if ($request->has('is_visible')) {
            $query->where('is_visible', $request->boolean('is_visible'));
        }

        // Sort
        $sortBy = $request->get('sort_by', 'order');
        $sortDir = $request->get('sort_dir', 'asc');
        
        if ($sortBy === 'post_count') {
            $tags = $query->get()->sortBy('post_count', SORT_REGULAR, $sortDir === 'desc');
            
            return response()->json([
                'data' => $tags->values(),
                'total' => $tags->count()
            ]);
        }

        $query->orderBy($sortBy, $sortDir);

        // Pagination
        $perPage = $request->get('per_page', 100);
        
        if ($request->boolean('all')) {
            $tags = $query->get();
            return response()->json([
                'data' => $tags,
                'total' => $tags->count()
            ]);
        }

        $tags = $query->paginate($perPage);

        return response()->json($tags);
    }

    /**
     * Store a newly created tag
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'slug' => 'required|string|max:255|unique:tags,slug|regex:/^[a-z0-9-]+$/',
            'color' => 'required|string|max:7|regex:/^#[0-9A-Fa-f]{6}$/',
            'is_visible' => 'boolean',
        ]);

        // Auto-generate slug if not provided
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['name']);
        }

        $tag = Tag::create($validated);

        return response()->json([
            'message' => 'Tag created successfully',
            'data' => $tag
        ], 201);
    }

    /**
     * Display the specified tag
     */
    public function show($id)
    {
        $tag = Tag::findOrFail($id);

        return response()->json($tag);
    }

    /**
     * Update the specified tag
     */
    public function update(Request $request, $id)
    {
        $tag = Tag::findOrFail($id);

        $validated = $request->validate([
            'name' => 'sometimes|required|string|max:255',
            'slug' => [
                'sometimes',
                'required',
                'string',
                'max:255',
                'regex:/^[a-z0-9-]+$/',
                Rule::unique('tags')->ignore($tag->id)
            ],
            'color' => 'sometimes|required|string|max:7|regex:/^#[0-9A-Fa-f]{6}$/',
            'is_visible' => 'boolean',
            'order' => 'sometimes|integer|min:0',
        ]);

        $tag->update($validated);

        return response()->json([
            'message' => 'Tag updated successfully',
            'data' => $tag
        ]);
    }

    /**
     * Remove the specified tag
     */
    public function destroy($id)
    {
        $tag = Tag::findOrFail($id);
        $tag->delete();

        return response()->json([
            'message' => 'Tag deleted successfully'
        ]);
    }

    /**
     * Bulk delete tags
     */
    public function bulkDelete(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'required|integer|exists:tags,id'
        ]);

        $count = Tag::whereIn('id', $validated['ids'])->delete();

        return response()->json([
            'message' => "{$count} tags deleted successfully",
            'deleted_count' => $count
        ]);
    }

    /**
     * Bulk update tag visibility
     */
    public function bulkUpdateVisibility(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'required|integer|exists:tags,id',
            'is_visible' => 'required|boolean'
        ]);

        $count = Tag::whereIn('id', $validated['ids'])
            ->update(['is_visible' => $validated['is_visible']]);

        return response()->json([
            'message' => "{$count} tags updated successfully",
            'updated_count' => $count
        ]);
    }

    /**
     * Reorder tags
     */
    public function reorder(Request $request)
    {
        $validated = $request->validate([
            'orders' => 'required|array',
            'orders.*.id' => 'required|integer|exists:tags,id',
            'orders.*.order' => 'required|integer|min:0'
        ]);

        foreach ($validated['orders'] as $item) {
            Tag::where('id', $item['id'])->update(['order' => $item['order']]);
        }

        return response()->json([
            'message' => 'Tags reordered successfully'
        ]);
    }

    /**
     * Get tag statistics
     */
    public function stats()
    {
        $totalTags = Tag::count();
        $visibleTags = Tag::where('is_visible', true)->count();
        $hiddenTags = $totalTags - $visibleTags;
        
        $tags = Tag::all();
        $totalPosts = $tags->sum('post_count');
        $avgPostsPerTag = $totalTags > 0 ? $totalPosts / $totalTags : 0;
        
        $mostUsed = $tags->sortByDesc('post_count')->first();

        return response()->json([
            'total_tags' => $totalTags,
            'visible_tags' => $visibleTags,
            'hidden_tags' => $hiddenTags,
            'total_posts' => $totalPosts,
            'avg_posts_per_tag' => round($avgPostsPerTag, 2),
            'most_used_tag' => $mostUsed
        ]);
    }

    /**
     * Merge tags
     */
    public function merge(Request $request)
    {
        $validated = $request->validate([
            'source_ids' => 'required|array|min:1',
            'source_ids.*' => 'required|integer|exists:tags,id',
            'target_id' => 'required|integer|exists:tags,id'
        ]);

        $targetTag = Tag::findOrFail($validated['target_id']);
        
        // Delete source tags
        Tag::whereIn('id', $validated['source_ids'])->delete();

        return response()->json([
            'message' => 'Tags merged successfully',
            'target_tag' => $targetTag
        ]);
    }

    /**
     * Export tags
     */
    public function export(Request $request)
    {
        $tags = Tag::all();

        $data = $tags->map(function ($tag) {
            return [
                'id' => $tag->id,
                'name' => $tag->name,
                'slug' => $tag->slug,
                'color' => $tag->color,
                'post_count' => $tag->post_count,
                'is_visible' => $tag->is_visible,
                'created_at' => $tag->created_at->toDateTimeString(),
            ];
        });

        return response()->json([
            'data' => $data,
            'filename' => 'tags_' . now()->format('Y-m-d_His') . '.json'
        ]);
    }
}
