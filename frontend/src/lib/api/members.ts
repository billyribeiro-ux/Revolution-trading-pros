/**
 * Enterprise Members API Service
 * Revolution Trading Pros - L8+ Google Enterprise Grade
 */

import { apiClient } from './client.svelte';
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
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBER MANAGEMENT API - ICT 11+ Enterprise Grade
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get member full details (including subscriptions, orders, activity, notes)
	 */
	async getMemberFull(id: number): Promise<MemberFullDetails> {
		return apiClient.get(`/admin/member-management/${id}`);
	},

	/**
	 * Create new member
	 */
	async createMember(data: CreateMemberRequest): Promise<{ message: string; member: Member; temporary_password?: string }> {
		return apiClient.post('/admin/member-management', data);
	},

	/**
	 * Update member
	 */
	async updateMember(id: number, data: UpdateMemberRequest): Promise<{ message: string; member: Member }> {
		return apiClient.put(`/admin/member-management/${id}`, data);
	},

	/**
	 * Delete member (soft delete)
	 */
	async deleteMember(id: number): Promise<{ message: string }> {
		return apiClient.delete(`/admin/member-management/${id}`);
	},

	/**
	 * Ban member
	 */
	async banMember(id: number, data?: { reason?: string; duration_days?: number }): Promise<{ message: string; banned_until?: string }> {
		return apiClient.post(`/admin/member-management/${id}/ban`, data || {});
	},

	/**
	 * Suspend member
	 */
	async suspendMember(id: number, data?: { reason?: string; duration_days?: number }): Promise<{ message: string; suspended_until?: string }> {
		return apiClient.post(`/admin/member-management/${id}/suspend`, data || {});
	},

	/**
	 * Unban/unsuspend member
	 */
	async unbanMember(id: number): Promise<{ message: string }> {
		return apiClient.post(`/admin/member-management/${id}/unban`, {});
	},

	/**
	 * Get member notes
	 */
	async getMemberNotes(id: number): Promise<MemberNote[]> {
		return apiClient.get(`/admin/member-management/${id}/notes`);
	},

	/**
	 * Create member note
	 */
	async createMemberNote(id: number, content: string): Promise<{ message: string; note: MemberNote }> {
		return apiClient.post(`/admin/member-management/${id}/notes`, { content });
	},

	/**
	 * Delete member note
	 */
	async deleteMemberNote(memberId: number, noteId: number): Promise<{ message: string }> {
		return apiClient.delete(`/admin/member-management/${memberId}/notes/${noteId}`);
	},

	/**
	 * Get member activity log
	 */
	async getMemberActivity(id: number, params?: { page?: number; per_page?: number; action?: string }): Promise<{
		activity: MemberActivity[];
		pagination: PaginationInfo;
	}> {
		const searchParams = new URLSearchParams();
		if (params?.page) searchParams.append('page', String(params.page));
		if (params?.per_page) searchParams.append('per_page', String(params.per_page));
		if (params?.action) searchParams.append('action', params.action);
		const queryString = searchParams.toString();
		return apiClient.get(`/admin/member-management/${id}/activity${queryString ? `?${queryString}` : ''}`);
	},

	/**
	 * Export members to Excel/PDF
	 */
	async exportMembersAdvanced(filters?: { format?: 'csv' | 'xlsx' | 'pdf'; status?: string; date_from?: string; date_to?: string }): Promise<Blob> {
		const params = new URLSearchParams();
		if (filters?.format) params.append('format', filters.format);
		if (filters?.status) params.append('status', filters.status);
		if (filters?.date_from) params.append('date_from', filters.date_from);
		if (filters?.date_to) params.append('date_to', filters.date_to);
		const queryString = params.toString();
		const token = getAuthToken();
		const response = await fetch(
			`/api/admin/member-management/export${queryString ? `?${queryString}` : ''}`,
			{
				headers: {
					Authorization: `Bearer ${token}`
				}
			}
		);
		return response.blob();
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// ADDITIONAL TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface MemberFullDetailsMember {
	id: number;
	name: string;
	first_name: string | null;
	last_name: string | null;
	email: string;
	avatar: string | null;
	status: 'active' | 'trial' | 'churned' | 'never_subscribed' | 'banned' | 'suspended' | 'restricted';
	status_label: string;
	joined_at: string;
	total_spent: number;
	active_subscriptions_count: number;
	current_plan: string | null;
	current_product: string | null;
	subscriptions?: Subscription[];
	orders?: Order[];
	email_verified?: boolean;
	last_login?: string | null;
	churned_at?: string;
	last_product?: string;
	last_plan?: string;
	churn_reason?: string | null;
	days_since_churn?: number | null;
	tags: string[];
	banned_until?: string | null;
	ban_reason?: string | null;
}

export interface MemberFullDetails {
	member: MemberFullDetailsMember;
	subscriptions: MemberSubscriptionDetail[];
	orders: MemberOrderDetail[];
	activity: MemberActivity[];
	notes: MemberNote[];
	stats: {
		total_spent: number;
		active_subscriptions: number;
		total_orders: number;
		member_since_days: number;
	};
	engagement_score: number;
	timeline: TimelineEvent[];
}

export interface MemberSubscriptionDetail {
	id: number;
	product_name: string | null;
	status: string;
	price: number | null;
	billing_period: string | null;
	started_at: string | null;
	expires_at: string | null;
	cancelled_at: string | null;
	created_at: string;
}

export interface MemberOrderDetail {
	id: number;
	order_number: string | null;
	total: number | null;
	status: string | null;
	created_at: string;
}

export interface MemberActivity {
	id: number;
	user_id: number;
	action: string;
	description: string | null;
	metadata: Record<string, unknown> | null;
	ip_address: string | null;
	user_agent: string | null;
	created_at: string;
}

export interface MemberNote {
	id: number;
	user_id: number;
	content: string;
	created_by: number | null;
	created_by_name: string | null;
	created_at: string;
}

export interface TimelineEvent {
	type: string;
	title: string;
	date: string;
	icon: string;
	meta?: Record<string, unknown>;
}

export interface CreateMemberRequest {
	name: string;
	email: string;
	password?: string;
	role?: string;
	send_welcome_email?: boolean;
}

export interface UpdateMemberRequest {
	name?: string;
	email?: string;
	role?: string;
	avatar_url?: string;
	password?: string;
}

export default membersApi;
