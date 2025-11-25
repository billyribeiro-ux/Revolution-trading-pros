import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';

/**
 * Revolution Trading Pros - Enterprise Authentication Store
 * =========================================================
 * Manages client-side authentication state with persistent storage.
 * Implements Microsoft-style single-session authentication.
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

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
	token: string | null;
	refreshToken: string | null;
	sessionId: string | null;
	tokenExpiry: number | null;
	isAuthenticated: boolean;
	isLoading: boolean;
	sessionInvalidated: boolean;
	invalidationReason: string | null;
}

// Storage keys
const STORAGE_KEY = 'rtp_auth_token';
const REFRESH_STORAGE_KEY = 'rtp_refresh_token';
const EXPIRY_STORAGE_KEY = 'rtp_token_expiry';
const SESSION_ID_KEY = 'rtp_session_id';

/**
 * Initialize state from localStorage if available
 */
function getInitialState(): AuthState {
	if (browser) {
		const storedToken = localStorage.getItem(STORAGE_KEY);
		const storedRefreshToken = localStorage.getItem(REFRESH_STORAGE_KEY);
		const storedSessionId = localStorage.getItem(SESSION_ID_KEY);
		const storedExpiryRaw = localStorage.getItem(EXPIRY_STORAGE_KEY);
		const storedExpiry = storedExpiryRaw ? parseInt(storedExpiryRaw, 10) : null;

		return {
			user: null,
			token: storedToken,
			refreshToken: storedRefreshToken,
			sessionId: storedSessionId,
			tokenExpiry: storedExpiry,
			isAuthenticated: !!storedToken,
			isLoading: !!storedToken, // Will validate token on mount
			sessionInvalidated: false,
			invalidationReason: null
		};
	}
	return {
		user: null,
		token: null,
		refreshToken: null,
		sessionId: null,
		tokenExpiry: null,
		isAuthenticated: false,
		isLoading: false,
		sessionInvalidated: false,
		invalidationReason: null
	};
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(getInitialState());

	return {
		subscribe,

		/**
		 * Set authenticated user and token with session
		 */
		setAuth: (
			user: User,
			token: string,
			refreshToken?: string | null,
			sessionId?: string | null,
			expiresInSeconds?: number
		) => {
			let tokenExpiry: number | null = null;
			if (expiresInSeconds) {
				tokenExpiry = Date.now() + expiresInSeconds * 1000;
			}

			if (browser) {
				localStorage.setItem(STORAGE_KEY, token);

				if (refreshToken) {
					localStorage.setItem(REFRESH_STORAGE_KEY, refreshToken);
				} else {
					localStorage.removeItem(REFRESH_STORAGE_KEY);
				}

				if (sessionId) {
					localStorage.setItem(SESSION_ID_KEY, sessionId);
				} else {
					localStorage.removeItem(SESSION_ID_KEY);
				}

				if (tokenExpiry) {
					localStorage.setItem(EXPIRY_STORAGE_KEY, tokenExpiry.toString());
				} else {
					localStorage.removeItem(EXPIRY_STORAGE_KEY);
				}
			}

			update((state) => ({
				...state,
				user,
				token,
				refreshToken: refreshToken ?? null,
				sessionId: sessionId ?? null,
				tokenExpiry,
				isAuthenticated: true,
				isLoading: false,
				sessionInvalidated: false,
				invalidationReason: null
			}));
		},

		/**
		 * Update tokens without changing user
		 */
		updateTokens: (
			token: string,
			refreshToken?: string | null,
			sessionId?: string | null,
			expiresInSeconds?: number
		) => {
			let tokenExpiry: number | null = null;
			if (expiresInSeconds) {
				tokenExpiry = Date.now() + expiresInSeconds * 1000;
			}

			if (browser) {
				if (token) {
					localStorage.setItem(STORAGE_KEY, token);
				} else {
					localStorage.removeItem(STORAGE_KEY);
				}

				if (refreshToken) {
					localStorage.setItem(REFRESH_STORAGE_KEY, refreshToken);
				} else {
					localStorage.removeItem(REFRESH_STORAGE_KEY);
				}

				if (sessionId) {
					localStorage.setItem(SESSION_ID_KEY, sessionId);
				}

				if (tokenExpiry) {
					localStorage.setItem(EXPIRY_STORAGE_KEY, tokenExpiry.toString());
				} else {
					localStorage.removeItem(EXPIRY_STORAGE_KEY);
				}
			}

			update((state) => ({
				...state,
				token,
				refreshToken: refreshToken ?? state.refreshToken,
				sessionId: sessionId ?? state.sessionId,
				tokenExpiry
			}));
		},

		/**
		 * Update user data without changing token
		 */
		setUser: (user: User) => {
			update((state) => ({
				...state,
				user
			}));
		},

		/**
		 * Mark session as invalidated (kicked out from another device)
		 */
		setSessionInvalidated: (reason?: string) => {
			update((state) => ({
				...state,
				sessionInvalidated: true,
				invalidationReason: reason || 'Your session was ended because you signed in from another device.'
			}));
		},

		/**
		 * Clear the session invalidation flag
		 */
		clearSessionInvalidated: () => {
			update((state) => ({
				...state,
				sessionInvalidated: false,
				invalidationReason: null
			}));
		},

		/**
		 * Clear authentication state
		 */
		clearAuth: () => {
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
				localStorage.removeItem(REFRESH_STORAGE_KEY);
				localStorage.removeItem(EXPIRY_STORAGE_KEY);
				localStorage.removeItem(SESSION_ID_KEY);
			}
			set({
				user: null,
				token: null,
				refreshToken: null,
				sessionId: null,
				tokenExpiry: null,
				isAuthenticated: false,
				isLoading: false,
				sessionInvalidated: false,
				invalidationReason: null
			});
		},

		/**
		 * Set loading state
		 */
		setLoading: (isLoading: boolean) => {
			update((state) => ({
				...state,
				isLoading
			}));
		},

		/**
		 * Get current token (useful for API calls)
		 */
		getToken: (): string | null => {
			if (browser) {
				return localStorage.getItem(STORAGE_KEY);
			}
			return null;
		},

		/**
		 * Get current session ID
		 */
		getSessionId: (): string | null => {
			if (browser) {
				return localStorage.getItem(SESSION_ID_KEY);
			}
			return null;
		},

		/**
		 * Logout user and clear auth state
		 */
		logout: async () => {
			const token = browser ? localStorage.getItem(STORAGE_KEY) : null;
			const sessionId = browser ? localStorage.getItem(SESSION_ID_KEY) : null;

			// Clear local state first
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
				localStorage.removeItem(REFRESH_STORAGE_KEY);
				localStorage.removeItem(EXPIRY_STORAGE_KEY);
				localStorage.removeItem(SESSION_ID_KEY);
			}

			set({
				user: null,
				token: null,
				refreshToken: null,
				sessionId: null,
				tokenExpiry: null,
				isAuthenticated: false,
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
						headers
					});
				} catch (error) {
					// Ignore API errors during logout
					console.error('Logout API error:', error);
				}
			}
		}
	};
}

export const authStore = createAuthStore();

// Derived stores for convenience
export const user = derived(authStore, ($auth) => $auth.user);
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
export const isLoading = derived(authStore, ($auth) => $auth.isLoading);
export const sessionId = derived(authStore, ($auth) => $auth.sessionId);
export const sessionInvalidated = derived(authStore, ($auth) => $auth.sessionInvalidated);
export const invalidationReason = derived(authStore, ($auth) => $auth.invalidationReason);
