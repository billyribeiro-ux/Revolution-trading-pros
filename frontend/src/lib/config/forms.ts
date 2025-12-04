/**
 * Form Registry - Centralized Form Configuration
 *
 * This is the single source of truth for all forms in the application.
 * Add new forms here and they become available with full TypeScript support.
 *
 * Usage:
 *   import { FORMS } from '$lib/config/forms';
 *   <Form form={FORMS.CONTACT} />
 *
 * @version 1.0.0
 */

// ============================================================================
// Form Definition Types
// ============================================================================

export interface FormDefinition {
	/** Unique identifier for the form */
	id: string;
	/** URL-safe slug for the form */
	slug: string;
	/** Display name (for admin/debugging) */
	name: string;
	/** Optional description */
	description?: string;
}

// ============================================================================
// Form Registry - Define All Forms Here
// ============================================================================

/**
 * FORMS - Central registry of all application forms
 *
 * Add new forms here. Each form needs:
 * - id: Unique identifier (matches database ID or generated UUID)
 * - slug: URL-safe slug (what you set in admin when creating the form)
 * - name: Human-readable name
 *
 * After adding a form here, you can use it like:
 *   <Form form={FORMS.YOUR_NEW_FORM} />
 */
export const FORMS = {
	// -------------------------------------------------------------------------
	// Contact & Communication Forms
	// -------------------------------------------------------------------------

	/** Main contact form - appears on /contact page */
	CONTACT: {
		id: 'contact',
		slug: 'contact-us',
		name: 'Contact Us',
		description: 'Main site contact form'
	},

	/** Quick contact form - for footer/sidebar */
	CONTACT_QUICK: {
		id: 'contact-quick',
		slug: 'quick-contact',
		name: 'Quick Contact',
		description: 'Shortened contact form for widgets'
	},

	// -------------------------------------------------------------------------
	// Newsletter & Subscription Forms
	// -------------------------------------------------------------------------

	/** Newsletter signup form */
	NEWSLETTER: {
		id: 'newsletter',
		slug: 'newsletter-signup',
		name: 'Newsletter Signup',
		description: 'Email newsletter subscription'
	},

	/** Trading alerts signup */
	TRADING_ALERTS: {
		id: 'trading-alerts',
		slug: 'trading-alerts-signup',
		name: 'Trading Alerts Signup',
		description: 'Subscribe to trading alerts'
	},

	// -------------------------------------------------------------------------
	// Lead Generation Forms
	// -------------------------------------------------------------------------

	/** Request a quote form */
	QUOTE_REQUEST: {
		id: 'quote',
		slug: 'request-quote',
		name: 'Request a Quote',
		description: 'Service quote request form'
	},

	/** Free consultation request */
	CONSULTATION: {
		id: 'consultation',
		slug: 'free-consultation',
		name: 'Free Consultation',
		description: 'Book a free consultation'
	},

	/** Demo request form */
	DEMO_REQUEST: {
		id: 'demo',
		slug: 'request-demo',
		name: 'Request Demo',
		description: 'Request a product demo'
	},

	// -------------------------------------------------------------------------
	// Feedback & Survey Forms
	// -------------------------------------------------------------------------

	/** General feedback form */
	FEEDBACK: {
		id: 'feedback',
		slug: 'feedback',
		name: 'Feedback',
		description: 'General feedback form'
	},

	/** Customer satisfaction survey */
	SATISFACTION_SURVEY: {
		id: 'satisfaction',
		slug: 'satisfaction-survey',
		name: 'Satisfaction Survey',
		description: 'Customer satisfaction survey'
	},

	/** NPS (Net Promoter Score) survey */
	NPS_SURVEY: {
		id: 'nps',
		slug: 'nps-survey',
		name: 'NPS Survey',
		description: 'Net Promoter Score survey'
	},

	// -------------------------------------------------------------------------
	// Support Forms
	// -------------------------------------------------------------------------

	/** Support ticket form */
	SUPPORT_TICKET: {
		id: 'support',
		slug: 'support-ticket',
		name: 'Support Ticket',
		description: 'Submit a support ticket'
	},

	/** Bug report form */
	BUG_REPORT: {
		id: 'bug-report',
		slug: 'bug-report',
		name: 'Bug Report',
		description: 'Report a bug or issue'
	},

	/** Feature request form */
	FEATURE_REQUEST: {
		id: 'feature-request',
		slug: 'feature-request',
		name: 'Feature Request',
		description: 'Request a new feature'
	},

	// -------------------------------------------------------------------------
	// Registration & Application Forms
	// -------------------------------------------------------------------------

	/** Event registration form */
	EVENT_REGISTRATION: {
		id: 'event-registration',
		slug: 'event-registration',
		name: 'Event Registration',
		description: 'Register for events'
	},

	/** Webinar registration */
	WEBINAR_REGISTRATION: {
		id: 'webinar',
		slug: 'webinar-registration',
		name: 'Webinar Registration',
		description: 'Register for webinars'
	},

	/** Job application form */
	JOB_APPLICATION: {
		id: 'careers',
		slug: 'job-application',
		name: 'Job Application',
		description: 'Career application form'
	},

	/** Partner application */
	PARTNER_APPLICATION: {
		id: 'partner',
		slug: 'partner-application',
		name: 'Partner Application',
		description: 'Partnership application form'
	},

	// -------------------------------------------------------------------------
	// Trading Specific Forms
	// -------------------------------------------------------------------------

	/** Account opening form */
	ACCOUNT_OPENING: {
		id: 'account-opening',
		slug: 'open-account',
		name: 'Open Account',
		description: 'Trading account opening form'
	},

	/** Strategy consultation */
	STRATEGY_CONSULTATION: {
		id: 'strategy',
		slug: 'strategy-consultation',
		name: 'Strategy Consultation',
		description: 'Trading strategy consultation'
	},

	/** Portfolio review request */
	PORTFOLIO_REVIEW: {
		id: 'portfolio-review',
		slug: 'portfolio-review',
		name: 'Portfolio Review',
		description: 'Request portfolio review'
	},

} as const satisfies Record<string, FormDefinition>;

// ============================================================================
// Type Exports
// ============================================================================

/** Type for form keys (e.g., 'CONTACT', 'NEWSLETTER') */
export type FormKey = keyof typeof FORMS;

/** Type for form definitions */
export type FormConfig = typeof FORMS[FormKey];

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Get a form definition by its key
 */
export function getForm(key: FormKey): FormDefinition {
	return FORMS[key];
}

/**
 * Get a form definition by slug
 */
export function getFormBySlug(slug: string): FormDefinition | undefined {
	return Object.values(FORMS).find(form => form.slug === slug);
}

/**
 * Get a form definition by ID
 */
export function getFormById(id: string): FormDefinition | undefined {
	return Object.values(FORMS).find(form => form.id === id);
}

/**
 * Get all form definitions
 */
export function getAllForms(): FormDefinition[] {
	return Object.values(FORMS);
}

/**
 * Check if a form exists in the registry
 */
export function formExists(slugOrId: string): boolean {
	return Object.values(FORMS).some(
		form => form.slug === slugOrId || form.id === slugOrId
	);
}

// ============================================================================
// Form Groups (for categorization)
// ============================================================================

export const FORM_GROUPS = {
	CONTACT: [FORMS.CONTACT, FORMS.CONTACT_QUICK],
	NEWSLETTER: [FORMS.NEWSLETTER, FORMS.TRADING_ALERTS],
	LEADS: [FORMS.QUOTE_REQUEST, FORMS.CONSULTATION, FORMS.DEMO_REQUEST],
	FEEDBACK: [FORMS.FEEDBACK, FORMS.SATISFACTION_SURVEY, FORMS.NPS_SURVEY],
	SUPPORT: [FORMS.SUPPORT_TICKET, FORMS.BUG_REPORT, FORMS.FEATURE_REQUEST],
	REGISTRATION: [FORMS.EVENT_REGISTRATION, FORMS.WEBINAR_REGISTRATION, FORMS.JOB_APPLICATION, FORMS.PARTNER_APPLICATION],
	TRADING: [FORMS.ACCOUNT_OPENING, FORMS.STRATEGY_CONSULTATION, FORMS.PORTFOLIO_REVIEW],
} as const;
