<?php

namespace App\Http\Controllers;

use App\Contracts\PostServiceInterface;
use App\Models\Post;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class PostController extends Controller
{
    public function __construct(
        private PostServiceInterface $postService
    ) {}

    /**
     * Get all published posts
     */
    public function index(Request $request): JsonResponse
    {
        $posts = Post::query()
            ->where('status', 'published')
            ->whereNotNull('published_at')
            ->with(['author', 'categories', 'tags'])
            ->orderByDesc('published_at')
            ->paginate($request->get('per_page', 15));

        return response()->json($posts);
    }

    /**
     * Get single post with related posts
     */
    public function show(string $slug): JsonResponse
    {
        $post = Post::where('slug', $slug)
            ->where('status', 'published')
            ->with(['author', 'categories', 'tags', 'media'])
            ->firstOrFail();

        // Get related posts using enterprise service
        $relatedPosts = $this->postService->getRelatedPosts($post, 5, 'hybrid');

        return response()->json([
            'post' => $post,
            'related_posts' => $relatedPosts,
        ]);
    }

    /**
     * Get related posts with debugging scores
     */
    public function relatedWithScores(string $slug): JsonResponse
    {
        $post = Post::where('slug', $slug)
            ->where('status', 'published')
            ->firstOrFail();

        $scoredResults = $this->postService->getRelatedPostsWithScores($post, 10, 'hybrid');

        return response()->json([
            'post' => $post,
            'results' => $scoredResults->map(fn($item) => [
                'post' => $item['post'],
                'score' => $item['score'],
                'reason' => $item['reason'],
            ]),
        ]);
    }

    /**
     * Store new post
     */
    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'title' => 'required|string|max:255',
            'slug' => 'required|string|unique:posts,slug',
            'content' => 'required',
            'excerpt' => 'nullable|string',
            'categories' => 'nullable|array',
            'tags' => 'nullable|array',
            'status' => 'required|in:draft,published',
            'published_at' => 'nullable|date',
        ]);

        $post = Post::create($validated);

        return response()->json($post, 201);
    }

    /**
     * Update post
     */
    public function update(Request $request, string $slug): JsonResponse
    {
        $post = Post::where('slug', $slug)->firstOrFail();

        $validated = $request->validate([
            'title' => 'sometimes|string|max:255',
            'slug' => 'sometimes|string|unique:posts,slug,' . $post->id,
            'content' => 'sometimes',
            'excerpt' => 'nullable|string',
            'categories' => 'nullable|array',
            'tags' => 'nullable|array',
            'status' => 'sometimes|in:draft,published',
            'published_at' => 'nullable|date',
        ]);

        $post->update($validated);

        return response()->json($post);
    }

    /**
     * Delete post
     */
    public function destroy(string $slug): JsonResponse
    {
        $post = Post::where('slug', $slug)->firstOrFail();
        $post->delete();

        return response()->json(['message' => 'Post deleted successfully']);
    }
}
