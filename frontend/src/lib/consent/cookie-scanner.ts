/**
 * Cookie Scanner
 *
 * Automatically detects and categorizes cookies set on the page.
 * Provides transparency about what cookies are active.
 *
 * @module consent/cookie-scanner
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentCategory, CookieInfo } from './types';

/**
 * Known cookie patterns and their categories.
 */
interface CookiePattern {
	pattern: RegExp;
	category: ConsentCategory;
	vendor: string;
	purpose: string;
	duration: string;
}

/**
 * Database of known cookies.
 */
const KNOWN_COOKIES: CookiePattern[] = [
	// Google Analytics
	{
		pattern: /^_ga$/i,
		category: 'analytics',
		vendor: 'Google Analytics',
		purpose: 'Distinguish unique users',
		duration: '2 years'
	},
	{
		pattern: /^_ga_/i,
		category: 'analytics',
		vendor: 'Google Analytics',
		purpose: 'Store session state',
		duration: '2 years'
	},
	{
		pattern: /^_gid$/i,
		category: 'analytics',
		vendor: 'Google Analytics',
		purpose: 'Distinguish users',
		duration: '24 hours'
	},
	{
		pattern: /^_gat/i,
		category: 'analytics',
		vendor: 'Google Analytics',
		purpose: 'Throttle request rate',
		duration: '1 minute'
	},
	{
		pattern: /^_gac_/i,
		category: 'marketing',
		vendor: 'Google Ads',
		purpose: 'Campaign information',
		duration: '90 days'
	},

	// Meta/Facebook
	{
		pattern: /^_fbp$/i,
		category: 'marketing',
		vendor: 'Meta Pixel',
		purpose: 'Identify browsers',
		duration: '3 months'
	},
	{
		pattern: /^_fbc$/i,
		category: 'marketing',
		vendor: 'Meta Pixel',
		purpose: 'Store click identifier',
		duration: '3 months'
	},
	{
		pattern: /^fr$/i,
		category: 'marketing',
		vendor: 'Facebook',
		purpose: 'Ad delivery and measurement',
		duration: '3 months'
	},

	// Google Ads
	{
		pattern: /^_gcl_/i,
		category: 'marketing',
		vendor: 'Google Ads',
		purpose: 'Conversion tracking',
		duration: '90 days'
	},
	{
		pattern: /^IDE$/i,
		category: 'marketing',
		vendor: 'Google DoubleClick',
		purpose: 'Ad targeting',
		duration: '1 year'
	},
	{
		pattern: /^test_cookie$/i,
		category: 'marketing',
		vendor: 'Google DoubleClick',
		purpose: 'Check cookie support',
		duration: 'Session'
	},

	// LinkedIn
	{
		pattern: /^li_/i,
		category: 'marketing',
		vendor: 'LinkedIn',
		purpose: 'Advertising and analytics',
		duration: 'Varies'
	},
	{
		pattern: /^bcookie$/i,
		category: 'marketing',
		vendor: 'LinkedIn',
		purpose: 'Browser identifier',
		duration: '2 years'
	},
	{
		pattern: /^lidc$/i,
		category: 'marketing',
		vendor: 'LinkedIn',
		purpose: 'Load balancing',
		duration: '1 day'
	},

	// Twitter/X
	{
		pattern: /^_twitter/i,
		category: 'marketing',
		vendor: 'Twitter/X',
		purpose: 'Advertising',
		duration: 'Varies'
	},
	{
		pattern: /^muc_ads$/i,
		category: 'marketing',
		vendor: 'Twitter/X',
		purpose: 'Ad targeting',
		duration: '2 years'
	},

	// TikTok
	{
		pattern: /^_ttp$/i,
		category: 'marketing',
		vendor: 'TikTok',
		purpose: 'Track unique visitors',
		duration: '13 months'
	},
	{
		pattern: /^tt_/i,
		category: 'marketing',
		vendor: 'TikTok',
		purpose: 'Advertising',
		duration: 'Varies'
	},

	// Hotjar
	{
		pattern: /^_hj/i,
		category: 'analytics',
		vendor: 'Hotjar',
		purpose: 'Session recording',
		duration: 'Varies'
	},
	{
		pattern: /^hjSession/i,
		category: 'analytics',
		vendor: 'Hotjar',
		purpose: 'Session data',
		duration: '30 minutes'
	},

	// Mixpanel
	{
		pattern: /^mp_/i,
		category: 'analytics',
		vendor: 'Mixpanel',
		purpose: 'Analytics tracking',
		duration: '1 year'
	},

	// Amplitude
	{
		pattern: /^amplitude_/i,
		category: 'analytics',
		vendor: 'Amplitude',
		purpose: 'Analytics tracking',
		duration: '1 year'
	},

	// Segment
	{
		pattern: /^ajs_/i,
		category: 'analytics',
		vendor: 'Segment',
		purpose: 'Analytics routing',
		duration: '1 year'
	},

	// Intercom
	{
		pattern: /^intercom-/i,
		category: 'preferences',
		vendor: 'Intercom',
		purpose: 'Chat widget',
		duration: 'Varies'
	},

	// Zendesk
	{
		pattern: /^__zlcmid$/i,
		category: 'preferences',
		vendor: 'Zendesk',
		purpose: 'Live chat',
		duration: '1 year'
	},

	// Cloudflare
	{
		pattern: /^__cf_bm$/i,
		category: 'necessary',
		vendor: 'Cloudflare',
		purpose: 'Bot management',
		duration: '30 minutes'
	},
	{
		pattern: /^cf_clearance$/i,
		category: 'necessary',
		vendor: 'Cloudflare',
		purpose: 'Security clearance',
		duration: '1 year'
	},

	// Stripe
	{
		pattern: /^__stripe/i,
		category: 'necessary',
		vendor: 'Stripe',
		purpose: 'Payment processing',
		duration: 'Session'
	},

	// Common session/auth cookies
	{
		pattern: /^session/i,
		category: 'necessary',
		vendor: 'Application',
		purpose: 'Session management',
		duration: 'Session'
	},
	{
		pattern: /^auth/i,
		category: 'necessary',
		vendor: 'Application',
		purpose: 'Authentication',
		duration: 'Varies'
	},
	{
		pattern: /^token/i,
		category: 'necessary',
		vendor: 'Application',
		purpose: 'Authentication',
		duration: 'Varies'
	},
	{
		pattern: /^csrf/i,
		category: 'necessary',
		vendor: 'Application',
		purpose: 'Security',
		duration: 'Session'
	},
	{
		pattern: /^xsrf/i,
		category: 'necessary',
		vendor: 'Application',
		purpose: 'Security',
		duration: 'Session'
	},

	// Our consent cookie
	{
		pattern: /^rtp_consent$/i,
		category: 'necessary',
		vendor: 'Revolution Trading Pros',
		purpose: 'Store consent preferences',
		duration: '1 year'
	}
];

/**
 * Parse document.cookie into individual cookies.
 */
function parseCookies(): Map<string, string> {
	const cookies = new Map<string, string>();

	if (!browser) return cookies;

	document.cookie.split(';').forEach((cookie) => {
		const [name, ...valueParts] = cookie.trim().split('=');
		if (name) {
			cookies.set(name, valueParts.join('='));
		}
	});

	return cookies;
}

/**
 * Identify a cookie based on known patterns.
 */
function identifyCookie(name: string): CookiePattern | null {
	for (const pattern of KNOWN_COOKIES) {
		if (pattern.pattern.test(name)) {
			return pattern;
		}
	}
	return null;
}

/**
 * Scanned cookie result.
 */
export interface ScannedCookie extends CookieInfo {
	category: ConsentCategory;
	vendor: string;
	isKnown: boolean;
	value?: string; // Only included if not sensitive
}

/**
 * Cookie scan result.
 */
export interface CookieScanResult {
	timestamp: string;
	totalCookies: number;
	categorizedCookies: number;
	uncategorizedCookies: number;
	cookies: ScannedCookie[];
	byCategory: Record<ConsentCategory, ScannedCookie[]>;
	byVendor: Record<string, ScannedCookie[]>;
}

/**
 * Scan all cookies and categorize them.
 */
export function scanCookies(): CookieScanResult {
	const cookies = parseCookies();
	const scannedCookies: ScannedCookie[] = [];

	cookies.forEach((value, name) => {
		const known = identifyCookie(name);

		const scanned: ScannedCookie = {
			name,
			category: known?.category || 'preferences',
			vendor: known?.vendor || 'Unknown',
			purpose: known?.purpose || 'Unknown purpose',
			duration: known?.duration || 'Unknown',
			type: name.startsWith('_') ? 'third-party' : 'first-party',
			isKnown: !!known
		};

		// Don't expose sensitive cookie values
		if (
			!name.toLowerCase().includes('token') &&
			!name.toLowerCase().includes('auth') &&
			!name.toLowerCase().includes('session') &&
			!name.toLowerCase().includes('key')
		) {
			scanned.value = value.length > 50 ? value.substring(0, 50) + '...' : value;
		}

		scannedCookies.push(scanned);
	});

	// Group by category
	const byCategory: Record<ConsentCategory, ScannedCookie[]> = {
		necessary: [],
		analytics: [],
		marketing: [],
		preferences: []
	};

	scannedCookies.forEach((cookie) => {
		byCategory[cookie.category].push(cookie);
	});

	// Group by vendor
	const byVendor: Record<string, ScannedCookie[]> = {};
	scannedCookies.forEach((cookie) => {
		const vendor = cookie.vendor;
		if (!byVendor[vendor]) {
			byVendor[vendor] = [];
		}
		byVendor[vendor]!.push(cookie);
	});

	return {
		timestamp: new Date().toISOString(),
		totalCookies: scannedCookies.length,
		categorizedCookies: scannedCookies.filter((c) => c.isKnown).length,
		uncategorizedCookies: scannedCookies.filter((c) => !c.isKnown).length,
		cookies: scannedCookies,
		byCategory,
		byVendor
	};
}

/**
 * Get cookies that shouldn't be set without consent.
 */
export function getUnconsentedCookies(allowedCategories: ConsentCategory[]): ScannedCookie[] {
	const scan = scanCookies();

	return scan.cookies.filter((cookie) => {
		// Necessary cookies are always allowed
		if (cookie.category === 'necessary') return false;

		// Check if category is consented
		return !allowedCategories.includes(cookie.category);
	});
}

/**
 * Monitor for new cookies being set.
 */
export function watchCookies(
	callback: (newCookies: ScannedCookie[]) => void,
	intervalMs: number = 5000
): () => void {
	if (!browser) return () => {};

	let lastCookieNames = new Set(parseCookies().keys());

	const interval = setInterval(() => {
		const currentCookies = parseCookies();
		const currentNames = new Set(currentCookies.keys());

		const newNames: string[] = [];
		currentNames.forEach((name) => {
			if (!lastCookieNames.has(name)) {
				newNames.push(name);
			}
		});

		if (newNames.length > 0) {
			const scan = scanCookies();
			const newCookies = scan.cookies.filter((c) => newNames.includes(c.name));
			callback(newCookies);
		}

		lastCookieNames = currentNames;
	}, intervalMs);

	return () => clearInterval(interval);
}

/**
 * Delete a specific cookie.
 */
export function deleteCookie(name: string, domain?: string): void {
	if (!browser) return;

	let cookieString = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/`;

	if (domain) {
		cookieString += `; domain=${domain}`;
	}

	document.cookie = cookieString;

	// Also try without domain
	document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/`;

	console.debug(`[CookieScanner] Deleted cookie: ${name}`);
}

/**
 * Delete all cookies in a category.
 */
export function deleteCookiesByCategory(category: ConsentCategory): number {
	const scan = scanCookies();
	const toDelete = scan.byCategory[category] || [];

	toDelete.forEach((cookie) => {
		deleteCookie(cookie.name);
	});

	console.debug(`[CookieScanner] Deleted ${toDelete.length} ${category} cookies`);
	return toDelete.length;
}

/**
 * Get a summary of cookies for display.
 */
export function getCookieSummary(): {
	total: number;
	byCategory: Record<ConsentCategory, number>;
	vendors: string[];
} {
	const scan = scanCookies();

	return {
		total: scan.totalCookies,
		byCategory: {
			necessary: scan.byCategory.necessary.length,
			analytics: scan.byCategory.analytics.length,
			marketing: scan.byCategory.marketing.length,
			preferences: scan.byCategory.preferences.length
		},
		vendors: Object.keys(scan.byVendor)
	};
}
