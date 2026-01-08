/**
 * Video Components - Zero-Latency Video System
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * Export all video-related components for easy imports.
 *
 * @version 1.0.0
 */

// Optimized Bunny.net video player with blurhash
export { default as BunnyVideoPlayer } from './BunnyVideoPlayer.svelte';

// Optimized video card with preloading
export { default as VideoCardOptimized } from './VideoCardOptimized.svelte';

// Re-export utilities for convenience
export {
	decodeBlurhash,
	encodeBlurhash,
	blurhash,
	blurhashBg,
	getCachedBlurhash,
	precomputeBlurhashes,
	DEFAULT_BLURHASHES
} from '$lib/utils/blurhash';

export {
	videoPreloader,
	preloadVideo,
	addBunnyPreconnects,
	type VideoPreloadInfo,
	type PreloadOptions
} from '$lib/utils/videoPreloader';
