<?php

declare(strict_types=1);

namespace Tests\Feature\Api;

use App\Models\Post;
use App\Models\User;
use App\Models\Category;
use App\Models\Tag;
use App\Enums\PostStatus;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Foundation\Testing\WithFaker;
use Tests\TestCase;

/**
 * PostController API Integration Tests
 *
 * Tests for blog post API endpoints including:
 * - Listing posts with pagination
 * - Filtering and search
 * - Single post retrieval
 * - HTTP caching headers
 * - SEO data generation
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class PostControllerTest extends TestCase
{
    use RefreshDatabase, WithFaker;

    private User $author;
    private Category $category;
    private Tag $tag;

    protected function setUp(): void
    {
        parent::setUp();

        // Create test author
        $this->author = User::factory()->create([
            'name' => 'Test Author',
            'email' => 'author@test.com',
        ]);

        // Create test category
        $this->category = Category::factory()->create([
            'name' => 'Technology',
            'slug' => 'technology',
            'is_active' => true,
        ]);

        // Create test tag
        $this->tag = Tag::factory()->create([
            'name' => 'Laravel',
            'slug' => 'laravel',
        ]);
    }

    // =========================================================================
    // List Posts Tests
    // =========================================================================

    public function test_can_list_published_posts(): void
    {
        // Create published posts
        Post::factory()->count(5)->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        // Create draft posts (should not appear)
        Post::factory()->count(2)->create([
            'status' => PostStatus::DRAFT,
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts');

        $response->assertStatus(200)
            ->assertJsonCount(5, 'data')
            ->assertJsonStructure([
                'data' => [
                    '*' => [
                        'id',
                        'title',
                        'slug',
                        'excerpt',
                        'published_at',
                    ],
                ],
                'current_page',
                'last_page',
                'total',
            ]);
    }

    public function test_posts_are_paginated(): void
    {
        Post::factory()->count(25)->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts?per_page=10');

        $response->assertStatus(200)
            ->assertJsonCount(10, 'data')
            ->assertJsonPath('current_page', 1)
            ->assertJsonPath('last_page', 3)
            ->assertJsonPath('total', 25);
    }

    public function test_can_filter_posts_by_category(): void
    {
        // Create posts with category
        $postsWithCategory = Post::factory()->count(3)->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        foreach ($postsWithCategory as $post) {
            $post->categories()->attach($this->category);
        }

        // Create posts without category
        Post::factory()->count(2)->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson("/api/posts?category_slug={$this->category->slug}");

        $response->assertStatus(200)
            ->assertJsonCount(3, 'data');
    }

    public function test_can_search_posts(): void
    {
        Post::factory()->create([
            'title' => 'Laravel Best Practices',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        Post::factory()->create([
            'title' => 'Vue.js Tutorial',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts?search=Laravel');

        $response->assertStatus(200)
            ->assertJsonCount(1, 'data')
            ->assertJsonPath('data.0.title', 'Laravel Best Practices');
    }

    public function test_posts_are_sorted_by_published_date_desc(): void
    {
        $oldPost = Post::factory()->create([
            'title' => 'Old Post',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subWeek(),
            'author_id' => $this->author->id,
        ]);

        $newPost = Post::factory()->create([
            'title' => 'New Post',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts');

        $response->assertStatus(200)
            ->assertJsonPath('data.0.title', 'New Post')
            ->assertJsonPath('data.1.title', 'Old Post');
    }

    // =========================================================================
    // Single Post Tests
    // =========================================================================

    public function test_can_get_single_post(): void
    {
        $post = Post::factory()->create([
            'title' => 'Test Post',
            'slug' => 'test-post',
            'content_blocks' => [
                ['type' => 'paragraph', 'data' => ['text' => 'Hello world']],
            ],
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson("/api/posts/{$post->slug}");

        $response->assertStatus(200)
            ->assertJsonPath('data.title', 'Test Post')
            ->assertJsonPath('data.slug', 'test-post')
            ->assertJsonStructure([
                'data' => [
                    'id',
                    'title',
                    'slug',
                    'content_blocks',
                    'published_at',
                    'author',
                ],
                'seo',
                'reading',
            ]);
    }

    public function test_returns_404_for_nonexistent_post(): void
    {
        $response = $this->getJson('/api/posts/nonexistent-slug');

        $response->assertStatus(404);
    }

    public function test_draft_posts_are_not_publicly_accessible(): void
    {
        $post = Post::factory()->create([
            'slug' => 'draft-post',
            'status' => PostStatus::DRAFT,
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson("/api/posts/{$post->slug}");

        $response->assertStatus(404);
    }

    // =========================================================================
    // HTTP Caching Tests
    // =========================================================================

    public function test_posts_list_includes_cache_headers(): void
    {
        Post::factory()->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts');

        $response->assertStatus(200)
            ->assertHeader('ETag')
            ->assertHeader('Cache-Control');
    }

    public function test_single_post_includes_cache_headers(): void
    {
        $post = Post::factory()->create([
            'slug' => 'cached-post',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson("/api/posts/{$post->slug}");

        $response->assertStatus(200)
            ->assertHeader('ETag')
            ->assertHeader('Cache-Control');
    }

    public function test_conditional_request_returns_304(): void
    {
        $post = Post::factory()->create([
            'slug' => 'etag-test-post',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        // First request to get ETag
        $firstResponse = $this->getJson("/api/posts/{$post->slug}");
        $etag = $firstResponse->headers->get('ETag');

        // Second request with If-None-Match
        $secondResponse = $this->getJson("/api/posts/{$post->slug}", [
            'If-None-Match' => $etag,
        ]);

        $secondResponse->assertStatus(304);
    }

    // =========================================================================
    // SEO Data Tests
    // =========================================================================

    public function test_single_post_includes_seo_data(): void
    {
        $post = Post::factory()->create([
            'title' => 'SEO Test Post',
            'slug' => 'seo-test-post',
            'meta_title' => 'Custom Meta Title',
            'meta_description' => 'Custom meta description for testing',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson("/api/posts/{$post->slug}");

        $response->assertStatus(200)
            ->assertJsonStructure([
                'seo' => [
                    'title',
                    'description',
                    'canonical',
                    'og',
                    'twitter',
                    'schema',
                ],
            ]);
    }

    // =========================================================================
    // Popular Posts Tests
    // =========================================================================

    public function test_can_get_popular_posts(): void
    {
        // Create posts with different view counts
        Post::factory()->create([
            'title' => 'Most Popular',
            'views_count' => 1000,
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        Post::factory()->create([
            'title' => 'Less Popular',
            'views_count' => 100,
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->getJson('/api/posts/popular');

        $response->assertStatus(200)
            ->assertJsonPath('data.0.title', 'Most Popular');
    }

    // =========================================================================
    // Validation Tests
    // =========================================================================

    public function test_rejects_invalid_per_page_value(): void
    {
        $response = $this->getJson('/api/posts?per_page=1000');

        $response->assertStatus(422)
            ->assertJsonValidationErrors(['per_page']);
    }

    public function test_rejects_invalid_sort_field(): void
    {
        $response = $this->getJson('/api/posts?sort_by=invalid_field');

        $response->assertStatus(422)
            ->assertJsonValidationErrors(['sort_by']);
    }
}
