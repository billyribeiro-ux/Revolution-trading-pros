/**
 * Enhanced Past Members Dashboard API Service
 * Apple HIG-quality time-based filtering and bulk campaigns.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { authStore } from '$lib/stores/auth';

const API_BASE = '/api/admin/past-members-dashboard';

// Time periods available for filtering
export type TimePeriod = '30d' | '60d' | '90d' | '6mo' | '1yr' | 'all';

export const TIME_PERIOD_LABELS: Record<TimePeriod, string> = {
	'30d': 'Last 30 Days',
	'60d': 'Last 60 Days',
	'90d': 'Last 90 Days',
	'6mo': 'Last 6 Months',
	'1yr': 'Last Year',
	'all': 'All Time'
};

export interface PastMember {
	id: number;
	name: string;
	email: string;
	last_membership: {
		plan_name: string;
		status: string;
		started_at: string | null;
		expired_at: string | null;
		days_since_expired: number;
		cancellation_reason: string | null;
	} | null;
	total_memberships: number;
	total_spent: number;
	created_at: string;
}

export interface PeriodStats {
	period: TimePeriod;
	label: string;
	total_count: number;
	potential_revenue: number;
	avg_days_since_expired: number;
	top_plans: Array<{ name: string; count: number }>;
}

export interface DashboardOverview {
	periods: Record<TimePeriod, PeriodStats>;
	total_past_members: number;
	recent_campaigns: Array<{
		id: number;
		type: string;
		period: string;
		sent_count: number;
		created_at: string;
	}>;
}

export interface ServiceStats {
	id: number;
	name: string;
	type: string;
	past_member_count: number;
	potential_mrr: number;
	top_churn_reasons: string[];
}

export interface ChurnReason {
	reason: string;
	count: number;
	percentage: number;
}

export interface CampaignHistory {
	id: number;
	type: 'winback' | 'survey';
	template: string;
	period: string;
	sent_count: number;
	success_count: number;
	success_rate: number;
	created_at: string;
}

export interface BulkEmailOptions {
	period: TimePeriod;
	template: '30_free' | 'discount' | 'missed' | 'custom';
	custom_subject?: string;
	custom_body?: string;
	offer_code?: string;
	discount_percent?: number;
}

export interface BulkEmailResult {
	message: string;
	campaign_id: number;
	results: {
		sent: number;
		failed: number;
		queued: number;
	};
}

async function getAuthHeaders(): Promise<Record<string, string>> {
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

/**
 * Fetch dashboard overview with all period stats.
 */
export async function getDashboardOverview(): Promise<DashboardOverview> {
	const response = await fetch(`${API_BASE}/overview`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch dashboard overview');
	}

	return response.json();
}

/**
 * Fetch past members for a specific time period.
 */
export async function getPastMembersByPeriod(
	period: TimePeriod,
	page: number = 1,
	perPage: number = 20,
	search?: string
): Promise<{
	period: TimePeriod;
	label: string;
	data: PastMember[];
	current_page: number;
	last_page: number;
	per_page: number;
	total: number;
}> {
	const params = new URLSearchParams({
		page: page.toString(),
		per_page: perPage.toString()
	});

	if (search) params.append('search', search);

	const response = await fetch(`${API_BASE}/period/${period}?${params}`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch past members by period');
	}

	return response.json();
}

/**
 * Fetch past members grouped by service.
 */
export async function getServiceStats(): Promise<ServiceStats[]> {
	const response = await fetch(`${API_BASE}/services`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch service stats');
	}

	return response.json();
}

/**
 * Fetch churn reasons breakdown.
 */
export async function getChurnReasons(): Promise<ChurnReason[]> {
	const response = await fetch(`${API_BASE}/churn-reasons`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch churn reasons');
	}

	return response.json();
}

/**
 * Fetch campaign history.
 */
export async function getCampaignHistory(): Promise<CampaignHistory[]> {
	const response = await fetch(`${API_BASE}/campaigns`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch campaign history');
	}

	return response.json();
}

/**
 * Send bulk win-back email for a specific time period.
 */
export async function sendBulkWinBack(options: BulkEmailOptions): Promise<BulkEmailResult> {
	const response = await fetch(`${API_BASE}/bulk-winback`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify(options)
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Failed to send bulk win-back' }));
		throw new Error(error.message || 'Failed to send bulk win-back');
	}

	return response.json();
}

/**
 * Send bulk survey for a specific time period.
 */
export async function sendBulkSurvey(
	period: TimePeriod,
	incentive?: string
): Promise<BulkEmailResult> {
	const response = await fetch(`${API_BASE}/bulk-survey`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			period,
			incentive_description: incentive
		})
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Failed to send bulk survey' }));
		throw new Error(error.message || 'Failed to send bulk survey');
	}

	return response.json();
}

export default {
	getDashboardOverview,
	getPastMembersByPeriod,
	getServiceStats,
	getChurnReasons,
	getCampaignHistory,
	sendBulkWinBack,
	sendBulkSurvey,
	TIME_PERIOD_LABELS
};
