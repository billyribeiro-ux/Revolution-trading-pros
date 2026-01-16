import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { 
	isSuperadmin, 
	isAdmin as checkIsAdmin, 
	hasPermission as checkHasPermission,
	hasAnyPermission,
	hasAllPermissions,
	getUserPermissions,
	getHighestRole,
	SUPERADMIN_EMAILS,
	ROLES,
	PERMISSIONS,
	type PermissionType 
} from '$lib/config/roles';

/**
 * Revolution Trading Pros - Secure Authentication Store
 * ======================================================
 * SECURITY HARDENED: Implements secure token storage pattern
 * - Access tokens stored in memory only (prevents XSS theft)
 * - Session ID stored in localStorage (low-risk identifier only)
 * - Refresh handled via httpOnly cookies (server-side)
 * - Separate isInitializing vs isLoading states to prevent UI flash
 *
 * @version 3.0.0 - Security Hardened
 * @author Revolution Trading Pros
 * @security XSS-resistant token storage
 */

// =============================================================================
// Type Definitions
// =============================================================================

export interface User {
	id: number;  // Rust API uses i64 (number in TypeScript)
	name: string;
	first_name?: string;
	last_name?: string;
	email: string;
	email_verified_at?: string | null;  // ISO timestamp or null
	email_verified?: boolean;  // Computed from email_verified_at
	created_at: string;
	updated_at?: string;
	roles?: string[];
	permissions?: string[];
	role?: string;  // Rust API sends single role
	is_admin?: boolean;
	avatar?: string;
	avatar_url?: string;  // Rust API format
	mfa_enabled?: boolean;  // Rust API
}

export interface UserSession {
	id: number;
	session_id: string;
	device_name: string;
	device_type: 'desktop' | 'mobile' | 'tablet' | 'unknown';
	device_description: string;
	browser: string | null;
	os: string | null;
	ip_address: string;
	location: string | null;
	last_activity_at: string;
	created_at: string;
	is_current: boolean;
}

interface AuthState {
	user: User | null;
	sessionId: string | null;
	tokenExpiry: number | null;
	isAuthenticated: boolean;
	isInitializing: boolean; // Separate from isLoading to prevent UI flash
	isLoading: boolean;
	sessionInvalidated: boolean;
	invalidationReason: string | null;
}

// =============================================================================
// Secure Token Storage (Memory Only)
// =============================================================================

/**
 * ICT 7 SECURITY: Tokens stored in memory only - XSS resistant
 * Apple Principal Engineer Grade: Defense in depth
 *
 * CRITICAL SECURITY NOTES:
 * - Access tokens: Memory only (this closure)
 * - Refresh tokens: httpOnly cookies ONLY (set by server)
 * - NEVER store tokens in localStorage (XSS vulnerable)
 *
 * The refresh token in memory is only used for the current API call.
 * Page refreshes rely on httpOnly cookies handled by hooks.server.ts
 */
const createSecureTokenStorage = () => {
	let accessToken: string | null = null;
	let refreshToken: string | null = null;

	return {
		setAccessToken: (token: string | null): void => {
			accessToken = token;
		},
		getAccessToken: (): string | null => {
			return accessToken;
		},
		setRefreshToken: (token: string | null): void => {
			// ICT 7 SECURITY: Store in memory only - NOT localStorage
			// Refresh tokens are also stored in httpOnly cookies by the server
			// This memory copy is for client-side API calls only
			refreshToken = token;
			// NOTE: We intentionally do NOT persist to localStorage (XSS risk)
		},
		getRefreshToken: (): string | null => {
			// ICT 7 SECURITY: Memory only - no localStorage fallback
			// If memory is empty (page refresh), rely on httpOnly cookies
			return refreshToken;
		},
		clearTokens: (): void => {
			accessToken = null;
			refreshToken = null;
			// ICT 7: Also clear any legacy localStorage tokens for cleanup
			safeLocalStorage('remove', REFRESH_TOKEN_KEY);
		}
	};
};

const secureTokens = createSecureTokenStorage();

// =============================================================================
// Session Storage (Low-risk data only)
// =============================================================================

const SESSION_ID_KEY = 'rtp_session_id';
const TOKEN_EXPIRY_KEY = 'rtp_token_expiry';
const REFRESH_TOKEN_KEY = 'rtp_refresh_token';

/**
 * Safe localStorage access with error handling
 */
function safeLocalStorage<T>(
	operation: 'get' | 'set' | 'remove',
	key: string,
	value?: T
): T | null {
	if (!browser) return null;

	try {
		switch (operation) {
			case 'get': {
				const stored = localStorage.getItem(key);
				return stored as T | null;
			}
			case 'set':
				if (value !== undefined && value !== null) {
					localStorage.setItem(key, String(value));
				}
				return null;
			case 'remove':
				localStorage.removeItem(key);
				return null;
		}
	} catch (error) {
		// Handle QuotaExceededError or SecurityError silently
		if (import.meta.env.DEV) {
			console.warn('[Auth] localStorage operation failed:', error);
		}
		return null;
	}
}

// =============================================================================
// Initial State
// =============================================================================

function getInitialState(): AuthState {
	const storedSessionId = safeLocalStorage<string>('get', SESSION_ID_KEY);
	const storedExpiryRaw = safeLocalStorage<string>('get', TOKEN_EXPIRY_KEY);
	const storedExpiry = storedExpiryRaw ? parseInt(storedExpiryRaw, 10) : null;

	// Check if we have a valid session that needs validation
	const hasSession = !!storedSessionId;

	return {
		user: null,
		sessionId: storedSessionId,
		tokenExpiry: storedExpiry,
		isAuthenticated: false, // Will be set true after token validation
		isInitializing: hasSession, // True if we need to validate existing session
		isLoading: false,
		sessionInvalidated: false,
		invalidationReason: null
	};
}

// =============================================================================
// Auth Store Factory
// =============================================================================

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(getInitialState());

	// Token refresh race condition prevention
	let refreshPromise: Promise<void> | null = null;

	return {
		subscribe,

		/**
		 * Set authenticated user and token with session
		 * @security ICT11+ Principal Engineer: Both tokens stored in memory only
		 */
		setAuth: (
			user: User,
			token: string,
			sessionId?: string | null,
			expiresInSeconds?: number,
			refreshToken?: string | null
		): void => {
			// DEFENSIVE: Ensure user object has required properties to prevent runtime errors
			const safeUser: User = {
				id: user?.id ?? 0,
				name: user?.name ?? '',
				email: user?.email ?? '',
				email_verified_at: user?.email_verified_at ?? null,
				...(user?.email_verified !== undefined && { email_verified: user.email_verified }),
				created_at: user?.created_at ?? '',
				...(user?.updated_at !== undefined && { updated_at: user.updated_at }),
				...(user?.first_name !== undefined && { first_name: user.first_name }),
				...(user?.last_name !== undefined && { last_name: user.last_name }),
				roles: user?.roles ?? (user?.role ? [user.role] : []),
				...(user?.role !== undefined && { role: user.role }),
				permissions: user?.permissions ?? [],
				is_admin: user?.is_admin ?? (user?.role === 'admin' || user?.role === 'super_admin'),
				...(user?.avatar ?? user?.avatar_url ? { avatar: user?.avatar ?? user?.avatar_url } : {}),
				...(user?.avatar_url !== undefined && { avatar_url: user.avatar_url }),
				...(user?.mfa_enabled !== undefined && { mfa_enabled: user.mfa_enabled })
			};

			// SECURITY: Store tokens in memory only (XSS-resistant)
			secureTokens.setAccessToken(token);
			if (refreshToken) {
				secureTokens.setRefreshToken(refreshToken);
			}

			let tokenExpiry: number | null = null;
			if (expiresInSeconds) {
				tokenExpiry = Date.now() + expiresInSeconds * 1000;
				safeLocalStorage('set', TOKEN_EXPIRY_KEY, tokenExpiry.toString());
			}

			// Session ID is safe to store (not a secret, just an identifier)
			if (sessionId) {
				safeLocalStorage('set', SESSION_ID_KEY, sessionId);
			}

			update((state) => ({
				...state,
				user: safeUser,
				sessionId: sessionId ?? state.sessionId,
				tokenExpiry,
				isAuthenticated: true,
				isInitializing: false,
				isLoading: false,
				sessionInvalidated: false,
				invalidationReason: null
			}));
		},

		/**
		 * Update access token without changing user
		 * @security Token stored in memory only
		 */
		updateToken: (token: string, expiresInSeconds?: number): void => {
			secureTokens.setAccessToken(token);

			let tokenExpiry: number | null = null;
			if (expiresInSeconds) {
				tokenExpiry = Date.now() + expiresInSeconds * 1000;
				safeLocalStorage('set', TOKEN_EXPIRY_KEY, tokenExpiry.toString());
			}

			update((state) => ({
				...state,
				tokenExpiry
			}));
		},

		/**
		 * Update user data without changing token
		 * Also sets isAuthenticated to true since we have valid user data
		 */
		setUser: (user: User): void => {
			// DEFENSIVE: Ensure user object has required properties to prevent runtime errors
			const safeUser: User = {
				id: user?.id ?? 0,
				name: user?.name ?? '',
				email: user?.email ?? '',
				email_verified_at: user?.email_verified_at ?? null,
				...(user?.email_verified !== undefined && { email_verified: user.email_verified }),
				created_at: user?.created_at ?? '',
				...(user?.updated_at !== undefined && { updated_at: user.updated_at }),
				...(user?.first_name !== undefined && { first_name: user.first_name }),
				...(user?.last_name !== undefined && { last_name: user.last_name }),
				roles: user?.roles ?? (user?.role ? [user.role] : []),
				...(user?.role !== undefined && { role: user.role }),
				permissions: user?.permissions ?? [],
				is_admin: user?.is_admin ?? (user?.role === 'admin' || user?.role === 'super_admin'),
				...(user?.avatar ?? user?.avatar_url ? { avatar: user?.avatar ?? user?.avatar_url } : {}),
				...(user?.avatar_url !== undefined && { avatar_url: user.avatar_url }),
				...(user?.mfa_enabled !== undefined && { mfa_enabled: user.mfa_enabled })
			};

			update((state) => ({
				...state,
				user: safeUser,
				isAuthenticated: true, // User data means we're authenticated
				isInitializing: false  // Done initializing
			}));
		},

		/**
		 * Mark session as invalidated (kicked out from another device)
		 */
		setSessionInvalidated: (reason?: string): void => {
			update((state) => ({
				...state,
				sessionInvalidated: true,
				invalidationReason:
					reason || 'Your session was ended because you signed in from another device.'
			}));
		},

		/**
		 * Clear the session invalidation flag
		 */
		clearSessionInvalidated: (): void => {
			update((state) => ({
				...state,
				sessionInvalidated: false,
				invalidationReason: null
			}));
		},

		/**
		 * Clear authentication state completely
		 * @security Clears both memory tokens and localStorage
		 */
		clearAuth: (): void => {
			// Clear memory tokens
			secureTokens.clearTokens();

			// Clear localStorage
			safeLocalStorage('remove', SESSION_ID_KEY);
			safeLocalStorage('remove', TOKEN_EXPIRY_KEY);

			set({
				user: null,
				sessionId: null,
				tokenExpiry: null,
				isAuthenticated: false,
				isInitializing: false,
				isLoading: false,
				sessionInvalidated: false,
				invalidationReason: null
			});
		},

		/**
		 * Set loading state (for user-initiated operations)
		 */
		setLoading: (isLoading: boolean): void => {
			update((state) => ({
				...state,
				isLoading
			}));
		},

		/**
		 * Complete initialization (call after validating session)
		 */
		completeInitialization: (authenticated: boolean): void => {
			update((state) => ({
				...state,
				isInitializing: false,
				isAuthenticated: authenticated
			}));
		},

		/**
		 * Get current access token (from memory)
		 * @security Token never exposed to localStorage
		 */
		getToken: (): string | null => {
			return secureTokens.getAccessToken();
		},

		/**
		 * Get current session ID
		 */
		getSessionId: (): string | null => {
			return safeLocalStorage('get', SESSION_ID_KEY);
		},

		/**
		 * Check if token is expired
		 */
		isTokenExpired: (): boolean => {
			const expiryRaw = safeLocalStorage<string>('get', TOKEN_EXPIRY_KEY);
			if (!expiryRaw) return true;

			const expiry = parseInt(expiryRaw, 10);
			// Consider expired 30 seconds before actual expiry (buffer)
			return Date.now() >= expiry - 30000;
		},

		/**
		 * ICT 7 SECURITY: Refresh access token using httpOnly cookies
		 * Apple Principal Engineer Grade: Tokens never exposed to JavaScript
		 *
		 * SECURITY ARCHITECTURE:
		 * 1. Refresh token stored in httpOnly cookie (set by server)
		 * 2. credentials: 'include' sends the cookie automatically
		 * 3. Server reads cookie, validates, returns new access token
		 * 4. New access token stored in memory only
		 *
		 * This prevents XSS attacks from stealing refresh tokens.
		 */
		refreshToken: async (): Promise<boolean> => {
			// Prevent multiple concurrent refresh attempts (race condition prevention)
			if (refreshPromise) {
				await refreshPromise;
				return !!secureTokens.getAccessToken();
			}

			refreshPromise = (async () => {
				try {
					// ICT 7 SECURITY: Try memory token first, then rely on httpOnly cookie
					const currentRefreshToken = secureTokens.getRefreshToken();

					// Build request body - include token if in memory, otherwise server uses cookie
					const requestBody: Record<string, string> = {};
					if (currentRefreshToken) {
						requestBody.refresh_token = currentRefreshToken;
					}

					const response = await fetch('/api/auth/refresh', {
						method: 'POST',
						credentials: 'include', // ICT 7: Sends httpOnly cookies
						headers: {
							'Content-Type': 'application/json',
							'Accept': 'application/json'
						},
						body: JSON.stringify(requestBody)
					});

					if (!response.ok) {
						// ICT 7: Check for specific error codes
						if (response.status === 401) {
							throw new Error('Refresh token expired or invalid');
						}
						throw new Error(`Token refresh failed: ${response.status}`);
					}

					const rawData = await response.json();

					// ICT 7: Unwrap the data envelope
					const data = rawData.data || rawData;

					// ICT 7: Backend sends "access_token", not "token"
					const newAccessToken = data.access_token || data.token;

					if (newAccessToken) {
						// Store new access token in memory only (XSS safe)
						secureTokens.setAccessToken(newAccessToken);

						// Update refresh token in memory if rotated (new one provided)
						// The httpOnly cookie is updated by the server response
						if (data.refresh_token) {
							secureTokens.setRefreshToken(data.refresh_token);
						}

						if (data.expires_in) {
							const tokenExpiry = Date.now() + data.expires_in * 1000;
							safeLocalStorage('set', TOKEN_EXPIRY_KEY, tokenExpiry.toString());
							update((state) => ({ ...state, tokenExpiry }));
						}

						if (import.meta.env.DEV) {
							console.debug('[Auth] Token refreshed successfully');
						}
					} else {
						throw new Error('No access token in refresh response');
					}
				} catch (error) {
					if (import.meta.env.DEV) {
						console.error('[Auth] Token refresh failed:', error);
					}
					throw error;
				} finally {
					refreshPromise = null;
				}
			})();

			try {
				await refreshPromise;
				return true;
			} catch {
				return false;
			}
		},

		/**
		 * Logout user and clear auth state
		 * @security Clears all tokens and redirects
		 */
		logout: async (redirectTo: string = '/login'): Promise<void> => {
			const token = secureTokens.getAccessToken();
			const sessionId = safeLocalStorage<string>('get', SESSION_ID_KEY);

			// Clear local state immediately
			secureTokens.clearTokens();
			safeLocalStorage('remove', SESSION_ID_KEY);
			safeLocalStorage('remove', TOKEN_EXPIRY_KEY);

			set({
				user: null,
				sessionId: null,
				tokenExpiry: null,
				isAuthenticated: false,
				isInitializing: false,
				isLoading: false,
				sessionInvalidated: false,
				invalidationReason: null
			});

			// Call API logout endpoint (fire and forget with timeout)
			// ICT11+ Pattern: Prevent hanging on unresponsive server
			if (token && browser) {
				try {
					const headers: Record<string, string> = {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json'
					};

					if (sessionId) {
						headers['X-Session-ID'] = sessionId;
					}

					// ICT11+ Pattern: 5-second timeout prevents blocking on network issues
					const controller = new AbortController();
					const timeoutId = setTimeout(() => controller.abort(), 5000);

					// ICT7 FIX: Use /api/logout which has a frontend proxy at /api/logout/+server.ts
					await fetch('/api/logout', {
						method: 'POST',
						headers,
						credentials: 'include', // Clear httpOnly cookie
						signal: controller.signal,
						body: JSON.stringify({ session_id: sessionId || null })
					});

					clearTimeout(timeoutId);
				} catch (error) {
					// Ignore API errors during logout (including timeout/abort)
					if (import.meta.env.DEV) {
						console.error('[Auth] Logout API error:', error);
					}
				}
			}

			// Use SvelteKit navigation with replaceState to prevent history pollution
			// Google Enterprise Pattern: After logout, back button shouldn't return to protected pages
			if (browser && redirectTo) {
				await goto(redirectTo, { replaceState: true });
			}
		}
	};
}

// =============================================================================
// Export Store and Derived Stores
// =============================================================================

export const authStore = createAuthStore();

// Derived stores for convenience
// DEFENSIVE: Ensure user object has required properties when accessed via derived store
export const user = derived(authStore, ($auth) => {
	if (!$auth.user) return null;
	return {
		...$auth.user,
		email: $auth.user.email ?? '',
		name: $auth.user.name ?? '',
		roles: $auth.user.roles ?? [],
		permissions: $auth.user.permissions ?? []
	};
});
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
export const isLoading = derived(authStore, ($auth) => $auth.isLoading);
export const isInitializing = derived(authStore, ($auth) => $auth.isInitializing);
export const sessionId = derived(authStore, ($auth) => $auth.sessionId);
export const sessionInvalidated = derived(authStore, ($auth) => $auth.sessionInvalidated);
export const invalidationReason = derived(authStore, ($auth) => $auth.invalidationReason);

// Role-based derived stores
// DEFENSIVE: Ensure user object has email property before passing to role functions
const getSafeUser = (user: User | null) => {
	if (!user) return null;
	return {
		...user,
		email: user.email ?? '',
		roles: user.roles ?? [],
		permissions: user.permissions ?? []
	};
};

export const isSuperAdmin = derived(authStore, ($auth) => isSuperadmin(getSafeUser($auth.user)));
export const isAdminUser = derived(authStore, ($auth) => checkIsAdmin(getSafeUser($auth.user)));
export const userRole = derived(authStore, ($auth) => getHighestRole(getSafeUser($auth.user)));
export const userPermissions = derived(authStore, ($auth) => getUserPermissions(getSafeUser($auth.user)));

/**
 * Create a derived store that checks for a specific permission
 */
export function createPermissionStore(permission: PermissionType | string) {
	return derived(authStore, ($auth) => checkHasPermission(getSafeUser($auth.user), permission));
}

// Export function to get token (for use in API clients)
export const getAuthToken = (): string | null => authStore.getToken();
export const getSessionId = (): string | null => authStore.getSessionId();

// Re-export role helpers for convenience
export { 
	isSuperadmin, 
	checkIsAdmin, 
	checkHasPermission as hasPermission, 
	hasAnyPermission, 
	hasAllPermissions,
	SUPERADMIN_EMAILS,
	ROLES,
	PERMISSIONS
};
