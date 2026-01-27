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
 *
 * @version 2.0.0 - January 2026
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import type { EditorContent } from './editor.svelte';

// =============================================================================
// Constants
// =============================================================================

const DB_NAME = 'rtp-cms-autosave';
const DB_VERSION = 1;
const STORE_NAME = 'drafts';
const AUTOSAVE_DELAY_MS = 2000; // 2 seconds debounce
const MAX_DRAFTS = 50; // Maximum number of drafts to keep

// =============================================================================
// Type Definitions
// =============================================================================

export interface AutosaveDraft {
	id: string;
	contentId: string;
	content: EditorContent;
	savedAt: number;
	version: number;
}

export interface AutosaveState {
	isDraftLoaded: boolean;
	hasDraft: boolean;
	lastSavedAt: Date | null;
	isSaving: boolean;
	error: string | null;
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
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				const store = db.createObjectStore(STORE_NAME, { keyPath: 'id' });
				store.createIndex('contentId', 'contentId', { unique: false });
				store.createIndex('savedAt', 'savedAt', { unique: false });
			}
		};
	});

	return dbInitPromise;
}

// =============================================================================
// Autosave Store Class - Svelte 5 Runes
// =============================================================================

class AutosaveStoreClass {
	// State
	private _isDraftLoaded = $state(false);
	private _hasDraft = $state(false);
	private _lastSavedAt = $state<Date | null>(null);
	private _isSaving = $state(false);
	private _error = $state<string | null>(null);
	private _currentDraft = $state<AutosaveDraft | null>(null);

	// Debounce timer
	private saveTimer: ReturnType<typeof setTimeout> | null = null;
	private pendingContent: EditorContent | null = null;

	// Subscribers
	private subscribers = new Set<(state: AutosaveState) => void>();

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
			error: this._error
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
	 * Save draft immediately
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
				version: content.version
			};

			await new Promise<void>((resolve, reject) => {
				const transaction = db.transaction(STORE_NAME, 'readwrite');
				const store = transaction.objectStore(STORE_NAME);
				const request = store.put(draft);

				request.onsuccess = () => {
					this._currentDraft = draft;
					this._hasDraft = true;
					this._lastSavedAt = new Date();
					resolve();
				};
				request.onerror = () => reject(request.error);
			});

			// Cleanup old drafts
			await this.cleanupOldDrafts();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] Draft saved:', content.id);
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
	 * Load draft for content
	 */
	async loadDraft(contentId: string): Promise<AutosaveDraft | null> {
		if (!browser) return null;

		try {
			const db = await getDB();
			const draft = await new Promise<AutosaveDraft | null>((resolve, reject) => {
				const transaction = db.transaction(STORE_NAME, 'readonly');
				const store = transaction.objectStore(STORE_NAME);
				const request = store.get(`draft-${contentId}`);

				request.onsuccess = () => resolve(request.result || null);
				request.onerror = () => reject(request.error);
			});

			this._currentDraft = draft;
			this._hasDraft = draft !== null;
			this._isDraftLoaded = true;
			this.notifySubscribers();

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
				const transaction = db.transaction(STORE_NAME, 'readwrite');
				const store = transaction.objectStore(STORE_NAME);
				const request = store.delete(`draft-${contentId}`);

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._currentDraft = null;
			this._hasDraft = false;
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
				const transaction = db.transaction(STORE_NAME, 'readonly');
				const store = transaction.objectStore(STORE_NAME);
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

			const transaction = db.transaction(STORE_NAME, 'readwrite');
			const store = transaction.objectStore(STORE_NAME);

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
				const transaction = db.transaction(STORE_NAME, 'readwrite');
				const store = transaction.objectStore(STORE_NAME);
				const request = store.clear();

				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});

			this._currentDraft = null;
			this._hasDraft = false;
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Autosave] All drafts cleared');
			}
		} catch (error) {
			console.error('[Autosave] Clear all failed:', error);
		}
	}

	// ==========================================================================
	// Reset
	// ==========================================================================

	reset(): void {
		if (this.saveTimer) {
			clearTimeout(this.saveTimer);
			this.saveTimer = null;
		}
		this.pendingContent = null;
		this._isDraftLoaded = false;
		this._hasDraft = false;
		this._lastSavedAt = null;
		this._isSaving = false;
		this._error = null;
		this._currentDraft = null;
		this.notifySubscribers();
	}
}

// =============================================================================
// Export Store Instance
// =============================================================================

export const autosaveStore = new AutosaveStoreClass();

// =============================================================================
// Convenience Exports
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
