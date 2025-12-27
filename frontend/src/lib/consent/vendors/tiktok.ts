/**
 * TikTok Pixel Vendor Integration
 *
 * Integrates TikTok's tracking pixel with consent-aware loading.
 *
 * @see https://ads.tiktok.com/marketing_api/docs
 * @module consent/vendors/tiktok
 * @version 1.0.0
 */

import { browser, dev } from '$app/environment';
import { env } from '$env/dynamic/public';
import type { VendorConfig } from '../types';

interface TikTokPixelInstance {
	_u?: string;
	push?: (args: unknown[]) => void;
}

declare global {
	interface Window {
		ttq?: {
			_i?: Record<string, TikTokPixelInstance | unknown[]>;
			load: (pixelId: string) => void;
			page: () => void;
			track: (event: string, data?: Record<string, unknown>) => void;
			identify: (data: Record<string, unknown>) => void;
			instances?: () => string[];
		};
		TiktokAnalyticsObject?: string;
	}
}

let tiktokReady = false;
const eventQueue: Array<{ event: string; data?: Record<string, unknown> }> = [];

/**
 * Initialize TikTok Pixel
 */
function initializeTikTok(pixelId: string): void {
	if (!browser || !pixelId) return;

	// Create ttq stub if not present
	if (!window.ttq) {
		window.TiktokAnalyticsObject = 'ttq';

		const _i: Record<string, TikTokPixelInstance | unknown[]> = {};

		window.ttq = {
			_i,
			load(t: string) {
				const e = 'https://analytics.tiktok.com/i18n/pixel/events.js';
				const instance: TikTokPixelInstance = { _u: e };
				_i[t] = instance;

				const o = document.createElement('script');
				o.type = 'text/javascript';
				o.async = true;
				o.src = e + '?sdkid=' + t + '&lib=' + 'ttq';

				const a = document.getElementsByTagName('script')[0];
				a.parentNode?.insertBefore(o, a);
			},
			page() {
				Object.values(_i).forEach((instance: unknown) => {
					if (Array.isArray(instance)) {
						instance.push(['page']);
					}
				});
			},
			track(event: string, data?: Record<string, unknown>) {
				Object.values(_i).forEach((instance: unknown) => {
					if (Array.isArray(instance)) {
						instance.push(['track', event, data]);
					}
				});
			},
			identify(data: Record<string, unknown>) {
				Object.values(_i).forEach((instance: unknown) => {
					if (Array.isArray(instance)) {
						instance.push(['identify', data]);
					}
				});
			},
			instances() {
				return Object.keys(_i);
			}
		};
	}

	// Load the pixel
	window.ttq!.load(pixelId);
	window.ttq!.page();

	tiktokReady = true;

	// Process queued events
	processEventQueue();

	console.debug('[TikTok] Pixel initialized:', pixelId);
}

/**
 * Process queued events
 */
function processEventQueue(): void {
	if (!window.ttq || !tiktokReady) return;

	while (eventQueue.length > 0) {
		const { event, data } = eventQueue.shift()!;
		window.ttq.track(event, data);
	}
}

/**
 * Track a TikTok event
 */
export function trackTikTokEvent(
	event: string,
	data?: Record<string, unknown>
): void {
	if (!browser) return;

	if (!tiktokReady) {
		eventQueue.push({ event, data });
		return;
	}

	window.ttq?.track(event, data);
	console.debug('[TikTok] Tracked event:', event, data);
}

/**
 * Track page view
 */
export function trackTikTokPageView(): void {
	if (!browser) return;

	if (!tiktokReady) {
		eventQueue.push({ event: 'PageView' });
		return;
	}

	window.ttq?.page();
	console.debug('[TikTok] Tracked page view');
}

/**
 * Identify a user
 */
export function identifyTikTokUser(data: {
	email?: string;
	phone_number?: string;
	external_id?: string;
}): void {
	if (!browser || !tiktokReady) return;

	window.ttq?.identify(data);
	console.debug('[TikTok] Identified user');
}

/**
 * Check if TikTok Pixel is ready
 */
export function isTikTokReady(): boolean {
	return tiktokReady && !!window.ttq;
}

/**
 * Standard TikTok events
 */
export const TIKTOK_EVENTS = {
	ADD_TO_CART: 'AddToCart',
	COMPLETE_PAYMENT: 'CompletePayment',
	COMPLETE_REGISTRATION: 'CompleteRegistration',
	CONTACT: 'Contact',
	DOWNLOAD: 'Download',
	INITIATE_CHECKOUT: 'InitiateCheckout',
	PLACE_AN_ORDER: 'PlaceAnOrder',
	SEARCH: 'Search',
	SUBMIT_FORM: 'SubmitForm',
	SUBSCRIBE: 'Subscribe',
	VIEW_CONTENT: 'ViewContent',
} as const;

/**
 * TikTok Pixel vendor configuration
 */
export const tiktokVendor: VendorConfig = {
	id: 'tiktok',
	name: 'TikTok Pixel',
	description:
		'TikTok Pixel tracks ad performance and user interactions to optimize advertising campaigns.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://www.tiktok.com/legal/privacy-policy',
	cookies: [
		{
			name: '_ttp',
			purpose: 'TikTok tracking pixel identifier',
			duration: '13 months',
			type: 'third-party',
		},
		{
			name: 'tt_*',
			purpose: 'TikTok analytics cookies',
			duration: 'Session to 13 months',
			type: 'third-party',
		},
	],
	dataLocations: ['United States', 'Singapore'],
	supportsRevocation: true,

	load: () => {
		const pixelId = env.PUBLIC_TIKTOK_PIXEL_ID;
		if (!pixelId) {
			if (!dev) console.warn('[TikTok] Missing PUBLIC_TIKTOK_PIXEL_ID environment variable');
			return;
		}
		initializeTikTok(pixelId);
	},

	onConsentRevoked: () => {
		tiktokReady = false;
		eventQueue.length = 0;
		console.debug('[TikTok] Consent revoked');
	},
};
