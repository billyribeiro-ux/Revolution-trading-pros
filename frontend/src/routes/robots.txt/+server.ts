/**
 * Dynamic robots.txt Generator for Revolution Trading Pros
 * Following Google November 2025 crawling guidelines
 *
 * Updates for November 2025:
 * - Comprehensive AI training bot blocking (GPTBot, Claude, Gemini, etc.)
 * - Google Search Generative Experience (SGE) compatibility
 * - Enhanced crawl directives for modern search engines
 * - Sitemap index support
 *
 * @version 2.0.0 - November 2025 SEO Standards
 */

import type { RequestHandler } from '@sveltejs/kit';

// Use environment variable - configure VITE_SITE_URL for your domain
const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const LAST_UPDATED = new Date().toISOString().split('T')[0];

const robotsTxt = `# ═══════════════════════════════════════════════════════════════════════════════
# Revolution Trading Pros - robots.txt
# ${SITE_URL}
# Last Updated: ${LAST_UPDATED}
# Compliant with Google November 2025 Standards
# ═══════════════════════════════════════════════════════════════════════════════

# =============================================================================
# Default Rules - All Search Engines
# =============================================================================
User-agent: *
Allow: /

# Public content - explicitly allowed for better crawl efficiency
Allow: /blog/
Allow: /courses/
Allow: /indicators/
Allow: /alerts/
Allow: /live-trading-rooms/
Allow: /resources/
Allow: /about
Allow: /our-mission

# Private/Admin areas - disallowed
Disallow: /admin/
Disallow: /dashboard/
Disallow: /account/
Disallow: /checkout/
Disallow: /cart/
Disallow: /api/
Disallow: /embed/
Disallow: /crm/
Disallow: /verify-email/

# Auth-related pages - no index needed
Disallow: /forgot-password
Disallow: /reset-password

# Technical paths
Disallow: /_app/
Disallow: /.svelte-kit/
Disallow: /node_modules/

# =============================================================================
# Googlebot Specific Rules (Primary Search Engine)
# =============================================================================
User-agent: Googlebot
Allow: /
Crawl-delay: 1

# Allow Googlebot to crawl JavaScript and CSS for proper rendering
Allow: /*.js$
Allow: /*.css$
Allow: /*.json$

User-agent: Googlebot-Image
Allow: /
Allow: /images/
Allow: /*.png$
Allow: /*.jpg$
Allow: /*.jpeg$
Allow: /*.webp$
Allow: /*.svg$

User-agent: Googlebot-Video
Allow: /
Allow: /*.mp4$
Allow: /*.webm$

User-agent: Googlebot-News
Allow: /blog/

# =============================================================================
# Bingbot Specific Rules
# =============================================================================
User-agent: Bingbot
Allow: /
Crawl-delay: 1

User-agent: msnbot
Allow: /
Crawl-delay: 2

# =============================================================================
# Other Major Search Engines
# =============================================================================
User-agent: Yandex
Allow: /
Crawl-delay: 2

User-agent: Baiduspider
Allow: /
Crawl-delay: 3

User-agent: DuckDuckBot
Allow: /
Crawl-delay: 1

User-agent: Applebot
Allow: /

# =============================================================================
# Social Media Crawlers - Allow for rich previews
# =============================================================================
User-agent: Twitterbot
Allow: /

User-agent: facebookexternalhit
Allow: /

User-agent: LinkedInBot
Allow: /

User-agent: Pinterest
Allow: /

User-agent: Slackbot
Allow: /

User-agent: WhatsApp
Allow: /

User-agent: Discordbot
Allow: /

User-agent: TelegramBot
Allow: /

# =============================================================================
# Block AI Training Bots (November 2025 Comprehensive List)
# These bots scrape content for AI model training without permission
# =============================================================================

# OpenAI Crawlers
User-agent: GPTBot
Disallow: /

User-agent: ChatGPT-User
Disallow: /

User-agent: OAI-SearchBot
Disallow: /

# Anthropic Crawlers
User-agent: anthropic-ai
Disallow: /

User-agent: Claude-Web
Disallow: /

User-agent: ClaudeBot
Disallow: /

# Google AI Training (opt-out from Gemini training while allowing Search)
User-agent: Google-Extended
Disallow: /

# Common Crawl (used for many AI training datasets)
User-agent: CCBot
Disallow: /

# Cohere AI
User-agent: cohere-ai
Disallow: /

# Perplexity AI
User-agent: PerplexityBot
Disallow: /

# Meta AI
User-agent: FacebookBot
Disallow: /

User-agent: Meta-ExternalAgent
Disallow: /

User-agent: Meta-ExternalFetcher
Disallow: /

# ByteDance/TikTok AI
User-agent: Bytespider
Disallow: /

# Amazon/Alexa AI
User-agent: Amazonbot
Disallow: /

# Apple AI (distinct from Applebot for Search)
User-agent: Applebot-Extended
Disallow: /

# Microsoft AI Training
User-agent: bingbot-ai
Disallow: /

# Other AI Scrapers
User-agent: Diffbot
Disallow: /

User-agent: Omgilibot
Disallow: /

User-agent: Omgili
Disallow: /

User-agent: YouBot
Disallow: /

User-agent: AI2Bot
Disallow: /

User-agent: Ai2Bot-Dolma
Disallow: /

User-agent: Scrapy
Disallow: /

User-agent: img2dataset
Disallow: /

User-agent: DataForSeoBot
Disallow: /

User-agent: Timpibot
Disallow: /

User-agent: VelenPublicWebCrawler
Disallow: /

User-agent: Webzio-Extended
Disallow: /

User-agent: iscibot
Disallow: /

User-agent: iaskspider
Disallow: /

# =============================================================================
# SEO Tool Bots - Rate limited but allowed
# =============================================================================
User-agent: AhrefsBot
Allow: /
Crawl-delay: 10

User-agent: SemrushBot
Allow: /
Crawl-delay: 10

User-agent: MJ12bot
Allow: /
Crawl-delay: 10

User-agent: DotBot
Allow: /
Crawl-delay: 10

# =============================================================================
# Malicious/Aggressive Bots - Block completely
# =============================================================================
User-agent: MauiBot
Disallow: /

User-agent: SeznamBot
Disallow: /

User-agent: Sogou
Disallow: /

User-agent: Exabot
Disallow: /

User-agent: BLEXBot
Disallow: /

User-agent: Gigabot
Disallow: /

# =============================================================================
# Sitemap Locations
# =============================================================================
Sitemap: ${SITE_URL}/sitemap.xml
Sitemap: ${SITE_URL}/news-sitemap.xml
Sitemap: ${SITE_URL}/video-sitemap.xml

# =============================================================================
# Host Declaration (for search engines that support it)
# =============================================================================
Host: ${SITE_URL}

# =============================================================================
# Additional Directives
# =============================================================================
# Request rate limiting for well-behaved bots
# Note: Crawl-delay is honored by Bing, Yandex, and some others (not Google)
# Google respects rate limits set in Search Console instead
`;

export const GET: RequestHandler = async () => {
	return new Response(robotsTxt.trim(), {
		headers: {
			'Content-Type': 'text/plain; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800',
			'X-Content-Type-Options': 'nosniff'
		}
	});
};

// Enable prerendering for static generation
export const prerender = true;
