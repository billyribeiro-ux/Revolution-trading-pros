/**
 * Popup configuration types for a fully customizable popup system.
 * Extracted into a pure TypeScript module so `popups.svelte.ts` contains only
 * Svelte 5 rune state and store logic.
 */

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
	// HISTORICAL `any` (R23-A audited): `content`/`design`/`animation` round-trip
	// through JSONB and may be either the strict envelope (PopupContent /
	// PopupDesign / PopupAnimation) OR a backward-compat string/partial blob
	// from the legacy WP popup importer. Narrowing them surfaces ~110
	// downstream errors across `admin/popups/{new,[id]}/+page.svelte` and the
	// popup-demo routes — out of R23-A's scope. Keep `any` here and tighten
	// in a dedicated sweep after the admin-popup editor is rewritten.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup config blob; narrowing cascades ~110 errors (R23-A)
	content: any;
	successMessage?: string;
	variantTitle?: string;
	abTestId?: string;
	buttons: PopupButton[];
	closeButton: boolean;
	closeOnOverlayClick: boolean;
	closeOnEscape?: boolean;

	// Design — see comment above; left as `any` until the admin editor migrates.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup config blob (R23-A)
	design: any;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup config blob (R23-A)
	animation: any;
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

	// Backend compatibility properties (snake_case from API).
	// HISTORICAL `any` (R23-A audited): trigger_rules / display_rules /
	// frequency_rules are JSONB on the backend with a structural shape
	// expected by `admin/popups/new` and `admin/popups/[id]/edit` pages —
	// narrowing them surfaces ~20 downstream errors. Out of R23-A's scope.
	priority?: number;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup rules blob; narrowing cascades ~20 errors (R23-A)
	trigger_rules?: any;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup rules blob (R23-A)
	display_rules?: any;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- legacy WP popup rules blob (R23-A)
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

	// FIX-2026-04-26 (cross-cutting audit §F top-10 #7): scheduled show window
	// (ISO datetime strings). Driven from the popup edit/new pages — without
	// these typed here, ~12 `as any` casts around `formData.start_date` /
	// `formData.end_date` are needed in the templates.
	start_date?: string;
	end_date?: string;
}
