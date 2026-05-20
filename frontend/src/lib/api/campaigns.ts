/**
 * Email Campaigns API Service - Google L11+ Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete email campaign management with:
 * - Full CRUD operations
 * - A/B testing
 * - Scheduling
 * - Analytics
 * - Real-time updates
 *
 * @version 1.0.0
 *
 * ⚠️ FIX-2026-04-26: ORPHAN — no matching backend route exists.
 * This client calls `/api/admin/email/campaigns/*` (~10 endpoints) but neither
 * the SvelteKit proxy at frontend/src/routes/api/admin/email/campaigns/ nor a
 * Rust handler under api/src/routes/ exist. Every call from this file 404/400s.
 *
 * Decision required: either build the matching backend (Rust handlers + proxies)
 * or remove the campaigns page (frontend/src/routes/admin/email/campaigns/+page.svelte).
 * Until then, the campaigns page emits "Failed to load campaigns" toasts on every
 * mount.
 *
 * See docs/audits/AUDIT_REPORT.md §10 Finding 2 + docs/audits/ADMIN_DASHBOARD_REPORT.md (orphan API clients).
 */

import { get as _get } from 'svelte/store';
import { authStore } from '$lib/stores/auth.svelte';
import type { PaginationMeta, QueryParams } from './_types';

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
	template_id: number;
	segment_id: number | null;
	from_name: string;
	from_email: string;
	reply_to: string | null;
	scheduled_at: string | null;
	sent_at: string | null;
	status: 'draft' | 'scheduled' | 'sending' | 'sent';
	total_recipients: number;
	sent_count: number;
	opened_count: number;
	clicked_count: number;
	bounced_count: number;
	unsubscribed_count: number;
	ab_test_config: ABTestConfig | null;
	created_by: number;
	created_at: string;
	updated_at: string;
	// Calculated
	open_rate?: number;
	click_rate?: number;
	bounce_rate?: number;
	unsubscribe_rate?: number;
	progress_percentage?: number;
	// Relations
	template?: { id: number; name: string };
	creator?: { id: number; name: string };
}

export interface ABTestConfig {
	enabled: boolean;
	subject_b?: string;
	split_percentage?: number;
	winner?: 'a' | 'b' | null;
	variant_a?: { subject: string; open_rate: number };
	variant_b?: { subject: string; open_rate: number };
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
	avg_open_rate: number;
	avg_click_rate: number;
	avg_bounce_rate: number;
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
			const errorBody: unknown = await response.json().catch(() => null);
			const message =
				typeof errorBody === 'object' &&
				errorBody !== null &&
				'message' in errorBody &&
				typeof (errorBody as { message: unknown }).message === 'string'
					? (errorBody as { message: string }).message
					: `HTTP ${response.status}`;
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
		// NB: The orphaned backend (see file header — no Rust handler exists yet)
		// will eventually need to choose between Laravel-style `last_page` and the
		// canonical `total_pages` emitted by every other paginated handler in this
		// stack (`PaginationMeta` in `./_types.ts`). The public signature here
		// keeps `last_page` to preserve the existing wire contract this client
		// was built against. The internal response shape is widened to
		// `PaginationMeta` (which contains `total` + optional fields); callers
		// reading `last_page` are responsible for narrowing.
		const response = await this.request<{
			success: boolean;
			data: Campaign[];
			meta: PaginationMeta & { last_page?: number; current_page?: number };
		}>('GET', '/admin/email/campaigns', undefined, filters as QueryParams | undefined);
		return {
			data: response.data,
			meta: {
				total: response.meta.total,
				current_page: response.meta.current_page ?? 1,
				last_page: response.meta.last_page ?? 1
			}
		};
	}

	async getCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'GET',
			`/admin/email/campaigns/${id}`
		);
		return response.data;
	}

	async createCampaign(data: CreateCampaignData): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			'/admin/email/campaigns',
			data
		);
		return response.data;
	}

	async updateCampaign(id: number, data: Partial<CreateCampaignData>): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'PUT',
			`/admin/email/campaigns/${id}`,
			data
		);
		return response.data;
	}

	async deleteCampaign(id: number): Promise<void> {
		await this.request('DELETE', `/admin/email/campaigns/${id}`);
	}

	async duplicateCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/duplicate`
		);
		return response.data;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Campaign Actions
	// ═══════════════════════════════════════════════════════════════════════

	async sendCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/send`
		);
		return response.data;
	}

	async scheduleCampaign(id: number, scheduledAt: string): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/schedule`,
			{ scheduled_at: scheduledAt }
		);
		return response.data;
	}

	async cancelCampaign(id: number): Promise<Campaign> {
		const response = await this.request<{ success: boolean; data: Campaign }>(
			'POST',
			`/admin/email/campaigns/${id}/cancel`
		);
		return response.data;
	}

	async sendTestEmail(id: number, email: string): Promise<void> {
		await this.request('POST', `/admin/email/campaigns/${id}/test`, { email });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Stats & Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getStats(): Promise<CampaignStats> {
		const response = await this.request<{ success: boolean; data: CampaignStats }>(
			'GET',
			'/admin/email/campaigns/stats'
		);
		return response.data;
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
