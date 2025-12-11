/**
 * Reddit Pixel Vendor Integration
 *
 * Integrates Reddit's conversion tracking with consent-aware loading.
 * Includes Limited Data Use (LDU) support for privacy compliance.
 *
 * @see https://www.redditinc.com/advertising/
 * @module consent/vendors/reddit
 * @version 1.0.0
 */

import { browser, dev } from '$app/environment';
import { env } from '$env/dynamic/public';
import type { VendorConfig } from '../types';

declare global {
	interface Window {
		rdt?: {
			(...args: unknown[]): void;
			callQueue?: unknown[][];
			sendEvent?: (event: string, data?: Record<string, unknown>) => void;
		};
	}
}

let redditReady = false;
let limitedDataUseEnabled = false;
const eventQueue: Array<{ event: string; data?: Record<string, unknown> }> = [];

/**
 * Initialize Reddit Pixel
 */
function initializeReddit(pixelId: string): void {
	if (!browser || !pixelId) return;

	// Create rdt stub if not present
	if (!window.rdt) {
		window.rdt = function (...args: unknown[]) {
			window.rdt!.callQueue = window.rdt!.callQueue || [];
			window.rdt!.callQueue.push(args);
		};
	}

	// Load Reddit script
	const script = document.createElement('script');
	script.async = true;
	script.src = 'https://www.redditstatic.com/ads/pixel.js';
	script.onload = () => {
		window.rdt?.('init', pixelId, {
			optOut: false,
			useDecimalCurrencyValues: true,
			aaid: env.PUBLIC_REDDIT_AAID || '',
		});
		window.rdt?.('track', 'PageVisit');
		redditReady = true;
		processEventQueue();
		console.debug('[Reddit] Pixel initialized:', pixelId);
	};

	document.head.appendChild(script);
}

/**
 * Process queued events
 */
function processEventQueue(): void {
	if (!window.rdt || !redditReady) return;

	while (eventQueue.length > 0) {
		const { event, data } = eventQueue.shift()!;
		window.rdt('track', event, data);
	}
}

/**
 * Track a Reddit event
 */
export function trackRedditEvent(
	event: string,
	data?: Record<string, unknown>
): void {
	if (!browser) return;

	if (!redditReady) {
		eventQueue.push({ event, data });
		return;
	}

	window.rdt?.('track', event, data);
	console.debug('[Reddit] Tracked event:', event, data);
}

/**
 * Track page view
 */
export function trackRedditPageView(): void {
	if (!browser) return;

	if (!redditReady) {
		eventQueue.push({ event: 'PageVisit' });
		return;
	}

	window.rdt?.('track', 'PageVisit');
	console.debug('[Reddit] Tracked page view');
}

/**
 * Check if Reddit Pixel is ready
 */
export function isRedditReady(): boolean {
	return redditReady && !!window.rdt;
}

/**
 * Enable Limited Data Use (LDU) for Reddit
 * Similar to Meta LDU, this restricts data collection for privacy compliance.
 */
export function setRedditLimitedDataUse(enabled: boolean): void {
	if (!browser) return;

	limitedDataUseEnabled = enabled;

	if (window.rdt) {
		window.rdt('set', 'optOut', enabled);
		console.debug('[Reddit] Limited Data Use:', enabled ? 'enabled' : 'disabled');
	}
}

/**
 * Check if Limited Data Use is enabled
 */
export function isRedditLDUEnabled(): boolean {
	return limitedDataUseEnabled;
}

/**
 * Standard Reddit events
 */
export const REDDIT_EVENTS = {
	PAGE_VISIT: 'PageVisit',
	VIEW_CONTENT: 'ViewContent',
	SEARCH: 'Search',
	ADD_TO_CART: 'AddToCart',
	ADD_TO_WISHLIST: 'AddToWishlist',
	PURCHASE: 'Purchase',
	LEAD: 'Lead',
	SIGN_UP: 'SignUp',
	CUSTOM: 'Custom',
} as const;

/**
 * Reddit Pixel vendor configuration
 */
export const redditVendor: VendorConfig = {
	id: 'reddit',
	name: 'Reddit Pixel',
	description:
		'Reddit Pixel tracks conversions and user interactions from Reddit advertising campaigns.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://www.reddit.com/policies/privacy-policy',
	cookies: [
		{
			name: 'rdt_*',
			purpose: 'Reddit tracking and advertising',
			duration: '90 days',
			type: 'third-party',
		},
		{
			name: '_rdt_uuid',
			purpose: 'Reddit unique user identifier',
			duration: '90 days',
			type: 'third-party',
		},
	],
	dataLocations: ['United States'],
	supportsRevocation: true,

	load: () => {
		const pixelId = env.PUBLIC_REDDIT_PIXEL_ID;
		if (!pixelId) {
			if (!dev) console.warn('[Reddit] Missing PUBLIC_REDDIT_PIXEL_ID environment variable');
			return;
		}
		initializeReddit(pixelId);
	},

	onConsentRevoked: () => {
		redditReady = false;
		limitedDataUseEnabled = false;
		eventQueue.length = 0;
		console.debug('[Reddit] Consent revoked');
	},
};
