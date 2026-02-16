/**
 * Video Preloader Service - Zero-Latency Video Loading
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Preloads video manifests and first segments on hover/viewport entry
 * for instant playback when user clicks play.
 *
 * Supports:
 * - Bunny.net HLS streams
 * - Intersection Observer for viewport preloading
 * - Hover-based aggressive preloading
 * - Connection-aware loading (respects Save-Data)
 *
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface PreloadOptions {
	/** Preload thumbnail image */
	thumbnail?: boolean;
	/** Preload HLS manifest */
	manifest?: boolean;
	/** Preload first N video segments */
	segments?: number;
	/** Priority level (high, low, auto) */
	priority?: 'high' | 'low' | 'auto';
}

export interface VideoPreloadInfo {
	videoId: string;
	libraryId: string;
	thumbnailUrl?: string;
	embedUrl?: string;
}

interface PreloadState {
	thumbnail: boolean;
	manifest: boolean;
	segments: number;
	timestamp: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// BUNNY.NET CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BUNNY_CDN_BASE = 'https://iframe.mediadelivery.net';
const BUNNY_THUMBNAIL_BASE = 'https://vz-'; // + libraryId + .b-cdn.net

// ═══════════════════════════════════════════════════════════════════════════
// PRELOADER CLASS
// ═══════════════════════════════════════════════════════════════════════════

class VideoPreloader {
	private preloadedVideos = new Map<string, PreloadState>();
	private observer: IntersectionObserver | null = null;
	private hoverTimeouts = new Map<string, number>();

	// Connection quality detection
	private saveData = false;
	private effectiveType: string = '4g';

	constructor() {
		if (browser) {
			this.detectConnectionQuality();
			this.setupIntersectionObserver();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════
	// CONNECTION QUALITY
	// ═══════════════════════════════════════════════════════════════════════

	private detectConnectionQuality(): void {
		const connection =
			(navigator as any).connection ||
			(navigator as any).mozConnection ||
			(navigator as any).webkitConnection;

		if (connection) {
			this.saveData = connection.saveData || false;
			this.effectiveType = connection.effectiveType || '4g';

			connection.addEventListener?.('change', () => {
				this.saveData = connection.saveData || false;
				this.effectiveType = connection.effectiveType || '4g';
			});
		}
	}

	private shouldPreload(): boolean {
		// Respect Save-Data header
		if (this.saveData) return false;

		// Only preload on good connections
		return ['4g', '3g'].includes(this.effectiveType);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// INTERSECTION OBSERVER (Viewport Preloading)
	// ═══════════════════════════════════════════════════════════════════════

	private setupIntersectionObserver(): void {
		if (!browser || typeof IntersectionObserver === 'undefined') return;

		this.observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						const element = entry.target as HTMLElement;
						const videoId = element.dataset.videoId;
						const libraryId = element.dataset.libraryId;
						const thumbnailUrl = element.dataset.thumbnailUrl;

						if (videoId && libraryId) {
							// Light preload when entering viewport
							this.preload(
								{ videoId, libraryId, thumbnailUrl },
								{ thumbnail: true, manifest: false, segments: 0 }
							);
						}
					}
				});
			},
			{
				rootMargin: '200px', // Start preloading 200px before visible
				threshold: 0
			}
		);
	}

	/**
	 * Register an element for viewport-based preloading
	 */
	observe(element: HTMLElement): void {
		this.observer?.observe(element);
	}

	/**
	 * Unregister an element from viewport-based preloading
	 */
	unobserve(element: HTMLElement): void {
		this.observer?.unobserve(element);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// HOVER PRELOADING (Aggressive)
	// ═══════════════════════════════════════════════════════════════════════

	/**
	 * Start preloading on hover (with debounce)
	 */
	onHoverStart(video: VideoPreloadInfo, delay: number = 100): void {
		if (!this.shouldPreload()) return;

		const key = this.getVideoKey(video);

		// Clear any existing timeout
		if (this.hoverTimeouts.has(key)) {
			clearTimeout(this.hoverTimeouts.get(key));
		}

		// Debounce to avoid preloading on quick mouse movements
		const timeout = window.setTimeout(() => {
			this.preload(video, {
				thumbnail: true,
				manifest: true,
				segments: 2 // Preload first 2 segments for instant playback
			});
			this.hoverTimeouts.delete(key);
		}, delay);

		this.hoverTimeouts.set(key, timeout);
	}

	/**
	 * Cancel pending preload on hover end
	 */
	onHoverEnd(video: VideoPreloadInfo): void {
		const key = this.getVideoKey(video);
		if (this.hoverTimeouts.has(key)) {
			clearTimeout(this.hoverTimeouts.get(key));
			this.hoverTimeouts.delete(key);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════
	// CORE PRELOADING
	// ═══════════════════════════════════════════════════════════════════════

	/**
	 * Preload video resources
	 */
	async preload(video: VideoPreloadInfo, options: PreloadOptions = {}): Promise<void> {
		if (!browser || !this.shouldPreload()) return;

		const key = this.getVideoKey(video);
		const state = this.preloadedVideos.get(key) || {
			thumbnail: false,
			manifest: false,
			segments: 0,
			timestamp: Date.now()
		};

		const promises: Promise<void>[] = [];

		// Preload thumbnail
		if (options.thumbnail && !state.thumbnail) {
			promises.push(this.preloadThumbnail(video));
			state.thumbnail = true;
		}

		// Preload HLS manifest
		if (options.manifest && !state.manifest) {
			promises.push(this.preloadManifest(video));
			state.manifest = true;
		}

		// Preload video segments
		if (options.segments && state.segments < options.segments) {
			promises.push(this.preloadSegments(video, options.segments - state.segments));
			state.segments = options.segments;
		}

		this.preloadedVideos.set(key, state);

		// Execute preloads in parallel
		await Promise.allSettled(promises);
	}

	/**
	 * Check if a video is preloaded
	 */
	isPreloaded(video: VideoPreloadInfo): boolean {
		const key = this.getVideoKey(video);
		return this.preloadedVideos.has(key);
	}

	/**
	 * Get preload state for a video
	 */
	getPreloadState(video: VideoPreloadInfo): PreloadState | null {
		const key = this.getVideoKey(video);
		return this.preloadedVideos.get(key) || null;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// RESOURCE PRELOADERS
	// ═══════════════════════════════════════════════════════════════════════

	private async preloadThumbnail(video: VideoPreloadInfo): Promise<void> {
		const thumbnailUrl =
			video.thumbnailUrl ||
			`${BUNNY_THUMBNAIL_BASE}${video.libraryId}.b-cdn.net/${video.videoId}/thumbnail.jpg`;

		return this.preloadResource(thumbnailUrl, 'image');
	}

	private async preloadManifest(video: VideoPreloadInfo): Promise<void> {
		// Bunny.net HLS manifest URL
		const manifestUrl = `${BUNNY_CDN_BASE}/play/${video.libraryId}/${video.videoId}`;

		try {
			const response = await fetch(manifestUrl, {
				method: 'GET',
				mode: 'cors',
				credentials: 'omit'
			});

			if (response.ok) {
				// Parse manifest to get segment URLs for further preloading
				const text = await response.text();
				this.parseAndCacheManifest(video, text);
			}
		} catch (error) {
			// Silent fail - preloading is best-effort
			logger.debug('[VideoPreloader] Manifest preload failed:', error);
		}
	}

	private async preloadSegments(video: VideoPreloadInfo, count: number): Promise<void> {
		// Get cached manifest segments
		const key = this.getVideoKey(video);
		const segments = this.manifestSegments.get(key);

		if (!segments || segments.length === 0) return;

		const segmentsToPreload = segments.slice(0, count);

		await Promise.allSettled(
			segmentsToPreload.map((segmentUrl) => this.preloadResource(segmentUrl, 'fetch'))
		);
	}

	private manifestSegments = new Map<string, string[]>();

	private parseAndCacheManifest(video: VideoPreloadInfo, manifestText: string): void {
		const key = this.getVideoKey(video);
		const segments: string[] = [];

		// Parse HLS manifest for segment URLs
		const lines = manifestText.split('\n');
		for (const line of lines) {
			// HLS segment files typically end with .ts or .m4s
			if (line.endsWith('.ts') || line.endsWith('.m4s')) {
				// Resolve relative URLs
				const segmentUrl = line.startsWith('http')
					? line
					: `${BUNNY_CDN_BASE}/play/${video.libraryId}/${video.videoId}/${line}`;
				segments.push(segmentUrl);
			}
		}

		this.manifestSegments.set(key, segments);
	}

	private async preloadResource(url: string, as: 'image' | 'fetch'): Promise<void> {
		return new Promise((resolve) => {
			if (as === 'image') {
				const img = new Image();
				img.onload = () => resolve();
				img.onerror = () => resolve();
				img.src = url;
			} else {
				// Use link preload for other resources
				const link = document.createElement('link');
				link.rel = 'preload';
				link.as = 'fetch';
				link.href = url;
				link.crossOrigin = 'anonymous';
				link.onload = () => resolve();
				link.onerror = () => resolve();
				document.head.appendChild(link);

				// Cleanup after 30 seconds
				setTimeout(() => {
					link.remove();
				}, 30000);
			}
		});
	}

	// ═══════════════════════════════════════════════════════════════════════
	// UTILITIES
	// ═══════════════════════════════════════════════════════════════════════

	private getVideoKey(video: VideoPreloadInfo): string {
		return `${video.libraryId}:${video.videoId}`;
	}

	/**
	 * Clear all preload state
	 */
	clear(): void {
		this.preloadedVideos.clear();
		this.manifestSegments.clear();
		this.hoverTimeouts.forEach((timeout) => clearTimeout(timeout));
		this.hoverTimeouts.clear();
	}

	/**
	 * Destroy the preloader
	 */
	destroy(): void {
		this.clear();
		this.observer?.disconnect();
		this.observer = null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// SINGLETON EXPORT
// ═══════════════════════════════════════════════════════════════════════════

export const videoPreloader = new VideoPreloader();

// ═══════════════════════════════════════════════════════════════════════════
// SVELTE ACTION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Svelte action for automatic video preloading
 *
 * Usage:
 * <div use:preloadVideo={{ videoId: '...', libraryId: '...' }}>
 */
export function preloadVideo(
	element: HTMLElement,
	video: VideoPreloadInfo
): { update: (v: VideoPreloadInfo) => void; destroy: () => void } {
	// Set data attributes for intersection observer
	element.dataset.videoId = video.videoId;
	element.dataset.libraryId = video.libraryId;
	if (video.thumbnailUrl) {
		element.dataset.thumbnailUrl = video.thumbnailUrl;
	}

	// Register for viewport preloading
	videoPreloader.observe(element);

	// Setup hover handlers
	const handleMouseEnter = () => videoPreloader.onHoverStart(video);
	const handleMouseLeave = () => videoPreloader.onHoverEnd(video);

	element.addEventListener('mouseenter', handleMouseEnter);
	element.addEventListener('mouseleave', handleMouseLeave);

	return {
		update(newVideo: VideoPreloadInfo) {
			element.dataset.videoId = newVideo.videoId;
			element.dataset.libraryId = newVideo.libraryId;
			if (newVideo.thumbnailUrl) {
				element.dataset.thumbnailUrl = newVideo.thumbnailUrl;
			}
		},
		destroy() {
			videoPreloader.unobserve(element);
			element.removeEventListener('mouseenter', handleMouseEnter);
			element.removeEventListener('mouseleave', handleMouseLeave);
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// PRECONNECT HINTS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Add preconnect hints for Bunny.net CDN
 * Call this early in page load
 */
export function addBunnyPreconnects(): void {
	if (!browser) return;

	const hints = [
		'https://iframe.mediadelivery.net',
		'https://video.bunnycdn.com',
		'https://vz-abc123.b-cdn.net' // Replace with actual library ID
	];

	hints.forEach((url) => {
		// Check if hint already exists
		const existing = document.querySelector(`link[href="${url}"][rel="preconnect"]`);
		if (existing) return;

		const link = document.createElement('link');
		link.rel = 'preconnect';
		link.href = url;
		link.crossOrigin = 'anonymous';
		document.head.appendChild(link);
	});
}
