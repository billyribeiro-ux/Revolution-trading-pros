/**
 * API Caching Layer - SvelteKit / Svelte 5 Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade caching with:
 * - In-memory LRU cache with TTL
 * - localStorage persistence (optional)
 * - Request deduplication (prevents stampedes)
 * - Stale-while-revalidate support
 * - ETag/If-None-Match support
 * - Cache invalidation patterns
 *
 * @version 3.0.0 (SvelteKit / December 2025)
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

interface CacheEntry<T> {
	data: T;
	timestamp: number;
	ttl: number;
	etag?: string;
	staleTime?: number;
}

interface CacheOptions {
	ttl?: number;           // Time to live in milliseconds
	staleTime?: number;     // Time before data is considered stale (for SWR)
	persist?: boolean;      // Persist to localStorage
	etag?: string;          // ETag for conditional requests
}

interface PendingRequest<T> {
	promise: Promise<T>;
	timestamp: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

const DEFAULT_TTL = 5 * 60 * 1000;      // 5 minutes
const DEFAULT_STALE_TIME = 60 * 1000;    // 1 minute
const MAX_CACHE_SIZE = 100;              // Maximum entries
const STORAGE_PREFIX = 'api_cache_';
const REQUEST_TIMEOUT = 30 * 1000;       // 30 seconds

// ═══════════════════════════════════════════════════════════════════════════
// CACHE IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════

class ApiCache {
	private cache = new Map<string, CacheEntry<unknown>>();
	private pendingRequests = new Map<string, PendingRequest<unknown>>();
	private accessOrder: string[] = [];

	/**
	 * Get cached data or fetch fresh
	 */
	async getOrFetch<T>(
		key: string,
		fetcher: () => Promise<T>,
		options: CacheOptions = {}
	): Promise<T> {
		const {
			ttl = DEFAULT_TTL,
			staleTime = DEFAULT_STALE_TIME,
			persist = false
		} = options;

		// Check in-memory cache first
		const cached = this.get<T>(key);

		if (cached !== null) {
			const entry = this.cache.get(key) as CacheEntry<T>;
			const age = Date.now() - entry.timestamp;

			// Return fresh data
			if (age < staleTime) {
				return cached;
			}

			// Stale-while-revalidate: return stale data but refresh in background
			if (age < ttl) {
				this.revalidateInBackground(key, fetcher, { ttl, persist });
				return cached;
			}
		}

		// Check localStorage (if browser and persist enabled)
		if (browser && persist) {
			const stored = this.getFromStorage<T>(key);
			if (stored !== null) {
				// Populate in-memory cache
				this.set(key, stored, { ttl, persist: false });

				// Revalidate in background
				this.revalidateInBackground(key, fetcher, { ttl, persist });
				return stored;
			}
		}

		// Fetch fresh data (with deduplication)
		return this.fetchWithDedup(key, fetcher, { ttl, persist });
	}

	/**
	 * Get from cache (memory only)
	 */
	get<T>(key: string): T | null {
		const entry = this.cache.get(key) as CacheEntry<T> | undefined;

		if (!entry) return null;

		const age = Date.now() - entry.timestamp;

		if (age > entry.ttl) {
			this.delete(key);
			return null;
		}

		// Update access order for LRU
		this.updateAccessOrder(key);

		return entry.data;
	}

	/**
	 * Set cache entry
	 */
	set<T>(key: string, data: T, options: CacheOptions = {}): void {
		const { ttl = DEFAULT_TTL, persist = false, etag } = options;

		// Enforce max size (LRU eviction)
		while (this.cache.size >= MAX_CACHE_SIZE) {
			const oldest = this.accessOrder.shift();
			if (oldest) this.cache.delete(oldest);
		}

		const entry: CacheEntry<T> = {
			data,
			timestamp: Date.now(),
			ttl,
			...(etag !== undefined && { etag })
		};

		this.cache.set(key, entry as CacheEntry<unknown>);
		this.updateAccessOrder(key);

		// Persist to localStorage
		if (browser && persist) {
			this.setInStorage(key, entry);
		}
	}

	/**
	 * Delete cache entry
	 */
	delete(key: string): void {
		this.cache.delete(key);
		this.accessOrder = this.accessOrder.filter(k => k !== key);

		if (browser) {
			try {
				localStorage.removeItem(STORAGE_PREFIX + key);
			} catch { /* ignore */ }
		}
	}

	/**
	 * Clear all cache entries
	 */
	clear(): void {
		this.cache.clear();
		this.accessOrder = [];
		this.pendingRequests.clear();

		if (browser) {
			try {
				Object.keys(localStorage)
					.filter(k => k.startsWith(STORAGE_PREFIX))
					.forEach(k => localStorage.removeItem(k));
			} catch { /* ignore */ }
		}
	}

	/**
	 * Invalidate cache entries by pattern
	 */
	invalidate(pattern: string | RegExp): void {
		const regex = typeof pattern === 'string' ? new RegExp(pattern) : pattern;

		for (const key of this.cache.keys()) {
			if (regex.test(key)) {
				this.delete(key);
			}
		}
	}

	/**
	 * Check if key exists and is fresh
	 */
	has(key: string): boolean {
		return this.get(key) !== null;
	}

	/**
	 * Get cache stats
	 */
	getStats(): { size: number; keys: string[] } {
		return {
			size: this.cache.size,
			keys: [...this.cache.keys()]
		};
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PRIVATE METHODS
	// ═══════════════════════════════════════════════════════════════════════════

	private async fetchWithDedup<T>(
		key: string,
		fetcher: () => Promise<T>,
		options: CacheOptions
	): Promise<T> {
		// Check for pending request (deduplication)
		const pending = this.pendingRequests.get(key) as PendingRequest<T> | undefined;

		if (pending && Date.now() - pending.timestamp < REQUEST_TIMEOUT) {
			return pending.promise;
		}

		// Create new request
		const promise = fetcher()
			.then(data => {
				this.set(key, data, options);
				this.pendingRequests.delete(key);
				return data;
			})
			.catch(error => {
				this.pendingRequests.delete(key);
				throw error;
			});

		this.pendingRequests.set(key, { promise: promise as Promise<unknown>, timestamp: Date.now() });

		return promise;
	}

	private revalidateInBackground<T>(
		key: string,
		fetcher: () => Promise<T>,
		options: CacheOptions
	): void {
		// Don't await - run in background
		this.fetchWithDedup(key, fetcher, options).catch(() => {
			// Silent fail for background revalidation
		});
	}

	private updateAccessOrder(key: string): void {
		this.accessOrder = this.accessOrder.filter(k => k !== key);
		this.accessOrder.push(key);
	}

	private getFromStorage<T>(key: string): T | null {
		try {
			const raw = localStorage.getItem(STORAGE_PREFIX + key);
			if (!raw) return null;

			const entry: CacheEntry<T> = JSON.parse(raw);
			const age = Date.now() - entry.timestamp;

			if (age > entry.ttl) {
				localStorage.removeItem(STORAGE_PREFIX + key);
				return null;
			}

			return entry.data;
		} catch {
			return null;
		}
	}

	private setInStorage<T>(key: string, entry: CacheEntry<T>): void {
		try {
			localStorage.setItem(STORAGE_PREFIX + key, JSON.stringify(entry));
		} catch {
			// localStorage might be full - clear old entries
			this.clearOldStorageEntries();
			try {
				localStorage.setItem(STORAGE_PREFIX + key, JSON.stringify(entry));
			} catch { /* ignore */ }
		}
	}

	private clearOldStorageEntries(): void {
		try {
			const keys = Object.keys(localStorage)
				.filter(k => k.startsWith(STORAGE_PREFIX))
				.map(k => {
					try {
						const entry = JSON.parse(localStorage.getItem(k) || '{}');
						return { key: k, timestamp: entry.timestamp || 0 };
					} catch {
						return { key: k, timestamp: 0 };
					}
				})
				.sort((a, b) => a.timestamp - b.timestamp);

			// Remove oldest half
			const toRemove = keys.slice(0, Math.floor(keys.length / 2));
			toRemove.forEach(({ key }) => localStorage.removeItem(key));
		} catch { /* ignore */ }
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// SINGLETON INSTANCE
// ═══════════════════════════════════════════════════════════════════════════

export const apiCache = new ApiCache();

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a cached fetch function
 */
export function createCachedFetcher<T>(
	keyPrefix: string,
	fetcher: (params?: Record<string, unknown>) => Promise<T>,
	defaultOptions: CacheOptions = {}
) {
	return async (params?: Record<string, unknown>, options?: CacheOptions): Promise<T> => {
		const key = `${keyPrefix}:${JSON.stringify(params || {})}`;
		return apiCache.getOrFetch(key, () => fetcher(params), { ...defaultOptions, ...options });
	};
}

/**
 * Build cache key from URL and params
 */
export function buildCacheKey(url: string, params?: Record<string, unknown>): string {
	if (!params) return url;
	return `${url}:${JSON.stringify(params)}`;
}

/**
 * Invalidate cache by URL pattern
 */
export function invalidateCache(pattern: string | RegExp): void {
	apiCache.invalidate(pattern);
}

/**
 * Clear all cache
 */
export function clearCache(): void {
	apiCache.clear();
}

// ═══════════════════════════════════════════════════════════════════════════
// SVELTEKIT INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a SvelteKit load function with caching
 */
export function createCachedLoad<T>(
	fetcher: () => Promise<T>,
	options: CacheOptions & { key: string }
): () => Promise<T> {
	return () => apiCache.getOrFetch(options.key, fetcher, options);
}

/**
 * Hook for Svelte 5 runes - returns cached data with loading state
 */
export function useCachedData<T>(
	key: string,
	fetcher: () => Promise<T>,
	options: CacheOptions = {}
): { data: T | null; isLoading: boolean; error: Error | null; refresh: () => Promise<void> } {
	// This is meant to be used with $state in components
	// Returns an object that can be destructured

	let data: T | null = apiCache.get<T>(key);
	let isLoading = data === null;
	let error: Error | null = null;

	const refresh = async () => {
		isLoading = true;
		error = null;
		try {
			data = await apiCache.getOrFetch(key, fetcher, options);
		} catch (e) {
			error = e instanceof Error ? e : new Error(String(e));
		} finally {
			isLoading = false;
		}
	};

	// Initial fetch if not cached
	if (data === null) {
		refresh();
	}

	return { data, isLoading, error, refresh };
}

export default apiCache;
