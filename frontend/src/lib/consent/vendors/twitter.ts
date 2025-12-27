/**
 * Twitter/X Pixel Vendor Integration
 *
 * Integrates Twitter's (X) conversion tracking with consent-aware loading.
 *
 * @see https://business.twitter.com/en/help/campaign-measurement-and-analytics/conversion-tracking.html
 * @module consent/vendors/twitter
 * @version 1.0.0
 */

import { browser, dev } from '$app/environment';
import { env } from '$env/dynamic/public';
import type { VendorConfig } from '../types';

declare global {
	interface Window {
		twq?: {
			(...args: unknown[]): void;
			exe?: (...args: unknown[]) => void;
			version: string;
			queue: unknown[][];
		};
	}
}

let twitterReady = false;
const eventQueue: Array<{ event: string; data?: Record<string, unknown> }> = [];

/**
 * Initialize Twitter Pixel
 */
function initializeTwitter(pixelId: string): void {
	if (!browser || !pixelId) return;

	// Create twq stub if not present
	if (!window.twq) {
		const twq = (window.twq = function (...args: unknown[]) {
			if (twq.exe) {
				twq.exe.apply(twq, args);
			} else {
				twq.queue.push(args);
			}
		} as typeof window.twq);

		twq!.version = '1.1';
		twq!.queue = [];
	}

	// Load Twitter script
	const script = document.createElement('script');
	script.async = true;
	script.src = 'https://static.ads-twitter.com/uwt.js';
	script.onload = () => {
		window.twq?.('config', pixelId);
		twitterReady = true;
		processEventQueue();
		console.debug('[Twitter] Pixel initialized:', pixelId);
	};

	document.head.appendChild(script);
}

/**
 * Process queued events
 */
function processEventQueue(): void {
	if (!window.twq || !twitterReady) return;

	while (eventQueue.length > 0) {
		const { event, data } = eventQueue.shift()!;
		window.twq!('event', event, data || {});
	}
}

/**
 * Track a Twitter event
 */
export function trackTwitterEvent(
	event: string,
	data?: Record<string, unknown>
): void {
	if (!browser) return;

	if (!twitterReady) {
		eventQueue.push({ event, data });
		return;
	}

	window.twq?.('event', event, data || {});
	console.debug('[Twitter] Tracked event:', event, data);
}

/**
 * Track page view
 */
export function trackTwitterPageView(): void {
	if (!browser) return;

	if (!twitterReady) {
		eventQueue.push({ event: 'tw-PageView' });
		return;
	}

	window.twq?.('track', 'PageView');
	console.debug('[Twitter] Tracked page view');
}

/**
 * Check if Twitter Pixel is ready
 */
export function isTwitterReady(): boolean {
	return twitterReady && !!window.twq;
}

/**
 * Standard Twitter events
 */
export const TWITTER_EVENTS = {
	PAGE_VIEW: 'tw-PageView',
	PURCHASE: 'tw-Purchase',
	SIGN_UP: 'tw-SignUp',
	DOWNLOAD: 'tw-Download',
	ADD_TO_CART: 'tw-AddToCart',
	ADD_TO_WISHLIST: 'tw-AddToWishlist',
	CHECKOUT_INITIATED: 'tw-CheckoutInitiated',
	CONTENT_VIEW: 'tw-ContentView',
	LEAD: 'tw-Lead',
	SEARCH: 'tw-Search',
	CUSTOM: 'tw-Custom',
} as const;

/**
 * Twitter Pixel vendor configuration
 */
export const twitterVendor: VendorConfig = {
	id: 'twitter',
	name: 'Twitter/X Pixel',
	description:
		'Twitter Pixel tracks ad conversions and user interactions to measure advertising ROI.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://twitter.com/en/privacy',
	cookies: [
		{
			name: '_twitter_*',
			purpose: 'Twitter tracking and advertising',
			duration: '2 years',
			type: 'third-party',
		},
		{
			name: 'muc_ads',
			purpose: 'Twitter advertising cookie',
			duration: '2 years',
			type: 'third-party',
		},
		{
			name: 'personalization_id',
			purpose: 'Twitter personalization',
			duration: '2 years',
			type: 'third-party',
		},
	],
	dataLocations: ['United States'],
	supportsRevocation: true,

	load: () => {
		const pixelId = env.PUBLIC_TWITTER_PIXEL_ID;
		if (!pixelId) {
			if (!dev) console.warn('[Twitter] Missing PUBLIC_TWITTER_PIXEL_ID environment variable');
			return;
		}
		initializeTwitter(pixelId);
	},

	onConsentRevoked: () => {
		twitterReady = false;
		eventQueue.length = 0;
		console.debug('[Twitter] Consent revoked');
	},
};
