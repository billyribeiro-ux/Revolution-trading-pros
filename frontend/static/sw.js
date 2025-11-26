/**
 * Service Worker - Google L11+ Performance Optimization
 * ══════════════════════════════════════════════════════════════════════════════
 * Implements aggressive caching strategies for instant subsequent page loads
 * Cache-first for static assets, Network-first for dynamic content
 * ══════════════════════════════════════════════════════════════════════════════
 */

const CACHE_VERSION = 'v1';
const STATIC_CACHE = `static-${CACHE_VERSION}`;
const DYNAMIC_CACHE = `dynamic-${CACHE_VERSION}`;
const IMAGE_CACHE = `images-${CACHE_VERSION}`;

// Critical assets to cache immediately
const PRECACHE_URLS = [
	'/',
	'/offline.html'
];

// Cache duration in seconds
const CACHE_DURATION = {
	static: 7 * 24 * 60 * 60, // 7 days
	dynamic: 1 * 60 * 60, // 1 hour
	images: 30 * 24 * 60 * 60 // 30 days
};

// Install event - precache critical assets
self.addEventListener('install', (event) => {
	event.waitUntil(
		caches.open(STATIC_CACHE).then((cache) => {
			return cache.addAll(PRECACHE_URLS);
		}).then(() => {
			return self.skipWaiting(); // Activate immediately
		})
	);
});

// Activate event - cleanup old caches
self.addEventListener('activate', (event) => {
	event.waitUntil(
		caches.keys().then((cacheNames) => {
			return Promise.all(
				cacheNames
					.filter((name) => name.startsWith('static-') || name.startsWith('dynamic-') || name.startsWith('images-'))
					.filter((name) => name !== STATIC_CACHE && name !== DYNAMIC_CACHE && name !== IMAGE_CACHE)
					.map((name) => caches.delete(name))
			);
		}).then(() => {
			return self.clients.claim(); // Take control immediately
		})
	);
});

// Fetch event - intelligent caching strategy
self.addEventListener('fetch', (event) => {
	const { request } = event;
	const url = new URL(request.url);

	// Skip non-GET requests
	if (request.method !== 'GET') return;

	// Skip chrome extensions and non-http(s) requests
	if (!url.protocol.startsWith('http')) return;

	// Skip API calls - always fetch fresh
	if (url.pathname.startsWith('/api/')) {
		event.respondWith(fetch(request));
		return;
	}

	// Images - Cache first, network fallback
	if (request.destination === 'image') {
		event.respondWith(
			caches.match(request).then((cached) => {
				if (cached) return cached;
				
				return fetch(request).then((response) => {
					if (response.ok) {
						const clone = response.clone();
						caches.open(IMAGE_CACHE).then((cache) => {
							cache.put(request, clone);
						});
					}
					return response;
				});
			})
		);
		return;
	}

	// Static assets (JS, CSS, fonts) - Cache first, network fallback
	if (
		request.destination === 'script' ||
		request.destination === 'style' ||
		request.destination === 'font' ||
		url.pathname.match(/\.(js|css|woff2?|ttf|otf)$/)
	) {
		event.respondWith(
			caches.match(request).then((cached) => {
				if (cached) return cached;
				
				return fetch(request).then((response) => {
					if (response.ok) {
						const clone = response.clone();
						caches.open(STATIC_CACHE).then((cache) => {
							cache.put(request, clone);
						});
					}
					return response;
				}).catch(() => {
					// Return offline page if available
					return caches.match('/offline.html');
				});
			})
		);
		return;
	}

	// HTML pages - Network first, cache fallback (stale-while-revalidate)
	if (request.destination === 'document' || request.headers.get('accept')?.includes('text/html')) {
		event.respondWith(
			fetch(request).then((response) => {
				if (response.ok) {
					const clone = response.clone();
					caches.open(DYNAMIC_CACHE).then((cache) => {
						cache.put(request, clone);
					});
				}
				return response;
			}).catch(() => {
				// Fallback to cache
				return caches.match(request).then((cached) => {
					return cached || caches.match('/offline.html');
				});
			})
		);
		return;
	}

	// Default - network first
	event.respondWith(
		fetch(request).catch(() => {
			return caches.match(request);
		})
	);
});

// Background sync for offline actions (future enhancement)
self.addEventListener('sync', (event) => {
	if (event.tag === 'sync-data') {
		event.waitUntil(syncData());
	}
});

async function syncData() {
	// Implement background sync logic here
	console.log('Background sync triggered');
}

// Push notifications (future enhancement)
self.addEventListener('push', (event) => {
	const data = event.data?.json() ?? {};
	const title = data.title || 'Revolution Trading Pros';
	const options = {
		body: data.body || 'New update available',
		icon: '/icon-192.png',
		badge: '/badge-72.png',
		data: data.url
	};

	event.waitUntil(
		self.registration.showNotification(title, options)
	);
});

self.addEventListener('notificationclick', (event) => {
	event.notification.close();
	if (event.notification.data) {
		event.waitUntil(
			clients.openWindow(event.notification.data)
		);
	}
});
