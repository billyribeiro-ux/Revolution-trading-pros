/**
 * Axum Alerts Domain Adapter — Server-Only
 *
 * Typed server-side calls to the Axum alerts API.
 * Used by Remote Functions — never imported from client code.
 *
 * @version 1.0.0
 */

import { axum, AxumError } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types — Raw Axum response shapes (snake_case)
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumAlert {
	id: number;
	alert_type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	published_at: string;
	message: string;
	is_new: boolean;
	notes: string | null;
	tos_string: string | null;
}

export interface AxumAlertsResponse {
	data: AxumAlert[];
	total: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch paginated alerts for a trading room from Axum.
 */
export async function fetchAlerts(
	roomSlug: string,
	limit: number,
	offset: number
): Promise<AxumAlertsResponse> {
	const data = await axum.get<AxumAlertsResponse | AxumAlert[]>(
		`/api/alerts/${roomSlug}`,
		{ params: { limit, offset } }
	);

	// Handle both response shapes: array or {data, total}
	if (Array.isArray(data)) {
		return { data, total: data.length };
	}

	return {
		data: data?.data ?? [],
		total: data?.total ?? 0
	};
}

/**
 * Fetch a single alert by ID.
 */
export async function fetchAlert(
	roomSlug: string,
	alertId: number
): Promise<AxumAlert | null> {
	try {
		return await axum.get<AxumAlert>(`/api/alerts/${roomSlug}/${alertId}`);
	} catch (err) {
		if (err instanceof AxumError && err.isNotFound) return null;
		throw err;
	}
}

/**
 * Create a new alert.
 */
export async function createAlert(
	roomSlug: string,
	payload: Record<string, unknown>
): Promise<AxumAlert> {
	return axum.post<AxumAlert>(`/api/alerts/${roomSlug}`, payload);
}

/**
 * Update an existing alert.
 */
export async function updateAlert(
	roomSlug: string,
	alertId: number,
	payload: Record<string, unknown>
): Promise<AxumAlert> {
	return axum.put<AxumAlert>(`/api/alerts/${roomSlug}/${alertId}`, payload);
}

/**
 * Delete an alert.
 */
export async function deleteAlert(
	roomSlug: string,
	alertId: number
): Promise<void> {
	await axum.delete(`/api/alerts/${roomSlug}/${alertId}`);
}
