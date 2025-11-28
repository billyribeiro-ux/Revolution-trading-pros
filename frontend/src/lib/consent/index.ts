/**
 * SvelteKit 5 Consent & Tracking System
 *
 * A comprehensive, production-grade consent management system with:
 * - Cookie + localStorage persistence
 * - Google Consent Mode v2 integration
 * - Vendor loading based on consent categories
 * - GA4 and Meta Pixel pre-configured
 *
 * Quick Start:
 * 1. Set environment variables:
 *    PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX
 *    PUBLIC_META_PIXEL_ID=1234567890
 *
 * 2. Import and use in your +layout.svelte:
 *    ```svelte
 *    <script>
 *      import { ConsentBanner, ConsentPreferencesModal, initializeConsent } from '$lib/consent';
 *      import { onMount } from 'svelte';
 *
 *      onMount(() => {
 *        initializeConsent();
 *      });
 *    </script>
 *
 *    <ConsentBanner />
 *    <ConsentPreferencesModal />
 *    ```
 *
 * @module consent
 * @version 1.0.0
 */

// Types
export type {
	ConsentCategory,
	ConsentState,
	VendorConfig,
	GoogleConsentParams,
	ConsentStorageOptions,
} from './types';

export {
	DEFAULT_CONSENT_STATE,
	DEFAULT_STORAGE_OPTIONS,
	CONSENT_SCHEMA_VERSION,
} from './types';

// Store
export {
	consentStore,
	showConsentBanner,
	hasAnalyticsConsent,
	hasMarketingConsent,
	hasPreferencesConsent,
	showPreferencesModal,
	openPreferencesModal,
	closePreferencesModal,
} from './store';

// Storage
export {
	loadConsent,
	saveConsent,
	clearConsent,
	isStorageAvailable,
} from './storage';

// Google Consent Mode
export {
	applyConsentMode,
	updateConsent as updateGoogleConsent,
	setDefaultConsent,
	mapConsentToGoogle,
	grantAllConsent,
	denyNonEssentialConsent,
	isConsentModeInitialized,
} from './google-consent-mode';

// Vendor Loader
export {
	loadVendorsForConsent,
	loadVendor,
	isVendorLoaded,
	getLoadedVendors,
	hasRequiredConsent,
	injectScript,
	injectInlineScript,
} from './vendor-loader';

// Vendors
export {
	vendors,
	getVendor,
	getVendorsByCategory,
	getVendorInfo,
	ga4Vendor,
	metaPixelVendor,
	// GA4 tracking functions
	trackGA4PageView,
	trackGA4Event,
	setGA4UserProperties,
	setGA4UserId,
	isGA4Ready,
	// Meta Pixel tracking functions
	trackPixelEvent,
	trackCustomPixelEvent,
	trackPixelPageView,
	setLimitedDataUse,
	isMetaPixelReady,
} from './vendors';

// Components
export { ConsentBanner, ConsentPreferencesModal } from './components';

import { browser } from '$app/environment';
import { consentStore } from './store';
import { applyConsentMode } from './google-consent-mode';
import { loadVendorsForConsent } from './vendor-loader';
import { vendors } from './vendors';
import type { ConsentState } from './types';

/**
 * Initialize the consent system.
 * Call this once on client mount (in +layout.svelte onMount).
 *
 * This will:
 * 1. Load stored consent preferences
 * 2. Apply Google Consent Mode defaults
 * 3. Load vendors that have consent
 * 4. Subscribe to consent changes
 */
export function initializeConsent(): () => void {
	if (!browser) {
		return () => {};
	}

	// Step 1: Initialize store from storage
	const initialConsent = consentStore.initialize();

	// Step 2: Apply initial consent mode
	applyConsentMode(initialConsent);

	// Step 3: Load vendors that have consent
	loadVendorsForConsent(initialConsent, vendors);

	// Step 4: Subscribe to future changes
	const unsubscribe = consentStore.subscribe((consent: ConsentState) => {
		// This will be called on every update (including initialization)
		// The vendor loader and consent mode functions are idempotent
		applyConsentMode(consent);
		loadVendorsForConsent(consent, vendors);
	});

	console.debug('[Consent] System initialized');

	return unsubscribe;
}

/**
 * Re-initialize consent after a page navigation (for SPAs).
 * Call this in your router's navigation handler if needed.
 */
export function onPageNavigation(): void {
	if (!browser) return;

	// Vendors should track their own page views
	// This is a hook for any additional navigation handling
	console.debug('[Consent] Page navigation detected');
}
