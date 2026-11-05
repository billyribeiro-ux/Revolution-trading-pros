/** Same-origin email campaign authoring and delivery API. */

import { get as _get } from 'svelte/store';
import { authStore } from '#lib/stores/auth.svelte.js';
import type { QueryParams } from './_types';
import {
	campaignEnvelope,
	campaignPage,
	campaignStats,
	campaignDeleted
} from './campaign-contract.js';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
const API_BASE = '/api';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface Campaign {
	id: number;
	name: string;
	subject: string;
	template_id: number | null;
	segment_id: number | null;
	from_name: string;
	from_email: string;
	reply_to: string | null;
	scheduled_at: string | null;
	planned_at?: string | null;
	sent_at: string | null;
	status:
		| 'draft'
		| 'scheduled'
		| 'sending'
		| 'sent'
		| 'paused'
		| 'failed'
		| 'cancelled'
		| 'archived';
	total_recipients: number;
	sent_count: number;
	opened_count: number;
	clicked_count: number;
	bounced_count: number;
	unsubscribed_count: number;
	ab_test_config: ABTestConfig | null;
	created_by: number | null;
	created_at: string;
	updated_at: string;
	// Calculated
	open_rate?: number | null;
	click_rate?: number | null;
	bounce_rate?: number | null;
	unsubscribe_rate?: number | null;
	progress_percentage?: number;
	// Relations
	template?: { id: number; name: string } | null;
	creator?: { id: number; name: string } | null;
}

export interface ABTestConfig {
	enabled: boolean;
	subject_b?: string;
	split_percentage?: number;
	winner?: 'a' | 'b' | null;
	variant_a?: { subject: string; open_rate: number | null };
	variant_b?: { subject: string; open_rate: number | null };
}

export interface CampaignStats {
	total_campaigns: number;
	total_sent: number;
	total_opened: number;
	total_clicked: number;
	total_bounced: number;
	total_unsubscribed: number;
	drafts: number;
	scheduled: number;
	sending: number;
	sent: number;
	avg_open_rate: number | null;
	avg_click_rate: number | null;
	avg_bounce_rate: number | null;
	recent: {
		campaigns: number;
		sent: number;
		opened: number;
		clicked: number;
	};
}

export interface CampaignAnalytics {
	campaign: {
		id: number;
		name: string;
		sent_count: number;
		opened_count: number;
		clicked_count: number;
		bounced_count: number;
		unsubscribed_count: number;
		open_rate: number;
		click_rate: number;
		bounce_rate: number;
	};
	hourly_stats: Array<{
		date: string;
		hour: number;
		event_type: string;
		count: number;
	}>;
	device_stats: Array<{
		device_type: string;
		count: number;
	}>;
	top_links: Array<{
		link_url: string;
		clicks: number;
	}>;
	geo_stats: Array<{
		country: string;
		count: number;
	}>;
}

export interface CreateCampaignData {
	name: string;
	subject: string;
	template_id: number;
	segment_id?: number | null;
	from_name?: string;
	from_email?: string;
	reply_to?: string;
	scheduled_at?: string | null;
	ab_test_config?: ABTestConfig | null;
}

export interface CampaignFilters {
	status?: string;
	search?: string;
	from_date?: string;
	to_date?: string;
	page?: number;
	per_page?: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Client
// ═══════════════════════════════════════════════════════════════════════════

class CampaignsApiClient {
	private getAuthHeaders(): Record<string, string> {
		// Use secure getter from auth store
		const token = authStore.getToken();
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		return headers;
	}

	private async request<T>(
		method: string,
		endpoint: string,
		body?: unknown,
		params?: QueryParams
	): Promise<T> {
		let url = `${API_BASE}${endpoint}`;

		if (params) {
			// Matches the R12-A precedent in `config.ts` / `bing-seo.ts`:
			// repeated-key serialisation for arrays so `{ tags: ['a','b'] }`
			// emits `?tags=a&tags=b` (the backend shape) rather than `?tags=a%2Cb`.
			const searchParams = new URLSearchParams();
			Object.entries(params).forEach(([key, value]) => {
				if (value === undefined || value === null || value === '') return;
				if (Array.isArray(value)) {
					for (const item of value) searchParams.append(key, String(item));
				} else {
					searchParams.append(key, String(value));
				}
			});
			const queryString = searchParams.toString();
			if (queryString) {
				url += `?${queryString}`;
			}
		}

		const fetchOptions: RequestInit = {
			method,
			headers: this.getAuthHeaders(),
			...(body !== undefined && { body: JSON.stringify(body) })
		};

		const response = await fetch(url, fetchOptions);

		if (!response.ok) {
			const raw = await response.text();
			let payload: unknown = null;
			try {
				payload = JSON.parse(raw);
			} catch {
				/* Plain validation responses are supported below. */
			}
			let message = `HTTP ${response.status}`;
			if (payload && typeof payload === 'object') {
				if ('message' in payload && typeof payload.message === 'string') message = payload.message;
				else if ('error' in payload && typeof payload.error === 'string') message = payload.error;
			} else if (response.headers.get('Content-Type')?.includes('text/plain') && raw.trim())
				message = raw.slice(0, 1000);
			throw new Error(message);
		}

		return response.json();
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Campaign CRUD
	// ═══════════════════════════════════════════════════════════════════════

	async getCampaigns(filters?: CampaignFilters): Promise<{
		data: Campaign[];
		meta: { total: number; current_page: number; last_page: number };
	}> {
		return campaignPage(
			await this.request<unknown>(
				'GET',
				'/admin/email/campaigns',
				undefined,
				filters as QueryParams | undefined
			)
		);
	}

	async getCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'GET',
			`/admin/email/campaigns/${id}`
		);
		return campaignEnvelope(response, id);
	}

	async createCampaign(data: CreateCampaignData): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			'/admin/email/campaigns',
			data
		);
		return campaignEnvelope(response);
	}

	async updateCampaign(id: number, data: Partial<CreateCampaignData>): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'PUT',
			`/admin/email/campaigns/${id}`,
			data
		);
		return campaignEnvelope(response, id);
	}

	async deleteCampaign(id: number): Promise<void> {
		campaignDeleted(await this.request<unknown>('DELETE', `/admin/email/campaigns/${id}`), id);
	}

	async duplicateCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/duplicate`
		);
		const clone = campaignEnvelope(response);
		if (clone.id === id) throw new Error('Campaign duplication did not create a new record');
		return clone;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Campaign Actions
	// ═══════════════════════════════════════════════════════════════════════

	async sendCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/send`
		);
		return campaignEnvelope(response, id);
	}

	async scheduleCampaign(id: number, scheduledAt: string): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/schedule`,
			{ scheduled_at: scheduledAt }
		);
		return campaignEnvelope(response, id);
	}

	async cancelCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/cancel`
		);
		return campaignEnvelope(response, id);
	}

	async sendTestEmail(id: number, email: string): Promise<void> {
		await this.request('POST', `/admin/email/campaigns/${id}/test`, { email });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Stats & Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getStats(): Promise<CampaignStats> {
		return campaignStats(await this.request<unknown>('GET', '/admin/email/campaigns/stats'));
	}

	async getAnalytics(id: number): Promise<CampaignAnalytics> {
		const response = await this.request<{ success: boolean; data: CampaignAnalytics }>(
			'GET',
			`/admin/email/campaigns/${id}/analytics`
		);
		return response.data;
	}

	async getPreview(
		id: number
	): Promise<{ subject: string; from_name: string; from_email: string; html: string }> {
		const response = await this.request<{
			success: boolean;
			data: { subject: string; from_name: string; from_email: string; html: string };
		}>('GET', `/admin/email/campaigns/${id}/preview`);
		return response.data;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Export singleton instance
// ═══════════════════════════════════════════════════════════════════════════

export const campaignsApi = new CampaignsApiClient();

// Export convenience functions
export const getCampaigns = (filters?: CampaignFilters) => campaignsApi.getCampaigns(filters);
export const getCampaign = (id: number) => campaignsApi.getCampaign(id);
export const createCampaign = (data: CreateCampaignData) => campaignsApi.createCampaign(data);
export const updateCampaign = (id: number, data: Partial<CreateCampaignData>) =>
	campaignsApi.updateCampaign(id, data);
export const deleteCampaign = (id: number) => campaignsApi.deleteCampaign(id);
export const duplicateCampaign = (id: number) => campaignsApi.duplicateCampaign(id);
export const sendCampaign = (id: number) => campaignsApi.sendCampaign(id);
export const scheduleCampaign = (id: number, scheduledAt: string) =>
	campaignsApi.scheduleCampaign(id, scheduledAt);
export const cancelCampaign = (id: number) => campaignsApi.cancelCampaign(id);
export const sendTestEmail = (id: number, email: string) => campaignsApi.sendTestEmail(id, email);
export const getCampaignStats = () => campaignsApi.getStats();
export const getCampaignAnalytics = (id: number) => campaignsApi.getAnalytics(id);
export const getCampaignPreview = (id: number) => campaignsApi.getPreview(id);
