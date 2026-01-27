/**
 * CMS Autosave Store - Apple ICT 7+ Principal Engineer Grade
 * ===========================================================
 * SVELTE 5 RUNES VERSION
 *
 * Robust autosave implementation with:
 * - IndexedDB persistence for offline support
 * - Debounced saves to prevent excessive writes
 * - Draft recovery on page reload
 * - Conflict detection with server
 * - Offline queue with automatic sync
 * - Exponential backoff retry logic
 * - Multi-tab coordination via BroadcastChannel
 *
 * @version 3.0.0 - January 2026
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import type { EditorContent } from './editor.svelte';

// =============================================================================
// Constants
// =============================================================================

const DB_NAME = 'rtp-cms-autosave';
const DB_VERSION = 2; // Upgraded for sync queue store
const DRAFTS_STORE = 'drafts';
const SYNC_QUEUE_STORE = 'syncQueue';
const AUTOSAVE_DELAY_MS = 2000; // 2 seconds debounce
const MAX_DRAFTS = 50; // Maximum number of drafts to keep
const MAX_RETRY_ATTEMPTS = 5;
const BASE_RETRY_DELAY_MS = 1000; // 1 second base delay
const MAX_RETRY_DELAY_MS = 60000; // 1 minute max delay
const SYNC_CHANNEL_NAME = 'rtp-cms-sync';
const VERSION_CHECK_TIMEOUT_MS = 10000; // 10 second timeout for version checks

// =============================================================================
// Type Definitions
// =============================================================================

export interface AutosaveDraft {
	id: string;
	contentId: string;
	content: EditorContent;
	savedAt: number;
	version: number;
	serverVersion?: number; // Last known server version when draft was created
	checksum?: string; // Content checksum for change detection
}

export interface SyncQueueItem {
	id: string;
	contentId: string;
	content: EditorContent;
	queuedAt: number;
	retryCount: number;
	nextRetryAt: number;
	lastError?: string;
	status: 'pending' | 'syncing' | 'failed';
}

export interface ConflictData {
	contentId: string;
	localVersion: number;
	serverVersion: number;
	localContent: EditorContent;
	serverContent: EditorContent;
	localModifiedAt: number;
	serverModifiedAt: number;
	detectedAt: number;
}

export interface ServerVersionResponse {
	version: number;
	updatedAt: string;
	updatedBy?: string;
	checksum?: string;
}

export interface SyncResponse {
	success: boolean;
	contentId: string;
	newVersion?: number;
	error?: string;
	conflict?: {
		serverVersion: number;
		serverContent: EditorContent;
	};
}

export interface AutosaveState {
	isDraftLoaded: boolean;
	hasDraft: boolean;
	lastSavedAt: Date | null;
	isSaving: boolean;
	error: string | null;
	isOnline: boolean;
	syncQueue: SyncQueueItem[];
	hasConflict: boolean;
	conflictData: ConflictData | null;
	isSyncing: boolean;
	pendingSyncCount: number;
}

export type ConflictResolution = 'mine' | 'theirs' | 'merge';

export interface MergeResult {
	content: EditorContent;
	resolvedFields: string[];
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Generate a simple checksum for content change detection
 */
function generateChecksum(content: EditorContent): string {
	const str = JSON.stringify({
		title: content.title,
		content: content.content,
		contentBlocks: content.contentBlocks,
		excerpt: content.excerpt,
		customFields: content.customFields
	});
	let hash = 0;
	for (let i = 0; i < str.length; i++) {
		const char = str.charCodeAt(i);
		hash = ((hash << 5) - hash) + char;
		hash = hash & hash; // Convert to 32-bit integer
	}
	return hash.toString(16);
}

/**
 * Calculate exponential backoff delay
 */
function calculateBackoffDelay(retryCount: number): number {
	const delay = BASE_RETRY_DELAY_MS * Math.pow(2, retryCount);
	// Add jitter (0-25% of delay)
	const jitter = delay * Math.random() * 0.25;
	return Math.min(delay + jitter, MAX_RETRY_DELAY_MS);
}

/**
 * Deep merge two content objects for conflict resolution
 */
function mergeContent(local: EditorContent, server: EditorContent): MergeResult {
	const resolvedFields: string[] = [];

	// Start with server content as base
	const merged: EditorContent = { ...server };

	// Prefer local changes for content-specific fields
	if (local.title !== server.title) {
		merged.title = local.title;
		resolvedFields.push('title');
	}
	if (local.content !== server.content) {
		merged.content = local.content;
		resolvedFields.push('content');
	}
	if (JSON.stringify(local.contentBlocks) !== JSON.stringify(server.contentBlocks)) {
		merged.contentBlocks = local.contentBlocks;
		resolvedFields.push('contentBlocks');
	}
	if (local.excerpt !== server.excerpt) {
		merged.excerpt = local.excerpt;
		resolvedFields.push('excerpt');
	}

	// Keep server metadata (version, status, etc.)
	merged.version = server.version + 1;

	return { content: merged, resolvedFields };
}

// =============================================================================
// IndexedDB Utilities
// =============================================================================

let dbInstance: IDBDatabase | null = null;
let dbInitPromise: Promise<IDBDatabase> | null = null;

async function getDB(): Promise<IDBDatabase> {
	if (!browser) throw new Error('IndexedDB not available in SSR');

	if (dbInstance) return dbInstance;

	if (dbInitPromise) return dbInitPromise;

	dbInitPromise = new Promise((resolve, reject) => {
		const request = indexedDB.open(DB_NAME, DB_VERSION);

		request.onerror = () => {
			console.error('[Autosave] IndexedDB error:', request.error);
			reject(request.error);
		};

		request.onsuccess = () => {
			dbInstance = request.result;
			resolve(dbInstance);
		};

		request.onupgradeneeded = (event) => {
			const db = (event.target as IDBOpenDBRequest).result;

			// Create drafts store if it doesn't exist
			if (!db.objectStoreNames.contains(DRAFTS_STORE)) {
				const store = db.createObjectStore(DRAFTS_STORE, { keyPath: 'id' });
				store.createIndex('contentId', 'contentId', { unique: false });
				store.createIndex('savedAt', 'savedAt', { unique: false });
			}

			// Create sync queue store
			if (!db.objectStoreNames.contains(SYNC_QUEUE_STORE)) {
				const queueStore = db.createObjectStore(SYNC_QUEUE_STORE, { keyPath: 'id' });
				queueStore.createIndex('contentId', 'contentId', { unique: false });
				queueStore.createIndex('status', 'status', { unique: false });
				queueStore.createIndex('nextRetryAt', 'nextRetryAt', { unique: false });
			}
		};
	});

	return dbInitPromise;
}

// =============================================================================
// Autosave Store Class - Svelte 5 Runes
// =============================================================================

class AutosaveStoreClass {
	// Core State
	private _isDraftLoaded = $state(false);
	private _hasDraft = $state(false);
	private _lastSavedAt = $state<Date | null>(null);
	private _isSaving = $state(false);
	private _error = $state<string | null>(null);
	private _currentDraft = $state<AutosaveDraft | null>(null);

	// Offline & Sync State
	private _isOnline = $state(browser ? navigator.onLine : true);
	private _syncQueue = $state<SyncQueueItem[]>([]);
	private _hasConflict = $state(false);
	private _conflictData = $state<ConflictData | null>(null);
	private _isSyncing = $state(false);

	// Internal timers and state
	private saveTimer: ReturnType<typeof setTimeout> | null = null;
	private syncTimer: ReturnType<typeof setTimeout> | null = null;
	private pendingContent: EditorContent | null = null;

	// Multi-tab coordination
	private broadcastChannel: BroadcastChannel | null = null;

	// Subscribers
	private subscribers = new Set<(state: AutosaveState) => void>();

	// ==========================================================================
	// Constructor & Initialization
	// ==========================================================================

	constructor() {
		if (browser) {
			this.initializeOfflineDetection();
			this.initializeBroadcastChannel();
			this.loadSyncQueue();
		}
	}

	/**
	 * Initialize offline detection using navigator.onLine and events
	 */
	private initializeOfflineDetection(): void {
		// Set initial state
		this._isOnline = navigator.onLine;

		// Listen for online/offline events
		window.addEventListener('online', this.handleOnline);
		window.addEventListener('offline', this.handleOffline);

		// Handle page visibility for sync on return
		document.addEventListener('visibilitychange', this.handleVisibilityChange);

		// Handle before unload to persist pending changes
		window.addEventListener('beforeunload', this.handleBeforeUnload);
	}

	/**
	 * Initialize BroadcastChannel for multi-tab coordination
	 */
	private initializeBroadcastChannel(): void {
		try {
			this.broadcastChannel = new BroadcastChannel(SYNC_CHANNEL_NAME);
			this.broadcastChannel.onmessage = this.handleBroadcastMessage;
		} catch (e) {
			// BroadcastChannel not supported
			console.warn('[Autosave] BroadcastChannel not supported');
		}
	}

	/**
	 * Load sync queue from IndexedDB on startup
	 */
	private async loadSyncQueue(): Promise<void> {
		try {
			const db = await getDB();
			const items = await new Promise<SyncQueueItem[]>((resolve, reject) => {
				const transaction = db.transaction(SYNC_QUEUE_STORE, 'readonly');
				const store = transaction.objectStore(SYNC_QUEUE_STORE);
				const request = store.getAll();

				request.onsuccess = () => resolve(request.result || []);
				request.onerror = () => reject(request.error);
			});

			this._syncQueue = items;
			this.notifySubscribers();

			// Start sync if online and have pending items
			if (this._isOnline && items.some(i => i.status === 'pending')) {
				this.scheduleSyncRetry();
			}
		} catch (error) {
			console.error('[Autosave] Failed to load sync queue:', error);
		}
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	private handleOnline = async (): Promise<void> => {
		console.info('[Autosave] Network connection restored');
		this._isOnline = true;
		this.notifySubscribers();
		this.broadcastMessage({ type: 'online' });

		// Auto-sync pending changes
		await this.syncPending();
	};

	private handleOffline = (): void => {
		console.info('[Autosave] Network connection lost');
		this._isOnline = false;
		this.notifySubscribers();
		this.broadcastMessage({ type: 'offline' });

		// Cancel any pending sync timers
		if (this.syncTimer) {
			clearTimeout(this.syncTimer);
			this.syncTimer = null;
		}
	};

	private handleVisibilityChange = async (): Promise<void> => {
		if (document.visibilityState === 'visible' && this._isOnline) {
			// Check for updates when tab becomes visible
			await this.checkForConflicts();
		}
	};

	private handleBeforeUnload = (event: BeforeUnloadEvent): void => {
		// Save any pending content immediately
		if (this.pendingContent) {
			this.saveDraftSync(this.pendingContent);
		}

		// Warn if there are unsynced changes
		if (this._syncQueue.length > 0 || this._hasConflict) {
			event.preventDefault();
			event.returnValue = 'You have unsaved changes. Are you sure you want to leave?';
		}
	};

	private handleBroadcastMessage = (event: MessageEvent): void => {
		const message = event.data;

		switch (message.type) {
			case 'sync_complete':
				// Another tab completed sync, reload queue
				this.loadSyncQueue();
				break;
			case 'conflict_detected':
				// Another tab detected a conflict
				if (message.contentId === this._currentDraft?.contentId) {
					this._hasConflict = true;
					this._conflictData = message.conflictData;
					this.notifySubscribers();
				}
				break;
			case 'draft_updated':
				// Another tab updated a draft
				if (message.contentId === this._currentDraft?.contentId) {
					this.loadDraft(message.contentId);
				}
				break;
		}
	};

	private broadcastMessage(message: Record<string, unknown>): void {
		try {
			this.broadcastChannel?.postMessage(message);
		} catch {
			// Channel closed or not available
		}
	}

	// ==========================================================================
	// Getters
	// ==========================================================================

	get isDraftLoaded(): boolean {
		return this._isDraftLoaded;
	}
	get hasDraft(): boolean {
		return this._hasDraft;
	}
	get lastSavedAt(): Date | null {
		return this._lastSavedAt;
	}
	get isSaving(): boolean {
		return this._isSaving;
	}
	get error(): string | null {
		return this._error;
	}
	get currentDraft(): AutosaveDraft | null {
		return this._currentDraft;
	}
	get isOnline(): boolean {
		return this._isOnline;
	}
	get syncQueue(): SyncQueueItem[] {
		return this._syncQueue;
	}
	get hasConflict(): boolean {
		return this._hasConflict;
	}
	get conflictData(): ConflictData | null {
		return this._conflictData;
	}
	get isSyncing(): boolean {
		return this._isSyncing;
	}
	get pendingSyncCount(): number {
		return this._syncQueue.filter(i => i.status === 'pending' || i.status === 'syncing').length;
	}

	// ==========================================================================
	// Subscribe
	// ==========================================================================

	subscribe(fn: (state: AutosaveState) => void): () => void {
		this.subscribers.add(fn);
		fn(this.getState());
		return () => this.subscribers.delete(fn);
	}

	private getState(): AutosaveState {
		return {
			isDraftLoaded: this._isDraftLoaded,
			hasDraft: this._hasDraft,
			lastSavedAt: this._lastSavedAt,
			isSaving: this._isSaving,
			error: this._error,
			isOnline: this._isOnline,
			syncQueue: this._syncQueue,
			hasConflict: this._hasConflict,
			conflictData: this._conflictData,
			isSyncing: this._isSyncing,
			pendingSyncCount: this.pendingSyncCount
		};
	}

	private notifySubscribers(): void {
		const state = this.getState();
		this.subscribers.forEach((fn) => fn(state));
	}

	// ==========================================================================
	// Draft Management
	// ==========================================================================

	/**
	 * Queue content for autosave (debounced)
	 */
	queueSave(content: EditorContent): void {
		if (!browser) return;

		this.pendingContent = content;

		// Clear existing timer
		if (this.saveTimer) {
			clearTimeout(this.saveTimer);
		}

		// Set new timer
		this.saveTimer = setTimeout(() => {
			this.saveDraft();
		}, AUTOSAVE_DELAY_MS);
	}

	/**
	 * Save draft immediately (async)
	 */
	async saveDraft(): Promise<void> {
		if (!browser || !this.pendingContent) return;

		const content = this.pendingContent;
		this.pendingContent = null;

		if (this.saveTimer) {
			clearTimeout(this.saveTimer);
			this.saveTimer = null;
		}

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			const db = await getDB();
			const draft: AutosaveDraft = {
				id: `draft-${content.id}`,
				contentId: content.id,
				content,
				savedAt: Date.now(),
				version: content.version,
				serverVersion: this._currentDraft?.serverVersion,
				checksum: generateChecksum(content)
			};

			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(DRAFTS_STORE, 'readwrite');
				const store = transaction.objectStore(DRAFTS_STORE);
				const request = store.put(draft);

				request.onsuccess = () => {
					this._currentDraft = draft;
					this._hasDraft = true;
					this._lastSavedAt = new Date();
					resolve();
				};
				request.onerror = () => reject(request.error);
			});

			// If offline, queue for sync
			if (!this._isOnline) {
				await this.queueForSync(content);
			}

			// Cleanup old drafts
			await this.cleanupOldDrafts();

			// Notify other tabs
			this.broadcastMessage({
				type: 'draft_updated',
				contentId: content.id
			});

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Draft saved:', content.id, this._isOnline ? '(online)' : '(offline, queued for sync)');
			}
		} catch (error) {
			console.error('[Autosave] Save failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to save draft';
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Synchronous save for beforeunload handler
	 */
	private saveDraftSync(content: EditorContent): void {
		try {
			const draft: AutosaveDraft = {
				id: `draft-${content.id}`,
				contentId: content.id,
				content,
				savedAt: Date.now(),
				version: content.version,
				checksum: generateChecksum(content)
			};

			// Use localStorage as fallback for synchronous save
			localStorage.setItem(`rtp-cms-emergency-draft-${content.id}`, JSON.stringify(draft));
		} catch (error) {
			console.error('[Autosave] Emergency save failed:', error);
		}
	}

	/**
	 * Load draft for content
	 */
	async loadDraft(contentId: string): Promise<AutosaveDraft | null> {
		if (!browser) return null;

		try {
			// Check for emergency localStorage draft first
			const emergencyDraft = localStorage.getItem(`rtp-cms-emergency-draft-${contentId}`);
			if (emergencyDraft) {
				const draft = JSON.parse(emergencyDraft) as AutosaveDraft;
				localStorage.removeItem(`rtp-cms-emergency-draft-${contentId}`);

				// Save to IndexedDB
				const db = await getDB();
				await new Promise<void>((resolve, reject) => {
					const transaction = db.transaction(DRAFTS_STORE, 'readwrite');
					const store = transaction.objectStore(DRAFTS_STORE);
					const request = store.put(draft);
					request.onsuccess = () => resolve();
					request.onerror = () => reject(request.error);
				});

				this._currentDraft = draft;
				this._hasDraft = true;
				this._isDraftLoaded = true;
				this.notifySubscribers();
				return draft;
			}

			const db = await getDB();
			const draft = await new Promise<AutosaveDraft | null>((resolve, reject) => {
				const transaction = db.transaction(DRAFTS_STORE, 'readonly');
				const store = transaction.objectStore(DRAFTS_STORE);
				const request = store.get(`draft-${contentId}`);

				request.onsuccess = () => resolve(request.result || null);
				request.onerror = () => reject(request.error);
			});

			this._currentDraft = draft;
			this._hasDraft = draft !== null;
			this._isDraftLoaded = true;
			this.notifySubscribers();

			// Check for conflicts if online
			if (draft && this._isOnline) {
				await this.checkVersionConflict(contentId, draft);
			}

			return draft;
		} catch (error) {
			console.error('[Autosave] Load failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to load draft';
			this._isDraftLoaded = true;
			this.notifySubscribers();
			return null;
		}
	}

	/**
	 * Clear draft for content
	 */
	async clearDraft(contentId: string): Promise<void> {
		if (!browser) return;

		try {
			const db = await getDB();
			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(DRAFTS_STORE, 'readwrite');
				const store = transaction.objectStore(DRAFTS_STORE);
				const request = store.delete(`draft-${contentId}`);

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			// Also remove from sync queue
			await this.removeFromSyncQueue(contentId);

			this._currentDraft = null;
			this._hasDraft = false;
			this._hasConflict = false;
			this._conflictData = null;
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Draft cleared:', contentId);
			}
		} catch (error) {
			console.error('[Autosave] Clear failed:', error);
		}
	}

	/**
	 * Get all drafts
	 */
	async getAllDrafts(): Promise<AutosaveDraft[]> {
		if (!browser) return [];

		try {
			const db = await getDB();
			return new Promise((resolve, reject) => {
				const transaction = db.transaction(DRAFTS_STORE, 'readonly');
				const store = transaction.objectStore(DRAFTS_STORE);
				const request = store.getAll();

				request.onsuccess = () => resolve(request.result || []);
				request.onerror = () => reject(request.error);
			});
		} catch (error) {
			console.error('[Autosave] Get all failed:', error);
			return [];
		}
	}

	/**
	 * Check if draft is newer than server content
	 */
	isDraftNewer(serverVersion: number, serverUpdatedAt?: string): boolean {
		if (!this._currentDraft) return false;

		// Check version first
		if (this._currentDraft.version > serverVersion) return true;

		// Check timestamp if versions match
		if (serverUpdatedAt) {
			const serverTime = new Date(serverUpdatedAt).getTime();
			return this._currentDraft.savedAt > serverTime;
		}

		return false;
	}

	// ==========================================================================
	// Sync Queue Management
	// ==========================================================================

	/**
	 * Queue content for sync when back online
	 */
	private async queueForSync(content: EditorContent): Promise<void> {
		try {
			const db = await getDB();
			const queueItem: SyncQueueItem = {
				id: `sync-${content.id}-${Date.now()}`,
				contentId: content.id,
				content,
				queuedAt: Date.now(),
				retryCount: 0,
				nextRetryAt: Date.now(),
				status: 'pending'
			};

			// Remove any existing queue items for this content
			await this.removeFromSyncQueue(content.id);

			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(SYNC_QUEUE_STORE, 'readwrite');
				const store = transaction.objectStore(SYNC_QUEUE_STORE);
				const request = store.put(queueItem);

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._syncQueue = [...this._syncQueue.filter(i => i.contentId !== content.id), queueItem];
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Queued for sync:', content.id);
			}
		} catch (error) {
			console.error('[Autosave] Failed to queue for sync:', error);
		}
	}

	/**
	 * Remove content from sync queue
	 */
	private async removeFromSyncQueue(contentId: string): Promise<void> {
		try {
			const db = await getDB();
			const itemsToRemove = this._syncQueue.filter(i => i.contentId === contentId);

			if (itemsToRemove.length === 0) return;

			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(SYNC_QUEUE_STORE, 'readwrite');
				const store = transaction.objectStore(SYNC_QUEUE_STORE);

				let completed = 0;
				for (const item of itemsToRemove) {
					const request = store.delete(item.id);
					request.onsuccess = () => {
						completed++;
						if (completed === itemsToRemove.length) resolve();
					};
					request.onerror = () => reject(request.error);
				}

				if (itemsToRemove.length === 0) resolve();
			});

			this._syncQueue = this._syncQueue.filter(i => i.contentId !== contentId);
			this.notifySubscribers();
		} catch (error) {
			console.error('[Autosave] Failed to remove from sync queue:', error);
		}
	}

	/**
	 * Update sync queue item status
	 */
	private async updateSyncQueueItem(item: SyncQueueItem): Promise<void> {
		try {
			const db = await getDB();
			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(SYNC_QUEUE_STORE, 'readwrite');
				const store = transaction.objectStore(SYNC_QUEUE_STORE);
				const request = store.put(item);

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._syncQueue = this._syncQueue.map(i => i.id === item.id ? item : i);
			this.notifySubscribers();
		} catch (error) {
			console.error('[Autosave] Failed to update sync queue item:', error);
		}
	}

	/**
	 * Get all queued items
	 */
	getQueuedItems(): SyncQueueItem[] {
		return [...this._syncQueue];
	}

	/**
	 * Clear entire sync queue
	 */
	async clearQueue(): Promise<void> {
		if (!browser) return;

		try {
			const db = await getDB();
			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(SYNC_QUEUE_STORE, 'readwrite');
				const store = transaction.objectStore(SYNC_QUEUE_STORE);
				const request = store.clear();

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._syncQueue = [];
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Sync queue cleared');
			}
		} catch (error) {
			console.error('[Autosave] Failed to clear queue:', error);
		}
	}

	// ==========================================================================
	// Sync Operations
	// ==========================================================================

	/**
	 * Sync all pending items with the server
	 */
	async syncPending(): Promise<void> {
		if (!browser || !this._isOnline || this._isSyncing) return;

		const pendingItems = this._syncQueue.filter(
			i => i.status === 'pending' && i.nextRetryAt <= Date.now()
		);

		if (pendingItems.length === 0) return;

		this._isSyncing = true;
		this.notifySubscribers();

		if (import.meta.env.DEV) {
			console.debug('[Autosave] Starting sync for', pendingItems.length, 'items');
		}

		for (const item of pendingItems) {
			try {
				// Update status to syncing
				item.status = 'syncing';
				await this.updateSyncQueueItem(item);

				// Check server version first
				const serverVersion = await this.fetchServerVersion(item.contentId);

				if (serverVersion && serverVersion.version > item.content.version) {
					// Conflict detected
					await this.handleSyncConflict(item, serverVersion);
					continue;
				}

				// Attempt sync
				const response = await this.submitSync(item.content);

				if (response.success) {
					// Remove from queue
					await this.removeFromSyncQueue(item.contentId);

					// Update draft with new server version
					if (this._currentDraft?.contentId === item.contentId) {
						this._currentDraft.serverVersion = response.newVersion;
					}

					if (import.meta.env.DEV) {
						console.debug('[Autosave] Synced:', item.contentId);
					}
				} else if (response.conflict) {
					// Server reported conflict
					await this.handleSyncConflict(item, {
						version: response.conflict.serverVersion,
						updatedAt: new Date().toISOString()
					});
				} else {
					// Failed, schedule retry
					await this.handleSyncFailure(item, response.error || 'Sync failed');
				}
			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Unknown error';
				await this.handleSyncFailure(item, errorMessage);
			}
		}

		this._isSyncing = false;
		this.notifySubscribers();

		// Notify other tabs
		this.broadcastMessage({ type: 'sync_complete' });

		// Schedule retry for failed items
		this.scheduleSyncRetry();
	}

	/**
	 * Handle sync failure with exponential backoff
	 */
	private async handleSyncFailure(item: SyncQueueItem, error: string): Promise<void> {
		item.retryCount++;
		item.lastError = error;

		if (item.retryCount >= MAX_RETRY_ATTEMPTS) {
			item.status = 'failed';
			console.error('[Autosave] Max retries reached for:', item.contentId);
		} else {
			item.status = 'pending';
			item.nextRetryAt = Date.now() + calculateBackoffDelay(item.retryCount);
		}

		await this.updateSyncQueueItem(item);
	}

	/**
	 * Handle sync conflict
	 */
	private async handleSyncConflict(
		item: SyncQueueItem,
		serverVersion: ServerVersionResponse
	): Promise<void> {
		// Fetch full server content
		let serverContent: EditorContent | null = null;
		try {
			const response = await fetch(`/api/cms/content/${item.contentId}`, {
				headers: { 'Accept': 'application/json' }
			});
			if (response.ok) {
				serverContent = await response.json();
			}
		} catch {
			console.error('[Autosave] Failed to fetch server content for conflict');
		}

		if (!serverContent) {
			// Can't get server content, mark as failed
			await this.handleSyncFailure(item, 'Failed to fetch server content for conflict resolution');
			return;
		}

		const conflictData: ConflictData = {
			contentId: item.contentId,
			localVersion: item.content.version,
			serverVersion: serverVersion.version,
			localContent: item.content,
			serverContent,
			localModifiedAt: item.queuedAt,
			serverModifiedAt: new Date(serverVersion.updatedAt).getTime(),
			detectedAt: Date.now()
		};

		// Mark item as failed pending conflict resolution
		item.status = 'failed';
		item.lastError = 'Conflict detected - manual resolution required';
		await this.updateSyncQueueItem(item);

		// Set conflict state
		this._hasConflict = true;
		this._conflictData = conflictData;
		this.notifySubscribers();

		// Notify other tabs
		this.broadcastMessage({
			type: 'conflict_detected',
			contentId: item.contentId,
			conflictData
		});

		if (import.meta.env.DEV) {
			console.debug('[Autosave] Conflict detected:', item.contentId);
		}
	}

	/**
	 * Schedule retry for pending sync items
	 */
	private scheduleSyncRetry(): void {
		if (this.syncTimer) {
			clearTimeout(this.syncTimer);
		}

		const pendingItems = this._syncQueue.filter(i => i.status === 'pending');
		if (pendingItems.length === 0) return;

		// Find next retry time
		const nextRetry = Math.min(...pendingItems.map(i => i.nextRetryAt));
		const delay = Math.max(0, nextRetry - Date.now());

		this.syncTimer = setTimeout(() => {
			if (this._isOnline) {
				this.syncPending();
			}
		}, delay);
	}

	// ==========================================================================
	// Conflict Detection & Resolution
	// ==========================================================================

	/**
	 * Check for conflicts with all drafts
	 */
	private async checkForConflicts(): Promise<void> {
		if (!this._currentDraft || !this._isOnline) return;

		await this.checkVersionConflict(
			this._currentDraft.contentId,
			this._currentDraft
		);
	}

	/**
	 * Check if local draft conflicts with server version
	 */
	private async checkVersionConflict(
		contentId: string,
		draft: AutosaveDraft
	): Promise<boolean> {
		if (!this._isOnline) return false;

		try {
			const serverVersion = await this.fetchServerVersion(contentId);

			if (!serverVersion) return false;

			// Check if server version is newer than when draft was created
			if (serverVersion.version > (draft.serverVersion ?? draft.version)) {
				// Fetch full server content for comparison
				const response = await fetch(`/api/cms/content/${contentId}`, {
					headers: { 'Accept': 'application/json' }
				});

				if (!response.ok) return false;

				const serverContent = await response.json() as EditorContent;

				// Check if content actually differs (not just version number)
				const serverChecksum = generateChecksum(serverContent);
				if (draft.checksum === serverChecksum) {
					// Content is the same, just update server version
					draft.serverVersion = serverVersion.version;
					return false;
				}

				// Real conflict detected
				const conflictData: ConflictData = {
					contentId,
					localVersion: draft.version,
					serverVersion: serverVersion.version,
					localContent: draft.content,
					serverContent,
					localModifiedAt: draft.savedAt,
					serverModifiedAt: new Date(serverVersion.updatedAt).getTime(),
					detectedAt: Date.now()
				};

				this._hasConflict = true;
				this._conflictData = conflictData;
				this.notifySubscribers();

				return true;
			}

			return false;
		} catch (error) {
			console.error('[Autosave] Version check failed:', error);
			return false;
		}
	}

	/**
	 * Resolve conflict with specified strategy
	 */
	async resolveConflict(resolution: ConflictResolution): Promise<void> {
		if (!this._conflictData) {
			console.warn('[Autosave] No conflict to resolve');
			return;
		}

		const { contentId, localContent, serverContent } = this._conflictData;

		try {
			let resolvedContent: EditorContent;

			switch (resolution) {
				case 'mine':
					// Keep local changes, but update version
					resolvedContent = {
						...localContent,
						version: serverContent.version + 1
					};
					break;

				case 'theirs':
					// Accept server version
					resolvedContent = serverContent;
					// Clear local draft
					await this.clearDraft(contentId);
					this._hasConflict = false;
					this._conflictData = null;
					this.notifySubscribers();
					return;

				case 'merge':
					// Attempt automatic merge
					const mergeResult = mergeContent(localContent, serverContent);
					resolvedContent = mergeResult.content;

					if (import.meta.env.DEV) {
						console.debug('[Autosave] Merged fields:', mergeResult.resolvedFields);
					}
					break;

				default:
					throw new Error(`Unknown resolution strategy: ${resolution}`);
			}

			// Queue resolved content for sync
			this.pendingContent = resolvedContent;
			await this.saveDraft();

			if (this._isOnline) {
				await this.queueForSync(resolvedContent);
				await this.syncPending();
			}

			// Clear conflict state
			this._hasConflict = false;
			this._conflictData = null;
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Conflict resolved with strategy:', resolution);
			}
		} catch (error) {
			console.error('[Autosave] Conflict resolution failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to resolve conflict';
			this.notifySubscribers();
		}
	}

	// ==========================================================================
	// API Communication
	// ==========================================================================

	/**
	 * Fetch server version for content
	 */
	private async fetchServerVersion(contentId: string): Promise<ServerVersionResponse | null> {
		try {
			const controller = new AbortController();
			const timeoutId = setTimeout(() => controller.abort(), VERSION_CHECK_TIMEOUT_MS);

			const response = await fetch(`/api/cms/content/${contentId}/version`, {
				headers: { 'Accept': 'application/json' },
				signal: controller.signal
			});

			clearTimeout(timeoutId);

			if (!response.ok) {
				if (response.status === 404) {
					return null; // Content doesn't exist on server
				}
				throw new Error(`Server returned ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				console.warn('[Autosave] Version check timed out');
			} else {
				console.error('[Autosave] Version check failed:', error);
			}
			return null;
		}
	}

	/**
	 * Submit content sync to server
	 */
	private async submitSync(content: EditorContent): Promise<SyncResponse> {
		try {
			const response = await fetch('/api/cms/sync', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Accept': 'application/json'
				},
				body: JSON.stringify({
					contentId: content.id,
					content,
					expectedVersion: content.version
				})
			});

			const data = await response.json();

			if (!response.ok) {
				if (response.status === 409) {
					// Conflict response
					return {
						success: false,
						contentId: content.id,
						conflict: data.conflict
					};
				}
				return {
					success: false,
					contentId: content.id,
					error: data.error || `Server returned ${response.status}`
				};
			}

			return {
				success: true,
				contentId: content.id,
				newVersion: data.version
			};
		} catch (error) {
			return {
				success: false,
				contentId: content.id,
				error: error instanceof Error ? error.message : 'Network error'
			};
		}
	}

	// ==========================================================================
	// Cleanup
	// ==========================================================================

	/**
	 * Remove old drafts to prevent IndexedDB from growing too large
	 */
	private async cleanupOldDrafts(): Promise<void> {
		try {
			const db = await getDB();
			const drafts = await this.getAllDrafts();

			if (drafts.length <= MAX_DRAFTS) return;

			// Sort by savedAt descending and get IDs to delete
			const sortedDrafts = drafts.sort((a, b) => b.savedAt - a.savedAt);
			const toDelete = sortedDrafts.slice(MAX_DRAFTS);

			const transaction = db.transaction(DRAFTS_STORE, 'readwrite');
			const store = transaction.objectStore(DRAFTS_STORE);

			for (const draft of toDelete) {
				store.delete(draft.id);
			}

			if (import.meta.env.DEV) {
				console.debug(`[Autosave] Cleaned up ${toDelete.length} old drafts`);
			}
		} catch (error) {
			console.error('[Autosave] Cleanup failed:', error);
		}
	}

	/**
	 * Clear all drafts
	 */
	async clearAllDrafts(): Promise<void> {
		if (!browser) return;

		try {
			const db = await getDB();
			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(DRAFTS_STORE, 'readwrite');
				const store = transaction.objectStore(DRAFTS_STORE);
				const request = store.clear();

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._currentDraft = null;
			this._hasDraft = false;
			this._hasConflict = false;
			this._conflictData = null;
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] All drafts cleared');
			}
		} catch (error) {
			console.error('[Autosave] Clear all failed:', error);
		}
	}

	// ==========================================================================
	// Reset & Cleanup
	// ==========================================================================

	/**
	 * Reset store state (but keep IndexedDB data)
	 */
	reset(): void {
		if (this.saveTimer) {
			clearTimeout(this.saveTimer);
			this.saveTimer = null;
		}
		if (this.syncTimer) {
			clearTimeout(this.syncTimer);
			this.syncTimer = null;
		}
		this.pendingContent = null;
		this._isDraftLoaded = false;
		this._hasDraft = false;
		this._lastSavedAt = null;
		this._isSaving = false;
		this._error = null;
		this._currentDraft = null;
		this._hasConflict = false;
		this._conflictData = null;
		this._isSyncing = false;
		this.notifySubscribers();
	}

	/**
	 * Full cleanup including event listeners
	 */
	destroy(): void {
		this.reset();

		if (browser) {
			window.removeEventListener('online', this.handleOnline);
			window.removeEventListener('offline', this.handleOffline);
			document.removeEventListener('visibilitychange', this.handleVisibilityChange);
			window.removeEventListener('beforeunload', this.handleBeforeUnload);
			this.broadcastChannel?.close();
		}
	}
}

// =============================================================================
// Export Store Instance
// =============================================================================

export const autosaveStore = new AutosaveStoreClass();

// =============================================================================
// Convenience Exports - Derived State
// =============================================================================

export const hasDraft = {
	get current() {
		return autosaveStore.hasDraft;
	},
	subscribe(fn: (value: boolean) => void) {
		return autosaveStore.subscribe((state) => fn(state.hasDraft));
	}
};

export const autosaveLastSaved = {
	get current() {
		return autosaveStore.lastSavedAt;
	},
	subscribe(fn: (value: Date | null) => void) {
		return autosaveStore.subscribe((state) => fn(state.lastSavedAt));
	}
};

export const isOnline = {
	get current() {
		return autosaveStore.isOnline;
	},
	subscribe(fn: (value: boolean) => void) {
		return autosaveStore.subscribe((state) => fn(state.isOnline));
	}
};

export const hasConflict = {
	get current() {
		return autosaveStore.hasConflict;
	},
	subscribe(fn: (value: boolean) => void) {
		return autosaveStore.subscribe((state) => fn(state.hasConflict));
	}
};

export const pendingSyncCount = {
	get current() {
		return autosaveStore.pendingSyncCount;
	},
	subscribe(fn: (value: number) => void) {
		return autosaveStore.subscribe((state) => fn(state.pendingSyncCount));
	}
};

export const isSyncing = {
	get current() {
		return autosaveStore.isSyncing;
	},
	subscribe(fn: (value: boolean) => void) {
		return autosaveStore.subscribe((state) => fn(state.isSyncing));
	}
};

export const conflictData = {
	get current() {
		return autosaveStore.conflictData;
	},
	subscribe(fn: (value: ConflictData | null) => void) {
		return autosaveStore.subscribe((state) => fn(state.conflictData));
	}
};
