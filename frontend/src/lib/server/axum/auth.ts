/**
 * Axum Auth Domain Adapter — Server-Only
 *
 * @version 1.0.0
 */

import { axum } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface AxumUserProfile {
	id: number;
	email: string;
	name: string;
	role: string;
	is_admin?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Fetch current user profile (requires auth token in cookies).
 */
export async function fetchCurrentUser(): Promise<AxumUserProfile> {
	return axum.get<AxumUserProfile>('/api/auth/me');
}

/**
 * Check if current user is an admin.
 * Returns false on any error (unauthenticated, network, etc).
 */
export async function checkAdminStatus(): Promise<boolean> {
	try {
		const user = await fetchCurrentUser();
		return user.is_admin === true || user.role === 'admin' || user.role === 'super_admin';
	} catch {
		return false;
	}
}
