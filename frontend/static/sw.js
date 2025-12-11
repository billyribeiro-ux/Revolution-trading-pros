/**
 * Service Worker - Revolution Trading Pros
 * ICT11+ Production-Grade Caching Strategy
 * Lightning Stack Edition - Enhanced Offline Support
 *
 * @version 2.0.0 - December 2024
 *
 * Caching Strategy:
 * - Static assets: Cache First (fonts, images, CSS, JS)
 * - API data: Network First with cache fallback
 * - HTML pages: Stale-While-Revalidate
 */

const CACHE_VERSION = 'v2';
const STATIC_CACHE = `rtp-static-${CACHE_VERSION}`;
const DYNAMIC_CACHE = `rtp-dynamic-${CACHE_VERSION}`;
const API_CACHE = `rtp-api-${CACHE_VERSION}`;

// Static assets to cache immediately
const STATIC_ASSETS = [
  '/',
  '/blog',
  '/manifest.json',
  '/favicon.png'
];

// API endpoints to cache for offline
const CACHED_API_PATTERNS = [
  '/api/posts',
  '/api/categories',
];

// Install event - cache static assets
self.addEventListener('install', (event) => {
  console.log('[SW] Installing service worker v2...');
  event.waitUntil(
    caches.open(STATIC_CACHE)
      .then((cache) => {
        return cache.addAll(STATIC_ASSETS).catch((error) => {
          console.warn('[SW] Some assets failed to cache:', error);
          return Promise.resolve();
        });
      })
      .then(() => self.skipWaiting())
  );
});

// Activate event - clean old caches
self.addEventListener('activate', (event) => {
  console.log('[SW] Activating service worker v2...');
  event.waitUntil(
    caches.keys()
      .then((keys) => {
        return Promise.all(
          keys
            .filter((key) =>
              key !== STATIC_CACHE &&
              key !== DYNAMIC_CACHE &&
              key !== API_CACHE
            )
            .map((key) => {
              console.log('[SW] Removing old cache:', key);
              return caches.delete(key);
            })
        );
      })
      .then(() => self.clients.claim())
  );
});

// Fetch event - smart caching based on request type
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') return;

  // Skip cross-origin requests (except CDN)
  if (url.origin !== self.location.origin &&
      !url.hostname.includes('cloudflare') &&
      !url.hostname.includes('r2.dev')) {
    return;
  }

  // Skip Vite HMR and development files
  if (url.pathname.startsWith('/src/') ||
      url.pathname.startsWith('/@') ||
      url.pathname.startsWith('/node_modules/') ||
      url.pathname.includes('.svelte') ||
      url.pathname.includes('__vite') ||
      url.pathname.includes('?t=')) {
    return;
  }

  // Handle API requests - Network First with cache fallback
  if (url.pathname.startsWith('/api/')) {
    // Only cache GET requests for specific API patterns
    const shouldCache = CACHED_API_PATTERNS.some(pattern =>
      url.pathname.startsWith(pattern)
    );

    if (shouldCache) {
      event.respondWith(networkFirstWithCache(request, API_CACHE));
    }
    return;
  }

  // Cache-first for static assets (images, fonts, CSS, JS)
  if (request.destination === 'image' ||
      request.destination === 'font' ||
      url.pathname.match(/\.(js|css|woff2?|ttf|otf|png|jpg|jpeg|webp|avif|svg|ico)$/)) {
    event.respondWith(cacheFirstWithNetwork(request, STATIC_CACHE));
    return;
  }

  // Stale-while-revalidate for HTML pages
  if (request.mode === 'navigate' ||
      request.headers.get('accept')?.includes('text/html')) {
    event.respondWith(staleWhileRevalidate(request, DYNAMIC_CACHE));
    return;
  }

  // Default: network first
  event.respondWith(networkFirstWithCache(request, DYNAMIC_CACHE));
});

/**
 * Cache First strategy - good for static assets
 */
async function cacheFirstWithNetwork(request, cacheName) {
  try {
    const cache = await caches.open(cacheName);
    const cached = await cache.match(request);

    if (cached) {
      // Update cache in background
      fetch(request).then((response) => {
        if (response.ok) {
          cache.put(request, response);
        }
      }).catch(() => {});
      return cached;
    }

    const response = await fetch(request);
    if (response.ok) {
      cache.put(request, response.clone());
    }
    return response;
  } catch (error) {
    const cached = await caches.match(request);
    if (cached) return cached;
    throw error;
  }
}

/**
 * Network First strategy - good for API data
 */
async function networkFirstWithCache(request, cacheName) {
  try {
    const response = await fetch(request);
    if (response.ok) {
      const cache = await caches.open(cacheName);
      cache.put(request, response.clone());
    }
    return response;
  } catch (error) {
    const cached = await caches.match(request);
    if (cached) {
      console.log('[SW] Serving from cache (offline):', request.url);
      return cached;
    }
    return new Response(
      JSON.stringify({ error: 'Offline', message: 'No cached data available' }),
      { status: 503, headers: { 'Content-Type': 'application/json' } }
    );
  }
}

/**
 * Stale-While-Revalidate strategy - good for HTML pages
 */
async function staleWhileRevalidate(request, cacheName) {
  const cache = await caches.open(cacheName);
  const cached = await cache.match(request);

  const fetchPromise = fetch(request)
    .then((response) => {
      if (response.ok) {
        cache.put(request, response.clone());
      }
      return response;
    })
    .catch(() => null);

  // Return cached immediately if available
  if (cached) {
    return cached;
  }

  // Wait for network
  const response = await fetchPromise;
  if (response) {
    return response;
  }

  // Return offline page
  return getOfflinePage();
}

/**
 * Generate offline fallback page
 */
function getOfflinePage() {
  return new Response(
    `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Offline - Revolution Trading Pros</title>
  <style>
    body {
      font-family: system-ui, -apple-system, sans-serif;
      background: linear-gradient(135deg, #0f172a, #1e293b);
      color: #f1f5f9;
      min-height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      margin: 0;
      padding: 1rem;
    }
    .container { text-align: center; max-width: 400px; }
    h1 { color: #60a5fa; margin-bottom: 1rem; }
    p { color: #94a3b8; line-height: 1.6; }
    button {
      background: #3b82f6;
      color: white;
      border: none;
      padding: 0.75rem 1.5rem;
      border-radius: 8px;
      font-size: 1rem;
      cursor: pointer;
      margin-top: 1rem;
    }
    button:hover { background: #2563eb; }
  </style>
</head>
<body>
  <div class="container">
    <h1>You're Offline</h1>
    <p>It looks like you've lost your internet connection. Some features may not be available until you're back online.</p>
    <button onclick="location.reload()">Try Again</button>
  </div>
</body>
</html>`,
    { status: 200, headers: { 'Content-Type': 'text/html' } }
  );
}

/**
 * Handle messages from main thread
 */
self.addEventListener('message', (event) => {
  if (event.data?.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }

  if (event.data?.type === 'CACHE_URLS') {
    const urls = event.data.urls || [];
    caches.open(DYNAMIC_CACHE).then((cache) => {
      urls.forEach((url) => {
        fetch(url).then((response) => {
          if (response.ok) cache.put(url, response);
        }).catch(() => {});
      });
    });
  }

  if (event.data?.type === 'CLEAR_CACHE') {
    caches.keys().then((keys) => {
      keys.forEach((key) => caches.delete(key));
    });
  }
});

console.log('[SW] Service worker v2 loaded');
