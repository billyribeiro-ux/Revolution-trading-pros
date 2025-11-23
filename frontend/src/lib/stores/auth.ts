import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

/**
 * L70 Enterprise Authentication Store
 * ====================================
 * Manages client-side authentication state with persistent storage.
 * Follows Microsoft Principal Engineer standards for security and reliability.
 */

export interface User {
	id: number;
	name: string;
	email: string;
	email_verified_at: string | null;
	created_at: string;
	updated_at: string;
	// Optional role metadata from backend (Spatie roles)
	roles?: string[];
	is_admin?: boolean;
}

interface AuthState {
	user: User | null;
	token: string | null;
	refreshToken: string | null;
	tokenExpiry: number | null;
	isAuthenticated: boolean;
	isLoading: boolean;
}

const STORAGE_KEY = 'rtp_auth_token';
const REFRESH_STORAGE_KEY = 'rtp_refresh_token';
const EXPIRY_STORAGE_KEY = 'rtp_token_expiry';

// Initialize state from localStorage if available
function getInitialState(): AuthState {
	if (browser) {
		const storedToken = localStorage.getItem(STORAGE_KEY);
		const storedRefreshToken = localStorage.getItem(REFRESH_STORAGE_KEY);
		const storedExpiryRaw = localStorage.getItem(EXPIRY_STORAGE_KEY);
		const storedExpiry = storedExpiryRaw ? parseInt(storedExpiryRaw, 10) : null;
		return {
			user: null,
			token: storedToken,
			refreshToken: storedRefreshToken,
			tokenExpiry: storedExpiry,
			isAuthenticated: !!storedToken,
			isLoading: !!storedToken // Will validate token on mount
		};
	}
	return {
		user: null,
		token: null,
		refreshToken: null,
		tokenExpiry: null,
		isAuthenticated: false,
		isLoading: false
	};
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(getInitialState());

	return {
		subscribe,

		/**
		 * Set authenticated user and token
		 */
		setAuth: (user: User, token: string, refreshToken?: string | null, expiresInSeconds?: number) => {
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
				tokenExpiry,
				isAuthenticated: true,
				isLoading: false
			}));
		},

		/**
		 * Update tokens without changing user
		 */
		updateTokens: (token: string, refreshToken?: string | null, expiresInSeconds?: number) => {
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
				if (tokenExpiry) {
					localStorage.setItem(EXPIRY_STORAGE_KEY, tokenExpiry.toString());
				} else {
					localStorage.removeItem(EXPIRY_STORAGE_KEY);
				}
			}
			update((state) => ({
				...state,
				token,
				refreshToken: refreshToken ?? null,
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
		 * Clear authentication state
		 */
		clearAuth: () => {
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
				localStorage.removeItem(REFRESH_STORAGE_KEY);
				localStorage.removeItem(EXPIRY_STORAGE_KEY);
			}
			set({
				user: null,
				token: null,
				refreshToken: null,
				tokenExpiry: null,
				isAuthenticated: false,
				isLoading: false
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
		 * Logout user and clear auth state
		 * Alias for clearAuth for compatibility
		 */
		logout: async () => {
			// Clear local state first
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
				localStorage.removeItem(REFRESH_STORAGE_KEY);
				localStorage.removeItem(EXPIRY_STORAGE_KEY);
			}
			set({
				user: null,
				token: null,
				refreshToken: null,
				tokenExpiry: null,
				isAuthenticated: false,
				isLoading: false
			});

			// Optionally call API logout endpoint (fire and forget)
			try {
				const token = localStorage.getItem(STORAGE_KEY);
				if (token && browser) {
					await fetch('/api/logout', {
						method: 'POST',
						headers: {
							Authorization: `Bearer ${token}`,
							'Content-Type': 'application/json'
						}
					});
				}
			} catch (error) {
				// Ignore API errors during logout
				console.error('Logout API error:', error);
			}
		}
	};
}

export const authStore = createAuthStore();

// Derived stores for convenience
export const user = derived(authStore, ($auth) => $auth.user);
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
export const isLoading = derived(authStore, ($auth) => $auth.isLoading);
