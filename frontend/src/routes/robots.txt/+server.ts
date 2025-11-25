/**
 * Dynamic robots.txt Generator for Revolution Trading Pros
 * Following Google's 2025 crawling guidelines
 */

import type { RequestHandler } from './$types';

const SITE_URL = 'https://revolutiontradingpros.com';

const robotsTxt = `# Revolution Trading Pros - robots.txt
# https://revolutiontradingpros.com
# Last Updated: ${new Date().toISOString().split('T')[0]}

# =============================================================================
# Default Rules - All Search Engines
# =============================================================================
User-agent: *
Allow: /

# Public content - explicitly allowed
Allow: /blog/
Allow: /courses/
Allow: /indicators/
Allow: /alerts/
Allow: /live-trading-rooms/
Allow: /resources/
Allow: /about
Allow: /our-mission

# Private areas - disallowed
Disallow: /admin/
Disallow: /dashboard/
Disallow: /account/
Disallow: /checkout/
Disallow: /cart/
Disallow: /api/
Disallow: /embed/
Disallow: /crm/
Disallow: /verify-email/

# Password reset pages - no index needed
Disallow: /forgot-password
Disallow: /reset-password

# =============================================================================
# Googlebot Specific Rules
# =============================================================================
User-agent: Googlebot
Allow: /
Crawl-delay: 1

User-agent: Googlebot-Image
Allow: /

# =============================================================================
# Bingbot Specific Rules
# =============================================================================
User-agent: Bingbot
Allow: /
Crawl-delay: 1

# =============================================================================
# Social Media Crawlers
# =============================================================================
User-agent: Twitterbot
Allow: /

User-agent: facebookexternalhit
Allow: /

User-agent: LinkedInBot
Allow: /

# =============================================================================
# Block AI Training Bots (per 2025 standards)
# =============================================================================
User-agent: GPTBot
Disallow: /

User-agent: ChatGPT-User
Disallow: /

User-agent: CCBot
Disallow: /

User-agent: anthropic-ai
Disallow: /

User-agent: Claude-Web
Disallow: /

User-agent: Google-Extended
Disallow: /

# =============================================================================
# Sitemap Location
# =============================================================================
Sitemap: ${SITE_URL}/sitemap.xml

# =============================================================================
# Host Declaration
# =============================================================================
Host: ${SITE_URL}
`;

export const GET: RequestHandler = async () => {
	return new Response(robotsTxt.trim(), {
		headers: {
			'Content-Type': 'text/plain',
			'Cache-Control': 'max-age=3600, s-maxage=86400'
		}
	});
};

// Enable prerendering for static generation
export const prerender = true;
