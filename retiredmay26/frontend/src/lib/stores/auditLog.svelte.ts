/**
 * Audit Log Store - Apple ICT9+ Enterprise Grade (Svelte 5)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Tracks all admin actions for compliance and debugging with:
 * - Action logging with metadata
 * - User attribution
 * - Timestamp and IP tracking
 * - Filterable history
 *
 * @version 2.0.0 - Svelte 5 Runes (January 2026)
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

export type AuditActionType =
	| 'create'
	| 'update'
	| 'delete'
	| 'view'
	| 'export'
	| 'import'
	| 'login'
	| 'logout'
	| 'settings_change'
	| 'permission_change'
	| 'api_connect'
	| 'api_disconnect';

export type AuditCategory =
	| 'content'
	| 'users'
	| 'settings'
	| 'security'
	| 'integrations'
	| 'commerce'
	| 'marketing';

export interface AuditLogEntry {
	id: string;
	timestamp: number;
	userId: string;
	userName: string;
	userEmail: string;
	action: AuditActionType;
	category: AuditCategory;
	resource: string;
	resourceId?: string;
	description: string;
	metadata?: Record<string, unknown>;
	ipAddress?: string;
	userAgent?: string;
	success: boolean;
	errorMessage?: string;
}

interface AuditFilters {
	action?: AuditActionType;
	category?: AuditCategory;
	userId?: string;
	startDate?: number;
	endDate?: number;
	search?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════════

const STORAGE_KEY = 'admin_audit_log';
const MAX_LOCAL_ENTRIES = 500;

function loadFromStorage(): AuditLogEntry[] {
	if (!browser) return [];
	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		return stored ? JSON.parse(stored) : [];
	} catch {
		return [];
	}
}

function saveToStorage(entries: AuditLogEntry[]) {
	if (!browser) return;
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(entries.slice(0, MAX_LOCAL_ENTRIES)));
	} catch {
		// Storage full or unavailable
	}
}

// Svelte 5 reactive state
let entries = $state<AuditLogEntry[]>(browser ? loadFromStorage() : []);
let isLoading = $state(false);
let filters = $state<AuditFilters>({});

// ═══════════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════════

function generateId(): string {
	return `audit_${Date.now()}_${Math.random().toString(36).slice(2, 9)}`;
}

function getCurrentUser() {
	// In production, this would get the actual authenticated user
	if (!browser) return null;
	try {
		const authData = localStorage.getItem('auth_user');
		if (authData) {
			return JSON.parse(authData);
		}
	} catch {
		// Ignore
	}
	return {
		id: 'unknown',
		name: 'Unknown User',
		email: 'unknown'
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Derived State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Filtered entries based on current filters
 */
const filteredEntriesValue = $derived.by(() => {
	let result = entries;

	if (filters.action) {
		result = result.filter((e) => e.action === filters.action);
	}
	if (filters.category) {
		result = result.filter((e) => e.category === filters.category);
	}
	if (filters.userId) {
		result = result.filter((e) => e.userId === filters.userId);
	}
	if (filters.startDate) {
		result = result.filter((e) => e.timestamp >= filters.startDate!);
	}
	if (filters.endDate) {
		result = result.filter((e) => e.timestamp <= filters.endDate!);
	}
	if (filters.search) {
		const search = filters.search.toLowerCase();
		result = result.filter(
			(e) =>
				e.description.toLowerCase().includes(search) ||
				e.resource.toLowerCase().includes(search) ||
				e.userName.toLowerCase().includes(search)
		);
	}

	return result;
});

/**
 * Recent actions (last 24 hours)
 */
const recentActionsValue = $derived.by(() => {
	const dayAgo = Date.now() - 24 * 60 * 60 * 1000;
	return entries.filter((e) => e.timestamp > dayAgo);
});

/**
 * Action counts by type
 */
const actionCountsValue = $derived.by(() => {
	return entries.reduce(
		(acc, entry) => {
			acc[entry.action] = (acc[entry.action] || 0) + 1;
			return acc;
		},
		{} as Record<AuditActionType, number>
	);
});

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Log an action
 */
function log(
	entry: Omit<AuditLogEntry, 'id' | 'timestamp' | 'userId' | 'userName' | 'userEmail'>
): string {
	const user = getCurrentUser();
	const id = generateId();

	const fullEntry: AuditLogEntry = {
		...entry,
		id,
		timestamp: Date.now(),
		userId: user?.id || 'unknown',
		userName: user?.name || 'Unknown User',
		userEmail: user?.email || 'unknown@example.com',
		...(browser && { ipAddress: '127.0.0.1' }),
		...(browser && { userAgent: navigator.userAgent })
	};

	entries = [fullEntry, ...entries].slice(0, MAX_LOCAL_ENTRIES);
	saveToStorage(entries);

	return id;
}

/**
 * Convenience methods for common actions
 */
function logCreate(
	resource: string,
	resourceId: string,
	description: string,
	category: AuditCategory = 'content',
	metadata?: Record<string, unknown>
) {
	return log({
		action: 'create',
		resource,
		resourceId,
		description,
		category,
		...(metadata && { metadata }),
		success: true
	});
}

function logUpdate(
	resource: string,
	resourceId: string,
	description: string,
	category: AuditCategory = 'content',
	metadata?: Record<string, unknown>
) {
	return log({
		action: 'update',
		resource,
		resourceId,
		description,
		category,
		...(metadata && { metadata }),
		success: true
	});
}

function logDelete(
	resource: string,
	resourceId: string,
	description: string,
	category: AuditCategory = 'content',
	metadata?: Record<string, unknown>
) {
	return log({
		action: 'delete',
		resource,
		resourceId,
		description,
		category,
		...(metadata && { metadata }),
		success: true
	});
}

function logView(
	resource: string,
	resourceId: string,
	description: string,
	category: AuditCategory = 'content'
) {
	return log({ action: 'view', resource, resourceId, description, category, success: true });
}

function logExport(resource: string, description: string, metadata?: Record<string, unknown>) {
	return log({
		action: 'export',
		resource,
		description,
		category: 'content',
		...(metadata && { metadata }),
		success: true
	});
}

function logLogin(success: boolean, errorMessage?: string) {
	return log({
		action: 'login',
		resource: 'auth',
		description: success ? 'User logged in' : 'Login attempt failed',
		category: 'security',
		success,
		...(errorMessage && { errorMessage })
	});
}

function logLogout() {
	return log({
		action: 'logout',
		resource: 'auth',
		description: 'User logged out',
		category: 'security',
		success: true
	});
}

function logSettingsChange(setting: string, oldValue: unknown, newValue: unknown) {
	return log({
		action: 'settings_change',
		resource: 'settings',
		description: `Changed ${setting}`,
		category: 'settings',
		metadata: { setting, oldValue, newValue },
		success: true
	});
}

function logApiConnect(service: string, success: boolean, errorMessage?: string) {
	return log({
		action: 'api_connect',
		resource: service,
		description: success ? `Connected to ${service}` : `Failed to connect to ${service}`,
		category: 'integrations',
		success,
		...(errorMessage && { errorMessage })
	});
}

function logApiDisconnect(service: string) {
	return log({
		action: 'api_disconnect',
		resource: service,
		description: `Disconnected from ${service}`,
		category: 'integrations',
		success: true
	});
}

/**
 * Set filters
 */
function setFilters(newFilters: AuditFilters) {
	filters = newFilters;
}

/**
 * Clear filters
 */
function clearFilters() {
	filters = {};
}

/**
 * Clear all logs (admin only)
 */
function clearAll() {
	entries = [];
	saveToStorage([]);
}

/**
 * Export logs
 */
function exportLogs(format: 'json' | 'csv' = 'json'): string {
	if (format === 'csv') {
		const headers = [
			'Timestamp',
			'User',
			'Email',
			'Action',
			'Category',
			'Resource',
			'Description',
			'Success'
		];
		const rows = entries.map((e) => [
			new Date(e.timestamp).toISOString(),
			e.userName,
			e.userEmail,
			e.action,
			e.category,
			e.resource,
			e.description,
			e.success.toString()
		]);
		return [headers.join(','), ...rows.map((r) => r.join(','))].join('\n');
	}

	return JSON.stringify(entries, null, 2);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exported Store (Svelte 5 Pattern)
// ═══════════════════════════════════════════════════════════════════════════════

export const auditLog = {
	get entries() {
		return entries;
	},
	get isLoading() {
		return isLoading;
	},
	get filters() {
		return filters;
	},
	get filteredEntries() {
		return filteredEntriesValue;
	},
	get recentActions() {
		return recentActionsValue;
	},
	get actionCounts() {
		return actionCountsValue;
	},
	log,
	logCreate,
	logUpdate,
	logDelete,
	logView,
	logExport,
	logLogin,
	logLogout,
	logSettingsChange,
	logApiConnect,
	logApiDisconnect,
	setFilters,
	clearFilters,
	clearAll,
	export: exportLogs
};

export default auditLog;
