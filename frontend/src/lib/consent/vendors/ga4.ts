/**
 * Google Analytics 4 (GA4) Vendor Configuration
 *
 * Implements GA4 integration with full Google Consent Mode v2 support.
 * GA4 is loaded only when analytics consent is granted.
 *
 * Configuration:
 *   Set PUBLIC_GA4_MEASUREMENT_ID in your environment (.env or .env.local):
 *   PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX
 *
 * Verification:
 *   Use Google Tag Assistant (https://tagassistant.google.com/)
 *   to verify consent mode and tag firing.
 *
 * @module consent/vendors/ga4
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { env } from '$env/dynamic/public';
import type { VendorConfig } from '../types';

// Use dynamic environment variable (optional at build time)
const PUBLIC_GA4_MEASUREMENT_ID = env.PUBLIC_GA4_MEASUREMENT_ID || '';
import { injectScript } from '../vendor-loader';
import { applyConsentMode } from '../google-consent-mode';
import { consentStore } from '../store';

// Window interface extended in src/app.d.ts

/**
 * Track if GA4 has been initialized.
 */
let ga4Initialized = false;

/**
 * Initialize the gtag function and dataLayer.
 */
function initGtag(): void {
	if (!browser) return;

	window.dataLayer = window.dataLayer || [];

	if (typeof window.gtag !== 'function') {
		window.gtag = function (...args: unknown[]) {
			window.dataLayer.push(args);
		};
	}
}

/**
 * Send a page view event to GA4.
 */
export function trackPageView(url?: string): void {
	if (!browser || !ga4Initialized) return;

	const pageUrl = url || window.location.href;
	const pagePath = url ? new URL(url, window.location.origin).pathname : window.location.pathname;

	window.gtag('event', 'page_view', {
		page_location: pageUrl,
		page_path: pagePath,
		page_title: document.title,
	});

	console.debug('[GA4] Tracked page view:', pagePath);
}

/**
 * Send a custom event to GA4.
 *
 * @param eventName - The event name (e.g., 'purchase', 'sign_up')
 * @param params - Optional event parameters
 */
export function trackEvent(
	eventName: string,
	params?: Record<string, unknown>
): void {
	if (!browser || !ga4Initialized) return;

	window.gtag('event', eventName, params);
	console.debug('[GA4] Tracked event:', eventName, params);
}

/**
 * Set user properties in GA4.
 *
 * @param properties - User properties to set
 */
export function setUserProperties(properties: Record<string, unknown>): void {
	if (!browser || !ga4Initialized || !PUBLIC_GA4_MEASUREMENT_ID) return;

	window.gtag('config', PUBLIC_GA4_MEASUREMENT_ID, {
		user_properties: properties,
	});

	console.debug('[GA4] Set user properties:', properties);
}

/**
 * Set the user ID for cross-device tracking (requires user consent).
 *
 * @param userId - The user identifier
 */
export function setUserId(userId: string | null): void {
	if (!browser || !ga4Initialized || !PUBLIC_GA4_MEASUREMENT_ID) return;

	window.gtag('config', PUBLIC_GA4_MEASUREMENT_ID, {
		user_id: userId,
	});

	console.debug('[GA4] Set user ID:', userId ? '[set]' : '[cleared]');
}

/**
 * GA4 Vendor Configuration
 */
export const ga4Vendor: VendorConfig = {
	id: 'ga4',
	name: 'Google Analytics 4',
	description:
		'Analytics service that helps us understand how visitors interact with our website to improve user experience.',
	requiredCategories: ['analytics'],
	privacyPolicyUrl: 'https://policies.google.com/privacy',

	async load(): Promise<void> {
		// Validate environment variable
		if (!PUBLIC_GA4_MEASUREMENT_ID) {
			console.debug(
				'[GA4] PUBLIC_GA4_MEASUREMENT_ID not set. Skipping GA4 initialization. ' +
					'Set this environment variable to enable Google Analytics.'
			);
			return;
		}

		// Validate measurement ID format
		if (!PUBLIC_GA4_MEASUREMENT_ID.startsWith('G-')) {
			console.warn(
				'[GA4] Invalid measurement ID format. Expected "G-XXXXXXXXXX". Got:',
				PUBLIC_GA4_MEASUREMENT_ID
			);
			return;
		}

		if (!browser) return;

		// Prevent double initialization
		if (ga4Initialized) {
			console.debug('[GA4] Already initialized');
			return;
		}

		try {
			// Step 1: Initialize gtag function
			initGtag();

			// Step 2: Apply consent mode BEFORE loading gtag.js
			// This ensures consent defaults are set before any data collection
			const currentConsent = consentStore.getState();
			applyConsentMode(currentConsent);

			// Step 3: Load the gtag.js script
			await injectScript(
				`https://www.googletagmanager.com/gtag/js?id=${PUBLIC_GA4_MEASUREMENT_ID}`,
				{
					id: 'ga4-gtag',
					async: true,
				}
			);

			// Step 4: Initialize gtag with measurement ID
			window.gtag('js', new Date());

			// Step 5: Configure GA4 with privacy-preserving defaults
			// ICT8-11+ Fix: Disable browser history change detection to prevent
			// conflicts with SvelteKit's router (avoids history.pushState warnings)
			window.gtag('config', PUBLIC_GA4_MEASUREMENT_ID, {
				// Don't automatically send page views - we'll track them manually
				// This gives us more control and avoids duplicate events during SPA navigation
				send_page_view: false,

				// CRITICAL: Disable automatic page_view tracking on history changes
				// This prevents GA4 from calling history.pushState which conflicts with SvelteKit
				page_changes_enabled: false,

				// Anonymize IP addresses (now default in GA4, but explicit is better)
				anonymize_ip: true,

				// Don't allow Google signals (requires additional consent handling)
				allow_google_signals: currentConsent.marketing,

				// Don't allow ad personalization by default
				allow_ad_personalization_signals: currentConsent.marketing,
			});

			ga4Initialized = true;

			console.debug('[GA4] Initialized successfully with ID:', PUBLIC_GA4_MEASUREMENT_ID);

			// Send initial page view
			trackPageView();
		} catch (error) {
			console.error('[GA4] Failed to initialize:', error);
			throw error;
		}
	},

	onConsentRevoked(): void {
		// GA4 respects consent mode updates automatically.
		// When consent is revoked, we update consent mode and GA4
		// will stop collecting data that requires that consent.
		console.debug('[GA4] Consent revoked - GA4 will respect consent mode update');

		// Note: We don't need to do anything special here because:
		// 1. Google Consent Mode v2 handles this automatically
		// 2. The consent mode is updated via applyConsentMode() in the main flow
		// 3. GA4 checks consent status before each hit

		// If you want to completely stop GA4 from running, you would need to
		// reload the page (scripts can't be unloaded). This is generally not
		// recommended as consent mode handles data collection properly.
	},
};

/**
 * Check if GA4 is initialized and ready.
 */
export function isGA4Ready(): boolean {
	return ga4Initialized;
}

/**
 * Reset GA4 state (for testing).
 */
export function resetGA4(): void {
	ga4Initialized = false;
}

export default ga4Vendor;
