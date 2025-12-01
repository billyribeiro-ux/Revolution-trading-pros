<?php

namespace App\Http\Controllers\Api\Admin;

use App\Http\Controllers\Controller;
use App\Models\Post;
use Illuminate\Http\Request;
use Illuminate\Support\Str;
use Illuminate\Validation\Rule;

class PostController extends Controller
{
    /**
     * Allowed columns for sorting (SQL injection prevention)
     */
    private const ALLOWED_SORT_COLUMNS = [
        'created_at', 'updated_at', 'published_at', 'title', 'status',
        'view_count', 'comment_count', 'seo_score', 'id'
    ];

    /**
     * Display a listing of posts for admin.
     */
    public function index(Request $request)
    {
        $query = Post::with(['author:id,name']);

        // Filter by status
        if ($request->has('status') && $request->status !== 'all') {
            $query->where('status', $request->status);
        }

        // Filter by category
        if ($request->has('category') && $request->category !== 'all') {
            $query->whereJsonContains('categories', $request->category);
        }

        // Search
        if ($request->has('search') && $request->search) {
            $query->where(function ($q) use ($request) {
                $q->where('title', 'like', '%' . $request->search . '%')
                  ->orWhere('excerpt', 'like', '%' . $request->search . '%')
                  ->orWhere('slug', 'like', '%' . $request->search . '%');
            });
        }

        // Date range filter
        if ($request->has('start_date')) {
            $query->where('created_at', '>=', $request->start_date);
        }
        if ($request->has('end_date')) {
            $query->where('created_at', '<=', $request->end_date);
        }

        // Sorting with whitelist validation (SQL injection prevention)
        $sortBy = $request->input('sort_by', 'created_at');
        $sortOrder = $request->input('sort_order', 'desc');

        if (!in_array($sortBy, self::ALLOWED_SORT_COLUMNS, true)) {
            $sortBy = 'created_at';
        }
        if (!in_array(strtolower($sortOrder), ['asc', 'desc'], true)) {
            $sortOrder = 'desc';
        }

        $query->orderBy($sortBy, $sortOrder);

        // Pagination
        $perPage = $request->input('per_page', 20);
        $posts = $query->paginate($perPage);

        return response()->json([
            'data' => $posts->items(),
            'total' => $posts->total(),
            'current_page' => $posts->currentPage(),
            'last_page' => $posts->lastPage(),
            'per_page' => $posts->perPage(),
        ]);
    }

    /**
     * Get statistics for dashboard.
     */
    public function stats()
    {
        $stats = [
            'total' => Post::count(),
            'published' => Post::where('status', 'published')->count(),
            'draft' => Post::where('status', 'draft')->count(),
            'scheduled' => Post::where('status', 'scheduled')->count(),
            'archived' => Post::where('status', 'archived')->count(),
            'total_views' => Post::sum('view_count'),
            'total_comments' => Post::sum('comment_count'),
            'avg_engagement' => Post::avg('engagement_rate'),
            'avg_seo_score' => Post::avg('seo_score'),
        ];

        return response()->json($stats);
    }

    /**
     * Store a newly created post.
     */
    public function store(Request $request)
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'slug' => 'nullable|string|unique:posts,slug|max:255',
            'excerpt' => 'nullable|string',
            'content_blocks' => 'nullable|array',
            'featured_image' => 'nullable|string',
            'featured_image_alt' => 'nullable|string|max:500',
            'featured_image_title' => 'nullable|string|max:255',
            'featured_image_caption' => 'nullable|string|max:1000',
            'featured_image_description' => 'nullable|string|max:5000',
            'featured_media_id' => 'nullable|integer|exists:media,id',
            'og_image' => 'nullable|string',
            'twitter_card' => 'nullable|string',
            'status' => ['required', Rule::in(['draft', 'published', 'scheduled', 'archived'])],
            'published_at' => 'nullable|date',
            'scheduled_at' => 'nullable|date',
            'auto_publish' => 'nullable|boolean',
            'categories' => 'nullable|array',
            'tags' => 'nullable|array',
            'keywords' => 'nullable|array',
            'reading_time' => 'nullable|string',
            'related_posts' => 'nullable|array',
            'custom_fields' => 'nullable|array',
            'is_featured' => 'nullable|boolean',
            'is_pinned' => 'nullable|boolean',
            'allow_comments' => 'nullable|boolean',
            'visibility' => 'nullable|string',
            'password' => 'nullable|string',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string',
            'indexable' => 'nullable|boolean',
            'canonical_url' => 'nullable|string',
            'schema_markup' => 'nullable|array',
        ]);

        // Generate slug if not provided
        if (empty($validated['slug'])) {
            $validated['slug'] = Str::slug($validated['title']);
            
            // Ensure uniqueness
            $originalSlug = $validated['slug'];
            $count = 1;
            while (Post::where('slug', $validated['slug'])->exists()) {
                $validated['slug'] = $originalSlug . '-' . $count;
                $count++;
            }
        }

        // Set author
        $validated['author_id'] = auth()->id() ?? 1;

        // Calculate word count
        if (isset($validated['content_blocks'])) {
            $validated['word_count'] = $this->calculateWordCount($validated['content_blocks']);
        }

        // Set published_at if status is published and not set
        if ($validated['status'] === 'published' && empty($validated['published_at'])) {
            $validated['published_at'] = now();
        }

        $post = Post::create($validated);

        return response()->json([
            'message' => 'Post created successfully',
            'post' => $post->load('author:id,name')
        ], 201);
    }

    /**
     * Display the specified post.
     */
    public function show(string $id)
    {
        $post = Post::with(['author:id,name'])->findOrFail($id);
        return response()->json(['post' => $post]);
    }

    /**
     * Update the specified post.
     */
    public function update(Request $request, string $id)
    {
        $post = Post::findOrFail($id);

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'slug' => ['sometimes', 'string', 'max:255', Rule::unique('posts')->ignore($post->id)],
            'excerpt' => 'nullable|string',
            'content_blocks' => 'nullable|array',
            'featured_image' => 'nullable|string',
            'featured_image_alt' => 'nullable|string|max:500',
            'featured_image_title' => 'nullable|string|max:255',
            'featured_image_caption' => 'nullable|string|max:1000',
            'featured_image_description' => 'nullable|string|max:5000',
            'featured_media_id' => 'nullable|integer|exists:media,id',
            'og_image' => 'nullable|string',
            'twitter_card' => 'nullable|string',
            'status' => ['sometimes', Rule::in(['draft', 'published', 'scheduled', 'archived'])],
            'published_at' => 'nullable|date',
            'scheduled_at' => 'nullable|date',
            'auto_publish' => 'nullable|boolean',
            'categories' => 'nullable|array',
            'tags' => 'nullable|array',
            'keywords' => 'nullable|array',
            'reading_time' => 'nullable|string',
            'related_posts' => 'nullable|array',
            'custom_fields' => 'nullable|array',
            'is_featured' => 'nullable|boolean',
            'is_pinned' => 'nullable|boolean',
            'allow_comments' => 'nullable|boolean',
            'visibility' => 'nullable|string',
            'password' => 'nullable|string',
            'meta_title' => 'nullable|string|max:255',
            'meta_description' => 'nullable|string',
            'indexable' => 'nullable|boolean',
            'canonical_url' => 'nullable|string',
            'schema_markup' => 'nullable|array',
        ]);

        // Update word count if content changed
        if (isset($validated['content_blocks'])) {
            $validated['word_count'] = $this->calculateWordCount($validated['content_blocks']);
        }

        // Update version and edit tracking
        $validated['version'] = $post->version + 1;
        $validated['last_edited_at'] = now();
        $validated['last_edited_by'] = auth()->id();

        // Set published_at if changing to published
        if (isset($validated['status']) && $validated['status'] === 'published' && !$post->published_at) {
            $validated['published_at'] = now();
        }

        $post->update($validated);

        return response()->json([
            'message' => 'Post updated successfully',
            'post' => $post->fresh()->load('author:id,name')
        ]);
    }

    /**
     * Remove the specified post.
     */
    public function destroy(string $id)
    {
        $post = Post::findOrFail($id);
        $post->delete();

        return response()->json([
            'message' => 'Post deleted successfully'
        ]);
    }

    /**
     * Bulk delete posts.
     */
    public function bulkDelete(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'integer|exists:posts,id'
        ]);

        Post::whereIn('id', $validated['ids'])->delete();

        return response()->json([
            'message' => count($validated['ids']) . ' posts deleted successfully'
        ]);
    }

    /**
     * Bulk update post status.
     */
    public function bulkUpdateStatus(Request $request)
    {
        $validated = $request->validate([
            'ids' => 'required|array',
            'ids.*' => 'integer|exists:posts,id',
            'status' => ['required', Rule::in(['draft', 'published', 'scheduled', 'archived'])]
        ]);

        $updateData = ['status' => $validated['status']];
        
        // Set published_at if publishing
        if ($validated['status'] === 'published') {
            $updateData['published_at'] = now();
        }

        Post::whereIn('id', $validated['ids'])->update($updateData);

        return response()->json([
            'message' => count($validated['ids']) . ' posts updated successfully'
        ]);
    }

    /**
     * Get analytics for a specific post.
     */
    public function analytics(string $id)
    {
        $post = Post::findOrFail($id);

        $analytics = [
            'views' => $post->view_count,
            'comments' => $post->comment_count,
            'shares' => $post->share_count,
            'likes' => $post->like_count,
            'engagement_rate' => $post->engagement_rate,
            'ctr' => $post->ctr,
            'avg_time_on_page' => $post->avg_time_on_page,
            'seo_score' => $post->seo_score,
            'readability_score' => $post->readability_score,
            'word_count' => $post->word_count,
        ];

        return response()->json(['analytics' => $analytics]);
    }

    /**
     * Export posts.
     */
    public function export(Request $request)
    {
        $format = $request->input('format', 'csv');
        $posts = Post::with('author:id,name')->get();

        // Implementation depends on format
        // For now, return JSON
        return response()->json(['posts' => $posts]);
    }

    /**
     * Calculate word count from content blocks.
     */
    private function calculateWordCount(array $contentBlocks): int
    {
        $text = '';
        foreach ($contentBlocks as $block) {
            if (isset($block['content'])) {
                $text .= ' ' . strip_tags($block['content']);
            }
        }
        return str_word_count($text);
    }
}
