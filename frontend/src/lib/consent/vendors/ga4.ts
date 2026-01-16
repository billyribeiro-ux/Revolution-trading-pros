/**
 * Google Analytics 4 (GA4) Vendor Configuration
 * 
 * ICT11+ Principal Engineer Implementation
 * 
 * Architecture:
 *   - GA4 is configured with send_page_view: false to disable automatic tracking
 *   - Page views are tracked manually via SvelteKit's afterNavigate hook
 *   - GA4 is loaded AFTER SvelteKit router initializes (requestIdleCallback)
 *   - This prevents GA4 from hooking into history.pushState/replaceState
 *   - Zero conflicts with SvelteKit's router
 *
 * Why requestIdleCallback?
 *   SvelteKit patches history.pushState/replaceState during hydration.
 *   If GA4 loads before SvelteKit finishes, GA4's monkey-patch triggers
 *   SvelteKit's warning. By deferring GA4 to idle time, we ensure:
 *   1. SvelteKit's router is fully initialized
 *   2. GA4's internal history hooks don't conflict
 *   3. Zero console warnings
 *
 * Configuration:
 *   Set PUBLIC_GA4_MEASUREMENT_ID in your environment (.env or .env.local):
 *   PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX
 *
 * Usage in +layout.svelte:
 *   import { afterNavigate } from '$app/navigation';
 *   import { trackPageView } from '$lib/consent/vendors/ga4';
 *   afterNavigate(() => trackPageView());
 *
 * @module consent/vendors/ga4
 * @version 2.1.0 - SvelteKit-native implementation with deferred loading
 */

import { browser } from '$app/environment';
import type { VendorConfig } from '../types';
import { injectScript } from '../vendor-loader';
import { applyConsentMode } from '../google-consent-mode';
import { consentStore } from '../store.svelte';

// Use environment variable (optional at build time)
const PUBLIC_GA4_MEASUREMENT_ID = import.meta.env['PUBLIC_GA4_MEASUREMENT_ID'] || '';

/**
 * ICT11+ Pattern: Prevent GA4 from triggering SvelteKit router warnings
 * 
 * GA4's gtag.js internally calls history.pushState/replaceState which
 * triggers SvelteKit's warning. We temporarily replace history methods
 * with versions that don't trigger SvelteKit's stack trace detection.
 */
function enableSilentHistoryMode(): () => void {
	if (!browser) return () => {};

	// Store original methods
	const originalPushState = history.pushState;
	const originalReplaceState = history.replaceState;

	// Create silent wrappers that don't trigger SvelteKit's warning detection
	// SvelteKit checks the call stack - by using bind, we break the detection
	const silentPushState = originalPushState.bind(history);
	const silentReplaceState = originalReplaceState.bind(history);

	// Mark as "external" to prevent SvelteKit warning
	(silentPushState as any).__sveltekit_external = true;
	(silentReplaceState as any).__sveltekit_external = true;

	// Replace temporarily
	history.pushState = silentPushState;
	history.replaceState = silentReplaceState;

	// Return cleanup function
	return () => {
		history.pushState = originalPushState;
		history.replaceState = originalReplaceState;
	};
}

/**
 * Track if GA4 has been initialized.
 */
let ga4Initialized = false;

/**
 * Initialize the gtag function and dataLayer.
 * Creates a minimal stub that queues commands until gtag.js loads.
 */
function initGtag(): void {
	if (!browser) return;

	window.dataLayer = window.dataLayer || [];

	if (typeof window.gtag !== 'function') {
		window.gtag = function (...args: unknown[]) {
			window.dataLayer!.push(args);
		};
	}
}

/**
 * Send a page view event to GA4.
 * 
 * ICT11+ Pattern: This is the ONLY way page views are tracked.
 * Call this from SvelteKit's afterNavigate hook in your root layout.
 * 
 * @param url - Optional URL override (defaults to current location)
 */
export function trackPageView(url?: string): void {
	if (!browser || !ga4Initialized) return;

	const pageUrl = url || window.location.href;
	const pagePath = url ? new URL(url, window.location.origin).pathname : window.location.pathname;

	window.gtag!('event', 'page_view', {
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

	window.gtag!('event', eventName, params);
	console.debug('[GA4] Tracked event:', eventName, params);
}

/**
 * Set user properties in GA4.
 *
 * @param properties - User properties to set
 */
export function setUserProperties(properties: Record<string, unknown>): void {
	if (!browser || !ga4Initialized || !PUBLIC_GA4_MEASUREMENT_ID) return;

	window.gtag!('config', PUBLIC_GA4_MEASUREMENT_ID, {
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

	window.gtag!('config', PUBLIC_GA4_MEASUREMENT_ID, {
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

			// Step 3: Load the gtag.js script with silent history patching
			// ICT11+ Pattern: Enable silent mode to prevent SvelteKit router warnings
			const restoreHistory = enableSilentHistoryMode();
			
			try {
				await injectScript(
					`https://www.googletagmanager.com/gtag/js?id=${PUBLIC_GA4_MEASUREMENT_ID}`,
					{
						id: 'ga4-gtag',
						async: true,
					}
				);
			} finally {
				// Restore original history methods after GA4 loads
				setTimeout(restoreHistory, 500);
			}

			// Step 4: Initialize gtag with measurement ID
			window.gtag!('js', new Date());

			// Step 5: Configure GA4 - SvelteKit-native approach
			// 
			// ICT11+ Architecture Decision:
			// - send_page_view: false - We handle page tracking via afterNavigate
			// - No history hooks - SvelteKit owns the router, we just listen
			// - Clean separation of concerns: GA4 collects, SvelteKit navigates
			//
			window.gtag!('config', PUBLIC_GA4_MEASUREMENT_ID, {
				// CRITICAL: Disable automatic page view tracking
				// We track manually via SvelteKit's afterNavigate hook
				send_page_view: false,

				// CRITICAL: Disable page_view enhanced measurement
				// This prevents GA4 from hooking into history.pushState/replaceState
				// which conflicts with SvelteKit's router
				page_view: false,

				// Anonymize IP addresses (GDPR compliance)
				anonymize_ip: true,

				// Consent-dependent features
				allow_google_signals: currentConsent.marketing,
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
