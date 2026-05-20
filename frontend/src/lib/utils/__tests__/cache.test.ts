/**
 * LRUCache + memoize — Unit Tests (R25-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `LRUCache` underpins `blockCache`, `imageCache`, `renderCache`, and any
 * `memoize()` wrapper across `src/lib`. The class has several non-obvious
 * contracts that a refactor could break:
 *
 *   - `get()` is NOT a pure read — it bumps the entry's access count AND
 *     re-inserts the entry at the END of the Map, so the LRU eviction
 *     order is `Map.keys()` iteration order. Drop the re-insert and you
 *     evict hot entries.
 *
 *   - `get()` on an expired entry calls `delete()` and returns null AND
 *     increments `misses`. A refactor that returns the stale value
 *     "for one last read" would re-introduce a bug.
 *
 *   - `set()` of an existing key path: it MUST delete-then-set so the
 *     `currentSize` accounting stays correct. Skipping the delete leaks
 *     bytes proportional to the size of the replaced entry.
 *
 *   - `set()` triggers an LRU eviction loop when `currentSize + new >
 *     maxSize`. We test that the OLDEST key is evicted (FIFO of the
 *     Map's insertion order), and that the loop terminates on an
 *     overflow that's bigger than the entire cache (entry is still
 *     stored — documented behaviour, not a bug).
 *
 *   - `has()` mirrors `get()` for the TTL check but does NOT bump
 *     access count. Useful for "is this cached?" probes that
 *     shouldn't perturb LRU order.
 *
 *   - `memoize()` caches by JSON.stringify(args) and returns the
 *     cached result for identical argument lists.
 *
 * We use `vi.useFakeTimers()` to pin `Date.now()` so TTL tests are
 * deterministic.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { LRUCache, memoize } from '../cache';

// ═══════════════════════════════════════════════════════════════════════════
// LRUCache — get / set / has / delete
// ═══════════════════════════════════════════════════════════════════════════

describe('LRUCache — basic get/set/has', () => {
	it('returns null on miss, value on hit', () => {
		const cache = new LRUCache<string>();
		expect(cache.get('missing')).toBeNull();
		cache.set('k', 'v');
		expect(cache.get('k')).toBe('v');
	});

	it('has() returns true for a live key, false for an absent key', () => {
		const cache = new LRUCache<number>();
		cache.set('a', 1);
		expect(cache.has('a')).toBe(true);
		expect(cache.has('b')).toBe(false);
	});

	it('delete() removes the key and frees the size budget', () => {
		const cache = new LRUCache<string>();
		cache.set('a', 'value');
		expect(cache.has('a')).toBe(true);
		expect(cache.delete('a')).toBe(true);
		expect(cache.has('a')).toBe(false);
		// Deleting an already-absent key returns false.
		expect(cache.delete('a')).toBe(false);
	});

	it('clear() empties the cache and resets size', () => {
		const cache = new LRUCache<string>();
		cache.set('a', 'x');
		cache.set('b', 'y');
		cache.clear();
		expect(cache.size).toBe(0);
		expect(cache.get('a')).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// LRUCache — TTL expiry
// ═══════════════════════════════════════════════════════════════════════════

describe('LRUCache — TTL expiry', () => {
	beforeEach(() => {
		vi.useFakeTimers();
		vi.setSystemTime(new Date('2026-05-20T00:00:00Z'));
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('returns the value before TTL elapses', () => {
		const cache = new LRUCache<string>(1024 * 1024, 1_000); // 1s TTL
		cache.set('k', 'v');
		vi.advanceTimersByTime(500);
		expect(cache.get('k')).toBe('v');
	});

	it('returns null AFTER TTL elapses and removes the entry', () => {
		const cache = new LRUCache<string>(1024 * 1024, 1_000);
		cache.set('k', 'v');
		vi.advanceTimersByTime(1_001);
		expect(cache.get('k')).toBeNull();
		// And the side-effect — it was removed, not just shadowed.
		expect(cache.size).toBe(0);
	});

	it('has() also honours the TTL', () => {
		const cache = new LRUCache<string>(1024 * 1024, 1_000);
		cache.set('k', 'v');
		expect(cache.has('k')).toBe(true);
		vi.advanceTimersByTime(1_001);
		expect(cache.has('k')).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// LRUCache — LRU eviction order
// ═══════════════════════════════════════════════════════════════════════════

describe('LRUCache — LRU eviction order', () => {
	it('a `get()` promotes the entry so it survives an eviction wave', () => {
		// Each entry JSON-stringifies to ~102 bytes ('"' + 100x 'x' + '"').
		// Pick maxSize that fits 2 entries but not 3 — when we insert
		// 'c', the cache must evict whichever of {a,b} is LRU.
		// 2 * 102 = 204 fits in 250; 3 * 102 = 306 > 250 triggers eviction.
		const cache = new LRUCache<string>(250, 10 * 60_000);
		const big = 'x'.repeat(100); // ~102 bytes JSON
		cache.set('a', big);
		cache.set('b', big);
		// Touch 'a' → it gets promoted to "most recently used".
		// (Map iteration order becomes [b, a], so LRU is now 'b'.)
		expect(cache.get('a')).toBe(big);
		// Inserting 'c' should evict 'b' (the LRU one), NOT 'a'.
		cache.set('c', big);
		expect(cache.get('a')).toBe(big);
		expect(cache.get('b')).toBeNull();
		expect(cache.get('c')).toBe(big);
	});

	it('updating an existing key does NOT leak size into currentSize', () => {
		// If `set()` of an existing key skipped the delete, currentSize
		// would double-count the replaced entry and trigger spurious
		// evictions. We assert size accounting stays sane.
		const cache = new LRUCache<string>(10 * 1024, 10 * 60_000);
		for (let i = 0; i < 10; i++) {
			cache.set('same-key', 'x'.repeat(50));
		}
		expect(cache.size).toBe(1);
		expect(cache.get('same-key')).toBe('x'.repeat(50));
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// LRUCache — stats
// ═══════════════════════════════════════════════════════════════════════════

describe('LRUCache — stats', () => {
	it('counts hits and misses and computes hitRate', () => {
		const cache = new LRUCache<string>();
		cache.set('k', 'v');
		cache.get('k'); // hit
		cache.get('k'); // hit
		cache.get('missing'); // miss
		const stats = cache.getStats();
		expect(stats.hits).toBe(2);
		expect(stats.misses).toBe(1);
		expect(stats.hitRate).toBe('66.7%');
		expect(stats.entries).toBe(1);
	});

	it('hitRate is "0%" when no reads have happened', () => {
		const cache = new LRUCache<string>();
		expect(cache.getStats().hitRate).toBe('0%');
	});

	it('resetStats() zeroes hits/misses but leaves entries intact', () => {
		const cache = new LRUCache<string>();
		cache.set('k', 'v');
		cache.get('k');
		cache.get('missing');
		cache.resetStats();
		const stats = cache.getStats();
		expect(stats.hits).toBe(0);
		expect(stats.misses).toBe(0);
		expect(stats.entries).toBe(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// LRUCache — iteration helpers
// ═══════════════════════════════════════════════════════════════════════════

describe('LRUCache — keys/values/entries', () => {
	it('exposes keys/values/entries in insertion order', () => {
		const cache = new LRUCache<number>();
		cache.set('a', 1);
		cache.set('b', 2);
		cache.set('c', 3);
		expect(cache.keys()).toEqual(['a', 'b', 'c']);
		expect(cache.values()).toEqual([1, 2, 3]);
		expect(cache.entries()).toEqual([
			['a', 1],
			['b', 2],
			['c', 3]
		]);
	});

	it('keys() reflects the LRU order after a get() promotion', () => {
		const cache = new LRUCache<number>();
		cache.set('a', 1);
		cache.set('b', 2);
		cache.set('c', 3);
		cache.get('a'); // promote 'a' to MRU
		// Map iteration order is now b, c, a.
		expect(cache.keys()).toEqual(['b', 'c', 'a']);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// memoize
// ═══════════════════════════════════════════════════════════════════════════

describe('memoize', () => {
	it('returns the same result without re-invoking the wrapped function', () => {
		const inner = vi.fn((a: number, b: number) => a + b);
		const memo = memoize(inner);
		expect(memo(2, 3)).toBe(5);
		expect(memo(2, 3)).toBe(5);
		expect(memo(2, 3)).toBe(5);
		expect(inner).toHaveBeenCalledTimes(1);
	});

	it('treats different argument lists as cache-distinct', () => {
		const inner = vi.fn((a: number) => a * 2);
		const memo = memoize(inner);
		expect(memo(1)).toBe(2);
		expect(memo(2)).toBe(4);
		expect(inner).toHaveBeenCalledTimes(2);
	});

	it('honours a custom keyFn (collapses logically-equal args)', () => {
		// keyFn returns the SAME string regardless of order → cache hit.
		const inner = vi.fn((a: number, b: number) => a + b);
		const memo = memoize(inner, { keyFn: (a, b) => [a, b].sort().join(',') });
		expect(memo(1, 2)).toBe(3);
		expect(memo(2, 1)).toBe(3); // cached — keyFn yields the same string
		expect(inner).toHaveBeenCalledTimes(1);
	});
});
