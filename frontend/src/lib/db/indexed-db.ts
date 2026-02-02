/**
 * IndexedDB Wrapper
 * ═══════════════════════════════════════════════════════════════════════════
 * Offline-first storage for blocks and documents
 */

import type { Block } from '$lib/components/cms/blocks/types';

const DB_NAME = 'RTP_CMS';
const DB_VERSION = 2;
const BLOCKS_STORE = 'blocks';
const DOCUMENTS_STORE = 'documents';
const DRAFTS_STORE = 'drafts';

export interface Document {
	id: string;
	title: string;
	blocks: Block[];
	createdAt: string;
	updatedAt: string;
	author: string;
	status: 'draft' | 'published' | 'archived';
}

export interface Draft {
	id: string;
	documentId: string;
	blocks: Block[];
	savedAt: string;
	autoSaved: boolean;
}

class BlockDatabase {
	private db: IDBDatabase | null = null;
	private initPromise: Promise<void> | null = null;

	async init(): Promise<void> {
		if (this.db) return;
		if (this.initPromise) return this.initPromise;

		this.initPromise = new Promise((resolve, reject) => {
			if (typeof indexedDB === 'undefined') {
				reject(new Error('IndexedDB not available'));
				return;
			}

			const request = indexedDB.open(DB_NAME, DB_VERSION);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => {
				this.db = request.result;
				resolve();
			};

			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;

				if (!db.objectStoreNames.contains(BLOCKS_STORE)) {
					const blockStore = db.createObjectStore(BLOCKS_STORE, { keyPath: 'id' });
					blockStore.createIndex('type', 'type', { unique: false });
					blockStore.createIndex('updatedAt', 'metadata.updatedAt', { unique: false });
				}

				if (!db.objectStoreNames.contains(DOCUMENTS_STORE)) {
					const docStore = db.createObjectStore(DOCUMENTS_STORE, { keyPath: 'id' });
					docStore.createIndex('status', 'status', { unique: false });
					docStore.createIndex('updatedAt', 'updatedAt', { unique: false });
				}

				if (!db.objectStoreNames.contains(DRAFTS_STORE)) {
					const draftStore = db.createObjectStore(DRAFTS_STORE, { keyPath: 'id' });
					draftStore.createIndex('documentId', 'documentId', { unique: false });
					draftStore.createIndex('savedAt', 'savedAt', { unique: false });
				}
			};
		});

		return this.initPromise;
	}

	private ensureDb(): IDBDatabase {
		if (!this.db) throw new Error('Database not initialized');
		return this.db;
	}

	// Block operations
	async saveBlock(block: Block): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([BLOCKS_STORE], 'readwrite');
			const store = transaction.objectStore(BLOCKS_STORE);
			const request = store.put(block);

			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
		});
	}

	async getBlock(id: string): Promise<Block | null> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([BLOCKS_STORE], 'readonly');
			const store = transaction.objectStore(BLOCKS_STORE);
			const request = store.get(id);

			request.onsuccess = () => resolve(request.result || null);
			request.onerror = () => reject(request.error);
		});
	}

	async getAllBlocks(): Promise<Block[]> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([BLOCKS_STORE], 'readonly');
			const store = transaction.objectStore(BLOCKS_STORE);
			const request = store.getAll();

			request.onsuccess = () => resolve(request.result);
			request.onerror = () => reject(request.error);
		});
	}

	async deleteBlock(id: string): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([BLOCKS_STORE], 'readwrite');
			const store = transaction.objectStore(BLOCKS_STORE);
			const request = store.delete(id);

			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
		});
	}

	// Document operations
	async saveDocument(doc: Document): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DOCUMENTS_STORE], 'readwrite');
			const store = transaction.objectStore(DOCUMENTS_STORE);
			const request = store.put(doc);

			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
		});
	}

	async getDocument(id: string): Promise<Document | null> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DOCUMENTS_STORE], 'readonly');
			const store = transaction.objectStore(DOCUMENTS_STORE);
			const request = store.get(id);

			request.onsuccess = () => resolve(request.result || null);
			request.onerror = () => reject(request.error);
		});
	}

	async getAllDocuments(): Promise<Document[]> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DOCUMENTS_STORE], 'readonly');
			const store = transaction.objectStore(DOCUMENTS_STORE);
			const request = store.getAll();

			request.onsuccess = () => resolve(request.result);
			request.onerror = () => reject(request.error);
		});
	}

	// Draft operations
	async saveDraft(draft: Draft): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DRAFTS_STORE], 'readwrite');
			const store = transaction.objectStore(DRAFTS_STORE);
			const request = store.put(draft);

			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
		});
	}

	async getDraft(documentId: string): Promise<Draft | null> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DRAFTS_STORE], 'readonly');
			const store = transaction.objectStore(DRAFTS_STORE);
			const index = store.index('documentId');
			const request = index.get(documentId);

			request.onsuccess = () => resolve(request.result || null);
			request.onerror = () => reject(request.error);
		});
	}

	async deleteDraft(id: string): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([DRAFTS_STORE], 'readwrite');
			const store = transaction.objectStore(DRAFTS_STORE);
			const request = store.delete(id);

			request.onsuccess = () => resolve();
			request.onerror = () => reject(request.error);
		});
	}

	// Utility
	async clear(): Promise<void> {
		await this.init();
		const db = this.ensureDb();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction(
				[BLOCKS_STORE, DOCUMENTS_STORE, DRAFTS_STORE],
				'readwrite'
			);

			transaction.objectStore(BLOCKS_STORE).clear();
			transaction.objectStore(DOCUMENTS_STORE).clear();
			transaction.objectStore(DRAFTS_STORE).clear();

			transaction.oncomplete = () => resolve();
			transaction.onerror = () => reject(transaction.error);
		});
	}

	close(): void {
		if (this.db) {
			this.db.close();
			this.db = null;
			this.initPromise = null;
		}
	}
}

export const blockDB = new BlockDatabase();
