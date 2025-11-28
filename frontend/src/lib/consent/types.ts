/**
 * SvelteKit 5 Consent & Tracking System - Type Definitions
 *
 * This module defines the core types for the consent management system,
 * including consent categories, state management, and vendor configuration.
 *
 * @module consent/types
 * @version 1.0.0
 */

/**
 * Consent categories following the IAB TCF v2.2 model.
 * Each category represents a distinct purpose for data processing.
 */
export type ConsentCategory =
	| 'necessary'     // Essential cookies/functionality (always enabled)
	| 'analytics'     // Performance and analytics tracking (GA4, etc.)
	| 'marketing'     // Advertising and marketing (Meta Pixel, etc.)
	| 'preferences';  // User preference storage (personalization)

/**
 * Google Consent Mode v2 consent parameters.
 * These map to Google's official consent mode API.
 * @see https://developers.google.com/tag-platform/gtagjs/reference#consent
 */
export interface GoogleConsentParams {
	/** Controls storage for advertising purposes (cookies, etc.) */
	ad_storage: 'granted' | 'denied';
	/** Controls storage for analytics purposes */
	analytics_storage: 'granted' | 'denied';
	/** Controls whether user data can be sent to Google for advertising */
	ad_user_data: 'granted' | 'denied';
	/** Controls whether ad personalization is enabled */
	ad_personalization: 'granted' | 'denied';
	/** Controls storage for functionality purposes */
	functionality_storage?: 'granted' | 'denied';
	/** Controls storage for personalization purposes */
	personalization_storage?: 'granted' | 'denied';
	/** Controls storage for security purposes */
	security_storage?: 'granted' | 'denied';
}

/**
 * The current state of user consent preferences.
 * Tracks which categories have been granted consent.
 */
export interface ConsentState {
	/** User has given consent for this category */
	necessary: boolean;     // Always true (required for site to function)
	analytics: boolean;     // Performance and measurement
	marketing: boolean;     // Advertising and retargeting
	preferences: boolean;   // Personalization features

	/** Timestamp when consent was last updated (ISO 8601) */
	updatedAt: string;

	/** Whether the user has made an active choice (vs. using defaults) */
	hasInteracted: boolean;

	/** Version of the consent schema (for future migrations) */
	version: number;
}

/**
 * Configuration for a third-party vendor/tracker.
 * Each vendor specifies which consent categories it requires.
 */
export interface VendorConfig {
	/** Unique identifier for the vendor (e.g., 'ga4', 'meta-pixel') */
	id: string;

	/** Human-readable name for display in consent UI */
	name: string;

	/** Description of what this vendor does with data */
	description: string;

	/** Consent categories required for this vendor to load */
	requiredCategories: ConsentCategory[];

	/**
	 * Function to load/initialize the vendor's scripts.
	 * Called only when all required consent categories are granted.
	 * Should be idempotent (safe to call multiple times).
	 */
	load: () => void | Promise<void>;

	/**
	 * Optional function to handle consent being revoked.
	 * Called when a required category is denied after previously being granted.
	 */
	onConsentRevoked?: () => void;

	/** Privacy policy URL for this vendor */
	privacyPolicyUrl?: string;

	/** Whether this vendor is loaded (internal state, managed by loader) */
	_loaded?: boolean;
}

/**
 * Options for storing consent preferences.
 */
export interface ConsentStorageOptions {
	/** Cookie name for storing consent */
	cookieName: string;
	/** Cookie expiry in days */
	cookieExpiry: number;
	/** localStorage key for backup storage */
	localStorageKey: string;
	/** Cookie domain (for cross-subdomain consent) */
	domain?: string;
	/** SameSite attribute for the cookie */
	sameSite: 'strict' | 'lax' | 'none';
	/** Whether to use Secure flag (HTTPS only) */
	secure: boolean;
}

/**
 * Default storage options for consent preferences.
 */
export const DEFAULT_STORAGE_OPTIONS: ConsentStorageOptions = {
	cookieName: 'rtp_consent',
	cookieExpiry: 365, // 1 year
	localStorageKey: 'rtp_consent',
	sameSite: 'lax',
	secure: true,
};

/**
 * Default consent state (privacy-preserving defaults).
 * Only necessary cookies are enabled by default.
 */
export const DEFAULT_CONSENT_STATE: ConsentState = {
	necessary: true,     // Always required
	analytics: false,    // Opt-in required
	marketing: false,    // Opt-in required
	preferences: false,  // Opt-in required
	updatedAt: new Date().toISOString(),
	hasInteracted: false,
	version: 1,
};

/**
 * Schema version for consent state migrations.
 */
export const CONSENT_SCHEMA_VERSION = 1;
