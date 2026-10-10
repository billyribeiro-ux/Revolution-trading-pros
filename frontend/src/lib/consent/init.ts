/**
 * Consent system bootstrap.
 *
 * Kept apart from the `#lib/consent/index.js` barrel so the root layout, which
 * only needs `initializeConsent`, does not pull the banner and template-editor
 * components (and everything they import) into every page's server and client
 * graph. The barrel re-exports these functions unchanged.
 */

import { browser } from '$app/env';
import { consentStore } from '#lib/consent/store.svelte.js';
import { applyConsentMode } from '#lib/consent/google-consent-mode.js';
import { applyBingConsentMode } from '#lib/consent/bing-consent-mode.js';
import { applyUrlPassthrough, captureUrlParameters } from '#lib/consent/url-passthrough.js';
import { loadVendorsForConsent } from '#lib/consent/vendor-loader.js';
import { vendors } from '#lib/consent/vendors/index.js';
import {
	initConsentAwareBehaviorTracking,
	cleanupBehaviorIntegration
} from '#lib/consent/behavior-integration.js';
import { exportAnalyticsData, trackConsentInteraction } from '#lib/consent/analytics.js';
import { exportAuditLog } from '#lib/consent/audit-log.js';
import { scanCookies } from '#lib/consent/cookie-scanner.js';
// The template store module directly — not the `templates` barrel, which also
// exports the banner/editor components.
import { initializeTemplateStore } from '#lib/consent/templates/store.js';
import type { ConsentState } from '#lib/consent/types.js';

/**
 * Initialize the complete consent system.
 * Call this once on client mount (in +layout.svelte onMount).
 *
 * Synced with consent-magic-pro v5.1.0 feature set.
 *
 * This will:
 * 1. Detect privacy signals (GPC, DNT, region)
 * 2. Load stored consent preferences
 * 3. Apply Google Consent Mode v2 defaults
 * 4. Apply Bing/Microsoft Consent Mode defaults
 * 5. Configure URL Passthrough for attribution
 * 6. Load vendors that have consent
 * 7. Initialize consent-aware behavior tracking
 * 8. Initialize banner template system
 * 9. Subscribe to consent changes
 */
export function initializeConsent(): () => void {
	if (!browser) {
		return () => {};
	}

	// Step 1: Initialize store (includes privacy signal detection)
	const initialConsent = consentStore.initialize();

	// Step 2: Capture URL parameters for attribution passthrough
	captureUrlParameters();

	// Step 3: Apply Google Consent Mode v2
	applyConsentMode(initialConsent);

	// Step 4: Apply Bing/Microsoft Consent Mode
	applyBingConsentMode(initialConsent);

	// Step 5: Apply URL Passthrough based on consent
	applyUrlPassthrough(initialConsent);

	// Step 6: Load vendors that have consent
	loadVendorsForConsent(initialConsent, vendors);

	// Step 7: Initialize consent-aware behavior tracking
	initConsentAwareBehaviorTracking();

	// Step 8: Initialize banner template system
	initializeTemplateStore();

	// Step 9: Track banner shown if needed
	if (!initialConsent.hasInteracted) {
		trackConsentInteraction('banner_shown');
	}

	// Step 10: Subscribe to future changes
	const unsubscribe = consentStore.subscribe((consent: ConsentState) => {
		applyConsentMode(consent);
		applyBingConsentMode(consent);
		applyUrlPassthrough(consent);
		loadVendorsForConsent(consent, vendors);
	});

	console.debug('[Consent] System initialized with consent-magic-pro v5.1.0 features');

	// Return cleanup function
	return () => {
		unsubscribe();
		cleanupBehaviorIntegration();
	};
}

/**
 * Re-initialize consent after a page navigation (for SPAs).
 */
export function onPageNavigation(): void {
	if (!browser) return;
	console.debug('[Consent] Page navigation detected');
}

/**
 * Export all consent data for GDPR data subject request.
 */
export async function exportConsentData(): Promise<string> {
	return JSON.stringify(
		{
			exportDate: new Date().toISOString(),
			currentConsent: consentStore.getState(),
			auditLog: JSON.parse(exportAuditLog()),
			analytics: JSON.parse(exportAnalyticsData()),
			cookies: scanCookies()
		},
		null,
		2
	);
}
