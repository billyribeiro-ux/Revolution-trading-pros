/**
 * Image Upload Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Composable logic for image uploads with progress tracking and validation
 */

import { validateFile, sanitizeFilename, type FileValidationOptions } from '$lib/utils/sanitization';

// ============================================================================
// Types
// ============================================================================

export interface ImageUploadOptions {
	/** Maximum file size in bytes (default: 10MB) */
	maxSize?: number;
	/** Allowed MIME types (default: common image types) */
	allowedTypes?: string[];
	/** Allowed file extensions */
	allowedExtensions?: string[];
	/** Upload progress callback */
	onProgress?: (percent: number) => void;
	/** Success callback with uploaded image URL */
	onSuccess: (url: string, metadata?: ImageMetadata) => void;
	/** Error callback */
	onError?: (error: Error) => void;
	/** Upload endpoint URL */
	endpoint?: string;
	/** Additional headers for upload request */
	headers?: Record<string, string>;
	/** Whether to generate a thumbnail */
	generateThumbnail?: boolean;
	/** Maximum thumbnail dimension */
	thumbnailSize?: number;
	/** Whether to extract and preserve EXIF data */
	preserveExif?: boolean;
}

export interface ImageMetadata {
	width: number;
	height: number;
	size: number;
	type: string;
	name: string;
	thumbnailUrl?: string;
	blurhash?: string;
	exif?: Record<string, unknown>;
}

export interface UploadState {
	uploading: boolean;
	progress: number;
	error: string | null;
	url: string | null;
	metadata: ImageMetadata | null;
}

interface UploadResponse {
	success: boolean;
	url?: string;
	thumbnailUrl?: string;
	blurhash?: string;
	error?: string;
	metadata?: Partial<ImageMetadata>;
}

// ============================================================================
// Constants
// ============================================================================

const DEFAULT_MAX_SIZE = 10 * 1024 * 1024; // 10MB
const DEFAULT_ALLOWED_TYPES = [
	'image/jpeg',
	'image/png',
	'image/gif',
	'image/webp',
	'image/svg+xml',
	'image/avif'
];
const DEFAULT_ALLOWED_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'avif'];
const DEFAULT_ENDPOINT = '/api/media/upload';
const DEFAULT_THUMBNAIL_SIZE = 300;

// ============================================================================
// Hook Implementation
// ============================================================================

export function useImageUpload(options: ImageUploadOptions) {
	const {
		maxSize = DEFAULT_MAX_SIZE,
		allowedTypes = DEFAULT_ALLOWED_TYPES,
		allowedExtensions = DEFAULT_ALLOWED_EXTENSIONS,
		onProgress,
		onSuccess,
		onError,
		endpoint = DEFAULT_ENDPOINT,
		headers = {},
		generateThumbnail = true,
		thumbnailSize = DEFAULT_THUMBNAIL_SIZE,
		preserveExif = false
	} = options;

	// Reactive state
	let uploading = $state(false);
	let progress = $state(0);
	let error = $state<string | null>(null);
	let url = $state<string | null>(null);
	let metadata = $state<ImageMetadata | null>(null);

	// XHR reference for cancellation
	let currentXhr: XMLHttpRequest | null = null;

	// ========================================================================
	// Validation
	// ========================================================================

	function validateImage(file: File): { valid: boolean; error?: string } {
		const validationOptions: FileValidationOptions = {
			maxSize,
			allowedTypes,
			allowedExtensions
		};

		const result = validateFile(file, validationOptions);

		if (!result.valid) {
			return { valid: false, error: result.error };
		}

		return { valid: true };
	}

	// ========================================================================
	// Image Processing
	// ========================================================================

	async function getImageDimensions(file: File): Promise<{ width: number; height: number }> {
		return new Promise((resolve, reject) => {
			const img = new Image();
			const objectUrl = URL.createObjectURL(file);

			img.onload = () => {
				URL.revokeObjectURL(objectUrl);
				resolve({ width: img.naturalWidth, height: img.naturalHeight });
			};

			img.onerror = () => {
				URL.revokeObjectURL(objectUrl);
				reject(new Error('Failed to load image'));
			};

			img.src = objectUrl;
		});
	}

	async function createThumbnail(file: File): Promise<Blob | null> {
		return new Promise((resolve) => {
			const img = new Image();
			const objectUrl = URL.createObjectURL(file);

			img.onload = () => {
				URL.revokeObjectURL(objectUrl);

				// Calculate thumbnail dimensions
				let width = img.naturalWidth;
				let height = img.naturalHeight;

				if (width > height) {
					if (width > thumbnailSize) {
						height = Math.round((height * thumbnailSize) / width);
						width = thumbnailSize;
					}
				} else {
					if (height > thumbnailSize) {
						width = Math.round((width * thumbnailSize) / height);
						height = thumbnailSize;
					}
				}

				// Create canvas and draw
				const canvas = document.createElement('canvas');
				canvas.width = width;
				canvas.height = height;

				const ctx = canvas.getContext('2d');
				if (!ctx) {
					resolve(null);
					return;
				}

				ctx.drawImage(img, 0, 0, width, height);

				canvas.toBlob(
					(blob) => resolve(blob),
					'image/jpeg',
					0.8
				);
			};

			img.onerror = () => {
				URL.revokeObjectURL(objectUrl);
				resolve(null);
			};

			img.src = objectUrl;
		});
	}

	// ========================================================================
	// Upload Logic
	// ========================================================================

	async function upload(file: File): Promise<void> {
		// Reset state
		error = null;
		url = null;
		metadata = null;
		progress = 0;

		// Validate file
		const validation = validateImage(file);
		if (!validation.valid) {
			error = validation.error || 'Invalid file';
			onError?.(new Error(error));
			return;
		}

		uploading = true;

		try {
			// Get image dimensions
			const dimensions = await getImageDimensions(file);

			// Create form data
			const formData = new FormData();
			const sanitizedName = sanitizeFilename(file.name);
			formData.append('file', file, sanitizedName);
			formData.append('width', dimensions.width.toString());
			formData.append('height', dimensions.height.toString());

			// Generate thumbnail if requested
			if (generateThumbnail) {
				const thumbnail = await createThumbnail(file);
				if (thumbnail) {
					formData.append('thumbnail', thumbnail, `thumb_${sanitizedName}`);
				}
			}

			// Add options
			formData.append('generateThumbnail', generateThumbnail.toString());
			formData.append('preserveExif', preserveExif.toString());

			// Upload with XHR for progress tracking
			const response = await uploadWithProgress(formData);

			if (response.success && response.url) {
				// Build metadata
				const uploadedMetadata: ImageMetadata = {
					width: dimensions.width,
					height: dimensions.height,
					size: file.size,
					type: file.type,
					name: sanitizedName,
					thumbnailUrl: response.thumbnailUrl,
					blurhash: response.blurhash,
					...response.metadata
				};

				url = response.url;
				metadata = uploadedMetadata;
				progress = 100;

				onSuccess(response.url, uploadedMetadata);
			} else {
				throw new Error(response.error || 'Upload failed');
			}
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Upload failed';
			error = errorMessage;
			onError?.(err instanceof Error ? err : new Error(errorMessage));
		} finally {
			uploading = false;
			currentXhr = null;
		}
	}

	function uploadWithProgress(formData: FormData): Promise<UploadResponse> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			currentXhr = xhr;

			// Progress handler
			xhr.upload.addEventListener('progress', (event) => {
				if (event.lengthComputable) {
					const percent = Math.round((event.loaded / event.total) * 100);
					progress = percent;
					onProgress?.(percent);
				}
			});

			// Load handler
			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					try {
						const response = JSON.parse(xhr.responseText) as UploadResponse;
						resolve(response);
					} catch {
						reject(new Error('Invalid server response'));
					}
				} else {
					try {
						const errorResponse = JSON.parse(xhr.responseText);
						reject(new Error(errorResponse.error || `Upload failed: ${xhr.status}`));
					} catch {
						reject(new Error(`Upload failed: ${xhr.status}`));
					}
				}
			});

			// Error handler
			xhr.addEventListener('error', () => {
				reject(new Error('Network error during upload'));
			});

			// Abort handler
			xhr.addEventListener('abort', () => {
				reject(new Error('Upload cancelled'));
			});

			// Timeout handler
			xhr.addEventListener('timeout', () => {
				reject(new Error('Upload timed out'));
			});

			// Configure and send
			xhr.open('POST', endpoint);
			xhr.timeout = 120000; // 2 minutes

			// Set custom headers
			for (const [key, value] of Object.entries(headers)) {
				xhr.setRequestHeader(key, value);
			}

			xhr.send(formData);
		});
	}

	// ========================================================================
	// Multi-file Upload
	// ========================================================================

	async function uploadMultiple(
		files: File[],
		onFileComplete?: (index: number, url: string, meta: ImageMetadata) => void
	): Promise<Array<{ url: string; metadata: ImageMetadata }>> {
		const results: Array<{ url: string; metadata: ImageMetadata }> = [];

		for (let i = 0; i < files.length; i++) {
			const file = files[i];

			// Create a temporary promise to capture the result
			await new Promise<void>((resolve, reject) => {
				const originalOnSuccess = onSuccess;
				const originalOnError = onError;

				// Temporarily override callbacks
				options.onSuccess = (uploadedUrl, uploadedMeta) => {
					if (uploadedMeta) {
						results.push({ url: uploadedUrl, metadata: uploadedMeta });
						onFileComplete?.(i, uploadedUrl, uploadedMeta);
					}
					options.onSuccess = originalOnSuccess;
					options.onError = originalOnError;
					resolve();
				};

				options.onError = (err) => {
					options.onSuccess = originalOnSuccess;
					options.onError = originalOnError;
					reject(err);
				};

				upload(file);
			});
		}

		return results;
	}

	// ========================================================================
	// Control Methods
	// ========================================================================

	function cancel(): void {
		if (currentXhr) {
			currentXhr.abort();
			currentXhr = null;
		}
		uploading = false;
		progress = 0;
	}

	function reset(): void {
		cancel();
		error = null;
		url = null;
		metadata = null;
		progress = 0;
	}

	// ========================================================================
	// Drag and Drop Support
	// ========================================================================

	function handleDrop(event: DragEvent): void {
		event.preventDefault();
		event.stopPropagation();

		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			// Only upload the first image file
			const imageFile = Array.from(files).find((file) =>
				allowedTypes.includes(file.type)
			);

			if (imageFile) {
				upload(imageFile);
			} else {
				error = 'No valid image file found';
				onError?.(new Error(error));
			}
		}
	}

	function handleDragOver(event: DragEvent): void {
		event.preventDefault();
		event.stopPropagation();
	}

	// ========================================================================
	// File Input Support
	// ========================================================================

	function handleFileInput(event: Event): void {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];

		if (file) {
			upload(file);
		}

		// Reset input to allow re-selecting the same file
		input.value = '';
	}

	function openFilePicker(): void {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = allowedTypes.join(',');
		input.onchange = handleFileInput;
		input.click();
	}

	// ========================================================================
	// Paste Support
	// ========================================================================

	function handlePaste(event: ClipboardEvent): void {
		const items = event.clipboardData?.items;
		if (!items) return;

		for (const item of Array.from(items)) {
			if (item.type.startsWith('image/')) {
				const file = item.getAsFile();
				if (file) {
					event.preventDefault();
					upload(file);
					return;
				}
			}
		}
	}

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// Reactive state (via getters)
		get uploading() {
			return uploading;
		},
		get progress() {
			return progress;
		},
		get error() {
			return error;
		},
		get url() {
			return url;
		},
		get metadata() {
			return metadata;
		},

		// Methods
		upload,
		uploadMultiple,
		cancel,
		reset,
		validateImage,

		// Event handlers
		handleDrop,
		handleDragOver,
		handleFileInput,
		handlePaste,
		openFilePicker
	};
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Create a presigned URL upload handler
 */
export function createPresignedUploader(
	getPresignedUrl: (filename: string, contentType: string) => Promise<string>
) {
	return async function uploadToPresigned(file: File, onProgress?: (percent: number) => void): Promise<string> {
		const presignedUrl = await getPresignedUrl(file.name, file.type);

		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();

			xhr.upload.addEventListener('progress', (event) => {
				if (event.lengthComputable) {
					const percent = Math.round((event.loaded / event.total) * 100);
					onProgress?.(percent);
				}
			});

			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					// Extract the URL without query params (the actual object URL)
					const objectUrl = presignedUrl.split('?')[0];
					resolve(objectUrl);
				} else {
					reject(new Error(`Upload failed: ${xhr.status}`));
				}
			});

			xhr.addEventListener('error', () => reject(new Error('Network error')));
			xhr.addEventListener('abort', () => reject(new Error('Upload cancelled')));

			xhr.open('PUT', presignedUrl);
			xhr.setRequestHeader('Content-Type', file.type);
			xhr.send(file);
		});
	};
}

/**
 * Convert data URL to File
 */
export function dataUrlToFile(dataUrl: string, filename: string): File {
	const arr = dataUrl.split(',');
	const mime = arr[0].match(/:(.*?);/)?.[1] || 'image/png';
	const bstr = atob(arr[1]);
	let n = bstr.length;
	const u8arr = new Uint8Array(n);

	while (n--) {
		u8arr[n] = bstr.charCodeAt(n);
	}

	return new File([u8arr], filename, { type: mime });
}

/**
 * Compress image before upload
 */
export async function compressImage(
	file: File,
	maxWidth: number = 1920,
	maxHeight: number = 1080,
	quality: number = 0.85
): Promise<Blob> {
	return new Promise((resolve, reject) => {
		const img = new Image();
		const objectUrl = URL.createObjectURL(file);

		img.onload = () => {
			URL.revokeObjectURL(objectUrl);

			let { width, height } = img;

			// Calculate new dimensions
			if (width > maxWidth || height > maxHeight) {
				const ratio = Math.min(maxWidth / width, maxHeight / height);
				width = Math.round(width * ratio);
				height = Math.round(height * ratio);
			}

			// Create canvas
			const canvas = document.createElement('canvas');
			canvas.width = width;
			canvas.height = height;

			const ctx = canvas.getContext('2d');
			if (!ctx) {
				reject(new Error('Failed to get canvas context'));
				return;
			}

			ctx.drawImage(img, 0, 0, width, height);

			canvas.toBlob(
				(blob) => {
					if (blob) {
						resolve(blob);
					} else {
						reject(new Error('Failed to compress image'));
					}
				},
				file.type === 'image/png' ? 'image/png' : 'image/jpeg',
				quality
			);
		};

		img.onerror = () => {
			URL.revokeObjectURL(objectUrl);
			reject(new Error('Failed to load image'));
		};

		img.src = objectUrl;
	});
}
