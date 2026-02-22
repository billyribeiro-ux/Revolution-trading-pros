/**
 * Image Uploader - Cloudflare R2 CDN Upload System
 * =============================================================================
 * Enterprise-grade image upload with presigned URLs, retry logic, and progress tracking
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { API_BASE_URL, CDN_URL } from '$lib/api/config';
import { getAuthToken } from '$lib/stores/auth.svelte';
import { logger } from '$lib/utils/logger';

// =============================================================================
// Types
// =============================================================================

export interface UploadOptions {
	/** Collection/folder to upload to */
	collection?: string;
	/** Custom filename (optional, defaults to original) */
	filename?: string;
	/** Alt text for the image */
	alt?: string;
	/** Caption for the image */
	caption?: string;
	/** Progress callback (0-100) */
	onProgress?: (progress: number) => void;
	/** Upload started callback */
	onStart?: () => void;
	/** Upload complete callback */
	onComplete?: (result: UploadResult) => void;
	/** Error callback */
	onError?: (error: Error) => void;
	/** Maximum retry attempts */
	maxRetries?: number;
	/** Retry delay in ms */
	retryDelay?: number;
	/** Use presigned URL for direct R2 upload */
	usePresigned?: boolean;
	/** Abort signal for cancellation */
	signal?: AbortSignal;
}

export interface UploadResult {
	/** Unique asset ID from backend */
	id: number;
	/** Full CDN URL for the image */
	url: string;
	/** Original filename */
	filename: string;
	/** MIME type */
	mimeType: string;
	/** File size in bytes */
	size: number;
	/** Image width in pixels */
	width: number;
	/** Image height in pixels */
	height: number;
	/** Blurhash placeholder string */
	blurhash?: string;
	/** Alt text */
	alt?: string;
	/** Caption */
	caption?: string;
	/** Collection/folder */
	collection?: string;
	/** Creation timestamp */
	createdAt: string;
}

export interface PresignedUploadData {
	uploadUrl: string;
	assetId: number;
	key: string;
	expiresAt: string;
}

export interface UploadProgress {
	loaded: number;
	total: number;
	percentage: number;
}

// =============================================================================
// Upload Controller - Manages cancelable uploads
// =============================================================================

export class UploadController {
	private abortController: AbortController;
	private xhr: XMLHttpRequest | null = null;

	constructor() {
		this.abortController = new AbortController();
	}

	get signal(): AbortSignal {
		return this.abortController.signal;
	}

	abort(): void {
		this.abortController.abort();
		if (this.xhr) {
			this.xhr.abort();
			this.xhr = null;
		}
	}

	setXHR(xhr: XMLHttpRequest): void {
		this.xhr = xhr;
	}

	isAborted(): boolean {
		return this.abortController.signal.aborted;
	}
}

// =============================================================================
// Main Upload Function
// =============================================================================

/**
 * Upload an image to Cloudflare R2 CDN
 *
 * @param file - The file to upload
 * @param options - Upload configuration options
 * @returns Promise resolving to UploadResult
 *
 * @example
 * ```typescript
 * const result = await uploadImage(file, {
 *   collection: 'blog-images',
 *   onProgress: (progress) => logger.info(`${progress}%`),
 * });
 * logger.info(result.url); // CDN URL
 * ```
 */
export async function uploadImage(file: File, options: UploadOptions = {}): Promise<UploadResult> {
	const {
		collection = 'blog-images',
		filename,
		alt,
		caption,
		onProgress,
		onStart,
		onComplete,
		onError,
		maxRetries = 3,
		retryDelay = 1000,
		usePresigned = false,
		signal
	} = options;

	// Validate file type
	if (!file.type.startsWith('image/')) {
		const error = new Error('Invalid file type. Only images are allowed.');
		onError?.(error);
		throw error;
	}

	// Validate file size (max 10MB for images)
	const maxSize = 10 * 1024 * 1024;
	if (file.size > maxSize) {
		const error = new Error(`File too large. Maximum size is ${maxSize / 1024 / 1024}MB.`);
		onError?.(error);
		throw error;
	}

	onStart?.();

	let lastError: Error | null = null;

	for (let attempt = 0; attempt < maxRetries; attempt++) {
		try {
			// Check if aborted
			if (signal?.aborted) {
				throw new Error('Upload cancelled');
			}

			let result: UploadResult;

			if (usePresigned) {
				// Try presigned URL upload for direct R2 access
				result = await uploadWithPresignedUrl(file, {
					collection,
					filename,
					alt,
					caption,
					onProgress,
					signal
				});
			} else {
				// Standard multipart upload through backend
				result = await uploadMultipart(file, {
					collection,
					filename,
					alt,
					caption,
					onProgress,
					signal
				});
			}

			onComplete?.(result);
			return result;
		} catch (error) {
			lastError = error instanceof Error ? error : new Error(String(error));

			// Don't retry if cancelled
			if (signal?.aborted || lastError.message === 'Upload cancelled') {
				throw lastError;
			}

			// Don't retry on client errors (4xx)
			if (lastError.message.includes('4')) {
				throw lastError;
			}

			// Wait before retry with exponential backoff
			if (attempt < maxRetries - 1) {
				await sleep(retryDelay * Math.pow(2, attempt));
			}
		}
	}

	onError?.(lastError!);
	throw lastError;
}

// =============================================================================
// Multipart Upload (Standard)
// =============================================================================

async function uploadMultipart(
	file: File,
	options: {
		collection?: string;
		filename?: string;
		alt?: string;
		caption?: string;
		onProgress?: (progress: number) => void;
		signal?: AbortSignal;
	}
): Promise<UploadResult> {
	const { collection, filename, alt, caption, onProgress, signal } = options;

	return new Promise((resolve, reject) => {
		const xhr = new XMLHttpRequest();
		const formData = new FormData();

		// Append file with optional custom filename
		formData.append('file', file, filename || file.name);

		// Append metadata
		if (collection) formData.append('collection', collection);
		if (alt) formData.append('alt', alt);
		if (caption) formData.append('caption', caption);

		// Track upload progress
		xhr.upload.addEventListener('progress', (event) => {
			if (event.lengthComputable) {
				const percentage = Math.round((event.loaded / event.total) * 100);
				onProgress?.(percentage);
			}
		});

		// Handle completion
		xhr.addEventListener('load', () => {
			if (xhr.status >= 200 && xhr.status < 300) {
				try {
					const response = JSON.parse(xhr.responseText);
					const data = response.data || response;

					// Build CDN URL
					const cdnUrl = data.url?.startsWith('http')
						? data.url
						: `${CDN_URL}/${data.key || data.path || ''}`;

					const result: UploadResult = {
						id: data.id,
						url: cdnUrl,
						filename: data.filename || file.name,
						mimeType: data.mime_type || data.mimeType || file.type,
						size: data.size || file.size,
						width: data.width || 0,
						height: data.height || 0,
						blurhash: data.blurhash,
						alt: data.alt || alt,
						caption: data.caption || caption,
						collection: data.collection || collection,
						createdAt: data.created_at || data.createdAt || new Date().toISOString()
					};

					resolve(result);
				} catch (_parseError) {
					reject(new Error('Failed to parse upload response'));
				}
			} else {
				let errorMessage = `Upload failed with status ${xhr.status}`;
				try {
					const errorData = JSON.parse(xhr.responseText);
					errorMessage = errorData.message || errorData.error || errorMessage;
				} catch {
					// Use default error message
				}
				reject(new Error(errorMessage));
			}
		});

		// Handle errors
		xhr.addEventListener('error', () => {
			reject(new Error('Network error during upload'));
		});

		xhr.addEventListener('abort', () => {
			reject(new Error('Upload cancelled'));
		});

		xhr.addEventListener('timeout', () => {
			reject(new Error('Upload timed out'));
		});

		// Handle abort signal
		if (signal) {
			signal.addEventListener('abort', () => {
				xhr.abort();
			});
		}

		// Configure and send request
		xhr.open('POST', `${API_BASE_URL}/api/cms/assets/upload`);

		// Add auth token
		const token = getAuthToken();
		if (token) {
			xhr.setRequestHeader('Authorization', `Bearer ${token}`);
		}

		xhr.withCredentials = true;
		xhr.timeout = 300000; // 5 minute timeout for large files

		xhr.send(formData);
	});
}

// =============================================================================
// Presigned URL Upload (Direct to R2)
// =============================================================================

async function uploadWithPresignedUrl(
	file: File,
	options: {
		collection?: string;
		filename?: string;
		alt?: string;
		caption?: string;
		onProgress?: (progress: number) => void;
		signal?: AbortSignal;
	}
): Promise<UploadResult> {
	const { collection, filename, alt, caption, onProgress, signal } = options;

	// Step 1: Get presigned URL from backend
	const presignedData = await getPresignedUploadUrl(file, {
		collection,
		filename: filename || file.name,
		signal
	});

	// Step 2: Upload directly to R2
	await uploadToPresignedUrl(file, presignedData.uploadUrl, {
		onProgress,
		signal
	});

	// Step 3: Confirm upload with backend
	const result = await confirmUpload(presignedData.assetId, {
		alt,
		caption,
		signal
	});

	return result;
}

async function getPresignedUploadUrl(
	file: File,
	options: {
		collection?: string;
		filename?: string;
		signal?: AbortSignal;
	}
): Promise<PresignedUploadData> {
	const { collection, filename, signal } = options;

	const token = getAuthToken();
	const response = await fetch(`${API_BASE_URL}/api/cms/assets/presigned-upload`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		signal,
		body: JSON.stringify({
			filename: filename || file.name,
			contentType: file.type,
			size: file.size,
			collection
		})
	});

	if (!response.ok) {
		const errorData = await response.json().catch(() => ({}));
		throw new Error(errorData.message || 'Failed to get presigned URL');
	}

	const data = await response.json();
	return {
		uploadUrl: data.data.upload_url || data.data.uploadUrl,
		assetId: data.data.asset_id || data.data.assetId,
		key: data.data.key,
		expiresAt: data.data.expires_at || data.data.expiresAt
	};
}

async function uploadToPresignedUrl(
	file: File,
	uploadUrl: string,
	options: {
		onProgress?: (progress: number) => void;
		signal?: AbortSignal;
	}
): Promise<void> {
	const { onProgress, signal } = options;

	return new Promise((resolve, reject) => {
		const xhr = new XMLHttpRequest();

		xhr.upload.addEventListener('progress', (event) => {
			if (event.lengthComputable) {
				const percentage = Math.round((event.loaded / event.total) * 100);
				onProgress?.(percentage);
			}
		});

		xhr.addEventListener('load', () => {
			if (xhr.status >= 200 && xhr.status < 300) {
				resolve();
			} else {
				reject(new Error(`Direct upload failed with status ${xhr.status}`));
			}
		});

		xhr.addEventListener('error', () => {
			reject(new Error('Network error during direct upload'));
		});

		xhr.addEventListener('abort', () => {
			reject(new Error('Upload cancelled'));
		});

		if (signal) {
			signal.addEventListener('abort', () => {
				xhr.abort();
			});
		}

		xhr.open('PUT', uploadUrl);
		xhr.setRequestHeader('Content-Type', file.type);
		xhr.send(file);
	});
}

async function confirmUpload(
	assetId: number,
	options: {
		alt?: string;
		caption?: string;
		signal?: AbortSignal;
	}
): Promise<UploadResult> {
	const { alt, caption, signal } = options;

	const token = getAuthToken();
	const response = await fetch(`${API_BASE_URL}/api/cms/assets/confirm-upload`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			...(token ? { Authorization: `Bearer ${token}` } : {})
		},
		credentials: 'include',
		signal,
		body: JSON.stringify({
			asset_id: assetId,
			alt,
			caption
		})
	});

	if (!response.ok) {
		const errorData = await response.json().catch(() => ({}));
		throw new Error(errorData.message || 'Failed to confirm upload');
	}

	const data = await response.json();
	const assetData = data.data || data;

	return {
		id: assetData.id,
		url: assetData.url?.startsWith('http') ? assetData.url : `${CDN_URL}/${assetData.key}`,
		filename: assetData.filename,
		mimeType: assetData.mime_type || assetData.mimeType,
		size: assetData.size,
		width: assetData.width || 0,
		height: assetData.height || 0,
		blurhash: assetData.blurhash,
		alt: assetData.alt,
		caption: assetData.caption,
		collection: assetData.collection,
		createdAt: assetData.created_at || assetData.createdAt
	};
}

// =============================================================================
// Batch Upload
// =============================================================================

export interface BatchUploadResult {
	successful: UploadResult[];
	failed: { file: File; error: Error }[];
	total: number;
}

/**
 * Upload multiple images with progress tracking
 *
 * @param files - Array of files to upload
 * @param options - Upload options applied to all files
 * @param onBatchProgress - Callback for overall batch progress
 * @returns Promise resolving to BatchUploadResult
 */
export async function uploadImages(
	files: File[],
	options: UploadOptions = {},
	onBatchProgress?: (completed: number, total: number) => void
): Promise<BatchUploadResult> {
	const results: BatchUploadResult = {
		successful: [],
		failed: [],
		total: files.length
	};

	let completed = 0;

	// Process files sequentially to avoid overwhelming the server
	for (const file of files) {
		try {
			const result = await uploadImage(file, {
				...options,
				onProgress: undefined // Use batch progress instead
			});
			results.successful.push(result);
		} catch (error) {
			results.failed.push({
				file,
				error: error instanceof Error ? error : new Error(String(error))
			});
		}

		completed++;
		onBatchProgress?.(completed, files.length);
	}

	return results;
}

// =============================================================================
// Utility Functions
// =============================================================================

function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Check if a file is a valid image type
 */
export function isValidImageType(file: File): boolean {
	const validTypes = [
		'image/jpeg',
		'image/png',
		'image/webp',
		'image/gif',
		'image/svg+xml',
		'image/avif'
	];
	return validTypes.includes(file.type);
}

/**
 * Get human-readable file size
 */
export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

/**
 * Generate a unique filename with timestamp
 */
export function generateUniqueFilename(originalName: string): string {
	const ext = originalName.split('.').pop() || 'jpg';
	const baseName = originalName.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9-_]/g, '-');
	const timestamp = Date.now();
	const random = Math.random().toString(36).substring(2, 8);
	return `${baseName}-${timestamp}-${random}.${ext}`;
}

// =============================================================================
// Exports
// =============================================================================

export default uploadImage;
