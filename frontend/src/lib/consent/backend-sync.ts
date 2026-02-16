/**
 * Backend Sync Module
 *
 * Syncs consent data to a backend server for:
 * - Centralized consent management
 * - Cross-device consent
 * - GDPR audit compliance
 * - Analytics aggregation
 *
 * @module consent/backend-sync
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, ConsentAuditEntry } from './types';
import { getAuditLog } from './audit-log';
import { logger } from '$lib/utils/logger';

/**
 * Backend sync configuration
 */
export interface BackendSyncConfig {
	/** API endpoint for syncing consent */
	endpoint: string;
	/** API key for authentication (optional) */
	apiKey?: string;
	/** Whether to sync audit log */
	syncAuditLog?: boolean;
	/** Whether to sync on every change */
	syncOnChange?: boolean;
	/** Debounce time in ms for sync */
	debounceMs?: number;
	/** Custom headers */
	headers?: Record<string, string>;
}

/**
 * Sync request payload
 */
export interface ConsentSyncPayload {
	consentId: string;
	userId?: string;
	sessionId?: string;
	consent: {
		necessary: boolean;
		analytics: boolean;
		marketing: boolean;
		preferences: boolean;
	};
	metadata: {
		updatedAt: string;
		expiresAt?: string;
		consentMethod?: string;
		countryCode?: string;
		policyVersion?: string;
		privacySignals?: {
			gpc: boolean;
			dnt: boolean;
			region?: string;
		};
	};
	auditLog?: ConsentAuditEntry[];
	userAgent?: string;
	url?: string;
}

/**
 * Sync response
 */
export interface ConsentSyncResponse {
	success: boolean;
	consentId: string;
	serverTimestamp: string;
	message?: string;
}

/**
 * Default configuration
 */
const DEFAULT_CONFIG: BackendSyncConfig = {
	endpoint: '/api/consent/sync',
	syncAuditLog: true,
	syncOnChange: true,
	debounceMs: 1000
};

let config: BackendSyncConfig = { ...DEFAULT_CONFIG };
let syncTimeout: ReturnType<typeof setTimeout> | null = null;
let lastSyncedState: string | null = null;

/**
 * Configure backend sync
 */
export function configureBackendSync(newConfig: Partial<BackendSyncConfig>): void {
	config = { ...config, ...newConfig };
	logger.debug('[BackendSync] Configured:', config);
}

/**
 * Get current sync configuration
 */
export function getSyncConfig(): BackendSyncConfig {
	return { ...config };
}

/**
 * Generate a session ID for tracking
 */
function generateSessionId(): string {
	if (!browser) return '';

	let sessionId = sessionStorage.getItem('rtp_session_id');
	if (!sessionId) {
		sessionId = `sess_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 10)}`;
		sessionStorage.setItem('rtp_session_id', sessionId);
	}
	return sessionId;
}

/**
 * Build sync payload from consent state
 */
export function buildSyncPayload(consent: ConsentState, userId?: string): ConsentSyncPayload {
	const payload: ConsentSyncPayload = {
		consentId: consent.consentId || 'unknown',
		...(userId && { userId }),
		sessionId: generateSessionId(),
		consent: {
			necessary: consent.necessary,
			analytics: consent.analytics,
			marketing: consent.marketing,
			preferences: consent.preferences
		},
		metadata: {
			updatedAt: consent.updatedAt,
			...(consent.expiresAt && { expiresAt: consent.expiresAt }),
			...(consent.consentMethod && { consentMethod: consent.consentMethod }),
			...(consent.countryCode && { countryCode: consent.countryCode }),
			...(consent.policyVersion && { policyVersion: consent.policyVersion }),
			...(consent.privacySignals && {
				privacySignals: {
					gpc: consent.privacySignals.gpc,
					dnt: consent.privacySignals.dnt,
					...(consent.privacySignals.region && { region: consent.privacySignals.region })
				}
			})
		},
		...(browser && { userAgent: navigator.userAgent }),
		...(browser && { url: window.location.href })
	};

	if (config.syncAuditLog) {
		payload.auditLog = getAuditLog().slice(-20); // Last 20 entries
	}

	return payload;
}

/**
 * Sync consent to backend
 */
export async function syncConsentToBackend(
	consent: ConsentState,
	userId?: string
): Promise<ConsentSyncResponse> {
	if (!browser) {
		return { success: false, consentId: '', serverTimestamp: '', message: 'Not in browser' };
	}

	// Check if state has changed
	const stateHash = JSON.stringify({
		analytics: consent.analytics,
		marketing: consent.marketing,
		preferences: consent.preferences
	});

	if (stateHash === lastSyncedState) {
		logger.debug('[BackendSync] State unchanged, skipping sync');
		return {
			success: true,
			consentId: consent.consentId || '',
			serverTimestamp: new Date().toISOString(),
			message: 'No changes to sync'
		};
	}

	const payload = buildSyncPayload(consent, userId);

	try {
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			...config.headers
		};

		if (config.apiKey) {
			headers['X-API-Key'] = config.apiKey;
		}

		const response = await fetch(config.endpoint, {
			method: 'POST',
			headers,
			body: JSON.stringify(payload),
			credentials: 'include'
		});

		if (!response.ok) {
			throw new Error(`HTTP ${response.status}: ${response.statusText}`);
		}

		const result: ConsentSyncResponse = await response.json();

		lastSyncedState = stateHash;
		logger.debug('[BackendSync] Sync successful:', result);

		return result;
	} catch (error) {
		logger.error('[BackendSync] Sync failed:', error);
		return {
			success: false,
			consentId: consent.consentId || '',
			serverTimestamp: new Date().toISOString(),
			message: error instanceof Error ? error.message : 'Unknown error'
		};
	}
}

/**
 * Debounced sync (for use with store subscription)
 */
export function debouncedSync(consent: ConsentState, userId?: string): void {
	if (!config.syncOnChange) return;

	if (syncTimeout) {
		clearTimeout(syncTimeout);
	}

	syncTimeout = setTimeout(() => {
		syncConsentToBackend(consent, userId);
	}, config.debounceMs);
}

/**
 * Fetch consent from backend (for cross-device sync)
 */
export async function fetchConsentFromBackend(userId: string): Promise<ConsentState | null> {
	if (!browser) return null;

	try {
		const headers: Record<string, string> = {
			...config.headers
		};

		if (config.apiKey) {
			headers['X-API-Key'] = config.apiKey;
		}

		const response = await fetch(`${config.endpoint}?userId=${encodeURIComponent(userId)}`, {
			method: 'GET',
			headers,
			credentials: 'include'
		});

		if (!response.ok) {
			if (response.status === 404) {
				logger.debug('[BackendSync] No consent found for user');
				return null;
			}
			throw new Error(`HTTP ${response.status}`);
		}

		const data = await response.json();
		logger.debug('[BackendSync] Fetched consent from backend:', data);

		return data.consent as ConsentState;
	} catch (error) {
		logger.error('[BackendSync] Fetch failed:', error);
		return null;
	}
}

/**
 * Delete consent from backend (for GDPR right to erasure)
 */
export async function deleteConsentFromBackend(consentId: string): Promise<boolean> {
	if (!browser) return false;

	try {
		const headers: Record<string, string> = {
			...config.headers
		};

		if (config.apiKey) {
			headers['X-API-Key'] = config.apiKey;
		}

		const response = await fetch(`${config.endpoint}?consentId=${encodeURIComponent(consentId)}`, {
			method: 'DELETE',
			headers,
			credentials: 'include'
		});

		if (!response.ok) {
			throw new Error(`HTTP ${response.status}`);
		}

		logger.debug('[BackendSync] Consent deleted from backend');
		return true;
	} catch (error) {
		logger.error('[BackendSync] Delete failed:', error);
		return false;
	}
}

/**
 * Export all consent data for GDPR data portability
 */
export async function exportConsentFromBackend(
	userId: string
): Promise<Record<string, unknown> | null> {
	if (!browser) return null;

	try {
		const headers: Record<string, string> = {
			...config.headers
		};

		if (config.apiKey) {
			headers['X-API-Key'] = config.apiKey;
		}

		const response = await fetch(`${config.endpoint}/export?userId=${encodeURIComponent(userId)}`, {
			method: 'GET',
			headers,
			credentials: 'include'
		});

		if (!response.ok) {
			throw new Error(`HTTP ${response.status}`);
		}

		const data = await response.json();
		logger.debug('[BackendSync] Exported consent data');

		return data;
	} catch (error) {
		logger.error('[BackendSync] Export failed:', error);
		return null;
	}
}
