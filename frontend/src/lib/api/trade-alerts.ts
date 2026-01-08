/**
 * Trade Alerts API Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Fetch trade alerts for memberships from backend
 * NO MOCK DATA - All data from backend API
 *
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth';

// ICT 7 FIX: VITE_API_URL does NOT include /api suffix (per config.ts pattern)
const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = browser ? (import.meta.env['VITE_API_URL'] || PROD_API_ROOT) : '';
const API_BASE = API_ROOT ? `${API_ROOT}/api` : '';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type AlertType = 'buy' | 'sell' | 'update' | 'close';
export type AlertStatus = 'active' | 'closed' | 'expired';

export interface TradeAlert {
	id: string;
	membership_id: string;
	membership_slug: string;
	type: AlertType;
	status: AlertStatus;
	symbol: string;
	entry_price?: number;
	target_price?: number;
	stop_loss?: number;
	current_price?: number;
	notes?: string;
	created_at: string;
	updated_at?: string;
	closed_at?: string;
	profit_loss?: number;
	profit_loss_percentage?: number;
}

export interface TradeAlertsResponse {
	alerts: TradeAlert[];
	total: number;
	page: number;
	per_page: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

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

class ApiError extends Error {
	constructor(
		message: string,
		public status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new ApiError(error.message || 'Request failed', response.status);
	}

	const data = await response.json();
	return data.data ?? data;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get trade alerts for a specific membership
 */
export async function getTradeAlerts(
	membershipSlug: string,
	options?: {
		type?: AlertType;
		status?: AlertStatus;
		page?: number;
		per_page?: number;
	}
): Promise<TradeAlertsResponse> {
	if (!browser) {
		return { alerts: [], total: 0, page: 1, per_page: 20 };
	}

	const params = new URLSearchParams();
	if (options?.type) params.append('type', options.type);
	if (options?.status) params.append('status', options.status);
	if (options?.page) params.append('page', options.page.toString());
	if (options?.per_page) params.append('per_page', options.per_page.toString());

	const url = `${API_BASE}/memberships/${membershipSlug}/alerts${params.toString() ? '?' + params : ''}`;

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: await getAuthHeaders(),
			credentials: 'include'
		});

		return await handleResponse<TradeAlertsResponse>(response);
	} catch (error) {
		console.error('[TradeAlerts] Error fetching alerts:', error);
		// Return empty instead of throwing - let UI handle gracefully
		return { alerts: [], total: 0, page: 1, per_page: 20 };
	}
}

/**
 * Get a single trade alert by ID
 */
export async function getTradeAlert(alertId: string): Promise<TradeAlert | null> {
	if (!browser) {
		return null;
	}

	const url = `${API_BASE}/alerts/${alertId}`;

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: await getAuthHeaders(),
			credentials: 'include'
		});

		return await handleResponse<TradeAlert>(response);
	} catch (error) {
		console.error('[TradeAlerts] Error fetching alert:', error);
		return null;
	}
}

export default {
	getTradeAlerts,
	getTradeAlert
};
