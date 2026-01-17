/**
 * SvelteKit 5 Consent Storage Module
 *
 * Handles persisting consent preferences using cookies (primary)
 * and localStorage (fallback/backup).
 *
 * @module consent/storage
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, ConsentStorageOptions } from './types';
import { DEFAULT_CONSENT_STATE, DEFAULT_STORAGE_OPTIONS, CONSENT_SCHEMA_VERSION } from './types';

/**
 * Check if cookies are available and writable.
 */
function cookiesAvailable(): boolean {
	if (!browser) return false;

	try {
		document.cookie = '__consent_test=1';
		const available = document.cookie.includes('__consent_test');
		document.cookie = '__consent_test=; expires=Thu, 01 Jan 1970 00:00:00 GMT';
		return available;
	} catch {
		return false;
	}
}

/**
 * Check if localStorage is available and writable.
 */
function localStorageAvailable(): boolean {
	if (!browser) return false;

	try {
		const key = '__consent_test';
		localStorage.setItem(key, '1');
		localStorage.removeItem(key);
		return true;
	} catch {
		return false;
	}
}

/**
 * Parse a cookie string to get a specific value.
 */
function getCookie(name: string): string | null {
	if (!browser) return null;

	const cookies = document.cookie.split(';');
	for (const cookie of cookies) {
		const [key, value] = cookie.trim().split('=');
		if (key === name && value !== undefined) {
			try {
				return decodeURIComponent(value);
			} catch {
				return value ?? null;
			}
		}
	}
	return null;
}

/**
 * Set a cookie with the specified options.
 */
function setCookie(name: string, value: string, options: ConsentStorageOptions): boolean {
	if (!browser || !cookiesAvailable()) return false;

	try {
		const expires = new Date();
		expires.setTime(expires.getTime() + options.cookieExpiry * 24 * 60 * 60 * 1000);

		let cookieString = `${name}=${encodeURIComponent(value)}`;
		cookieString += `; expires=${expires.toUTCString()}`;
		cookieString += `; path=/`;
		cookieString += `; SameSite=${options.sameSite}`;

		if (options.domain) {
			cookieString += `; domain=${options.domain}`;
		}

		if (options.secure && location.protocol === 'https:') {
			cookieString += '; Secure';
		}

		document.cookie = cookieString;
		return true;
	} catch (e) {
		console.debug('[Consent] Failed to set cookie:', e);
		return false;
	}
}

/**
 * Delete a cookie.
 */
function deleteCookie(name: string, options: ConsentStorageOptions): void {
	if (!browser) return;

	let cookieString = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/`;
	if (options.domain) {
		cookieString += `; domain=${options.domain}`;
	}
	document.cookie = cookieString;
}

/**
 * Validate and migrate consent state if needed.
 */
function validateAndMigrate(data: unknown): ConsentState | null {
	if (!data || typeof data !== 'object') return null;

	const state = data as Partial<ConsentState>;

	// Check required fields exist
	if (
		typeof state.necessary !== 'boolean' ||
		typeof state.analytics !== 'boolean' ||
		typeof state.marketing !== 'boolean' ||
		typeof state.preferences !== 'boolean'
	) {
		return null;
	}

	// Migrate from older versions if needed
	const version = state.version ?? 0;
	if (version < CONSENT_SCHEMA_VERSION) {
		console.debug(
			`[Consent] Migrating consent state from v${version} to v${CONSENT_SCHEMA_VERSION}`
		);
		// Future migrations would go here
	}

	return {
		necessary: true, // Always force necessary to true
		analytics: Boolean(state.analytics),
		marketing: Boolean(state.marketing),
		preferences: Boolean(state.preferences),
		updatedAt: state.updatedAt || new Date().toISOString(),
		hasInteracted: Boolean(state.hasInteracted),
		version: CONSENT_SCHEMA_VERSION
	};
}

/**
 * Load consent state from storage (cookie first, then localStorage).
 */
export function loadConsent(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): ConsentState {
	if (!browser) {
		return { ...DEFAULT_CONSENT_STATE };
	}

	// Try cookie first
	const cookieValue = getCookie(options.cookieName);
	if (cookieValue) {
		try {
			const parsed = JSON.parse(cookieValue);
			const validated = validateAndMigrate(parsed);
			if (validated) {
				console.debug('[Consent] Loaded consent from cookie');
				return validated;
			}
		} catch (e) {
			console.debug('[Consent] Failed to parse cookie:', e);
		}
	}

	// Fallback to localStorage
	if (localStorageAvailable()) {
		try {
			const stored = localStorage.getItem(options.localStorageKey);
			if (stored) {
				const parsed = JSON.parse(stored);
				const validated = validateAndMigrate(parsed);
				if (validated) {
					console.debug('[Consent] Loaded consent from localStorage');
					// Sync back to cookie if available
					saveConsent(validated, options);
					return validated;
				}
			}
		} catch (e) {
			console.debug('[Consent] Failed to load from localStorage:', e);
		}
	}

	console.debug('[Consent] No stored consent found, using defaults');
	return { ...DEFAULT_CONSENT_STATE };
}

/**
 * Save consent state to storage (both cookie and localStorage).
 */
export function saveConsent(
	state: ConsentState,
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): boolean {
	if (!browser) return false;

	const stateWithMeta: ConsentState = {
		...state,
		necessary: true, // Always ensure necessary is true
		updatedAt: new Date().toISOString(),
		version: CONSENT_SCHEMA_VERSION
	};

	const serialized = JSON.stringify(stateWithMeta);
	let saved = false;

	// Try cookie first
	if (setCookie(options.cookieName, serialized, options)) {
		saved = true;
		console.debug('[Consent] Saved consent to cookie');
	}

	// Also save to localStorage as backup
	if (localStorageAvailable()) {
		try {
			localStorage.setItem(options.localStorageKey, serialized);
			saved = true;
			console.debug('[Consent] Saved consent to localStorage');
		} catch (e) {
			console.debug('[Consent] Failed to save to localStorage:', e);
		}
	}

	if (!saved) {
		console.warn('[Consent] Could not persist consent preferences');
	}

	return saved;
}

/**
 * Clear all stored consent data.
 */
export function clearConsent(options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS): void {
	if (!browser) return;

	deleteCookie(options.cookieName, options);

	if (localStorageAvailable()) {
		try {
			localStorage.removeItem(options.localStorageKey);
		} catch {
			// Ignore errors
		}
	}

	console.debug('[Consent] Cleared stored consent');
}

/**
 * Check if consent storage is available.
 */
export function isStorageAvailable(): boolean {
	return cookiesAvailable() || localStorageAvailable();
}
