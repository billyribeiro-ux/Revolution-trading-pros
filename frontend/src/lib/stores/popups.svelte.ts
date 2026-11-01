/**
 * Popup Store - Svelte 5 Runes
 * @version 2.0.0 - Migrated to Svelte 5 Runes (February 2026)
 */

import { browser } from '$app/env';
import type { JsonValue } from '#lib/api/_types.js';
import type {
	Popup,
	PopupAnimation,
	PopupButton,
	PopupContent,
	PopupDesign,
	PopupDisplayRules
} from './popups.types.js';

export type {
	Popup,
	PopupAnimation,
	PopupButton,
	PopupContent,
	PopupDesign,
	PopupDisplayRules
};

// Default popup configuration for easy creation
export const defaultPopupConfig: Partial<Popup> = {
	isActive: true,
	closeButton: true,
	closeOnOverlayClick: true,

	design: {
		width: '600px',
		maxWidth: '90vw',
		height: 'auto',
		padding: '2rem',
		backgroundColor: '#1e293b',
		textColor: '#f1f5f9',
		borderRadius: '24px',
		borderWidth: '1px',
		borderColor: 'rgba(99, 102, 241, 0.3)',
		borderStyle: 'solid',
		backdropBlur: '16px',
		boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)'
	},

	animation: {
		entrance: 'zoom',
		exit: 'fade',
		duration: 300,
		easing: 'ease-out'
	},

	overlayColor: '#000000',
	overlayOpacity: 0.7,

	displayRules: {
		delaySeconds: 2,
		showOnScroll: false,
		scrollPercentage: 50,
		showOnExit: false,
		frequency: 'once-per-session',
		pages: ['/*'],
		excludePages: [],
		deviceTargeting: 'all',
		newVisitorsOnly: false,
		returningVisitorsOnly: false,
		minPageViews: 0
	},

	buttons: [],
	impressions: 0,
	conversions: 0
};

// Popup state management
interface PopupState {
	activePopup: Popup | null;
	popupHistory: {
		popupId: string;
		lastShown: number;
		shownCount: number;
	}[];
	pageViews: number;
	sessionStart: number;
}

const STORAGE_KEY = 'popup_state';

function loadPopupState(): PopupState {
	if (!browser) {
		return {
			activePopup: null,
			popupHistory: [],
			pageViews: 0,
			sessionStart: Date.now()
		};
	}

	const stored = localStorage.getItem(STORAGE_KEY);
	if (stored) {
		try {
			return JSON.parse(stored);
		} catch (e) {
			console.error('Failed to parse popup state:', e);
		}
	}

	return {
		activePopup: null,
		popupHistory: [],
		pageViews: 0,
		sessionStart: Date.now()
	};
}

function savePopupState(state: PopupState) {
	if (browser) {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
	}
}

// =============================================================================
// Svelte 5 Reactive State
// =============================================================================

let storeState = $state<PopupState>(loadPopupState());

// Derived value for active popup
const activePopupValue = $derived(storeState.activePopup);

// =============================================================================
// Exported Store (Svelte 5 Pattern)
// =============================================================================

export const popupStore = {
	// Getters
	get state() {
		return storeState;
	},
	get activePopup() {
		return activePopupValue;
	},
	get popupHistory() {
		return storeState.popupHistory;
	},
	get pageViews() {
		return storeState.pageViews;
	},

	// Show a popup
	show(popup: Popup) {
		storeState = { ...storeState, activePopup: popup };
		savePopupState(storeState);
	},

	// Hide the current popup
	hide() {
		storeState = { ...storeState, activePopup: null };
		savePopupState(storeState);
	},

	// Record that a popup was shown
	recordImpression(popupId: string, _meta?: Record<string, JsonValue>) {
		const now = Date.now();
		const history = [...storeState.popupHistory];
		const existing = history.find((h) => h.popupId === popupId);

		if (existing) {
			existing.lastShown = now;
			existing.shownCount++;
		} else {
			history.push({
				popupId,
				lastShown: now,
				shownCount: 1
			});
		}

		storeState = { ...storeState, popupHistory: history };
		savePopupState(storeState);

		// Send to backend API
		if (browser) {
			import('#lib/api/popups.js').then(({ recordPopupImpression }) => {
				recordPopupImpression(popupId).catch((err) => {
					console.error('Failed to record impression:', err);
				});
			});
		}
	},

	// Record a conversion (button click, form submit, etc.). Historically a
	// `Record<string, any>` because callers (`PopupModal.svelte:620,692`,
	// `PopupRenderer.svelte:229,250`) pass freeform analytic payloads
	// (`{ formData, variant, submissionTime }`, `{ action, buttonText, variant }`)
	// that the backend logs verbatim. The API's `ConversionData` envelope
	// (`action?, value?, revenue?, metadata?, timestamp?`) is narrower — and
	// LB-1 (below) documents the latent drift between the two shapes. Until
	// callers are migrated to the envelope, accept JSON-shaped payloads here.
	recordConversion(popupId: string, data?: Record<string, JsonValue>) {
		// Send to backend API
		if (browser) {
			import('#lib/api/popups.js').then(({ recordPopupConversion }) => {
				// `data` is a freeform JSON payload (callers pass `{ formData,
				// variant, submissionTime, ... }`); pass through unchanged so
				// the wire shape stays flat — see LB-1 in R23-A report for the
				// `ConversionData`-envelope drift.
				recordPopupConversion(popupId, data as Parameters<typeof recordPopupConversion>[1]).catch(
					(err) => {
						console.error('Failed to record conversion:', err);
					}
				);
			});
		}
	},

	// Check if popup should be shown based on frequency rules
	canShow(popup: Popup): boolean {
		if (!browser) return false;

		const history = storeState.popupHistory.find((h) => h.popupId === popup.id);
		const now = Date.now();

		if (!history) return true;

		switch (popup.displayRules.frequency) {
			case 'always':
				return true;

			case 'once-per-session':
				// Check if shown in current session
				return history.lastShown < storeState.sessionStart;

			case 'once-per-day':
				const dayInMs = 24 * 60 * 60 * 1000;
				return now - history.lastShown > dayInMs;

			case 'once-per-week':
				const weekInMs = 7 * 24 * 60 * 60 * 1000;
				return now - history.lastShown > weekInMs;

			case 'once-ever':
				return history.shownCount === 0;

			default:
				return true;
		}
	},

	// Increment page view count
	incrementPageView() {
		storeState = { ...storeState, pageViews: storeState.pageViews + 1 };
		savePopupState(storeState);
	},

	// Clear all popup history (for testing/debugging)
	clearHistory() {
		storeState = {
			...storeState,
			popupHistory: [],
			pageViews: 0,
			sessionStart: Date.now()
		};
		savePopupState(storeState);
	}
};

// Legacy export for backward compatibility
export const activePopup = {
	get value() {
		return activePopupValue;
	}
};
