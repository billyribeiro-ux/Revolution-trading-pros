/** Forms API client. Verified core routes and remaining gaps are tracked in docs/plugins. */

import { mergeHeaders } from '#lib/utils/http-headers.js';
import { browser } from '$app/env';
import { writable, derived as _derived, get } from 'svelte/store';
import type { FormTheme } from '#lib/data/formTemplates.js';
import { getAuthToken, getSessionId as _getAuthSessionId } from '#lib/stores/auth.svelte.js';
import { logger } from '#lib/utils/logger.js';
import type { JsonValue, PaginatedResponse } from './_types';
import { formWritePayload, normalizeForm } from './forms-contract.js';

// R8-A: re-export `PaginatedResponse` so the type is reachable to forms-area
// callers without forcing them to `import from '#lib/api/_types'` directly.
// `_types.ts` is intentionally underscore-prefixed (private to `api/`); this
// is the public surface for now. Forward-compat for the day forms.rs ships
// an explicit `{ data, meta }` contract — at that point `getForms` will
// return `PaginatedResponse<Form>` and the `forms` / `total` unwrap below
// will go away.
export type { PaginatedResponse };

/**
 * Local error-message helper. Used by every public-API catch handler now
 * that `catch (error: any)` was removed (R7-A precedent). Not exported —
 * each `api/` module carries its own copy today; a shared helper is a
 * future R-* candidate.
 */
function errorMessage(err: unknown): string {
	if (err instanceof Error) return err.message;
	if (typeof err === 'string') return err;
	return String(err);
}

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
// All API calls go through SvelteKit proxy routes
const API_BASE = '/api';
const AI_API = '/api/ai';

const CACHE_TTL = 300000; // 5 minutes
const MAX_RETRIES = 3;
const RETRY_DELAY = 1000;
const OFFLINE_QUEUE_KEY = 'forms_offline_queue';
const AUTOSAVE_INTERVAL = 30000; // 30 seconds

// ═══════════════════════════════════════════════════════════════════════════
// Enhanced Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface FieldValidation {
	required?: boolean;
	min_length?: number;
	max_length?: number;
	pattern?: string;
	pattern_message?: string;
	min?: number;
	max?: number;
	step?: number; // For number/range inputs
	min_date?: string;
	max_date?: string;
	file_types?: string[];
	max_file_size?: number;
	max_size?: number; // Alias for max_file_size
	accept?: string; // For file inputs
	custom?: string; // Custom validation function
	async?: boolean; // Async validation
	debounce?: number; // Debounce validation
}

export interface FieldAttributes {
	autocomplete?: string;
	inputmode?: string;
	spellcheck?: boolean;
	readonly?: boolean;
	disabled?: boolean;
	autofocus?: boolean;
	multiple?: boolean;
	accept?: string; // For file inputs
	capture?: string; // For camera inputs
	// Open-ended pass-through for HTML attributes the explicit fields above
	// don't enumerate (e.g. `pattern`, `minlength` on text inputs). Stays
	// `JsonValue` because the value gets serialized in JSON columns on the
	// backend and re-hydrated; callers (FormFieldRenderer) must narrow.
	[key: string]: JsonValue | undefined;
}

export interface NewsletterFieldOptions {
	checkbox_label?: string;
	show_privacy_link?: boolean;
	privacy_url?: string;
}

export interface FormField {
	id?: number;
	form_id?: number;
	field_type: FieldType;
	label: string;
	name: string;
	placeholder?: string;
	help_text?: string;
	// `default_value` and `options` are genuinely heterogeneous — they round-trip
	// through PostgreSQL JSONB columns and the same `FormField` row can hold:
	//   - default_value: a string (text inputs), boolean (toggle), number
	//     (range/slider), string[] (multiselect), or object (address/payment).
	//   - options: an Array<{label,value}> (select/radio/checkbox), a
	//     newsletter config object (newsletter_*), a `PaymentItem[]`, or a
	//     `string[]`. See `FormFieldRenderer.svelte:680-757` for the union of
	//     concrete shapes; `ConditionalLogicBuilder.svelte:102-104` uses
	//     `Array.isArray` to narrow. Callers MUST narrow at use site.
	default_value?: JsonValue;
	options?: JsonValue;
	validation?: FieldValidation | null;
	conditional_logic?: ConditionalLogic | null;
	attributes?: FieldAttributes | null;
	required: boolean;
	order: number;
	width: 1 | 2 | 3 | 4 | 6 | 12; // Grid system
	ai_suggestions?: string[];
	analytics?: FieldAnalytics;
	created_at?: string;
	updated_at?: string;
}

export type FieldType =
	| 'text'
	| 'email'
	| 'tel'
	| 'phone'
	| 'url'
	| 'number'
	| 'password'
	| 'textarea'
	| 'select'
	| 'multiselect'
	| 'checkbox'
	| 'radio'
	| 'date'
	| 'time'
	| 'datetime'
	| 'file'
	| 'image'
	| 'signature'
	| 'rating'
	| 'slider'
	| 'range'
	| 'toggle'
	| 'color'
	| 'address'
	| 'location'
	| 'payment'
	| 'captcha'
	| 'hidden'
	| 'section'
	| 'html'
	| 'heading'
	| 'divider'
	| 'step'
	| 'page_break'
	| 'quiz'
	| 'repeater'
	| 'consent'
	| 'newsletter_subscribe'
	| 'newsletter_categories'
	| 'newsletter_frequency';

export interface FieldOption {
	label: string;
	value: string;
	disabled?: boolean;
	selected?: boolean;
	icon?: string;
	color?: string;
	description?: string;
	checkbox_label?: string;
	show_privacy_link?: boolean;
	privacy_url?: string;
}

export interface ConditionalLogic {
	enabled: boolean;
	action: 'show' | 'hide' | 'enable' | 'disable' | 'require';
	logic: 'all' | 'any';
	rules: ConditionalRule[];
}

export interface ConditionalRule {
	field: string;
	operator: ConditionalOperator;
	// Heterogeneous comparison target — equals/contains compare against
	// strings, greater_than/less_than against numbers, is_checked against
	// booleans. Narrow at use site.
	value?: JsonValue;
}

export type ConditionalOperator =
	| 'equals'
	| 'not_equals'
	| 'contains'
	| 'not_contains'
	| 'starts_with'
	| 'ends_with'
	| 'greater_than'
	| 'less_than'
	| 'greater_than_or_equal'
	| 'less_than_or_equal'
	| 'is_empty'
	| 'is_not_empty'
	| 'is_checked'
	| 'is_not_checked';

export interface FieldAnalytics {
	interactions: number;
	completions: number;
	errors: number;
	avg_time: number;
	drop_offs: number;
}

export interface FormSettings {
	// Behavior
	send_email?: boolean;
	save_submissions?: boolean;
	allow_drafts?: boolean;
	require_login?: boolean;
	limit_submissions?: number;
	close_after_date?: string;
	submit_text?: string; // Custom submit button text

	// Notifications
	success_message?: string;
	error_message?: string;
	redirect_url?: string;
	notification_emails?: string[];
	email_to?: string; // Primary email recipient
	auto_responder?: boolean;

	// Advanced
	webhook_url?: string;
	integrations?: FormIntegration[];
	analytics_enabled?: boolean;
	ab_testing?: ABTestConfig;
	spam_protection?: SpamProtection;

	// Appearance
	theme?: FormTheme;
	custom_css?: string;
	custom_js?: string;
}

export interface FormIntegration {
	type: 'zapier' | 'webhook' | 'email' | 'slack' | 'crm' | 'analytics';
	enabled: boolean;
	// Integration-specific config — different per provider (webhook URL,
	// Slack channel ID, CRM API key, etc.). JSON column on the backend.
	config: Record<string, JsonValue>;
}

export interface ABTestConfig {
	enabled: boolean;
	variants: FormVariant[];
	traffic_allocation: number[]; // Percentage for each variant
}

export interface FormVariant {
	id: string;
	name: string;
	// A/B variant overrides — patch shape applied to form fields/styles.
	changes: Record<string, JsonValue>;
}

export interface SpamProtection {
	honeypot?: boolean;
	captcha?: 'recaptcha' | 'hcaptcha' | 'custom';
	rate_limiting?: boolean;
	blacklist?: string[];
}

export interface FormStyles {
	// Colors
	primary_color?: string;
	secondary_color?: string;
	background_color?: string;
	text_color?: string;
	border_color?: string;
	error_color?: string;
	success_color?: string;

	// Typography
	font_family?: string;
	font_size?: string;
	line_height?: string;

	// Layout
	max_width?: string;
	padding?: string;
	border_radius?: string;
	shadow?: string;

	// Animations
	animation_type?: 'none' | 'fade' | 'slide' | 'zoom';
	animation_duration?: string;

	// Open-ended for custom CSS-derived properties (e.g. theme-specific
	// gradient stops). Survive JSONB round-trip via JsonValue.
	[key: string]: JsonValue | undefined;
}

export interface Form {
	id?: number;
	title: string;
	slug: string;
	description?: string;
	type?: 'standard' | 'survey' | 'quiz' | 'payment' | 'registration';
	settings?: FormSettings;
	styles?: FormStyles;
	status: 'draft' | 'published' | 'scheduled' | 'archived';
	version?: number;
	parent_id?: number; // For versioning
	created_by?: number;
	submission_count?: number;
	conversion_rate?: number;
	avg_completion_time?: number;
	published_at?: string;
	scheduled_for?: string;
	created_at?: string;
	updated_at?: string;
	deleted_at?: string;
	fields?: FormField[];
	analytics?: FormAnalytics;
	creator?: User;
	collaborators?: User[];
	tags?: string[];
}

export interface User {
	id: number;
	name: string;
	email: string;
	avatar?: string;
	role?: string;
}

/**
 * Daily submission count point from the analytics endpoint's 30-day trend
 * series. Backend handler (`api/src/routes/forms/analytics.rs:93`) emits
 * `[{ date: 'YYYY-MM-DD', count: N }]` from a `GROUP BY DATE(created_at)`.
 */
export interface FormSubmissionTrendPoint {
	date: string;
	count: number;
}

/**
 * Per-status row from the analytics endpoint's status breakdown. Backend
 * handler (`api/src/routes/forms/analytics.rs:94`) emits
 * `[{ status: 'unread'|'read'|'starred'|'archived', count: N }]`. The
 * `status` field mirrors `FormSubmission.status` minus 'complete'/'partial'/
 * 'spam' which the GROUP BY rolls into 'unread' via `COALESCE`.
 */
export interface FormStatusBreakdownRow {
	status: string;
	count: number;
}

/**
 * Wire shape returned by `GET /forms/:id/analytics` (admin). Mirrors the
 * literal `Json(json!({...}))` payload built in
 * `api/src/routes/forms/analytics.rs`. Pre-R24-B this interface was missing
 * `conversion_rate`, `submission_trends`, `status_breakdown` — three fields
 * the backend emits unconditionally; the `FormAnalytics.svelte` template was
 * reading `total_submissions`/`read_count`/etc which simply don't exist in
 * the response, rendering every metric as `0`. See R23-A LB-2.
 */
export interface FormAnalytics {
	views: number;
	unique_views: number;
	submissions: number;
	conversions: number;
	/** Backend emits this as a pre-formatted "{:.1}" string (e.g. "12.5"). */
	conversion_rate?: string;
	avg_time_to_complete: number;
	submission_trends: FormSubmissionTrendPoint[];
	status_breakdown: FormStatusBreakdownRow[];
	field_drop_offs: Record<string, number>;
	device_breakdown: Record<string, number>;
	referrer_breakdown: Record<string, number>;
	geographic_breakdown: Record<string, number>;
}

export interface FormSubmission {
	id?: number;
	form_id: number;
	user_id?: number;
	submission_id: string;
	status: 'complete' | 'partial' | 'unread' | 'read' | 'starred' | 'archived' | 'spam';
	score?: number; // For quizzes
	payment_status?: 'pending' | 'completed' | 'failed';
	ip_address?: string;
	user_agent?: string;
	referrer?: string;
	utm_source?: string;
	utm_medium?: string;
	utm_campaign?: string;
	device_type?: string;
	browser?: string;
	os?: string;
	country?: string;
	city?: string;
	completion_time?: number;
	// Submission metadata (UTM enrichment, device fingerprint, etc.) — JSON
	// column on the backend.
	metadata?: Record<string, JsonValue>;
	created_at?: string;
	updated_at?: string;
	deleted_at?: string;
	data?: SubmissionData[];
	user?: User;
	notes?: SubmissionNote[];
}

export interface SubmissionData {
	id?: number;
	submission_id: number;
	field_id: number;
	field_name: string;
	// The submitted value for ONE field; type depends on the field shape
	// (string for text, boolean for checkbox, string[] for multiselect,
	// object for address/signature). Callers MUST narrow before reading
	// shape-specific methods like `.substring()` / `.length`.
	value: JsonValue;
	validated?: boolean;
	created_at?: string;
	updated_at?: string;
	field?: FormField;
}

export interface SubmissionNote {
	id: number;
	submission_id: number;
	user_id: number;
	note: string;
	created_at: string;
	user: User;
}

// ═══════════════════════════════════════════════════════════════════════════
// Core Service Class
// ═══════════════════════════════════════════════════════════════════════════

class FormsService {
	private static instance: FormsService;
	// R7-A precedent: cache values stored as `unknown`; reads go through a
	// generic `getFromCache<T>(): T | null`. Pre-fix, `any` allowed a cached
	// `0` / `""` / `false` to force a refetch via the `if (cached) ...`
	// truthy-check.
	private cache = new Map<string, { data: unknown; expiry: number }>();
	private cacheAuthToken = '';
	private cacheEpoch = 0;
	private wsConnection?: WebSocket;
	// `unknown` rather than a concrete `T` because the map holds different
	// fetch results keyed by URL; dedup-then-replay returns the same Promise
	// reference and the caller's `authFetch<T>` generic re-establishes T.
	private pendingRequests = new Map<string, Promise<unknown>>();

	// Stores
	public forms = writable<Form[]>([]);
	public currentForm = writable<Form | null>(null);
	public submissions = writable<FormSubmission[]>([]);
	public isLoading = writable(false);
	public error = writable<string | null>(null);
	public offlineMode = writable(false);

	private constructor() {
		this.initialize();
	}

	static getInstance(): FormsService {
		if (!FormsService.instance) {
			FormsService.instance = new FormsService();
		}
		return FormsService.instance;
	}

	/**
	 * Initialize service
	 */
	private initialize(): void {
		if (!browser) return;

		// Old versions persisted complete submissions in localStorage. Remove
		// those copies rather than replaying sensitive data without user action.
		localStorage.removeItem(OFFLINE_QUEUE_KEY);

		// Setup WebSocket connection
		this.setupWebSocket();

		// Setup offline detection
		this.setupOfflineDetection();

		// Setup autosave
		this.setupAutosave();
	}

	/**
	 * Get auth token from the secure auth store
	 * @security Uses memory-only token storage from auth store
	 */
	private getAuthToken(): string {
		if (!browser) return '';
		// Use the auth store's secure token getter (memory-only, not localStorage)
		return getAuthToken() || '';
	}

	/**
	 * Make authenticated request with caching and retry
	 */
	private async authFetch<T>(
		url: string,
		options: RequestInit & {
			skipCache?: boolean;
			cacheTTL?: number;
			retries?: number;
		} = {}
	): Promise<T> {
		const {
			skipCache = false,
			cacheTTL = CACHE_TTL,
			retries = MAX_RETRIES,
			...fetchOptions
		} = options;

		// A login change must not expose the previous user's cached admin forms.
		const authToken = this.getAuthToken();
		if (authToken !== this.cacheAuthToken) {
			this.cache.clear();
			this.cacheAuthToken = authToken;
			this.cacheEpoch += 1;
		}
		const method = (fetchOptions.method ?? 'GET').toUpperCase();
		const isGet = method === 'GET';
		const requestEpoch = this.cacheEpoch;
		const cacheKey = `${requestEpoch}:${method}:${url}`;
		if (isGet && !skipCache) {
			// R7-A precedent: explicit `!== null` test on the generic
			// `getFromCache<T>` return so a cached `0` / `""` / `false`
			// doesn't force a refetch.
			const cached = this.getFromCache<T>(cacheKey);
			if (cached !== null) return cached;
		}

		// Check for pending request (deduplication)
		const pending = isGet ? this.pendingRequests.get(cacheKey) : undefined;
		if (pending) {
			// Same-URL dedup — caller's generic re-establishes T from the
			// stored `Promise<unknown>`.
			return pending as Promise<T>;
		}

		// Create request promise
		// Mutations are not idempotent without a server-issued idempotency key.
		// Never collapse or automatically retry them after an ambiguous failure.
		const requestPromise = this.executeRequest<T>(url, fetchOptions, isGet ? retries : 0);
		if (!isGet) return requestPromise;

		// Store pending request
		this.pendingRequests.set(cacheKey, requestPromise);

		try {
			const result = await requestPromise;

			// Cache successful GET requests
			if (this.cacheEpoch === requestEpoch) {
				this.setCache(cacheKey, result, cacheTTL);
			}

			return result;
		} finally {
			this.pendingRequests.delete(cacheKey);
		}
	}

	/**
	 * Execute request with retry logic
	 */
	private async executeRequest<T>(
		url: string,
		options: RequestInit,
		retriesLeft: number
	): Promise<T> {
		const token = this.getAuthToken();
		const headers = mergeHeaders(
			{
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			},
			options.headers
		);

		try {
			const response = await fetch(url, {
				...options,
				headers
			});

			if (!response.ok) {
				if (response.status === 401) {
					// Try to refresh token before giving up
					try {
						const { authStore } = await import('#lib/stores/auth.svelte.js');
						const refreshed = await authStore.refreshToken();
						if (refreshed && retriesLeft > 0) {
							// Token refreshed, retry the request
							return this.executeRequest<T>(url, options, retriesLeft - 1);
						}
					} catch {
						// Refresh failed, clear auth and redirect
						await this.clearAuth();
					}
				}

				// R7-A precedent: tighten the parsed-error-body shape from
				// `any` to `{ message?: string }`. Pre-fix, if the backend
				// returned `{ "error": "..." }` (no `message`), the thrown
				// error was `"undefined"` — silently swallowing the real
				// message. Now falls through to `HTTP <status>`.
				const errorBody: { message?: string } = await response
					.json()
					.catch(() => ({ message: 'Request failed' }));
				throw new Error(errorBody.message || `HTTP ${response.status}`);
			}

			return response.json();
		} catch (error: unknown) {
			// Retry logic
			if (retriesLeft > 0 && this.shouldRetry(error)) {
				await new Promise((resolve) => setTimeout(resolve, RETRY_DELAY));
				return this.executeRequest<T>(url, options, retriesLeft - 1);
			}

			throw error;
		}
	}

	/**
	 * Cache management
	 *
	 * R7-A precedent: generic getter so the caller's `T` is re-established
	 * after the `unknown`-typed store, and explicit `null` sentinel for miss
	 * (callers MUST compare `!== null`, NOT a truthy check — a cached
	 * `0` / `""` / `false` is a valid hit).
	 */
	private getFromCache<T>(key: string): T | null {
		const cached = this.cache.get(key);
		if (cached && Date.now() < cached.expiry) {
			return cached.data as T;
		}
		return null;
	}

	private setCache(key: string, data: unknown, ttl: number): void {
		this.cache.set(key, {
			data,
			expiry: Date.now() + ttl
		});
	}

	private clearCache(pattern?: string): void {
		if (pattern) {
			for (const key of this.cache.keys()) {
				if (key.includes(pattern)) {
					this.cache.delete(key);
				}
			}
		} else {
			this.cache.clear();
		}
	}

	/**
	 * WebSocket connection for real-time updates
	 * ICT 7 FIX: Only attempt WebSocket if VITE_WS_URL is explicitly configured
	 */
	private setupWebSocket(): void {
		if (!browser || !this.getAuthToken()) return;

		// ICT 7 FIX: Only attempt WebSocket if VITE_WS_URL is explicitly configured and safe
		const configuredWsUrl = import.meta.env['VITE_WS_URL'];
		if (!configuredWsUrl) {
			// Silently skip - WebSocket is optional
			return;
		}

		// Never attempt insecure ws:// on https:// origin (avoids Mixed Content & CSP violations)
		if (
			typeof window !== 'undefined' &&
			window.location.protocol === 'https:' &&
			configuredWsUrl.startsWith('ws:')
		) {
			return;
		}

		try {
			this.wsConnection = new WebSocket(`${configuredWsUrl}/forms`);

			this.wsConnection.onopen = () => {
				this.authenticate();
			};

			this.wsConnection.onmessage = (event) => {
				this.handleWebSocketMessage(event);
			};

			this.wsConnection.onerror = () => {
				// Silently handle - WebSocket is optional
			};

			this.wsConnection.onclose = () => {
				// Don't auto-reconnect - WebSocket is optional
			};
		} catch (_error) {
			// Silently handle - WebSocket is optional
		}
	}

	private authenticate(): void {
		if (!this.wsConnection) return;

		this.wsConnection.send(
			JSON.stringify({
				type: 'auth',
				token: this.getAuthToken()
			})
		);
	}

	private handleWebSocketMessage(event: MessageEvent): void {
		try {
			const message = JSON.parse(event.data);

			switch (message.type) {
				case 'form_updated':
					this.handleFormUpdate(message.data);
					break;
				case 'new_submission':
					this.handleNewSubmission(message.data);
					break;
				case 'collaboration':
					this.handleCollaboration(message.data);
					break;
			}
		} catch (error) {
			logger.error('[FormsService] Failed to handle WebSocket message', { error });
		}
	}

	private handleFormUpdate(form: Form): void {
		this.forms.update((forms) => {
			const index = forms.findIndex((f) => f.id === form.id);
			if (index >= 0) {
				forms[index] = form;
			} else {
				forms.push(form);
			}
			return forms;
		});

		// Update current form if it's the same
		const current = get(this.currentForm);
		if (current?.id === form.id) {
			this.currentForm.set(form);
		}
	}

	private handleNewSubmission(submission: FormSubmission): void {
		this.submissions.update((subs) => [...subs, submission]);

		// Show notification
		this.showNotification(`New submission for form ${submission.form_id}`);
	}

	private handleCollaboration(data: JsonValue): void {
		// Handle real-time collaboration updates. Logged for now; future work
		// will narrow on `data.type` once collab message shape stabilizes.
		logger.debug('[FormsService] Collaboration update', { data });
	}

	/**
	 * Offline support
	 */
	private setupOfflineDetection(): void {
		if (!browser) return;

		window.addEventListener('online', () => {
			this.offlineMode.set(false);
		});

		window.addEventListener('offline', () => {
			this.offlineMode.set(true);
		});
	}

	/**
	 * Autosave functionality
	 */
	private setupAutosave(): void {
		if (!browser) return;

		setInterval(() => {
			const form = get(this.currentForm);
			if (form && form.status === 'draft') {
				void this.saveDraft(form);
			}
		}, AUTOSAVE_INTERVAL);
	}

	private async saveDraft(form: Form): Promise<void> {
		// `form.id` is optional on `Form`; a draft without a persisted id can't
		// be PUT to `/forms/:id`. Guard instead of asserting non-null.
		if (form.id === undefined) return;
		try {
			await this.updateForm(form.id, { ...form, status: 'draft' });
		} catch (error) {
			logger.error('[FormsService] Failed to save draft', { error });
		}
	}

	/**
	 * Utilities
	 */
	private shouldRetry(error: unknown): boolean {
		const message = error instanceof Error ? error.message : '';
		return message.includes('Network') || message.includes('fetch');
	}

	private async clearAuth(): Promise<void> {
		if (!browser) return;
		// Import and use the auth store's logout method
		// This properly clears the secure memory-only token and handles redirect
		const { authStore } = await import('#lib/stores/auth.svelte.js');
		await authStore.logout('/login');
	}

	private showNotification(message: string): void {
		// Implement notification system. Until then, route through the
		// production-safe logger (dev-only output).
		logger.info('[Notification]', message);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Forms Management
	 */
	async getForms(
		page = 1,
		perPage = 20,
		filters?: { status?: string }
	): Promise<{ forms: Form[]; total: number; perPage: number }> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const params = new URLSearchParams({
				page: page.toString(),
				per_page: perPage.toString()
			});

			// Add status filter if provided
			if (filters?.status) {
				params.set('status', filters.status);
			}

			// R8-A / R2-03 follow-up: the backend (`api/src/routes/forms.rs:126-134`)
			// returns exactly one shape for `GET /forms` — the canonical
			// envelope `{ data: Form[], meta: { current_page, per_page, total,
			// total_pages } }`. The four legacy fallbacks below
			// (`response.data.data`, `response.forms`, `Array.isArray(response)`)
			// are DEAD CODE per the current Rust handler — but the task spec
			// rules out backend changes AND user-level CLAUDE.md flags "Let
			// me also clean up these unrelated things" as scope creep. Path
			// of least surprise: keep the defensive shape-probing, just
			// re-type the boundary as `unknown` and narrow at each branch.
			// This deletes 1 `any` (the `authFetch<any>`) while preserving
			// the 5-shape compat that some external consumer might depend on.
			const response = await this.authFetch<unknown>(`${API_BASE}/admin/forms?${params}`);

			let forms: Form[] = [];
			let total = 0;
			let responsePerPage = perPage;
			const isRecord = (v: unknown): v is Record<string, unknown> =>
				typeof v === 'object' && v !== null && !Array.isArray(v);

			if (
				isRecord(response) &&
				isRecord(response['data']) &&
				Array.isArray(response['data']['data'])
			) {
				// Paginated wrapped: { data: { data: [], total, per_page } }
				const inner = response['data'] as { data: unknown[]; total?: number; per_page?: number };
				forms = inner.data.map(normalizeForm);
				total = inner.total ?? forms.length;
				responsePerPage = inner.per_page ?? perPage;
			} else if (isRecord(response) && Array.isArray(response['data'])) {
				// Canonical envelope from forms.rs:126-134:
				// { data: Form[], meta: { total, ... } }
				forms = response['data'].map(normalizeForm);
				if (isRecord(response['meta']) && typeof response['meta']['total'] === 'number') {
					total = response['meta']['total'];
				} else {
					total = forms.length;
				}
				if (isRecord(response['meta']) && typeof response['meta']['per_page'] === 'number') {
					responsePerPage = response['meta']['per_page'];
				}
			} else if (isRecord(response) && Array.isArray(response['forms'])) {
				// Legacy format: { forms: Form[], total, perPage }
				forms = response['forms'].map(normalizeForm);
				total = typeof response['total'] === 'number' ? response['total'] : forms.length;
				if (typeof response['perPage'] === 'number') {
					responsePerPage = response['perPage'];
				}
			} else if (Array.isArray(response)) {
				// Direct array response
				forms = response.map(normalizeForm);
				total = forms.length;
			}

			this.forms.set(forms);
			return {
				forms,
				total,
				perPage: responsePerPage
			};
		} finally {
			this.isLoading.set(false);
		}
	}

	async getForm(id: number): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const form = normalizeForm(await this.authFetch<unknown>(`${API_BASE}/admin/forms/${id}`));
			this.currentForm.set(form);

			return form;
		} catch (error: unknown) {
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	async createForm(formData: Partial<Form>): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const form = normalizeForm(
				await this.authFetch<unknown>(`${API_BASE}/admin/forms`, {
					method: 'POST',
					body: JSON.stringify(formWritePayload(formData)),
					skipCache: true
				})
			);

			this.forms.update((forms) => [...forms, form]);
			this.clearCache('/forms');

			return form;
		} catch (error: unknown) {
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	async updateForm(id: number, formData: Partial<Form>): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			// Optimistic update
			this.forms.update((forms) => {
				const index = forms.findIndex((f) => f.id === id);
				if (index >= 0) {
					// Merge with existing form, ensuring all required properties are present
					const existingForm = forms[index];
					forms[index] = { ...existingForm, ...formData } as Form;
				}
				return forms;
			});

			const form = normalizeForm(
				await this.authFetch<unknown>(`${API_BASE}/admin/forms/${id}`, {
					method: 'PUT',
					body: JSON.stringify(formWritePayload(formData)),
					skipCache: true
				})
			);

			this.clearCache('/forms');

			return form;
		} catch (error: unknown) {
			// Revert optimistic update
			await this.getForms();
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	async deleteForm(id: number): Promise<void> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			// Optimistic delete
			this.forms.update((forms) => forms.filter((f) => f.id !== id));

			await this.authFetch(`${API_BASE}/admin/forms/${id}`, {
				method: 'DELETE',
				skipCache: true
			});

			this.clearCache('/forms');
		} catch (error: unknown) {
			// Revert optimistic delete
			await this.getForms();
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	async duplicateForm(id: number): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const form = normalizeForm(
				await this.authFetch<unknown>(`${API_BASE}/admin/forms/${id}/duplicate`, {
					method: 'POST',
					skipCache: true
				})
			);

			this.forms.update((forms) => [...forms, form]);
			this.clearCache('/forms');

			return form;
		} catch (error: unknown) {
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	/**
	 * AI-Powered Features
	 */
	async generateFormWithAI(prompt: string): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const response = await this.authFetch<{ form: Form }>(`${AI_API}/generate-form`, {
				method: 'POST',
				body: JSON.stringify({ prompt }),
				skipCache: true
			});

			return response.form;
		} catch (error: unknown) {
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	async suggestFields(context: string): Promise<FormField[]> {
		try {
			const response = await this.authFetch<{ fields: FormField[] }>(`${AI_API}/suggest-fields`, {
				method: 'POST',
				body: JSON.stringify({ context }),
				skipCache: true
			});

			return response.fields;
		} catch (error) {
			logger.error('[FormsService] Failed to get field suggestions', { error });
			return [];
		}
	}

	async optimizeForm(formId: number): Promise<Form> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const response = await this.authFetch<{ form: Form; suggestions: string[] }>(
				`${AI_API}/optimize-form/${formId}`,
				{
					method: 'POST',
					skipCache: true
				}
			);

			return response.form;
		} catch (error: unknown) {
			this.error.set(errorMessage(error));
			throw error;
		} finally {
			this.isLoading.set(false);
		}
	}

	/**
	 * Submissions Management
	 */
	async submitForm(
		slug: string,
		data: Record<string, JsonValue>
	): Promise<{
		success: boolean;
		message?: string;
		submission_id?: string;
		errors?: Record<string, string[]>;
		redirect_url?: string;
	}> {
		const response = await fetch(`${API_BASE}/forms/${slug}/submit`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json'
			},
			body: JSON.stringify(data)
		});

		const result: {
			success: boolean;
			message?: string;
			submission_id?: string;
			errors?: Record<string, string[]>;
			redirect_url?: string;
		} = await response.json();
		if ((!response.ok || !result.success) && !result.errors) {
			throw new Error(result.message || `Submission failed (HTTP ${response.status})`);
		}

		return result;
	}

	async getSubmissions(
		formId: number,
		page = 1,
		perPage = 20,
		// Free-form filter dict — keys map 1:1 to backend query string
		// parameters (`status`, `from`, `to`, etc.). Values must already be
		// stringified by the caller; spread into URLSearchParams below.
		filters?: Record<string, string>
	): Promise<{
		submissions: FormSubmission[];
		total?: number;
		perPage?: number;
		currentPage?: number;
	}> {
		this.isLoading.set(true);
		this.error.set(null);

		try {
			const params = new URLSearchParams({
				page: page.toString(),
				per_page: perPage.toString(),
				...filters
			});

			const response = await this.authFetch<{
				submissions: FormSubmission[];
				total?: number;
				perPage?: number;
				currentPage?: number;
				meta?: { total: number; per_page: number; current_page: number };
			}>(`${API_BASE}/admin/forms/${formId}/submissions?${params}`);

			this.submissions.set(response.submissions);
			return {
				...response,
				total: response.meta?.total ?? response.total,
				perPage: response.meta?.per_page ?? response.perPage,
				currentPage: response.meta?.current_page ?? response.currentPage
			};
		} finally {
			this.isLoading.set(false);
		}
	}

	async exportSubmissions(formId: number, format: 'csv' | 'excel' | 'pdf' = 'csv'): Promise<Blob> {
		if (format !== 'csv') throw new Error('Only CSV submission export is supported');
		const token = this.getAuthToken();
		const response = await fetch(
			`${API_BASE}/admin/forms/${formId}/submissions/export?format=${format}`,
			{
				headers: {
					Accept: format === 'csv' ? 'text/csv' : 'application/octet-stream',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				}
			}
		);

		if (!response.ok) {
			throw new Error('Export failed');
		}

		return response.blob();
	}

	/**
	 * Analytics & Insights
	 */
	async getFormAnalytics(formId: number): Promise<FormAnalytics> {
		return this.authFetch<FormAnalytics>(`${API_BASE}/admin/forms/${formId}/analytics`);
	}

	// Analytics endpoints below return shapes the backend doesn't define
	// (`utoipa::path` not applied — see R2-03 report) and that no caller in
	// the repo reads off concretely as of R8-A. Type as `JsonValue` so
	// callers must narrow at use site rather than write `.foo.bar` on
	// truly unknown data. R7-A precedent (`getRedemptionReport`,
	// `getROIReport`).
	async getConversionFunnel(formId: number): Promise<JsonValue> {
		return this.authFetch<JsonValue>(`${API_BASE}/forms/${formId}/analytics/funnel`);
	}

	async getFieldHeatmap(formId: number): Promise<JsonValue> {
		return this.authFetch<JsonValue>(`${API_BASE}/forms/${formId}/analytics/heatmap`);
	}

	async getUserRecording(submissionId: string): Promise<JsonValue> {
		return this.authFetch<JsonValue>(`${API_BASE}/submissions/${submissionId}/recording`);
	}

	/**
	 * A/B Testing
	 */
	async createABTest(formId: number, config: ABTestConfig): Promise<JsonValue> {
		return this.authFetch<JsonValue>(`${API_BASE}/forms/${formId}/ab-test`, {
			method: 'POST',
			body: JSON.stringify(config),
			skipCache: true
		});
	}

	async getABTestResults(formId: number, testId: string): Promise<JsonValue> {
		return this.authFetch<JsonValue>(`${API_BASE}/forms/${formId}/ab-test/${testId}/results`);
	}

	/**
	 * Real-time Collaboration
	 */
	joinCollaboration(formId: number): void {
		if (!this.wsConnection) return;

		this.wsConnection.send(
			JSON.stringify({
				type: 'join_collaboration',
				form_id: formId
			})
		);
	}

	leaveCollaboration(formId: number): void {
		if (!this.wsConnection) return;

		this.wsConnection.send(
			JSON.stringify({
				type: 'leave_collaboration',
				form_id: formId
			})
		);
	}

	sendCollaborationUpdate(formId: number, update: JsonValue): void {
		if (!this.wsConnection) return;

		this.wsConnection.send(
			JSON.stringify({
				type: 'collaboration_update',
				form_id: formId,
				update
			})
		);
	}

	/**
	 * Validation
	 */
	async validateField(
		field: FormField,
		value: JsonValue
	): Promise<{ valid: boolean; errors: string[] }> {
		const errors: string[] = [];

		const isEmpty =
			value == null ||
			(typeof value === 'string' && value.trim().length === 0) ||
			(Array.isArray(value) && value.length === 0);
		if (field.required && isEmpty) {
			errors.push(`${field.label} is required`);
		}

		// Type-specific validation
		if (field.validation) {
			// Min/Max length
			if (typeof value === 'string') {
				if (field.validation.min_length != null && value.length < field.validation.min_length) {
					errors.push(`${field.label} must be at least ${field.validation.min_length} characters`);
				}
				if (field.validation.max_length != null && value.length > field.validation.max_length) {
					errors.push(
						`${field.label} must be no more than ${field.validation.max_length} characters`
					);
				}
			}

			// Pattern validation — `RegExp.test()` requires a string. Pre-R8-A
			// `value: any` let this through with `regex.test(undefined)`
			// stringifying to `"undefined"` (a latent bug); now we only run
			// the regex when the input is genuinely a string.
			if (field.validation.pattern && typeof value === 'string') {
				try {
					const regex = new RegExp(field.validation.pattern);
					if (!regex.test(value)) {
						errors.push(field.validation.pattern_message || `${field.label} format is invalid`);
					}
				} catch {
					errors.push(`${field.label} validation pattern is invalid`);
				}
			}

			// Async validation
			if (field.validation.async) {
				try {
					const response = await this.authFetch<{ valid: boolean; message?: string }>(
						`${API_BASE}/validate`,
						{
							method: 'POST',
							body: JSON.stringify({ field, value }),
							skipCache: true
						}
					);

					if (response.valid !== true) {
						const message = typeof response.message === 'string' ? response.message.trim() : '';
						errors.push(message || `${field.label} could not be validated`);
					}
				} catch (error) {
					logger.error('[FormsService] Async validation failed', { error });
					errors.push(`${field.label} could not be validated`);
				}
			}
		}

		return { valid: errors.length === 0, errors };
	}

	/**
	 * Templates
	 */
	async getFormTemplates(category?: string): Promise<Form[]> {
		const params = category ? `?category=${category}` : '';
		return this.authFetch<Form[]>(`${API_BASE}/forms/templates${params}`);
	}

	async createFromTemplate(templateId: number): Promise<Form> {
		const form = await this.authFetch<Form>(`${API_BASE}/forms/templates/${templateId}/use`, {
			method: 'POST',
			skipCache: true
		});

		this.forms.update((forms) => [...forms, form]);

		return form;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Export singleton instance and convenience functions
// ═══════════════════════════════════════════════════════════════════════════

const formsService = FormsService.getInstance();

// Export stores
export const forms = formsService.forms;
export const currentForm = formsService.currentForm;
export const submissions = formsService.submissions;
export const isLoading = formsService.isLoading;
export const error = formsService.error;
export const offlineMode = formsService.offlineMode;

// Export methods
export const getForms = (
	page?: number,
	perPage?: number,
	filters?: { status?: string }
): Promise<{ forms: Form[]; total: number; perPage: number }> =>
	formsService.getForms(page, perPage, filters);

export const getForm = (id: number) => formsService.getForm(id);

export const createForm = (formData: Partial<Form>) => formsService.createForm(formData);

export const updateForm = (id: number, formData: Partial<Form>) =>
	formsService.updateForm(id, formData);

export const deleteForm = (id: number) => formsService.deleteForm(id);

export const duplicateForm = (id: number) => formsService.duplicateForm(id);

export const generateFormWithAI = (prompt: string) => formsService.generateFormWithAI(prompt);

export const suggestFields = (context: string) => formsService.suggestFields(context);

export const optimizeForm = (formId: number) => formsService.optimizeForm(formId);

export const submitForm = (slug: string, data: Record<string, JsonValue>) =>
	formsService.submitForm(slug, data);

export const previewForm = async (slug: string): Promise<Form> => {
	try {
		const response = await fetch(`${API_BASE}/forms/${encodeURIComponent(slug)}`);
		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Form not found' }));
			throw new Error(errorData.message || `Failed to load form preview (HTTP ${response.status})`);
		}
		return normalizeForm(await response.json());
	} catch (error) {
		// Log error for debugging while preserving the error chain
		logger.error('[FormsAPI] previewForm error', { error });
		throw error instanceof Error ? error : new Error('Failed to preview form');
	}
};

/**
 * Get form by numeric ID (FluentForm-style)
 * This fetches a published form by its database ID
 */
export const getFormById = async (id: number): Promise<Form> => {
	try {
		const response = await fetch(`${API_BASE}/forms/${id}`);
		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Form not found' }));
			throw new Error(errorData.message || `Failed to load form (HTTP ${response.status})`);
		}
		return normalizeForm(await response.json());
	} catch (error) {
		// Log error for debugging while preserving the error chain
		logger.error('[FormsAPI] getFormById error', { error });
		throw error instanceof Error ? error : new Error('Failed to load form');
	}
};

export const getSubmissions = (
	formId: number,
	page?: number,
	perPage?: number,
	filters?: Record<string, string>
) => formsService.getSubmissions(formId, page, perPage, filters);

export const exportSubmissions = (formId: number, format?: 'csv' | 'excel' | 'pdf') =>
	formsService.exportSubmissions(formId, format);

export const getFormAnalytics = (formId: number) => formsService.getFormAnalytics(formId);

export const getConversionFunnel = (formId: number) => formsService.getConversionFunnel(formId);

export const getFieldHeatmap = (formId: number) => formsService.getFieldHeatmap(formId);

export const validateField = (field: FormField, value: JsonValue) =>
	formsService.validateField(field, value);

export const getFormTemplates = (category?: string) => formsService.getFormTemplates(category);

export const createFromTemplate = (templateId: number) =>
	formsService.createFromTemplate(templateId);

// Real-time collaboration
export const joinCollaboration = (formId: number) => formsService.joinCollaboration(formId);

export const leaveCollaboration = (formId: number) => formsService.leaveCollaboration(formId);

export const sendCollaborationUpdate = (formId: number, update: JsonValue) =>
	formsService.sendCollaborationUpdate(formId, update);

// A/B Testing
export const createABTest = (formId: number, config: ABTestConfig) =>
	formsService.createABTest(formId, config);

export const getABTestResults = (formId: number, testId: string) =>
	formsService.getABTestResults(formId, testId);

/**
 * @deprecated Use crmAPI from '#lib/api/crm.js' instead for contact management.
 * This export is maintained for backwards compatibility only.
 */
export { crmAPI as contactsApi } from './crm';

// Form actions
export const publishForm = (formId: number) =>
	formsService.updateForm(formId, { status: 'published' });

export const unpublishForm = (formId: number) =>
	formsService.updateForm(formId, { status: 'draft' });
export const archiveForm = (formId: number) =>
	formsService.updateForm(formId, { status: 'archived' });

// Submission management - Connected to real backend API
// @security Uses secure token getter from auth store (memory-only, not localStorage)
export const updateSubmissionStatus = async (
	formId: number,
	submissionId: number | string,
	status: string
) => {
	const token = getAuthToken();
	const response = await fetch(
		`${API_BASE}/admin/forms/${formId}/submissions/${submissionId}/status`,
		{
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify({ status })
		}
	);
	if (!response.ok) throw new Error('Failed to update submission status');
	return response.json();
};

export const deleteSubmission = async (formId: number, submissionId: number | string) => {
	const token = getAuthToken();
	const response = await fetch(`${API_BASE}/admin/forms/${formId}/submissions/${submissionId}`, {
		method: 'DELETE',
		headers: {
			Authorization: `Bearer ${token}`
		}
	});
	if (!response.ok) throw new Error('Failed to delete submission');
	return response.json();
};

export const bulkUpdateSubmissionStatus = async (
	formId: number,
	submissionIds: (number | string)[],
	status: string
) => {
	const token = getAuthToken();
	const response = await fetch(`${API_BASE}/admin/forms/${formId}/submissions/bulk-update-status`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify({ submission_ids: submissionIds, status })
	});
	if (!response.ok) throw new Error('Failed to bulk update submissions');
	return response.json();
};

export const bulkDeleteSubmissions = async (formId: number, submissionIds: (number | string)[]) => {
	const token = getAuthToken();
	const response = await fetch(`${API_BASE}/admin/forms/${formId}/submissions/bulk-delete`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify({ submission_ids: submissionIds })
	});
	if (!response.ok) throw new Error('Failed to bulk delete submissions');
	return response.json();
};

// Type exports
export interface FormEntry {
	id: number;
	form_id: number;
	// Per-field submission data, keyed by field name. Each value's type
	// depends on the field shape (string / boolean / number / object).
	// JSON column on the backend.
	data: Record<string, JsonValue>;
	status?: string;
	created_at: string;
	updated_at?: string;
}

/**
 * @deprecated Use Contact type from '#lib/crm/types.js' instead.
 */
export type { Contact } from '#lib/crm/types.js';

// Analytics
export const getSubmissionStats = (formId: number) => formsService.getFormAnalytics(formId);

// Field types
export const getFieldTypes = () => [
	{ type: 'text', label: 'Text Input', icon: 'text' },
	{ type: 'email', label: 'Email', icon: 'mail' },
	{ type: 'number', label: 'Number', icon: 'hash' },
	{ type: 'textarea', label: 'Text Area', icon: 'align-left' },
	{ type: 'select', label: 'Dropdown', icon: 'chevron-down' },
	{ type: 'radio', label: 'Radio Buttons', icon: 'circle-dot' },
	{ type: 'checkbox', label: 'Checkboxes', icon: 'square-check' },
	{ type: 'date', label: 'Date', icon: 'calendar' },
	{ type: 'file', label: 'File Upload', icon: 'upload' }
];

// Export formsApi for compatibility
export const formsApi = {
	list: () => formsService.getForms(),
	get: (id: number) => formsService.getForm(id),
	create: (data: Partial<Form>) => formsService.createForm(data),
	update: (id: number, data: Partial<Form>) => formsService.updateForm(id, data),
	delete: (id: number) => formsService.deleteForm(id),
	getEntries: (formId: number, page?: number) => formsService.getSubmissions(formId, page),
	exportEntries: (formId: number, format: 'csv' | 'excel' | 'pdf' = 'csv'): Promise<Blob> =>
		formsService.exportSubmissions(formId, format)
};

export default formsService;
