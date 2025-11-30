/**
 * Bing/Microsoft Consent Mode Integration
 *
 * Implements Microsoft's UET (Universal Event Tracking) consent signals
 * for privacy-compliant advertising tracking.
 *
 * @see https://help.ads.microsoft.com/apex/index/3/en/60119
 * @module consent/bing-consent-mode
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState } from './types';

/**
 * Bing/Microsoft consent parameters
 */
export interface BingConsentParams {
	/** Ad storage consent for UET */
	ad_storage: 'granted' | 'denied';
}

declare global {
	interface Window {
		uetq?: unknown[];
	}
}

/**
 * Ensure the UET queue is available.
 */
function ensureUetq(): void {
	if (!browser) return;
	window.uetq = window.uetq || [];
}

/**
 * Convert ConsentState to Bing Consent Mode parameters.
 */
export function mapConsentToBing(consent: ConsentState): BingConsentParams {
	return {
		// ad_storage: Requires marketing consent
		ad_storage: consent.marketing ? 'granted' : 'denied',
	};
}

/**
 * Set the default Bing consent state (before user interaction).
 * This should be called as early as possible, ideally before UET loads.
 *
 * @param consent - The initial consent state
 */
export function setBingDefaultConsent(consent: ConsentState): void {
	if (!browser) return;

	ensureUetq();

	const params = mapConsentToBing(consent);

	// Push consent default to UET queue
	window.uetq!.push('consent', 'default', {
		ad_storage: params.ad_storage,
	});

	console.debug('[BingConsentMode] Set default consent:', params);
}

/**
 * Update Bing consent state after user interaction.
 * Call this whenever the user changes their preferences.
 *
 * @param consent - The updated consent state
 */
export function updateBingConsent(consent: ConsentState): void {
	if (!browser) return;

	ensureUetq();

	const params = mapConsentToBing(consent);

	// Push consent update to UET queue
	window.uetq!.push('consent', 'update', {
		ad_storage: params.ad_storage,
	});

	console.debug('[BingConsentMode] Updated consent:', params);
}

/**
 * Apply Bing consent mode settings.
 * Automatically uses 'default' for first call and 'update' for subsequent calls.
 *
 * @param consent - The consent state to apply
 */
let bingConsentInitialized = false;

export function applyBingConsentMode(consent: ConsentState): void {
	if (!browser) return;

	if (!bingConsentInitialized) {
		setBingDefaultConsent(consent);
		bingConsentInitialized = true;
	} else {
		updateBingConsent(consent);
	}
}

/**
 * Check if Bing Consent Mode has been initialized.
 */
export function isBingConsentModeInitialized(): boolean {
	return bingConsentInitialized;
}

/**
 * Reset Bing consent mode initialization state (for testing).
 */
export function resetBingConsentMode(): void {
	bingConsentInitialized = false;
}

/**
 * Grant all Bing consent (utility for "Accept All").
 */
export function grantAllBingConsent(): void {
	if (!browser) return;

	ensureUetq();

	window.uetq!.push('consent', 'update', {
		ad_storage: 'granted',
	});

	bingConsentInitialized = true;
	console.debug('[BingConsentMode] Granted all consent');
}

/**
 * Deny all Bing consent (utility for "Reject All").
 */
export function denyAllBingConsent(): void {
	if (!browser) return;

	ensureUetq();

	window.uetq!.push('consent', 'update', {
		ad_storage: 'denied',
	});

	bingConsentInitialized = true;
	console.debug('[BingConsentMode] Denied all consent');
}

/**
 * Get the current Bing consent mode configuration for rendering.
 */
export function getBingConsentModeConfig(consent: ConsentState): {
	enabled: boolean;
	params: BingConsentParams;
} {
	return {
		enabled: true,
		params: mapConsentToBing(consent),
	};
}
