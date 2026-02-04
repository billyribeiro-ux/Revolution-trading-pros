/**
 * Revolution Trading Pros - Blog Editor Sync Manager
 * ===================================================
 * Manages synchronization of offline changes with the server.
 * Implements conflict resolution, exponential backoff, and event-driven architecture.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

import { browser } from '$app/environment';
import {
	blogEditorDB,
	generateChecksum,
	createRevisionFromDraft,
	type Draft,
	type PendingChange
} from './db';
import type { Block } from '../types';

// =============================================================================
// Types
// =============================================================================

export type SyncEventType = 'sync-start' | 'sync-complete' | 'sync-error' | 'conflict';

export interface SyncStartEvent {
	type: 'sync-start';
	pendingCount: number;
	timestamp: number;
}

export interface SyncCompleteEvent {
	type: 'sync-complete';
	synced: number;
	failed: number;
	duration: number;
	timestamp: number;
}

export interface SyncErrorEvent {
	type: 'sync-error';
	changeId: string;
	error: string;
	attempt: number;
	willRetry: boolean;
	timestamp: number;
}

export interface ConflictEvent {
	type: 'conflict';
	changeId: string;
	draftId: string;
	localVersion: number;
	serverVersion: number;
	resolution: 'server-wins' | 'local-wins' | 'manual';
	backupCreated: boolean;
	timestamp: number;
}

export type SyncEvent = SyncStartEvent | SyncCompleteEvent | SyncErrorEvent | ConflictEvent;

export interface SyncStatus {
	isOnline: boolean;
	isSyncing: boolean;
	pendingCount: number;
	lastSyncedAt: number | null;
	lastError: string | null;
	consecutiveFailures: number;
}

export interface SyncOptions {
	/** Maximum number of retry attempts */
	maxRetries: number;
	/** Base delay for exponential backoff (ms) */
	baseDelay: number;
	/** Maximum delay cap (ms) */
	maxDelay: number;
	/** Batch size for syncing */
	batchSize: number;
	/** Enable automatic sync on online event */
	autoSyncOnOnline: boolean;
	/** Interval for periodic sync attempts (ms) */
	syncInterval: number;
	/** API endpoint base URL */
	apiBaseUrl: string;
}

export interface ConflictResolution {
	strategy: 'server-wins' | 'local-wins' | 'manual';
	createBackup: boolean;
	mergeFunction?: (local: Draft, server: Draft) => Draft;
}

export interface SyncResult {
	success: boolean;
	changeId: string;
	error?: string;
	serverData?: unknown;
	conflict?: boolean;
}

// =============================================================================
// Default Configuration
// =============================================================================

const DEFAULT_OPTIONS: SyncOptions = {
	maxRetries: 5,
	baseDelay: 1000,
	maxDelay: 60000,
	batchSize: 5,
	autoSyncOnOnline: true,
	syncInterval: 30000, // 30 seconds
	apiBaseUrl: '/api'
};

// =============================================================================
// Sync Manager Class
// =============================================================================

class SyncManager {
	private options: SyncOptions;
	private status: SyncStatus;
	private listeners: Map<SyncEventType, Set<(event: SyncEvent) => void>> = new Map();
	private syncTimer: ReturnType<typeof setInterval> | null = null;
	private syncPromise: Promise<void> | null = null;
	private abortController: AbortController | null = null;

	constructor(options: Partial<SyncOptions> = {}) {
		this.options = { ...DEFAULT_OPTIONS, ...options };
		this.status = {
			isOnline: browser ? navigator.onLine : true,
			isSyncing: false,
			pendingCount: 0,
			lastSyncedAt: null,
			lastError: null,
			consecutiveFailures: 0
		};

		// Initialize event listener maps
		const eventTypes: SyncEventType[] = ['sync-start', 'sync-complete', 'sync-error', 'conflict'];
		for (const type of eventTypes) {
			this.listeners.set(type, new Set());
		}

		if (browser) {
			this.setupNetworkListeners();
			this.startPeriodicSync();
			this.updatePendingCount();
		}
	}

	// ===========================================================================
	// Event Handling
	// ===========================================================================

	/**
	 * Subscribe to sync events
	 */
	on(type: SyncEventType, callback: (event: SyncEvent) => void): () => void {
		const listeners = this.listeners.get(type);
		if (listeners) {
			listeners.add(callback);
		}
		return () => this.off(type, callback);
	}

	/**
	 * Unsubscribe from sync events
	 */
	off(type: SyncEventType, callback: (event: SyncEvent) => void): void {
		const listeners = this.listeners.get(type);
		if (listeners) {
			listeners.delete(callback);
		}
	}

	/**
	 * Emit a sync event
	 */
	private emit(event: SyncEvent): void {
		const listeners = this.listeners.get(event.type);
		if (listeners) {
			for (const callback of listeners) {
				try {
					callback(event);
				} catch (error) {
					console.error('[SyncManager] Error in event listener:', error);
				}
			}
		}
	}

	// ===========================================================================
	// Network Status
	// ===========================================================================

	private setupNetworkListeners(): void {
		window.addEventListener('online', this.handleOnline);
		window.addEventListener('offline', this.handleOffline);
	}

	private handleOnline = (): void => {
		this.status.isOnline = true;
		this.status.consecutiveFailures = 0;

		if (import.meta.env.DEV) {
			console.debug('[SyncManager] Network online');
		}

		if (this.options.autoSyncOnOnline) {
			// Delay slightly to allow network to stabilize
			setTimeout(() => this.syncNow(), 1000);
		}
	};

	private handleOffline = (): void => {
		this.status.isOnline = false;

		if (import.meta.env.DEV) {
			console.debug('[SyncManager] Network offline');
		}

		// Abort any in-progress sync
		if (this.abortController) {
			this.abortController.abort();
		}
	};

	// ===========================================================================
	// Periodic Sync
	// ===========================================================================

	private startPeriodicSync(): void {
		if (this.syncTimer) {
			clearInterval(this.syncTimer);
		}

		this.syncTimer = setInterval(() => {
			if (this.status.isOnline && !this.status.isSyncing) {
				this.syncNow().catch((error) => {
					if (import.meta.env.DEV) {
						console.error('[SyncManager] Periodic sync failed:', error);
					}
				});
			}
		}, this.options.syncInterval);
	}

	/**
	 * Stops periodic sync
	 */
	stopPeriodicSync(): void {
		if (this.syncTimer) {
			clearInterval(this.syncTimer);
			this.syncTimer = null;
		}
	}

	// ===========================================================================
	// Queue Management
	// ===========================================================================

	/**
	 * Queue a change for syncing
	 */
	async queueChange(
		draft: Draft,
		operation: PendingChange['operation'],
		priority: number = 5
	): Promise<string> {
		const change: PendingChange = {
			id: crypto.randomUUID(),
			draftId: draft.id,
			postId: draft.postId,
			operation,
			data: { ...draft },
			previousChecksum: draft.checksum,
			createdAt: Date.now(),
			attempts: 0,
			lastAttemptAt: null,
			error: null,
			priority
		};

		await blogEditorDB.savePendingChange(change);
		await this.updatePendingCount();

		// Attempt immediate sync if online
		if (this.status.isOnline && !this.status.isSyncing) {
			this.syncNow().catch(() => {
				// Errors handled via events
			});
		}

		return change.id;
	}

	/**
	 * Remove a change from the queue
	 */
	async dequeueChange(changeId: string): Promise<void> {
		await blogEditorDB.deletePendingChange(changeId);
		await this.updatePendingCount();
	}

	/**
	 * Clear all pending changes
	 */
	async clearQueue(): Promise<void> {
		await blogEditorDB.clearPendingChanges();
		await this.updatePendingCount();
	}

	/**
	 * Update pending count in status
	 */
	private async updatePendingCount(): Promise<void> {
		const changes = await blogEditorDB.getAllPendingChanges();
		this.status.pendingCount = changes.length;
	}

	// ===========================================================================
	// Synchronization
	// ===========================================================================

	/**
	 * Trigger immediate sync
	 */
	async syncNow(): Promise<void> {
		// Prevent concurrent syncs
		if (this.syncPromise) {
			return this.syncPromise;
		}

		if (!this.status.isOnline) {
			return;
		}

		this.syncPromise = this.performSync();

		try {
			await this.syncPromise;
		} finally {
			this.syncPromise = null;
		}
	}

	/**
	 * Performs the actual sync operation
	 */
	private async performSync(): Promise<void> {
		const changes = await blogEditorDB.getAllPendingChanges();

		if (changes.length === 0) {
			return;
		}

		this.status.isSyncing = true;
		this.abortController = new AbortController();

		const startTime = Date.now();
		let synced = 0;
		let failed = 0;

		this.emit({
			type: 'sync-start',
			pendingCount: changes.length,
			timestamp: startTime
		});

		// Process changes in batches
		for (let i = 0; i < changes.length; i += this.options.batchSize) {
			if (this.abortController.signal.aborted) {
				break;
			}

			const batch = changes.slice(i, i + this.options.batchSize);
			const results = await Promise.allSettled(
				batch.map((change) => this.syncChange(change))
			);

			for (let j = 0; j < results.length; j++) {
				const result = results[j];
				const change = batch[j];

				if (result.status === 'fulfilled' && result.value.success) {
					synced++;
					await this.dequeueChange(change.id);
				} else {
					failed++;
					const error =
						result.status === 'rejected'
							? String(result.reason)
							: result.value.error ?? 'Unknown error';

					await this.handleSyncError(change, error);
				}
			}
		}

		const duration = Date.now() - startTime;
		this.status.isSyncing = false;
		this.status.lastSyncedAt = Date.now();
		this.abortController = null;

		if (failed === 0) {
			this.status.consecutiveFailures = 0;
			this.status.lastError = null;
		} else {
			this.status.consecutiveFailures++;
		}

		this.emit({
			type: 'sync-complete',
			synced,
			failed,
			duration,
			timestamp: Date.now()
		});

		await this.updatePendingCount();
	}

	/**
	 * Sync a single change
	 */
	private async syncChange(change: PendingChange): Promise<SyncResult> {
		const delay = this.calculateBackoff(change.attempts);

		// Wait before retrying if this is a retry
		if (change.attempts > 0) {
			await this.sleep(delay);
		}

		try {
			const result = await this.sendToServer(change);

			if (result.conflict) {
				await this.handleConflict(change, result);
				return { success: false, changeId: change.id, conflict: true };
			}

			return { success: true, changeId: change.id, serverData: result.serverData };
		} catch (error) {
			return {
				success: false,
				changeId: change.id,
				error: error instanceof Error ? error.message : String(error)
			};
		}
	}

	/**
	 * Send change to server
	 */
	private async sendToServer(
		change: PendingChange
	): Promise<{ serverData?: unknown; conflict?: boolean; serverVersion?: number }> {
		const endpoints: Record<PendingChange['operation'], { method: string; path: string }> = {
			create: { method: 'POST', path: '/posts' },
			update: { method: 'PUT', path: `/posts/${change.postId}` },
			delete: { method: 'DELETE', path: `/posts/${change.postId}` },
			publish: { method: 'POST', path: `/posts/${change.postId}/publish` },
			unpublish: { method: 'POST', path: `/posts/${change.postId}/unpublish` }
		};

		const endpoint = endpoints[change.operation];
		const url = `${this.options.apiBaseUrl}${endpoint.path}`;

		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		// Add version header for conflict detection
		if (change.data.version) {
			headers['X-Draft-Version'] = String(change.data.version);
		}

		if (change.previousChecksum) {
			headers['X-Draft-Checksum'] = change.previousChecksum;
		}

		const response = await fetch(url, {
			method: endpoint.method,
			headers,
			credentials: 'include',
			signal: this.abortController?.signal,
			body: endpoint.method !== 'DELETE' ? JSON.stringify(this.preparePayload(change)) : undefined
		});

		if (response.status === 409) {
			// Conflict detected
			const serverData = await response.json();
			return {
				conflict: true,
				serverData: serverData.data,
				serverVersion: serverData.version
			};
		}

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw new Error(errorData.message ?? `HTTP ${response.status}`);
		}

		const data = await response.json();
		return { serverData: data.data ?? data };
	}

	/**
	 * Prepare payload for API request
	 */
	private preparePayload(change: PendingChange): Record<string, unknown> {
		const { data } = change;
		return {
			title: data.title,
			slug: data.slug,
			excerpt: data.excerpt,
			content_blocks: data.content_blocks,
			featured_image: data.featured_image,
			meta_title: data.meta_title,
			meta_description: data.meta_description,
			canonical_url: data.canonical_url,
			schema_markup: data.schema_markup,
			indexable: data.indexable
		};
	}

	/**
	 * Handle sync error with retry logic
	 */
	private async handleSyncError(change: PendingChange, error: string): Promise<void> {
		const willRetry = change.attempts < this.options.maxRetries;

		change.attempts++;
		change.lastAttemptAt = Date.now();
		change.error = error;

		await blogEditorDB.savePendingChange(change);

		this.status.lastError = error;

		this.emit({
			type: 'sync-error',
			changeId: change.id,
			error,
			attempt: change.attempts,
			willRetry,
			timestamp: Date.now()
		});
	}

	/**
	 * Handle conflict resolution
	 */
	private async handleConflict(
		change: PendingChange,
		result: { serverData?: unknown; serverVersion?: number }
	): Promise<void> {
		const draft = await blogEditorDB.getDraft(change.draftId);
		if (!draft) {
			return;
		}

		// Create backup of local version
		const backup = createRevisionFromDraft(draft, 'manual', 'system');
		backup.title = `[Conflict Backup] ${draft.title}`;
		await blogEditorDB.saveRevision(backup);

		// Server wins by default - update local with server version
		const serverData = result.serverData as Partial<Draft>;
		if (serverData) {
			const updatedDraft: Draft = {
				...draft,
				...serverData,
				content_blocks: (serverData.content_blocks as Block[]) ?? draft.content_blocks,
				version: result.serverVersion ?? draft.version + 1,
				checksum: generateChecksum(serverData.content_blocks ?? draft.content_blocks),
				syncedAt: Date.now()
			};

			await blogEditorDB.saveDraft(updatedDraft);
		}

		// Remove the conflicting change
		await this.dequeueChange(change.id);

		this.emit({
			type: 'conflict',
			changeId: change.id,
			draftId: change.draftId,
			localVersion: draft.version,
			serverVersion: result.serverVersion ?? 0,
			resolution: 'server-wins',
			backupCreated: true,
			timestamp: Date.now()
		});
	}

	/**
	 * Calculate exponential backoff delay
	 */
	private calculateBackoff(attempt: number): number {
		const delay = this.options.baseDelay * Math.pow(2, attempt);
		const jitter = Math.random() * 0.3 * delay; // Add 0-30% jitter
		return Math.min(delay + jitter, this.options.maxDelay);
	}

	/**
	 * Sleep utility
	 */
	private sleep(ms: number): Promise<void> {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	// ===========================================================================
	// Status & Getters
	// ===========================================================================

	/**
	 * Get current sync status
	 */
	getStatus(): SyncStatus {
		return { ...this.status };
	}

	/**
	 * Check if currently online
	 */
	isOnline(): boolean {
		return this.status.isOnline;
	}

	/**
	 * Check if sync is in progress
	 */
	isSyncing(): boolean {
		return this.status.isSyncing;
	}

	/**
	 * Get pending changes count
	 */
	getPendingCount(): number {
		return this.status.pendingCount;
	}

	/**
	 * Get last sync timestamp
	 */
	getLastSyncedAt(): number | null {
		return this.status.lastSyncedAt;
	}

	// ===========================================================================
	// Cleanup
	// ===========================================================================

	/**
	 * Cleanup and dispose
	 */
	dispose(): void {
		this.stopPeriodicSync();

		if (browser) {
			window.removeEventListener('online', this.handleOnline);
			window.removeEventListener('offline', this.handleOffline);
		}

		if (this.abortController) {
			this.abortController.abort();
		}

		// Clear all listeners
		for (const listeners of this.listeners.values()) {
			listeners.clear();
		}
	}
}

// =============================================================================
// Singleton Export
// =============================================================================

export const syncManager = new SyncManager();

// =============================================================================
// Sync Status Indicator Component Helper
// =============================================================================

export interface SyncIndicatorState {
	status: 'synced' | 'syncing' | 'pending' | 'error' | 'offline';
	message: string;
	pendingCount: number;
	lastSyncedAt: Date | null;
}

/**
 * Get sync indicator state for UI display
 */
export function getSyncIndicatorState(): SyncIndicatorState {
	const status = syncManager.getStatus();

	if (!status.isOnline) {
		return {
			status: 'offline',
			message: 'You are offline. Changes will sync when reconnected.',
			pendingCount: status.pendingCount,
			lastSyncedAt: status.lastSyncedAt ? new Date(status.lastSyncedAt) : null
		};
	}

	if (status.isSyncing) {
		return {
			status: 'syncing',
			message: `Syncing ${status.pendingCount} change${status.pendingCount !== 1 ? 's' : ''}...`,
			pendingCount: status.pendingCount,
			lastSyncedAt: status.lastSyncedAt ? new Date(status.lastSyncedAt) : null
		};
	}

	if (status.lastError && status.consecutiveFailures > 0) {
		return {
			status: 'error',
			message: `Sync failed: ${status.lastError}. Retrying...`,
			pendingCount: status.pendingCount,
			lastSyncedAt: status.lastSyncedAt ? new Date(status.lastSyncedAt) : null
		};
	}

	if (status.pendingCount > 0) {
		return {
			status: 'pending',
			message: `${status.pendingCount} unsaved change${status.pendingCount !== 1 ? 's' : ''}`,
			pendingCount: status.pendingCount,
			lastSyncedAt: status.lastSyncedAt ? new Date(status.lastSyncedAt) : null
		};
	}

	return {
		status: 'synced',
		message: status.lastSyncedAt
			? `Last saved ${formatRelativeTime(status.lastSyncedAt)}`
			: 'All changes saved',
		pendingCount: 0,
		lastSyncedAt: status.lastSyncedAt ? new Date(status.lastSyncedAt) : null
	};
}

/**
 * Format relative time for display
 */
function formatRelativeTime(timestamp: number): string {
	const now = Date.now();
	const diff = now - timestamp;

	if (diff < 60000) {
		return 'just now';
	}

	if (diff < 3600000) {
		const minutes = Math.floor(diff / 60000);
		return `${minutes} minute${minutes !== 1 ? 's' : ''} ago`;
	}

	if (diff < 86400000) {
		const hours = Math.floor(diff / 3600000);
		return `${hours} hour${hours !== 1 ? 's' : ''} ago`;
	}

	const days = Math.floor(diff / 86400000);
	return `${days} day${days !== 1 ? 's' : ''} ago`;
}

// =============================================================================
// Background Sync Registration (Service Worker)
// =============================================================================

/**
 * Register for background sync if supported
 */
export async function registerBackgroundSync(): Promise<boolean> {
	if (!browser || !('serviceWorker' in navigator) || !('sync' in ServiceWorkerRegistration.prototype)) {
		return false;
	}

	try {
		const registration = await navigator.serviceWorker.ready;
		// @ts-expect-error - Background sync API types
		await registration.sync.register('blog-editor-sync');
		return true;
	} catch (error) {
		if (import.meta.env.DEV) {
			console.warn('[SyncManager] Background sync registration failed:', error);
		}
		return false;
	}
}

/**
 * Check if background sync is supported
 */
export function isBackgroundSyncSupported(): boolean {
	return browser && 'serviceWorker' in navigator && 'sync' in ServiceWorkerRegistration.prototype;
}
