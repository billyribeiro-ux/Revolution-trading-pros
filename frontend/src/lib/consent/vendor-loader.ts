/**
 * Vendor Loader - Consent-Aware Script Loading
 *
 * Manages loading of third-party vendor scripts based on consent state.
 * Ensures scripts are only loaded when appropriate consent is granted.
 *
 * @module consent/vendor-loader
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, ConsentCategory, VendorConfig } from './types';

/**
 * Track which vendors have been loaded to prevent double-loading.
 */
const loadedVendors = new Set<string>();

/**
 * Track which vendors are currently loading.
 */
const loadingVendors = new Set<string>();

/**
 * Check if all required categories for a vendor are granted.
 */
export function hasRequiredConsent(
	vendor: VendorConfig,
	consent: ConsentState
): boolean {
	return vendor.requiredCategories.every((category: ConsentCategory) => {
		return consent[category] === true;
	});
}

/**
 * Load a single vendor if consent is granted.
 */
export async function loadVendor(
	vendor: VendorConfig,
	consent: ConsentState
): Promise<boolean> {
	if (!browser) return false;

	// Skip if already loaded or loading
	if (loadedVendors.has(vendor.id)) {
		console.debug(`[VendorLoader] ${vendor.id} already loaded, skipping`);
		return true;
	}

	if (loadingVendors.has(vendor.id)) {
		console.debug(`[VendorLoader] ${vendor.id} is currently loading, skipping`);
		return false;
	}

	// Check consent
	if (!hasRequiredConsent(vendor, consent)) {
		console.debug(
			`[VendorLoader] ${vendor.id} requires consent for: ${vendor.requiredCategories.join(', ')}`
		);
		return false;
	}

	try {
		loadingVendors.add(vendor.id);
		console.debug(`[VendorLoader] Loading ${vendor.id}...`);

		await vendor.load();

		loadedVendors.add(vendor.id);
		vendor._loaded = true;
		loadingVendors.delete(vendor.id);

		console.debug(`[VendorLoader] Successfully loaded ${vendor.id}`);
		return true;
	} catch (error) {
		loadingVendors.delete(vendor.id);
		console.error(`[VendorLoader] Failed to load ${vendor.id}:`, error);
		return false;
	}
}

/**
 * Handle consent being revoked for a vendor.
 */
export function handleConsentRevoked(
	vendor: VendorConfig,
	consent: ConsentState
): void {
	if (!browser) return;

	// Only call revoke handler if vendor was loaded and consent is now missing
	if (vendor._loaded && !hasRequiredConsent(vendor, consent)) {
		if (vendor.onConsentRevoked) {
			try {
				vendor.onConsentRevoked();
				console.debug(`[VendorLoader] Called onConsentRevoked for ${vendor.id}`);
			} catch (error) {
				console.error(`[VendorLoader] Error in onConsentRevoked for ${vendor.id}:`, error);
			}
		}
	}
}

/**
 * Load all vendors that have appropriate consent.
 *
 * @param consent - The current consent state
 * @param vendors - Array of vendor configurations
 */
export async function loadVendorsForConsent(
	consent: ConsentState,
	vendors: VendorConfig[]
): Promise<void> {
	if (!browser) return;

	console.debug('[VendorLoader] Loading vendors for consent state:', consent);

	// Process vendors in parallel
	const results = await Promise.allSettled(
		vendors.map(async (vendor) => {
			if (hasRequiredConsent(vendor, consent)) {
				return loadVendor(vendor, consent);
			} else {
				// Handle revoked consent
				handleConsentRevoked(vendor, consent);
				return false;
			}
		})
	);

	const loaded = results.filter(
		(r) => r.status === 'fulfilled' && r.value === true
	).length;

	console.debug(`[VendorLoader] Loaded ${loaded}/${vendors.length} vendors`);
}

/**
 * Check if a specific vendor has been loaded.
 */
export function isVendorLoaded(vendorId: string): boolean {
	return loadedVendors.has(vendorId);
}

/**
 * Get list of all loaded vendor IDs.
 */
export function getLoadedVendors(): string[] {
	return Array.from(loadedVendors);
}

/**
 * Reset vendor loading state (for testing).
 */
export function resetVendorLoader(): void {
	loadedVendors.clear();
	loadingVendors.clear();
}

/**
 * Utility: Inject a script tag into the document head.
 * Returns a promise that resolves when the script loads.
 */
export function injectScript(
	src: string,
	options: {
		async?: boolean;
		defer?: boolean;
		id?: string;
		onLoad?: () => void;
		onError?: (error: Error) => void;
	} = {}
): Promise<void> {
	return new Promise((resolve, reject) => {
		if (!browser) {
			resolve();
			return;
		}

		// Check if script already exists
		if (options.id && document.getElementById(options.id)) {
			console.debug(`[VendorLoader] Script ${options.id} already exists`);
			resolve();
			return;
		}

		const script = document.createElement('script');
		script.src = src;
		script.async = options.async ?? true;
		script.defer = options.defer ?? false;

		if (options.id) {
			script.id = options.id;
		}

		script.onload = () => {
			options.onLoad?.();
			resolve();
		};

		script.onerror = () => {
			const error = new Error(`Failed to load script: ${src}`);
			options.onError?.(error);
			reject(error);
		};

		document.head.appendChild(script);
	});
}

/**
 * Utility: Inject an inline script into the document head.
 */
export function injectInlineScript(
	code: string,
	options: { id?: string } = {}
): void {
	if (!browser) return;

	// Check if script already exists
	if (options.id && document.getElementById(options.id)) {
		console.debug(`[VendorLoader] Inline script ${options.id} already exists`);
		return;
	}

	const script = document.createElement('script');
	script.type = 'text/javascript';
	script.textContent = code;

	if (options.id) {
		script.id = options.id;
	}

	document.head.appendChild(script);
}
