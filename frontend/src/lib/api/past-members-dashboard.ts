/**
 * Past Members Dashboard API Service - Svelte 5 / SvelteKit Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade API service with:
 * - Multi-tier caching (memory + localStorage)
 * - Request deduplication
 * - Stale-while-revalidate
 * - ETag support for 304 responses
 * - TypeScript strict typing
 *
 * @version 3.0.0 (SvelteKit / December 2025)
 */

import { authStore } from '$lib/stores/auth.svelte';
import { apiCache, buildCacheKey, invalidateCache } from './cache';

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

const API_BASE = '/api/admin/past-members-dashboard';

// Cache TTLs (in milliseconds)
const CACHE_TTL = {
	overview: 5 * 60 * 1000,        // 5 minutes
	period: 3 * 60 * 1000,          // 3 minutes
	services: 10 * 60 * 1000,       // 10 minutes
	churnReasons: 5 * 60 * 1000,    // 5 minutes
	campaigns: 2 * 60 * 1000        // 2 minutes
} as const;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type TimePeriod = '30d' | '60d' | '90d' | '6mo' | '1yr' | 'all';

export const TIME_PERIOD_LABELS: Record<TimePeriod, string> = {
	'30d': 'Last 30 Days',
	'60d': 'Last 60 Days',
	'90d': 'Last 90 Days',
	'6mo': 'Last 6 Months',
	'1yr': 'Last Year',
	'all': 'All Time'
} as const;

export interface WinBackPotential {
	score: number;
	level: 'high' | 'medium' | 'low';
	factors: string[];
}

export interface LastMembership {
	plan_name: string | null;
	status: string;
	started_at: string | null;
	expired_at: string | null;
	days_since_expired: number | null;
	cancellation_reason: string | null;
}

export interface PastMember {
	id: number;
	name: string;
	first_name: string | null;
	last_name: string | null;
	email: string;
	avatar: string | null;
	last_membership: LastMembership | null;
	total_memberships?: number;
	total_spent: number;
	churned_at: string | null;
	churn_reason: string | null;
	days_since_churn: number | null;
	joined_at: string;
	win_back_potential: WinBackPotential;
}

export interface PeriodStats {
	total_count: number;
	potential_revenue: number;
	avg_days_since_expired: number;
	previous_period: number;
	change_percent: number;
	trend: 'up' | 'down' | 'stable';
	top_plans: Array<{ name: string; count: number }>;
	label?: string;
}

export interface DashboardOverview {
	periods: Record<TimePeriod, PeriodStats>;
	total_past_members?: number;
	recent_campaigns?: CampaignHistory[];
}

export interface ServiceStats {
	id: number;
	name: string;
	type: string;
	slug: string;
	price: number;
	icon: string | null;
	churned_count: number;
	active_count: number;
	churn_rate: number;
}

export interface ChurnReason {
	reason: string;
	count: number;
	percentage: number;
}

export interface CampaignHistory {
	campaign_id: string;
	campaign_type: 'winback' | 'survey';
	template: string;
	total_sent: number;
	started_at: string;
	completed_at: string | null;
	unique_recipients: number;
	offers_used: number;
	duration_seconds: number | null;
}

export interface BulkEmailOptions {
	period: TimePeriod;
	template: '30_free' | 'discount' | 'missed' | 'custom';
	service_id?: number;
	custom_subject?: string;
	custom_body?: string;
	offer_code?: string;
	discount_percent?: number;
	expires_in_days?: number;
	limit?: number;
	exclude_contacted_within_days?: number;
	dry_run?: boolean;
}

export interface BulkEmailResult {
	success: boolean;
	message: string;
	campaign_id: string | null;
	period: TimePeriod;
	stats: {
		eligible: number;
		queued: number;
		failed: number;
	};
	dry_run?: boolean;
}

export interface PaginatedResponse<T> {
	success: boolean;
	period: TimePeriod;
	period_label: string;
	stats: PeriodStats;
	members: T[];
	pagination: {
		total: number;
		per_page: number;
		current_page: number;
		last_page: number;
		from: number;
		to: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

async function getAuthHeaders(etag?: string): Promise<Record<string, string>> {
	const token = authStore.getToken();
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		'Accept': 'application/json'
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	if (etag) {
		headers['If-None-Match'] = etag;
	}

	return headers;
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (response.status === 304) {
		// Not modified - caller should use cached data
		throw new NotModifiedError();
	}

	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new ApiError(error.message || 'Request failed', response.status);
	}

	const data = await response.json();
	return data.data ?? data;
}

class ApiError extends Error {
	constructor(message: string, public status: number) {
		super(message);
		this.name = 'ApiError';
	}
}

class NotModifiedError extends Error {
	constructor() {
		super('Not Modified');
		this.name = 'NotModifiedError';
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch dashboard overview with all period stats (cached)
 */
export async function getDashboardOverview(options?: { skipCache?: boolean }): Promise<DashboardOverview> {
	const cacheKey = buildCacheKey(`${API_BASE}/overview`);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	return apiCache.getOrFetch<DashboardOverview>(
		cacheKey,
		async () => {
			const response = await fetch(`${API_BASE}/overview`, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
			return handleResponse<DashboardOverview>(response);
		},
		{ ttl: CACHE_TTL.overview, persist: true }
	);
}

/**
 * Fetch past members for a specific time period (cached)
 */
export async function getPastMembersByPeriod(
	period: TimePeriod,
	page: number = 1,
	perPage: number = 25,
	search?: string,
	options?: { skipCache?: boolean; serviceId?: number; minSpent?: number; sortBy?: string; sortDir?: 'asc' | 'desc' }
): Promise<PaginatedResponse<PastMember>> {
	const params = new URLSearchParams({
		page: page.toString(),
		per_page: perPage.toString()
	});

	if (search) params.append('search', search);
	if (options?.serviceId) params.append('service_id', options.serviceId.toString());
	if (options?.minSpent) params.append('min_spent', options.minSpent.toString());
	if (options?.sortBy) params.append('sort_by', options.sortBy);
	if (options?.sortDir) params.append('sort_dir', options.sortDir);

	const url = `${API_BASE}/period/${period}?${params}`;
	const cacheKey = buildCacheKey(url);

	if (options?.skipCache) {
		apiCache.delete(cacheKey);
	}

	return apiCache.getOrFetch<PaginatedResponse<PastMember>>(
		cacheKey,
		async () => {
			const response = await fetch(url, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
			return handleResponse<PaginatedResponse<PastMember>>(response);
		},
		{ ttl: CACHE_TTL.period }
	);
}

/**
 * Fetch available services with stats (cached)
 */
export async function getServiceStats(): Promise<ServiceStats[]> {
	const cacheKey = buildCacheKey(`${API_BASE}/services`);

	return apiCache.getOrFetch<ServiceStats[]>(
		cacheKey,
		async () => {
			const response = await fetch(`${API_BASE}/services`, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
			const data = await handleResponse<{ services: ServiceStats[] } | ServiceStats[]>(response);
			return Array.isArray(data) ? data : data.services;
		},
		{ ttl: CACHE_TTL.services, persist: true }
	);
}

/**
 * Fetch churn reasons breakdown (cached)
 */
export async function getChurnReasons(period?: TimePeriod): Promise<ChurnReason[]> {
	const params = period ? `?period=${period}` : '';
	const url = `${API_BASE}/churn-reasons${params}`;
	const cacheKey = buildCacheKey(url);

	return apiCache.getOrFetch<ChurnReason[]>(
		cacheKey,
		async () => {
			const response = await fetch(url, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
			const data = await handleResponse<{ reasons: ChurnReason[] } | ChurnReason[]>(response);
			return Array.isArray(data) ? data : data.reasons;
		},
		{ ttl: CACHE_TTL.churnReasons }
	);
}

/**
 * Fetch campaign history (cached)
 */
export async function getCampaignHistory(): Promise<CampaignHistory[]> {
	const cacheKey = buildCacheKey(`${API_BASE}/campaigns`);

	return apiCache.getOrFetch<CampaignHistory[]>(
		cacheKey,
		async () => {
			const response = await fetch(`${API_BASE}/campaigns`, {
				method: 'GET',
				headers: await getAuthHeaders(),
				credentials: 'include'
			});
			const data = await handleResponse<{ campaigns: CampaignHistory[] } | CampaignHistory[]>(response);
			return Array.isArray(data) ? data : data.campaigns;
		},
		{ ttl: CACHE_TTL.campaigns }
	);
}

/**
 * Fetch real-time stats (no cache)
 */
export async function getRealtimeStats(): Promise<{
	active_campaigns: number;
	emails_sent_today: number;
	recent_churns: number;
	timestamp: string;
}> {
	const response = await fetch(`${API_BASE}/realtime`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	return handleResponse(response);
}

/**
 * Send bulk win-back email (invalidates cache)
 */
export async function sendBulkWinBack(options: BulkEmailOptions): Promise<BulkEmailResult> {
	const response = await fetch(`${API_BASE}/bulk-winback`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify(options)
	});

	const result = await handleResponse<BulkEmailResult>(response);

	// Invalidate related caches
	if (result.success && !options.dry_run) {
		invalidateCache(/past-members-dashboard/);
	}

	return result;
}

/**
 * Send bulk survey (invalidates cache)
 */
export async function sendBulkSurvey(
	period: TimePeriod,
	incentive?: string,
	options?: { serviceId?: number; limit?: number }
): Promise<BulkEmailResult> {
	const response = await fetch(`${API_BASE}/bulk-survey`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			period,
			incentive,
			service_id: options?.serviceId,
			limit: options?.limit
		})
	});

	const result = await handleResponse<BulkEmailResult>(response);

	// Invalidate related caches
	if (result.success) {
		invalidateCache(/past-members-dashboard/);
	}

	return result;
}

/**
 * Invalidate dashboard cache manually
 */
export async function invalidateDashboardCache(): Promise<void> {
	// Call backend to clear server-side cache
	await fetch(`${API_BASE}/invalidate-cache`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	// Clear client-side cache
	invalidateCache(/past-members-dashboard/);
}

/**
 * Preload dashboard data (for faster initial load)
 */
export async function preloadDashboardData(): Promise<void> {
	await Promise.allSettled([
		getDashboardOverview(),
		getServiceStats(),
		getChurnReasons(),
		getCampaignHistory()
	]);
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT
// ═══════════════════════════════════════════════════════════════════════════

export default {
	getDashboardOverview,
	getPastMembersByPeriod,
	getServiceStats,
	getChurnReasons,
	getCampaignHistory,
	getRealtimeStats,
	sendBulkWinBack,
	sendBulkSurvey,
	invalidateDashboardCache,
	preloadDashboardData,
	TIME_PERIOD_LABELS
};
