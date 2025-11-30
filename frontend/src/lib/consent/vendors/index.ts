/**
 * Central Vendor Registry
 *
 * This module exports all configured vendor integrations.
 * Add new vendors here to include them in the consent system.
 *
 * Synced with consent-magic-pro v5.1.0 feature set.
 *
 * @module consent/vendors
 * @version 2.0.0
 */

import type { VendorConfig } from '../types';
import { ga4Vendor } from './ga4';
import { metaPixelVendor } from './meta-pixel';
import { tiktokVendor } from './tiktok';
import { twitterVendor } from './twitter';
import { linkedinVendor } from './linkedin';
import { pinterestVendor } from './pinterest';
import { redditVendor } from './reddit';

/**
 * All registered vendor configurations.
 *
 * Vendors are loaded in array order, but only when their
 * required consent categories are granted.
 *
 * Environment variables required for each vendor:
 * - GA4: PUBLIC_GA4_MEASUREMENT_ID
 * - Meta Pixel: PUBLIC_META_PIXEL_ID
 * - TikTok: PUBLIC_TIKTOK_PIXEL_ID
 * - Twitter: PUBLIC_TWITTER_PIXEL_ID
 * - LinkedIn: PUBLIC_LINKEDIN_PARTNER_ID
 * - Pinterest: PUBLIC_PINTEREST_TAG_ID
 * - Reddit: PUBLIC_REDDIT_PIXEL_ID
 */
export const vendors: VendorConfig[] = [
	ga4Vendor,
	metaPixelVendor,
	tiktokVendor,
	twitterVendor,
	linkedinVendor,
	pinterestVendor,
	redditVendor,
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
export { tiktokVendor } from './tiktok';
export { twitterVendor } from './twitter';
export { linkedinVendor } from './linkedin';
export { pinterestVendor } from './pinterest';
export { redditVendor } from './reddit';

// Re-export tracking functions for convenience

// GA4
export {
	trackPageView as trackGA4PageView,
	trackEvent as trackGA4Event,
	setUserProperties as setGA4UserProperties,
	setUserId as setGA4UserId,
	isGA4Ready,
} from './ga4';

// Meta Pixel
export {
	trackPixelEvent,
	trackCustomPixelEvent,
	trackPixelPageView,
	setLimitedDataUse,
	isMetaPixelReady,
} from './meta-pixel';

// TikTok
export {
	trackTikTokEvent,
	trackTikTokPageView,
	identifyTikTokUser,
	isTikTokReady,
	TIKTOK_EVENTS,
} from './tiktok';

// Twitter/X
export {
	trackTwitterEvent,
	trackTwitterPageView,
	isTwitterReady,
	TWITTER_EVENTS,
} from './twitter';

// LinkedIn
export {
	trackLinkedInConversion,
	trackLinkedInPageView,
	isLinkedInReady,
} from './linkedin';

// Pinterest
export {
	trackPinterestEvent,
	trackPinterestPageView,
	isPinterestReady,
	PINTEREST_EVENTS,
} from './pinterest';

// Reddit
export {
	trackRedditEvent,
	trackRedditPageView,
	isRedditReady,
	setRedditLimitedDataUse,
	isRedditLDUEnabled,
	REDDIT_EVENTS,
} from './reddit';
