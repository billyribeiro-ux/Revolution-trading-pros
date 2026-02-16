/**
 * Revolution Trading Pros - Blog Editor IndexedDB Wrapper
 * ========================================================
 * Enterprise-grade offline storage for the block editor using native IndexedDB.
 * Provides type-safe CRUD operations with automatic cleanup and schema migrations.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

import { browser } from '$app/environment';
import type { Block, Revision } from '../types';
import type { Post } from '$lib/types/post';
import { logger } from '$lib/utils/logger';

// =============================================================================
// Database Configuration
// =============================================================================

const DB_NAME = 'revolution-blog-editor';
const DB_VERSION = 1;
const CLEANUP_INTERVAL_MS = 24 * 60 * 60 * 1000; // 24 hours
const MAX_AGE_MS = 30 * 24 * 60 * 60 * 1000; // 30 days

// Store names
const STORES = {
	DRAFTS: 'drafts',
	PENDING_CHANGES: 'pendingChanges',
	ASSETS: 'assets',
	REVISIONS: 'revisions'
} as const;

type StoreName = (typeof STORES)[keyof typeof STORES];

// =============================================================================
// Type Definitions
// =============================================================================

export interface Draft {
	id: string;
	postId: number | null; // null for new posts
	title: string;
	slug: string;
	excerpt: string | null;
	content_blocks: Block[];
	featured_image: string | null;
	meta_title: string | null;
	meta_description: string | null;
	canonical_url: string | null;
	schema_markup: unknown | null;
	indexable: boolean;
	authorId: number | null;
	createdAt: number;
	updatedAt: number;
	version: number;
	syncedAt: number | null;
	checksum: string;
}

export interface PendingChange {
	id: string;
	draftId: string;
	postId: number | null;
	operation: 'create' | 'update' | 'delete' | 'publish' | 'unpublish';
	data: Partial<Draft>;
	previousChecksum: string | null;
	createdAt: number;
	attempts: number;
	lastAttemptAt: number | null;
	error: string | null;
	priority: number; // Higher = more urgent
}

export interface CachedAsset {
	id: string;
	url: string;
	mimeType: string;
	blob: Blob;
	size: number;
	createdAt: number;
	accessedAt: number;
	draftIds: string[]; // References to drafts using this asset
}

export interface StoredRevision extends Omit<Revision, 'createdAt' | 'postId'> {
	draftId: string;
	postId: string | null; // Matches Revision.postId type but allows null for new drafts
	createdAt: number; // Stored as timestamp for indexing
	expiresAt: number;
}

// =============================================================================
// Database Schema & Migrations
// =============================================================================

interface SchemaVersion {
	version: number;
	migrate: (db: IDBDatabase, transaction: IDBTransaction) => void;
}

const SCHEMA_VERSIONS: SchemaVersion[] = [
	{
		version: 1,
		migrate: (db: IDBDatabase) => {
			// Drafts store - main draft storage
			if (!db.objectStoreNames.contains(STORES.DRAFTS)) {
				const draftsStore = db.createObjectStore(STORES.DRAFTS, { keyPath: 'id' });
				draftsStore.createIndex('postId', 'postId', { unique: false });
				draftsStore.createIndex('updatedAt', 'updatedAt', { unique: false });
				draftsStore.createIndex('syncedAt', 'syncedAt', { unique: false });
			}

			// Pending changes store - sync queue
			if (!db.objectStoreNames.contains(STORES.PENDING_CHANGES)) {
				const pendingStore = db.createObjectStore(STORES.PENDING_CHANGES, { keyPath: 'id' });
				pendingStore.createIndex('draftId', 'draftId', { unique: false });
				pendingStore.createIndex('postId', 'postId', { unique: false });
				pendingStore.createIndex('createdAt', 'createdAt', { unique: false });
				pendingStore.createIndex('priority', 'priority', { unique: false });
				pendingStore.createIndex('operation', 'operation', { unique: false });
			}

			// Assets store - cached media files
			if (!db.objectStoreNames.contains(STORES.ASSETS)) {
				const assetsStore = db.createObjectStore(STORES.ASSETS, { keyPath: 'id' });
				assetsStore.createIndex('url', 'url', { unique: true });
				assetsStore.createIndex('accessedAt', 'accessedAt', { unique: false });
				assetsStore.createIndex('size', 'size', { unique: false });
			}

			// Revisions store - version history
			if (!db.objectStoreNames.contains(STORES.REVISIONS)) {
				const revisionsStore = db.createObjectStore(STORES.REVISIONS, { keyPath: 'id' });
				revisionsStore.createIndex('draftId', 'draftId', { unique: false });
				revisionsStore.createIndex('postId', 'postId', { unique: false });
				revisionsStore.createIndex('createdAt', 'createdAt', { unique: false });
				revisionsStore.createIndex('expiresAt', 'expiresAt', { unique: false });
			}
		}
	}
];

// =============================================================================
// Database Connection Manager
// =============================================================================

class DatabaseConnection {
	private db: IDBDatabase | null = null;
	private connectionPromise: Promise<IDBDatabase> | null = null;
	private lastCleanup: number = 0;

	/**
	 * Opens or returns existing database connection
	 */
	async open(): Promise<IDBDatabase> {
		if (!browser) {
			throw new Error('IndexedDB is only available in browser environment');
		}

		if (this.db) {
			return this.db;
		}

		if (this.connectionPromise) {
			return this.connectionPromise;
		}

		this.connectionPromise = new Promise<IDBDatabase>((resolve, reject) => {
			const request = indexedDB.open(DB_NAME, DB_VERSION);

			request.onerror = () => {
				this.connectionPromise = null;
				reject(new Error(`Failed to open database: ${request.error?.message}`));
			};

			request.onsuccess = () => {
				this.db = request.result;

				// Handle unexpected closure
				this.db.onclose = () => {
					this.db = null;
					this.connectionPromise = null;
				};

				this.db.onerror = (event) => {
					logger.error('[BlogEditorDB] Database error:', event);
				};

				resolve(this.db);
			};

			request.onupgradeneeded = (event) => {
				const db = request.result;
				const transaction = request.transaction!;
				const oldVersion = event.oldVersion;

				// Run migrations for versions greater than old version
				for (const schema of SCHEMA_VERSIONS) {
					if (schema.version > oldVersion) {
						schema.migrate(db, transaction);
					}
				}
			};

			request.onblocked = () => {
				logger.warn('[BlogEditorDB] Database upgrade blocked - close other tabs');
			};
		});

		return this.connectionPromise;
	}

	/**
	 * Closes database connection
	 */
	close(): void {
		if (this.db) {
			this.db.close();
			this.db = null;
		}
		this.connectionPromise = null;
	}

	/**
	 * Executes a transaction with automatic retry
	 */
	async transaction<T>(
		storeNames: StoreName | StoreName[],
		mode: IDBTransactionMode,
		operation: (stores: Map<StoreName, IDBObjectStore>) => Promise<T>
	): Promise<T> {
		const db = await this.open();
		const storeNamesArray = Array.isArray(storeNames) ? storeNames : [storeNames];

		return new Promise<T>((resolve, reject) => {
			try {
				const transaction = db.transaction(storeNamesArray, mode);
				const stores = new Map<StoreName, IDBObjectStore>();

				for (const storeName of storeNamesArray) {
					stores.set(storeName, transaction.objectStore(storeName));
				}

				transaction.onerror = () => {
					reject(new Error(`Transaction failed: ${transaction.error?.message}`));
				};

				transaction.oncomplete = () => {
					// Transaction completed - results already resolved
				};

				// Execute the operation
				operation(stores).then(resolve).catch(reject);
			} catch (error) {
				reject(error);
			}
		});
	}

	/**
	 * Schedules and runs cleanup if needed
	 */
	async maybeCleanup(): Promise<void> {
		const now = Date.now();
		if (now - this.lastCleanup < CLEANUP_INTERVAL_MS) {
			return;
		}

		this.lastCleanup = now;
		await this.cleanup();
	}

	/**
	 * Cleans up old data from all stores
	 */
	async cleanup(): Promise<CleanupResult> {
		const cutoff = Date.now() - MAX_AGE_MS;
		const result: CleanupResult = {
			drafts: 0,
			pendingChanges: 0,
			assets: 0,
			revisions: 0
		};

		// Clean up old drafts without pending changes
		const pendingDraftIds = new Set((await this.getAllPendingChanges()).map((pc) => pc.draftId));

		const drafts = await this.getAllDrafts();
		for (const draft of drafts) {
			if (draft.updatedAt < cutoff && !pendingDraftIds.has(draft.id)) {
				await this.deleteDraft(draft.id);
				result.drafts++;
			}
		}

		// Clean up old pending changes that have been retried too many times
		const MAX_ATTEMPTS = 10;
		const pendingChanges = await this.getAllPendingChanges();
		for (const change of pendingChanges) {
			if (change.attempts >= MAX_ATTEMPTS || change.createdAt < cutoff) {
				await this.deletePendingChange(change.id);
				result.pendingChanges++;
			}
		}

		// Clean up expired revisions
		const revisions = await this.getAllRevisions();
		const now = Date.now();
		for (const revision of revisions) {
			if (revision.expiresAt < now) {
				await this.deleteRevision(revision.id);
				result.revisions++;
			}
		}

		// Clean up orphaned assets
		const usedAssetIds = new Set<string>();
		for (const draft of await this.getAllDrafts()) {
			this.extractAssetIds(draft.content_blocks, usedAssetIds);
		}

		const assets = await this.getAllAssets();
		for (const asset of assets) {
			if (!usedAssetIds.has(asset.id) && asset.accessedAt < cutoff) {
				await this.deleteAsset(asset.id);
				result.assets++;
			}
		}

		if (import.meta.env.DEV) {
			logger.debug('[BlogEditorDB] Cleanup complete:', result);
		}

		return result;
	}

	/**
	 * Extracts asset IDs from blocks recursively
	 */
	private extractAssetIds(blocks: Block[], ids: Set<string>): void {
		for (const block of blocks) {
			if (block.content.mediaId) {
				ids.add(String(block.content.mediaId));
			}
			if (block.content.children) {
				this.extractAssetIds(block.content.children, ids);
			}
		}
	}

	// ===========================================================================
	// Drafts CRUD Operations
	// ===========================================================================

	async getDraft(id: string): Promise<Draft | null> {
		return this.transaction(STORES.DRAFTS, 'readonly', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			return new Promise<Draft | null>((resolve, reject) => {
				const request = store.get(id);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getDraftByPostId(postId: number): Promise<Draft | null> {
		return this.transaction(STORES.DRAFTS, 'readonly', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			const index = store.index('postId');
			return new Promise<Draft | null>((resolve, reject) => {
				const request = index.get(postId);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getAllDrafts(): Promise<Draft[]> {
		return this.transaction(STORES.DRAFTS, 'readonly', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			return new Promise<Draft[]>((resolve, reject) => {
				const request = store.getAll();
				request.onsuccess = () => resolve(request.result ?? []);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async saveDraft(draft: Draft): Promise<void> {
		await this.transaction(STORES.DRAFTS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.put(draft);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});

		// Trigger cleanup occasionally
		await this.maybeCleanup();
	}

	async deleteDraft(id: string): Promise<void> {
		return this.transaction(STORES.DRAFTS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.delete(id);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async clearDrafts(): Promise<void> {
		return this.transaction(STORES.DRAFTS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.DRAFTS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.clear();
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	// ===========================================================================
	// Pending Changes CRUD Operations
	// ===========================================================================

	async getPendingChange(id: string): Promise<PendingChange | null> {
		return this.transaction(STORES.PENDING_CHANGES, 'readonly', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			return new Promise<PendingChange | null>((resolve, reject) => {
				const request = store.get(id);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getAllPendingChanges(): Promise<PendingChange[]> {
		return this.transaction(STORES.PENDING_CHANGES, 'readonly', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			const index = store.index('priority');
			return new Promise<PendingChange[]>((resolve, reject) => {
				const request = index.getAll();
				request.onsuccess = () => {
					// Sort by priority (desc) then createdAt (asc)
					const results = (request.result ?? []).sort((a, b) => {
						if (a.priority !== b.priority) {
							return b.priority - a.priority;
						}
						return a.createdAt - b.createdAt;
					});
					resolve(results);
				};
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getPendingChangesByDraftId(draftId: string): Promise<PendingChange[]> {
		return this.transaction(STORES.PENDING_CHANGES, 'readonly', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			const index = store.index('draftId');
			return new Promise<PendingChange[]>((resolve, reject) => {
				const request = index.getAll(draftId);
				request.onsuccess = () => resolve(request.result ?? []);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async savePendingChange(change: PendingChange): Promise<void> {
		return this.transaction(STORES.PENDING_CHANGES, 'readwrite', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.put(change);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async deletePendingChange(id: string): Promise<void> {
		return this.transaction(STORES.PENDING_CHANGES, 'readwrite', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.delete(id);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async clearPendingChanges(): Promise<void> {
		return this.transaction(STORES.PENDING_CHANGES, 'readwrite', async (stores) => {
			const store = stores.get(STORES.PENDING_CHANGES)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.clear();
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	// ===========================================================================
	// Assets CRUD Operations
	// ===========================================================================

	async getAsset(id: string): Promise<CachedAsset | null> {
		const asset = await this.transaction(STORES.ASSETS, 'readonly', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			return new Promise<CachedAsset | null>((resolve, reject) => {
				const request = store.get(id);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});

		// Update access time
		if (asset) {
			asset.accessedAt = Date.now();
			await this.saveAsset(asset);
		}

		return asset;
	}

	async getAssetByUrl(url: string): Promise<CachedAsset | null> {
		const asset = await this.transaction(STORES.ASSETS, 'readonly', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			const index = store.index('url');
			return new Promise<CachedAsset | null>((resolve, reject) => {
				const request = index.get(url);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});

		// Update access time
		if (asset) {
			asset.accessedAt = Date.now();
			await this.saveAsset(asset);
		}

		return asset;
	}

	async getAllAssets(): Promise<CachedAsset[]> {
		return this.transaction(STORES.ASSETS, 'readonly', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			return new Promise<CachedAsset[]>((resolve, reject) => {
				const request = store.getAll();
				request.onsuccess = () => resolve(request.result ?? []);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async saveAsset(asset: CachedAsset): Promise<void> {
		return this.transaction(STORES.ASSETS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.put(asset);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async deleteAsset(id: string): Promise<void> {
		return this.transaction(STORES.ASSETS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.delete(id);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async clearAssets(): Promise<void> {
		return this.transaction(STORES.ASSETS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.ASSETS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.clear();
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getAssetsTotalSize(): Promise<number> {
		const assets = await this.getAllAssets();
		return assets.reduce((total, asset) => total + asset.size, 0);
	}

	// ===========================================================================
	// Revisions CRUD Operations
	// ===========================================================================

	async getRevision(id: string): Promise<StoredRevision | null> {
		return this.transaction(STORES.REVISIONS, 'readonly', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			return new Promise<StoredRevision | null>((resolve, reject) => {
				const request = store.get(id);
				request.onsuccess = () => resolve(request.result ?? null);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getRevisionsByDraftId(draftId: string): Promise<StoredRevision[]> {
		return this.transaction(STORES.REVISIONS, 'readonly', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			const index = store.index('draftId');
			return new Promise<StoredRevision[]>((resolve, reject) => {
				const request = index.getAll(draftId);
				request.onsuccess = () => {
					// Sort by createdAt descending
					const results = (request.result ?? []).sort((a, b) => b.createdAt - a.createdAt);
					resolve(results);
				};
				request.onerror = () => reject(request.error);
			});
		});
	}

	async getAllRevisions(): Promise<StoredRevision[]> {
		return this.transaction(STORES.REVISIONS, 'readonly', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			return new Promise<StoredRevision[]>((resolve, reject) => {
				const request = store.getAll();
				request.onsuccess = () => resolve(request.result ?? []);
				request.onerror = () => reject(request.error);
			});
		});
	}

	async saveRevision(revision: StoredRevision): Promise<void> {
		return this.transaction(STORES.REVISIONS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.put(revision);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async deleteRevision(id: string): Promise<void> {
		return this.transaction(STORES.REVISIONS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.delete(id);
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	async clearRevisions(): Promise<void> {
		return this.transaction(STORES.REVISIONS, 'readwrite', async (stores) => {
			const store = stores.get(STORES.REVISIONS)!;
			return new Promise<void>((resolve, reject) => {
				const request = store.clear();
				request.onsuccess = () => resolve();
				request.onerror = () => reject(request.error);
			});
		});
	}

	// ===========================================================================
	// Database Statistics
	// ===========================================================================

	async getStats(): Promise<DatabaseStats> {
		const [drafts, pendingChanges, assets, revisions] = await Promise.all([
			this.getAllDrafts(),
			this.getAllPendingChanges(),
			this.getAllAssets(),
			this.getAllRevisions()
		]);

		const assetsSize = assets.reduce((total, asset) => total + asset.size, 0);

		return {
			draftsCount: drafts.length,
			pendingChangesCount: pendingChanges.length,
			assetsCount: assets.length,
			assetsSize,
			revisionsCount: revisions.length,
			oldestDraft: drafts.length ? Math.min(...drafts.map((d) => d.createdAt)) : null,
			newestDraft: drafts.length ? Math.max(...drafts.map((d) => d.updatedAt)) : null
		};
	}

	// ===========================================================================
	// Database Management
	// ===========================================================================

	async deleteDatabase(): Promise<void> {
		this.close();

		return new Promise<void>((resolve, reject) => {
			const request = indexedDB.deleteDatabase(DB_NAME);
			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
			request.onblocked = () => {
				logger.warn('[BlogEditorDB] Database deletion blocked');
			};
		});
	}

	async exportData(): Promise<ExportedData> {
		const [drafts, pendingChanges, revisions] = await Promise.all([
			this.getAllDrafts(),
			this.getAllPendingChanges(),
			this.getAllRevisions()
		]);

		return {
			version: DB_VERSION,
			exportedAt: Date.now(),
			drafts,
			pendingChanges,
			revisions
			// Assets are excluded due to size (Blobs)
		};
	}

	async importData(data: ExportedData): Promise<ImportResult> {
		const result: ImportResult = {
			drafts: 0,
			pendingChanges: 0,
			revisions: 0,
			errors: []
		};

		// Import drafts
		for (const draft of data.drafts) {
			try {
				await this.saveDraft(draft);
				result.drafts++;
			} catch (error) {
				result.errors.push(`Draft ${draft.id}: ${String(error)}`);
			}
		}

		// Import pending changes
		for (const change of data.pendingChanges) {
			try {
				await this.savePendingChange(change);
				result.pendingChanges++;
			} catch (error) {
				result.errors.push(`PendingChange ${change.id}: ${String(error)}`);
			}
		}

		// Import revisions
		for (const revision of data.revisions) {
			try {
				await this.saveRevision(revision);
				result.revisions++;
			} catch (error) {
				result.errors.push(`Revision ${revision.id}: ${String(error)}`);
			}
		}

		return result;
	}
}

// =============================================================================
// Helper Types
// =============================================================================

export interface CleanupResult {
	drafts: number;
	pendingChanges: number;
	assets: number;
	revisions: number;
}

export interface DatabaseStats {
	draftsCount: number;
	pendingChangesCount: number;
	assetsCount: number;
	assetsSize: number;
	revisionsCount: number;
	oldestDraft: number | null;
	newestDraft: number | null;
}

export interface ExportedData {
	version: number;
	exportedAt: number;
	drafts: Draft[];
	pendingChanges: PendingChange[];
	revisions: StoredRevision[];
}

export interface ImportResult {
	drafts: number;
	pendingChanges: number;
	revisions: number;
	errors: string[];
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Generates a checksum for draft content to detect changes
 */
export function generateChecksum(content: unknown): string {
	const str = JSON.stringify(content);
	let hash = 0;
	for (let i = 0; i < str.length; i++) {
		const char = str.charCodeAt(i);
		hash = (hash << 5) - hash + char;
		hash = hash & hash; // Convert to 32-bit integer
	}
	return Math.abs(hash).toString(16).padStart(8, '0');
}

/**
 * Creates a new draft from a Post
 */
export function createDraftFromPost(post: Post): Draft {
	const now = Date.now();
	return {
		id: crypto.randomUUID(),
		postId: post.id,
		title: post.title,
		slug: post.slug,
		excerpt: post.excerpt,
		content_blocks: post.content_blocks ?? [],
		featured_image: post.featured_image,
		meta_title: post.meta_title ?? null,
		meta_description: post.meta_description ?? null,
		canonical_url: post.canonical_url ?? null,
		schema_markup: post.schema_markup ?? null,
		indexable: post.indexable ?? true,
		authorId: post.author?.id ?? null,
		createdAt: now,
		updatedAt: now,
		version: 1,
		syncedAt: now,
		checksum: generateChecksum(post.content_blocks)
	};
}

/**
 * Creates a new empty draft
 */
export function createNewDraft(authorId: number | null = null): Draft {
	const now = Date.now();
	return {
		id: crypto.randomUUID(),
		postId: null,
		title: 'Untitled Draft',
		slug: '',
		excerpt: null,
		content_blocks: [],
		featured_image: null,
		meta_title: null,
		meta_description: null,
		canonical_url: null,
		schema_markup: null,
		indexable: true,
		authorId,
		createdAt: now,
		updatedAt: now,
		version: 1,
		syncedAt: null,
		checksum: generateChecksum([])
	};
}

/**
 * Creates a pending change from a draft
 */
export function createPendingChange(
	draft: Draft,
	operation: PendingChange['operation'],
	priority: number = 5
): PendingChange {
	return {
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
}

/**
 * Creates a revision from a draft
 */
export function createRevisionFromDraft(
	draft: Draft,
	type: StoredRevision['type'] = 'auto',
	userId: string = 'system'
): StoredRevision {
	const now = Date.now();
	return {
		id: crypto.randomUUID(),
		draftId: draft.id,
		postId: draft.postId !== null ? String(draft.postId) : null,
		blocks: draft.content_blocks,
		createdAt: now,
		createdBy: userId,
		createdByName: 'System',
		type,
		title: draft.title,
		wordCount: countWords(draft.content_blocks),
		blockCount: draft.content_blocks.length,
		expiresAt: now + MAX_AGE_MS
	};
}

/**
 * Counts words in blocks
 */
function countWords(blocks: Block[]): number {
	let count = 0;
	for (const block of blocks) {
		if (block.content.text) {
			count += block.content.text.split(/\s+/).filter(Boolean).length;
		}
		if (block.content.html) {
			const text = block.content.html.replace(/<[^>]*>/g, ' ');
			count += text.split(/\s+/).filter(Boolean).length;
		}
		if (block.content.children) {
			count += countWords(block.content.children);
		}
	}
	return count;
}

// =============================================================================
// Singleton Export
// =============================================================================

export const blogEditorDB = new DatabaseConnection();
