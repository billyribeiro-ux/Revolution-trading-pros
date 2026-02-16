/**
 * Revolution Trading Pros - Blog Editor Offline Support
 * ======================================================
 * Main entry point for offline support system.
 * Exports Svelte 5 reactive stores and composable hooks.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

import { browser } from '$app/environment';
import type { Post } from '$lib/types/post';
import {
	blogEditorDB,
	createDraftFromPost,
	createNewDraft,
	createRevisionFromDraft,
	generateChecksum,
	type Draft,
	type PendingChange,
	type CachedAsset,
	type StoredRevision,
	type DatabaseStats,
	type CleanupResult
} from './db';
import {
	syncManager,
	getSyncIndicatorState,
	registerBackgroundSync,
	isBackgroundSyncSupported,
	type SyncEvent,
	type SyncEventType,
	type SyncStatus,
	type SyncIndicatorState
} from './sync';
import { logger } from '$lib/utils/logger';

// =============================================================================
// Re-exports
// =============================================================================

export {
	// Database
	blogEditorDB,
	createDraftFromPost,
	createNewDraft,
	createRevisionFromDraft,
	generateChecksum,
	type Draft,
	type PendingChange,
	type CachedAsset,
	type StoredRevision,
	type DatabaseStats,
	type CleanupResult,
	// Sync
	syncManager,
	getSyncIndicatorState,
	registerBackgroundSync,
	isBackgroundSyncSupported,
	type SyncEvent,
	type SyncEventType,
	type SyncStatus,
	type SyncIndicatorState
};

// =============================================================================
// Svelte 5 Reactive State Class
// =============================================================================

type Subscriber<T> = (value: T) => void;
type Unsubscriber = () => void;

/**
 * Offline state snapshot for subscribe compatibility
 */
interface OfflineStateSnapshot {
	isOffline: boolean;
	pendingChanges: number;
	lastSynced: Date | null;
	isSyncing: boolean;
	syncError: string | null;
	indicatorState: SyncIndicatorState;
}

/**
 * Offline Editor Store Class - Svelte 5 Runes
 * Manages offline state with reactive properties using $state
 */
class OfflineEditorStore {
	// Reactive state using $state runes
	private _isOffline = $state(!browser || !navigator.onLine);
	private _pendingChanges = $state(0);
	private _lastSynced = $state<Date | null>(null);
	private _isSyncing = $state(false);
	private _syncError = $state<string | null>(null);
	private _indicatorState = $state<SyncIndicatorState>(getSyncIndicatorState());

	// Subscribers for backward compatibility
	private subscribers = new Set<Subscriber<OfflineStateSnapshot>>();

	// Event listener cleanup
	private cleanupFns: (() => void)[] = [];

	constructor() {
		if (browser) {
			this.initialize();
		}
	}

	/**
	 * Initialize listeners and sync state
	 */
	private async initialize(): Promise<void> {
		// Network status listeners
		const handleOnline = () => {
			this._isOffline = false;
			this.updateIndicatorState();
			this.notifySubscribers();
		};

		const handleOffline = () => {
			this._isOffline = true;
			this.updateIndicatorState();
			this.notifySubscribers();
		};

		window.addEventListener('online', handleOnline);
		window.addEventListener('offline', handleOffline);
		this.cleanupFns.push(() => {
			window.removeEventListener('online', handleOnline);
			window.removeEventListener('offline', handleOffline);
		});

		// Sync manager event listeners
		const handleSyncStart = () => {
			this._isSyncing = true;
			this.updateIndicatorState();
			this.notifySubscribers();
		};

		const handleSyncComplete = (event: SyncEvent) => {
			if (event.type === 'sync-complete') {
				this._isSyncing = false;
				this._lastSynced = new Date(event.timestamp);
				this._syncError = null;
				this.updatePendingCount();
				this.updateIndicatorState();
				this.notifySubscribers();
			}
		};

		const handleSyncError = (event: SyncEvent) => {
			if (event.type === 'sync-error') {
				this._syncError = event.error;
				this.updateIndicatorState();
				this.notifySubscribers();
			}
		};

		const handleConflict = (event: SyncEvent) => {
			if (event.type === 'conflict') {
				this.updatePendingCount();
				this.updateIndicatorState();
				this.notifySubscribers();
			}
		};

		const unsubStart = syncManager.on('sync-start', handleSyncStart);
		const unsubComplete = syncManager.on('sync-complete', handleSyncComplete);
		const unsubError = syncManager.on('sync-error', handleSyncError);
		const unsubConflict = syncManager.on('conflict', handleConflict);

		this.cleanupFns.push(unsubStart, unsubComplete, unsubError, unsubConflict);

		// Initial state update
		await this.updatePendingCount();
		this.updateIndicatorState();

		// Register background sync
		await registerBackgroundSync();
	}

	/**
	 * Update pending changes count from database
	 */
	private async updatePendingCount(): Promise<void> {
		try {
			const changes = await blogEditorDB.getAllPendingChanges();
			this._pendingChanges = changes.length;
		} catch (error) {
			logger.error('[OfflineEditorStore] Failed to get pending count:', error);
		}
	}

	/**
	 * Update sync indicator state
	 */
	private updateIndicatorState(): void {
		this._indicatorState = getSyncIndicatorState();
	}

	// ===========================================================================
	// Subscribe for Svelte Store Compatibility
	// ===========================================================================

	/**
	 * Subscribe to state changes (Svelte store contract)
	 */
	subscribe(fn: Subscriber<OfflineStateSnapshot>): Unsubscriber {
		this.subscribers.add(fn);
		fn(this.getSnapshot());
		return () => {
			this.subscribers.delete(fn);
		};
	}

	private getSnapshot(): OfflineStateSnapshot {
		return {
			isOffline: this._isOffline,
			pendingChanges: this._pendingChanges,
			lastSynced: this._lastSynced,
			isSyncing: this._isSyncing,
			syncError: this._syncError,
			indicatorState: this._indicatorState
		};
	}

	private notifySubscribers(): void {
		const snapshot = this.getSnapshot();
		for (const fn of this.subscribers) {
			try {
				fn(snapshot);
			} catch (error) {
				logger.error('[OfflineEditorStore] Error in subscriber:', error);
			}
		}
	}

	// ===========================================================================
	// Getters (Reactive in Svelte 5)
	// ===========================================================================

	get isOffline(): boolean {
		return this._isOffline;
	}

	get pendingChanges(): number {
		return this._pendingChanges;
	}

	get lastSynced(): Date | null {
		return this._lastSynced;
	}

	get isSyncing(): boolean {
		return this._isSyncing;
	}

	get syncError(): string | null {
		return this._syncError;
	}

	get indicatorState(): SyncIndicatorState {
		return this._indicatorState;
	}

	// ===========================================================================
	// Actions
	// ===========================================================================

	/**
	 * Trigger immediate sync
	 */
	async syncNow(): Promise<void> {
		if (this._isOffline) {
			throw new Error('Cannot sync while offline');
		}
		await syncManager.syncNow();
	}

	/**
	 * Save a draft (from Post or existing Draft)
	 */
	async saveDraft(postOrDraft: Post | Draft): Promise<Draft> {
		let draft: Draft;

		if ('content_blocks' in postOrDraft && 'checksum' in postOrDraft) {
			// It's already a Draft
			draft = postOrDraft as Draft;
			draft.updatedAt = Date.now();
			draft.checksum = generateChecksum(draft.content_blocks);
			draft.version++;
		} else {
			// It's a Post, convert to Draft
			const existing = postOrDraft.id ? await blogEditorDB.getDraftByPostId(postOrDraft.id) : null;

			if (existing) {
				draft = {
					...existing,
					title: postOrDraft.title,
					slug: postOrDraft.slug,
					excerpt: postOrDraft.excerpt,
					content_blocks: postOrDraft.content_blocks ?? [],
					featured_image: postOrDraft.featured_image,
					meta_title: postOrDraft.meta_title ?? null,
					meta_description: postOrDraft.meta_description ?? null,
					canonical_url: postOrDraft.canonical_url ?? null,
					schema_markup: postOrDraft.schema_markup ?? null,
					indexable: postOrDraft.indexable ?? true,
					updatedAt: Date.now(),
					version: existing.version + 1,
					checksum: generateChecksum(postOrDraft.content_blocks)
				};
			} else {
				draft = createDraftFromPost(postOrDraft);
			}
		}

		// Save to IndexedDB
		await blogEditorDB.saveDraft(draft);

		// Queue for sync
		const operation = draft.postId ? 'update' : 'create';
		await syncManager.queueChange(draft, operation);

		// Update state
		await this.updatePendingCount();
		this.notifySubscribers();

		return draft;
	}

	/**
	 * Load a draft by ID
	 */
	async loadDraft(id: string): Promise<Draft | null> {
		return blogEditorDB.getDraft(id);
	}

	/**
	 * Load a draft by post ID
	 */
	async loadDraftByPostId(postId: number): Promise<Draft | null> {
		return blogEditorDB.getDraftByPostId(postId);
	}

	/**
	 * Get all drafts
	 */
	async getAllDrafts(): Promise<Draft[]> {
		return blogEditorDB.getAllDrafts();
	}

	/**
	 * Delete a draft
	 */
	async deleteDraft(id: string): Promise<void> {
		const draft = await blogEditorDB.getDraft(id);
		if (draft) {
			await blogEditorDB.deleteDraft(id);

			// Queue delete for sync if it has a server ID
			if (draft.postId) {
				await syncManager.queueChange(draft, 'delete', 10); // High priority
			}

			await this.updatePendingCount();
			this.notifySubscribers();
		}
	}

	/**
	 * Clear all drafts
	 */
	async clearDrafts(): Promise<void> {
		await blogEditorDB.clearDrafts();
		await this.updatePendingCount();
		this.notifySubscribers();
	}

	/**
	 * Create a new empty draft
	 */
	async createNewDraft(authorId: number | null = null): Promise<Draft> {
		const draft = createNewDraft(authorId);
		await blogEditorDB.saveDraft(draft);
		return draft;
	}

	/**
	 * Save a revision
	 */
	async saveRevision(
		draft: Draft,
		type: StoredRevision['type'] = 'auto',
		userId: string = 'system'
	): Promise<StoredRevision> {
		const revision = createRevisionFromDraft(draft, type, userId);
		await blogEditorDB.saveRevision(revision);
		return revision;
	}

	/**
	 * Get revisions for a draft
	 */
	async getRevisions(draftId: string): Promise<StoredRevision[]> {
		return blogEditorDB.getRevisionsByDraftId(draftId);
	}

	/**
	 * Restore a revision
	 */
	async restoreRevision(revisionId: string): Promise<Draft | null> {
		const revision = await blogEditorDB.getRevision(revisionId);
		if (!revision) {
			return null;
		}

		const draft = await blogEditorDB.getDraft(revision.draftId);
		if (!draft) {
			return null;
		}

		// Create backup of current state
		await this.saveRevision(draft, 'manual', 'system');

		// Restore from revision
		draft.content_blocks = revision.blocks;
		draft.updatedAt = Date.now();
		draft.version++;
		draft.checksum = generateChecksum(revision.blocks);

		await blogEditorDB.saveDraft(draft);
		await syncManager.queueChange(draft, 'update');

		await this.updatePendingCount();
		this.notifySubscribers();

		return draft;
	}

	/**
	 * Cache an asset
	 */
	async cacheAsset(url: string, blob: Blob, mimeType: string): Promise<CachedAsset> {
		const asset: CachedAsset = {
			id: crypto.randomUUID(),
			url,
			mimeType,
			blob,
			size: blob.size,
			createdAt: Date.now(),
			accessedAt: Date.now(),
			draftIds: []
		};

		await blogEditorDB.saveAsset(asset);
		return asset;
	}

	/**
	 * Get cached asset by URL
	 */
	async getCachedAsset(url: string): Promise<CachedAsset | null> {
		return blogEditorDB.getAssetByUrl(url);
	}

	/**
	 * Get database statistics
	 */
	async getStats(): Promise<DatabaseStats> {
		return blogEditorDB.getStats();
	}

	/**
	 * Run manual cleanup
	 */
	async cleanup(): Promise<CleanupResult> {
		const result = await blogEditorDB.cleanup();
		await this.updatePendingCount();
		this.notifySubscribers();
		return result;
	}

	/**
	 * Export all data
	 */
	async exportData(): Promise<string> {
		const data = await blogEditorDB.exportData();
		return JSON.stringify(data, null, 2);
	}

	/**
	 * Dispose and cleanup
	 */
	dispose(): void {
		for (const cleanup of this.cleanupFns) {
			cleanup();
		}
		this.cleanupFns = [];
		this.subscribers.clear();
		syncManager.dispose();
	}
}

// =============================================================================
// Singleton Store Instance
// =============================================================================

export const offlineEditorStore = new OfflineEditorStore();

// =============================================================================
// Derived Stores for Convenience
// =============================================================================

/**
 * Reactive isOffline store
 */
export const isOffline = {
	get current() {
		return offlineEditorStore.isOffline;
	},
	subscribe(fn: (value: boolean) => void) {
		return offlineEditorStore.subscribe((snapshot) => fn(snapshot.isOffline));
	}
};

/**
 * Reactive pendingChanges store
 */
export const pendingChanges = {
	get current() {
		return offlineEditorStore.pendingChanges;
	},
	subscribe(fn: (value: number) => void) {
		return offlineEditorStore.subscribe((snapshot) => fn(snapshot.pendingChanges));
	}
};

/**
 * Reactive lastSynced store
 */
export const lastSynced = {
	get current() {
		return offlineEditorStore.lastSynced;
	},
	subscribe(fn: (value: Date | null) => void) {
		return offlineEditorStore.subscribe((snapshot) => fn(snapshot.lastSynced));
	}
};

/**
 * Reactive isSyncing store
 */
export const isSyncing = {
	get current() {
		return offlineEditorStore.isSyncing;
	},
	subscribe(fn: (value: boolean) => void) {
		return offlineEditorStore.subscribe((snapshot) => fn(snapshot.isSyncing));
	}
};

/**
 * Reactive syncIndicator store
 */
export const syncIndicator = {
	get current() {
		return offlineEditorStore.indicatorState;
	},
	subscribe(fn: (value: SyncIndicatorState) => void) {
		return offlineEditorStore.subscribe((snapshot) => fn(snapshot.indicatorState));
	}
};

// =============================================================================
// useOfflineEditor Hook
// =============================================================================

export interface OfflineEditorHook {
	/** Whether the browser is offline */
	readonly isOffline: boolean;
	/** Number of pending changes waiting to sync */
	readonly pendingChanges: number;
	/** Last successful sync timestamp */
	readonly lastSynced: Date | null;
	/** Whether sync is currently in progress */
	readonly isSyncing: boolean;
	/** Current sync error if any */
	readonly syncError: string | null;
	/** Sync indicator state for UI display */
	readonly indicatorState: SyncIndicatorState;
	/** Trigger immediate sync */
	syncNow(): Promise<void>;
	/** Save a post as a draft */
	saveDraft(post: Post): Promise<Draft>;
	/** Load a draft by ID */
	loadDraft(id: string): Promise<Draft | null>;
	/** Load a draft by post ID */
	loadDraftByPostId(postId: number): Promise<Draft | null>;
	/** Delete a draft */
	deleteDraft(id: string): Promise<void>;
	/** Clear all drafts */
	clearDrafts(): Promise<void>;
	/** Create a new empty draft */
	createNewDraft(authorId?: number | null): Promise<Draft>;
	/** Get all drafts */
	getAllDrafts(): Promise<Draft[]>;
	/** Save a revision */
	saveRevision(draft: Draft, type?: StoredRevision['type']): Promise<StoredRevision>;
	/** Get revisions for a draft */
	getRevisions(draftId: string): Promise<StoredRevision[]>;
	/** Restore a revision */
	restoreRevision(revisionId: string): Promise<Draft | null>;
	/** Cache an asset for offline use */
	cacheAsset(url: string, blob: Blob, mimeType: string): Promise<CachedAsset>;
	/** Get a cached asset */
	getCachedAsset(url: string): Promise<CachedAsset | null>;
	/** Get database statistics */
	getStats(): Promise<DatabaseStats>;
	/** Run manual cleanup */
	cleanup(): Promise<CleanupResult>;
	/** Export all data as JSON */
	exportData(): Promise<string>;
	/** Subscribe to sync events */
	onSyncEvent(type: SyncEventType, callback: (event: SyncEvent) => void): () => void;
}

/**
 * Composable hook for offline editor functionality
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useOfflineEditor } from '$lib/components/blog/BlockEditor/offline';
 *
 *   const offline = useOfflineEditor();
 *
 *   async function handleSave() {
 *     await offline.saveDraft(currentPost);
 *   }
 * </script>
 *
 * {#if offline.isOffline}
 *   <span>Offline - {offline.pendingChanges} pending</span>
 * {/if}
 * ```
 */
export function useOfflineEditor(): OfflineEditorHook {
	return {
		get isOffline() {
			return offlineEditorStore.isOffline;
		},
		get pendingChanges() {
			return offlineEditorStore.pendingChanges;
		},
		get lastSynced() {
			return offlineEditorStore.lastSynced;
		},
		get isSyncing() {
			return offlineEditorStore.isSyncing;
		},
		get syncError() {
			return offlineEditorStore.syncError;
		},
		get indicatorState() {
			return offlineEditorStore.indicatorState;
		},
		syncNow: () => offlineEditorStore.syncNow(),
		saveDraft: (post: Post) => offlineEditorStore.saveDraft(post),
		loadDraft: (id: string) => offlineEditorStore.loadDraft(id),
		loadDraftByPostId: (postId: number) => offlineEditorStore.loadDraftByPostId(postId),
		deleteDraft: (id: string) => offlineEditorStore.deleteDraft(id),
		clearDrafts: () => offlineEditorStore.clearDrafts(),
		createNewDraft: (authorId?: number | null) => offlineEditorStore.createNewDraft(authorId),
		getAllDrafts: () => offlineEditorStore.getAllDrafts(),
		saveRevision: (draft: Draft, type?: StoredRevision['type']) =>
			offlineEditorStore.saveRevision(draft, type),
		getRevisions: (draftId: string) => offlineEditorStore.getRevisions(draftId),
		restoreRevision: (revisionId: string) => offlineEditorStore.restoreRevision(revisionId),
		cacheAsset: (url: string, blob: Blob, mimeType: string) =>
			offlineEditorStore.cacheAsset(url, blob, mimeType),
		getCachedAsset: (url: string) => offlineEditorStore.getCachedAsset(url),
		getStats: () => offlineEditorStore.getStats(),
		cleanup: () => offlineEditorStore.cleanup(),
		exportData: () => offlineEditorStore.exportData(),
		onSyncEvent: (type: SyncEventType, callback: (event: SyncEvent) => void) =>
			syncManager.on(type, callback)
	};
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Format bytes to human readable string
 */
export function formatBytes(bytes: number): string {
	if (bytes === 0) return '0 B';

	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));

	return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

/**
 * Check if offline support is available
 */
export function isOfflineSupportAvailable(): boolean {
	return browser && 'indexedDB' in window;
}

/**
 * Get storage quota info
 */
export async function getStorageQuota(): Promise<{
	used: number;
	quota: number;
	percentUsed: number;
} | null> {
	if (!browser || !('storage' in navigator) || !('estimate' in navigator.storage)) {
		return null;
	}

	try {
		const estimate = await navigator.storage.estimate();
		const used = estimate.usage ?? 0;
		const quota = estimate.quota ?? 0;
		const percentUsed = quota > 0 ? (used / quota) * 100 : 0;

		return { used, quota, percentUsed };
	} catch {
		return null;
	}
}

/**
 * Request persistent storage
 */
export async function requestPersistentStorage(): Promise<boolean> {
	if (!browser || !('storage' in navigator) || !('persist' in navigator.storage)) {
		return false;
	}

	try {
		return await navigator.storage.persist();
	} catch {
		return false;
	}
}

/**
 * Check if storage is persistent
 */
export async function isStoragePersistent(): Promise<boolean> {
	if (!browser || !('storage' in navigator) || !('persisted' in navigator.storage)) {
		return false;
	}

	try {
		return await navigator.storage.persisted();
	} catch {
		return false;
	}
}
