// ============================================================
// MULTI-TIER CACHING LAYER
// Tier 1: In-memory Map (fastest)
// Tier 2: localStorage with TTL (persists across reloads)
// ============================================================

import * as v from 'valibot';
import { browser } from '$app/env';
import { createSubscriber } from 'svelte/reactivity';
import { providerNameSchema } from './schemas.js';
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
	HISTORICAL_PRICES: 3_600_000
} as const;

/**
 * Every cache key starts with this. It must stay distinct from the other
 * `rtp:calc:*` keys (CTA / lead-modal dismissals) so `clearAll()` only ever
 * removes cached market data.
 */
const KEY_PREFIX = 'rtp:calc:cache:';

/** Namespaces written under `rtp:calc:<ns>:` before the dedicated prefix existed. */
const LEGACY_PREFIXES = [
	'search',
	'quote',
	'expirations',
	'chain',
	'riskfree',
	'dividend',
	'earnings',
	'hv'
].map((namespace) => `rtp:calc:${namespace}:`);

interface CacheItem<T> {
	data: T;
	timestamp: number;
	ttl: number;
	source: DataProviderName;
}

/** Envelope of a persisted entry; `data` is validated separately per caller. */
const storedItemSchema = v.object({
	data: v.unknown(),
	timestamp: v.number(),
	ttl: v.number(),
	source: providerNameSchema
});

/**
 * Create a cache instance.
 * `getStats()` is reactive: read it in a `$derived` / template and it updates
 * whenever entries are written or removed.
 */
export function createCache() {
	const memoryCache = new Map<string, CacheItem<unknown>>();

	let notifyChange: (() => void) | undefined;
	const trackChanges = createSubscriber((update) => {
		notifyChange = update;
		return () => {
			notifyChange = undefined;
		};
	});

	function makeKey(namespace: string, ...parts: string[]): string {
		return `${KEY_PREFIX}${namespace}:${parts.join(':')}`;
	}

	function isFresh(item: { timestamp: number; ttl: number }): boolean {
		return Date.now() - item.timestamp < item.ttl;
	}

	function getMemory<T>(key: string): T | null {
		const item = memoryCache.get(key) as CacheItem<T> | undefined;
		if (!item) return null;
		if (isFresh(item)) return item.data;
		memoryCache.delete(key);
		return null;
	}

	function getStorage<T>(key: string, schema?: v.GenericSchema<unknown, T>): T | null {
		if (!browser) return null;
		try {
			const raw = localStorage.getItem(key);
			if (!raw) return null;

			const envelope = v.safeParse(storedItemSchema, JSON.parse(raw));
			if (!envelope.success || !isFresh(envelope.output)) {
				localStorage.removeItem(key);
				return null;
			}

			// Without a schema the caller vouches for the payload's shape.
			const data = schema ? v.safeParse(schema, envelope.output.data) : null;
			if (data && !data.success) {
				localStorage.removeItem(key);
				return null;
			}

			const item: CacheItem<T> = {
				...envelope.output,
				data: data ? data.output : (envelope.output.data as T)
			};
			memoryCache.set(key, item);
			return item.data;
		} catch {
			// Blocked storage, malformed JSON, …: treat as a miss.
			return null;
		}
	}

	/**
	 * Read a fresh entry: memory first, then localStorage. Pass `schema` to
	 * validate a persisted payload before it is trusted (it may be stale-shaped
	 * from an older release or edited by hand); invalid entries are dropped.
	 */
	function get<T>(key: string, schema?: v.GenericSchema<unknown, T>): T | null {
		const memResult = getMemory<T>(key);
		if (memResult !== null) return memResult;
		return getStorage(key, schema);
	}

	/** Drop every expired cache entry from localStorage; returns how many went. */
	function pruneExpiredStorage(): number {
		let removed = 0;
		for (let i = localStorage.length - 1; i >= 0; i--) {
			const key = localStorage.key(i);
			if (!key?.startsWith(KEY_PREFIX)) continue;
			try {
				const envelope = v.safeParse(storedItemSchema, JSON.parse(localStorage.getItem(key) ?? ''));
				if (envelope.success && isFresh(envelope.output)) continue;
			} catch {
				// Unparseable: fall through and remove it.
			}
			localStorage.removeItem(key);
			removed++;
		}
		return removed;
	}

	function set<T>(key: string, data: T, ttl: number, source: DataProviderName): void {
		const item: CacheItem<T> = { data, timestamp: Date.now(), ttl, source };
		memoryCache.set(key, item);
		notifyChange?.();
		if (!browser) return;

		const serialized = JSON.stringify(item);
		try {
			localStorage.setItem(key, serialized);
		} catch {
			// Usually the quota: expired entries are never read back, so they pile
			// up. Prune them and retry once; the memory tier works regardless.
			try {
				if (pruneExpiredStorage() > 0) localStorage.setItem(key, serialized);
			} catch {
				/* storage unavailable or still full */
			}
		}
	}

	function invalidate(key: string): void {
		memoryCache.delete(key);
		notifyChange?.();
		if (!browser) return;
		try {
			localStorage.removeItem(key);
		} catch {
			/* ignore */
		}
	}

	function invalidateByPrefix(...prefixes: string[]): void {
		const matches = (key: string) => prefixes.some((prefix) => key.startsWith(prefix));

		for (const key of memoryCache.keys()) {
			if (matches(key)) memoryCache.delete(key);
		}
		notifyChange?.();
		if (!browser) return;
		try {
			for (let i = localStorage.length - 1; i >= 0; i--) {
				const key = localStorage.key(i);
				if (key && matches(key)) localStorage.removeItem(key);
			}
		} catch {
			/* ignore */
		}
	}

	/** Remove every cached market-data entry (and entries from the old key layout). */
	function clearAll(): void {
		invalidateByPrefix(KEY_PREFIX, ...LEGACY_PREFIXES);
	}

	function getStats(): { memoryEntries: number; memoryBytes: number } {
		trackChanges();
		let bytes = 0;
		for (const [key, value] of memoryCache.entries()) {
			bytes += key.length * 2 + JSON.stringify(value).length * 2;
		}
		return { memoryEntries: memoryCache.size, memoryBytes: bytes };
	}

	return { makeKey, get, set, invalidate, invalidateByPrefix, clearAll, getStats };
}

export type CacheInstance = ReturnType<typeof createCache>;
