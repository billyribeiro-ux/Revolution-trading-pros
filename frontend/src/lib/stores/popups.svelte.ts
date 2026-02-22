/**
 * Popup Store - Svelte 5 Runes
 * @version 2.0.0 - Migrated to Svelte 5 Runes (February 2026)
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

// Popup configuration types for a fully customizable popup system
export interface PopupButton {
	text: string;
	link?: string;
	action?: 'close' | 'submit' | 'custom' | 'link';
	style: 'primary' | 'secondary' | 'outline' | 'ghost';
	gradient?: string;
	customClass?: string;
	newTab?: boolean;
	customAction?: () => void;
}

export interface PopupContent {
	type: 'html' | 'markdown' | 'component';
	data: string;
	customCss?: string;
}

export interface PopupDesign {
	// Layout
	width: string; // e.g., '600px', '90%', 'auto'
	maxWidth: string;
	height: string; // e.g., 'auto', '400px', '80vh'
	padding: string;

	// Colors and styling
	backgroundColor: string;
	textColor: string;
	borderRadius: string;
	borderWidth: string;
	borderColor: string;
	borderStyle: string;

	// Background effects
	backgroundGradient?: string;
	backgroundImage?: string;
	backdropBlur: string;
	boxShadow: string;

	// Custom CSS
	customCss?: string;

	// Button styling (backend compatibility)
	titleColor?: string;
	buttonColor?: string;
	buttonTextColor?: string;
	buttonBorderRadius?: string;
	buttonPadding?: string;
	buttonShadow?: string;
}

export interface PopupAnimation {
	entrance:
		| 'fade'
		| 'slide-up'
		| 'slide-down'
		| 'slide-left'
		| 'slide-right'
		| 'zoom'
		| 'bounce'
		| 'flip';
	exit:
		| 'fade'
		| 'slide-up'
		| 'slide-down'
		| 'slide-left'
		| 'slide-right'
		| 'zoom'
		| 'bounce'
		| 'flip';
	duration: number; // in ms
	easing: 'linear' | 'ease' | 'ease-in' | 'ease-out' | 'ease-in-out' | 'custom';
	customEasing?: string;
}

export interface PopupDisplayRules {
	// Timing
	delaySeconds: number;
	showOnScroll: boolean;
	scrollPercentage: number;
	showOnExit: boolean; // Exit intent
	showOnIdle?: boolean;
	idleSeconds?: number; // Seconds of inactivity before showing
	timeOnPage?: number; // Seconds user must be on page before showing

	// Frequency
	frequency: 'always' | 'once-per-session' | 'once-per-day' | 'once-per-week' | 'once-ever';

	// Targeting
	pages: string[]; // URL patterns, e.g., ['/cart', '/checkout', '/products/*']
	excludePages: string[];
	deviceTargeting: 'all' | 'desktop' | 'mobile' | 'tablet';

	// Advanced targeting
	utmSource?: string[]; // Show only for specific UTM sources
	utmCampaign?: string[]; // Show only for specific campaigns
	referrer?: string[]; // Show based on referrer domain
	country?: string[]; // Geo-targeting (requires backend)

	// User behavior
	newVisitorsOnly: boolean;
	returningVisitorsOnly: boolean;
	minPageViews: number;
	maxPageViews?: number; // Hide after certain page views
}

export interface Popup {
	id: string;
	name: string; // Internal name for admin
	isActive: boolean;

	// Content
	title?: string;
	content: any; // PopupContent or string for backend compatibility
	successMessage?: string;
	variantTitle?: string;
	abTestId?: string;
	buttons: PopupButton[];
	closeButton: boolean;
	closeOnOverlayClick: boolean;
	closeOnEscape?: boolean;

	// Design
	design: any; // PopupDesign for backend compatibility (may be partial)
	animation: any; // PopupAnimation or string for backend compatibility
	attentionAnimation?: {
		enabled: boolean;
		type: 'shake' | 'pulse' | 'bounce';
		delay?: number;
		repeat?: number;
	};
	overlayColor: string;
	overlayOpacity: number;

	// Display rules
	displayRules: PopupDisplayRules;

	// Form integration (optional)
	formFields?: {
		name: string;
		type: 'text' | 'email' | 'tel' | 'textarea' | 'select' | 'checkbox' | 'url';
		placeholder: string;
		required: boolean;
		label?: string;
		options?: Array<string | { value?: string; label?: string }>; // For select fields
		validation?: (value: unknown) => true | string;
	}[];
	formAction?: string; // API endpoint or email

	// Advanced features
	countdownTimer?: {
		enabled: boolean;
		endDate: string; // ISO date string
		showDays: boolean;
		showHours: boolean;
		showMinutes: boolean;
		showSeconds: boolean;
		timerColor?: string;
		onExpire?: 'hide' | 'show-message' | 'redirect';
		expireMessage?: string;
		redirectUrl?: string;
	};

	videoEmbed?: {
		enabled: boolean;
		url: string; // YouTube, Vimeo, or direct video URL
		autoplay: boolean;
		muted: boolean;
		controls: boolean;
		aspectRatio: '16:9' | '4:3' | '1:1' | 'custom';
		customAspectRatio?: string;
	};

	multiStep?: {
		enabled: boolean;
		steps: {
			title?: string;
			content: PopupContent;
			buttons?: PopupButton[];
		}[];
		showProgress: boolean;
		progressColor?: string;
	};

	socialProof?: {
		enabled: boolean;
		type: 'recent-purchase' | 'visitor-count' | 'testimonial';
		message: string;
		updateInterval?: number; // Seconds
	};

	// Analytics
	impressions: number;
	conversions: number;

	// Meta
	createdAt: string;
	updatedAt: string;

	// Backend compatibility properties (snake_case from API)
	priority?: number;
	trigger_rules?: any;
	display_rules?: any;
	frequency_rules?: any;
	show_close_button?: boolean;
	close_on_overlay_click?: boolean;
	auto_close_after?: number;
	cta_text?: string;
	cta_url?: string;
	cta_new_tab?: boolean;
	has_form?: boolean;
	form_id?: string;
	type?: string;
	status?: string;
	position?: string;
	size?: string;
	performance_status?: string;
	total_views?: number;
	total_conversions?: number;
	conversion_rate?: number;
}

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
			logger.error('Failed to parse popup state:', e);
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
	recordImpression(popupId: string, _meta?: Record<string, any>) {
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
			import('$lib/api/popups').then(({ recordPopupImpression }) => {
				recordPopupImpression(popupId).catch((err) => {
					logger.error('Failed to record impression:', err);
				});
			});
		}
	},

	// Record a conversion (button click, form submit, etc.)
	recordConversion(popupId: string, data?: Record<string, any>) {
		// Send to backend API
		if (browser) {
			import('$lib/api/popups').then(({ recordPopupConversion }) => {
				recordPopupConversion(popupId, data).catch((err) => {
					logger.error('Failed to record conversion:', err);
				});
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
