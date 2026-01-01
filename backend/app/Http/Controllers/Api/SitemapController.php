<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Models\Post;
use App\Models\Category;
use App\Models\Tag;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Cache;

/**
 * Sitemap Controller
 *
 * Generates dynamic XML sitemaps for SEO.
 * Supports sitemap index for large sites.
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class SitemapController extends Controller
{
    private const CACHE_TTL = 3600; // 1 hour
    private const MAX_URLS_PER_SITEMAP = 50000;

    /**
     * Generate main sitemap or sitemap index
     */
    public function index(): Response
    {
        $postCount = Post::published()->count();

        // If we have many posts, generate sitemap index
        if ($postCount > self::MAX_URLS_PER_SITEMAP) {
            return $this->sitemapIndex();
        }

        return $this->mainSitemap();
    }

    /**
     * Generate sitemap index for large sites
     */
    public function sitemapIndex(): Response
    {
        $xml = Cache::remember('sitemap:index', self::CACHE_TTL, function () {
            $postCount = Post::published()->count();
            $sitemapCount = ceil($postCount / self::MAX_URLS_PER_SITEMAP);

            $content = '<?xml version="1.0" encoding="UTF-8"?>' . PHP_EOL;
            $content .= '<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">' . PHP_EOL;

            // Posts sitemaps
            for ($i = 1; $i <= $sitemapCount; $i++) {
                $content .= '  <sitemap>' . PHP_EOL;
                $content .= '    <loc>' . url("/api/sitemap/posts/{$i}") . '</loc>' . PHP_EOL;
                $content .= '    <lastmod>' . now()->toW3cString() . '</lastmod>' . PHP_EOL;
                $content .= '  </sitemap>' . PHP_EOL;
            }

            // Categories sitemap
            $content .= '  <sitemap>' . PHP_EOL;
            $content .= '    <loc>' . url('/api/sitemap/categories') . '</loc>' . PHP_EOL;
            $content .= '    <lastmod>' . now()->toW3cString() . '</lastmod>' . PHP_EOL;
            $content .= '  </sitemap>' . PHP_EOL;

            // Tags sitemap
            $content .= '  <sitemap>' . PHP_EOL;
            $content .= '    <loc>' . url('/api/sitemap/tags') . '</loc>' . PHP_EOL;
            $content .= '    <lastmod>' . now()->toW3cString() . '</lastmod>' . PHP_EOL;
            $content .= '  </sitemap>' . PHP_EOL;

            $content .= '</sitemapindex>';

            return $content;
        });

        return response($xml, 200)
            ->header('Content-Type', 'application/xml')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Generate main sitemap with all URLs
     */
    public function mainSitemap(): Response
    {
        $xml = Cache::remember('sitemap:main', self::CACHE_TTL, function () {
            $content = $this->buildXmlHeader();

            // Static pages
            $content .= $this->buildUrl(url('/'), now(), 'daily', '1.0');
            $content .= $this->buildUrl(url('/blog'), now(), 'daily', '0.9');
            $content .= $this->buildUrl(url('/about'), now(), 'monthly', '0.7');
            $content .= $this->buildUrl(url('/contact'), now(), 'monthly', '0.6');

            // Blog posts
            $posts = Post::published()
                ->select('slug', 'updated_at', 'published_at')
                ->orderBy('published_at', 'desc')
                ->get();

            foreach ($posts as $post) {
                $content .= $this->buildUrl(
                    url("/blog/{$post->slug}"),
                    $post->updated_at ?? $post->published_at,
                    'weekly',
                    '0.8'
                );
            }

            // Categories
            $categories = Category::where('is_active', true)
                ->select('slug', 'updated_at')
                ->get();

            foreach ($categories as $category) {
                $content .= $this->buildUrl(
                    url("/blog/category/{$category->slug}"),
                    $category->updated_at,
                    'weekly',
                    '0.6'
                );
            }

            // Tags
            $tags = Tag::select('slug', 'updated_at')->get();

            foreach ($tags as $tag) {
                $content .= $this->buildUrl(
                    url("/blog/tag/{$tag->slug}"),
                    $tag->updated_at,
                    'weekly',
                    '0.5'
                );
            }

            $content .= $this->buildXmlFooter();

            return $content;
        });

        return response($xml, 200)
            ->header('Content-Type', 'application/xml')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Generate posts sitemap (paginated)
     */
    public function posts(int $page = 1): Response
    {
        $xml = Cache::remember("sitemap:posts:{$page}", self::CACHE_TTL, function () use ($page) {
            $posts = Post::published()
                ->select('slug', 'updated_at', 'published_at', 'featured_image')
                ->orderBy('published_at', 'desc')
                ->skip(($page - 1) * self::MAX_URLS_PER_SITEMAP)
                ->take(self::MAX_URLS_PER_SITEMAP)
                ->get();

            $content = $this->buildXmlHeader(true);

            foreach ($posts as $post) {
                $content .= $this->buildUrl(
                    url("/blog/{$post->slug}"),
                    $post->updated_at ?? $post->published_at,
                    'weekly',
                    '0.8',
                    $post->featured_image
                );
            }

            $content .= $this->buildXmlFooter();

            return $content;
        });

        return response($xml, 200)
            ->header('Content-Type', 'application/xml')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Generate categories sitemap
     */
    public function categories(): Response
    {
        $xml = Cache::remember('sitemap:categories', self::CACHE_TTL, function () {
            $categories = Category::where('is_active', true)
                ->select('slug', 'updated_at')
                ->get();

            $content = $this->buildXmlHeader();

            foreach ($categories as $category) {
                $content .= $this->buildUrl(
                    url("/blog/category/{$category->slug}"),
                    $category->updated_at,
                    'weekly',
                    '0.6'
                );
            }

            $content .= $this->buildXmlFooter();

            return $content;
        });

        return response($xml, 200)
            ->header('Content-Type', 'application/xml')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Generate tags sitemap
     */
    public function tags(): Response
    {
        $xml = Cache::remember('sitemap:tags', self::CACHE_TTL, function () {
            $tags = Tag::select('slug', 'updated_at')->get();

            $content = $this->buildXmlHeader();

            foreach ($tags as $tag) {
                $content .= $this->buildUrl(
                    url("/blog/tag/{$tag->slug}"),
                    $tag->updated_at,
                    'weekly',
                    '0.5'
                );
            }

            $content .= $this->buildXmlFooter();

            return $content;
        });

        return response($xml, 200)
            ->header('Content-Type', 'application/xml')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Build XML header
     */
    private function buildXmlHeader(bool $includeImage = false): string
    {
        $namespaces = 'xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"';
        if ($includeImage) {
            $namespaces .= ' xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"';
        }

        return '<?xml version="1.0" encoding="UTF-8"?>' . PHP_EOL .
               "<urlset {$namespaces}>" . PHP_EOL;
    }

    /**
     * Build XML footer
     */
    private function buildXmlFooter(): string
    {
        return '</urlset>';
    }

    /**
     * Build URL entry
     */
    private function buildUrl(
        string $loc,
        $lastmod,
        string $changefreq,
        string $priority,
        ?string $image = null
    ): string {
        $lastmodStr = $lastmod instanceof \DateTimeInterface
            ? $lastmod->format('Y-m-d')
            : (is_string($lastmod) ? date('Y-m-d', strtotime($lastmod)) : date('Y-m-d'));

        $content = '  <url>' . PHP_EOL;
        $content .= '    <loc>' . htmlspecialchars($loc) . '</loc>' . PHP_EOL;
        $content .= '    <lastmod>' . $lastmodStr . '</lastmod>' . PHP_EOL;
        $content .= '    <changefreq>' . $changefreq . '</changefreq>' . PHP_EOL;
        $content .= '    <priority>' . $priority . '</priority>' . PHP_EOL;

        // Add image if available
        if ($image) {
            $content .= '    <image:image>' . PHP_EOL;
            $content .= '      <image:loc>' . htmlspecialchars($image) . '</image:loc>' . PHP_EOL;
            $content .= '    </image:image>' . PHP_EOL;
        }

        $content .= '  </url>' . PHP_EOL;

        return $content;
    }

    /**
     * Clear sitemap cache
     */
    public function clearCache(): void
    {
        Cache::forget('sitemap:index');
        Cache::forget('sitemap:main');
        Cache::forget('sitemap:categories');
        Cache::forget('sitemap:tags');

        // Clear paginated post sitemaps
        $postCount = Post::published()->count();
        $sitemapCount = ceil($postCount / self::MAX_URLS_PER_SITEMAP);
        for ($i = 1; $i <= $sitemapCount; $i++) {
            Cache::forget("sitemap:posts:{$i}");
        }
    }
}
