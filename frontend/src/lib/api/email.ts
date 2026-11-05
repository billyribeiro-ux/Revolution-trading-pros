/**
 * Email Marketing & Automation API Client
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise email marketing system that surpasses ActiveCampaign, Klaviyo,
 * Mailchimp, ConvertKit, and HubSpot.
 *
 * @version 1.0.0
 */

import {
	emailCampaignEnvelope,
	emailCampaignPage,
	emailCampaignWrite,
	campaignId,
	emailCampaignDeleted,
	emailCampaignAction,
	emailCampaignReportStats
} from './email-campaign-catalog.js';
import { emailAnalytics } from './email-analytics.js';
import {
	catalogSequenceEnvelope,
	catalogSequencePage,
	catalogSequenceEmail,
	sequenceCatalogWrite,
	sequenceEmailWrite,
	sequenceId as checkedSequenceId,
	sequenceEmailId,
	catalogSequenceStats
} from './email-sequence-catalog.js';
import { campaignsApi } from './campaigns.js';
import type { CampaignTestOptions } from './campaign-test.js';
import type { ABTestConfig } from './campaigns.js';
import { get as _get } from 'svelte/store';
import { authStore } from '#lib/stores/auth.svelte.js';
import type { JsonValue, QueryParams } from './_types';
import {
	catalogTemplateId,
	catalogTemplateEnvelope,
	catalogTemplatePage,
	catalogTemplateWrite,
	catalogTemplateDeleted
} from './email-template-catalog.js';

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
const API_BASE = '';

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface EmailCampaign {
	id: string;
	name: string;
	subject: string;
	preview_text?: string;
	from_name: string;
	from_email: string;
	reply_to?: string | null;
	template_id?: string | null;
	html_content: string;
	plain_text_content?: string | null;
	status:
		| 'draft'
		| 'scheduled'
		| 'sending'
		| 'sent'
		| 'paused'
		| 'failed'
		| 'cancelled'
		| 'archived';
	type: 'broadcast' | 'automation' | 'sequence' | 'transactional';
	segment_id?: string | null;
	send_at?: string | null;
	sent_at?: string | null;
	stats?: RecordedCampaignEmailStats;
	ab_test?: ABTest | null;
	ab_test_config?: ABTestConfig | null;
	tags: string[];
	created_by: number | null;
	created_at: string;
	updated_at: string;
}

export interface RecordedCampaignEmailStats {
	sent: number;
	delivered: number | null;
	opened: number;
	clicked: number;
	bounced: number;
	complained: number | null;
	unsubscribed: number;
	open_rate: number | null;
	click_rate: number | null;
	bounce_rate: number | null;
	complaint_rate: number | null;
}

export interface EmailTemplate {
	id: string;
	name: string;
	slug: string;
	subject: string;
	is_active: boolean;
	variables?: JsonValue[];
	description?: string;
	category: string;
	thumbnail?: string;
	html_content: string;
	plain_text_content?: string | null;
	/**
	 * Builder-state blob (grapesjs / mjml / homegrown JSON). Shape varies by
	 * template editor; treat as opaque JSON on the client and round-trip it
	 * to/from the backend unchanged. Callers that need to read or mutate the
	 * design tree must narrow with `typeof` / `Array.isArray` first.
	 */
	design_json?: JsonValue;
	is_public: boolean;
	usage_count: number;
	tags: string[];
	created_at: string;
	updated_at: string;
}

export interface EmailSequence {
	id: string;
	name: string;
	description?: string;
	trigger_type: 'manual' | 'signup' | 'purchase' | 'tag_added' | 'custom_event';
	/**
	 * Trigger-specific configuration bag — keys vary by `trigger_type`
	 * (e.g. `tag_added` carries `{ tag: string }`, `custom_event` carries
	 * `{ event_name: string; properties?: Record<string, JsonValue> }`).
	 * Modelled as opaque JSON because the backend stores the DSL verbatim.
	 */
	trigger_config?: JsonValue;
	status: 'active' | 'paused' | 'draft' | 'completed';
	definition_error?: string | null;
	historical_steps?: JsonValue;
	emails: SequenceEmail[];
	stats?: SequenceStats;
	subscribers_count: number;
	created_at: string;
	updated_at: string;
}

export interface SequenceEmail {
	id: string;
	sequence_id: string;
	step_number: number;
	name: string;
	subject: string;
	html_content: string;
	plain_text_content?: string;
	delay_value: number;
	delay_unit: 'minutes' | 'hours' | 'days' | 'weeks';
	conditions?: SequenceCondition[];
	stats?: EmailStats;
}

export interface SequenceCondition {
	type: 'opened' | 'clicked' | 'not_opened' | 'not_clicked' | 'tag' | 'custom';
	/**
	 * Condition-specific value (e.g. a tag name string for `tag`, a JSON
	 * predicate for `custom`). Opaque JSON on the client.
	 */
	value?: JsonValue;
	previous_email_id?: string;
}

export interface EmailAutomation {
	id: string;
	name: string;
	description?: string;
	trigger: AutomationTrigger;
	conditions?: AutomationCondition[];
	actions: AutomationAction[];
	status: 'active' | 'paused' | 'draft';
	stats?: AutomationStats;
	created_at: string;
	updated_at: string;
}

export interface AutomationTrigger {
	type: 'tag_added' | 'tag_removed' | 'field_changed' | 'event' | 'date' | 'webhook';
	/** Trigger-specific config DSL; opaque JSON on the client. */
	config: JsonValue;
}

export interface AutomationCondition {
	field: string;
	operator: 'equals' | 'not_equals' | 'contains' | 'greater_than' | 'less_than';
	/** Comparison value — type depends on `field` (string / number / boolean). */
	value: JsonValue;
}

export interface AutomationAction {
	type: 'send_email' | 'add_tag' | 'remove_tag' | 'update_field' | 'wait' | 'webhook';
	/** Action-specific config DSL; opaque JSON on the client. */
	config: JsonValue;
	delay?: number;
}

export interface EmailSegment {
	id: string;
	name: string;
	description?: string;
	conditions: SegmentCondition[];
	match_type: 'all' | 'any';
	subscribers_count: number;
	created_at: string;
	updated_at: string;
}

export interface SegmentCondition {
	field: string;
	operator: 'equals' | 'not_equals' | 'contains' | 'greater_than' | 'less_than' | 'in' | 'not_in';
	/** Comparison value; scalar for most operators, array for `in`/`not_in`. */
	value: JsonValue;
}

export interface EmailSubscriber {
	id: string;
	email: string;
	first_name?: string;
	last_name?: string;
	status: 'pending' | 'subscribed' | 'unsubscribed' | 'bounced' | 'complained';
	display_name?: string | null;
	confirmation?: ConfirmationReceipt | null;
	tags: string[];
	/**
	 * Subscriber custom fields (account-defined schema). Values are JSON
	 * scalars/structures stored alongside the subscriber; treat as opaque
	 * JSON, narrow at the read site.
	 */
	custom_fields: Record<string, JsonValue>;
	email_score: number | null;
	last_opened_at?: string;
	last_clicked_at?: string;
	subscribed_at: string | null;
	unsubscribed_at?: string;
	created_at: string;
	updated_at: string;
}

export interface SubscriberWrite {
	email?: string;
	first_name?: string;
	last_name?: string;
	tags?: string[];
	custom_fields?: Record<string, JsonValue>;
}
export interface ConfirmationReceipt {
	status: 'processing' | 'accepted' | 'unavailable' | 'failed' | 'unknown';
	message?: string;
	requested_at?: string;
}
export interface SubscriberStats {
	total: number;
	subscribed: number;
	pending: number;
	unsubscribed: number;
	bounced: number;
	complained: number;
}

export interface EmailStats {
	sent: number;
	delivered: number;
	opened: number;
	clicked: number;
	bounced: number;
	complained: number;
	unsubscribed: number;
	open_rate: number;
	click_rate: number;
	bounce_rate: number;
	complaint_rate: number;
	revenue?: number;
}

export interface SequenceStats extends Omit<
	EmailStats,
	'open_rate' | 'click_rate' | 'bounce_rate' | 'complaint_rate' | 'revenue'
> {
	open_rate: number | null;
	click_rate: number | null;
	bounce_rate: number | null;
	complaint_rate: number | null;
	revenue?: number | null;
	active_subscribers: number;
	completed_subscribers: number;
	completion_rate: number | null;
}

export interface AutomationStats {
	triggered: number;
	completed: number;
	emails_sent: number;
	completion_rate: number;
}

export interface ABTest {
	id: string;
	variants: ABVariant[];
	winner_id?: string;
	test_percentage: number;
	winning_metric: 'open_rate' | 'click_rate' | 'conversion_rate';
	status: 'running' | 'completed' | 'cancelled';
}

export interface ABVariant {
	id: string;
	name: string;
	subject: string;
	from_name?: string;
	html_content?: string;
	allocation: number;
	stats?: EmailStats;
}

export interface EmailEvent {
	id: string;
	email_id: string;
	subscriber_id: string;
	type: 'sent' | 'delivered' | 'opened' | 'clicked' | 'bounced' | 'complained' | 'unsubscribed';
	/**
	 * Event-specific metadata bag (link clicked, bounce reason, etc.).
	 * Opaque JSON on the client.
	 */
	metadata?: JsonValue;
	created_at: string;
}

export interface DeliverabilityReport {
	domain: string;
	spf_valid: boolean;
	dkim_valid: boolean;
	dmarc_valid: boolean;
	reputation_score: number;
	inbox_placement_rate: number;
	spam_score: number;
	recommendations: string[];
}

export interface EmailAnalytics {
	period: '7d' | '30d' | '90d' | '365d';
	metrics_scope: 'native_campaign_outbox';
	window: { start: string; end: string; previous_start: string; timezone: 'UTC' };
	campaigns: number;
	new_campaigns: number;
	emails_sent: number;
	total_opens: number;
	total_clicks: number;
	average_open_rate: number | null;
	average_click_rate: number | null;
	comparison: {
		emails_sent_change_pct: number | null;
		open_rate_change_points: number | null;
		click_rate_change_points: number | null;
	};
	top_campaigns: Array<{
		id: string;
		name: string;
		sent: number;
		opens: number;
		open_rate: number;
	}>;
	engagement_trend: Array<{ date: string; opens: number; clicks: number }>;
	device_breakdown: { desktop: number; mobile: number; tablet: number; unknown: number };
	client_breakdown: Record<string, number>;
	unlisted_client_observations: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Client Class
// ═══════════════════════════════════════════════════════════════════════════

class EmailAPI {
	private async request<T>(
		method: string,
		endpoint: string,
		data?: unknown,
		params?: QueryParams
	): Promise<T> {
		// Use secure getter from auth store
		const token = authStore.getToken();

		// ICT 11+ Fix: Add /api prefix if endpoint doesn't already have it
		const apiEndpoint = endpoint.startsWith('/api/') ? endpoint : `/api${endpoint}`;
		const searchParams = new URLSearchParams();
		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				if (value === undefined || value === null) return;
				// `URLSearchParams.append` coerces values to strings;
				// preserve the pre-typing behaviour where arrays collapse via
				// their default `Array.prototype.toString()` (comma-joined),
				// which matches the existing backend parser. Per-key repeat
				// would be a behaviour change so we keep `String(value)`.
				searchParams.append(key, String(value));
			});
		}
		const queryString = searchParams.toString();
		const url = `${API_BASE}${apiEndpoint}${queryString ? `?${queryString}` : ''}`;

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const fetchOptions: RequestInit = {
			method,
			headers,
			credentials: 'include'
		};
		if (data !== undefined) {
			fetchOptions.body = JSON.stringify(data);
		}

		const response = await fetch(url, fetchOptions);

		if (!response.ok) {
			const payload: unknown = await response.json().catch(() => null);
			let message = `HTTP ${response.status}`;
			if (payload && typeof payload === 'object') {
				if ('message' in payload && typeof payload.message === 'string') message = payload.message;
				else if ('error' in payload) {
					if (typeof payload.error === 'string') message = payload.error;
					else if (
						payload.error &&
						typeof payload.error === 'object' &&
						'message' in payload.error &&
						typeof payload.error.message === 'string'
					)
						message = payload.error.message;
				}
			}
			throw new Error(message);
		}

		return response.json();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Campaign Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getCampaigns(params?: {
		status?: string;
		type?: string;
		search?: string;
		page?: number;
		per_page?: number;
	}) {
		return emailCampaignPage(
			await this.request<unknown>('GET', '/admin/email/campaigns', undefined, params)
		);
	}
	async getCampaign(id: string): Promise<{ campaign: EmailCampaign }> {
		return {
			campaign: emailCampaignEnvelope(
				await this.request<unknown>('GET', `/admin/email/campaigns/${campaignId(id)}`),
				campaignId(id)
			)
		};
	}
	async createCampaign(data: Partial<EmailCampaign>): Promise<{ campaign: EmailCampaign }> {
		const campaign = emailCampaignEnvelope(
			await this.request<unknown>('POST', '/admin/email/campaigns', emailCampaignWrite(data))
		);
		if (campaign.status !== 'draft') throw new Error('Campaign draft creation was not confirmed');
		return { campaign };
	}
	async updateCampaign(
		id: string,
		data: Partial<EmailCampaign>
	): Promise<{ campaign: EmailCampaign }> {
		return {
			campaign: emailCampaignEnvelope(
				await this.request<unknown>(
					'PUT',
					`/admin/email/campaigns/${campaignId(id)}`,
					emailCampaignWrite(data)
				),
				campaignId(id)
			)
		};
	}
	async deleteCampaign(id: string): Promise<{ success: boolean }> {
		emailCampaignDeleted(
			await this.request<unknown>('DELETE', `/admin/email/campaigns/${campaignId(id)}`),
			campaignId(id)
		);
		return { success: true };
	}
	async sendCampaign(id: string, options?: { send_at?: string }): Promise<{ success: boolean }> {
		emailCampaignAction(
			await this.request<unknown>('POST', `/admin/email/campaigns/${campaignId(id)}/send`, options),
			campaignId(id),
			'submit'
		);
		return { success: true };
	}
	async pauseCampaign(id: string): Promise<{ success: boolean }> {
		emailCampaignAction(
			await this.request<unknown>('POST', `/admin/email/campaigns/${campaignId(id)}/pause`),
			campaignId(id),
			'pause'
		);
		return { success: true };
	}
	async resumeCampaign(id: string): Promise<{ success: boolean }> {
		emailCampaignAction(
			await this.request<unknown>('POST', `/admin/email/campaigns/${campaignId(id)}/resume`),
			campaignId(id),
			'submit'
		);
		return { success: true };
	}
	async duplicateCampaign(id: string): Promise<{ campaign: EmailCampaign }> {
		const campaign = emailCampaignEnvelope(
			await this.request<unknown>('POST', `/admin/email/campaigns/${campaignId(id)}/duplicate`)
		);
		if (campaign.id === id || campaign.status !== 'draft' || campaign.stats?.sent !== 0)
			throw new Error('Campaign duplication was not confirmed');
		return { campaign };
	}

	async sendTestEmail(
		id: string,
		emails: string[],
		options: CampaignTestOptions = {}
	): Promise<{ success: boolean }> {
		const receipt = await campaignsApi.sendTestEmail(campaignId(id), emails, options);
		if (!receipt.submissions.every((sample) => sample.status === 'accepted'))
			throw new Error(
				'Test message acceptance is unconfirmed; review the saved campaign test receipts'
			);
		return { success: true };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Template Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getTemplates(params?: {
		search?: string;
		category?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ templates: EmailTemplate[]; total: number }> {
		return catalogTemplatePage(
			await this.request('GET', '/admin/email/templates', undefined, params)
		);
	}

	async getTemplate(id: string): Promise<{ template: EmailTemplate }> {
		return catalogTemplateEnvelope(
			await this.request('GET', `/admin/email/templates/${catalogTemplateId(id)}`)
		);
	}

	async createTemplate(data: Partial<EmailTemplate>): Promise<{ template: EmailTemplate }> {
		return catalogTemplateEnvelope(
			await this.request('POST', '/admin/email/templates', catalogTemplateWrite(data, true))
		);
	}

	async updateTemplate(
		id: string,
		data: Partial<EmailTemplate>
	): Promise<{ template: EmailTemplate }> {
		return catalogTemplateEnvelope(
			await this.request(
				'PUT',
				`/admin/email/templates/${catalogTemplateId(id)}`,
				catalogTemplateWrite(data)
			)
		);
	}

	async deleteTemplate(id: string): Promise<{ success: boolean }> {
		return catalogTemplateDeleted(
			await this.request('DELETE', `/admin/email/templates/${catalogTemplateId(id)}`)
		);
	}

	async duplicateTemplate(id: string): Promise<{ template: EmailTemplate }> {
		return catalogTemplateEnvelope(
			await this.request('POST', `/admin/email/templates/${catalogTemplateId(id)}/duplicate`)
		);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Sequence Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getSequences(params?: {
		status?: string;
		search?: string;
		page?: number;
		per_page?: number;
	}): Promise<{
		sequences: EmailSequence[];
		total: number;
		page: number;
		per_page: number;
		last_page: number;
	}> {
		return catalogSequencePage(
			await this.request<unknown>('GET', '/admin/email/sequences', undefined, params)
		);
	}

	async getSequence(id: string): Promise<{ sequence: EmailSequence }> {
		return {
			sequence: catalogSequenceEnvelope(
				await this.request<unknown>('GET', `/admin/email/sequences/${checkedSequenceId(id)}`),
				id
			)
		};
	}

	async createSequence(data: Partial<EmailSequence>): Promise<{ sequence: EmailSequence }> {
		return {
			sequence: catalogSequenceEnvelope(
				await this.request<unknown>('POST', '/admin/email/sequences', sequenceCatalogWrite(data))
			)
		};
	}

	async updateSequence(
		id: string,
		data: Partial<EmailSequence>
	): Promise<{ sequence: EmailSequence }> {
		return {
			sequence: catalogSequenceEnvelope(
				await this.request<unknown>(
					'PUT',
					`/admin/email/sequences/${checkedSequenceId(id)}`,
					sequenceCatalogWrite(data, id)
				),
				id
			)
		};
	}

	async deleteSequence(id: string): Promise<{ success: boolean }> {
		const raw: unknown = await this.request(
			'DELETE',
			`/admin/email/sequences/${checkedSequenceId(id)}`
		);
		if (
			!raw ||
			typeof raw !== 'object' ||
			!('deleted' in raw) ||
			raw.deleted !== true ||
			!('id' in raw) ||
			raw.id !== id
		)
			throw new Error('Sequence deletion was not confirmed');
		return { success: true };
	}

	async activateSequence(id: string): Promise<{ success: boolean }> {
		const sequence = catalogSequenceEnvelope(
			await this.request<unknown>(
				'POST',
				`/admin/email/sequences/${checkedSequenceId(id)}/activate`
			),
			id
		);
		if (sequence.status !== 'active') throw new Error('Sequence activation was not confirmed');
		return { success: true };
	}

	async pauseSequence(id: string): Promise<{ success: boolean }> {
		const sequence = catalogSequenceEnvelope(
			await this.request<unknown>('POST', `/admin/email/sequences/${checkedSequenceId(id)}/pause`),
			id
		);
		if (sequence.status !== 'paused') throw new Error('Sequence pause was not confirmed');
		return { success: true };
	}

	async addSequenceEmail(
		sequenceId: string,
		data: Partial<SequenceEmail>
	): Promise<{ email: SequenceEmail }> {
		sequenceId = checkedSequenceId(sequenceId);
		return {
			email: catalogSequenceEmail(
				await this.request<unknown>('POST', `/admin/email/sequences/${sequenceId}/emails`, {
					...sequenceEmailWrite(data, sequenceId),
					status: 'active'
				}),
				sequenceId
			)
		};
	}

	async updateSequenceEmail(
		sequenceId: string,
		emailId: string,
		data: Partial<SequenceEmail>
	): Promise<{ email: SequenceEmail }> {
		sequenceId = checkedSequenceId(sequenceId);
		emailId = sequenceEmailId(emailId);
		const email = catalogSequenceEmail(
			await this.request<unknown>(
				'PUT',
				`/admin/email/sequences/${sequenceId}/emails/${emailId}`,
				sequenceEmailWrite(data, sequenceId)
			),
			sequenceId
		);
		if (email.id !== emailId) throw new Error('Sequence email update identity did not match');
		return { email };
	}

	async deleteSequenceEmail(sequenceId: string, emailId: string): Promise<{ success: boolean }> {
		sequenceId = checkedSequenceId(sequenceId);
		emailId = sequenceEmailId(emailId);
		const result: unknown = await this.request(
			'DELETE',
			`/admin/email/sequences/${sequenceId}/emails/${emailId}`
		);
		if (
			!result ||
			typeof result !== 'object' ||
			!('deleted' in result) ||
			result.deleted !== true ||
			!('id' in result) ||
			result.id !== emailId ||
			!('sequence_id' in result) ||
			result.sequence_id !== sequenceId
		)
			throw new Error('Sequence email deletion was not confirmed');
		return { success: true };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Automation Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getAutomations(params?: {
		status?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ automations: EmailAutomation[]; total: number }> {
		return this.request('GET', '/admin/email/automations', undefined, params);
	}

	async getAutomation(id: string): Promise<{ automation: EmailAutomation }> {
		return this.request('GET', `/admin/email/automations/${id}`);
	}

	async createAutomation(data: Partial<EmailAutomation>): Promise<{ automation: EmailAutomation }> {
		return this.request('POST', '/admin/email/automations', data);
	}

	async updateAutomation(
		id: string,
		data: Partial<EmailAutomation>
	): Promise<{ automation: EmailAutomation }> {
		return this.request('PUT', `/admin/email/automations/${id}`, data);
	}

	async deleteAutomation(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/automations/${id}`);
	}

	async activateAutomation(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/automations/${id}/activate`);
	}

	async pauseAutomation(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/automations/${id}/pause`);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Segment Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getSegments(params?: {
		page?: number;
		per_page?: number;
	}): Promise<{ segments: EmailSegment[]; total: number }> {
		return this.request('GET', '/admin/email/segments', undefined, params);
	}

	async getSegment(id: string): Promise<{ segment: EmailSegment }> {
		return this.request('GET', `/admin/email/segments/${id}`);
	}

	async createSegment(data: Partial<EmailSegment>): Promise<{ segment: EmailSegment }> {
		return this.request('POST', '/admin/email/segments', data);
	}

	async updateSegment(id: string, data: Partial<EmailSegment>): Promise<{ segment: EmailSegment }> {
		return this.request('PUT', `/admin/email/segments/${id}`, data);
	}

	async deleteSegment(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/segments/${id}`);
	}

	async getSegmentSubscribers(
		id: string,
		params?: {
			page?: number;
			per_page?: number;
		}
	): Promise<{ subscribers: EmailSubscriber[]; total: number }> {
		return this.request('GET', `/admin/email/segments/${id}/subscribers`, undefined, params);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Subscriber Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getSubscribers(params?: {
		status?: string;
		search?: string;
		tags?: string[];
		page?: number;
		per_page?: number;
	}): Promise<{ subscribers: EmailSubscriber[]; total: number }> {
		return this.request('GET', '/admin/email/subscribers', undefined, params);
	}

	async getSubscriber(id: string): Promise<{ subscriber: EmailSubscriber }> {
		return this.request('GET', `/admin/email/subscribers/${id}`);
	}

	async createSubscriber(
		data: SubscriberWrite
	): Promise<{ subscriber: EmailSubscriber; confirmation: ConfirmationReceipt }> {
		return this.request('POST', '/admin/email/subscribers', data);
	}

	async updateSubscriber(
		id: string,
		data: SubscriberWrite
	): Promise<{ subscriber: EmailSubscriber }> {
		return this.request('PUT', `/admin/email/subscribers/${id}`, data);
	}

	async getSubscriberStats(): Promise<SubscriberStats> {
		return this.request('GET', '/admin/email/subscribers/stats');
	}
	async bulkDeleteSubscribers(ids: string[]): Promise<{ success: boolean; deleted: number }> {
		return this.request('POST', '/admin/email/subscribers/bulk-delete', { ids });
	}

	async deleteSubscriber(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/subscribers/${id}`);
	}

	async addSubscriberTags(id: string, tags: string[]): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/subscribers/${id}/tags`, { tags });
	}

	async removeSubscriberTags(id: string, tags: string[]): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/subscribers/${id}/tags`, { tags });
	}

	async unsubscribe(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/subscribers/${id}/unsubscribe`);
	}

	async resubscribe(
		id: string
	): Promise<{ success: boolean; subscriber: EmailSubscriber; confirmation: ConfirmationReceipt }> {
		return this.request('POST', `/admin/email/subscribers/${id}/resubscribe`);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Analytics
	// ═══════════════════════════════════════════════════════════════════════════

	async getAnalytics(period: string = '30d'): Promise<{ analytics: EmailAnalytics }> {
		return {
			analytics: emailAnalytics(
				await this.request<unknown>('GET', '/admin/email/analytics', undefined, { period })
			)
		};
	}

	async getCampaignStats(id: string): Promise<{ stats: RecordedCampaignEmailStats }> {
		const numeric = campaignId(id);
		return {
			stats: emailCampaignReportStats(
				await this.request<unknown>('GET', `/admin/email/campaigns/${numeric}/report`),
				numeric
			)
		};
	}

	async getSequenceStats(id: string): Promise<{ stats: SequenceStats }> {
		return {
			stats: catalogSequenceStats(
				await this.request<unknown>('GET', `/admin/email/sequences/${checkedSequenceId(id)}/stats`),
				id
			)
		};
	}

	async getAutomationStats(id: string): Promise<{ stats: AutomationStats }> {
		return this.request('GET', `/admin/email/automations/${id}/stats`);
	}

	async getEvents(params?: {
		campaign_id?: string;
		subscriber_id?: string;
		type?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ events: EmailEvent[]; total: number }> {
		return this.request('GET', '/admin/email/events', undefined, params);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Deliverability
	// ═══════════════════════════════════════════════════════════════════════════

	async getDeliverabilityReport(): Promise<{ report: DeliverabilityReport }> {
		return this.request('GET', '/admin/email/deliverability');
	}

	async checkDomainAuth(domain: string): Promise<{
		spf: boolean;
		dkim: boolean;
		dmarc: boolean;
		recommendations: string[];
	}> {
		return this.request('GET', '/admin/email/deliverability/check', undefined, { domain });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// AI Features
	// ═══════════════════════════════════════════════════════════════════════════

	async generateEmailContent(
		prompt: string,
		/**
		 * Optional context bag forwarded to the AI generator (audience,
		 * tone, brand voice, etc.). Opaque JSON because the backend's
		 * prompt-engineering layer owns the schema.
		 */
		context?: JsonValue
	): Promise<{
		subject: string;
		html_content: string;
		plain_text_content: string;
	}> {
		return this.request('POST', '/admin/email/ai/generate', { prompt, context });
	}

	async optimizeSubjectLine(subject: string): Promise<{
		suggestions: string[];
		scores: number[];
	}> {
		return this.request('POST', '/admin/email/ai/optimize-subject', { subject });
	}

	async predictSendTime(subscriberId: string): Promise<{
		recommended_time: string;
		confidence: number;
	}> {
		return this.request('GET', `/admin/email/ai/predict-send-time/${subscriberId}`);
	}
}

export const emailApi = new EmailAPI();
