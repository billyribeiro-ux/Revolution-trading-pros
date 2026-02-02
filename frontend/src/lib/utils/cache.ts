/**
 * Advanced Caching System
 * ═══════════════════════════════════════════════════════════════════════════
 * LRU cache with TTL and size limits
 */

interface CacheEntry<T> {
	value: T;
	timestamp: number;
	size: number;
	accessCount: number;
}

interface CacheStats {
	entries: number;
	sizeBytes: number;
	sizeMB: string;
	hits: number;
	misses: number;
	hitRate: string;
}

export class LRUCache<T> {
	private cache = new Map<string, CacheEntry<T>>();
	private maxSize: number;
	private maxAge: number;
	private currentSize = 0;
	private hits = 0;
	private misses = 0;

	constructor(maxSize = 50 * 1024 * 1024, maxAge = 5 * 60 * 1000) {
		this.maxSize = maxSize;
		this.maxAge = maxAge;
	}

	get(key: string): T | null {
		const entry = this.cache.get(key);

		if (!entry) {
			this.misses++;
			return null;
		}

		if (Date.now() - entry.timestamp > this.maxAge) {
			this.delete(key);
			this.misses++;
			return null;
		}

		entry.accessCount++;
		this.cache.delete(key);
		this.cache.set(key, entry);
		this.hits++;

		return entry.value;
	}

	set(key: string, value: T): void {
		const size = this.estimateSize(value);

		if (this.cache.has(key)) {
			this.delete(key);
		}

		while (this.currentSize + size > this.maxSize && this.cache.size > 0) {
			const firstKey = this.cache.keys().next().value;
			if (firstKey) this.delete(firstKey);
		}

		this.cache.set(key, {
			value,
			timestamp: Date.now(),
			size,
			accessCount: 1
		});

		this.currentSize += size;
	}

	has(key: string): boolean {
		const entry = this.cache.get(key);
		if (!entry) return false;

		if (Date.now() - entry.timestamp > this.maxAge) {
			this.delete(key);
			return false;
		}

		return true;
	}

	delete(key: string): boolean {
		const entry = this.cache.get(key);
		if (entry) {
			this.currentSize -= entry.size;
			this.cache.delete(key);
			return true;
		}
		return false;
	}

	clear(): void {
		this.cache.clear();
		this.currentSize = 0;
	}

	private estimateSize(value: T): number {
		try {
			const json = JSON.stringify(value);
			return new Blob([json]).size;
		} catch {
			return 1024;
		}
	}

	getStats(): CacheStats {
		const total = this.hits + this.misses;
		return {
			entries: this.cache.size,
			sizeBytes: this.currentSize,
			sizeMB: (this.currentSize / 1024 / 1024).toFixed(2),
			hits: this.hits,
			misses: this.misses,
			hitRate: total > 0 ? ((this.hits / total) * 100).toFixed(1) + '%' : '0%'
		};
	}

	resetStats(): void {
		this.hits = 0;
		this.misses = 0;
	}

	keys(): string[] {
		return Array.from(this.cache.keys());
	}

	values(): T[] {
		return Array.from(this.cache.values()).map((entry) => entry.value);
	}

	entries(): [string, T][] {
		return Array.from(this.cache.entries()).map(([key, entry]) => [key, entry.value]);
	}

	get size(): number {
		return this.cache.size;
	}
}

// Memoization decorator
export function memoize<T extends (...args: any[]) => any>(
	fn: T,
	options: { maxAge?: number; keyFn?: (...args: Parameters<T>) => string } = {}
): T {
	const cache = new LRUCache<ReturnType<T>>(10 * 1024 * 1024, options.maxAge ?? 60000);
	const keyFn = options.keyFn ?? ((...args) => JSON.stringify(args));

	return ((...args: Parameters<T>): ReturnType<T> => {
		const key = keyFn(...args);
		const cached = cache.get(key);

		if (cached !== null) {
			return cached;
		}

		const result = fn(...args);
		cache.set(key, result);
		return result;
	}) as T;
}

// Global caches
export const blockCache = new LRUCache<any>(50 * 1024 * 1024, 5 * 60 * 1000);
export const imageCache = new LRUCache<string>(100 * 1024 * 1024, 30 * 60 * 1000);
export const renderCache = new LRUCache<string>(20 * 1024 * 1024, 60 * 1000);
