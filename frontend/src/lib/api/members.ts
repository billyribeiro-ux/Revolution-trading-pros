/**
 * Enterprise Members API Service
 * Revolution Trading Pros - L8+ Google Enterprise Grade
 */

import { apiClient } from './client';
import { getAuthToken } from '$lib/stores/auth.svelte';

// Types
export interface Member {
	id: number;
	name: string;
	first_name: string | null;
	last_name: string | null;
	email: string;
	avatar: string | null;
	status: 'active' | 'trial' | 'churned' | 'never_subscribed';
	status_label: string;
	joined_at: string;
	total_spent: number;
	active_subscriptions_count: number;
	current_plan: string | null;
	current_product: string | null;
	// Extended fields for detailed view
	subscriptions?: Subscription[];
	orders?: Order[];
	email_verified?: boolean;
	last_login?: string | null;
	// Churned member fields
	churned_at?: string;
	last_product?: string;
	last_plan?: string;
	churn_reason?: string | null;
	days_since_churn?: number | null;
	// Tags
	tags?: string[];
}

export interface Subscription {
	id: number;
	plan: string | null;
	product: string | null;
	status: string;
	price: number;
	interval: string;
	start_date: string | null;
	next_payment: string | null;
	cancelled_at: string | null;
	total_paid: number;
}

export interface Order {
	id: number;
	number: string;
	total: number;
	status: string;
	created_at: string;
}

export interface Service {
	id: number;
	name: string;
	type: string;
	price: number;
	is_active: boolean;
	members_count: number;
}

export interface MemberStats {
	overview: {
		total_members: number;
		new_this_month: number;
		new_last_month: number;
		growth_rate: number;
	};
	subscriptions: {
		active: number;
		trial: number;
		churned: number;
		churn_rate: number;
	};
	revenue: {
		mrr: number;
		total: number;
		avg_ltv: number;
	};
	top_services: {
		id: number;
		name: string;
		type: string;
		members_count: number;
	}[];
	growth_trend: {
		month: string;
		members: number;
		new: number;
	}[];
}

export interface ChurnedStats {
	total_churned: number;
	churned_this_month: number;
	potential_recovery_revenue: number;
	top_churn_reasons: { reason: string; count: number }[];
}

export interface EmailTemplate {
	id: string | number;
	name: string;
	subject: string;
	category: string;
	is_active: boolean;
	is_preset?: boolean;
	body?: string;
}

export interface MemberFilters {
	search?: string;
	status?: 'active' | 'trial' | 'churned' | 'never_subscribed';
	product_id?: number;
	plan_id?: number;
	date_from?: string;
	date_to?: string;
	spending_tier?: 'whale' | 'high' | 'medium' | 'low';
	sort_by?: string;
	sort_dir?: 'asc' | 'desc';
	per_page?: number;
	page?: number;
}

export interface ChurnedFilters {
	search?: string;
	churn_reason?: string;
	churned_within_days?: number;
	winback_potential?: 'high' | 'medium' | 'low';
	per_page?: number;
	page?: number;
}

export interface PaginationInfo {
	total: number;
	per_page: number;
	current_page: number;
	last_page: number;
}

// API Service
export const membersApi = {
	/**
	 * Get all members with filtering and pagination
	 */
	async getMembers(
		filters?: MemberFilters
	): Promise<{ members: Member[]; pagination: PaginationInfo }> {
		const params = new URLSearchParams();
		if (filters) {
			Object.entries(filters).forEach(([key, value]) => {
				if (value !== undefined && value !== null && value !== '') {
					params.append(key, String(value));
				}
			});
		}
		const queryString = params.toString();
		const endpoint = `/admin/members${queryString ? `?${queryString}` : ''}`;
		return apiClient.get(endpoint, { cache: { ttl: 60000 } });
	},

	/**
	 * Get member statistics
	 */
	async getStats(): Promise<MemberStats> {
		return apiClient.get('/admin/members/stats', { cache: { ttl: 300000 } });
	},

	/**
	 * Get all services for filtering
	 */
	async getServices(): Promise<{ services: Service[] }> {
		return apiClient.get('/admin/members/services', { cache: { ttl: 300000 } });
	},

	/**
	 * Get members by service/product
	 */
	async getMembersByService(
		serviceId: number,
		filters?: { status?: string; search?: string; per_page?: number; page?: number }
	): Promise<{
		service: { id: number; name: string; type: string };
		stats: {
			total_members: number;
			active_members: number;
			trial_members: number;
			churned_members: number;
			total_revenue: number;
		};
		members: Member[];
		pagination: PaginationInfo;
	}> {
		const params = new URLSearchParams();
		if (filters) {
			Object.entries(filters).forEach(([key, value]) => {
				if (value !== undefined && value !== null && value !== '') {
					params.append(key, String(value));
				}
			});
		}
		const queryString = params.toString();
		return apiClient.get(
			`/admin/members/service/${serviceId}${queryString ? `?${queryString}` : ''}`
		);
	},

	/**
	 * Get churned/past members
	 */
	async getChurnedMembers(
		filters?: ChurnedFilters
	): Promise<{ stats: ChurnedStats; members: Member[]; pagination: PaginationInfo }> {
		const params = new URLSearchParams();
		if (filters) {
			Object.entries(filters).forEach(([key, value]) => {
				if (value !== undefined && value !== null && value !== '') {
					params.append(key, String(value));
				}
			});
		}
		const queryString = params.toString();
		return apiClient.get(`/admin/members/churned${queryString ? `?${queryString}` : ''}`);
	},

	/**
	 * Get single member details
	 */
	async getMember(id: number): Promise<{
		member: Member;
		engagement_score: number;
		timeline: {
			type: string;
			title: string;
			date: string;
			icon: string;
			meta?: Record<string, unknown>;
		}[];
	}> {
		return apiClient.get(`/admin/members/${id}`);
	},

	/**
	 * Get email templates for campaigns
	 */
	async getEmailTemplates(): Promise<{
		templates: EmailTemplate[];
		preset_templates: EmailTemplate[];
	}> {
		return apiClient.get('/admin/members/email-templates', { cache: { ttl: 300000 } });
	},

	/**
	 * Send email to single member
	 */
	async sendEmail(
		memberId: number,
		data: {
			template_id?: string | number;
			subject: string;
			body: string;
			campaign_type?: 'winback' | 'promo' | 'general' | 'reminder';
		}
	): Promise<{ success: boolean; message: string }> {
		return apiClient.post(`/admin/members/${memberId}/send-email`, data);
	},

	/**
	 * Send bulk email to multiple members
	 */
	async sendBulkEmail(data: {
		member_ids: number[];
		template_id?: string | number;
		subject: string;
		body: string;
		campaign_type?: 'winback' | 'promo' | 'general' | 'reminder' | 'free_trial';
		personalize?: boolean;
	}): Promise<{ success: boolean; message: string; sent: number; failed: number }> {
		return apiClient.post('/admin/members/bulk-email', data);
	},

	/**
	 * Export members to CSV
	 */
	async exportMembers(filters?: { status?: string }): Promise<Blob> {
		const params = new URLSearchParams();
		if (filters?.status) {
			params.append('status', filters.status);
		}
		const queryString = params.toString();
		// Use secure auth store token (memory-only, not localStorage)
		const token = getAuthToken();
		const response = await fetch(
			`/api/admin/members/export${queryString ? `?${queryString}` : ''}`,
			{
				headers: {
					Authorization: `Bearer ${token}`
				}
			}
		);
		return response.blob();
	}
};

export default membersApi;
