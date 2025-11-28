/**
 * SvelteKit 5 Consent Store (Svelte 5 Runes)
 *
 * Reactive store for managing consent state using Svelte 5's rune system.
 * Provides a clean API for reading and updating consent preferences.
 *
 * @module consent/store
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import type { ConsentState, ConsentCategory } from './types';
import { DEFAULT_CONSENT_STATE } from './types';
import { loadConsent, saveConsent } from './storage';

/**
 * The main consent store - writable store with the current consent state.
 */
function createConsentStore() {
	// Initialize with defaults, will be hydrated from storage on mount
	const { subscribe, set, update } = writable<ConsentState>({ ...DEFAULT_CONSENT_STATE });

	let initialized = false;

	return {
		subscribe,

		/**
		 * Initialize the store from persistent storage.
		 * Should be called once on client-side mount.
		 */
		initialize(): ConsentState {
			if (!browser || initialized) {
				return get({ subscribe });
			}

			const stored = loadConsent();
			set(stored);
			initialized = true;
			console.debug('[ConsentStore] Initialized with state:', stored);
			return stored;
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
				const newState: ConsentState = {
					...state,
					[category]: granted,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
				};
				saveConsent(newState);
				return newState;
			});
		},

		/**
		 * Accept all consent categories.
		 */
		acceptAll(): void {
			update((state) => {
				const newState: ConsentState = {
					...state,
					necessary: true,
					analytics: true,
					marketing: true,
					preferences: true,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
				};
				saveConsent(newState);
				console.debug('[ConsentStore] Accepted all categories');
				return newState;
			});
		},

		/**
		 * Reject all non-essential consent categories.
		 */
		rejectAll(): void {
			update((state) => {
				const newState: ConsentState = {
					...state,
					necessary: true,
					analytics: false,
					marketing: false,
					preferences: false,
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
				};
				saveConsent(newState);
				console.debug('[ConsentStore] Rejected non-essential categories');
				return newState;
			});
		},

		/**
		 * Update multiple categories at once.
		 */
		updateCategories(categories: Partial<Record<ConsentCategory, boolean>>): void {
			update((state) => {
				const newState: ConsentState = {
					...state,
					...categories,
					necessary: true, // Always true
					hasInteracted: true,
					updatedAt: new Date().toISOString(),
				};
				saveConsent(newState);
				console.debug('[ConsentStore] Updated categories:', categories);
				return newState;
			});
		},

		/**
		 * Reset consent to defaults (for testing or GDPR "right to be forgotten").
		 */
		reset(): void {
			const resetState: ConsentState = {
				...DEFAULT_CONSENT_STATE,
				updatedAt: new Date().toISOString(),
			};
			set(resetState);
			saveConsent(resetState);
			console.debug('[ConsentStore] Reset to defaults');
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
	};
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
}

/**
 * Close the consent preferences modal.
 */
export function closePreferencesModal(): void {
	showPreferencesModal.set(false);
}
