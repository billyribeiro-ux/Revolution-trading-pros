/**
 * Pinterest Tag Vendor Integration
 *
 * Integrates Pinterest's conversion tracking with consent-aware loading.
 *
 * @see https://help.pinterest.com/en/business/article/track-conversions-with-pinterest-tag
 * @module consent/vendors/pinterest
 * @version 1.0.0
 */

import { browser, dev } from '$app/environment';
import type { VendorConfig } from '../types';

declare global {
	interface Window {
		pintrk?: {
			(...args: unknown[]): void;
			queue?: unknown[][];
			version?: string;
		};
	}
}

const PUBLIC_PINTEREST_TAG_ID = import.meta.env['PUBLIC_PINTEREST_TAG_ID'] || '';
let pinterestReady = false;
const eventQueue: Array<{ event: string; data?: Record<string, unknown> }> = [];

/**
 * Initialize Pinterest Tag
 */
function initializePinterest(tagId: string): void {
	if (!browser || !tagId) return;

	// Create pintrk stub if not present
	if (!window.pintrk) {
		window.pintrk = function (...args: unknown[]) {
			window.pintrk!.queue = window.pintrk!.queue || [];
			window.pintrk!.queue.push(args);
		};
		window.pintrk.version = '3.0';
	}

	// Load Pinterest script
	const script = document.createElement('script');
	script.async = true;
	script.src = 'https://s.pinimg.com/ct/core.js';
	script.onload = () => {
		window.pintrk?.('load', tagId);
		window.pintrk?.('page');
		pinterestReady = true;
		processEventQueue();
		console.debug('[Pinterest] Tag initialized:', tagId);
	};

	document.head.appendChild(script);
}

/**
 * Process queued events
 */
function processEventQueue(): void {
	if (!window.pintrk || !pinterestReady) return;

	while (eventQueue.length > 0) {
		const { event, data } = eventQueue.shift()!;
		window.pintrk('track', event, data);
	}
}

/**
 * Track a Pinterest event
 */
export function trackPinterestEvent(
	event: string,
	data?: Record<string, unknown>
): void {
	if (!browser) return;

	if (!pinterestReady) {
		eventQueue.push({ event, ...(data && { data }) });
		return;
	}

	window.pintrk?.('track', event, data);
	console.debug('[Pinterest] Tracked event:', event, data);
}

/**
 * Track page view
 */
export function trackPinterestPageView(): void {
	if (!browser) return;

	if (!pinterestReady) {
		eventQueue.push({ event: 'pagevisit' });
		return;
	}

	window.pintrk?.('page');
	console.debug('[Pinterest] Tracked page view');
}

/**
 * Check if Pinterest Tag is ready
 */
export function isPinterestReady(): boolean {
	return pinterestReady && !!window.pintrk;
}

/**
 * Standard Pinterest events
 */
export const PINTEREST_EVENTS = {
	PAGE_VISIT: 'pagevisit',
	VIEW_CATEGORY: 'viewcategory',
	SEARCH: 'search',
	ADD_TO_CART: 'addtocart',
	CHECKOUT: 'checkout',
	WATCH_VIDEO: 'watchvideo',
	SIGNUP: 'signup',
	LEAD: 'lead',
	CUSTOM: 'custom',
} as const;

/**
 * Pinterest Tag vendor configuration
 */
export const pinterestVendor: VendorConfig = {
	id: 'pinterest',
	name: 'Pinterest Tag',
	description:
		'Pinterest Tag tracks conversions and user interactions from Pinterest advertising campaigns.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://policy.pinterest.com/en/privacy-policy',
	cookies: [
		{
			name: '_pinterest_*',
			purpose: 'Pinterest tracking and advertising',
			duration: '1 year',
			type: 'third-party',
		},
		{
			name: '_pin_unauth',
			purpose: 'Pinterest unauthenticated user tracking',
			duration: '1 year',
			type: 'third-party',
		},
	],
	dataLocations: ['United States'],
	supportsRevocation: true,

	load: () => {
		const tagId = PUBLIC_PINTEREST_TAG_ID;
		if (!tagId) {
			if (!dev) console.warn('[Pinterest] Missing PUBLIC_PINTEREST_TAG_ID environment variable');
			return;
		}
		initializePinterest(tagId);
	},

	onConsentRevoked: () => {
		pinterestReady = false;
		eventQueue.length = 0;
		console.debug('[Pinterest] Consent revoked');
	},
};
