import { logger } from '$lib/utils/logger';
/**
 * Image Processor - Client-side Image Optimization
 * =============================================================================
 * Resize, compress, convert to WebP, strip EXIF, and generate blurhash
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// =============================================================================
// Types
// =============================================================================

export interface ProcessOptions {
	/** Maximum width in pixels (default: 2000) */
	maxWidth?: number;
	/** Maximum height in pixels (optional) */
	maxHeight?: number;
	/** Quality for JPEG/WebP compression (0-1, default: 0.8) */
	quality?: number;
	/** Convert to WebP if browser supports (default: true) */
	convertToWebP?: boolean;
	/** Strip EXIF metadata for privacy (default: true) */
	stripExif?: boolean;
	/** Generate blurhash placeholder (default: true) */
	generateBlurhash?: boolean;
	/** Maintain aspect ratio (default: true) */
	maintainAspectRatio?: boolean;
	/** Output format override */
	outputFormat?: 'jpeg' | 'png' | 'webp' | 'original';
}

export interface ProcessedImage {
	/** Processed image as Blob */
	blob: Blob;
	/** Processed image as File (for upload) */
	file: File;
	/** Data URL for preview */
	dataUrl: string;
	/** Image width after processing */
	width: number;
	/** Image height after processing */
	height: number;
	/** Original width */
	originalWidth: number;
	/** Original height */
	originalHeight: number;
	/** File size after processing */
	size: number;
	/** Original file size */
	originalSize: number;
	/** Compression ratio (0-1) */
	compressionRatio: number;
	/** MIME type of processed image */
	mimeType: string;
	/** Blurhash string for placeholder */
	blurhash?: string;
	/** Whether image was resized */
	wasResized: boolean;
	/** Whether image was converted to different format */
	wasConverted: boolean;
}

export interface ImageDimensions {
	width: number;
	height: number;
}

// =============================================================================
// Blurhash Encoding (Simplified Implementation)
// =============================================================================

/**
 * Simplified blurhash-like placeholder generator
 * Generates a compact string representing dominant colors
 */
function generateSimpleBlurhash(
	imageData: ImageData,
	componentX: number = 4,
	componentY: number = 3
): string {
	const { data, width, height } = imageData;

	// Sample colors from a grid
	const colors: number[][] = [];
	const sampleWidth = Math.floor(width / componentX);
	const sampleHeight = Math.floor(height / componentY);

	for (let y = 0; y < componentY; y++) {
		for (let x = 0; x < componentX; x++) {
			// Sample center of each grid cell
			const sampleX = Math.floor((x + 0.5) * sampleWidth);
			const sampleY = Math.floor((y + 0.5) * sampleHeight);
			const idx = (sampleY * width + sampleX) * 4;

			colors.push([data[idx], data[idx + 1], data[idx + 2]]);
		}
	}

	// Calculate average color for base
	const avgColor = colors
		.reduce((acc, c) => [acc[0] + c[0], acc[1] + c[1], acc[2] + c[2]], [0, 0, 0])
		.map((v) => Math.round(v / colors.length));

	// Encode as base83-like string (simplified)
	const base83Chars =
		'0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~';

	let hash = '';

	// Encode size flag (componentX-1 + (componentY-1) * 9)
	const sizeFlag = componentX - 1 + (componentY - 1) * 9;
	hash += base83Chars[sizeFlag];

	// Encode DC value (average color)
	const dcValue = (avgColor[0] << 16) + (avgColor[1] << 8) + avgColor[2];
	for (let i = 3; i >= 0; i--) {
		hash += base83Chars[(dcValue >> (i * 6)) & 63];
	}

	// Encode AC values (color differences from average)
	for (const color of colors.slice(0, 9)) {
		const diff = [
			Math.min(63, Math.max(0, Math.round((color[0] - avgColor[0] + 128) / 4))),
			Math.min(63, Math.max(0, Math.round((color[1] - avgColor[1] + 128) / 4))),
			Math.min(63, Math.max(0, Math.round((color[2] - avgColor[2] + 128) / 4)))
		];
		hash += base83Chars[diff[0]] + base83Chars[diff[1]] + base83Chars[diff[2]];
	}

	return hash.substring(0, 32); // Limit length
}

// =============================================================================
// Main Processing Function
// =============================================================================

/**
 * Process an image file with optimization
 *
 * @param file - The image file to process
 * @param options - Processing configuration
 * @returns Promise resolving to ProcessedImage
 *
 * @example
 * ```typescript
 * const processed = await processImage(file, {
 *   maxWidth: 1920,
 *   quality: 0.85,
 *   convertToWebP: true,
 * });
 * logger.info(processed.blurhash); // Placeholder hash
 * logger.info(processed.file); // Optimized file for upload
 * ```
 */
export async function processImage(
	file: File,
	options: ProcessOptions = {}
): Promise<ProcessedImage> {
	const {
		maxWidth = 2000,
		maxHeight,
		quality = 0.8,
		convertToWebP = true,
		stripExif = true,
		generateBlurhash = true,
		maintainAspectRatio = true,
		outputFormat
	} = options;

	// Load image
	const img = await loadImage(file);
	const originalWidth = img.width;
	const originalHeight = img.height;

	// Calculate new dimensions
	const dimensions = calculateDimensions(
		originalWidth,
		originalHeight,
		maxWidth,
		maxHeight,
		maintainAspectRatio
	);

	const wasResized = dimensions.width !== originalWidth || dimensions.height !== originalHeight;

	// Create canvas
	const canvas = document.createElement('canvas');
	canvas.width = dimensions.width;
	canvas.height = dimensions.height;

	const ctx = canvas.getContext('2d', { willReadFrequently: true });
	if (!ctx) {
		throw new Error('Failed to get canvas context');
	}

	// Draw image (this strips EXIF by default in canvas)
	if (stripExif) {
		// Drawing to canvas naturally strips EXIF data
		ctx.drawImage(img, 0, 0, dimensions.width, dimensions.height);
	} else {
		// If we wanted to preserve orientation, we'd need to read EXIF first
		// For now, we always strip EXIF for privacy
		ctx.drawImage(img, 0, 0, dimensions.width, dimensions.height);
	}

	// Generate blurhash before conversion
	let blurhash: string | undefined;
	if (generateBlurhash) {
		try {
			// Create a small version for blurhash calculation
			const blurCanvas = document.createElement('canvas');
			const blurSize = 32;
			blurCanvas.width = blurSize;
			blurCanvas.height = Math.round((blurSize * dimensions.height) / dimensions.width);

			const blurCtx = blurCanvas.getContext('2d', { willReadFrequently: true });
			if (blurCtx) {
				blurCtx.drawImage(canvas, 0, 0, blurCanvas.width, blurCanvas.height);
				const imageData = blurCtx.getImageData(0, 0, blurCanvas.width, blurCanvas.height);
				blurhash = generateSimpleBlurhash(imageData);
			}
		} catch (e) {
			logger.warn('Failed to generate blurhash:', e);
		}
	}

	// Determine output format
	let mimeType: string;
	let wasConverted = false;

	if (outputFormat) {
		switch (outputFormat) {
			case 'webp':
				mimeType = 'image/webp';
				wasConverted = file.type !== 'image/webp';
				break;
			case 'jpeg':
				mimeType = 'image/jpeg';
				wasConverted = file.type !== 'image/jpeg';
				break;
			case 'png':
				mimeType = 'image/png';
				wasConverted = file.type !== 'image/png';
				break;
			default:
				mimeType = file.type;
		}
	} else if (convertToWebP && supportsWebP()) {
		// Convert to WebP if supported and not already WebP
		if (file.type !== 'image/webp' && file.type !== 'image/gif' && file.type !== 'image/svg+xml') {
			mimeType = 'image/webp';
			wasConverted = true;
		} else {
			mimeType = file.type;
		}
	} else {
		// Keep original format, but ensure it's a supported format
		if (file.type === 'image/jpeg' || file.type === 'image/png' || file.type === 'image/webp') {
			mimeType = file.type;
		} else {
			// Convert unsupported formats to JPEG
			mimeType = 'image/jpeg';
			wasConverted = true;
		}
	}

	// Convert canvas to blob
	const blob = await canvasToBlob(canvas, mimeType, quality);

	// Create data URL for preview
	const dataUrl = canvas.toDataURL(mimeType, quality);

	// Create File object
	const extension = mimeType.split('/')[1];
	const baseName = file.name.replace(/\.[^/.]+$/, '');
	const newFilename = `${baseName}.${extension}`;

	const processedFile = new File([blob], newFilename, {
		type: mimeType,
		lastModified: Date.now()
	});

	return {
		blob,
		file: processedFile,
		dataUrl,
		width: dimensions.width,
		height: dimensions.height,
		originalWidth,
		originalHeight,
		size: blob.size,
		originalSize: file.size,
		compressionRatio: blob.size / file.size,
		mimeType,
		blurhash,
		wasResized,
		wasConverted
	};
}

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Load an image file into an HTMLImageElement
 */
function loadImage(file: File): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const img = new Image();
		const url = URL.createObjectURL(file);

		img.onload = () => {
			URL.revokeObjectURL(url);
			resolve(img);
		};

		img.onerror = () => {
			URL.revokeObjectURL(url);
			reject(new Error('Failed to load image'));
		};

		img.src = url;
	});
}

/**
 * Calculate new dimensions while maintaining aspect ratio
 */
function calculateDimensions(
	originalWidth: number,
	originalHeight: number,
	maxWidth: number,
	maxHeight?: number,
	maintainAspectRatio: boolean = true
): ImageDimensions {
	let width = originalWidth;
	let height = originalHeight;

	if (!maintainAspectRatio) {
		return {
			width: Math.min(originalWidth, maxWidth),
			height: maxHeight ? Math.min(originalHeight, maxHeight) : originalHeight
		};
	}

	// Scale down to fit within maxWidth
	if (width > maxWidth) {
		const ratio = maxWidth / width;
		width = maxWidth;
		height = Math.round(height * ratio);
	}

	// Scale down to fit within maxHeight if specified
	if (maxHeight && height > maxHeight) {
		const ratio = maxHeight / height;
		height = maxHeight;
		width = Math.round(width * ratio);
	}

	return { width, height };
}

/**
 * Convert canvas to Blob with specified format and quality
 */
function canvasToBlob(canvas: HTMLCanvasElement, mimeType: string, quality: number): Promise<Blob> {
	return new Promise((resolve, reject) => {
		canvas.toBlob(
			(blob) => {
				if (blob) {
					resolve(blob);
				} else {
					reject(new Error('Failed to convert canvas to blob'));
				}
			},
			mimeType,
			quality
		);
	});
}

/**
 * Check if browser supports WebP encoding
 */
let webPSupported: boolean | null = null;

function supportsWebP(): boolean {
	if (webPSupported !== null) {
		return webPSupported;
	}

	try {
		const canvas = document.createElement('canvas');
		canvas.width = 1;
		canvas.height = 1;
		webPSupported = canvas.toDataURL('image/webp').startsWith('data:image/webp');
	} catch {
		webPSupported = false;
	}

	return webPSupported;
}

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Get image dimensions from a File
 */
export async function getImageDimensions(file: File): Promise<ImageDimensions> {
	const img = await loadImage(file);
	return {
		width: img.width,
		height: img.height
	};
}

/**
 * Create a thumbnail from an image file
 */
export async function createThumbnail(file: File, size: number = 200): Promise<ProcessedImage> {
	return processImage(file, {
		maxWidth: size,
		maxHeight: size,
		quality: 0.7,
		convertToWebP: true,
		generateBlurhash: false
	});
}

/**
 * Check if a file is a valid image that can be processed
 */
export function canProcessImage(file: File): boolean {
	const supportedTypes = [
		'image/jpeg',
		'image/png',
		'image/webp',
		'image/gif',
		'image/bmp',
		'image/tiff'
	];
	return supportedTypes.includes(file.type);
}

/**
 * Get optimal quality based on file type and size
 */
export function getOptimalQuality(file: File, targetSize?: number): number {
	const sizeInMB = file.size / (1024 * 1024);

	// If target size specified, estimate quality
	if (targetSize) {
		const ratio = targetSize / file.size;
		return Math.min(0.95, Math.max(0.5, ratio * 0.9));
	}

	// Otherwise use size-based heuristics
	if (sizeInMB > 5) return 0.7;
	if (sizeInMB > 2) return 0.75;
	if (sizeInMB > 1) return 0.8;
	return 0.85;
}

/**
 * Estimate compressed file size (rough approximation)
 */
export function estimateCompressedSize(
	originalSize: number,
	quality: number,
	convertToWebP: boolean
): number {
	// WebP typically achieves 25-34% smaller file sizes
	const webPFactor = convertToWebP ? 0.7 : 1;
	// Quality roughly correlates with file size
	const qualityFactor = 0.3 + quality * 0.7;

	return Math.round(originalSize * webPFactor * qualityFactor);
}

/**
 * Extract dominant color from image
 */
export async function extractDominantColor(file: File): Promise<string> {
	const img = await loadImage(file);

	const canvas = document.createElement('canvas');
	// Use small canvas for faster processing
	canvas.width = 10;
	canvas.height = 10;

	const ctx = canvas.getContext('2d', { willReadFrequently: true });
	if (!ctx) throw new Error('Failed to get canvas context');

	ctx.drawImage(img, 0, 0, 10, 10);
	const imageData = ctx.getImageData(0, 0, 10, 10);
	const data = imageData.data;

	// Calculate average color
	let r = 0,
		g = 0,
		b = 0;
	const pixelCount = data.length / 4;

	for (let i = 0; i < data.length; i += 4) {
		r += data[i];
		g += data[i + 1];
		b += data[i + 2];
	}

	r = Math.round(r / pixelCount);
	g = Math.round(g / pixelCount);
	b = Math.round(b / pixelCount);

	// Convert to hex
	return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
}

// =============================================================================
// Exports
// =============================================================================

export default processImage;
