<?php

declare(strict_types=1);

namespace Tests\Unit\Services\Post;

use App\Contracts\PostServiceInterface;
use App\Models\Post;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\Cache;
use Tests\TestCase;

class PostServiceTest extends TestCase
{
    use RefreshDatabase;

    private PostServiceInterface $service;

    protected function setUp(): void
    {
        parent::setUp();
        
        $this->service = app(PostServiceInterface::class);
        Cache::flush();
    }

    /** @test */
    public function it_returns_related_posts_using_category_algorithm(): void
    {
        $sourcePost = Post::factory()->create([
            'categories' => ['Technology', 'AI'],
            'status' => 'published',
            'published_at' => now(),
        ]);

        $relatedPost = Post::factory()->create([
            'categories' => ['Technology', 'Programming'],
            'status' => 'published',
            'published_at' => now()->subDay(),
        ]);

        $unrelatedPost = Post::factory()->create([
            'categories' => ['Cooking', 'Recipes'],
            'status' => 'published',
            'published_at' => now()->subDays(2),
        ]);

        $results = $this->service->getRelatedPosts($sourcePost, 5, 'category');

        $this->assertCount(1, $results);
        $this->assertTrue($results->contains('id', $relatedPost->id));
        $this->assertFalse($results->contains('id', $unrelatedPost->id));
    }

    /** @test */
    public function it_excludes_source_post_from_results(): void
    {
        $sourcePost = Post::factory()->create([
            'categories' => ['Technology'],
            'status' => 'published',
            'published_at' => now(),
        ]);

        Post::factory()->count(3)->create([
            'categories' => ['Technology'],
            'status' => 'published',
            'published_at' => now()->subDay(),
        ]);

        $results = $this->service->getRelatedPosts($sourcePost, 10, 'category');

        $this->assertFalse($results->contains('id', $sourcePost->id));
    }

    /** @test */
    public function it_respects_limit_parameter(): void
    {
        $sourcePost = Post::factory()->create([
            'categories' => ['Technology'],
            'status' => 'published',
            'published_at' => now(),
        ]);

        Post::factory()->count(10)->create([
            'categories' => ['Technology'],
            'status' => 'published',
            'published_at' => now()->subDay(),
        ]);

        $results = $this->service->getRelatedPosts($sourcePost, 3, 'category');

        $this->assertCount(3, $results);
    }
}
