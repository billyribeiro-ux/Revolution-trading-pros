// ============================================================
// MULTI-TIER CACHING LAYER
// Tier 1: In-memory Map (fastest)
// Tier 2: localStorage with TTL (persists across reloads)
// ============================================================

import type { DataProviderName } from './types.js';

/** TTL values in milliseconds */
export const CACHE_TTL = {
	QUOTE: 5_000,
	OPTIONS_CHAIN: 30_000,
	EXPIRATIONS: 3_600_000,
	IV_SURFACE: 300_000,
	HISTORICAL_VOL: 3_600_000,
	RISK_FREE_RATE: 86_400_000,
	DIVIDEND: 86_400_000,
	EARNINGS: 21_600_000,
	TICKER_SEARCH: 600_000,
	HISTORICAL_PRICES: 3_600_000,
} as const;

interface CacheItem<T> {
	data: T;
	timestamp: number;
	ttl: number;
	source: DataProviderName;
}

/**
 * Create a cache instance.
 */
export function createCache() {
	const memoryCache = new Map<string, CacheItem<unknown>>();

	function makeKey(namespace: string, ...parts: string[]): string {
		return `rtp:calc:${namespace}:${parts.join(':')}`;
	}

	function isValid<T>(item: CacheItem<T> | null | undefined): item is CacheItem<T> {
		if (!item) return false;
		return Date.now() - item.timestamp < item.ttl;
	}

	function getMemory<T>(key: string): T | null {
		const item = memoryCache.get(key) as CacheItem<T> | undefined;
		if (isValid(item)) return item.data;
		if (item) memoryCache.delete(key);
		return null;
	}

	function getStorage<T>(key: string): T | null {
		if (typeof window === 'undefined') return null;
		try {
			const raw = localStorage.getItem(key);
			if (!raw) return null;
			const item: CacheItem<T> = JSON.parse(raw);
			if (isValid(item)) {
				memoryCache.set(key, item);
				return item.data;
			}
			localStorage.removeItem(key);
			return null;
		} catch {
			return null;
		}
	}

	function get<T>(key: string): T | null {
		const memResult = getMemory<T>(key);
		if (memResult !== null) return memResult;
		return getStorage<T>(key);
	}

	function set<T>(key: string, data: T, ttl: number, source: DataProviderName): void {
		const item: CacheItem<T> = { data, timestamp: Date.now(), ttl, source };
		memoryCache.set(key, item as CacheItem<unknown>);
		if (typeof window !== 'undefined') {
			try {
				localStorage.setItem(key, JSON.stringify(item));
			} catch {
				// localStorage full — memory cache still works
			}
		}
	}

	function invalidate(key: string): void {
		memoryCache.delete(key);
		if (typeof window !== 'undefined') {
			try { localStorage.removeItem(key); } catch { /* ignore */ }
		}
	}

	function invalidateByPrefix(prefix: string): void {
		for (const key of memoryCache.keys()) {
			if (key.startsWith(prefix)) memoryCache.delete(key);
		}
		if (typeof window !== 'undefined') {
			try {
				for (let i = localStorage.length - 1; i >= 0; i--) {
					const key = localStorage.key(i);
					if (key?.startsWith(prefix)) localStorage.removeItem(key);
				}
			} catch { /* ignore */ }
		}
	}

	function clearAll(): void {
		invalidateByPrefix('rtp:calc:');
	}

	function getStats(): { memoryEntries: number; memoryBytes: number } {
		let bytes = 0;
		for (const [key, value] of memoryCache.entries()) {
			bytes += key.length * 2 + JSON.stringify(value).length * 2;
		}
		return { memoryEntries: memoryCache.size, memoryBytes: bytes };
	}

	return { makeKey, get, set, invalidate, invalidateByPrefix, clearAll, getStats };
}

export type CacheInstance = ReturnType<typeof createCache>;
