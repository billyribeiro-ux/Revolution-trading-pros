/**
 * Blurhash Utilities - Zero-Latency Video Placeholders
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Provides instant visual feedback while videos/images load.
 * Renders in <1ms for perceived 0ms loading.
 *
 * @version 1.0.0
 */

import { decode, encode } from 'blurhash';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface BlurhashOptions {
	width?: number;
	height?: number;
	punch?: number; // 0-2, increases contrast
}

export interface BlurhashResult {
	hash: string;
	dataUrl: string;
	width: number;
	height: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// DECODE - Hash to Image (Client-side, <1ms)
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Decode a blurhash string to a data URL for instant display
 * Optimized for speed - uses small dimensions (32x32) for thumbnails
 */
export function decodeBlurhash(
	hash: string,
	options: BlurhashOptions = {}
): string | null {
	if (!hash || hash.length < 6) return null;

	const { width = 32, height = 18, punch = 1 } = options;

	try {
		const pixels = decode(hash, width, height, punch);
		return pixelsToDataURL(pixels, width, height);
	} catch (error) {
		console.warn('[Blurhash] Failed to decode:', error);
		return null;
	}
}

/**
 * Decode blurhash to canvas element (for Svelte action)
 */
export function decodeBlurhashToCanvas(
	canvas: HTMLCanvasElement,
	hash: string,
	options: BlurhashOptions = {}
): void {
	if (!hash || hash.length < 6) return;

	const { width = 32, height = 18, punch = 1 } = options;

	try {
		const pixels = decode(hash, width, height, punch);
		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		canvas.width = width;
		canvas.height = height;

		const imageData = ctx.createImageData(width, height);
		imageData.data.set(pixels);
		ctx.putImageData(imageData, 0, 0);
	} catch (error) {
		console.warn('[Blurhash] Failed to decode to canvas:', error);
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// ENCODE - Image to Hash (Server-side or Worker)
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Encode an image URL to blurhash (async, uses canvas)
 * Best run server-side or in a web worker
 */
export async function encodeBlurhash(
	imageUrl: string,
	componentX: number = 4,
	componentY: number = 3
): Promise<string | null> {
	try {
		const image = await loadImage(imageUrl);
		const canvas = document.createElement('canvas');
		const ctx = canvas.getContext('2d');
		if (!ctx) return null;

		// Use small dimensions for faster encoding
		const width = 64;
		const height = Math.round((image.height / image.width) * width);

		canvas.width = width;
		canvas.height = height;
		ctx.drawImage(image, 0, 0, width, height);

		const imageData = ctx.getImageData(0, 0, width, height);
		return encode(imageData.data, width, height, componentX, componentY);
	} catch (error) {
		console.warn('[Blurhash] Failed to encode:', error);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// SVELTE ACTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Svelte action for blurhash canvas
 * Usage: <canvas use:blurhash={hash} />
 */
export function blurhash(
	canvas: HTMLCanvasElement,
	hash: string
): { update: (newHash: string) => void; destroy: () => void } {
	decodeBlurhashToCanvas(canvas, hash);

	return {
		update(newHash: string) {
			decodeBlurhashToCanvas(canvas, newHash);
		},
		destroy() {
			// Cleanup if needed
		}
	};
}

/**
 * Svelte action for blurhash background on any element
 * Usage: <div use:blurhashBg={hash} />
 */
export function blurhashBg(
	element: HTMLElement,
	hash: string
): { update: (newHash: string) => void; destroy: () => void } {
	const dataUrl = decodeBlurhash(hash);
	if (dataUrl) {
		element.style.backgroundImage = `url(${dataUrl})`;
		element.style.backgroundSize = 'cover';
		element.style.backgroundPosition = 'center';
	}

	return {
		update(newHash: string) {
			const newDataUrl = decodeBlurhash(newHash);
			if (newDataUrl) {
				element.style.backgroundImage = `url(${newDataUrl})`;
			}
		},
		destroy() {
			element.style.backgroundImage = '';
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Convert decoded pixels to a data URL
 */
function pixelsToDataURL(
	pixels: Uint8ClampedArray,
	width: number,
	height: number
): string {
	const canvas = document.createElement('canvas');
	canvas.width = width;
	canvas.height = height;

	const ctx = canvas.getContext('2d');
	if (!ctx) return '';

	const imageData = ctx.createImageData(width, height);
	imageData.data.set(pixels);
	ctx.putImageData(imageData, 0, 0);

	return canvas.toDataURL();
}

/**
 * Load an image from URL
 */
function loadImage(url: string): Promise<HTMLImageElement> {
	return new Promise((resolve, reject) => {
		const img = new Image();
		img.crossOrigin = 'anonymous';
		img.onload = () => resolve(img);
		img.onerror = reject;
		img.src = url;
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// PRECOMPUTED BLURHASH CACHE
// ═══════════════════════════════════════════════════════════════════════════

const blurhashCache = new Map<string, string>();

/**
 * Get decoded blurhash with caching
 */
export function getCachedBlurhash(hash: string): string | null {
	if (!hash) return null;

	if (blurhashCache.has(hash)) {
		return blurhashCache.get(hash) || null;
	}

	const dataUrl = decodeBlurhash(hash);
	if (dataUrl) {
		blurhashCache.set(hash, dataUrl);
	}

	return dataUrl;
}

/**
 * Precompute and cache multiple blurhashes (e.g., on page load)
 */
export function precomputeBlurhashes(hashes: string[]): void {
	hashes.forEach((hash) => {
		if (hash && !blurhashCache.has(hash)) {
			const dataUrl = decodeBlurhash(hash);
			if (dataUrl) {
				blurhashCache.set(hash, dataUrl);
			}
		}
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// DEFAULT BLURHASHES (Fallbacks)
// ═══════════════════════════════════════════════════════════════════════════

export const DEFAULT_BLURHASHES = {
	// Dark gradient (for video placeholders)
	dark: 'L00000fQfQfQfQfQfQfQfQfQfQfQ',
	// Trading chart blue
	tradingBlue: 'L6PZfSi_.AyE_3t7t7WB~qofWBof',
	// Generic video thumbnail
	video: 'L5H2EC=PM+yV0g-mq.wG9c010J}@',
	// Light placeholder
	light: 'L2TSUA~qfQ~q?bj[fQj[~qj[WBfQ'
} as const;
