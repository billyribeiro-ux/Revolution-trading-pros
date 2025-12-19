/**
 * Email Marketing & Automation API Client
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise email marketing system that surpasses ActiveCampaign, Klaviyo,
 * Mailchimp, ConvertKit, and HubSpot.
 *
 * @version 1.0.0
 */

import { get } from 'svelte/store';
import { authStore } from '$lib/stores/auth';

// Production fallback - NEVER use localhost in production
const PROD_API = 'https://revolution-backend.fly.dev/api';
const API_BASE = import.meta.env.VITE_API_BASE_URL || PROD_API;

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
	reply_to?: string;
	template_id?: string;
	html_content: string;
	plain_text_content?: string;
	status: 'draft' | 'scheduled' | 'sending' | 'sent' | 'paused' | 'cancelled';
	type: 'broadcast' | 'automation' | 'sequence' | 'transactional';
	segment_id?: string;
	send_at?: string;
	sent_at?: string;
	stats?: EmailStats;
	ab_test?: ABTest;
	tags: string[];
	created_by: number;
	created_at: string;
	updated_at: string;
}

export interface EmailTemplate {
	id: string;
	name: string;
	description?: string;
	category: string;
	thumbnail?: string;
	html_content: string;
	plain_text_content?: string;
	design_json?: any; // For builder state
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
	trigger_config?: any;
	status: 'active' | 'paused' | 'draft';
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
	value?: any;
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
	config: any;
}

export interface AutomationCondition {
	field: string;
	operator: 'equals' | 'not_equals' | 'contains' | 'greater_than' | 'less_than';
	value: any;
}

export interface AutomationAction {
	type: 'send_email' | 'add_tag' | 'remove_tag' | 'update_field' | 'wait' | 'webhook';
	config: any;
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
	value: any;
}

export interface EmailSubscriber {
	id: string;
	email: string;
	first_name?: string;
	last_name?: string;
	status: 'subscribed' | 'unsubscribed' | 'bounced' | 'complained';
	tags: string[];
	custom_fields: Record<string, any>;
	email_score: number;
	last_opened_at?: string;
	last_clicked_at?: string;
	subscribed_at: string;
	unsubscribed_at?: string;
	created_at: string;
	updated_at: string;
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

export interface SequenceStats extends EmailStats {
	active_subscribers: number;
	completed_subscribers: number;
	completion_rate: number;
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
	metadata?: any;
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
	period: string;
	campaigns: number;
	emails_sent: number;
	total_opens: number;
	total_clicks: number;
	average_open_rate: number;
	average_click_rate: number;
	top_campaigns: Array<{ id: string; name: string; open_rate: number }>;
	engagement_trend: Array<{ date: string; opens: number; clicks: number }>;
	device_breakdown: { desktop: number; mobile: number; tablet: number };
	client_breakdown: Record<string, number>;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Client Class
// ═══════════════════════════════════════════════════════════════════════════

class EmailAPI {
	private async request<T>(
		method: string,
		endpoint: string,
		data?: any,
		params?: Record<string, any>
	): Promise<T> {
		// Use secure getter from auth store
		const token = authStore.getToken();

		const url = new URL(`${API_BASE}${endpoint}`);
		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					url.searchParams.append(key, String(value));
				}
			});
		}

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(url.toString(), {
			method,
			headers,
			body: data ? JSON.stringify(data) : undefined,
			credentials: 'include'
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			throw new Error(error.message || `HTTP ${response.status}`);
		}

		return response.json();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Campaign Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getCampaigns(params?: {
		status?: string;
		type?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ campaigns: EmailCampaign[]; total: number; page: number; per_page: number }> {
		return this.request('GET', '/admin/email/campaigns', undefined, params);
	}

	async getCampaign(id: string): Promise<{ campaign: EmailCampaign }> {
		return this.request('GET', `/admin/email/campaigns/${id}`);
	}

	async createCampaign(data: Partial<EmailCampaign>): Promise<{ campaign: EmailCampaign }> {
		return this.request('POST', '/admin/email/campaigns', data);
	}

	async updateCampaign(
		id: string,
		data: Partial<EmailCampaign>
	): Promise<{ campaign: EmailCampaign }> {
		return this.request('PUT', `/admin/email/campaigns/${id}`, data);
	}

	async deleteCampaign(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/campaigns/${id}`);
	}

	async sendCampaign(id: string, options?: { send_at?: string }): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/campaigns/${id}/send`, options);
	}

	async pauseCampaign(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/campaigns/${id}/pause`);
	}

	async resumeCampaign(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/campaigns/${id}/resume`);
	}

	async duplicateCampaign(id: string): Promise<{ campaign: EmailCampaign }> {
		return this.request('POST', `/admin/email/campaigns/${id}/duplicate`);
	}

	async sendTestEmail(id: string, emails: string[]): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/campaigns/${id}/test`, { emails });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Template Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getTemplates(params?: {
		category?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ templates: EmailTemplate[]; total: number }> {
		return this.request('GET', '/admin/email/templates', undefined, params);
	}

	async getTemplate(id: string): Promise<{ template: EmailTemplate }> {
		return this.request('GET', `/admin/email/templates/${id}`);
	}

	async createTemplate(data: Partial<EmailTemplate>): Promise<{ template: EmailTemplate }> {
		return this.request('POST', '/admin/email/templates', data);
	}

	async updateTemplate(
		id: string,
		data: Partial<EmailTemplate>
	): Promise<{ template: EmailTemplate }> {
		return this.request('PUT', `/admin/email/templates/${id}`, data);
	}

	async deleteTemplate(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/templates/${id}`);
	}

	async duplicateTemplate(id: string): Promise<{ template: EmailTemplate }> {
		return this.request('POST', `/admin/email/templates/${id}/duplicate`);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Sequence Management
	// ═══════════════════════════════════════════════════════════════════════════

	async getSequences(params?: {
		status?: string;
		page?: number;
		per_page?: number;
	}): Promise<{ sequences: EmailSequence[]; total: number }> {
		return this.request('GET', '/admin/email/sequences', undefined, params);
	}

	async getSequence(id: string): Promise<{ sequence: EmailSequence }> {
		return this.request('GET', `/admin/email/sequences/${id}`);
	}

	async createSequence(data: Partial<EmailSequence>): Promise<{ sequence: EmailSequence }> {
		return this.request('POST', '/admin/email/sequences', data);
	}

	async updateSequence(
		id: string,
		data: Partial<EmailSequence>
	): Promise<{ sequence: EmailSequence }> {
		return this.request('PUT', `/admin/email/sequences/${id}`, data);
	}

	async deleteSequence(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/sequences/${id}`);
	}

	async activateSequence(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/sequences/${id}/activate`);
	}

	async pauseSequence(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/sequences/${id}/pause`);
	}

	async addSequenceEmail(
		sequenceId: string,
		data: Partial<SequenceEmail>
	): Promise<{ email: SequenceEmail }> {
		return this.request('POST', `/admin/email/sequences/${sequenceId}/emails`, data);
	}

	async updateSequenceEmail(
		sequenceId: string,
		emailId: string,
		data: Partial<SequenceEmail>
	): Promise<{ email: SequenceEmail }> {
		return this.request('PUT', `/admin/email/sequences/${sequenceId}/emails/${emailId}`, data);
	}

	async deleteSequenceEmail(sequenceId: string, emailId: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/email/sequences/${sequenceId}/emails/${emailId}`);
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

	async createSubscriber(data: Partial<EmailSubscriber>): Promise<{ subscriber: EmailSubscriber }> {
		return this.request('POST', '/admin/email/subscribers', data);
	}

	async updateSubscriber(
		id: string,
		data: Partial<EmailSubscriber>
	): Promise<{ subscriber: EmailSubscriber }> {
		return this.request('PUT', `/admin/email/subscribers/${id}`, data);
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

	async resubscribe(id: string): Promise<{ success: boolean }> {
		return this.request('POST', `/admin/email/subscribers/${id}/resubscribe`);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Analytics
	// ═══════════════════════════════════════════════════════════════════════════

	async getAnalytics(period: string = '30d'): Promise<{ analytics: EmailAnalytics }> {
		return this.request('GET', '/admin/email/analytics', undefined, { period });
	}

	async getCampaignStats(id: string): Promise<{ stats: EmailStats }> {
		return this.request('GET', `/admin/email/campaigns/${id}/stats`);
	}

	async getSequenceStats(id: string): Promise<{ stats: SequenceStats }> {
		return this.request('GET', `/admin/email/sequences/${id}/stats`);
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
		context?: any
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
