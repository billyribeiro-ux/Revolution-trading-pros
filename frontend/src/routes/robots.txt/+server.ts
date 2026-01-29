/**
 * Dynamic robots.txt Generator for Revolution Trading Pros
 * Following Google January 2026 crawling guidelines
 *
 * Updates for January 2026:
 * - Comprehensive AI training bot blocking (all known 2025-2026 AI crawlers)
 * - Google Search Generative Experience (SGE) compatibility
 * - Enhanced crawl directives for modern search engines
 * - Sitemap index support
 * - New AI agents and research crawlers blocked
 * - Image/video AI training bots blocked
 *
 * @version 3.0.0 - January 2026 SEO Standards
 */

import type { RequestHandler } from '@sveltejs/kit';

// Use environment variable - configure VITE_SITE_URL for your domain
const SITE_URL = import.meta.env.VITE_SITE_URL || 'https://revolution-trading-pros.pages.dev';
const LAST_UPDATED = new Date().toISOString().split('T')[0];

const robotsTxt = `# ═══════════════════════════════════════════════════════════════════════════════
# Revolution Trading Pros - robots.txt
# ${SITE_URL}
# Last Updated: ${LAST_UPDATED}
# Compliant with Google January 2026 Standards
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
# Block AI Training Bots (January 2026 Comprehensive List)
# These bots scrape content for AI model training without permission
# Updated with all known AI crawlers as of January 2026
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

User-agent: Google-CloudVertexBot
Disallow: /

# Common Crawl (used for many AI training datasets)
User-agent: CCBot
Disallow: /

# Cohere AI
User-agent: cohere-ai
Disallow: /

User-agent: cohere-training
Disallow: /

# Perplexity AI
User-agent: PerplexityBot
Disallow: /

User-agent: PerplexityLabs
Disallow: /

# Meta AI
User-agent: FacebookBot
Disallow: /

User-agent: Meta-ExternalAgent
Disallow: /

User-agent: Meta-ExternalFetcher
Disallow: /

User-agent: MetaResearchBot
Disallow: /

# ByteDance/TikTok AI
User-agent: Bytespider
Disallow: /

User-agent: ByteDance
Disallow: /

# Amazon/Alexa AI
User-agent: Amazonbot
Disallow: /

User-agent: AmazonResearch
Disallow: /

# Apple AI (distinct from Applebot for Search)
User-agent: Applebot-Extended
Disallow: /

# Microsoft AI Training
User-agent: bingbot-ai
Disallow: /

User-agent: MicrosoftPreview
Disallow: /

# xAI (Grok)
User-agent: xAI-Grok
Disallow: /

User-agent: GrokBot
Disallow: /

# Mistral AI
User-agent: MistralBot
Disallow: /

User-agent: mistral-ai
Disallow: /

# Stability AI
User-agent: StabilityAI
Disallow: /

User-agent: stable-diffusion
Disallow: /

# Midjourney (image training)
User-agent: Midjourney
Disallow: /

# Runway AI (video training)
User-agent: RunwayML
Disallow: /

# Hugging Face Crawlers
User-agent: HuggingFaceBot
Disallow: /

User-agent: huggingface-datasets
Disallow: /

# AI21 Labs
User-agent: AI21Bot
Disallow: /

# Inflection AI (Pi)
User-agent: InflectionBot
Disallow: /

# Adept AI
User-agent: AdeptAI
Disallow: /

# Character.AI
User-agent: CharacterAI
Disallow: /

# Jasper AI
User-agent: JasperBot
Disallow: /

# Writer.com
User-agent: WriterBot
Disallow: /

# Replicate
User-agent: ReplicateBot
Disallow: /

# Together AI
User-agent: TogetherAI
Disallow: /

# Scale AI
User-agent: ScaleAI
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

# New 2025-2026 AI Research Bots
User-agent: DeepMindBot
Disallow: /

User-agent: NVIDIAResearch
Disallow: /

User-agent: IntelAI
Disallow: /

User-agent: SalesforceAI
Disallow: /

User-agent: AdobeAI
Disallow: /

User-agent: SamsungAI
Disallow: /

User-agent: SonyAI
Disallow: /

User-agent: AlibabaAI
Disallow: /

User-agent: TencentAI
Disallow: /

User-agent: BaiduAI
Disallow: /

User-agent: NERSCResearch
Disallow: /

# Web Archive Scrapers (often used for AI training)
User-agent: ia_archiver
Disallow: /

User-agent: archive.org_bot
Disallow: /

# Generic AI Training Patterns
User-agent: *-ai
Disallow: /

User-agent: *-training
Disallow: /

User-agent: *-research
Disallow: /

User-agent: *-dataset
Disallow: /

User-agent: *-llm
Disallow: /

User-agent: *-ml
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
# RSS/Atom Feed Locations
# =============================================================================
# RSS 2.0 Feed
# ${SITE_URL}/feed.xml
# Atom 1.0 Feed
# ${SITE_URL}/atom.xml

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
