<?php

declare(strict_types=1);

namespace Tests\Feature\Api;

use App\Models\Post;
use App\Models\User;
use App\Models\Category;
use App\Models\Tag;
use App\Enums\PostStatus;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

/**
 * SitemapController API Integration Tests
 *
 * Tests for sitemap generation endpoints.
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class SitemapControllerTest extends TestCase
{
    use RefreshDatabase;

    private User $author;

    protected function setUp(): void
    {
        parent::setUp();

        $this->author = User::factory()->create();
    }

    public function test_can_generate_main_sitemap(): void
    {
        // Create some published posts
        Post::factory()->count(3)->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->get('/api/sitemap');

        $response->assertStatus(200)
            ->assertHeader('Content-Type', 'application/xml; charset=UTF-8')
            ->assertHeader('Cache-Control');

        // Verify XML structure
        $xml = simplexml_load_string($response->getContent());
        $this->assertNotFalse($xml);
        $this->assertEquals('urlset', $xml->getName());
    }

    public function test_sitemap_includes_blog_posts(): void
    {
        $post = Post::factory()->create([
            'slug' => 'test-sitemap-post',
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->get('/api/sitemap');

        $response->assertStatus(200);
        $this->assertStringContainsString('/blog/test-sitemap-post', $response->getContent());
    }

    public function test_sitemap_excludes_draft_posts(): void
    {
        $draftPost = Post::factory()->create([
            'slug' => 'draft-post-sitemap',
            'status' => PostStatus::DRAFT,
            'author_id' => $this->author->id,
        ]);

        $response = $this->get('/api/sitemap');

        $response->assertStatus(200);
        $this->assertStringNotContainsString('/blog/draft-post-sitemap', $response->getContent());
    }

    public function test_can_generate_categories_sitemap(): void
    {
        Category::factory()->create([
            'slug' => 'technology',
            'is_active' => true,
        ]);

        $response = $this->get('/api/sitemap/categories');

        $response->assertStatus(200)
            ->assertHeader('Content-Type', 'application/xml; charset=UTF-8');

        $this->assertStringContainsString('/blog/category/technology', $response->getContent());
    }

    public function test_can_generate_tags_sitemap(): void
    {
        Tag::factory()->create([
            'slug' => 'laravel',
        ]);

        $response = $this->get('/api/sitemap/tags');

        $response->assertStatus(200)
            ->assertHeader('Content-Type', 'application/xml; charset=UTF-8');

        $this->assertStringContainsString('/blog/tag/laravel', $response->getContent());
    }

    public function test_sitemap_has_valid_xml_structure(): void
    {
        Post::factory()->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        $response = $this->get('/api/sitemap');

        $response->assertStatus(200);

        $xml = simplexml_load_string($response->getContent());
        $this->assertNotFalse($xml);

        // Check for required sitemap elements
        $namespaces = $xml->getNamespaces(true);
        $this->assertArrayHasKey('', $namespaces);
        $this->assertEquals('http://www.sitemaps.org/schemas/sitemap/0.9', $namespaces['']);

        // Each URL should have required elements
        foreach ($xml->url as $url) {
            $this->assertNotEmpty((string) $url->loc);
            $this->assertNotEmpty((string) $url->lastmod);
            $this->assertNotEmpty((string) $url->changefreq);
            $this->assertNotEmpty((string) $url->priority);
        }
    }

    public function test_sitemap_is_cached(): void
    {
        Post::factory()->create([
            'status' => PostStatus::PUBLISHED,
            'published_at' => now()->subDay(),
            'author_id' => $this->author->id,
        ]);

        // First request
        $response1 = $this->get('/api/sitemap');
        $content1 = $response1->getContent();

        // Second request should return cached content
        $response2 = $this->get('/api/sitemap');
        $content2 = $response2->getContent();

        $this->assertEquals($content1, $content2);
    }
}
