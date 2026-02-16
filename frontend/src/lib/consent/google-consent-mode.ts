/**
 * Google Consent Mode v2 Integration
 *
 * Implements Google's Consent Mode v2 API for privacy-compliant
 * analytics and advertising tracking.
 *
 * @see https://developers.google.com/tag-platform/gtagjs/reference#consent
 * @see https://developers.google.com/tag-platform/devguides/consent
 *
 * @module consent/google-consent-mode
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, GoogleConsentParams } from './types';
import { logger } from '$lib/utils/logger';

declare global {
	interface Window {
		dataLayer?: unknown[];
		gtag?: (...args: unknown[]) => void;
	}
}

/**
 * Ensure the gtag function and dataLayer are available.
 * Creates stubs if they don't exist.
 */
function ensureGtag(): void {
	if (!browser) return;

	// Initialize dataLayer if not present
	window.dataLayer = window.dataLayer || [];

	// Create gtag function if not present
	if (typeof window.gtag !== 'function') {
		window.gtag = function (...args: unknown[]) {
			window.dataLayer!.push(args);
		};
	}
}

/**
 * Convert ConsentState to Google Consent Mode parameters.
 */
export function mapConsentToGoogle(consent: ConsentState): GoogleConsentParams {
	return {
		// ad_storage: Requires marketing consent
		ad_storage: consent.marketing ? 'granted' : 'denied',

		// analytics_storage: Requires analytics consent
		analytics_storage: consent.analytics ? 'granted' : 'denied',

		// ad_user_data: Requires marketing consent (new in Consent Mode v2)
		ad_user_data: consent.marketing ? 'granted' : 'denied',

		// ad_personalization: Requires marketing consent (new in Consent Mode v2)
		ad_personalization: consent.marketing ? 'granted' : 'denied',

		// functionality_storage: Requires preferences consent
		functionality_storage: consent.preferences ? 'granted' : 'denied',

		// personalization_storage: Requires preferences consent
		personalization_storage: consent.preferences ? 'granted' : 'denied',

		// security_storage: Always granted (necessary category)
		security_storage: 'granted'
	};
}

/**
 * Set the default consent state (before user interaction).
 * This should be called as early as possible, ideally before gtag.js loads.
 *
 * @param consent - The initial consent state
 * @param waitForUpdate - Optional delay (ms) before applying defaults
 */
export function setDefaultConsent(consent: ConsentState, waitForUpdate?: number): void {
	if (!browser) return;

	ensureGtag();

	const params = mapConsentToGoogle(consent);

	// Set default consent with optional wait_for_update
	const consentParams: GoogleConsentParams & { wait_for_update?: number } = {
		...params
	};

	if (waitForUpdate && waitForUpdate > 0) {
		consentParams.wait_for_update = waitForUpdate;
	}

	window.gtag!('consent', 'default', consentParams);

	logger.debug('[GoogleConsentMode] Set default consent:', consentParams);
}

/**
 * Update the consent state after user interaction.
 * Call this whenever the user changes their preferences.
 *
 * @param consent - The updated consent state
 */
export function updateConsent(consent: ConsentState): void {
	if (!browser) return;

	ensureGtag();

	const params = mapConsentToGoogle(consent);

	window.gtag!('consent', 'update', params);

	logger.debug('[GoogleConsentMode] Updated consent:', params);
}

/**
 * Apply consent mode settings.
 * Automatically uses 'default' for first call and 'update' for subsequent calls.
 *
 * @param consent - The consent state to apply
 */
let consentInitialized = false;

export function applyConsentMode(consent: ConsentState): void {
	if (!browser) return;

	if (!consentInitialized) {
		// First call: set defaults
		setDefaultConsent(consent);
		consentInitialized = true;
	} else {
		// Subsequent calls: update
		updateConsent(consent);
	}
}

/**
 * Check if Google Consent Mode has been initialized.
 */
export function isConsentModeInitialized(): boolean {
	return consentInitialized;
}

/**
 * Reset consent mode initialization state (for testing).
 */
export function resetConsentMode(): void {
	consentInitialized = false;
}

/**
 * Grant all consent categories (utility for "Accept All").
 */
export function grantAllConsent(): void {
	if (!browser) return;

	ensureGtag();

	const params: GoogleConsentParams = {
		ad_storage: 'granted',
		analytics_storage: 'granted',
		ad_user_data: 'granted',
		ad_personalization: 'granted',
		functionality_storage: 'granted',
		personalization_storage: 'granted',
		security_storage: 'granted'
	};

	window.gtag!('consent', 'update', params);
	consentInitialized = true;

	logger.debug('[GoogleConsentMode] Granted all consent');
}

/**
 * Deny all non-essential consent (utility for "Reject All").
 */
export function denyNonEssentialConsent(): void {
	if (!browser) return;

	ensureGtag();

	const params: GoogleConsentParams = {
		ad_storage: 'denied',
		analytics_storage: 'denied',
		ad_user_data: 'denied',
		ad_personalization: 'denied',
		functionality_storage: 'denied',
		personalization_storage: 'denied',
		security_storage: 'granted' // Always granted
	};

	window.gtag!('consent', 'update', params);
	consentInitialized = true;

	logger.debug('[GoogleConsentMode] Denied non-essential consent');
}
