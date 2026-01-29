/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Shared Authentication Utilities
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Handles both API response formats from /api/auth/me:
 * Format A: { is_admin: boolean, role: string }
 * Format B: { user: { is_admin: boolean, role: string } }
 *
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+
 */

interface AuthResponseFormatA {
	is_admin?: boolean;
	role?: string;
}

interface AuthResponseFormatB {
	user?: {
		is_admin?: boolean;
		role?: string;
	};
}

type AuthResponse = AuthResponseFormatA & AuthResponseFormatB;

const ADMIN_ROLES = ['admin', 'super_admin'] as const;

/**
 * Check if current user has admin privileges
 * Handles multiple response formats for backward compatibility
 */
export async function checkAdminStatus(): Promise<boolean> {
	try {
		const response = await fetch('/api/auth/me', {
			credentials: 'include'
		});

		if (!response.ok) {
			return false;
		}

		const data: AuthResponse = await response.json();

		// Check Format A (direct properties)
		if (data.is_admin === true) return true;
		if (data.role && ADMIN_ROLES.includes(data.role as (typeof ADMIN_ROLES)[number])) return true;

		// Check Format B (nested under user)
		if (data.user?.is_admin === true) return true;
		if (data.user?.role && ADMIN_ROLES.includes(data.user.role as (typeof ADMIN_ROLES)[number]))
			return true;

		return false;
	} catch (error) {
		console.error('Auth check failed:', error);
		return false;
	}
}

/**
 * Check if user is authenticated (any role)
 */
export async function checkAuthStatus(): Promise<boolean> {
	try {
		const response = await fetch('/api/auth/me', {
			credentials: 'include'
		});
		return response.ok;
	} catch {
		return false;
	}
}
