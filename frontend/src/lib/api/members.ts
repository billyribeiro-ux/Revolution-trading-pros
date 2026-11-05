/**
 * Enterprise Members API Service
 * Revolution Trading Pros - L8+ Google Enterprise Grade
 */

import { apiClient } from './client.svelte';
import {
	memberProfileApi,
	type MemberProfile,
	type MemberProfileRecord,
	type MemberHistoryQuery
} from './member-profile.js';
import { getAuthToken } from '#lib/stores/auth.svelte.js';
import { downloadMemberExport } from './member-export';
import {
	readMemberFacts,
	parseMemberStats,
	parseServiceCatalog,
	parseServiceMembers,
	memberFactsQuery,
	type ServiceCatalogFilters,
	type ServiceMemberFilters
} from './member-statistics';
export type { MemberStats, Service } from './member-statistics';
import type { MemberStats } from './member-statistics';

// Types
export interface Member {
	id: number;
	name: string;
	first_name: string | null;
	last_name: string | null;
	email: string;
	avatar: string | null;
	role?: string | null;
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
	segment?: number;
	tag?: number;
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

/** Export every matching member, independently of list pagination. */
export type MemberExportFilters = Omit<MemberFilters, 'page' | 'per_page'>;

function memberExportQuery(
	filters?: MemberExportFilters & { format?: 'csv' | 'xlsx' | 'pdf' }
): string {
	const params = new URLSearchParams();
	for (const key of [
		'format',
		'segment',
		'tag',
		'search',
		'status',
		'product_id',
		'plan_id',
		'date_from',
		'date_to',
		'spending_tier',
		'sort_by',
		'sort_dir'
	] as const) {
		const value = filters?.[key];
		if (value !== undefined && value !== '') params.set(key, String(value));
	}
	return params.toString();
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
		return apiClient.get(endpoint, { cache: { enabled: false } });
	},

	/**
	 * Get member statistics
	 */
	async getStats(): Promise<MemberStats> {
		return readMemberFacts('/api/admin/members/stats', parseMemberStats);
	},

	/** Membership-plan identities, independently searched and paged. */
	async getServices(filters?: ServiceCatalogFilters) {
		const query = memberFactsQuery(filters);
		return readMemberFacts(`/api/admin/members/services${query ? `?${query}` : ''}`, (value) =>
			parseServiceCatalog(value, filters?.selected_id)
		);
	},

	/** One plan-local membership cohort; global member status remains separate. */
	async getMembersByService(serviceId: number, filters?: ServiceMemberFilters) {
		const query = memberFactsQuery(filters);
		return readMemberFacts(
			`/api/admin/members/service/${serviceId}${query ? `?${query}` : ''}`,
			(value) => parseServiceMembers(value, serviceId)
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
	async getMember(id: string | number): Promise<MemberProfile> {
		return memberProfileApi.get(id, { family: 'members' });
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
	async exportMembers(filters?: MemberExportFilters): Promise<Blob> {
		const queryString = memberExportQuery(filters);
		// Use secure auth store token (memory-only, not localStorage)
		const token = getAuthToken();
		return downloadMemberExport(
			`/api/admin/members/export${queryString ? `?${queryString}` : ''}`,
			'csv',
			token
		);
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBER MANAGEMENT API - ICT 11+ Enterprise Grade
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get member full details (including subscriptions, orders, activity, notes)
	 */
	async getMemberFull(id: string | number): Promise<MemberProfile> {
		return memberProfileApi.get(id);
	},

	/**
	 * Create new member
	 */
	async createMember(
		data: CreateMemberRequest
	): Promise<{ message: string; member: Member; temporary_password?: string }> {
		return apiClient.post('/admin/member-management', data);
	},

	/**
	 * Update member
	 */
	async updateMember(
		id: string | number,
		data: UpdateMemberRequest
	): Promise<{ message: string; member: Member }> {
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
	async banMember(
		id: number,
		data?: { reason?: string; duration_days?: number }
	): Promise<{ message: string; banned_until?: string }> {
		return apiClient.post(`/admin/member-management/${id}/ban`, data || {});
	},

	/**
	 * Suspend member
	 */
	async suspendMember(
		id: number,
		data?: { reason?: string; duration_days?: number }
	): Promise<{ message: string; suspended_until?: string }> {
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
	async getMemberNotes(id: string | number, filters: MemberHistoryQuery = {}) {
		return memberProfileApi.notes(id, filters);
	},
	async createMemberNote(id: string | number, content: string) {
		return memberProfileApi.addNote(id, content);
	},
	async deleteMemberNote(memberId: string | number, noteId: string | number) {
		return memberProfileApi.removeNote(memberId, noteId);
	},
	async getMemberActivity(id: string | number, filters: MemberHistoryQuery = {}) {
		return memberProfileApi.activity(id, filters);
	},

	/**
	 * Export members to Excel/PDF
	 */
	async exportMembersAdvanced(
		filters?: MemberExportFilters & { format?: 'csv' | 'xlsx' | 'pdf' }
	): Promise<Blob> {
		const queryString = memberExportQuery(filters);
		const token = getAuthToken();
		return downloadMemberExport(
			`/api/admin/member-management/export${queryString ? `?${queryString}` : ''}`,
			filters?.format ?? 'csv',
			token
		);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// ADDITIONAL TYPES
// ═══════════════════════════════════════════════════════════════════════════

/** Narrow edit seed: preserve recorded editable values without inventing financial facts. */
export interface MemberEditSeed {
	id: string | number;
	name: string | null;
	email: string;
	first_name?: string | null;
	last_name?: string | null;
	role?: string | null;
	avatar?: string | null;
	tags?: string[];
}
export type MemberFullDetailsMember = MemberProfileRecord;
export type MemberFullDetails = MemberProfile;

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
