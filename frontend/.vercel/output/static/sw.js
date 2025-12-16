/**
 * Service Worker - Revolution Trading Pros
 * ICT11+ Production-Grade Caching Strategy
 * 
 * @version 1.0.0
 */

const CACHE_NAME = 'rtp-cache-v1';
const STATIC_CACHE = 'rtp-static-v1';
const DYNAMIC_CACHE = 'rtp-dynamic-v1';

// Static assets to cache immediately
const STATIC_ASSETS = [
  '/',
  '/manifest.json',
  '/favicon.png'
];

// Install event - cache static assets
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(STATIC_CACHE)
      .then((cache) => cache.addAll(STATIC_ASSETS))
      .then(() => self.skipWaiting())
  );
});

// Activate event - clean old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => {
        return Promise.all(
          keys
            .filter((key) => key !== STATIC_CACHE && key !== DYNAMIC_CACHE)
            .map((key) => caches.delete(key))
        );
      })
      .then(() => self.clients.claim())
  );
});

// Fetch event - network-first strategy for API, cache-first for static
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') return;

  // Skip API requests and external resources
  if (url.pathname.startsWith('/api') || url.origin !== self.location.origin) {
    return;
  }

  // Skip Vite HMR and development files (prevents caching errors in dev mode)
  if (url.pathname.startsWith('/src/') || 
      url.pathname.startsWith('/@') || 
      url.pathname.startsWith('/node_modules/') ||
      url.pathname.includes('.svelte') ||
      url.pathname.includes('__vite') ||
      url.pathname.includes('?t=')) {
    return;
  }

  // Cache-first for static assets
  if (request.destination === 'image' || 
      request.destination === 'font' ||
      url.pathname.match(/\.(js|css|woff2?)$/)) {
    event.respondWith(
      caches.match(request)
        .then((cached) => {
          if (cached) return cached;
          return fetch(request).then((response) => {
            if (response.ok) {
              const clone = response.clone();
              caches.open(STATIC_CACHE).then((cache) => cache.put(request, clone));
            }
            return response;
          });
        })
    );
    return;
  }

  // Network-first for HTML pages
  event.respondWith(
    fetch(request)
      .then((response) => {
        if (response.ok) {
          const clone = response.clone();
          caches.open(DYNAMIC_CACHE).then((cache) => cache.put(request, clone));
        }
        return response;
      })
      .catch(() => caches.match(request))
  );
});
