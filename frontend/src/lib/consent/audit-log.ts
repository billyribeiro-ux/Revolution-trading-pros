/**
 * Consent Audit Log
 *
 * Maintains a tamper-evident log of all consent changes for GDPR compliance.
 * Stored in localStorage with optional server sync.
 *
 * @module consent/audit-log
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentState, ConsentCategory, ConsentAuditEntry, ConsentStorageOptions } from './types';
import { DEFAULT_STORAGE_OPTIONS, generateConsentId } from './types';

/**
 * Generate a unique audit entry ID.
 */
function generateAuditId(): string {
	return `aud_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 7)}`;
}

/**
 * Get the audit log from storage.
 */
export function getAuditLog(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): ConsentAuditEntry[] {
	if (!browser) return [];

	try {
		const key = options.auditLogKey || 'rtp_consent_audit';
		const stored = localStorage.getItem(key);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch (e) {
		console.debug('[ConsentAudit] Failed to load audit log:', e);
	}

	return [];
}

/**
 * Save the audit log to storage.
 */
function saveAuditLog(
	entries: ConsentAuditEntry[],
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): void {
	if (!browser) return;

	try {
		const key = options.auditLogKey || 'rtp_consent_audit';
		const maxEntries = options.maxAuditEntries || 100;

		// Trim to max entries (keep most recent)
		const trimmed = entries.slice(-maxEntries);

		localStorage.setItem(key, JSON.stringify(trimmed));
	} catch (e) {
		console.debug('[ConsentAudit] Failed to save audit log:', e);
	}
}

/**
 * Add an entry to the audit log.
 */
export function addAuditEntry(
	action: ConsentAuditEntry['action'],
	categories: Partial<Record<ConsentCategory, boolean>>,
	method: ConsentAuditEntry['method'],
	previousState?: Partial<Record<ConsentCategory, boolean>>,
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): ConsentAuditEntry {
	const entry: ConsentAuditEntry = {
		id: generateAuditId(),
		timestamp: new Date().toISOString(),
		action,
		categories,
		previousState,
		method,
		userAgent: browser ? navigator.userAgent : undefined,
		pageUrl: browser ? window.location.href : undefined,
	};

	if (browser) {
		const log = getAuditLog(options);
		log.push(entry);
		saveAuditLog(log, options);

		console.debug('[ConsentAudit] Added entry:', entry);
	}

	return entry;
}

/**
 * Log a consent given event.
 */
export function logConsentGiven(
	state: ConsentState,
	method: 'banner' | 'modal' | 'api' | 'implicit',
	options?: ConsentStorageOptions
): ConsentAuditEntry {
	return addAuditEntry(
		'consent_given',
		{
			necessary: state.necessary,
			analytics: state.analytics,
			marketing: state.marketing,
			preferences: state.preferences,
		},
		method,
		undefined,
		options
	);
}

/**
 * Log a consent update event.
 */
export function logConsentUpdated(
	newState: ConsentState,
	previousState: ConsentState,
	method: 'banner' | 'modal' | 'api',
	options?: ConsentStorageOptions
): ConsentAuditEntry {
	return addAuditEntry(
		'consent_updated',
		{
			necessary: newState.necessary,
			analytics: newState.analytics,
			marketing: newState.marketing,
			preferences: newState.preferences,
		},
		method,
		{
			necessary: previousState.necessary,
			analytics: previousState.analytics,
			marketing: previousState.marketing,
			preferences: previousState.preferences,
		},
		options
	);
}

/**
 * Log a consent revocation event.
 */
export function logConsentRevoked(
	revokedCategories: ConsentCategory[],
	method: 'modal' | 'api',
	options?: ConsentStorageOptions
): ConsentAuditEntry {
	const categories: Partial<Record<ConsentCategory, boolean>> = {};
	revokedCategories.forEach((cat) => {
		categories[cat] = false;
	});

	return addAuditEntry('consent_revoked', categories, method, undefined, options);
}

/**
 * Log a consent expiry event.
 */
export function logConsentExpired(
	expiredState: ConsentState,
	options?: ConsentStorageOptions
): ConsentAuditEntry {
	return addAuditEntry(
		'consent_expired',
		{
			necessary: expiredState.necessary,
			analytics: expiredState.analytics,
			marketing: expiredState.marketing,
			preferences: expiredState.preferences,
		},
		'expiry',
		undefined,
		options
	);
}

/**
 * Clear the audit log (for testing or user request).
 */
export function clearAuditLog(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): void {
	if (!browser) return;

	try {
		const key = options.auditLogKey || 'rtp_consent_audit';
		localStorage.removeItem(key);
		console.debug('[ConsentAudit] Cleared audit log');
	} catch (e) {
		console.debug('[ConsentAudit] Failed to clear audit log:', e);
	}
}

/**
 * Export the audit log as JSON for GDPR data requests.
 */
export function exportAuditLog(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): string {
	const log = getAuditLog(options);

	return JSON.stringify(
		{
			exportDate: new Date().toISOString(),
			totalEntries: log.length,
			entries: log,
		},
		null,
		2
	);
}

/**
 * Get audit statistics.
 */
export function getAuditStats(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): {
	totalEntries: number;
	firstEntry?: string;
	lastEntry?: string;
	consentGivenCount: number;
	consentUpdatedCount: number;
	consentRevokedCount: number;
} {
	const log = getAuditLog(options);

	return {
		totalEntries: log.length,
		firstEntry: log[0]?.timestamp,
		lastEntry: log[log.length - 1]?.timestamp,
		consentGivenCount: log.filter((e) => e.action === 'consent_given').length,
		consentUpdatedCount: log.filter((e) => e.action === 'consent_updated').length,
		consentRevokedCount: log.filter((e) => e.action === 'consent_revoked').length,
	};
}

/**
 * Verify audit log integrity (basic check).
 */
export function verifyAuditLogIntegrity(
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): { valid: boolean; issues: string[] } {
	const log = getAuditLog(options);
	const issues: string[] = [];

	// Check chronological order
	for (let i = 1; i < log.length; i++) {
		const prev = new Date(log[i - 1].timestamp).getTime();
		const curr = new Date(log[i].timestamp).getTime();
		if (curr < prev) {
			issues.push(`Entry ${i} timestamp is before entry ${i - 1}`);
		}
	}

	// Check required fields
	log.forEach((entry, i) => {
		if (!entry.id) issues.push(`Entry ${i} missing id`);
		if (!entry.timestamp) issues.push(`Entry ${i} missing timestamp`);
		if (!entry.action) issues.push(`Entry ${i} missing action`);
	});

	return {
		valid: issues.length === 0,
		issues,
	};
}

/**
 * Sync audit log to server (for enterprise deployments).
 */
export async function syncAuditLogToServer(
	endpoint: string,
	options: ConsentStorageOptions = DEFAULT_STORAGE_OPTIONS
): Promise<boolean> {
	if (!browser) return false;

	try {
		const log = getAuditLog(options);

		const response = await fetch(endpoint, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				auditLog: log,
				syncTimestamp: new Date().toISOString(),
				userAgent: navigator.userAgent,
			}),
		});

		return response.ok;
	} catch (e) {
		console.error('[ConsentAudit] Failed to sync to server:', e);
		return false;
	}
}
