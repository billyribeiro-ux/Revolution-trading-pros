/**
 * SvelteKit 5 Consent Store (Enhanced)
 *
 * Reactive store for managing consent state with:
 * - Privacy signal integration (GPC, DNT)
 * - Audit logging
 * - Consent analytics
 * - Expiry management
 *
 * @module consent/store
 * @version 2.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';
import type { ConsentState, ConsentCategory, PrivacySignals } from './types';
import {
	DEFAULT_CONSENT_STATE,
	DEFAULT_CONSENT_EXPIRY_DAYS,
	generateConsentId,
	CONSENT_EVENTS
} from './types';
import { loadConsent, saveConsent } from './storage';
import { detectPrivacySignals, getSignalBasedDefaults } from './privacy-signals';
import { logConsentGiven, logConsentUpdated } from './audit-log';
import { trackConsentInteraction } from './analytics';

/**
 * The main consent store.
 */
function createConsentStore() {
	const { subscribe, set, update } = writable<ConsentState>({ ...DEFAULT_CONSENT_STATE });

	let initialized = false;
	let privacySignals: PrivacySignals | null = null;

	return {
		subscribe,

		/**
		 * Initialize the store from persistent storage.
		 */
		initialize(): ConsentState {
			if (!browser || initialized) {
				return get({ subscribe });
			}

			// Detect privacy signals
			privacySignals = detectPrivacySignals();

			// Load stored consent
			let stored = loadConsent();

			// Check if consent has expired
			if (stored.expiresAt) {
				const expiry = new Date(stored.expiresAt);
				if (expiry < new Date()) {
					logger.debug('[ConsentStore] Consent expired, resetting');
					stored = { ...DEFAULT_CONSENT_STATE };
				}
			}

			// Apply privacy signal defaults if no interaction
			if (!stored.hasInteracted && (privacySignals.gpc || privacySignals.dnt)) {
				const defaults = getSignalBasedDefaults(privacySignals);
				stored = {
					...stored,
					...defaults,
					privacySignals
				};
			}

			set(stored);
			initialized = true;

			console.debug('[ConsentStore] Initialized:', stored);

			return stored;
		},

		/**
		 * Get detected privacy signals.
		 */
		getPrivacySignals(): PrivacySignals | null {
			return privacySignals;
		},

		/**
		 * Update a specific consent category.
		 */
		setCategory(category: ConsentCategory, granted: boolean): void {
			if (category === 'necessary') {
				console.debug('[ConsentStore] Cannot change necessary consent');
				return;
			}

			update((state) => {
				const previousState = { ...state };
				const newState: ConsentState = {
					...state,
					[category]: granted,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
					consentId: state.consentId || generateConsentId(),
					consentMethod: 'modal'
				};

				// Add expiry
				const expiry = new Date();
				expiry.setDate(expiry.getDate() + DEFAULT_CONSENT_EXPIRY_DAYS);
				newState.expiresAt = expiry.toISOString();

				saveConsent(newState);

				// Log the change
				if (granted !== previousState[category]) {
					logConsentUpdated(newState, previousState, 'modal');
				}

				// Dispatch event
				dispatchConsentEvent(previousState, newState);

				return newState;
			});
		},

		/**
		 * Accept all consent categories.
		 */
		acceptAll(method: 'banner' | 'modal' = 'banner'): void {
			update((state) => {
				const previousState = { ...state };
				const newState: ConsentState = {
					...state,
					necessary: true,
					analytics: true,
					marketing: true,
					preferences: true,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
					consentId: generateConsentId(),
					consentMethod: method,
					strictMode: privacySignals?.requiresStrictConsent || false,
					...(privacySignals?.region && { countryCode: privacySignals.region }),
					...(privacySignals && { privacySignals })
				};

				// Add expiry
				const expiry = new Date();
				expiry.setDate(expiry.getDate() + DEFAULT_CONSENT_EXPIRY_DAYS);
				newState.expiresAt = expiry.toISOString();

				saveConsent(newState);

				// Log and track
				if (!previousState.hasInteracted) {
					logConsentGiven(newState, method);
				} else {
					logConsentUpdated(newState, previousState, method);
				}
				trackConsentInteraction('accept_all', {
					analytics: true,
					marketing: true,
					preferences: true
				});

				// Dispatch event
				dispatchConsentEvent(previousState, newState);

				console.debug('[ConsentStore] Accepted all categories');
				return newState;
			});
		},

		/**
		 * Reject all non-essential consent categories.
		 */
		rejectAll(method: 'banner' | 'modal' = 'banner'): void {
			update((state) => {
				const previousState = { ...state };
				const newState: ConsentState = {
					...state,
					necessary: true,
					analytics: false,
					marketing: false,
					preferences: false,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
					consentId: generateConsentId(),
					consentMethod: method,
					strictMode: privacySignals?.requiresStrictConsent || false,
					...(privacySignals?.region && { countryCode: privacySignals.region }),
					...(privacySignals && { privacySignals })
				};

				// Add expiry
				const expiry = new Date();
				expiry.setDate(expiry.getDate() + DEFAULT_CONSENT_EXPIRY_DAYS);
				newState.expiresAt = expiry.toISOString();

				saveConsent(newState);

				// Log and track
				if (!previousState.hasInteracted) {
					logConsentGiven(newState, method);
				} else {
					logConsentUpdated(newState, previousState, method);
				}
				trackConsentInteraction('reject_all', {
					analytics: false,
					marketing: false,
					preferences: false
				});

				// Dispatch event
				dispatchConsentEvent(previousState, newState);

				logger.debug('[ConsentStore] Rejected non-essential categories');
				return newState;
			});
		},

		/**
		 * Update multiple categories at once.
		 */
		updateCategories(
			categories: Partial<Record<ConsentCategory, boolean>>,
			method: 'banner' | 'modal' | 'api' = 'modal'
		): void {
			update((state) => {
				const previousState = { ...state };
				const newState: ConsentState = {
					...state,
					...categories,
					necessary: true,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
					consentId: state.consentId || generateConsentId(),
					consentMethod: method,
					strictMode: privacySignals?.requiresStrictConsent || false,
					...(privacySignals?.region && { countryCode: privacySignals.region }),
					...(privacySignals && { privacySignals })
				};

				// Add expiry
				const expiry = new Date();
				expiry.setDate(expiry.getDate() + DEFAULT_CONSENT_EXPIRY_DAYS);
				newState.expiresAt = expiry.toISOString();

				saveConsent(newState);

				// Log and track
				if (!previousState.hasInteracted) {
					logConsentGiven(newState, method);
				} else {
					logConsentUpdated(newState, previousState, method);
				}
				trackConsentInteraction('save_preferences', categories as any);

				// Dispatch event
				dispatchConsentEvent(previousState, newState);

				console.debug('[ConsentStore] Updated categories:', categories);
				return newState;
			});
		},

		/**
		 * Reset consent to defaults.
		 */
		reset(): void {
			const resetState: ConsentState = {
				...DEFAULT_CONSENT_STATE,
				updatedAt: new Date().toISOString(),
				...(privacySignals && { privacySignals })
			};

			set(resetState);
			saveConsent(resetState);

			logger.debug('[ConsentStore] Reset to defaults');
		},

		/**
		 * Check if a specific category is granted.
		 */
		hasConsent(category: ConsentCategory): boolean {
			const state = get({ subscribe });
			return state[category] === true;
		},

		/**
		 * Check if all specified categories are granted.
		 */
		hasAllConsent(categories: ConsentCategory[]): boolean {
			const state = get({ subscribe });
			return categories.every((cat) => state[cat] === true);
		},

		/**
		 * Get the current consent state (non-reactive).
		 */
		getState(): ConsentState {
			return get({ subscribe });
		},

		/**
		 * Check if consent is about to expire (within 30 days).
		 */
		isExpiringSoon(): boolean {
			const state = get({ subscribe });
			if (!state.expiresAt) return false;

			const expiry = new Date(state.expiresAt);
			const thirtyDaysFromNow = new Date();
			thirtyDaysFromNow.setDate(thirtyDaysFromNow.getDate() + 30);

			return expiry < thirtyDaysFromNow;
		},

		/**
		 * Get days until consent expires.
		 */
		getDaysUntilExpiry(): number | null {
			const state = get({ subscribe });
			if (!state.expiresAt) return null;

			const expiry = new Date(state.expiresAt);
			const now = new Date();
			const diff = expiry.getTime() - now.getTime();

			return Math.ceil(diff / (1000 * 60 * 60 * 24));
		}
	};
}

/**
 * Dispatch a custom consent change event.
 */
function dispatchConsentEvent(previous: ConsentState, current: ConsentState): void {
	if (!browser) return;

	const changed: ConsentCategory[] = [];
	if (previous.analytics !== current.analytics) changed.push('analytics');
	if (previous.marketing !== current.marketing) changed.push('marketing');
	if (previous.preferences !== current.preferences) changed.push('preferences');

	window.dispatchEvent(
		new CustomEvent(CONSENT_EVENTS.CONSENT_UPDATED, {
			detail: {
				previous,
				current,
				changed,
				source: 'user'
			}
		})
	);
}

/**
 * The singleton consent store instance.
 */
export const consentStore = createConsentStore();

/**
 * Derived store: whether the consent banner should be shown.
 */
export const showConsentBanner = derived(consentStore, ($consent) => !$consent.hasInteracted);

/**
 * Derived store: whether analytics consent is granted.
 */
export const hasAnalyticsConsent = derived(consentStore, ($consent) => $consent.analytics);

/**
 * Derived store: whether marketing consent is granted.
 */
export const hasMarketingConsent = derived(consentStore, ($consent) => $consent.marketing);

/**
 * Derived store: whether preferences consent is granted.
 */
export const hasPreferencesConsent = derived(consentStore, ($consent) => $consent.preferences);

/**
 * Store for controlling the preferences modal visibility.
 */
export const showPreferencesModal = writable(false);

/**
 * Open the consent preferences modal.
 */
export function openPreferencesModal(): void {
	showPreferencesModal.set(true);
	trackConsentInteraction('modal_opened');

	if (browser) {
		window.dispatchEvent(new CustomEvent(CONSENT_EVENTS.CONSENT_MODAL_OPENED));
	}
}

/**
 * Close the consent preferences modal.
 */
export function closePreferencesModal(): void {
	showPreferencesModal.set(false);
	trackConsentInteraction('modal_closed');

	if (browser) {
		window.dispatchEvent(new CustomEvent(CONSENT_EVENTS.CONSENT_MODAL_CLOSED));
	}
}

/**
 * Listen for consent changes.
 */
export function onConsentChange(callback: (event: CustomEvent) => void): () => void {
	if (!browser) return () => {};

	window.addEventListener(CONSENT_EVENTS.CONSENT_UPDATED, callback as EventListener);

	return () => {
		window.removeEventListener(CONSENT_EVENTS.CONSENT_UPDATED, callback as EventListener);
	};
}
