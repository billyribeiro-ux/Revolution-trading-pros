/**
 * lazyLoad — Unit Tests (R27-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/lazyLoad.ts` is the dynamic-import wrapper used to load
 * heavy Svelte components on demand (idle / visible / hover preload).
 * Its contracts are subtle:
 *
 *   1. Every helper resolves to `module.default` (not the whole module).
 *      Forgetting that one extra unwrap means every consumer needs to
 *      change at the call-site — silent regression.
 *
 *   2. `lazyLoadComponent(fn, delay)` honours the `delay` (>0) — used
 *      to defer non-critical UI past the LCP frame.
 *
 *   3. `lazyLoadOnIdle` uses `requestIdleCallback` when available and
 *      falls back to `setTimeout(100)` otherwise (Safari path).
 *
 *   4. `lazyLoadOnVisible` calls `observer.disconnect()` after the
 *      first intersection — leaking observers is a memory bug we've
 *      shipped before.
 *
 *   5. `preloadComponent` is fire-and-forget: it MUST swallow errors
 *      (the user gets no UI signal from a failed preload). A rejected
 *      promise that bubbles would log a noisy "unhandled rejection".
 *
 *   6. `createLazyLoadStore` is a small state machine:
 *      - first `load()` populates `component` and toggles `loading`.
 *      - second `load()` returns the cached component (no re-import).
 *      - parallel `load()` returns `null` (loading=true) for the
 *        concurrent caller.
 *      - failed import surfaces via `store.error`.
 */

import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';

// SvelteKit's `$app/environment` exports `browser`. In jsdom the default
// stub leaves it undefined; the lazyLoad helpers fork on `browser === true`,
// so we mock both pathways explicitly.
vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

import {
	lazyLoadComponent,
	lazyLoadComponents,
	lazyLoadOnIdle,
	lazyLoadOnVisible,
	preloadComponent,
	createLazyLoadStore
} from '../lazyLoad';

// ═══════════════════════════════════════════════════════════════════════════
// lazyLoadComponent
// ═══════════════════════════════════════════════════════════════════════════

describe('lazyLoadComponent', () => {
	it('resolves to module.default (NOT the raw module)', async () => {
		const fakeComponent = { __id: 'FakeComp' };
		const importFn = vi.fn().mockResolvedValue({ default: fakeComponent });

		const result = await lazyLoadComponent(importFn);
		// CONTRACT: caller gets the default export unwrapped.
		expect(result).toBe(fakeComponent);
		expect(importFn).toHaveBeenCalledTimes(1);
	});

	it('honours the delay parameter before calling importFn', async () => {
		vi.useFakeTimers();
		const fakeComponent = { __id: 'Delayed' };
		const importFn = vi.fn().mockResolvedValue({ default: fakeComponent });

		const promise = lazyLoadComponent(importFn, 500);
		// With fake timers paused, the import shouldn't fire yet.
		expect(importFn).not.toHaveBeenCalled();

		await vi.advanceTimersByTimeAsync(500);
		const result = await promise;
		expect(result).toBe(fakeComponent);
		expect(importFn).toHaveBeenCalledTimes(1);
		vi.useRealTimers();
	});

	it('skips the delay branch when delay=0 (default)', async () => {
		const importFn = vi.fn().mockResolvedValue({ default: 'X' });
		const result = await lazyLoadComponent(importFn);
		expect(result).toBe('X');
	});

	it('NEGATIVE — propagates errors from importFn', async () => {
		const importFn = vi.fn().mockRejectedValue(new Error('module not found'));
		await expect(lazyLoadComponent(importFn)).rejects.toThrow('module not found');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// lazyLoadComponents (parallel)
// ═══════════════════════════════════════════════════════════════════════════

describe('lazyLoadComponents', () => {
	it('loads all imports in parallel and unwraps each module.default', async () => {
		const a = { __id: 'A' };
		const b = { __id: 'B' };
		const c = { __id: 'C' };
		const importFns = [
			vi.fn().mockResolvedValue({ default: a }),
			vi.fn().mockResolvedValue({ default: b }),
			vi.fn().mockResolvedValue({ default: c })
		];
		const result = await lazyLoadComponents(importFns);
		expect(result).toEqual([a, b, c]);
		// All three called — proves parallel dispatch, not sequential await.
		importFns.forEach((fn) => expect(fn).toHaveBeenCalledTimes(1));
	});

	it('NEGATIVE — rejects when ANY import fails (Promise.all semantics)', async () => {
		const importFns = [
			vi.fn().mockResolvedValue({ default: 'ok' }),
			vi.fn().mockRejectedValue(new Error('boom')),
			vi.fn().mockResolvedValue({ default: 'ok2' })
		];
		await expect(lazyLoadComponents(importFns)).rejects.toThrow('boom');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// lazyLoadOnIdle
// ═══════════════════════════════════════════════════════════════════════════

describe('lazyLoadOnIdle', () => {
	let originalRIC: typeof window.requestIdleCallback | undefined;

	beforeEach(() => {
		originalRIC = (window as typeof window & { requestIdleCallback?: unknown })
			.requestIdleCallback as typeof window.requestIdleCallback | undefined;
	});

	afterEach(() => {
		if (originalRIC) {
			(window as unknown as Record<string, unknown>).requestIdleCallback = originalRIC;
		} else {
			delete (window as unknown as Record<string, unknown>).requestIdleCallback;
		}
	});

	it('uses requestIdleCallback when available and resolves with module.default', async () => {
		// Polyfill requestIdleCallback to fire synchronously (deterministic).
		(window as unknown as Record<string, unknown>).requestIdleCallback = (
			cb: (deadline: { didTimeout: boolean; timeRemaining: () => number }) => void
		) => {
			cb({ didTimeout: false, timeRemaining: () => 50 });
			return 1;
		};

		const fake = { __id: 'idle' };
		const importFn = vi.fn().mockResolvedValue({ default: fake });
		const result = await lazyLoadOnIdle(importFn);
		expect(result).toBe(fake);
		expect(importFn).toHaveBeenCalledTimes(1);
	});

	it('falls back to setTimeout when requestIdleCallback is missing (Safari path)', async () => {
		// Delete the polyfill — exercises the fallback branch.
		delete (window as unknown as Record<string, unknown>).requestIdleCallback;

		vi.useFakeTimers();
		const fake = { __id: 'fallback' };
		const importFn = vi.fn().mockResolvedValue({ default: fake });
		const promise = lazyLoadOnIdle(importFn);

		// 100ms is the documented Safari fallback.
		await vi.advanceTimersByTimeAsync(100);
		const result = await promise;
		expect(result).toBe(fake);
		vi.useRealTimers();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// lazyLoadOnVisible (IntersectionObserver)
// ═══════════════════════════════════════════════════════════════════════════

describe('lazyLoadOnVisible', () => {
	let originalIO: typeof IntersectionObserver;

	beforeEach(() => {
		originalIO = global.IntersectionObserver;
	});

	afterEach(() => {
		global.IntersectionObserver = originalIO;
	});

	it('resolves when the element intersects AND calls observer.disconnect() exactly once', async () => {
		const disconnectSpy = vi.fn();
		const observeSpy = vi.fn();

		let captured: ((entries: IntersectionObserverEntry[]) => void) | null = null;
		global.IntersectionObserver = class MockIO {
			constructor(cb: (entries: IntersectionObserverEntry[]) => void) {
				captured = cb;
			}
			observe = observeSpy;
			disconnect = disconnectSpy;
			unobserve = vi.fn();
			takeRecords = vi.fn();
		} as unknown as typeof IntersectionObserver;

		const fake = { __id: 'visible' };
		const importFn = vi.fn().mockResolvedValue({ default: fake });
		const element = document.createElement('div');

		const promise = lazyLoadOnVisible(element, importFn);
		expect(observeSpy).toHaveBeenCalledWith(element);

		// Fire the intersection.
		captured!([{ isIntersecting: true } as IntersectionObserverEntry]);

		const result = await promise;
		expect(result).toBe(fake);
		// CONTRACT: disconnect must fire to free the observer.
		expect(disconnectSpy).toHaveBeenCalledTimes(1);
	});

	it('does NOT resolve while the element is not intersecting', async () => {
		let captured: ((entries: IntersectionObserverEntry[]) => void) | null = null;
		global.IntersectionObserver = class MockIO {
			constructor(cb: (entries: IntersectionObserverEntry[]) => void) {
				captured = cb;
			}
			observe = vi.fn();
			disconnect = vi.fn();
			unobserve = vi.fn();
			takeRecords = vi.fn();
		} as unknown as typeof IntersectionObserver;

		const importFn = vi.fn().mockResolvedValue({ default: 'x' });
		const element = document.createElement('div');
		const promise = lazyLoadOnVisible(element, importFn);

		// Race: non-intersecting entry must NOT trigger import.
		captured!([{ isIntersecting: false } as IntersectionObserverEntry]);
		// Now fire an intersecting one to unblock the test.
		captured!([{ isIntersecting: true } as IntersectionObserverEntry]);
		await promise;

		// Only the second (intersecting) entry should have triggered an import.
		expect(importFn).toHaveBeenCalledTimes(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// preloadComponent
// ═══════════════════════════════════════════════════════════════════════════

describe('preloadComponent', () => {
	it('returns undefined synchronously (fire-and-forget contract)', () => {
		const importFn = vi.fn().mockResolvedValue({ default: 'x' });
		const result = preloadComponent(importFn);
		expect(result).toBeUndefined();
		expect(importFn).toHaveBeenCalledTimes(1);
	});

	it('NEGATIVE — swallows rejected imports without throwing (no unhandled rejection)', async () => {
		const importFn = vi.fn().mockRejectedValue(new Error('preload failed'));
		// CONTRACT: failed preloads stay silent. We rely on the .catch() in the
		// implementation; this test passes if no unhandled rejection bubbles.
		expect(() => preloadComponent(importFn)).not.toThrow();
		// Let the microtask queue drain so the .catch() runs.
		await new Promise((resolve) => setTimeout(resolve, 0));
		expect(importFn).toHaveBeenCalledTimes(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// createLazyLoadStore
// ═══════════════════════════════════════════════════════════════════════════

describe('createLazyLoadStore', () => {
	it('starts with component=null, loading=false, error=null', () => {
		const store = createLazyLoadStore();
		expect(store.component).toBeNull();
		expect(store.loading).toBe(false);
		expect(store.error).toBeNull();
	});

	it('first load() populates component and resets loading to false', async () => {
		const store = createLazyLoadStore<{ __id: string }>();
		const fake = { __id: 'lazy' };
		const importFn = vi.fn().mockResolvedValue({ default: fake });

		const result = await store.load(importFn);
		expect(result).toBe(fake);
		expect(store.component).toBe(fake);
		expect(store.loading).toBe(false);
		expect(store.error).toBeNull();
	});

	it('second load() returns the cached component without re-importing', async () => {
		const store = createLazyLoadStore<{ __id: string }>();
		const fake = { __id: 'cached' };
		const importFn = vi.fn().mockResolvedValue({ default: fake });

		await store.load(importFn);
		await store.load(importFn);
		// CONTRACT: cache hit — importFn should fire exactly once.
		expect(importFn).toHaveBeenCalledTimes(1);
	});

	it('NEGATIVE — failed import surfaces via store.error and returns null', async () => {
		const store = createLazyLoadStore();
		const importFn = vi.fn().mockRejectedValue(new Error('not found'));

		const result = await store.load(importFn);
		expect(result).toBeNull();
		expect(store.error).toBeInstanceOf(Error);
		expect(store.error?.message).toBe('not found');
		expect(store.loading).toBe(false);
		// Failure path leaves component=null.
		expect(store.component).toBeNull();
	});
});
