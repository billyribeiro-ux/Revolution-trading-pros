<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Cache;

/**
 * Robots.txt Controller
 *
 * Generates dynamic robots.txt based on environment and configuration.
 * Supports different rules for production vs staging.
 *
 * @version 1.0.0 - Lightning Stack Edition
 */
class RobotsController extends Controller
{
    private const CACHE_TTL = 3600; // 1 hour

    /**
     * Generate robots.txt content
     */
    public function index(Request $request): Response
    {
        $content = Cache::remember('robots.txt', self::CACHE_TTL, function () {
            return $this->generateRobotsTxt();
        });

        return response($content, 200)
            ->header('Content-Type', 'text/plain')
            ->header('Cache-Control', 'public, max-age=3600');
    }

    /**
     * Generate robots.txt content based on environment
     */
    private function generateRobotsTxt(): string
    {
        $isProduction = app()->environment('production');
        $siteUrl = config('app.url');

        // Block all crawlers in non-production environments
        if (!$isProduction) {
            return $this->getDisallowAll();
        }

        return $this->getProductionRules($siteUrl);
    }

    /**
     * Get production robots.txt rules
     */
    private function getProductionRules(string $siteUrl): string
    {
        $rules = [];

        // User-agent directive for all crawlers
        $rules[] = '# Revolution Trading Pros - robots.txt';
        $rules[] = '# Generated dynamically by Lightning Stack';
        $rules[] = '';
        $rules[] = 'User-agent: *';

        // Allow general access
        $rules[] = 'Allow: /';

        // Disallow admin and private paths
        $rules[] = '';
        $rules[] = '# Admin & Private Areas';
        $rules[] = 'Disallow: /admin';
        $rules[] = 'Disallow: /admin/';
        $rules[] = 'Disallow: /api/admin/';
        $rules[] = 'Disallow: /dashboard';
        $rules[] = 'Disallow: /dashboard/';
        $rules[] = 'Disallow: /login';
        $rules[] = 'Disallow: /register';
        $rules[] = 'Disallow: /forgot-password';
        $rules[] = 'Disallow: /reset-password';

        // Disallow utility paths
        $rules[] = '';
        $rules[] = '# Utility Paths';
        $rules[] = 'Disallow: /api/';
        $rules[] = 'Allow: /api/sitemap';
        $rules[] = 'Allow: /api/feed/';
        $rules[] = 'Disallow: /*.json$';
        $rules[] = 'Disallow: /_app/';
        $rules[] = 'Disallow: /cdn-cgi/';

        // Disallow search and filter pages (duplicate content)
        $rules[] = '';
        $rules[] = '# Search & Filter Pages (prevent duplicate content)';
        $rules[] = 'Disallow: /*?search=';
        $rules[] = 'Disallow: /*?sort=';
        $rules[] = 'Disallow: /*?filter=';
        $rules[] = 'Disallow: /*?page=';

        // Googlebot-specific rules
        $rules[] = '';
        $rules[] = '# Googlebot Specific';
        $rules[] = 'User-agent: Googlebot';
        $rules[] = 'Allow: /';
        $rules[] = 'Allow: /*.js';
        $rules[] = 'Allow: /*.css';
        $rules[] = 'Disallow: /admin';

        // Bingbot-specific rules (favorable for IndexNow)
        $rules[] = '';
        $rules[] = '# Bingbot Specific';
        $rules[] = 'User-agent: Bingbot';
        $rules[] = 'Allow: /';
        $rules[] = 'Crawl-delay: 1';

        // GPTBot and AI crawlers
        $rules[] = '';
        $rules[] = '# AI Crawlers';
        $rules[] = 'User-agent: GPTBot';
        $rules[] = 'Disallow: /';
        $rules[] = '';
        $rules[] = 'User-agent: ChatGPT-User';
        $rules[] = 'Disallow: /';
        $rules[] = '';
        $rules[] = 'User-agent: Google-Extended';
        $rules[] = 'Disallow: /';
        $rules[] = '';
        $rules[] = 'User-agent: CCBot';
        $rules[] = 'Disallow: /';
        $rules[] = '';
        $rules[] = 'User-agent: anthropic-ai';
        $rules[] = 'Disallow: /';

        // Sitemap location
        $rules[] = '';
        $rules[] = '# Sitemaps';
        $rules[] = "Sitemap: {$siteUrl}/api/sitemap";
        $rules[] = "Sitemap: {$siteUrl}/api/sitemap/categories";
        $rules[] = "Sitemap: {$siteUrl}/api/sitemap/tags";

        // Host directive
        $rules[] = '';
        $rules[] = '# Canonical Host';
        $rules[] = 'Host: ' . parse_url($siteUrl, PHP_URL_HOST);

        return implode("\n", $rules);
    }

    /**
     * Get disallow-all rules for non-production
     */
    private function getDisallowAll(): string
    {
        return implode("\n", [
            '# Revolution Trading Pros - robots.txt',
            '# Non-production environment - all crawling disabled',
            '',
            'User-agent: *',
            'Disallow: /',
        ]);
    }

    /**
     * Clear robots.txt cache
     */
    public function clearCache(): void
    {
        Cache::forget('robots.txt');
    }
}
