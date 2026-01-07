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
 * @version 2.0.0 (Apple ICT 11+ Principal Engineer)
 * @author Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 */

import { build, files, version } from '$service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

// Cache name includes version to ensure fresh caches on new deployments
const CACHE_NAME = `cache-${version}`;

// Assets to cache (from SvelteKit build)
// Filter out any potentially problematic paths
const ASSETS = [
	...build, // Built app chunks
	...files  // Static files
].filter(asset => {
	// Skip assets that might cause issues
	if (!asset) return false;
	// Skip API routes (should never be cached)
	if (asset.startsWith('/api/')) return false;
	return true;
});

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
						console.warn(`[SW] Asset returned ${response.status}: ${asset}`);
						return false;
					}
				} catch (error) {
					// Log but don't throw - we want to continue caching other assets
					console.warn(`[SW] Failed to cache asset: ${asset}`, error);
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
	
	console.log(`[SW] Cached ${successCount}/${assets.length} assets (${failCount} failed)`);
}

// Install: Cache all assets with resilient individual caching
sw.addEventListener('install', (event) => {
	console.log(`[SW] Installing version ${version}`);
	console.log(`[SW] Preparing to cache ${ASSETS.length} assets`);
	
	event.waitUntil(
		caches.open(CACHE_NAME)
			.then(async (cache) => {
				// Use resilient individual caching instead of atomic addAll
				await cacheAssetsIndividually(cache, ASSETS);
			})
			.then(() => {
				console.log(`[SW] Installation complete, skipping waiting`);
				// Skip waiting to activate immediately
				return sw.skipWaiting();
			})
			.catch((error) => {
				// Even if caching fails, still try to activate
				console.error(`[SW] Installation error (continuing anyway):`, error);
				return sw.skipWaiting();
			})
	);
});

// Activate: Clean up old caches
sw.addEventListener('activate', (event) => {
	console.log(`[SW] Activating version ${version}`);
	
	event.waitUntil(
		caches.keys()
			.then((keys) => {
				return Promise.all(
					keys
						.filter((key) => key !== CACHE_NAME)
						.map((key) => {
							console.log(`[SW] Deleting old cache: ${key}`);
							return caches.delete(key);
						})
				);
			})
			.then(() => {
				// Take control of all clients immediately
				return sw.clients.claim();
			})
	);
});

// Fetch: Network-first with cache fallback for navigation
// Cache-first for static assets
sw.addEventListener('fetch', (event) => {
	const url = new URL(event.request.url);
	
	// Skip non-GET requests
	if (event.request.method !== 'GET') return;
	
	// Skip cross-origin requests
	if (url.origin !== location.origin) return;
	
	// Skip API requests - always go to network
	if (url.pathname.startsWith('/api')) return;
	
	// For navigation requests (HTML pages), use network-first
	if (event.request.mode === 'navigate') {
		event.respondWith(
			fetch(event.request)
				.catch(async () => {
					// If offline, try to serve cached page or offline fallback
					const cached = await caches.match(event.request);
					if (cached) return cached;
					const offline = await caches.match('/offline.html');
					return offline || new Response('Offline', { status: 503 });
				})
		);
		return;
	}
	
	// For static assets (JS, CSS, images), use cache-first
	if (ASSETS.includes(url.pathname)) {
		event.respondWith(
			caches.match(event.request)
				.then((cached) => {
					if (cached) return cached;
					
					// Not in cache, fetch from network
					return fetch(event.request)
						.then((response) => {
							// Don't cache error responses
							if (!response.ok) return response;
							
							// Cache the fresh response
							const responseClone = response.clone();
							caches.open(CACHE_NAME)
								.then((cache) => cache.put(event.request, responseClone));
							
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
					caches.open(CACHE_NAME)
						.then((cache) => cache.put(event.request, responseClone));
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

console.log(`[SW] Service worker v${version} loaded`);
