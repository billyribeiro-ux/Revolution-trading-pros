/**
 * Media API Client - Enterprise Media Management
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete API client for media management including uploads, optimization,
 * AI metadata, folders, versioning, and usage tracking.
 *
 * @version 1.0.0
 */

import { get as _get } from 'svelte/store';
import { authStore } from '$lib/stores/auth.svelte';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface MediaFile {
	id: string;
	filename: string;
	original_filename: string;
	title: string;
	alt_text: string;
	/** @deprecated Use alt_text instead */
	alt?: string;
	description?: string;
	caption?: string;
	collection?: string;
	file_type: 'image' | 'video' | 'audio' | 'document' | 'other';
	mime_type: string;
	file_size: number;
	/** Alias for file_size */
	size?: number;
	file_path: string;
	url: string;
	cdn_url?: string;
	thumbnail_url?: string;
	width?: number;
	height?: number;
	/** Computed dimensions object */
	dimensions?: { width: number; height: number };
	duration?: number;
	folder_id?: string;
	tags: string[];
	metadata: MediaMetadata;
	ai_metadata?: AIMetadata;
	exif?: Record<string, unknown>;
	versions: MediaVersion[];
	variants?: MediaVariant[];
	/** Custom properties for extensibility */
	custom_properties?: Record<string, unknown>;
	usage_count: number;
	is_optimized: boolean;
	has_webp: boolean;
	processing_status?: 'pending' | 'processing' | 'completed' | 'failed';
	created_by: number;
	created_at: string;
	updated_at: string;
	uploaded_at?: string;
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

// ═══════════════════════════════════════════════════════════════════════════
// Image Optimization Types (Imagify-like features)
// ═══════════════════════════════════════════════════════════════════════════

export interface MediaVariant {
	type: string;
	size: string | null;
	width: number;
	height: number;
	url: string;
	format: string;
	file_size: number;
	is_retina: boolean;
	savings_percent: number | null;
	lqip: string | null;
	blurhash: string | null;
}

export interface OptimizationPreset {
	id: number;
	name: string;
	slug: string;
	description: string | null;
	quality: {
		webp: number;
		avif: number;
		jpeg: number;
		png: number;
	};
	compression_mode: 'lossless' | 'lossy' | 'auto';
	convert_to_webp: boolean;
	convert_to_avif: boolean;
	generate_retina: boolean;
	responsive_sizes: Record<string, number>;
	is_default: boolean;
}

export interface OptimizationJob {
	id: number;
	media_id: number;
	status: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled';
	progress: number;
	current_operation: string | null;
	attempts: number;
	max_attempts: number;
	original_size: number | null;
	optimized_size: number | null;
	savings_percent: number | null;
	savings_bytes: number;
	processing_time: string;
	error: string | null;
	started_at: string | null;
	completed_at: string | null;
}

export interface OptimizationStatistics {
	total_images: number;
	optimized_images: number;
	pending_optimization: number;
	total_storage: number;
	total_variants: number;
	total_savings_bytes: number;
	total_files: number;
	total_size: number;
	total_videos: number;
	total_documents: number;
	total_downloads: number;
	total_views: number;
	needs_optimization: number;
	unused_files: number;
	storage_by_type: Record<string, { count: number; size: number; human_size: string }>;
	job_stats: {
		pending: number;
		processing: number;
		completed: number;
		failed: number;
		total_savings_bytes: number;
		avg_processing_time_ms: number;
	};
}

export interface QueueStatus {
	pending: number;
	processing: number;
	completed: number;
	failed: number;
	total_savings_bytes: number;
	avg_processing_time_ms: number;
}

export interface UploadOptions {
	folder_id?: string;
	title?: string;
	alt_text?: string;
	tags?: string[];
	optimize?: boolean;
	generate_webp?: boolean;
	collection?: string;
	preset?: string;
	process_immediately?: boolean;
	priority?: number;
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
		// Use secure getter from auth store
		const token = authStore.getToken();
		const headers: Record<string, string> = {
			Accept: 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
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
		// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
		const apiEndpoint = endpoint.startsWith('/api/') ? endpoint : `/api${endpoint}`;
		let url = apiEndpoint;

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
		optimized?: boolean;
		needs_optimization?: boolean;
		processing_status?: 'pending' | 'processing' | 'completed' | 'failed';
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
		if (options?.generate_webp !== undefined)
			formData.append('generate_webp', String(options.generate_webp));

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
			// Use secure getter from auth store
			const token = authStore.getToken();

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

			xhr.open('POST', `/api/admin/media/upload`);
			if (token) {
				xhr.setRequestHeader('Authorization', `Bearer ${token}`);
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
		return this.request('DELETE', `/admin/media/folders/${id}`, undefined, {
			delete_files: deleteFiles
		});
	}

	// ═══════════════════════════════════════════════════════════════════════
	// AI Metadata
	// ═══════════════════════════════════════════════════════════════════════

	async generateAIMetadata(id: string): Promise<{ success: boolean; metadata: AIMetadata }> {
		return this.request('POST', `/admin/media/files/${id}/ai-metadata`);
	}

	async applyAIMetadata(
		id: string,
		fields: string[]
	): Promise<{ success: boolean; file: MediaFile }> {
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

	async bulkOptimize(
		ids: string[],
		options: { preset?: string; priority?: number } = {}
	): Promise<BulkActionResult> {
		return this.request('POST', '/admin/media/bulk/optimize', { ids, ...options });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Versioning
	// ═══════════════════════════════════════════════════════════════════════

	async getVersions(id: string): Promise<{ versions: MediaVersion[] }> {
		return this.request('GET', `/admin/media/files/${id}/versions`);
	}

	async createVersion(
		id: string,
		file: File,
		changes: string
	): Promise<{ success: boolean; version: MediaVersion }> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('changes', changes);

		return this.request('POST', `/admin/media/files/${id}/versions`, formData, undefined, true);
	}

	async restoreVersion(
		id: string,
		versionId: string
	): Promise<{ success: boolean; file: MediaFile }> {
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

	async getMostUsed(
		limit: number = 10
	): Promise<{ files: Array<MediaFile & { usage_count: number }> }> {
		return this.request('GET', '/admin/media/most-used', undefined, { limit });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Search
	// ═══════════════════════════════════════════════════════════════════════

	async search(
		query: string,
		filters?: {
			file_type?: string;
			folder_id?: string;
			tags?: string[];
			date_from?: string;
			date_to?: string;
		}
	): Promise<{ files: MediaFile[] }> {
		return this.request('GET', '/admin/media/search', undefined, { query, ...filters });
	}

	async findSimilar(id: string, limit: number = 10): Promise<{ files: MediaFile[] }> {
		return this.request('GET', `/admin/media/files/${id}/similar`, undefined, { limit });
	}

	async findDuplicates(): Promise<{ duplicates: Array<{ hash: string; files: MediaFile[] }> }> {
		return this.request('GET', '/admin/media/duplicates');
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Image Optimization (Imagify-like features)
	// ═══════════════════════════════════════════════════════════════════════

	/**
	 * Optimize a single image
	 */
	async optimizeImage(
		id: string,
		options: { preset?: string; immediate?: boolean; priority?: number } = {}
	): Promise<{ success: boolean; data: MediaFile | OptimizationJob; message: string }> {
		return this.request('POST', `/admin/media/files/${id}/optimize`, options);
	}

	/**
	 * Optimize all unoptimized images
	 */
	async optimizeAll(
		options: { limit?: number; preset?: string; priority?: number } = {}
	): Promise<{ success: boolean; total_pending: number; jobs_queued: number; message: string }> {
		return this.request('POST', '/admin/media/optimize-all', options);
	}

	/**
	 * Regenerate variants for a media item
	 */
	async regenerateVariants(
		id: string,
		preset?: string
	): Promise<{ success: boolean; data: MediaFile; message: string }> {
		return this.request('POST', `/admin/media/files/${id}/regenerate`, { preset });
	}

	/**
	 * Get optimization job status
	 */
	async getOptimizationJob(jobId: number): Promise<{ success: boolean; data: OptimizationJob }> {
		return this.request('GET', `/admin/media/jobs/${jobId}`);
	}

	/**
	 * Get optimization queue status
	 */
	async getQueueStatus(): Promise<{ success: boolean; data: QueueStatus }> {
		return this.request('GET', '/admin/media/queue/status');
	}

	/**
	 * Get optimization statistics
	 */
	async getOptimizationStatistics(): Promise<{ success: boolean; data: OptimizationStatistics }> {
		return this.request('GET', '/admin/media/statistics');
	}

	/**
	 * Get available optimization presets
	 */
	async getOptimizationPresets(): Promise<{ success: boolean; data: OptimizationPreset[] }> {
		return this.request('GET', '/admin/media/presets');
	}

	/**
	 * Get media variants
	 */
	async getVariants(id: string): Promise<{ success: boolean; data: MediaVariant[] }> {
		return this.request('GET', `/admin/media/files/${id}/variants`);
	}

	/**
	 * Upload method (alias for uploadFile for compatibility)
	 */
	async upload(
		file: File,
		options: UploadOptions = {}
	): Promise<{ success: boolean; data: MediaFile }> {
		const result = await this.uploadFile(file, options);
		return { success: result.success, data: result.file };
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Method Aliases (for cherry-picked component compatibility)
	// ═══════════════════════════════════════════════════════════════════════

	/**
	 * List media files (alias for getFiles)
	 */
	async list(
		params: {
			page?: number;
			per_page?: number;
			search?: string;
			type?: string;
			collection?: string;
			optimized?: boolean;
			needs_optimization?: boolean;
			processing_status?: 'pending' | 'processing' | 'completed' | 'failed';
			sort_by?: string;
			sort_dir?: 'asc' | 'desc';
		} = {}
	): Promise<{
		data: MediaFile[];
		meta: { current_page: number; last_page: number; total: number };
	}> {
		const result = await this.getFiles({
			...(params.page !== undefined && { page: params.page }),
			...(params.per_page !== undefined && { per_page: params.per_page }),
			...(params.search !== undefined && { search: params.search }),
			...(params.type !== undefined && { file_type: params.type }),
			...(params.optimized !== undefined && { optimized: params.optimized }),
			...(params.needs_optimization !== undefined && {
				needs_optimization: params.needs_optimization
			}),
			...(params.processing_status !== undefined && {
				processing_status: params.processing_status
			}),
			...(params.sort_by !== undefined && { sort: params.sort_by }),
			...(params.sort_dir !== undefined && { order: params.sort_dir })
		});
		return {
			data: result.files,
			meta: {
				current_page: result.pagination.page,
				last_page: result.pagination.total_pages,
				total: result.pagination.total
			}
		};
	}

	/**
	 * Delete media file (alias for deleteFile)
	 */
	async delete(id: string): Promise<{ success: boolean }> {
		return this.deleteFile(id);
	}

	/**
	 * Optimize single media file (alias for optimizeImage)
	 */
	async optimize(
		id: string,
		options: { preset?: string } = {}
	): Promise<{ success: boolean; data: MediaFile }> {
		const result = await this.optimizeImage(id, options);
		return { success: result.success, data: result.data as MediaFile };
	}

	/**
	 * Update media file (alias for updateFile)
	 */
	async update(
		id: string,
		data: Partial<
			Pick<MediaFile, 'title' | 'alt_text' | 'description' | 'tags' | 'caption' | 'collection'>
		>
	): Promise<{ success: boolean; data: MediaFile }> {
		const result = await this.updateFile(id, data);
		return { success: result.success, data: result.file };
	}

	/**
	 * Get statistics (alias for getOptimizationStatistics)
	 */
	async getStatistics(): Promise<{ success: boolean; data: OptimizationStatistics }> {
		return this.getOptimizationStatistics();
	}

	/**
	 * Get presets (alias for getOptimizationPresets)
	 */
	async getPresets(): Promise<{ success: boolean; data: OptimizationPreset[] }> {
		return this.getOptimizationPresets();
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Sharp High-Performance Image Service
	// ═══════════════════════════════════════════════════════════════════════

	/**
	 * Check Sharp service health
	 */
	async checkSharpHealth(): Promise<{
		sharp_service: { available: boolean; url: string };
		capabilities: {
			formats: string[];
			inputFormats: string[];
			outputFormats: string[];
			responsiveSizes: Record<string, number>;
			qualityPresets: string[];
		};
		storage: { disk: string; r2_configured: boolean };
	}> {
		return this.request('GET', '/sharp/health');
	}

	/**
	 * Upload and process image via Sharp service (10x faster)
	 */
	async sharpUpload(
		file: File,
		options: {
			collection?: string;
			preset?: 'maximum' | 'balanced' | 'performance' | 'thumbnail';
			alt_text?: string;
			title?: string;
			generate_webp?: boolean;
			generate_avif?: boolean;
			generate_responsive?: boolean;
			generate_thumbnail?: boolean;
			generate_blurhash?: boolean;
			generate_retina?: boolean;
		} = {},
		onProgress?: (progress: number) => void
	): Promise<{
		success: boolean;
		media: SharpMediaResult;
		processing_time_ms: number;
		variants_count: number;
		stats?: { originalSize: number; optimizedSize: number; savingsPercent: number };
	}> {
		const formData = new FormData();
		formData.append('image', file);

		if (options.collection) formData.append('collection', options.collection);
		if (options.preset) formData.append('preset', options.preset);
		if (options.alt_text) formData.append('alt_text', options.alt_text);
		if (options.title) formData.append('title', options.title);
		if (options.generate_webp !== undefined)
			formData.append('generate_webp', String(options.generate_webp));
		if (options.generate_avif !== undefined)
			formData.append('generate_avif', String(options.generate_avif));
		if (options.generate_responsive !== undefined)
			formData.append('generate_responsive', String(options.generate_responsive));
		if (options.generate_thumbnail !== undefined)
			formData.append('generate_thumbnail', String(options.generate_thumbnail));
		if (options.generate_blurhash !== undefined)
			formData.append('generate_blurhash', String(options.generate_blurhash));
		if (options.generate_retina !== undefined)
			formData.append('generate_retina', String(options.generate_retina));

		if (onProgress) {
			return this.sharpUploadWithProgress(formData, onProgress);
		}

		return this.request('POST', '/admin/sharp/upload', formData, undefined, true);
	}

	private sharpUploadWithProgress(
		formData: FormData,
		onProgress: (progress: number) => void
	): Promise<{
		success: boolean;
		media: SharpMediaResult;
		processing_time_ms: number;
		variants_count: number;
		stats?: { originalSize: number; optimizedSize: number; savingsPercent: number };
	}> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			const token = authStore.getToken();

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

			xhr.open('POST', `/api/admin/sharp/upload`);
			if (token) {
				xhr.setRequestHeader('Authorization', `Bearer ${token}`);
			}

			xhr.send(formData);
		});
	}

	/**
	 * Bulk upload via Sharp service
	 */
	async sharpBulkUpload(
		files: File[],
		options: {
			collection?: string;
			preset?: 'maximum' | 'balanced' | 'performance' | 'thumbnail';
		} = {}
	): Promise<{
		success: boolean;
		uploaded: Array<{ index: number; success: boolean; media: SharpMediaResult }>;
		errors: Array<{ index: number; filename: string; error: string }>;
		total: number;
		successful: number;
		failed: number;
	}> {
		const formData = new FormData();
		files.forEach((file) => formData.append('images[]', file));

		if (options.collection) formData.append('collection', options.collection);
		if (options.preset) formData.append('preset', options.preset);

		return this.request('POST', '/admin/sharp/bulk-upload', formData, undefined, true);
	}

	/**
	 * Process image from URL via Sharp
	 */
	async sharpProcessUrl(
		url: string,
		options: {
			collection?: string;
			preset?: 'maximum' | 'balanced' | 'performance' | 'thumbnail';
		} = {}
	): Promise<{ success: boolean; media: SharpMediaResult }> {
		return this.request('POST', '/admin/sharp/process-url', { url, ...options });
	}

	/**
	 * Optimize existing media via Sharp
	 */
	async sharpOptimize(
		id: string,
		preset: 'maximum' | 'balanced' | 'performance' | 'thumbnail' = 'balanced'
	): Promise<{
		success: boolean;
		media: SharpMediaResult;
		stats?: { originalSize: number; optimizedSize: number; savingsPercent: number };
	}> {
		return this.request('POST', `/admin/sharp/optimize/${id}`, { preset });
	}

	/**
	 * Get media with all Sharp variants
	 */
	async sharpGetMedia(id: string): Promise<{
		success: boolean;
		media: SharpMediaResult;
		variants: SharpVariant[];
		srcset: string;
		picture_sources: Array<{ type: string; srcset: string }>;
	}> {
		return this.request('GET', `/admin/sharp/media/${id}`);
	}

	/**
	 * Get Sharp statistics
	 */
	async getSharpStatistics(): Promise<{
		success: boolean;
		statistics: {
			total_media: number;
			total_images: number;
			optimized_images: number;
			optimization_rate: number;
			total_variants: number;
			total_storage_bytes: number;
			total_storage_human: string;
			total_bytes_saved: number;
			total_savings_human: string;
			sharp_service: { available: boolean; url: string };
		};
	}> {
		return this.request('GET', '/admin/sharp/statistics');
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Sharp-specific Types
// ═══════════════════════════════════════════════════════════════════════════

export interface SharpMediaResult {
	id: number;
	filename: string;
	url: string;
	cdn_url?: string;
	thumbnail_url?: string;
	width: number;
	height: number;
	size: number;
	size_human: string;
	mime_type: string;
	type: string;
	collection?: string;
	alt_text?: string;
	title?: string;
	is_optimized: boolean;
	blurhash?: string;
	lqip?: string;
	variants: Array<{
		type: string;
		sizeName: string;
		url: string;
		width: number;
		height: number;
		size: number;
	}>;
	created_at: string;
}

export interface SharpVariant {
	id: number;
	type: string;
	size_name: string;
	format: string;
	width: number;
	height: number;
	url: string;
	size: number;
	is_retina: boolean;
}

// Export singleton instance
export const mediaApi = new MediaApiClient();
export default mediaApi;

// Type aliases for compatibility with cherry-picked components
export type MediaItem = MediaFile;
export type MediaStatistics = OptimizationStatistics;
