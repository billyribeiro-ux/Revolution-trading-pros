/**
 * Abandoned Cart Recovery API Service
 * Enterprise-grade cart recovery with automated email sequences.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { authStore } from '$lib/stores/auth';

const API_BASE = '/api/admin/abandoned-carts';

export type CartStatus = 'pending' | 'email_sent' | 'clicked' | 'recovered' | 'expired' | 'unsubscribed';

export interface AbandonedCart {
	id: number;
	user_id: number | null;
	session_id: string;
	user_name: string | null;
	user_email: string | null;
	cart_data: {
		items: Array<{
			id: number;
			name: string;
			price: number;
			quantity: number;
		}>;
		total: number;
		coupon?: string;
	};
	cart_value: number;
	status: CartStatus;
	recovery_attempts: number;
	last_email_at: string | null;
	clicked_at: string | null;
	recovered_at: string | null;
	abandoned_at: string;
	created_at: string;
}

export interface DashboardStats {
	total_abandoned: number;
	total_value: number;
	recovered_count: number;
	recovered_value: number;
	recovery_rate: number;
	avg_cart_value: number;
	pending_recovery: number;
	email_sent_count: number;
	clicked_count: number;
	by_status: Record<CartStatus, number>;
	trend_7_days: Array<{
		date: string;
		abandoned: number;
		recovered: number;
	}>;
}

export interface RecoveryTemplate {
	id: string;
	name: string;
	subject: string;
	preview: string;
	timing: string;
	has_discount: boolean;
}

export interface RecoveryEmailOptions {
	template: string;
	custom_subject?: string;
	custom_body?: string;
	discount_code?: string;
	discount_percent?: number;
}

export interface RecoveryResult {
	message: string;
	cart_id: number;
	email_sent: boolean;
}

export interface BulkRecoveryResult {
	message: string;
	results: {
		sent: number;
		failed: number;
		skipped: number;
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
 * Fetch dashboard statistics.
 */
export async function getDashboardStats(): Promise<DashboardStats> {
	const response = await fetch(`${API_BASE}/dashboard`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch dashboard stats');
	}

	return response.json();
}

/**
 * Fetch abandoned carts with filtering.
 */
export async function getAbandonedCarts(
	page: number = 1,
	perPage: number = 20,
	filters: {
		status?: CartStatus;
		search?: string;
		min_value?: number;
		abandoned_after?: string;
		abandoned_before?: string;
	} = {}
): Promise<{
	data: AbandonedCart[];
	current_page: number;
	last_page: number;
	per_page: number;
	total: number;
}> {
	const params = new URLSearchParams({
		page: page.toString(),
		per_page: perPage.toString()
	});

	if (filters.status) params.append('status', filters.status);
	if (filters.search) params.append('search', filters.search);
	if (filters.min_value) params.append('min_value', filters.min_value.toString());
	if (filters.abandoned_after) params.append('abandoned_after', filters.abandoned_after);
	if (filters.abandoned_before) params.append('abandoned_before', filters.abandoned_before);

	const response = await fetch(`${API_BASE}?${params}`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch abandoned carts');
	}

	return response.json();
}

/**
 * Fetch single abandoned cart details.
 */
export async function getAbandonedCart(id: number): Promise<AbandonedCart> {
	const response = await fetch(`${API_BASE}/${id}`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch abandoned cart');
	}

	return response.json();
}

/**
 * Fetch available recovery email templates.
 */
export async function getRecoveryTemplates(): Promise<RecoveryTemplate[]> {
	const response = await fetch(`${API_BASE}/templates`, {
		method: 'GET',
		headers: await getAuthHeaders(),
		credentials: 'include'
	});

	if (!response.ok) {
		throw new Error('Failed to fetch recovery templates');
	}

	return response.json();
}

/**
 * Send recovery email to a single cart.
 */
export async function sendRecoveryEmail(
	cartId: number,
	options: RecoveryEmailOptions = { template: 'reminder_1' }
): Promise<RecoveryResult> {
	const response = await fetch(`${API_BASE}/${cartId}/send-recovery`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify(options)
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Failed to send recovery email' }));
		throw new Error(error.message || 'Failed to send recovery email');
	}

	return response.json();
}

/**
 * Send bulk recovery emails.
 */
export async function sendBulkRecovery(
	cartIds: number[],
	options: RecoveryEmailOptions = { template: 'reminder_1' }
): Promise<BulkRecoveryResult> {
	const response = await fetch(`${API_BASE}/bulk-recovery`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({
			cart_ids: cartIds,
			...options
		})
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Failed to send bulk recovery' }));
		throw new Error(error.message || 'Failed to send bulk recovery');
	}

	return response.json();
}

/**
 * Mark cart as recovered manually.
 */
export async function markAsRecovered(cartId: number, notes?: string): Promise<{ message: string }> {
	const response = await fetch(`${API_BASE}/${cartId}/mark-recovered`, {
		method: 'POST',
		headers: await getAuthHeaders(),
		credentials: 'include',
		body: JSON.stringify({ notes })
	});

	if (!response.ok) {
		throw new Error('Failed to mark cart as recovered');
	}

	return response.json();
}

/**
 * Report abandoned cart from frontend (public endpoint).
 */
export async function reportAbandonedCart(data: {
	session_id: string;
	cart_data: object;
	cart_value: number;
	user_email?: string;
}): Promise<{ message: string; cart_id: number }> {
	const response = await fetch('/api/cart/abandoned', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(data)
	});

	if (!response.ok) {
		throw new Error('Failed to report abandoned cart');
	}

	return response.json();
}

export const STATUS_LABELS: Record<CartStatus, string> = {
	pending: 'Pending',
	email_sent: 'Email Sent',
	clicked: 'Clicked',
	recovered: 'Recovered',
	expired: 'Expired',
	unsubscribed: 'Unsubscribed'
};

export const STATUS_COLORS: Record<CartStatus, string> = {
	pending: 'warning',
	email_sent: 'info',
	clicked: 'primary',
	recovered: 'success',
	expired: 'neutral',
	unsubscribed: 'neutral'
};

export default {
	getDashboardStats,
	getAbandonedCarts,
	getAbandonedCart,
	getRecoveryTemplates,
	sendRecoveryEmail,
	sendBulkRecovery,
	markAsRecovered,
	reportAbandonedCart,
	STATUS_LABELS,
	STATUS_COLORS
};
