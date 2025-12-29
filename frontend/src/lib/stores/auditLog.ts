/**
 * Audit Log Store - Apple ICT9+ Enterprise Grade
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Tracks all admin actions for compliance and debugging with:
 * - Action logging with metadata
 * - User attribution
 * - Timestamp and IP tracking
 * - Filterable history
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
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

interface AuditLogState {
	entries: AuditLogEntry[];
	isLoading: boolean;
	filters: {
		action?: AuditActionType;
		category?: AuditCategory;
		userId?: string;
		startDate?: number;
		endDate?: number;
		search?: string;
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store
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

const initialState: AuditLogState = {
	entries: browser ? loadFromStorage() : [],
	isLoading: false,
	filters: {}
};

const auditLogStore = writable<AuditLogState>(initialState);

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
		id: 'admin',
		name: 'Admin User',
		email: 'admin@example.com'
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

export const auditLog = {
	subscribe: auditLogStore.subscribe,

	/**
	 * Log an action
	 */
	log(entry: Omit<AuditLogEntry, 'id' | 'timestamp' | 'userId' | 'userName' | 'userEmail'>): string {
		const user = getCurrentUser();
		const id = generateId();

		const fullEntry: AuditLogEntry = {
			...entry,
			id,
			timestamp: Date.now(),
			userId: user?.id || 'unknown',
			userName: user?.name || 'Unknown User',
			userEmail: user?.email || 'unknown@example.com',
			...(browser && { ipAddress: '127.0.0.1' }), // In production, get from request
			...(browser && { userAgent: navigator.userAgent })
		};

		auditLogStore.update(state => {
			const entries = [fullEntry, ...state.entries].slice(0, MAX_LOCAL_ENTRIES);
			saveToStorage(entries);
			return { ...state, entries };
		});

		// In production, also send to server
		// sendToServer(fullEntry);

		return id;
	},

	/**
	 * Convenience methods for common actions
	 */
	logCreate(resource: string, resourceId: string, description: string, category: AuditCategory = 'content', metadata?: Record<string, unknown>) {
		return this.log({ action: 'create', resource, resourceId, description, category, ...(metadata && { metadata }), success: true });
	},

	logUpdate(resource: string, resourceId: string, description: string, category: AuditCategory = 'content', metadata?: Record<string, unknown>) {
		return this.log({ action: 'update', resource, resourceId, description, category, ...(metadata && { metadata }), success: true });
	},

	logDelete(resource: string, resourceId: string, description: string, category: AuditCategory = 'content', metadata?: Record<string, unknown>) {
		return this.log({ action: 'delete', resource, resourceId, description, category, ...(metadata && { metadata }), success: true });
	},

	logView(resource: string, resourceId: string, description: string, category: AuditCategory = 'content') {
		return this.log({ action: 'view', resource, resourceId, description, category, success: true });
	},

	logExport(resource: string, description: string, metadata?: Record<string, unknown>) {
		return this.log({ action: 'export', resource, description, category: 'content', ...(metadata && { metadata }), success: true });
	},

	logLogin(success: boolean, errorMessage?: string) {
		return this.log({
			action: 'login',
			resource: 'auth',
			description: success ? 'User logged in' : 'Login attempt failed',
			category: 'security',
			success,
			...(errorMessage && { errorMessage })
		});
	},

	logLogout() {
		return this.log({
			action: 'logout',
			resource: 'auth',
			description: 'User logged out',
			category: 'security',
			success: true
		});
	},

	logSettingsChange(setting: string, oldValue: unknown, newValue: unknown) {
		return this.log({
			action: 'settings_change',
			resource: 'settings',
			description: `Changed ${setting}`,
			category: 'settings',
			metadata: { setting, oldValue, newValue },
			success: true
		});
	},

	logApiConnect(service: string, success: boolean, errorMessage?: string) {
		return this.log({
			action: 'api_connect',
			resource: service,
			description: success ? `Connected to ${service}` : `Failed to connect to ${service}`,
			category: 'integrations',
			success,
			...(errorMessage && { errorMessage })
		});
	},

	logApiDisconnect(service: string) {
		return this.log({
			action: 'api_disconnect',
			resource: service,
			description: `Disconnected from ${service}`,
			category: 'integrations',
			success: true
		});
	},

	/**
	 * Set filters
	 */
	setFilters(filters: AuditLogState['filters']) {
		auditLogStore.update(state => ({ ...state, filters }));
	},

	/**
	 * Clear filters
	 */
	clearFilters() {
		auditLogStore.update(state => ({ ...state, filters: {} }));
	},

	/**
	 * Clear all logs (admin only)
	 */
	clearAll() {
		auditLogStore.update(state => {
			saveToStorage([]);
			return { ...state, entries: [] };
		});
	},

	/**
	 * Export logs
	 */
	export(format: 'json' | 'csv' = 'json'): string {
		const state = get(auditLogStore);

		if (format === 'csv') {
			const headers = ['Timestamp', 'User', 'Email', 'Action', 'Category', 'Resource', 'Description', 'Success'];
			const rows = state.entries.map(e => [
				new Date(e.timestamp).toISOString(),
				e.userName,
				e.userEmail,
				e.action,
				e.category,
				e.resource,
				e.description,
				e.success.toString()
			]);
			return [headers.join(','), ...rows.map(r => r.join(','))].join('\n');
		}

		return JSON.stringify(state.entries, null, 2);
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Derived Stores
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Filtered entries based on current filters
 */
export const filteredEntries = derived(auditLogStore, ($state) => {
	let entries = $state.entries;
	const { filters } = $state;

	if (filters.action) {
		entries = entries.filter(e => e.action === filters.action);
	}
	if (filters.category) {
		entries = entries.filter(e => e.category === filters.category);
	}
	if (filters.userId) {
		entries = entries.filter(e => e.userId === filters.userId);
	}
	if (filters.startDate) {
		entries = entries.filter(e => e.timestamp >= filters.startDate!);
	}
	if (filters.endDate) {
		entries = entries.filter(e => e.timestamp <= filters.endDate!);
	}
	if (filters.search) {
		const search = filters.search.toLowerCase();
		entries = entries.filter(e =>
			e.description.toLowerCase().includes(search) ||
			e.resource.toLowerCase().includes(search) ||
			e.userName.toLowerCase().includes(search)
		);
	}

	return entries;
});

/**
 * Recent actions (last 24 hours)
 */
export const recentActions = derived(auditLogStore, ($state) => {
	const dayAgo = Date.now() - 24 * 60 * 60 * 1000;
	return $state.entries.filter(e => e.timestamp > dayAgo);
});

/**
 * Action counts by type
 */
export const actionCounts = derived(auditLogStore, ($state) => {
	return $state.entries.reduce((acc, entry) => {
		acc[entry.action] = (acc[entry.action] || 0) + 1;
		return acc;
	}, {} as Record<AuditActionType, number>);
});

export default auditLog;
