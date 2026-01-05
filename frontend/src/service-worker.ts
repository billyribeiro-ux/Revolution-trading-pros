/// <reference types="@sveltejs/kit" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

/**
 * Service Worker - Apple ICT 11+ Grade
 * ═══════════════════════════════════════════════════════════════════════════
 * Handles caching with proper versioning to prevent stale chunk 404 errors
 * Uses SvelteKit's $service-worker module for build manifest
 * ═══════════════════════════════════════════════════════════════════════════
 */

import { build, files, version } from '$service-worker';

const sw = self as unknown as ServiceWorkerGlobalScope;

// Cache name includes version to ensure fresh caches on new deployments
const CACHE_NAME = `cache-${version}`;

// Assets to cache (from SvelteKit build)
const ASSETS = [
	...build, // Built app chunks
	...files  // Static files
];

// Install: Cache all assets
sw.addEventListener('install', (event) => {
	console.log(`[SW] Installing version ${version}`);
	
	event.waitUntil(
		caches.open(CACHE_NAME)
			.then((cache) => {
				console.log(`[SW] Caching ${ASSETS.length} assets`);
				return cache.addAll(ASSETS);
			})
			.then(() => {
				// Skip waiting to activate immediately
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
				.catch(() => {
					// If offline, try to serve cached page or offline fallback
					return caches.match(event.request)
						.then((cached) => cached || caches.match('/offline.html'));
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
			.catch(() => caches.match(event.request))
	);
});

// Handle messages from clients
sw.addEventListener('message', (event) => {
	if (event.data?.type === 'SKIP_WAITING') {
		sw.skipWaiting();
	}
});

console.log(`[SW] Service worker v${version} loaded`);
