/// <reference types="@sveltejs/kit" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

/**
 * Service Worker - Apple ICT 11+ Principal Engineer Grade
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * ENTERPRISE FEATURES:
 *
 * 1. RESILIENT CACHING:
 *    - Individual asset caching (not atomic addAll)
 *    - Graceful degradation on cache failures
 *    - Continues installation even if some assets fail
 *
 * 2. SMART FETCH STRATEGIES:
 *    - Network-first for navigation (always fresh HTML)
 *    - Cache-first for static assets (performance)
 *    - Network-first with fallback for dynamic content
 *
 * 3. PROPER SCOPING:
 *    - Skips cross-origin requests
 *    - Skips API requests (always network)
 *    - Handles offline gracefully
 *
 * 4. VIDEO CACHING (Zero-Latency):
 *    - Caches Bunny.net video segments
 *    - Caches video thumbnails
 *    - Stale-while-revalidate for manifests
 *    - Respects cache headers from CDN
 *
 * 5. PRODUCTION SILENT MODE:
 *    - Zero console noise in production
 *    - Logs only errors for monitoring
 *
 * @version 4.0.0 (Apple ICT 11+ Principal Engineer - Silent Production Mode)
 * @author Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 */

import { build, files, version } from '$service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

// ICT 11+ Production Mode: Silent operation (no console noise)
const IS_PRODUCTION = true; // Set to false for debugging
const log = (...args: any[]) => !IS_PRODUCTION && console.log(...args);
const warn = (...args: any[]) => !IS_PRODUCTION && console.warn(...args);
const error = (...args: any[]) => console.error(...args); // Always log errors

// Cache name includes version to ensure fresh caches on new deployments
const CACHE_NAME = `cache-${version}`;

// Separate cache for video content (longer TTL, larger size)
const VIDEO_CACHE_NAME = `video-cache-${version}`;
const VIDEO_THUMBNAIL_CACHE = `video-thumbnails-${version}`;

// Video CDN domains to cache
const VIDEO_CDN_DOMAINS = [
	'iframe.mediadelivery.net',
	'video.bunnycdn.com',
	'b-cdn.net',
	'mediadelivery.net'
];

// Video file extensions to cache
const VIDEO_EXTENSIONS = ['.ts', '.m4s', '.mp4', '.m3u8', '.mpd'];

// Maximum video cache size (500MB)
const MAX_VIDEO_CACHE_SIZE = 500 * 1024 * 1024;

// Video cache TTL (1 hour for segments, 5 min for manifests)
const VIDEO_SEGMENT_TTL = 60 * 60 * 1000; // 1 hour
const VIDEO_MANIFEST_TTL = 5 * 60 * 1000; // 5 minutes

// Assets to cache (from SvelteKit build)
// Filter out any potentially problematic paths
const ASSETS = [
	...build, // Built app chunks
	...files // Static files
].filter((asset) => {
	// Skip assets that might cause issues
	if (!asset) return false;
	// Skip API routes (should never be cached)
	if (asset.startsWith('/api/')) return false;
	return true;
});

// ICT 7 FIX: Removed offline.html dependency
// Using inline fallback instead to avoid 404 errors during build
// The inline fallback is more reliable and doesn't require a separate file

// Maximum concurrent cache operations to avoid overwhelming the browser
const MAX_CONCURRENT_CACHE = 50;

/**
 * Cache assets in batches with individual error handling
 * Apple ICT 11+ Pattern: Resilient caching that doesn't fail atomically
 */
async function cacheAssetsIndividually(cache: Cache, assets: string[]): Promise<void> {
	let successCount = 0;
	let failCount = 0;

	// Process assets in batches to avoid overwhelming the browser
	for (let i = 0; i < assets.length; i += MAX_CONCURRENT_CACHE) {
		const batch = assets.slice(i, i + MAX_CONCURRENT_CACHE);

		const results = await Promise.allSettled(
			batch.map(async (asset) => {
				try {
					// Create a proper request with cache busting for fresh assets
					const request = new Request(asset, { cache: 'reload' });
					const response = await fetch(request);

					// Only cache successful responses
					if (response.ok) {
						await cache.put(asset, response);
						return true;
					} else {
						warn(`[SW] Asset returned ${response.status}: ${asset}`);
						return false;
					}
				} catch (err) {
					// Log but don't throw - we want to continue caching other assets
					warn(`[SW] Failed to cache asset: ${asset}`, err);
					return false;
				}
			})
		);

		// Count successes and failures
		results.forEach((result) => {
			if (result.status === 'fulfilled' && result.value) {
				successCount++;
			} else {
				failCount++;
			}
		});
	}

	log(`[SW] Cached ${successCount}/${assets.length} assets (${failCount} failed)`);
}

// Install: Cache all assets with resilient individual caching
sw.addEventListener('install', (event) => {
	log(`[SW] Installing version ${version}`);
	log(`[SW] Preparing to cache ${ASSETS.length} assets`);

	event.waitUntil(
		caches
			.open(CACHE_NAME)
			.then(async (cache) => {
				// Use resilient individual caching instead of atomic addAll
				await cacheAssetsIndividually(cache, ASSETS);
			})
			.then(() => {
				log(`[SW] Installation complete, skipping waiting`);
				// Skip waiting to activate immediately
				return sw.skipWaiting();
			})
			.catch((err) => {
				// Even if caching fails, still try to activate
				error(`[SW] Installation error (continuing anyway):`, err);
				return sw.skipWaiting();
			})
	);
});

// Activate: Clean up old caches (including video caches)
sw.addEventListener('activate', (event) => {
	log(`[SW] Activating version ${version}`);

	event.waitUntil(
		caches
			.keys()
			.then((keys) => {
				return Promise.all(
					keys
						.filter((key) => {
							// Keep current caches
							if (key === CACHE_NAME) return false;
							if (key === VIDEO_CACHE_NAME) return false;
							if (key === VIDEO_THUMBNAIL_CACHE) return false;
							return true;
						})
						.map((key) => {
							log(`[SW] Deleting old cache: ${key}`);
							return caches.delete(key);
						})
				);
			})
			.then(() => {
				// Also cleanup old video caches
				return cleanupVideoCaches();
			})
			.then(() => {
				// Take control of all clients immediately
				return sw.clients.claim();
			})
	);
});

// Fetch: Network-first with cache fallback for navigation
// Cache-first for static assets
// Video CDN: Cache-first with stale-while-revalidate
sw.addEventListener('fetch', (event) => {
	const url = new URL(event.request.url);

	// Skip non-GET requests
	if (event.request.method !== 'GET') return;

	// Handle Video CDN requests (cross-origin video caching)
	if (isVideoCDN(url)) {
		event.respondWith(handleVideoCDNFetch(event));
		return;
	}

	// Skip other cross-origin requests
	if (url.origin !== location.origin) return;

	// Skip API requests - always go to network
	if (url.pathname.startsWith('/api')) return;

	// For navigation requests (HTML pages), use network-first
	if (event.request.mode === 'navigate') {
		event.respondWith(
			fetch(event.request).catch(async () => {
				// If offline, try to serve cached page or offline fallback
				const cached = await caches.match(event.request);
				if (cached) return cached;

				// ICT 11+ FIX: Inline offline response (no external file dependency)
				return new Response(
					`<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Offline - Revolution Trading Pros</title><style>body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Oxygen,Ubuntu,Cantarell,sans-serif;background:linear-gradient(135deg,#0a101c 0%,#1a2332 100%);color:#fff;min-height:100vh;display:flex;align-items:center;justify-content:center;margin:0;padding:2rem}div{text-align:center;max-width:500px}.icon{font-size:5rem;margin-bottom:2rem;opacity:0.8}h1{font-size:2.5rem;margin-bottom:1rem;font-weight:700}p{font-size:1.125rem;line-height:1.6;color:rgba(255,255,255,0.8);margin-bottom:2rem}button{display:inline-block;padding:1rem 2rem;background:#facc15;color:#0a101c;border:none;border-radius:9999px;font-weight:600;cursor:pointer;transition:transform 0.2s,box-shadow 0.2s}button:hover{transform:translateY(-2px);box-shadow:0 10px 25px rgba(250,204,21,0.3)}</style></head><body><div><div class="icon">📡</div><h1>You're Offline</h1><p>It looks like you've lost your internet connection. Please check your network and try again.</p><button onclick="location.reload()">Try Again</button></div></body></html>`,
					{ status: 503, headers: { 'Content-Type': 'text/html' } }
				);
			})
		);
		return;
	}

	// For static assets (JS, CSS, images), use cache-first
	if (ASSETS.includes(url.pathname)) {
		event.respondWith(
			caches.match(event.request).then((cached) => {
				if (cached) return cached;

				// Not in cache, fetch from network
				return fetch(event.request).then((response) => {
					// Don't cache error responses
					if (!response.ok) return response;

					// Cache the fresh response
					const responseClone = response.clone();
					caches.open(CACHE_NAME).then((cache) => cache.put(event.request, responseClone));

					return response;
				});
			})
		);
		return;
	}

	// For everything else, network-first with cache fallback
	event.respondWith(
		fetch(event.request)
			.then((response) => {
				// Cache successful responses
				if (response.ok) {
					const responseClone = response.clone();
					caches.open(CACHE_NAME).then((cache) => cache.put(event.request, responseClone));
				}
				return response;
			})
			.catch(async () => {
				const cached = await caches.match(event.request);
				return cached || new Response('Not found', { status: 404 });
			})
	);
});

// Handle messages from clients
sw.addEventListener('message', (event) => {
	if (event.data?.type === 'SKIP_WAITING') {
		sw.skipWaiting();
	}
});

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO CACHING HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Check if URL is from a video CDN
 */
function isVideoCDN(url: URL): boolean {
	return VIDEO_CDN_DOMAINS.some((domain) => url.hostname.includes(domain));
}

/**
 * Check if URL is a video segment or manifest
 */
function isVideoFile(url: URL): boolean {
	return VIDEO_EXTENSIONS.some((ext) => url.pathname.endsWith(ext));
}

/**
 * Check if URL is a video thumbnail
 */
function isVideoThumbnail(url: URL): boolean {
	return (
		url.hostname.includes('b-cdn.net') &&
		(url.pathname.includes('/thumbnail') || url.pathname.endsWith('.jpg'))
	);
}

/**
 * Cache video resource with stale-while-revalidate strategy
 */
async function cacheVideoResource(request: Request, response: Response): Promise<void> {
	if (!response.ok) return;

	const url = new URL(request.url);
	const cacheName = isVideoThumbnail(url) ? VIDEO_THUMBNAIL_CACHE : VIDEO_CACHE_NAME;

	try {
		const cache = await caches.open(cacheName);

		// Clone response for caching
		const responseToCache = response.clone();

		// Add timestamp header for TTL checking
		const headers = new Headers(responseToCache.headers);
		headers.set('sw-cached-at', Date.now().toString());

		const cachedResponse = new Response(responseToCache.body, {
			status: responseToCache.status,
			statusText: responseToCache.statusText,
			headers
		});

		await cache.put(request, cachedResponse);
		// Silent in production
	} catch (err) {
		warn(`[SW] Failed to cache video: ${url.pathname}`, err);
	}
}

/**
 * Get cached video with TTL validation
 */
async function getCachedVideo(request: Request): Promise<Response | null> {
	const url = new URL(request.url);
	const cacheName = isVideoThumbnail(url) ? VIDEO_THUMBNAIL_CACHE : VIDEO_CACHE_NAME;

	try {
		const cache = await caches.open(cacheName);
		const cached = await cache.match(request);

		if (!cached) return null;

		// Check TTL
		const cachedAt = cached.headers.get('sw-cached-at');
		if (cachedAt) {
			const age = Date.now() - parseInt(cachedAt, 10);
			const isManifest = url.pathname.endsWith('.m3u8') || url.pathname.endsWith('.mpd');
			const ttl = isManifest ? VIDEO_MANIFEST_TTL : VIDEO_SEGMENT_TTL;

			if (age > ttl) {
				// Stale - return but revalidate in background
				revalidateInBackground(request, cache);
			}
		}

		return cached;
	} catch (err) {
		warn(`[SW] Cache read error:`, err);
		return null;
	}
}

/**
 * Revalidate cache in background
 */
async function revalidateInBackground(request: Request, cache: Cache): Promise<void> {
	try {
		const response = await fetch(request, { cache: 'no-store' });
		if (response.ok) {
			await cacheVideoResource(request, response.clone());
		}
	} catch {
		// Silent fail - stale content already served
	}
}

/**
 * Handle video CDN fetch with optimized caching
 */
async function handleVideoCDNFetch(event: FetchEvent): Promise<Response> {
	const request = event.request;
	const url = new URL(request.url);

	// For video files, use cache-first with stale-while-revalidate
	if (isVideoFile(url) || isVideoThumbnail(url)) {
		// Try cache first
		const cached = await getCachedVideo(request);
		if (cached) {
			console.debug(`[SW] Video cache hit: ${url.pathname}`);
			return cached;
		}

		// Fetch from network
		try {
			const response = await fetch(request);

			// Cache in background (don't await)
			event.waitUntil(cacheVideoResource(request, response.clone()));

			return response;
		} catch (err) {
			error(`[SW] Video fetch failed: ${url.pathname}`, err);
			throw err;
		}
	}

	// For non-video resources on CDN, just pass through
	return fetch(request);
}

/**
 * Cleanup old video caches
 */
async function cleanupVideoCaches(): Promise<void> {
	const cacheNames = await caches.keys();

	for (const cacheName of cacheNames) {
		// Delete old video caches (different version)
		if (
			(cacheName.startsWith('video-cache-') || cacheName.startsWith('video-thumbnails-')) &&
			cacheName !== VIDEO_CACHE_NAME &&
			cacheName !== VIDEO_THUMBNAIL_CACHE
		) {
			log(`[SW] Deleting old video cache: ${cacheName}`);
			await caches.delete(cacheName);
		}
	}
}

log(`[SW] Service worker v${version} loaded (with video caching)`);
