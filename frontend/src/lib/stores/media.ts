/**
 * Media Store - Enterprise Media Management State
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for media files, folders, uploads,
 * and selections across the Revolution Trading platform.
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { mediaApi, type MediaFile, type MediaFolder, type UploadProgress } from '$lib/api/media';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface MediaState {
	files: MediaFile[];
	folders: MediaFolder[];
	currentFolder: string | null;
	selectedFiles: Set<string>;
	viewMode: 'grid' | 'list';
	sortBy: 'name' | 'date' | 'size' | 'type';
	sortOrder: 'asc' | 'desc';
	searchQuery: string;
	filterType: string | null;
	isLoading: boolean;
	error: string | null;
	pagination: {
		page: number;
		per_page: number;
		total: number;
		total_pages: number;
	};
}

export interface UploadState {
	uploads: Map<string, UploadProgress>;
	isUploading: boolean;
	totalProgress: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Initial State
// ═══════════════════════════════════════════════════════════════════════════

const initialMediaState: MediaState = {
	files: [],
	folders: [],
	currentFolder: null,
	selectedFiles: new Set(),
	viewMode: 'grid',
	sortBy: 'date',
	sortOrder: 'desc',
	searchQuery: '',
	filterType: null,
	isLoading: false,
	error: null,
	pagination: {
		page: 1,
		per_page: 50,
		total: 0,
		total_pages: 0
	}
};

const initialUploadState: UploadState = {
	uploads: new Map(),
	isUploading: false,
	totalProgress: 0
};

// ═══════════════════════════════════════════════════════════════════════════
// Media Store
// ═══════════════════════════════════════════════════════════════════════════

function createMediaStore() {
	const { subscribe, set, update } = writable<MediaState>(initialMediaState);

	return {
		subscribe,

		// ───────────────────────────────────────────────────────────────────
		// Load Files
		// ───────────────────────────────────────────────────────────────────

		async loadFiles(page: number = 1) {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const currentState = get({ subscribe });
				const response = await mediaApi.getFiles({
					folder_id: currentState.currentFolder || undefined,
					file_type: currentState.filterType || undefined,
					search: currentState.searchQuery || undefined,
					sort: currentState.sortBy,
					order: currentState.sortOrder,
					page,
					per_page: currentState.pagination.per_page
				});

				update((state) => ({
					...state,
					files: response.files,
					pagination: response.pagination,
					isLoading: false
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load files',
					isLoading: false
				}));
			}
		},

		// ───────────────────────────────────────────────────────────────────
		// Load Folders
		// ───────────────────────────────────────────────────────────────────

		async loadFolders() {
			try {
				const response = await mediaApi.getFolders();
				update((state) => ({ ...state, folders: response.folders }));
			} catch (error) {
				console.error('Failed to load folders:', error);
			}
		},

		// ───────────────────────────────────────────────────────────────────
		// Navigation
		// ───────────────────────────────────────────────────────────────────

		setCurrentFolder(folderId: string | null) {
			update((state) => ({ ...state, currentFolder: folderId, selectedFiles: new Set() }));
			this.loadFiles(1);
		},

		// ───────────────────────────────────────────────────────────────────
		// Selection
		// ───────────────────────────────────────────────────────────────────

		selectFile(id: string) {
			update((state) => {
				const newSelected = new Set(state.selectedFiles);
				newSelected.add(id);
				return { ...state, selectedFiles: newSelected };
			});
		},

		deselectFile(id: string) {
			update((state) => {
				const newSelected = new Set(state.selectedFiles);
				newSelected.delete(id);
				return { ...state, selectedFiles: newSelected };
			});
		},

		toggleFileSelection(id: string) {
			update((state) => {
				const newSelected = new Set(state.selectedFiles);
				if (newSelected.has(id)) {
					newSelected.delete(id);
				} else {
					newSelected.add(id);
				}
				return { ...state, selectedFiles: newSelected };
			});
		},

		selectAll() {
			update((state) => ({
				...state,
				selectedFiles: new Set(state.files.map((f) => f.id))
			}));
		},

		deselectAll() {
			update((state) => ({ ...state, selectedFiles: new Set() }));
		},

		// ───────────────────────────────────────────────────────────────────
		// View & Sort
		// ───────────────────────────────────────────────────────────────────

		setViewMode(mode: 'grid' | 'list') {
			update((state) => ({ ...state, viewMode: mode }));
			if (browser) {
				localStorage.setItem('media_view_mode', mode);
			}
		},

		setSortBy(sortBy: 'name' | 'date' | 'size' | 'type') {
			update((state) => ({ ...state, sortBy }));
			this.loadFiles(1);
		},

		setSortOrder(order: 'asc' | 'desc') {
			update((state) => ({ ...state, sortOrder: order }));
			this.loadFiles(1);
		},

		// ───────────────────────────────────────────────────────────────────
		// Search & Filter
		// ───────────────────────────────────────────────────────────────────

		setSearchQuery(query: string) {
			update((state) => ({ ...state, searchQuery: query }));
			// Debounce search
			setTimeout(() => {
				this.loadFiles(1);
			}, 300);
		},

		setFilterType(type: string | null) {
			update((state) => ({ ...state, filterType: type }));
			this.loadFiles(1);
		},

		// ───────────────────────────────────────────────────────────────────
		// File Operations
		// ───────────────────────────────────────────────────────────────────

		async deleteFile(id: string, force: boolean = false) {
			try {
				await mediaApi.deleteFile(id, force);
				update((state) => ({
					...state,
					files: state.files.filter((f) => f.id !== id),
					selectedFiles: new Set([...state.selectedFiles].filter((fid) => fid !== id))
				}));
			} catch (error) {
				throw error;
			}
		},

		async bulkDelete(force: boolean = false) {
			const state = get({ subscribe });
			const ids = Array.from(state.selectedFiles);

			if (ids.length === 0) return;

			try {
				await mediaApi.bulkDelete(ids, force);
				update((s) => ({
					...s,
					files: s.files.filter((f) => !ids.includes(f.id)),
					selectedFiles: new Set()
				}));
			} catch (error) {
				throw error;
			}
		},

		async bulkMove(folderId: string) {
			const state = get({ subscribe });
			const ids = Array.from(state.selectedFiles);

			if (ids.length === 0) return;

			try {
				await mediaApi.bulkMove(ids, folderId);
				this.loadFiles();
				this.deselectAll();
			} catch (error) {
				throw error;
			}
		},

		// ───────────────────────────────────────────────────────────────────
		// Folder Operations
		// ───────────────────────────────────────────────────────────────────

		async createFolder(data: { name: string; parent_id?: string; description?: string }) {
			try {
				const response = await mediaApi.createFolder(data);
				update((state) => ({
					...state,
					folders: [...state.folders, response.folder]
				}));
				return response.folder;
			} catch (error) {
				throw error;
			}
		},

		async deleteFolder(id: string, deleteFiles: boolean = false) {
			try {
				await mediaApi.deleteFolder(id, deleteFiles);
				update((state) => ({
					...state,
					folders: state.folders.filter((f) => f.id !== id),
					currentFolder: state.currentFolder === id ? null : state.currentFolder
				}));
			} catch (error) {
				throw error;
			}
		},

		// ───────────────────────────────────────────────────────────────────
		// State Management
		// ───────────────────────────────────────────────────────────────────

		reset() {
			set(initialMediaState);
		},

		clearError() {
			update((state) => ({ ...state, error: null }));
		},

		// ───────────────────────────────────────────────────────────────────
		// Initialize
		// ───────────────────────────────────────────────────────────────────

		async initialize() {
			// Load saved view mode
			if (browser) {
				const savedViewMode = localStorage.getItem('media_view_mode');
				if (savedViewMode === 'grid' || savedViewMode === 'list') {
					update((state) => ({ ...state, viewMode: savedViewMode }));
				}
			}

			await this.loadFolders();
			await this.loadFiles();
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Upload Store
// ═══════════════════════════════════════════════════════════════════════════

function createUploadStore() {
	const { subscribe, set, update } = writable<UploadState>(initialUploadState);

	return {
		subscribe,

		async uploadFiles(
			files: File[],
			options?: {
				folder_id?: string;
				optimize?: boolean;
				generate_webp?: boolean;
			}
		) {
			const uploadMap = new Map<string, UploadProgress>();

			files.forEach((file) => {
				const id = `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
				uploadMap.set(id, {
					file,
					progress: 0,
					status: 'pending'
				});
			});

			update((state) => ({
				...state,
				uploads: uploadMap,
				isUploading: true
			}));

			// Upload files sequentially or in parallel
			for (const [id, upload] of uploadMap.entries()) {
				try {
					update((state) => {
						const newUploads = new Map(state.uploads);
						newUploads.set(id, { ...upload, status: 'uploading' });
						return { ...state, uploads: newUploads };
					});

					const result = await mediaApi.uploadFile(upload.file, options, (progress) => {
						update((state) => {
							const newUploads = new Map(state.uploads);
							const current = newUploads.get(id);
							if (current) {
								newUploads.set(id, { ...current, progress });
							}
							return { ...state, uploads: newUploads };
						});
					});

					update((state) => {
						const newUploads = new Map(state.uploads);
						newUploads.set(id, {
							...upload,
							status: 'complete',
							progress: 100,
							media_id: result.file.id
						});
						return { ...state, uploads: newUploads };
					});

					// Refresh media list
					mediaStore.loadFiles();
				} catch (error) {
					update((state) => {
						const newUploads = new Map(state.uploads);
						newUploads.set(id, {
							...upload,
							status: 'error',
							error: error instanceof Error ? error.message : 'Upload failed'
						});
						return { ...state, uploads: newUploads };
					});
				}
			}

			update((state) => ({ ...state, isUploading: false }));

			// Clear completed uploads after 5 seconds
			setTimeout(() => {
				update((state) => {
					const newUploads = new Map(state.uploads);
					for (const [id, upload] of newUploads.entries()) {
						if (upload.status === 'complete') {
							newUploads.delete(id);
						}
					}
					return { ...state, uploads: newUploads };
				});
			}, 5000);
		},

		clearUploads() {
			set(initialUploadState);
		},

		removeUpload(id: string) {
			update((state) => {
				const newUploads = new Map(state.uploads);
				newUploads.delete(id);
				return { ...state, uploads: newUploads };
			});
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Exports
// ═══════════════════════════════════════════════════════════════════════════

export const mediaStore = createMediaStore();
export const uploadStore = createUploadStore();

// Derived stores
export const currentFiles = derived(mediaStore, ($media) => $media.files);
export const currentFolders = derived(mediaStore, ($media) => $media.folders);
export const selectedFiles = derived(mediaStore, ($media) => $media.selectedFiles);
export const selectedCount = derived(selectedFiles, ($selected) => $selected.size);
export const hasSelection = derived(selectedCount, ($count) => $count > 0);
export const viewMode = derived(mediaStore, ($media) => $media.viewMode);
export const isLoading = derived(mediaStore, ($media) => $media.isLoading);

// Upload derived stores
export const activeUploads = derived(uploadStore, ($upload) =>
	Array.from($upload.uploads.values())
);
export const uploadCount = derived(activeUploads, ($uploads) => $uploads.length);
export const isUploading = derived(uploadStore, ($upload) => $upload.isUploading);
export const uploadProgress = derived(activeUploads, ($uploads) => {
	if ($uploads.length === 0) return 0;
	const total = $uploads.reduce((sum, u) => sum + u.progress, 0);
	return total / $uploads.length;
});
