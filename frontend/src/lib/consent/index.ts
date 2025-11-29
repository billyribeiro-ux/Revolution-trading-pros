/**
 * SvelteKit 5 Consent & Tracking System (Enhanced)
 *
 * Production-grade, enterprise-level consent management with:
 * - Cookie + localStorage persistence
 * - Google Consent Mode v2 integration
 * - GPC (Global Privacy Control) support
 * - DNT (Do Not Track) respect
 * - GDPR/CCPA compliance
 * - Consent audit logging
 * - Consent analytics
 * - Cookie scanning
 * - Behavior tracker integration
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
 *      import {
 *        ConsentBanner,
 *        ConsentPreferencesModal,
 *        ConsentSettingsButton,
 *        initializeConsent
 *      } from '$lib/consent';
 *      import { onMount } from 'svelte';
 *
 *      onMount(() => {
 *        initializeConsent();
 *      });
 *    </script>
 *
 *    <ConsentBanner />
 *    <ConsentPreferencesModal />
 *    <ConsentSettingsButton />
 *    ```
 *
 * @module consent
 * @version 2.0.0
 */

// =============================================================================
// TYPES
// =============================================================================

export type {
	ConsentCategory,
	ConsentState,
	VendorConfig,
	GoogleConsentParams,
	ConsentStorageOptions,
	PrivacySignals,
	ConsentAuditEntry,
	CookieInfo,
	ConsentCategoryMeta,
	ConsentChangeEvent,
	ConsentAnalytics,
	ConsentInteractionEvent,
	TCFConsentData,
} from './types';

export {
	DEFAULT_CONSENT_STATE,
	DEFAULT_STORAGE_OPTIONS,
	CONSENT_SCHEMA_VERSION,
	DEFAULT_CONSENT_EXPIRY_DAYS,
	CONSENT_CATEGORIES,
	CATEGORY_METADATA,
	TCF_PURPOSE_MAP,
	CONSENT_EVENTS,
	generateConsentId,
} from './types';

// =============================================================================
// STORE
// =============================================================================

export {
	consentStore,
	showConsentBanner,
	hasAnalyticsConsent,
	hasMarketingConsent,
	hasPreferencesConsent,
	showPreferencesModal,
	openPreferencesModal,
	closePreferencesModal,
	onConsentChange,
} from './store';

// =============================================================================
// STORAGE
// =============================================================================

export {
	loadConsent,
	saveConsent,
	clearConsent,
	isStorageAvailable,
} from './storage';

// =============================================================================
// GOOGLE CONSENT MODE
// =============================================================================

export {
	applyConsentMode,
	updateConsent as updateGoogleConsent,
	setDefaultConsent,
	mapConsentToGoogle,
	grantAllConsent,
	denyNonEssentialConsent,
	isConsentModeInitialized,
} from './google-consent-mode';

// =============================================================================
// VENDOR LOADER
// =============================================================================

export {
	loadVendorsForConsent,
	loadVendor,
	isVendorLoaded,
	getLoadedVendors,
	hasRequiredConsent,
	injectScript,
	injectInlineScript,
} from './vendor-loader';

// =============================================================================
// VENDORS
// =============================================================================

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

// =============================================================================
// PRIVACY SIGNALS
// =============================================================================

export {
	detectPrivacySignals,
	isDNTEnabled,
	isGPCEnabled,
	detectRegion,
	requiresStrictConsent,
	hasPrivacyLaw,
	getSignalBasedDefaults,
	getConsentUIMode,
	onPrivacySignalChange,
} from './privacy-signals';

// =============================================================================
// AUDIT LOG
// =============================================================================

export {
	getAuditLog,
	addAuditEntry,
	logConsentGiven,
	logConsentUpdated,
	logConsentRevoked,
	logConsentExpired,
	clearAuditLog,
	exportAuditLog,
	getAuditStats,
	verifyAuditLogIntegrity,
	syncAuditLogToServer,
} from './audit-log';

// =============================================================================
// COOKIE SCANNER
// =============================================================================

export {
	scanCookies,
	getUnconsentedCookies,
	watchCookies,
	deleteCookie,
	deleteCookiesByCategory,
	getCookieSummary,
} from './cookie-scanner';

export type { ScannedCookie, CookieScanResult } from './cookie-scanner';

// =============================================================================
// CONSENT ANALYTICS
// =============================================================================

export {
	getConsentAnalytics,
	getInteractionEvents,
	trackConsentInteraction,
	getAnalyticsSummary,
	exportAnalyticsData,
	clearAnalytics,
	syncAnalyticsToServer,
} from './analytics';

// =============================================================================
// BEHAVIOR INTEGRATION
// =============================================================================

export {
	initConsentAwareBehaviorTracking,
	setBehaviorUserId,
	trackBehaviorEvent,
	isBehaviorTrackingEnabled,
	getBehaviorTracker,
	cleanupBehaviorIntegration,
	flushBehaviorEvents,
} from './behavior-integration';

// =============================================================================
// COMPONENTS
// =============================================================================

export {
	ConsentBanner,
	ConsentPreferencesModal,
	ConsentSettingsButton,
	ContentPlaceholder,
} from './components';

// =============================================================================
// I18N (INTERNATIONALIZATION)
// =============================================================================

export {
	t,
	currentLanguage,
	initializeI18n,
	setLanguage,
	getLanguage,
	getSupportedLanguages,
	getTranslation,
	addTranslations,
	detectBrowserLanguage,
} from './i18n';

export type { SupportedLanguage, ConsentTranslations } from './i18n';

// =============================================================================
// CONSENT RECEIPT
// =============================================================================

export {
	generateConsentReceipt,
	exportReceiptAsJSON,
	downloadReceiptAsJSON,
	generateReceiptHTML,
	downloadReceiptAsHTML,
	printReceipt,
	verifyReceiptChecksum,
} from './consent-receipt';

export type { ConsentReceipt } from './consent-receipt';

// =============================================================================
// SCRIPT BLOCKER
// =============================================================================

export {
	getScriptCategory,
	shouldBlockScript,
	startScriptBlocking,
	releaseBlockedScripts,
	getBlockedScripts,
	isScriptBlockingActive,
	generateBlockingScript,
	generateNoscriptMessage,
} from './script-blocker';

// =============================================================================
// VERSIONING
// =============================================================================

export {
	CURRENT_POLICY_VERSION,
	POLICY_VERSIONS,
	checkVersionCompatibility,
	policyUpdateAvailable,
	checkForPolicyUpdates,
	acknowledgePolicyUpdate,
	getCurrentPolicyVersion,
	getChangelog,
	formatVersionDate,
	getVersionInfo,
} from './versioning';

export type { PolicyVersion, VersionCompareResult, VersionChangeType } from './versioning';

// =============================================================================
// BACKEND SYNC
// =============================================================================

export {
	configureBackendSync,
	getSyncConfig,
	buildSyncPayload,
	syncConsentToBackend,
	debouncedSync,
	fetchConsentFromBackend,
	deleteConsentFromBackend,
	exportConsentFromBackend,
} from './backend-sync';

export type { BackendSyncConfig, ConsentSyncPayload, ConsentSyncResponse } from './backend-sync';

// =============================================================================
// A/B TESTING
// =============================================================================

export {
	DEFAULT_VARIANTS,
	currentVariant,
	initializeABTest,
	recordImpression,
	recordDecision,
	getABTestAnalytics,
	getBestVariant,
	clearABTestData,
	exportABTestData,
	getVariantStyles,
} from './ab-testing';

export type {
	BannerLayout,
	BannerTone,
	ButtonStyle,
	ColorScheme,
	BannerVariant,
	ABTestResult,
	ABTestAnalytics,
} from './ab-testing';

// =============================================================================
// CROSS-DOMAIN
// =============================================================================

export {
	configureCrossDomain,
	getCrossDomainConfig,
	initializeCrossDomain,
	requestConsentFromParent,
	encodeConsentParam,
	generateConsentUrl,
	syncFromDomainCookie,
} from './cross-domain';

export type { CrossDomainConfig } from './cross-domain';

// =============================================================================
// TEMPLATES
// =============================================================================

export {
	// Types
	type TemplatePosition,
	type TemplateStyle,
	type ButtonVariant,
	type AnimationType,
	type ColorScheme as TemplateColorScheme,
	type Typography,
	type Spacing,
	type MobileConfig,
	type TabletConfig,
	type TemplateCopy,
	type BannerTemplate,
	type TemplateCustomization,
	type ActiveTemplateConfig,
	type TemplatePreview,
	// Registry
	BANNER_TEMPLATES,
	getTemplate,
	getTemplatesByCategory,
	getTemplateCategories,
	getTemplatePreviews,
	DEFAULT_TEMPLATE_ID,
	// Store
	activeTemplate,
	allTemplates,
	previewTemplate,
	isPreviewMode,
	initializeTemplateStore,
	setActiveTemplate,
	updateCustomization,
	clearCustomization,
	saveAsCustomTemplate,
	updateCustomTemplate,
	deleteCustomTemplate,
	enterPreviewMode,
	exitPreviewMode,
	applyPreview,
	getActiveTemplateConfig,
	getCurrentTemplate,
	exportTemplateConfig,
	importTemplateConfig,
	// Components
	BannerRenderer,
	TemplatePreviewCard,
	TemplateEditor,
} from './templates';

// =============================================================================
// INITIALIZATION
// =============================================================================

import { browser } from '$app/environment';
import { consentStore } from './store';
import { applyConsentMode } from './google-consent-mode';
import { loadVendorsForConsent } from './vendor-loader';
import { vendors } from './vendors';
import { initConsentAwareBehaviorTracking, cleanupBehaviorIntegration } from './behavior-integration';
import { trackConsentInteraction } from './analytics';
import { initializeTemplateStore } from './templates';
import type { ConsentState } from './types';

/**
 * Initialize the complete consent system.
 * Call this once on client mount (in +layout.svelte onMount).
 *
 * This will:
 * 1. Detect privacy signals (GPC, DNT, region)
 * 2. Load stored consent preferences
 * 3. Apply Google Consent Mode defaults
 * 4. Load vendors that have consent
 * 5. Initialize consent-aware behavior tracking
 * 6. Initialize banner template system
 * 7. Subscribe to consent changes
 */
export function initializeConsent(): () => void {
	if (!browser) {
		return () => {};
	}

	// Step 1: Initialize store (includes privacy signal detection)
	const initialConsent = consentStore.initialize();

	// Step 2: Apply initial consent mode
	applyConsentMode(initialConsent);

	// Step 3: Load vendors that have consent
	loadVendorsForConsent(initialConsent, vendors);

	// Step 4: Initialize consent-aware behavior tracking
	initConsentAwareBehaviorTracking();

	// Step 5: Initialize banner template system
	initializeTemplateStore();

	// Step 6: Track banner shown if needed
	if (!initialConsent.hasInteracted) {
		trackConsentInteraction('banner_shown');
	}

	// Step 7: Subscribe to future changes
	const unsubscribe = consentStore.subscribe((consent: ConsentState) => {
		applyConsentMode(consent);
		loadVendorsForConsent(consent, vendors);
	});

	console.debug('[Consent] System initialized with enhanced features and templates');

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
export function exportConsentData(): string {
	const { exportAuditLog } = require('./audit-log');
	const { exportAnalyticsData } = require('./analytics');
	const { scanCookies } = require('./cookie-scanner');

	return JSON.stringify(
		{
			exportDate: new Date().toISOString(),
			currentConsent: consentStore.getState(),
			auditLog: JSON.parse(exportAuditLog()),
			analytics: JSON.parse(exportAnalyticsData()),
			cookies: scanCookies(),
		},
		null,
		2
	);
}
