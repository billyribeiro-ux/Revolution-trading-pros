/**
 * SvelteKit 5 Consent & Tracking System - Enhanced Type Definitions
 *
 * Production-grade consent management with:
 * - IAB TCF v2.2 compliance
 * - GDPR/CCPA/ePrivacy support
 * - GPC (Global Privacy Control) integration
 * - Consent audit logging
 * - Geo-based defaults
 *
 * @module consent/types
 * @version 2.0.0
 */

// =============================================================================
// CONSENT CATEGORIES
// =============================================================================

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
 * All consent categories as an array for iteration.
 */
export const CONSENT_CATEGORIES: ConsentCategory[] = [
	'necessary',
	'analytics',
	'marketing',
	'preferences',
];

/**
 * Category metadata for UI display.
 */
export interface ConsentCategoryMeta {
	id: ConsentCategory;
	name: string;
	description: string;
	required: boolean;
	icon: string; // Lucide icon name
	cookieTypes: string[];
}

/**
 * Category metadata definitions.
 */
export const CATEGORY_METADATA: Record<ConsentCategory, ConsentCategoryMeta> = {
	necessary: {
		id: 'necessary',
		name: 'Strictly Necessary',
		description: 'Essential for the website to function. These cannot be disabled.',
		required: true,
		icon: 'shield-check',
		cookieTypes: ['Session', 'Authentication', 'Security', 'Load Balancing'],
	},
	analytics: {
		id: 'analytics',
		name: 'Analytics & Performance',
		description: 'Help us understand how visitors interact with our website.',
		required: false,
		icon: 'bar-chart-2',
		cookieTypes: ['Analytics', 'Performance', 'A/B Testing'],
	},
	marketing: {
		id: 'marketing',
		name: 'Marketing & Advertising',
		description: 'Used to track visitors and display relevant advertisements.',
		required: false,
		icon: 'megaphone',
		cookieTypes: ['Advertising', 'Retargeting', 'Social Media'],
	},
	preferences: {
		id: 'preferences',
		name: 'Preferences & Personalization',
		description: 'Remember your settings and personalize your experience.',
		required: false,
		icon: 'settings',
		cookieTypes: ['Language', 'Theme', 'Layout', 'Favorites'],
	},
};

// =============================================================================
// GOOGLE CONSENT MODE V2
// =============================================================================

/**
 * Google Consent Mode v2 consent parameters.
 * @see https://developers.google.com/tag-platform/gtagjs/reference#consent
 */
export interface GoogleConsentParams {
	ad_storage: 'granted' | 'denied';
	analytics_storage: 'granted' | 'denied';
	ad_user_data: 'granted' | 'denied';
	ad_personalization: 'granted' | 'denied';
	functionality_storage?: 'granted' | 'denied';
	personalization_storage?: 'granted' | 'denied';
	security_storage?: 'granted' | 'denied';
}

// =============================================================================
// CONSENT STATE
// =============================================================================

/**
 * Privacy signal detection results.
 */
export interface PrivacySignals {
	/** Do Not Track header is set */
	dnt: boolean;
	/** Global Privacy Control is enabled */
	gpc: boolean;
	/** Detected region code (e.g., 'EU', 'CA', 'US') */
	region?: string;
	/** Whether strict consent is required (EU, UK, etc.) */
	requiresStrictConsent: boolean;
}

/**
 * Extended consent state with enhanced features.
 */
export interface ConsentState {
	// Category consents
	necessary: boolean;
	analytics: boolean;
	marketing: boolean;
	preferences: boolean;

	// Metadata
	updatedAt: string;
	hasInteracted: boolean;
	version: number;

	// Enhanced fields
	/** Unique consent receipt ID for audit purposes */
	consentId?: string;
	/** IP country code at time of consent (for geo-compliance) */
	countryCode?: string;
	/** Whether consent was given under strict mode (GDPR) */
	strictMode?: boolean;
	/** Consent expiry timestamp (ISO 8601) */
	expiresAt?: string;
	/** Method of consent (banner, modal, api) */
	consentMethod?: 'banner' | 'modal' | 'api' | 'implicit';
	/** Privacy signals detected at consent time */
	privacySignals?: PrivacySignals;
	/** Policy version when consent was given */
	policyVersion?: string;
}

/**
 * Consent audit log entry.
 */
export interface ConsentAuditEntry {
	/** Unique entry ID */
	id: string;
	/** Timestamp of the action */
	timestamp: string;
	/** Type of action */
	action: 'consent_given' | 'consent_updated' | 'consent_revoked' | 'consent_expired';
	/** Categories that changed */
	categories: Partial<Record<ConsentCategory, boolean>>;
	/** Previous state (for updates) */
	previousState?: Partial<Record<ConsentCategory, boolean>>;
	/** Consent method used */
	method: 'banner' | 'modal' | 'api' | 'implicit' | 'expiry';
	/** User agent at time of consent */
	userAgent?: string;
	/** Page URL where consent was given */
	pageUrl?: string;
}

// =============================================================================
// VENDOR CONFIGURATION
// =============================================================================

/**
 * Cookie information for transparency.
 */
export interface CookieInfo {
	/** Cookie name */
	name: string;
	/** Cookie purpose */
	purpose: string;
	/** Cookie duration (e.g., '1 year', 'Session') */
	duration: string;
	/** Cookie type */
	type: 'first-party' | 'third-party';
	/** Whether this cookie is HTTP-only */
	httpOnly?: boolean;
	/** Whether this cookie is secure-only */
	secure?: boolean;
}

/**
 * Enhanced vendor configuration.
 */
export interface VendorConfig {
	/** Unique identifier */
	id: string;
	/** Human-readable name */
	name: string;
	/** Description */
	description: string;
	/** Required consent categories */
	requiredCategories: ConsentCategory[];
	/** Privacy policy URL */
	privacyPolicyUrl?: string;
	/** Cookies this vendor sets */
	cookies?: CookieInfo[];
	/** Data processing locations */
	dataLocations?: string[];
	/** Whether this vendor supports consent revocation */
	supportsRevocation?: boolean;
	/** Load function */
	load: () => void | Promise<void>;
	/** Revocation handler */
	onConsentRevoked?: () => void;
	/** Internal loading state */
	_loaded?: boolean;
}

// =============================================================================
// STORAGE OPTIONS
// =============================================================================

/**
 * Consent storage options.
 */
export interface ConsentStorageOptions {
	cookieName: string;
	cookieExpiry: number;
	localStorageKey: string;
	domain?: string;
	sameSite: 'strict' | 'lax' | 'none';
	secure: boolean;
	/** Key for audit log in localStorage */
	auditLogKey?: string;
	/** Maximum audit log entries to keep */
	maxAuditEntries?: number;
}

export const DEFAULT_STORAGE_OPTIONS: ConsentStorageOptions = {
	cookieName: 'rtp_consent',
	cookieExpiry: 365,
	localStorageKey: 'rtp_consent',
	sameSite: 'lax',
	secure: true,
	auditLogKey: 'rtp_consent_audit',
	maxAuditEntries: 100,
};

// =============================================================================
// DEFAULTS
// =============================================================================

/**
 * Generate a unique consent ID.
 */
export function generateConsentId(): string {
	return `cns_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * Default consent state (privacy-preserving defaults).
 */
export const DEFAULT_CONSENT_STATE: ConsentState = {
	necessary: true,
	analytics: false,
	marketing: false,
	preferences: false,
	updatedAt: new Date().toISOString(),
	hasInteracted: false,
	version: 2,
	strictMode: false,
};

/**
 * Schema version for consent state migrations.
 */
export const CONSENT_SCHEMA_VERSION = 2;

/**
 * Default consent expiry in days.
 */
export const DEFAULT_CONSENT_EXPIRY_DAYS = 365;

// =============================================================================
// TCF 2.2 SIGNALS
// =============================================================================

/**
 * IAB TCF v2.2 purpose IDs mapped to our categories.
 * @see https://iabeurope.eu/iab-europe-transparency-consent-framework-policies/
 */
export const TCF_PURPOSE_MAP: Record<ConsentCategory, number[]> = {
	necessary: [1], // Purpose 1: Store and/or access information on a device
	analytics: [7, 8, 9, 10], // Purposes 7-10: Measurement
	marketing: [2, 3, 4, 5, 6], // Purposes 2-6: Advertising
	preferences: [1], // Purpose 1: Storage
};

/**
 * TCF consent string components (simplified).
 */
export interface TCFConsentData {
	/** TCF version */
	version: number;
	/** Created timestamp */
	created: string;
	/** Last updated timestamp */
	lastUpdated: string;
	/** CMP ID (our ID) */
	cmpId: number;
	/** CMP version */
	cmpVersion: number;
	/** Consent screen */
	consentScreen: number;
	/** Consent language */
	consentLanguage: string;
	/** Vendor list version */
	vendorListVersion: number;
	/** Purpose consents (bit field as string) */
	purposeConsents: string;
	/** Vendor consents (bit field as string) */
	vendorConsents: string;
}

// =============================================================================
// CONSENT EVENTS
// =============================================================================

/**
 * Consent change event detail.
 */
export interface ConsentChangeEvent {
	/** Previous consent state */
	previous: ConsentState;
	/** New consent state */
	current: ConsentState;
	/** Categories that changed */
	changed: ConsentCategory[];
	/** Trigger source */
	source: 'user' | 'api' | 'expiry' | 'privacy-signal';
}

/**
 * Custom event names for consent system.
 */
export const CONSENT_EVENTS = {
	CONSENT_UPDATED: 'rtp:consent:updated',
	CONSENT_BANNER_SHOWN: 'rtp:consent:banner:shown',
	CONSENT_BANNER_HIDDEN: 'rtp:consent:banner:hidden',
	CONSENT_MODAL_OPENED: 'rtp:consent:modal:opened',
	CONSENT_MODAL_CLOSED: 'rtp:consent:modal:closed',
	VENDOR_LOADED: 'rtp:consent:vendor:loaded',
	VENDOR_BLOCKED: 'rtp:consent:vendor:blocked',
} as const;

// =============================================================================
// ANALYTICS
// =============================================================================

/**
 * Consent analytics data structure.
 */
export interface ConsentAnalytics {
	/** Total consent interactions */
	totalInteractions: number;
	/** Accept all rate (0-1) */
	acceptAllRate: number;
	/** Reject all rate (0-1) */
	rejectAllRate: number;
	/** Custom preferences rate (0-1) */
	customRate: number;
	/** Category acceptance rates */
	categoryRates: Record<ConsentCategory, number>;
	/** Average time to decision (ms) */
	avgTimeToDecision: number;
	/** Banner impressions */
	bannerImpressions: number;
	/** Modal opens */
	modalOpens: number;
}

/**
 * Consent interaction event for analytics.
 */
export interface ConsentInteractionEvent {
	type: 'banner_shown' | 'accept_all' | 'reject_all' | 'save_preferences' | 'modal_opened' | 'modal_closed';
	timestamp: number;
	categories?: Partial<Record<ConsentCategory, boolean>>;
	timeToDecision?: number;
	pageUrl: string;
}
