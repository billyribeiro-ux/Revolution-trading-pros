import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';

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
	id: number;
	name: string;
	first_name?: string;
	last_name?: string;
	email: string;
	email_verified_at: string | null;
	created_at: string;
	updated_at: string;
	roles?: string[];
	is_admin?: boolean;
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
 * SECURITY: Tokens stored in memory only - not accessible via XSS
 * This closure prevents direct access to tokens from global scope
 */
const createSecureTokenStorage = () => {
	let accessToken: string | null = null;

	return {
		setAccessToken: (token: string | null): void => {
			accessToken = token;
		},
		getAccessToken: (): string | null => {
			return accessToken;
		},
		clearTokens: (): void => {
			accessToken = null;
		}
	};
};

const secureTokens = createSecureTokenStorage();

// =============================================================================
// Session Storage (Low-risk data only)
// =============================================================================

const SESSION_ID_KEY = 'rtp_session_id';
const TOKEN_EXPIRY_KEY = 'rtp_token_expiry';

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
		 * @security Access token stored in memory only
		 */
		setAuth: (
			user: User,
			token: string,
			sessionId?: string | null,
			expiresInSeconds?: number
		): void => {
			// SECURITY: Store access token in memory only
			secureTokens.setAccessToken(token);

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
				user,
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
		 */
		setUser: (user: User): void => {
			update((state) => ({
				...state,
				user
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
		 * Refresh access token with race condition prevention
		 * @security Uses httpOnly cookie for refresh token (server-side)
		 */
		refreshToken: async (): Promise<boolean> => {
			// Prevent multiple concurrent refresh attempts
			if (refreshPromise) {
				await refreshPromise;
				return !!secureTokens.getAccessToken();
			}

			refreshPromise = (async () => {
				try {
					const response = await fetch('/api/auth/refresh', {
						method: 'POST',
						credentials: 'include', // Include httpOnly cookies
						headers: {
							'Content-Type': 'application/json',
							'X-Session-ID': safeLocalStorage('get', SESSION_ID_KEY) || ''
						}
					});

					if (!response.ok) {
						throw new Error('Token refresh failed');
					}

					const data = await response.json();

					if (data.token) {
						secureTokens.setAccessToken(data.token);

						if (data.expires_in) {
							const tokenExpiry = Date.now() + data.expires_in * 1000;
							safeLocalStorage('set', TOKEN_EXPIRY_KEY, tokenExpiry.toString());
							update((state) => ({ ...state, tokenExpiry }));
						}
					}
				} catch (error) {
					if (import.meta.env.DEV) {
						console.error('[Auth] Token refresh failed:', error);
					}
					// Don't clear auth here - let the caller decide
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

			// Call API logout endpoint (fire and forget)
			if (token && browser) {
				try {
					const headers: Record<string, string> = {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json'
					};

					if (sessionId) {
						headers['X-Session-ID'] = sessionId;
					}

					await fetch('/api/logout', {
						method: 'POST',
						headers,
						credentials: 'include' // Clear httpOnly cookie
					});
				} catch (error) {
					// Ignore API errors during logout
					if (import.meta.env.DEV) {
						console.error('[Auth] Logout API error:', error);
					}
				}
			}

			// Use SvelteKit navigation instead of hard redirect
			if (browser && redirectTo) {
				await goto(redirectTo);
			}
		}
	};
}

// =============================================================================
// Export Store and Derived Stores
// =============================================================================

export const authStore = createAuthStore();

// Derived stores for convenience
export const user = derived(authStore, ($auth) => $auth.user);
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
export const isLoading = derived(authStore, ($auth) => $auth.isLoading);
export const isInitializing = derived(authStore, ($auth) => $auth.isInitializing);
export const sessionId = derived(authStore, ($auth) => $auth.sessionId);
export const sessionInvalidated = derived(authStore, ($auth) => $auth.sessionInvalidated);
export const invalidationReason = derived(authStore, ($auth) => $auth.invalidationReason);

// Export function to get token (for use in API clients)
export const getAuthToken = (): string | null => authStore.getToken();
export const getSessionId = (): string | null => authStore.getSessionId();

// Re-export removeToast function if needed by Toast component
export { removeToast } from '$lib/stores/toast';
