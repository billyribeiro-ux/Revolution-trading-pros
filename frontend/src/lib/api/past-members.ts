/**
 * Past Members API Service
 * Handles all past members and win-back campaign operations.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { authStore } from '$lib/stores/auth.svelte';

const API_BASE = '/api/admin/past-members';

export interface PastMember {
	id: number;
	name: string;
	email: string;
	last_membership: {
		plan_name: string;
		status: string;
		started_at: string;
		expired_at: string;
		days_since_expired: number;
		cancellation_reason: string | null;
	} | null;
	total_memberships: number;
	created_at: string;
}

export interface PastMembersStats {
	total_past_members: number;
	expired_last_30_days: number;
	expired_last_60_days: number;
	expired_last_90_days: number;
	churn_by_plan: Record<string, number>;
	reactivated_last_6_months: number;
	win_back_rate: number;
}

export interface LifecycleAnalytics {
	average_membership_duration_days: number;
	cancellation_reasons: Record<string, number>;
	monthly_churn_trend: Record<string, number>;
	retention_by_plan: Array<{
		name: string;
		active: number;
		churned: number;
		retention_rate: number;
	}>;
}

export interface PaginatedResponse<T> {
	data: T[];
	current_page: number;
	last_page: number;
	per_page: number;
	total: number;
}

export interface WinBackOptions {
	offer_code?: string;
	discount_percent?: number;
	discount_months?: number;
	expires_in_days?: number;
}

export interface BulkResult {
	message: string;
	results: {
		sent: number;
		failed: number;
		errors?: string[];
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
 * Fetch paginated list of past members with optional filters.
 */
export async function getPastMembers(
	page: number = 1,
	perPage: number = 20,
	filters: {
		search?: string;
		plan_id?: number;
		days_since_expired?: number;
		expired_after?: string;
		expired_before?: string;
	} = {}
): Promise<PaginatedResponse<PastMember>> {
	const params = new URLSearchParams({
		page: page.toString(),
		per_page: perPage.toString()
	});

	if (filters.search) params.append('search', filters.search);
	if (filters.plan_id) params.append('plan_id', filters.plan_id.toString());
	if (filters.days_since_expired)
		params.append('days_since_expired', filters.days_since_expired.toString());
	if (filters.expired_after) params.append('expired_after', filters.expired_after);
	if (filters.expired_before) params.append('expired_before', filters.expired_before);

	const response = await fetch(`${API_BASE}?${params}`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch past members');
	}

	return response.json();
}

/**
 * Fetch past members statistics.
 */
export async function getPastMembersStats(): Promise<PastMembersStats> {
	const response = await fetch(`${API_BASE}/stats`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch past members stats');
	}

	return response.json();
}

/**
 * Fetch lifecycle analytics.
 */
export async function getLifecycleAnalytics(): Promise<LifecycleAnalytics> {
	const response = await fetch(`${API_BASE}/analytics`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch lifecycle analytics');
	}

	return response.json();
}

/**
 * Send win-back email to a single past member.
 */
export async function sendWinBackEmail(
	userId: number,
	options: WinBackOptions = {}
): Promise<{ message: string; user_id: number }> {
	const response = await fetch(`${API_BASE}/${userId}/win-back`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify(options)
	});

	if (!response.ok) {
		throw new Error('Failed to send win-back email');
	}

	return response.json();
}

/**
 * Send bulk win-back emails.
 */
export async function sendBulkWinBackEmails(
	userIds: number[],
	options: WinBackOptions = {}
): Promise<BulkResult> {
	const response = await fetch(`${API_BASE}/bulk-win-back`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			user_ids: userIds,
			...options
		})
	});

	if (!response.ok) {
		throw new Error('Failed to send bulk win-back emails');
	}

	return response.json();
}

/**
 * Send feedback survey to a single past member.
 */
export async function sendFeedbackSurvey(
	userId: number,
	incentiveDescription?: string
): Promise<{ message: string; user_id: number }> {
	const response = await fetch(`${API_BASE}/${userId}/survey`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			incentive_description: incentiveDescription
		})
	});

	if (!response.ok) {
		throw new Error('Failed to send feedback survey');
	}

	return response.json();
}

/**
 * Send bulk feedback surveys.
 */
export async function sendBulkFeedbackSurveys(
	userIds: number[],
	incentiveDescription?: string
): Promise<BulkResult> {
	const response = await fetch(`${API_BASE}/bulk-survey`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			user_ids: userIds,
			incentive_description: incentiveDescription
		})
	});

	if (!response.ok) {
		throw new Error('Failed to send bulk feedback surveys');
	}

	return response.json();
}

export default {
	getPastMembers,
	getPastMembersStats,
	getLifecycleAnalytics,
	sendWinBackEmail,
	sendBulkWinBackEmails,
	sendFeedbackSurvey,
	sendBulkFeedbackSurveys
};
