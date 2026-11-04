/**
 * SEO Single-Source Configuration (`src/lib/seo/config.ts`)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Canonical configuration for all SEO layers:
 * - Production origin & host policy
 * - Trailing-slash normalization policy
 * - Query-parameter allowlist & tracking denylist
 * - Private/noindex route prefixes
 * - AI training vs. AI search/citation bot rules
 * - Route-to-breadcrumb auto-mapping for visible <Breadcrumbs /> & BreadcrumbList JSON-LD
 */

export const PRODUCTION_ORIGIN = 'https://revolutiontradingpros.com' as const;
export const PREFERRED_HOST = 'revolutiontradingpros.com' as const;

/** Cloudflare static assets bypass hooks; host rules share the SEO origin policy. */
export const STATIC_NOINDEX_HOST_PATTERNS = [
	'https://localhost/*',
	'https://127.0.0.1/*',
	'https://[::1]/*',
	'https://:project.pages.dev/*',
	'https://:version.:project.pages.dev/*'
] as const;
export const TRAILING_SLASH = 'never' as const;
export const DEFAULT_LOCALE = 'en' as const;
export const DEFAULT_OG_LOCALE = 'en_US' as const;
export const SITE_NAME = 'Revolution Trading Pros' as const;
export const SITE_SHORT_NAME = 'RTP' as const;

/**
 * Owner policy (`AI_SEARCH_OPT_OUT`):
 * false = allow AI search/citation bots (OAI-SearchBot, PerplexityBot, ChatGPT-User, Claude-SearchBot)
 *         while blocking AI training crawlers (GPTBot, Google-Extended, ClaudeBot, CCBot).
 */
export const AI_SEARCH_OPT_OUT = false as const;

/**
 * AI training crawlers (blocked in robots.txt per owner policy).
 * Every token is a valid RFC 9309 User-agent product token (no wildcards).
 */
export const AI_TRAINING_BOTS = [
	'GPTBot',
	'Google-Extended',
	'ClaudeBot',
	'anthropic-ai',
	'CCBot',
	'Applebot-Extended',
	'Bytespider',
	'Meta-ExternalAgent',
	'Diffbot',
	'ImagesiftBot',
	'Omgilibot',
	'cohere-training-data-crawler'
] as const;

/**
 * AI search & citation crawlers (allowed when AI_SEARCH_OPT_OUT is false).
 */
export const AI_SEARCH_BOTS = [
	'OAI-SearchBot',
	'ChatGPT-User',
	'PerplexityBot',
	'Claude-SearchBot',
	'Claude-User'
] as const;

/**
 * Query parameters allowed in canonical URLs.
 */
export const QUERY_PARAM_ALLOWLIST = ['page'] as const;

/**
 * Tracking & marketing query parameters stripped from canonical URLs (`URL-04`).
 */
export const TRACKING_PARAM_DENYLIST = [
	'utm_source',
	'utm_medium',
	'utm_campaign',
	'utm_term',
	'utm_content',
	'utm_id',
	'gclid',
	'gclsrc',
	'fbclid',
	'msclkid',
	'dclid',
	'twclid',
	'ttclid',
	'yclid',
	'li_fat_id',
	'mc_cid',
	'mc_eid',
	'ref',
	'igshid',
	'_ga',
	'_gl',
	'_hsenc',
	'_hsmi',
	'hsa_acc',
	'hsa_cam',
	'hsa_grp',
	'hsa_ad',
	'hsa_src',
	'hsa_tgt',
	'hsa_kw',
	'hsa_mt',
	'hsa_net',
	'hsa_ver'
] as const;

/**
 * Private, authenticated, admin, auth, checkout, and internal route prefixes
 * that must NEVER be indexed (`IDX-08`, `IDX-09`, `MAP-01`).
 */
export const NOINDEX_PREFIXES = [
	'/account',
	'/admin',
	'/analytics',
	'/api',
	'/auth',
	'/behavior',
	'/cart',
	'/chatroom-archive',
	'/checkout',
	'/cms',
	'/crm',
	'/daily',
	'/dashboard',
	'/email',
	'/embed',
	'/forgot-password',
	'/learning-center',
	'/login',
	'/logout',
	'/maintenance',
	'/media',
	'/my',
	'/newsletter/confirm',
	'/newsletter/unsubscribe',
	'/popup-advanced-demo',
	'/popup-demo',
	'/posts',
	'/register',
	'/reset-password',
	'/search',
	'/settings',
	'/signup',
	'/test-backend',
	'/tools/options-calculator/embed',
	'/tools/options-calculator/shared',
	'/verify-email',
	'/watchlist',
	'/workbench',
	'/workflows'
] as const;

/**
 * Returns true if the given pathname matches any private/noindex prefix.
 */
export function isNoindexPath(pathname: string): boolean {
	const normalized = pathname === '/' ? '/' : pathname.replace(/\/+$/, '').toLowerCase();
	return NOINDEX_PREFIXES.some((prefix) => {
		const p = prefix.toLowerCase();
		return normalized === p || normalized.startsWith(`${p}/`);
	});
}

/**
 * Canonical URL formatter enforcing `PRODUCTION_ORIGIN` and `TRAILING_SLASH = 'never'`,
 * with root `/` normalized to `https://revolutiontradingpros.com` (or with trailing slash removed
 * consistently across `<link rel="canonical">`, `og:url`, and `/sitemap.xml`).
 */
export interface BreadcrumbItemConfig {
	name: string;
	url: string;
}

const SECTION_LABELS: Record<string, string> = {
	about: 'About Us',
	alerts: 'Trading Alerts',
	authors: 'Authors',
	blog: 'Trading Blog',
	careers: 'Careers',
	classes: 'Trading Classes',
	contact: 'Contact',
	'cookie-policy': 'Cookie Policy',
	courses: 'Trading Courses',
	disclaimer: 'Risk Disclaimer',
	guides: 'Trading Guides',
	indicators: 'Trading Indicators',
	'live-trading-rooms': 'Live Trading Rooms',
	mentorship: '1-on-1 Mentorship',
	'our-mission': 'Our Mission',
	pricing: 'Pricing',
	privacy: 'Privacy Policy',
	refund: 'Refund Policy',
	resources: 'Trading Resources',
	store: 'Store',
	scanners: 'Market Scanners',
	team: 'Our Team',
	terms: 'Terms of Service',
	tools: 'Trading Tools',
	tutorials: 'Video Tutorials'
};

const SLUG_LABELS: Record<string, string> = {
	'explosive-swings': 'Explosive Swings',
	'spx-profit-pulse': 'SPX Profit Pulse',
	'billy-ribeiro': 'Billy Ribeiro',
	'freddie-ferber': 'Freddie Ferber',
	'shao-wen': 'Shao Wen',
	'quickstart-precision-trading-c': 'Quickstart to Precision Trading',
	'tax-loss-harvest-c': 'Tax-Loss Harvesting Mastery',
	'day-trading-masterclass': 'Day Trading Masterclass',
	'options-trading': 'Options Trading Mastery',
	'risk-management': 'Risk Management',
	'swing-trading-pro': 'Swing Trading Pro',
	'exit-strategies': 'Exit Strategies',
	'position-sizing': 'Position Sizing',
	'swing-entry': 'Swing Entry Setups',
	'bollinger-bands': 'Bollinger Bands',
	macd: 'MACD Indicator',
	'moving-averages': 'Moving Averages',
	rsi: 'RSI Indicator',
	stochastic: 'Stochastic Oscillator',
	vwap: 'VWAP Indicator',
	'day-trading': 'Day Trading Room',
	'small-accounts': 'Small Accounts Mentorship',
	'swing-trading': 'Swing Trading Room',
	'etf-stocks-list': 'ETF & Stocks Master List',
	'stock-indexes-list': 'Major Stock Indexes Guide',
	'high-octane-scanner': 'High Octane Scanner',
	'options-calculator': 'Black-Scholes Options Calculator'
};

/**
 * Deterministically builds the breadcrumb trail for any public indexable pathname.
 * Returns an empty array for `/` or private/noindex routes.
 */
export function buildBreadcrumbsForPath(
	pathname: string,
	leafTitle?: string
): BreadcrumbItemConfig[] {
	const cleanPath = pathname === '/' ? '/' : pathname.replace(/\/+$/, '');
	if (cleanPath === '/' || isNoindexPath(cleanPath)) {
		return [];
	}

	const segments = cleanPath.split('/').filter(Boolean);
	const crumbs: BreadcrumbItemConfig[] = [{ name: 'Home', url: `${PRODUCTION_ORIGIN}/` }];

	let accPath = '';
	for (let i = 0; i < segments.length; i++) {
		const seg = segments[i];
		accPath += `/${seg}`;
		const isLast = i === segments.length - 1;

		// Special case: /store/scanners intermediate /store does not have its own standalone page,
		// so point /store to /store/scanners or skip /store intermediate segment
		if (seg === 'store' && segments[i + 1] === 'scanners') {
			continue;
		}
		// Special case: /guides/* and /authors/* parent hubs -> link to /resources and /team
		if (!isLast && seg === 'guides') {
			crumbs.push({ name: 'Trading Guides', url: `${PRODUCTION_ORIGIN}/resources` });
			continue;
		}
		if (!isLast && seg === 'authors') {
			crumbs.push({ name: 'Our Team', url: `${PRODUCTION_ORIGIN}/team` });
			continue;
		}
		if (!isLast && seg === 'tools') {
			crumbs.push({ name: 'Trading Tools', url: `${PRODUCTION_ORIGIN}/resources` });
			continue;
		}

		const label =
			SLUG_LABELS[seg] ??
			SECTION_LABELS[seg] ??
			(isLast && leafTitle
				? leafTitle.replace(/\s*[|—–-]\s*Revolution Trading Pros.*$/i, '').trim()
				: seg
						.split('-')
						.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
						.join(' '));

		crumbs.push({
			name: label,
			url: `${PRODUCTION_ORIGIN}${accPath}`
		});
	}

	return crumbs;
}
