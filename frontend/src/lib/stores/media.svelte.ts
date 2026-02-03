/**
 * Media Store - Enterprise Media Management State (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for media files, folders, uploads,
 * and selections across the Revolution Trading platform.
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

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
// Media Store (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let mediaState = $state<MediaState>({ ...initialMediaState });

export const mediaStore = {
	get state() {
		return mediaState;
	},

	get files() {
		return mediaState.files;
	},

	get folders() {
		return mediaState.folders;
	},

	get selectedFiles() {
		return mediaState.selectedFiles;
	},

	get isLoading() {
		return mediaState.isLoading;
	},

	get viewMode() {
		return mediaState.viewMode;
	},

	get currentFolder() {
		return mediaState.currentFolder;
	},

	get pagination() {
		return mediaState.pagination;
	},

	// ───────────────────────────────────────────────────────────────────
	// Load Files
	// ───────────────────────────────────────────────────────────────────

	async loadFiles(page: number = 1) {
		mediaState = { ...mediaState, isLoading: true, error: null };

		try {
			const response = await mediaApi.getFiles({
				...(mediaState.currentFolder && { folder_id: mediaState.currentFolder }),
				...(mediaState.filterType && { file_type: mediaState.filterType }),
				...(mediaState.searchQuery && { search: mediaState.searchQuery }),
				sort: mediaState.sortBy,
				order: mediaState.sortOrder,
				page,
				per_page: mediaState.pagination.per_page
			});

			mediaState = {
				...mediaState,
				files: response.files,
				pagination: response.pagination,
				isLoading: false
			};
		} catch (error) {
			mediaState = {
				...mediaState,
				error: error instanceof Error ? error.message : 'Failed to load files',
				isLoading: false
			};
		}
	},

	// ───────────────────────────────────────────────────────────────────
	// Load Folders
	// ───────────────────────────────────────────────────────────────────

	async loadFolders() {
		try {
			const response = await mediaApi.getFolders();
			mediaState = { ...mediaState, folders: response.folders };
		} catch (error) {
			console.error('Failed to load folders:', error);
		}
	},

	// ───────────────────────────────────────────────────────────────────
	// Navigation
	// ───────────────────────────────────────────────────────────────────

	setCurrentFolder(folderId: string | null) {
		mediaState = { ...mediaState, currentFolder: folderId, selectedFiles: new Set() };
		this.loadFiles(1);
	},

	// ───────────────────────────────────────────────────────────────────
	// Selection
	// ───────────────────────────────────────────────────────────────────

	selectFile(id: string) {
		const newSelected = new Set(mediaState.selectedFiles);
		newSelected.add(id);
		mediaState = { ...mediaState, selectedFiles: newSelected };
	},

	deselectFile(id: string) {
		const newSelected = new Set(mediaState.selectedFiles);
		newSelected.delete(id);
		mediaState = { ...mediaState, selectedFiles: newSelected };
	},

	toggleFileSelection(id: string) {
		const newSelected = new Set(mediaState.selectedFiles);
		if (newSelected.has(id)) {
			newSelected.delete(id);
		} else {
			newSelected.add(id);
		}
		mediaState = { ...mediaState, selectedFiles: newSelected };
	},

	selectAll() {
		mediaState = {
			...mediaState,
			selectedFiles: new Set(mediaState.files.map((f) => f.id))
		};
	},

	deselectAll() {
		mediaState = { ...mediaState, selectedFiles: new Set() };
	},

	// ───────────────────────────────────────────────────────────────────
	// View & Sort
	// ───────────────────────────────────────────────────────────────────

	setViewMode(mode: 'grid' | 'list') {
		mediaState = { ...mediaState, viewMode: mode };
		if (browser) {
			localStorage.setItem('media_view_mode', mode);
		}
	},

	setSortBy(sortBy: 'name' | 'date' | 'size' | 'type') {
		mediaState = { ...mediaState, sortBy };
		this.loadFiles(1);
	},

	setSortOrder(order: 'asc' | 'desc') {
		mediaState = { ...mediaState, sortOrder: order };
		this.loadFiles(1);
	},

	// ───────────────────────────────────────────────────────────────────
	// Search & Filter
	// ───────────────────────────────────────────────────────────────────

	setSearchQuery(query: string) {
		mediaState = { ...mediaState, searchQuery: query };
		// Debounce search
		setTimeout(() => {
			this.loadFiles(1);
		}, 300);
	},

	setFilterType(type: string | null) {
		mediaState = { ...mediaState, filterType: type };
		this.loadFiles(1);
	},

	// ───────────────────────────────────────────────────────────────────
	// File Operations
	// ───────────────────────────────────────────────────────────────────

	async deleteFile(id: string, force: boolean = false) {
		await mediaApi.deleteFile(id, force);
		mediaState = {
			...mediaState,
			files: mediaState.files.filter((f) => f.id !== id),
			selectedFiles: new Set([...mediaState.selectedFiles].filter((fid) => fid !== id))
		};
	},

	async bulkDelete(force: boolean = false) {
		const ids = Array.from(mediaState.selectedFiles);

		if (ids.length === 0) return;

		await mediaApi.bulkDelete(ids, force);
		mediaState = {
			...mediaState,
			files: mediaState.files.filter((f) => !ids.includes(f.id)),
			selectedFiles: new Set()
		};
	},

	async bulkMove(folderId: string) {
		const ids = Array.from(mediaState.selectedFiles);

		if (ids.length === 0) return;

		await mediaApi.bulkMove(ids, folderId);
		this.loadFiles();
		this.deselectAll();
	},

	// ───────────────────────────────────────────────────────────────────
	// Folder Operations
	// ───────────────────────────────────────────────────────────────────

	async createFolder(data: { name: string; parent_id?: string; description?: string }) {
		const response = await mediaApi.createFolder(data);
		mediaState = {
			...mediaState,
			folders: [...mediaState.folders, response.folder]
		};
		return response.folder;
	},

	async deleteFolder(id: string, deleteFiles: boolean = false) {
		await mediaApi.deleteFolder(id, deleteFiles);
		mediaState = {
			...mediaState,
			folders: mediaState.folders.filter((f) => f.id !== id),
			currentFolder: mediaState.currentFolder === id ? null : mediaState.currentFolder
		};
	},

	// ───────────────────────────────────────────────────────────────────
	// State Management
	// ───────────────────────────────────────────────────────────────────

	reset() {
		mediaState = { ...initialMediaState };
	},

	clearError() {
		mediaState = { ...mediaState, error: null };
	},

	// ───────────────────────────────────────────────────────────────────
	// Initialize
	// ───────────────────────────────────────────────────────────────────

	async initialize() {
		// Load saved view mode
		if (browser) {
			const savedViewMode = localStorage.getItem('media_view_mode');
			if (savedViewMode === 'grid' || savedViewMode === 'list') {
				mediaState = { ...mediaState, viewMode: savedViewMode };
			}
		}

		await this.loadFolders();
		await this.loadFiles();
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Upload Store (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let uploadState = $state<UploadState>({ ...initialUploadState });

export const uploadStore = {
	get state() {
		return uploadState;
	},

	get uploads() {
		return uploadState.uploads;
	},

	get isUploading() {
		return uploadState.isUploading;
	},

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

		uploadState = {
			...uploadState,
			uploads: uploadMap,
			isUploading: true
		};

		// Upload files sequentially or in parallel
		for (const [id, upload] of uploadMap.entries()) {
			try {
				const newUploads = new Map(uploadState.uploads);
				newUploads.set(id, { ...upload, status: 'uploading' });
				uploadState = { ...uploadState, uploads: newUploads };

				const result = await mediaApi.uploadFile(upload.file, options, (progress) => {
					const progressUploads = new Map(uploadState.uploads);
					const current = progressUploads.get(id);
					if (current) {
						progressUploads.set(id, { ...current, progress });
					}
					uploadState = { ...uploadState, uploads: progressUploads };
				});

				const completeUploads = new Map(uploadState.uploads);
				completeUploads.set(id, {
					...upload,
					status: 'complete',
					progress: 100,
					media_id: result.file.id
				});
				uploadState = { ...uploadState, uploads: completeUploads };

				// Refresh media list
				mediaStore.loadFiles();
			} catch (error) {
				const errorUploads = new Map(uploadState.uploads);
				errorUploads.set(id, {
					...upload,
					status: 'error',
					error: error instanceof Error ? error.message : 'Upload failed'
				});
				uploadState = { ...uploadState, uploads: errorUploads };
			}
		}

		uploadState = { ...uploadState, isUploading: false };

		// Clear completed uploads after 5 seconds
		setTimeout(() => {
			const cleanUploads = new Map(uploadState.uploads);
			for (const [id, upload] of cleanUploads.entries()) {
				if (upload.status === 'complete') {
					cleanUploads.delete(id);
				}
			}
			uploadState = { ...uploadState, uploads: cleanUploads };
		}, 5000);
	},

	clearUploads() {
		uploadState = { ...initialUploadState };
	},

	removeUpload(id: string) {
		const newUploads = new Map(uploadState.uploads);
		newUploads.delete(id);
		uploadState = { ...uploadState, uploads: newUploads };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════

export function getCurrentFiles() { return mediaState.files; }
export function getCurrentFolders() { return mediaState.folders; }
export function getSelectedFilesSet() { return mediaState.selectedFiles; }
export function getSelectedCount() { return mediaState.selectedFiles.size; }
export function getHasSelection() { return mediaState.selectedFiles.size > 0; }
export function getCurrentViewMode() { return mediaState.viewMode; }
export function getIsMediaLoading() { return mediaState.isLoading; }

// Upload getters
export function getActiveUploads() { return Array.from(uploadState.uploads.values()); }
export function getUploadCount() { return uploadState.uploads.size; }
export function getIsCurrentlyUploading() { return uploadState.isUploading; }
export function getUploadProgress() {
	const uploads = Array.from(uploadState.uploads.values());
	if (uploads.length === 0) return 0;
	const total = uploads.reduce((sum, u) => sum + u.progress, 0);
	return total / uploads.length;
}
