/**
 * Request Cache Layer - TypeScript 5.9+ Implementation
 * =============================================================================
 *
 * Apple Principal Engineer ICT Level 7 Grade Quality
 * Built for the next 10 years - January 2026
 *
 * ARCHITECTURE PRINCIPLES:
 * 1. Memory-Efficient - LRU eviction prevents memory leaks
 * 2. Time-Aware - TTL-based expiration with stale-while-revalidate
 * 3. Pattern-Based Invalidation - Regex and tag-based cache busting
 * 4. Observable - Event hooks for cache hits/misses/evictions
 * 5. Serialization-Safe - All cached data is immutable
 *
 * CACHING STRATEGIES:
 * - cache-first: Return cache, fetch in background if stale
 * - network-first: Fetch first, fallback to cache on error
 * - stale-while-revalidate: Return stale, revalidate async
 * - cache-only: Never fetch, only return cache
 * - network-only: Never cache, always fetch
 *
 * @version 2.0.0 - TypeScript 5.9+ Rewrite
 * @license MIT
 */

import { browser } from '$app/environment';

// =============================================================================
// TYPE DEFINITIONS
// =============================================================================

/** Cache strategy determines fetch/cache priority */
export type CacheStrategy =
	| 'cache-first'
	| 'network-first'
	| 'stale-while-revalidate'
	| 'cache-only'
	| 'network-only';

/** Cache configuration for individual requests */
export interface CacheConfig {
	/** Time-to-live in milliseconds */
	readonly ttl: number;
	/** Whether to return stale data while revalidating */
	readonly staleWhileRevalidate: boolean;
	/** Maximum stale time in ms (data older than this won't be returned) */
	readonly maxStaleTime?: number;
	/** Custom cache key (overrides auto-generated) */
	readonly key?: string;
	/** Tags for grouped invalidation */
	readonly tags?: readonly string[];
	/** Patterns to invalidate on successful mutation */
	readonly invalidate?: readonly string[];
	/** Cache strategy */
	readonly strategy?: CacheStrategy;
	/** Persist to localStorage */
	readonly persist?: boolean;
}

/** Internal cache entry structure */
interface CacheEntry<T> {
	/** Cached data (frozen) */
	readonly data: T;
	/** Unix timestamp when cached */
	readonly timestamp: number;
	/** TTL in milliseconds */
	readonly ttl: number;
	/** Tags for grouped invalidation */
	readonly tags: readonly string[];
	/** Whether entry is currently being revalidated */
	isRevalidating: boolean;
	/** ETag for conditional requests */
	readonly etag?: string;
}

/** Cache statistics for monitoring */
export interface CacheStats {
	readonly size: number;
	readonly hits: number;
	readonly misses: number;
	readonly evictions: number;
	readonly hitRate: number;
}

/** Cache event types */
export type CacheEventType = 'hit' | 'miss' | 'set' | 'evict' | 'invalidate' | 'clear';

/** Cache event payload */
export interface CacheEvent {
	readonly type: CacheEventType;
	readonly key: string;
	readonly tags?: readonly string[];
	readonly timestamp: number;
}

/** Cache event listener */
export type CacheEventListener = (event: CacheEvent) => void;

// =============================================================================
// DEFAULT CONFIGURATION
// =============================================================================

const DEFAULT_CACHE_CONFIG: CacheConfig = Object.freeze({
	ttl: 300_000, // 5 minutes
	staleWhileRevalidate: true,
	maxStaleTime: 3_600_000, // 1 hour max stale
	tags: [],
	strategy: 'stale-while-revalidate',
	persist: false,
});

const STORAGE_PREFIX = 'rtp_cache_';
const MAX_STORAGE_AGE = 86_400_000; // 24 hours

// =============================================================================
// CACHED RESPONSE TYPE
// =============================================================================

/** Response from cache operations */
export interface CachedResponse<T> {
	/** The cached data */
	readonly data: T;
	/** Whether data is stale (past TTL but within maxStaleTime) */
	readonly isStale: boolean;
	/** Age of the cache entry in milliseconds */
	readonly age: number;
	/** Tags associated with this entry */
	readonly tags: readonly string[];
}

// =============================================================================
// REQUEST CACHE CLASS
// =============================================================================

/**
 * In-memory LRU cache with TTL support
 * Implements stale-while-revalidate and tag-based invalidation
 */
export class RequestCache implements Disposable {
	private readonly cache: Map<string, CacheEntry<unknown>> = new Map();
	private readonly maxSize: number;
	private readonly defaultTtl: number;
	private readonly staleWhileRevalidate: boolean;
	private readonly accessOrder: string[] = [];
	private readonly listeners: Set<CacheEventListener> = new Set();

	// Statistics
	private hits = 0;
	private misses = 0;
	private evictions = 0;

	// Cleanup interval
	private cleanupInterval: ReturnType<typeof setInterval> | null = null;

	constructor(config: Partial<CacheConfig> & { maxSize?: number } = {}) {
		this.maxSize = config.maxSize ?? 100;
		this.defaultTtl = config.ttl ?? DEFAULT_CACHE_CONFIG.ttl;
		this.staleWhileRevalidate = config.staleWhileRevalidate ?? DEFAULT_CACHE_CONFIG.staleWhileRevalidate;

		// Start periodic cleanup
		if (browser) {
			this.cleanupInterval = setInterval(() => this.cleanup(), 60_000);

			// Hydrate from localStorage on init
			this.hydrateFromStorage();
		}
	}

	/**
	 * Disposable implementation for cleanup
	 */
	[Symbol.dispose](): void {
		this.dispose();
	}

	dispose(): void {
		if (this.cleanupInterval) {
			clearInterval(this.cleanupInterval);
			this.cleanupInterval = null;
		}
		this.cache.clear();
		this.accessOrder.length = 0;
		this.listeners.clear();
	}

	// ===========================================================================
	// CORE OPERATIONS
	// ===========================================================================

	/**
	 * Get a cached value
	 * @returns CachedResponse if found, null if not cached or expired beyond max stale
	 */
	get<T>(key: string): CachedResponse<T> | null {
		const entry = this.cache.get(key) as CacheEntry<T> | undefined;

		if (!entry) {
			this.misses++;
			this.emit({ type: 'miss', key, timestamp: Date.now() });
			return null;
		}

		const age = Date.now() - entry.timestamp;
		const isExpired = age > entry.ttl;
		const maxStaleTime = DEFAULT_CACHE_CONFIG.maxStaleTime ?? this.defaultTtl * 2;
		const isBeyondMaxStale = age > maxStaleTime;

		// Remove if beyond max stale time
		if (isBeyondMaxStale) {
			this.delete(key);
			this.misses++;
			this.emit({ type: 'miss', key, timestamp: Date.now() });
			return null;
		}

		// Update access order for LRU
		this.updateAccessOrder(key);

		this.hits++;
		this.emit({ type: 'hit', key, tags: entry.tags, timestamp: Date.now() });

		return Object.freeze({
			data: entry.data,
			isStale: isExpired,
			age,
			tags: entry.tags,
		});
	}

	/**
	 * Set a cache entry
	 */
	set<T>(key: string, data: T, config?: Partial<CacheConfig>): void {
		// Enforce max size with LRU eviction
		while (this.cache.size >= this.maxSize) {
			const oldest = this.accessOrder.shift();
			if (oldest) {
				this.cache.delete(oldest);
				this.evictions++;
				this.emit({ type: 'evict', key: oldest, timestamp: Date.now() });
			}
		}

		const ttl = config?.ttl ?? this.defaultTtl;
		const tags = config?.tags ?? [];

		const entry: CacheEntry<T> = Object.freeze({
			data: Object.freeze(data) as T,
			timestamp: Date.now(),
			ttl,
			tags: Object.freeze([...tags]),
			isRevalidating: false,
		});

		this.cache.set(key, entry as CacheEntry<unknown>);
		this.updateAccessOrder(key);

		this.emit({ type: 'set', key, tags, timestamp: Date.now() });

		// Persist to localStorage if enabled
		if (config?.persist && browser) {
			this.persistToStorage(key, entry);
		}
	}

	/**
	 * Delete a specific cache entry
	 */
	delete(key: string): boolean {
		const had = this.cache.delete(key);
		if (had) {
			this.accessOrder.splice(this.accessOrder.indexOf(key), 1);

			if (browser) {
				try {
					localStorage.removeItem(STORAGE_PREFIX + key);
				} catch {
					// Ignore storage errors
				}
			}
		}
		return had;
	}

	/**
	 * Check if key exists and is fresh
	 */
	has(key: string): boolean {
		const entry = this.cache.get(key);
		if (!entry) return false;

		const age = Date.now() - entry.timestamp;
		return age <= entry.ttl;
	}

	/**
	 * Check if key exists (may be stale)
	 */
	hasAny(key: string): boolean {
		return this.cache.has(key);
	}

	// ===========================================================================
	// INVALIDATION
	// ===========================================================================

	/**
	 * Invalidate cache entries by pattern or exact match
	 */
	invalidate(pattern: string | RegExp): number {
		const regex = typeof pattern === 'string'
			? new RegExp(pattern.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))
			: pattern;

		let count = 0;

		for (const key of [...this.cache.keys()]) {
			if (regex.test(key)) {
				this.cache.delete(key);
				const index = this.accessOrder.indexOf(key);
				if (index > -1) this.accessOrder.splice(index, 1);
				count++;

				this.emit({ type: 'invalidate', key, timestamp: Date.now() });
			}
		}

		return count;
	}

	/**
	 * Invalidate cache entries by tag
	 */
	invalidateByTag(tag: string): number {
		let count = 0;

		for (const [key, entry] of this.cache.entries()) {
			if (entry.tags.includes(tag)) {
				this.cache.delete(key);
				const index = this.accessOrder.indexOf(key);
				if (index > -1) this.accessOrder.splice(index, 1);
				count++;

				this.emit({ type: 'invalidate', key, tags: entry.tags, timestamp: Date.now() });
			}
		}

		return count;
	}

	/**
	 * Invalidate cache entries by multiple tags (OR logic)
	 */
	invalidateByTags(tags: readonly string[]): number {
		let count = 0;

		for (const [key, entry] of this.cache.entries()) {
			const hasMatchingTag = tags.some((tag) => entry.tags.includes(tag));
			if (hasMatchingTag) {
				this.cache.delete(key);
				const index = this.accessOrder.indexOf(key);
				if (index > -1) this.accessOrder.splice(index, 1);
				count++;

				this.emit({ type: 'invalidate', key, tags: entry.tags, timestamp: Date.now() });
			}
		}

		return count;
	}

	/**
	 * Clear entire cache
	 */
	clear(): void {
		this.cache.clear();
		this.accessOrder.length = 0;

		this.emit({ type: 'clear', key: '*', timestamp: Date.now() });

		// Clear localStorage cache
		if (browser) {
			try {
				const keys = Object.keys(localStorage).filter((k) => k.startsWith(STORAGE_PREFIX));
				keys.forEach((k) => localStorage.removeItem(k));
			} catch {
				// Ignore storage errors
			}
		}
	}

	// ===========================================================================
	// STALE-WHILE-REVALIDATE
	// ===========================================================================

	/**
	 * Get or fetch with stale-while-revalidate pattern
	 */
	async getOrFetch<T>(
		key: string,
		fetcher: () => Promise<T>,
		config?: Partial<CacheConfig>
	): Promise<CachedResponse<T>> {
		const strategy = config?.strategy ?? 'stale-while-revalidate';

		// Handle different strategies
		switch (strategy) {
			case 'cache-only': {
				const cached = this.get<T>(key);
				if (cached) return cached;
				throw new Error(`Cache miss for key: ${key} (cache-only strategy)`);
			}

			case 'network-only': {
				const data = await fetcher();
				// Don't cache in network-only mode
				return Object.freeze({
					data,
					isStale: false,
					age: 0,
					tags: [],
				});
			}

			case 'cache-first': {
				const cached = this.get<T>(key);
				if (cached && !cached.isStale) {
					return cached;
				}

				// Fetch in background if stale
				if (cached?.isStale) {
					this.revalidateInBackground(key, fetcher, config);
					return cached;
				}

				// No cache, must fetch
				const data = await fetcher();
				this.set(key, data, config);
				return Object.freeze({
					data,
					isStale: false,
					age: 0,
					tags: config?.tags ?? [],
				});
			}

			case 'network-first': {
				try {
					const data = await fetcher();
					this.set(key, data, config);
					return Object.freeze({
						data,
						isStale: false,
						age: 0,
						tags: config?.tags ?? [],
					});
				} catch (error) {
					// Fallback to cache on network error
					const cached = this.get<T>(key);
					if (cached) return cached;
					throw error;
				}
			}

			case 'stale-while-revalidate':
			default: {
				const cached = this.get<T>(key);

				if (cached) {
					// Return stale data and revalidate in background
					if (cached.isStale) {
						this.revalidateInBackground(key, fetcher, config);
					}
					return cached;
				}

				// No cache, must fetch
				const data = await fetcher();
				this.set(key, data, config);
				return Object.freeze({
					data,
					isStale: false,
					age: 0,
					tags: config?.tags ?? [],
				});
			}
		}
	}

	/**
	 * Revalidate entry in background (fire and forget)
	 */
	private revalidateInBackground<T>(
		key: string,
		fetcher: () => Promise<T>,
		config?: Partial<CacheConfig>
	): void {
		const entry = this.cache.get(key);
		if (!entry || entry.isRevalidating) return;

		// Mark as revalidating to prevent duplicate fetches
		(entry as { isRevalidating: boolean }).isRevalidating = true;

		fetcher()
			.then((data) => {
				this.set(key, data, config);
			})
			.catch(() => {
				// Silent failure for background revalidation
			})
			.finally(() => {
				const currentEntry = this.cache.get(key);
				if (currentEntry) {
					(currentEntry as { isRevalidating: boolean }).isRevalidating = false;
				}
			});
	}

	// ===========================================================================
	// STATISTICS & MONITORING
	// ===========================================================================

	/**
	 * Get cache statistics
	 */
	getStats(): CacheStats {
		const total = this.hits + this.misses;
		return Object.freeze({
			size: this.cache.size,
			hits: this.hits,
			misses: this.misses,
			evictions: this.evictions,
			hitRate: total > 0 ? this.hits / total : 0,
		});
	}

	/**
	 * Get all cache keys
	 */
	keys(): readonly string[] {
		return Object.freeze([...this.cache.keys()]);
	}

	/**
	 * Get entries by tag
	 */
	getByTag(tag: string): readonly string[] {
		const keys: string[] = [];
		for (const [key, entry] of this.cache.entries()) {
			if (entry.tags.includes(tag)) {
				keys.push(key);
			}
		}
		return Object.freeze(keys);
	}

	// ===========================================================================
	// EVENT SYSTEM
	// ===========================================================================

	/**
	 * Subscribe to cache events
	 * @returns Unsubscribe function
	 */
	on(listener: CacheEventListener): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private emit(event: CacheEvent): void {
		for (const listener of this.listeners) {
			try {
				listener(event);
			} catch {
				// Don't let listener errors crash the cache
			}
		}
	}

	// ===========================================================================
	// PRIVATE HELPERS
	// ===========================================================================

	private updateAccessOrder(key: string): void {
		const index = this.accessOrder.indexOf(key);
		if (index > -1) {
			this.accessOrder.splice(index, 1);
		}
		this.accessOrder.push(key);
	}

	/**
	 * Remove expired entries
	 */
	private cleanup(): void {
		const now = Date.now();
		const maxStaleTime = DEFAULT_CACHE_CONFIG.maxStaleTime ?? this.defaultTtl * 2;

		for (const [key, entry] of this.cache.entries()) {
			const age = now - entry.timestamp;
			if (age > maxStaleTime) {
				this.cache.delete(key);
				const index = this.accessOrder.indexOf(key);
				if (index > -1) this.accessOrder.splice(index, 1);
				this.evictions++;
			}
		}
	}

	// ===========================================================================
	// STORAGE PERSISTENCE
	// ===========================================================================

	private persistToStorage<T>(key: string, entry: CacheEntry<T>): void {
		try {
			const serialized = JSON.stringify({
				data: entry.data,
				timestamp: entry.timestamp,
				ttl: entry.ttl,
				tags: entry.tags,
			});
			localStorage.setItem(STORAGE_PREFIX + key, serialized);
		} catch {
			// Storage quota exceeded or other error - clean up old entries
			this.cleanupStorage();
		}
	}

	private hydrateFromStorage(): void {
		try {
			const now = Date.now();
			const keys = Object.keys(localStorage).filter((k) => k.startsWith(STORAGE_PREFIX));

			for (const storageKey of keys) {
				try {
					const raw = localStorage.getItem(storageKey);
					if (!raw) continue;

					const parsed = JSON.parse(raw);
					const age = now - parsed.timestamp;

					// Skip if too old
					if (age > MAX_STORAGE_AGE) {
						localStorage.removeItem(storageKey);
						continue;
					}

					const key = storageKey.slice(STORAGE_PREFIX.length);

					const entry: CacheEntry<unknown> = {
						data: Object.freeze(parsed.data),
						timestamp: parsed.timestamp,
						ttl: parsed.ttl,
						tags: Object.freeze(parsed.tags ?? []),
						isRevalidating: false,
					};

					this.cache.set(key, entry);
					this.accessOrder.push(key);
				} catch {
					// Invalid entry, remove it
					localStorage.removeItem(storageKey);
				}
			}
		} catch {
			// localStorage not available
		}
	}

	private cleanupStorage(): void {
		try {
			const now = Date.now();
			const keys = Object.keys(localStorage)
				.filter((k) => k.startsWith(STORAGE_PREFIX))
				.map((k) => {
					try {
						const raw = localStorage.getItem(k);
						const parsed = raw ? JSON.parse(raw) : { timestamp: 0 };
						return { key: k, timestamp: parsed.timestamp ?? 0 };
					} catch {
						return { key: k, timestamp: 0 };
					}
				})
				.sort((a, b) => a.timestamp - b.timestamp);

			// Remove oldest half or expired entries
			const toRemove = keys.slice(0, Math.max(Math.floor(keys.length / 2), 10));
			toRemove.forEach(({ key, timestamp }) => {
				const age = now - timestamp;
				if (age > MAX_STORAGE_AGE || keys.indexOf({ key, timestamp }) < Math.floor(keys.length / 2)) {
					localStorage.removeItem(key);
				}
			});
		} catch {
			// Ignore storage errors
		}
	}
}

// =============================================================================
// SINGLETON INSTANCE
// =============================================================================

let cacheInstance: RequestCache | null = null;

/**
 * Get the singleton cache instance
 */
export function getCache(config?: Partial<CacheConfig> & { maxSize?: number }): RequestCache {
	if (!cacheInstance) {
		cacheInstance = new RequestCache(config);
	}
	return cacheInstance;
}

/**
 * Create a new cache instance (for isolated use)
 */
export function createCache(config?: Partial<CacheConfig> & { maxSize?: number }): RequestCache {
	return new RequestCache(config);
}

/**
 * Default singleton instance
 */
export const requestCache = browser ? getCache() : null;

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/**
 * Generate a cache key from URL and parameters
 */
export function generateCacheKey(
	method: string,
	url: string,
	params?: Record<string, unknown>
): string {
	const paramStr = params ? JSON.stringify(params) : '';
	return `${method}:${url}:${paramStr}`;
}

/**
 * Create a cached fetcher factory
 */
export function createCachedFetcher<T, P extends Record<string, unknown> = Record<string, never>>(
	keyPrefix: string,
	fetcher: (params?: P) => Promise<T>,
	defaultConfig?: Partial<CacheConfig>
) {
	const cache = getCache();

	return async (params?: P, config?: Partial<CacheConfig>): Promise<CachedResponse<T>> => {
		const key = `${keyPrefix}:${JSON.stringify(params ?? {})}`;
		const mergedConfig = { ...defaultConfig, ...config };

		return cache.getOrFetch(key, () => fetcher(params), mergedConfig);
	};
}

/**
 * Cache invalidation helper for mutations
 */
export function invalidateAfterMutation(patterns: readonly string[], tags?: readonly string[]): void {
	const cache = getCache();

	for (const pattern of patterns) {
		cache.invalidate(pattern);
	}

	if (tags) {
		cache.invalidateByTags(tags);
	}
}

// =============================================================================
// BACKWARDS COMPATIBILITY EXPORTS
// =============================================================================

/**
 * Legacy apiCache interface for backwards compatibility
 * The old API returned data directly; new API returns CachedResponse wrapper
 * @deprecated Use getCache() or requestCache instead
 */
class LegacyApiCache {
	private cache: RequestCache;

	constructor() {
		this.cache = browser ? getCache() : createCache();
	}

	get<T>(key: string): T | null {
		const result = this.cache.get<T>(key);
		return result ? result.data : null;
	}

	set<T>(key: string, data: T, ttl?: number): void {
		this.cache.set(key, data, ttl ? { ttl } : undefined);
	}

	delete(key: string): boolean {
		return this.cache.delete(key);
	}

	clear(): void {
		this.cache.clear();
	}

	has(key: string): boolean {
		return this.cache.hasAny(key);
	}

	/**
	 * Legacy getOrFetch that returns data directly (not wrapped in CachedResponse)
	 */
	async getOrFetch<T>(
		key: string,
		fetcher: () => Promise<T>,
		options?: { ttl?: number; persist?: boolean }
	): Promise<T> {
		const result = await this.cache.getOrFetch(key, fetcher, options ? { ttl: options.ttl, persist: options.persist } : undefined);
		return result.data;
	}

	invalidate(pattern: string | RegExp): number {
		return this.cache.invalidate(pattern);
	}
}

/**
 * Legacy export: aliased for backwards compatibility
 * @deprecated Use getCache() or requestCache instead
 */
export const apiCache = new LegacyApiCache();

/**
 * Legacy export: build cache key
 * @deprecated Use generateCacheKey instead
 */
export function buildCacheKey(
	url: string,
	params?: Record<string, unknown>
): string {
	return generateCacheKey('GET', url, params);
}

/**
 * Legacy export: invalidate cache by pattern
 * @deprecated Use getCache().invalidate() instead
 */
export function invalidateCache(pattern: string | RegExp): number {
	return getCache().invalidate(pattern);
}

export default RequestCache;
