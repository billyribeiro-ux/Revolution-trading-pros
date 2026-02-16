/**
 * URL Passthrough Mode for Google Ads
 *
 * When users haven't consented to ad cookies, URL passthrough
 * allows passing ad click information (gclid, dclid, etc.) through
 * URL parameters for conversion tracking without cookies.
 *
 * @see https://support.google.com/google-ads/answer/11956797
 * @module consent/url-passthrough
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState } from './types';
import { logger } from '$lib/utils/logger';

/**
 * URL passthrough configuration
 */
export interface UrlPassthroughConfig {
	/** Whether URL passthrough is enabled */
	enabled: boolean;
	/** Whether to automatically add parameters to links */
	autoDecorate: boolean;
	/** URL parameters to pass through */
	parameters: string[];
}

/**
 * Default URL passthrough parameters
 */
export const DEFAULT_PASSTHROUGH_PARAMS = [
	'gclid', // Google Click ID
	'dclid', // Display Click ID
	'gclsrc', // Google Click Source
	'wbraid', // Web-to-App Attribution
	'gbraid', // App-to-Web Attribution
	'utm_source',
	'utm_medium',
	'utm_campaign',
	'utm_term',
	'utm_content',
	'fbclid', // Facebook Click ID
	'msclkid', // Microsoft Click ID
	'ttclid', // TikTok Click ID
	'twclid', // Twitter Click ID
	'li_fat_id', // LinkedIn First-Party ID
	'rdt_cid' // Reddit Click ID
];

declare global {
	interface Window {
		gtag?: (...args: unknown[]) => void;
		dataLayer?: unknown[];
	}
}

let passthroughEnabled = false;
let storedParams: Record<string, string> = {};

/**
 * Check if URL passthrough should be enabled based on consent state.
 * Passthrough is enabled when marketing consent is denied but we want
 * to preserve attribution information.
 */
export function shouldEnablePassthrough(consent: ConsentState): boolean {
	// Enable passthrough when marketing is denied (can't use cookies)
	// but we still want to track conversions via URL parameters
	return !consent.marketing;
}

/**
 * Extract and store passthrough parameters from current URL.
 * Call this on page load to preserve attribution data.
 */
export function captureUrlParameters(): Record<string, string> {
	if (!browser) return {};

	const params: Record<string, string> = {};
	const searchParams = new URLSearchParams(window.location.search);

	DEFAULT_PASSTHROUGH_PARAMS.forEach((param) => {
		const value = searchParams.get(param);
		if (value) {
			params[param] = value;
		}
	});

	storedParams = { ...storedParams, ...params };

	if (Object.keys(params).length > 0) {
		logger.debug('[UrlPassthrough] Captured parameters:', params);
	}

	return params;
}

/**
 * Get stored passthrough parameters.
 */
export function getStoredParameters(): Record<string, string> {
	return { ...storedParams };
}

/**
 * Clear stored passthrough parameters.
 */
export function clearStoredParameters(): void {
	storedParams = {};
}

/**
 * Add passthrough parameters to a URL.
 */
export function addPassthroughParams(url: string): string {
	if (!browser || Object.keys(storedParams).length === 0) return url;

	try {
		const urlObj = new URL(url, window.location.origin);

		Object.entries(storedParams).forEach(([key, value]) => {
			if (!urlObj.searchParams.has(key)) {
				urlObj.searchParams.set(key, value);
			}
		});

		return urlObj.toString();
	} catch {
		return url;
	}
}

/**
 * Configure Google's URL passthrough mode via gtag.
 * This tells Google to pass click IDs through URLs.
 */
export function configureGoogleUrlPassthrough(enabled: boolean): void {
	if (!browser) return;

	// Ensure gtag is available
	window.dataLayer = window.dataLayer || [];
	if (typeof window.gtag !== 'function') {
		window.gtag = function (...args: unknown[]) {
			window.dataLayer!.push(args);
		};
	}

	// Set url_passthrough parameter
	window.gtag('set', 'url_passthrough', enabled);

	logger.debug('[UrlPassthrough] Google URL passthrough:', enabled ? 'enabled' : 'disabled');
}

/**
 * Enable URL passthrough mode.
 */
export function enableUrlPassthrough(): void {
	if (!browser) return;

	passthroughEnabled = true;
	captureUrlParameters();
	configureGoogleUrlPassthrough(true);

	logger.debug('[UrlPassthrough] URL passthrough enabled');
}

/**
 * Disable URL passthrough mode.
 */
export function disableUrlPassthrough(): void {
	if (!browser) return;

	passthroughEnabled = false;
	configureGoogleUrlPassthrough(false);

	logger.debug('[UrlPassthrough] URL passthrough disabled');
}

/**
 * Apply URL passthrough based on consent state.
 */
export function applyUrlPassthrough(consent: ConsentState): void {
	if (!browser) return;

	// Always capture parameters for potential use
	captureUrlParameters();

	if (shouldEnablePassthrough(consent)) {
		enableUrlPassthrough();
	} else {
		disableUrlPassthrough();
	}
}

/**
 * Check if URL passthrough is currently enabled.
 */
export function isUrlPassthroughEnabled(): boolean {
	return passthroughEnabled;
}

/**
 * Decorate all links on the page with passthrough parameters.
 * Call this after enabling passthrough to ensure all outbound links
 * preserve attribution data.
 */
export function decorateLinks(): void {
	if (!browser || !passthroughEnabled || Object.keys(storedParams).length === 0) return;

	const links = document.querySelectorAll('a[href]');
	let decoratedCount = 0;

	links.forEach((link) => {
		const href = link.getAttribute('href');
		if (!href) return;

		// Only decorate internal links or specific tracking domains
		try {
			const url = new URL(href, window.location.origin);

			// Skip external links unless they're known conversion domains
			if (url.hostname !== window.location.hostname) {
				return;
			}

			const newHref = addPassthroughParams(href);
			if (newHref !== href) {
				link.setAttribute('href', newHref);
				decoratedCount++;
			}
		} catch {
			// Invalid URL, skip
		}
	});

	if (decoratedCount > 0) {
		logger.debug(`[UrlPassthrough] Decorated ${decoratedCount} links`);
	}
}

/**
 * Generate a URL with passthrough parameters for server-side use.
 */
export function generatePassthroughUrl(
	baseUrl: string,
	additionalParams?: Record<string, string>
): string {
	if (!browser) return baseUrl;

	try {
		const urlObj = new URL(baseUrl, window.location.origin);

		// Add stored passthrough params
		Object.entries(storedParams).forEach(([key, value]) => {
			if (!urlObj.searchParams.has(key)) {
				urlObj.searchParams.set(key, value);
			}
		});

		// Add any additional params
		if (additionalParams) {
			Object.entries(additionalParams).forEach(([key, value]) => {
				urlObj.searchParams.set(key, value);
			});
		}

		return urlObj.toString();
	} catch {
		return baseUrl;
	}
}

/**
 * Get passthrough configuration object.
 */
export function getPassthroughConfig(consent: ConsentState): UrlPassthroughConfig {
	return {
		enabled: shouldEnablePassthrough(consent),
		autoDecorate: true,
		parameters: DEFAULT_PASSTHROUGH_PARAMS
	};
}
