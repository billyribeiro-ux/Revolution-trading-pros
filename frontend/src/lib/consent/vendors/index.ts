/**
 * Central Vendor Registry
 *
 * This module exports all configured vendor integrations.
 * Add new vendors here to include them in the consent system.
 *
 * To add a new vendor:
 * 1. Create a new file in this directory (e.g., tiktok.ts)
 * 2. Implement the VendorConfig interface
 * 3. Import and add to the vendors array below
 *
 * @module consent/vendors
 * @version 1.0.0
 */

import type { VendorConfig } from '../types';
import { ga4Vendor } from './ga4';
import { metaPixelVendor } from './meta-pixel';

/**
 * All registered vendor configurations.
 *
 * Vendors are loaded in array order, but only when their
 * required consent categories are granted.
 *
 * To add a new vendor:
 * @example
 * ```typescript
 * import { tiktokVendor } from './tiktok';
 *
 * export const vendors: VendorConfig[] = [
 *   ga4Vendor,
 *   metaPixelVendor,
 *   tiktokVendor, // Add your new vendor here
 * ];
 * ```
 */
export const vendors: VendorConfig[] = [
	ga4Vendor,
	metaPixelVendor,
	// Add future vendors here:
	// tiktokVendor,
	// linkedinVendor,
	// pinterestVendor,
	// snapchatVendor,
	// redditVendor,
];

/**
 * Get a vendor by ID.
 */
export function getVendor(id: string): VendorConfig | undefined {
	return vendors.find((v) => v.id === id);
}

/**
 * Get all vendors that require a specific consent category.
 */
export function getVendorsByCategory(category: string): VendorConfig[] {
	return vendors.filter((v) => v.requiredCategories.includes(category as any));
}

/**
 * Get vendor metadata for display in the consent UI.
 */
export function getVendorInfo(): Array<{
	id: string;
	name: string;
	description: string;
	requiredCategories: string[];
	privacyPolicyUrl?: string;
}> {
	return vendors.map((v) => ({
		id: v.id,
		name: v.name,
		description: v.description,
		requiredCategories: [...v.requiredCategories],
		privacyPolicyUrl: v.privacyPolicyUrl,
	}));
}

// Re-export individual vendors for direct access
export { ga4Vendor } from './ga4';
export { metaPixelVendor } from './meta-pixel';

// Re-export tracking functions for convenience
export {
	trackPageView as trackGA4PageView,
	trackEvent as trackGA4Event,
	setUserProperties as setGA4UserProperties,
	setUserId as setGA4UserId,
	isGA4Ready,
} from './ga4';

export {
	trackPixelEvent,
	trackCustomPixelEvent,
	trackPixelPageView,
	setLimitedDataUse,
	isMetaPixelReady,
} from './meta-pixel';
