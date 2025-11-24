/**
 * Media API Client - Enterprise Media Management
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete API client for media management including uploads, optimization,
 * AI metadata, folders, versioning, and usage tracking.
 *
 * @version 1.0.0
 */

import { get } from 'svelte/store';
import { authStore } from '$lib/stores/auth';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api';

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface MediaFile {
	id: string;
	filename: string;
	original_filename: string;
	title: string;
	alt_text: string;
	description?: string;
	file_type: 'image' | 'video' | 'audio' | 'document' | 'other';
	mime_type: string;
	file_size: number;
	file_path: string;
	url: string;
	cdn_url?: string;
	thumbnail_url?: string;
	width?: number;
	height?: number;
	duration?: number;
	folder_id?: string;
	tags: string[];
	metadata: MediaMetadata;
	ai_metadata?: AIMetadata;
	versions: MediaVersion[];
	usage_count: number;
	is_optimized: boolean;
	has_webp: boolean;
	created_by: number;
	created_at: string;
	updated_at: string;
}

export interface MediaMetadata {
	exif?: Record<string, any>;
	dimensions?: { width: number; height: number };
	color_palette?: string[];
	dominant_color?: string;
	file_hash?: string;
	compression_ratio?: number;
	quality_score?: number;
}

export interface AIMetadata {
	generated_alt_text: string;
	generated_title: string;
	generated_tags: string[];
	keywords: string[];
	detected_objects: string[];
	detected_faces?: number;
	scene_type?: string;
	suggested_filename?: string;
	seo_score?: number;
	confidence_score: number;
	generated_at: string;
}

export interface MediaVersion {
	id: string;
	version_number: number;
	file_path: string;
	url: string;
	file_size: number;
	changes: string;
	created_by: number;
	created_at: string;
}

export interface MediaFolder {
	id: string;
	name: string;
	slug: string;
	parent_id?: string;
	path: string;
	description?: string;
	file_count: number;
	total_size: number;
	permissions: FolderPermissions;
	created_at: string;
	updated_at: string;
}

export interface FolderPermissions {
	is_public: boolean;
	allowed_roles: string[];
	allowed_users: number[];
}

export interface MediaUsage {
	id: string;
	media_id: string;
	usage_type: 'blog' | 'page' | 'email' | 'seo' | 'trading_room' | 'popup' | 'form' | 'other';
	entity_type: string;
	entity_id: string;
	entity_title?: string;
	url?: string;
	created_at: string;
}

export interface UploadProgress {
	file: File;
	progress: number;
	status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
	error?: string;
	media_id?: string;
	chunk_index?: number;
	total_chunks?: number;
}

export interface OptimizationResult {
	original_size: number;
	optimized_size: number;
	savings_bytes: number;
	savings_percent: number;
	webp_created: boolean;
	thumbnails_created: number;
	processing_time: number;
}

export interface BulkActionResult {
	success: number;
	failed: number;
	errors: Array<{ id: string; error: string }>;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Client
// ═══════════════════════════════════════════════════════════════════════════

class MediaApiClient {
	private getAuthHeaders(): Record<string, string> {
		const auth = get(authStore);
		const headers: Record<string, string> = {
			Accept: 'application/json'
		};

		if (auth.token) {
			headers['Authorization'] = `Bearer ${auth.token}`;
		}

		return headers;
	}

	private async request<T>(
		method: string,
		endpoint: string,
		body?: any,
		params?: Record<string, any>,
		isFormData: boolean = false
	): Promise<T> {
		let url = `${API_BASE_URL}${endpoint}`;

		if (params) {
			const searchParams = new URLSearchParams();
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					searchParams.append(key, String(value));
				}
			});
			const queryString = searchParams.toString();
			if (queryString) {
				url += `?${queryString}`;
			}
		}

		const headers = this.getAuthHeaders();
		if (!isFormData) {
			headers['Content-Type'] = 'application/json';
		}

		const response = await fetch(url, {
			method,
			headers,
			body: isFormData ? body : body ? JSON.stringify(body) : undefined
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			throw new Error(error.message || `HTTP ${response.status}`);
		}

		return response.json();
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Media Files
	// ═══════════════════════════════════════════════════════════════════════

	async getFiles(params?: {
		folder_id?: string;
		file_type?: string;
		search?: string;
		tags?: string[];
		sort?: string;
		order?: 'asc' | 'desc';
		page?: number;
		per_page?: number;
	}): Promise<{
		files: MediaFile[];
		pagination: {
			page: number;
			per_page: number;
			total: number;
			total_pages: number;
		};
	}> {
		return this.request('GET', '/admin/media/files', undefined, params);
	}

	async getFile(id: string): Promise<{ file: MediaFile }> {
		return this.request('GET', `/admin/media/files/${id}`);
	}

	async updateFile(
		id: string,
		data: {
			title?: string;
			alt_text?: string;
			description?: string;
			tags?: string[];
			folder_id?: string;
		}
	): Promise<{ success: boolean; file: MediaFile }> {
		return this.request('PUT', `/admin/media/files/${id}`, data);
	}

	async deleteFile(id: string, force: boolean = false): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/media/files/${id}`, undefined, { force });
	}

	async bulkDelete(ids: string[], force: boolean = false): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/delete', { ids, force });
	}

	async bulkMove(ids: string[], folder_id: string): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/move', { ids, folder_id });
	}

	async bulkTag(ids: string[], tags: string[]): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/tag', { ids, tags });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Upload
	// ═══════════════════════════════════════════════════════════════════════

	async uploadFile(
		file: File,
		options?: {
			folder_id?: string;
			title?: string;
			alt_text?: string;
			tags?: string[];
			optimize?: boolean;
			generate_webp?: boolean;
		},
		onProgress?: (progress: number) => void
	): Promise<{ success: boolean; file: MediaFile }> {
		const formData = new FormData();
		formData.append('file', file);

		if (options?.folder_id) formData.append('folder_id', options.folder_id);
		if (options?.title) formData.append('title', options.title);
		if (options?.alt_text) formData.append('alt_text', options.alt_text);
		if (options?.tags) formData.append('tags', JSON.stringify(options.tags));
		if (options?.optimize !== undefined) formData.append('optimize', String(options.optimize));
		if (options?.generate_webp !== undefined) formData.append('generate_webp', String(options.generate_webp));

		// For progress tracking, we need XMLHttpRequest
		if (onProgress) {
			return this.uploadWithProgress(formData, onProgress);
		}

		return this.request('POST', '/admin/media/upload', formData, undefined, true);
	}

	private uploadWithProgress(
		formData: FormData,
		onProgress: (progress: number) => void
	): Promise<{ success: boolean; file: MediaFile }> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			const auth = get(authStore);

			xhr.upload.addEventListener('progress', (e) => {
				if (e.lengthComputable) {
					const progress = (e.loaded / e.total) * 100;
					onProgress(progress);
				}
			});

			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					resolve(JSON.parse(xhr.responseText));
				} else {
					reject(new Error(`Upload failed: ${xhr.status}`));
				}
			});

			xhr.addEventListener('error', () => {
				reject(new Error('Upload failed'));
			});

			xhr.open('POST', `${API_BASE_URL}/admin/media/upload`);
			if (auth.token) {
				xhr.setRequestHeader('Authorization', `Bearer ${auth.token}`);
			}

			xhr.send(formData);
		});
	}

	async uploadChunk(
		file: File,
		chunkIndex: number,
		totalChunks: number,
		uploadId: string,
		chunk: Blob
	): Promise<{ success: boolean; upload_id: string; complete: boolean; file?: MediaFile }> {
		const formData = new FormData();
		formData.append('chunk', chunk);
		formData.append('chunk_index', String(chunkIndex));
		formData.append('total_chunks', String(totalChunks));
		formData.append('upload_id', uploadId);
		formData.append('filename', file.name);
		formData.append('file_size', String(file.size));

		return this.request('POST', '/admin/media/upload/chunk', formData, undefined, true);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Folders
	// ═══════════════════════════════════════════════════════════════════════

	async getFolders(): Promise<{ folders: MediaFolder[] }> {
		return this.request('GET', '/admin/media/folders');
	}

	async getFolder(id: string): Promise<{ folder: MediaFolder }> {
		return this.request('GET', `/admin/media/folders/${id}`);
	}

	async createFolder(data: {
		name: string;
		parent_id?: string;
		description?: string;
		permissions?: Partial<FolderPermissions>;
	}): Promise<{ success: boolean; folder: MediaFolder }> {
		return this.request('POST', '/admin/media/folders', data);
	}

	async updateFolder(
		id: string,
		data: {
			name?: string;
			description?: string;
			permissions?: Partial<FolderPermissions>;
		}
	): Promise<{ success: boolean; folder: MediaFolder }> {
		return this.request('PUT', `/admin/media/folders/${id}`, data);
	}

	async deleteFolder(id: string, deleteFiles: boolean = false): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/media/folders/${id}`, undefined, { delete_files: deleteFiles });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// AI Metadata
	// ═══════════════════════════════════════════════════════════════════════

	async generateAIMetadata(id: string): Promise<{ success: boolean; metadata: AIMetadata }> {
		return this.request('POST', `/admin/media/files/${id}/ai-metadata`);
	}

	async applyAIMetadata(id: string, fields: string[]): Promise<{ success: boolean; file: MediaFile }> {
		return this.request('POST', `/admin/media/files/${id}/apply-ai-metadata`, { fields });
	}

	async bulkGenerateAI(ids: string[]): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/generate-ai', { ids });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Optimization
	// ═══════════════════════════════════════════════════════════════════════

	async optimizeFile(id: string): Promise<{ success: boolean; result: OptimizationResult }> {
		return this.request('POST', `/admin/media/files/${id}/optimize`);
	}

	async generateWebP(id: string): Promise<{ success: boolean; webp_url: string }> {
		return this.request('POST', `/admin/media/files/${id}/webp`);
	}

	async generateThumbnails(
		id: string,
		sizes: Array<{ width: number; height: number; name: string }>
	): Promise<{ success: boolean; thumbnails: Array<{ name: string; url: string }> }> {
		return this.request('POST', `/admin/media/files/${id}/thumbnails`, { sizes });
	}

	async bulkOptimize(ids: string[]): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/optimize', { ids });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Versioning
	// ═══════════════════════════════════════════════════════════════════════

	async getVersions(id: string): Promise<{ versions: MediaVersion[] }> {
		return this.request('GET', `/admin/media/files/${id}/versions`);
	}

	async createVersion(id: string, file: File, changes: string): Promise<{ success: boolean; version: MediaVersion }> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('changes', changes);

		return this.request('POST', `/admin/media/files/${id}/versions`, formData, undefined, true);
	}

	async restoreVersion(id: string, versionId: string): Promise<{ success: boolean; file: MediaFile }> {
		return this.request('POST', `/admin/media/files/${id}/versions/${versionId}/restore`);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Usage Tracking
	// ═══════════════════════════════════════════════════════════════════════

	async getUsage(id: string): Promise<{ usage: MediaUsage[] }> {
		return this.request('GET', `/admin/media/files/${id}/usage`);
	}

	async trackUsage(data: {
		media_id: string;
		usage_type: string;
		entity_type: string;
		entity_id: string;
		entity_title?: string;
		url?: string;
	}): Promise<{ success: boolean }> {
		return this.request('POST', '/admin/media/usage', data);
	}

	async removeUsage(id: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/media/usage/${id}`);
	}

	async getUnusedFiles(days: number = 90): Promise<{ files: MediaFile[] }> {
		return this.request('GET', '/admin/media/unused', undefined, { days });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getStats(): Promise<{
		total_files: number;
		total_size: number;
		by_type: Record<string, number>;
		by_folder: Record<string, number>;
		optimization_stats: {
			optimized: number;
			unoptimized: number;
			total_savings: number;
		};
		ai_metadata_stats: {
			with_ai: number;
			without_ai: number;
		};
	}> {
		return this.request('GET', '/admin/media/stats');
	}

	async getMostUsed(limit: number = 10): Promise<{ files: Array<MediaFile & { usage_count: number }> }> {
		return this.request('GET', '/admin/media/most-used', undefined, { limit });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Search
	// ═══════════════════════════════════════════════════════════════════════

	async search(query: string, filters?: {
		file_type?: string;
		folder_id?: string;
		tags?: string[];
		date_from?: string;
		date_to?: string;
	}): Promise<{ files: MediaFile[] }> {
		return this.request('GET', '/admin/media/search', undefined, { query, ...filters });
	}

	async findSimilar(id: string, limit: number = 10): Promise<{ files: MediaFile[] }> {
		return this.request('GET', `/admin/media/files/${id}/similar`, undefined, { limit });
	}

	async findDuplicates(): Promise<{ duplicates: Array<{ hash: string; files: MediaFile[] }> }> {
		return this.request('GET', '/admin/media/duplicates');
	}
}

// Export singleton instance
export const mediaApi = new MediaApiClient();
export default mediaApi;
