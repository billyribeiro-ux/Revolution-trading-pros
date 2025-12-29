/**
 * Meta (Facebook) Pixel Vendor Configuration
 *
 * Implements Meta Pixel integration with consent-aware loading.
 * The pixel is loaded only when marketing consent is granted.
 *
 * Configuration:
 *   Set PUBLIC_META_PIXEL_ID in your environment (.env or .env.local):
 *   PUBLIC_META_PIXEL_ID=1234567890
 *
 * Verification:
 *   Use Meta Pixel Helper Chrome extension to verify pixel events:
 *   https://www.facebook.com/business/help/198406866570498
 *
 * Limited Data Use (LDU):
 *   For California (CCPA) compliance, you may want to enable Limited Data Use.
 *   See: https://developers.facebook.com/docs/marketing-apis/data-processing-options
 *
 * @module consent/vendors/meta-pixel
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { VendorConfig } from '../types';

// Use environment variable (optional at build time)
const PUBLIC_META_PIXEL_ID = import.meta.env['PUBLIC_META_PIXEL_ID'] || '';

/**
 * Type definitions for Meta Pixel (fbq).
 */
interface Fbq {
	(method: 'init', pixelId: string): void;
	(method: 'track', eventName: string, params?: Record<string, unknown>): void;
	(method: 'trackCustom', eventName: string, params?: Record<string, unknown>): void;
	(method: 'consent', action: 'grant' | 'revoke'): void;
	(method: 'dataProcessingOptions', options: string[], country?: number, state?: number): void;
	callMethod?: (...args: unknown[]) => void;
	queue: unknown[];
	loaded: boolean;
	version: string;
	push: (...args: unknown[]) => void;
}

// Window interface extended in src/app.d.ts

/**
 * Track if Meta Pixel has been initialized.
 */
let pixelInitialized = false;

/**
 * Track if consent has been granted to the pixel.
 */
let pixelConsentGranted = false;

/**
 * Initialize the fbq function stub (before script loads).
 */
function initFbqStub(): void {
	if (!browser) return;

	if (window.fbq) return;

	const fbq: Fbq = function (...args: unknown[]) {
		if (fbq.callMethod) {
			fbq.callMethod.apply(fbq, args);
		} else {
			fbq.queue.push(args);
		}
	} as Fbq;

	fbq.queue = [];
	fbq.loaded = true;
	fbq.version = '2.0';
	fbq.push = fbq as unknown as (...args: unknown[]) => void;

	window.fbq = fbq;
	window._fbq = fbq;
}

/**
 * Inject the Meta Pixel base code script.
 */
function injectPixelScript(): Promise<void> {
	return new Promise((resolve, reject) => {
		if (!browser) {
			resolve();
			return;
		}

		// Check if script already exists
		if (document.getElementById('meta-pixel-script')) {
			resolve();
			return;
		}

		const script = document.createElement('script');
		script.id = 'meta-pixel-script';
		script.async = true;
		script.src = 'https://connect.facebook.net/en_US/fbevents.js';

		script.onload = () => resolve();
		script.onerror = () => reject(new Error('Failed to load Meta Pixel script'));

		// Insert before the first script tag
		const firstScript = document.getElementsByTagName('script')[0];
		if (firstScript?.parentNode) {
			firstScript.parentNode.insertBefore(script, firstScript);
		} else {
			document.head.appendChild(script);
		}
	});
}

/**
 * Track a standard Meta Pixel event.
 *
 * @param eventName - Standard event name (e.g., 'PageView', 'Purchase', 'Lead')
 * @param params - Optional event parameters
 *
 * @see https://developers.facebook.com/docs/meta-pixel/reference#standard-events
 */
export function trackPixelEvent(
	eventName: string,
	params?: Record<string, unknown>
): void {
	if (!browser || !pixelInitialized || !pixelConsentGranted) {
		console.debug('[MetaPixel] Cannot track event - pixel not ready or consent not granted');
		return;
	}

	window.fbq!('track', eventName, params);
	console.debug('[MetaPixel] Tracked event:', eventName, params);
}

/**
 * Track a custom Meta Pixel event.
 *
 * @param eventName - Custom event name
 * @param params - Optional event parameters
 */
export function trackCustomPixelEvent(
	eventName: string,
	params?: Record<string, unknown>
): void {
	if (!browser || !pixelInitialized || !pixelConsentGranted) {
		console.debug('[MetaPixel] Cannot track custom event - pixel not ready or consent not granted');
		return;
	}

	window.fbq!('trackCustom', eventName, params);
	console.debug('[MetaPixel] Tracked custom event:', eventName, params);
}

/**
 * Track a page view event.
 */
export function trackPixelPageView(): void {
	trackPixelEvent('PageView');
}

/**
 * Enable Limited Data Use (LDU) for California CCPA compliance.
 *
 * @param enabled - Whether to enable LDU
 * @param state - State code (0 = not set, 1000 = California)
 *
 * @see https://developers.facebook.com/docs/marketing-apis/data-processing-options
 */
export function setLimitedDataUse(enabled: boolean, state: number = 0): void {
	if (!browser || !pixelInitialized) return;

	if (enabled) {
		// Enable LDU - limits data processing to anonymized data
		window.fbq!('dataProcessingOptions', ['LDU'], 1, state);
		console.debug('[MetaPixel] Limited Data Use enabled');
	} else {
		// Disable LDU - full data processing
		window.fbq!('dataProcessingOptions', []);
		console.debug('[MetaPixel] Limited Data Use disabled');
	}
}

/**
 * Meta Pixel Vendor Configuration
 */
export const metaPixelVendor: VendorConfig = {
	id: 'meta-pixel',
	name: 'Meta Pixel',
	description:
		'Marketing and advertising service that helps us measure the effectiveness of our ads and show relevant content.',
	requiredCategories: ['marketing'],
	privacyPolicyUrl: 'https://www.facebook.com/privacy/policy/',

	async load(): Promise<void> {
		// Validate environment variable
		if (!PUBLIC_META_PIXEL_ID) {
			console.debug(
				'[MetaPixel] PUBLIC_META_PIXEL_ID not set. Skipping Meta Pixel initialization. ' +
					'Set this environment variable to enable Meta Pixel tracking.'
			);
			return;
		}

		// Validate pixel ID format (should be numeric)
		if (!/^\d+$/.test(PUBLIC_META_PIXEL_ID)) {
			console.warn(
				'[MetaPixel] Invalid pixel ID format. Expected numeric value. Got:',
				PUBLIC_META_PIXEL_ID
			);
			return;
		}

		if (!browser) return;

		// Prevent double initialization
		if (pixelInitialized) {
			console.debug('[MetaPixel] Already initialized');
			return;
		}

		try {
			// Step 1: Initialize fbq stub
			initFbqStub();

			// Step 2: Grant consent before initializing
			// This tells Meta we have consent to track
			window.fbq!('consent', 'grant');
			pixelConsentGranted = true;

			// Step 3: Initialize the pixel with the ID
			window.fbq!('init', PUBLIC_META_PIXEL_ID);

			// Step 4: Load the fbevents.js script
			await injectPixelScript();

			pixelInitialized = true;

			console.debug('[MetaPixel] Initialized successfully with ID:', PUBLIC_META_PIXEL_ID);

			// Step 5: Track initial page view
			trackPixelPageView();
		} catch (error) {
			console.error('[MetaPixel] Failed to initialize:', error);
			throw error;
		}
	},

	onConsentRevoked(): void {
		// When marketing consent is revoked:
		// 1. Stop sending new events by setting consent flag
		// 2. Optionally revoke consent in the pixel

		pixelConsentGranted = false;

		if (browser && window.fbq) {
			try {
				// Revoke consent - tells Meta to stop collecting data
				window.fbq!('consent', 'revoke');
				console.debug('[MetaPixel] Consent revoked - stopped event tracking');
			} catch (error) {
				console.debug('[MetaPixel] Error revoking consent:', error);
			}
		}

		// Note: The pixel script itself cannot be unloaded without a page refresh.
		// However, with consent revoked, no new events will be sent.
		// For complete removal, the user would need to refresh the page.
	},
};

/**
 * Check if Meta Pixel is initialized and has consent.
 */
export function isMetaPixelReady(): boolean {
	return pixelInitialized && pixelConsentGranted;
}

/**
 * Reset Meta Pixel state (for testing).
 */
export function resetMetaPixel(): void {
	pixelInitialized = false;
	pixelConsentGranted = false;
}

export default metaPixelVendor;
