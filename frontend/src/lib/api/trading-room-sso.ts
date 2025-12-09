/**
 * Trading Room SSO API
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * API client for Trading Room JWT SSO authentication.
 * Provides secure access to trading rooms via JWT tokens.
 *
 * @version 1.0.0 - December 2025
 */

import { api } from './config';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface AccessibleRoom {
	id: number;
	name: string;
	slug: string;
	icon?: string;
	type: 'trading_room' | 'alert_service';
	has_access: boolean;
	membership_type: string | null;
	membership_status: 'trial' | 'active' | 'expiring' | 'paused' | 'complimentary' | null;
	sso_url: string | null;
}

export interface SSOTokenResponse {
	success: boolean;
	data: {
		jwt: string;
		redirect_url: string;
		room: {
			id: number;
			name: string;
			slug: string;
			type: string;
		};
		expires_in: number;
	};
}

export interface AccessibleRoomsResponse {
	success: boolean;
	data: {
		rooms: AccessibleRoom[];
		total_access: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get all accessible trading rooms for the current user
 */
export async function getAccessibleRooms(): Promise<AccessibleRoomsResponse> {
	return api.get('/api/trading-rooms/sso/accessible');
}

/**
 * Generate SSO token for a specific trading room
 */
export async function generateSSOToken(roomSlug: string): Promise<SSOTokenResponse> {
	return api.post(`/api/trading-rooms/${roomSlug}/sso`);
}

/**
 * Enter a trading room (generates token and opens in new tab)
 */
export async function enterTradingRoom(roomSlug: string): Promise<void> {
	try {
		const response = await generateSSOToken(roomSlug);

		if (response.success && response.data.redirect_url) {
			// Open trading room in new tab
			window.open(response.data.redirect_url, '_blank', 'noopener,noreferrer');
		} else {
			throw new Error('Failed to generate SSO token');
		}
	} catch (error) {
		console.error('[SSO] Failed to enter trading room:', error);
		throw error;
	}
}

/**
 * Get SSO redirect URL without opening (for custom handling)
 */
export async function getSSORedirectUrl(roomSlug: string): Promise<string> {
	const response = await generateSSOToken(roomSlug);

	if (response.success && response.data.redirect_url) {
		return response.data.redirect_url;
	}

	throw new Error('Failed to get SSO redirect URL');
}

// ═══════════════════════════════════════════════════════════════════════════
// MEMBERSHIP STATUS HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get human-readable membership status label
 */
export function getMembershipStatusLabel(status: AccessibleRoom['membership_status']): string {
	const labels: Record<string, string> = {
		trial: 'Trial',
		active: 'Active',
		expiring: 'Expiring Soon',
		paused: 'Paused',
		complimentary: 'Complimentary',
	};
	return labels[status ?? ''] ?? 'Active';
}

/**
 * Get status badge color class
 */
export function getMembershipStatusColor(status: AccessibleRoom['membership_status']): string {
	const colors: Record<string, string> = {
		trial: 'badge-trial',
		active: 'badge-active',
		expiring: 'badge-expiring',
		paused: 'badge-paused',
		complimentary: 'badge-complimentary',
	};
	return colors[status ?? ''] ?? 'badge-active';
}

export default {
	getAccessibleRooms,
	generateSSOToken,
	enterTradingRoom,
	getSSORedirectUrl,
	getMembershipStatusLabel,
	getMembershipStatusColor,
};
