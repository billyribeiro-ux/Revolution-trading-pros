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
 * SVELTE 5 RUNES VERSION
 * 
 * SECURITY HARDENED: Implements secure token storage pattern
 * - Access tokens stored in memory only (prevents XSS theft)
 * - Session ID stored in localStorage (low-risk identifier only)
 * - Refresh handled via httpOnly cookies (server-side)
 * - Separate isInitializing vs isLoading states to prevent UI flash
 *
 * @version 4.0.0 - Svelte 5 Runes Migration
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

// =============================================================================
// Secure Token Storage (Memory Only)
// =============================================================================

const SESSION_ID_KEY = 'rtp_session_id';
const TOKEN_EXPIRY_KEY = 'rtp_token_expiry';
const REFRESH_TOKEN_KEY = 'rtp_refresh_token';

/**
 * ICT 7 SECURITY: Tokens stored in memory only - XSS resistant
 * Apple Principal Engineer Grade: Defense in depth
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
			refreshToken = token;
		},
		getRefreshToken: (): string | null => {
			return refreshToken;
		},
		clearTokens: (): void => {
			accessToken = null;
			refreshToken = null;
			safeLocalStorage('remove', REFRESH_TOKEN_KEY);
		}
	};
};

const secureTokens = createSecureTokenStorage();

// =============================================================================
// Session Storage (Low-risk data only)
// =============================================================================

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
		if (import.meta.env.DEV) {
			console.warn('[Auth] localStorage operation failed:', error);
		}
		return null;
	}
}

// =============================================================================
// Helper Functions
// =============================================================================

function getInitialSessionId(): string | null {
	return safeLocalStorage<string>('get', SESSION_ID_KEY);
}

function getInitialTokenExpiry(): number | null {
	const storedExpiryRaw = safeLocalStorage<string>('get', TOKEN_EXPIRY_KEY);
	return storedExpiryRaw ? parseInt(storedExpiryRaw, 10) : null;
}

function getSafeUser(user: User | null): User | null {
	if (!user) return null;
	return {
		...user,
		email: user.email ?? '',
		roles: user.roles ?? [],
		permissions: user.permissions ?? []
	};
}

function createSafeUser(user: User): User {
	return {
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
}

// =============================================================================
// Auth Store Class - Svelte 5 Runes
// =============================================================================

// Store subscriber type
type Subscriber<T> = (value: T) => void;
type Unsubscriber = () => void;

// Auth state interface for subscribe compatibility
interface AuthStateSnapshot {
	user: User | null;
	sessionId: string | null;
	tokenExpiry: number | null;
	isAuthenticated: boolean;
	isInitializing: boolean;
	isLoading: boolean;
	sessionInvalidated: boolean;
	invalidationReason: string | null;
}

class AuthStoreClass {
	// State - using $state for reactivity
	private _user = $state<User | null>(null);
	private _sessionId = $state<string | null>(getInitialSessionId());
	private _tokenExpiry = $state<number | null>(getInitialTokenExpiry());
	private _isInitializing = $state(!!getInitialSessionId()); // True if we need to validate
	private _isLoading = $state(false);
	private _sessionInvalidated = $state(false);
	private _invalidationReason = $state<string | null>(null);

	// Token refresh race condition prevention
	private refreshPromise: Promise<void> | null = null;

	// Subscribers for backward compatibility with $store syntax
	private subscribers = new Set<Subscriber<AuthStateSnapshot>>();

	// ==========================================================================
	// Subscribe method for backward compatibility with $authStore syntax
	// ==========================================================================

	/**
	 * Subscribe to auth state changes
	 * Required for components using $authStore syntax
	 */
	subscribe(fn: Subscriber<AuthStateSnapshot>): Unsubscriber {
		this.subscribers.add(fn);
		// Immediately call with current state
		fn(this.getSnapshot());
		// Return unsubscribe function
		return () => {
			this.subscribers.delete(fn);
		};
	}

	private getSnapshot(): AuthStateSnapshot {
		return {
			user: this._user,
			sessionId: this._sessionId,
			tokenExpiry: this._tokenExpiry,
			isAuthenticated: this._user !== null,
			isInitializing: this._isInitializing,
			isLoading: this._isLoading,
			sessionInvalidated: this._sessionInvalidated,
			invalidationReason: this._invalidationReason
		};
	}

	private notifySubscribers(): void {
		const snapshot = this.getSnapshot();
		this.subscribers.forEach(fn => fn(snapshot));
	}

	// ==========================================================================
	// Getters for state access (reactive in Svelte 5)
	// ==========================================================================

	get user(): User | null { return this._user; }
	get sessionId(): string | null { return this._sessionId; }
	get tokenExpiry(): number | null { return this._tokenExpiry; }
	get isInitializing(): boolean { return this._isInitializing; }
	get isLoading(): boolean { return this._isLoading; }
	get sessionInvalidated(): boolean { return this._sessionInvalidated; }
	get invalidationReason(): string | null { return this._invalidationReason; }

	// ==========================================================================
	// Derived values as getters
	// ==========================================================================

	get isAuthenticated(): boolean {
		return this._user !== null;
	}

	get isSuperAdmin(): boolean {
		return isSuperadmin(getSafeUser(this._user));
	}

	get isAdminUser(): boolean {
		return checkIsAdmin(getSafeUser(this._user));
	}

	get userRole(): string | null {
		return getHighestRole(getSafeUser(this._user));
	}

	get userPermissions(): string[] {
		return getUserPermissions(getSafeUser(this._user));
	}

	// Safe user getter with defaults
	get safeUser(): User | null {
		if (!this._user) return null;
		return {
			...this._user,
			email: this._user.email ?? '',
			name: this._user.name ?? '',
			roles: this._user.roles ?? [],
			permissions: this._user.permissions ?? []
		};
	}

	// ==========================================================================
	// Methods
	// ==========================================================================

	/**
	 * Set authenticated user and token with session
	 * @security ICT11+ Principal Engineer: Both tokens stored in memory only
	 */
	setAuth(
		user: User,
		token: string,
		sessionId?: string | null,
		expiresInSeconds?: number,
		refreshToken?: string | null
	): void {
		const safeUser = createSafeUser(user);

		// SECURITY: Store tokens in memory only (XSS-resistant)
		secureTokens.setAccessToken(token);
		if (refreshToken) {
			secureTokens.setRefreshToken(refreshToken);
		}

		if (expiresInSeconds) {
			this._tokenExpiry = Date.now() + expiresInSeconds * 1000;
			safeLocalStorage('set', TOKEN_EXPIRY_KEY, this._tokenExpiry.toString());
		}

		// Session ID is safe to store (not a secret, just an identifier)
		if (sessionId) {
			this._sessionId = sessionId;
			safeLocalStorage('set', SESSION_ID_KEY, sessionId);
		}

		this._user = safeUser;
		this._isInitializing = false;
		this._isLoading = false;
		this._sessionInvalidated = false;
		this._invalidationReason = null;
		this.notifySubscribers();
	}

	/**
	 * Update access token without changing user
	 * @security Token stored in memory only
	 */
	updateToken(token: string, expiresInSeconds?: number): void {
		secureTokens.setAccessToken(token);

		if (expiresInSeconds) {
			this._tokenExpiry = Date.now() + expiresInSeconds * 1000;
			safeLocalStorage('set', TOKEN_EXPIRY_KEY, this._tokenExpiry.toString());
		}
		this.notifySubscribers();
	}

	/**
	 * Update user data without changing token
	 */
	setUser(user: User): void {
		const safeUser = createSafeUser(user);
		this._user = safeUser;
		this._isInitializing = false;
		this.notifySubscribers();
	}

	/**
	 * Mark session as invalidated (kicked out from another device)
	 */
	setSessionInvalidated(reason?: string): void {
		this._sessionInvalidated = true;
		this._invalidationReason = reason || 'Your session was ended because you signed in from another device.';
		this.notifySubscribers();
	}

	/**
	 * Clear the session invalidation flag
	 */
	clearSessionInvalidated(): void {
		this._sessionInvalidated = false;
		this._invalidationReason = null;
		this.notifySubscribers();
	}

	/**
	 * Clear authentication state completely
	 * @security Clears both memory tokens and localStorage
	 */
	clearAuth(): void {
		secureTokens.clearTokens();
		safeLocalStorage('remove', SESSION_ID_KEY);
		safeLocalStorage('remove', TOKEN_EXPIRY_KEY);

		this._user = null;
		this._sessionId = null;
		this._tokenExpiry = null;
		this._isInitializing = false;
		this._isLoading = false;
		this._sessionInvalidated = false;
		this._invalidationReason = null;
		this.notifySubscribers();
	}

	/**
	 * Set loading state (for user-initiated operations)
	 */
	setLoading(isLoading: boolean): void {
		this._isLoading = isLoading;
		this.notifySubscribers();
	}

	/**
	 * Complete initialization (call after validating session)
	 */
	completeInitialization(authenticated: boolean): void {
		this._isInitializing = false;
		if (!authenticated) {
			this._user = null;
		}
		this.notifySubscribers();
	}

	/**
	 * Get current access token (from memory)
	 * @security Token never exposed to localStorage
	 */
	getToken(): string | null {
		return secureTokens.getAccessToken();
	}

	/**
	 * Get current session ID
	 */
	getSessionId(): string | null {
		return safeLocalStorage('get', SESSION_ID_KEY);
	}

	/**
	 * Check if token is expired
	 */
	isTokenExpired(): boolean {
		const expiryRaw = safeLocalStorage<string>('get', TOKEN_EXPIRY_KEY);
		if (!expiryRaw) return true;

		const expiry = parseInt(expiryRaw, 10);
		// Consider expired 30 seconds before actual expiry (buffer)
		return Date.now() >= expiry - 30000;
	}

	/**
	 * Check if user has a specific permission
	 */
	hasPermission(permission: PermissionType | string): boolean {
		return checkHasPermission(getSafeUser(this._user), permission);
	}

	/**
	 * CLIENT-SIDE TOKEN REFRESH
	 *
	 * This is one of THREE token refresh implementations:
	 * 1. HERE: Client-side refresh via /api/auth/refresh proxy
	 * 2. /src/lib/api/auth.ts: Delegates to this store
	 * 3. /src/hooks.server.ts: Server-side refresh for SSR
	 *
	 * All three MUST use the same proxy endpoint and cookie handling.
	 * If modifying refresh logic, update all three locations.
	 */
	async refreshToken(): Promise<boolean> {
		// Prevent multiple concurrent refresh attempts
		if (this.refreshPromise) {
			await this.refreshPromise;
			return !!secureTokens.getAccessToken();
		}

		this.refreshPromise = (async () => {
			try {
				const currentRefreshToken = secureTokens.getRefreshToken();

				const requestBody: Record<string, string> = {};
				if (currentRefreshToken) {
					requestBody.refresh_token = currentRefreshToken;
				}

				const response = await fetch('/api/auth/refresh', {
					method: 'POST',
					credentials: 'include',
					headers: {
						'Content-Type': 'application/json',
						'Accept': 'application/json'
					},
					body: JSON.stringify(requestBody)
				});

				if (!response.ok) {
					if (response.status === 401) {
						throw new Error('Refresh token expired or invalid');
					}
					throw new Error(`Token refresh failed: ${response.status}`);
				}

				const rawData = await response.json();
				const data = rawData.data || rawData;
				const newAccessToken = data.access_token || data.token;

				if (newAccessToken) {
					secureTokens.setAccessToken(newAccessToken);

					if (data.refresh_token) {
						secureTokens.setRefreshToken(data.refresh_token);
					}

					if (data.expires_in) {
						this._tokenExpiry = Date.now() + data.expires_in * 1000;
						safeLocalStorage('set', TOKEN_EXPIRY_KEY, this._tokenExpiry.toString());
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
				this.refreshPromise = null;
			}
		})();

		try {
			await this.refreshPromise;
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Logout user and clear auth state
	 * @security Clears all tokens and redirects
	 */
	async logout(redirectTo: string = '/login'): Promise<void> {
		const token = secureTokens.getAccessToken();
		const sessionId = safeLocalStorage<string>('get', SESSION_ID_KEY);

		// Clear local state immediately
		secureTokens.clearTokens();
		safeLocalStorage('remove', SESSION_ID_KEY);
		safeLocalStorage('remove', TOKEN_EXPIRY_KEY);

		this._user = null;
		this._sessionId = null;
		this._tokenExpiry = null;
		this._isInitializing = false;
		this._isLoading = false;
		this._sessionInvalidated = false;
		this._invalidationReason = null;

		// Call API logout endpoint (fire and forget with timeout)
		if (token && browser) {
			try {
				const headers: Record<string, string> = {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				};

				if (sessionId) {
					headers['X-Session-ID'] = sessionId;
				}

				const controller = new AbortController();
				const timeoutId = setTimeout(() => controller.abort(), 5000);

				await fetch('/api/logout', {
					method: 'POST',
					headers,
					credentials: 'include',
					signal: controller.signal,
					body: JSON.stringify({ session_id: sessionId || null })
				});

				clearTimeout(timeoutId);
			} catch (error) {
				if (import.meta.env.DEV) {
					console.error('[Auth] Logout API error:', error);
				}
			}
		}

		if (browser && redirectTo) {
			await goto(redirectTo, { replaceState: true });
		}
	}
}

// =============================================================================
// Export Store Instance
// =============================================================================

export const authStore = new AuthStoreClass();

// =============================================================================
// Backward Compatibility Exports
// =============================================================================
// These provide the same interface as the old derived stores
// Components can import these directly for backward compatibility

/**
 * Reactive user getter - use in components as: {user?.name}
 * In Svelte 5, access via authStore.user or this exported getter
 */
export const user = {
	get current() { return authStore.user; },
	// For components that destructure or use directly
	subscribe(fn: (value: User | null) => void) {
		// Initial call
		fn(authStore.user);
		// Return unsubscribe (no-op since we use $state reactivity)
		return () => {};
	}
};

/**
 * Reactive auth status getter
 */
export const isAuthenticated = {
	get current() { return authStore.isAuthenticated; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.isAuthenticated);
		return () => {};
	}
};

export const isLoading = {
	get current() { return authStore.isLoading; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.isLoading);
		return () => {};
	}
};

export const isInitializing = {
	get current() { return authStore.isInitializing; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.isInitializing);
		return () => {};
	}
};

export const sessionId = {
	get current() { return authStore.sessionId; },
	subscribe(fn: (value: string | null) => void) {
		fn(authStore.sessionId);
		return () => {};
	}
};

export const sessionInvalidated = {
	get current() { return authStore.sessionInvalidated; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.sessionInvalidated);
		return () => {};
	}
};

export const invalidationReason = {
	get current() { return authStore.invalidationReason; },
	subscribe(fn: (value: string | null) => void) {
		fn(authStore.invalidationReason);
		return () => {};
	}
};

export const isSuperAdmin = {
	get current() { return authStore.isSuperAdmin; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.isSuperAdmin);
		return () => {};
	}
};

export const isAdminUser = {
	get current() { return authStore.isAdminUser; },
	subscribe(fn: (value: boolean) => void) {
		fn(authStore.isAdminUser);
		return () => {};
	}
};

export const userRole = {
	get current() { return authStore.userRole; },
	subscribe(fn: (value: string | null) => void) {
		fn(authStore.userRole);
		return () => {};
	}
};

export const userPermissions = {
	get current() { return authStore.userPermissions; },
	subscribe(fn: (value: string[]) => void) {
		fn(authStore.userPermissions);
		return () => {};
	}
};

// =============================================================================
// Convenience Exports
// =============================================================================

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
