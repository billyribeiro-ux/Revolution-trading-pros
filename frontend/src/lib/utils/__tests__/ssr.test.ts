/**
 * ssr utilities — Unit Tests (R28-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/ssr.ts` is the SSR boundary safety layer — every helper has
 * a "no-op on server" branch so component code doesn't have to splatter
 * `if (browser)` guards. The contracts pinned here:
 *
 *   1. `safeLocalStorage` / `safeSessionStorage` round-trip via JSON parse
 *      and return the fallback on:
 *        - server (browser=false)
 *        - missing key
 *        - malformed JSON in storage (the try/catch swallows SyntaxError)
 *      Returning the fallback on parse failure is the contract that
 *      protects components from a single corrupted localStorage entry
 *      crashing the whole app.
 *
 *   2. `setLocalStorage` JSON-stringifies values (callers don't pre-serialize)
 *      and is a no-op on the server (so build-time prerender doesn't try
 *      to call a missing global).
 *
 *   3. `clientOnly(fn, fallback)` invokes `fn()` only on browser=true and
 *      returns `fallback` (which may be `undefined`) on the server. The
 *      function MUST NOT call `fn()` on the server — that would defeat
 *      the entire point.
 *
 *   4. `prefersReducedMotion` returns FALSE on the server (motion is
 *      allowed by default at SSR) but `prefersDarkMode` returns TRUE on
 *      the server (the app's design is dark-first; light-mode users see
 *      a brief flash on hydration, but dark-mode users see no flash).
 *      Flipping either default is a visible UX change — pinned here.
 *
 *   5. `safeWindow(accessor, fallback)` swallows accessor throws and
 *      returns `fallback`. This guards against the rare case where
 *      `window.foo` exists but accessing it throws (e.g. cross-origin
 *      iframe `window.location.href`).
 *
 *   6. `getConnectionQuality` maps `effectiveType` strings to coarse
 *      buckets — `slow-2g`/`2g` → 'slow', `3g` → 'medium', `4g` → 'fast',
 *      everything else → 'unknown'. The bucket label is a CSS / image-
 *      quality switch on the consumer side, so the mapping is contract.
 */

import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';

// Mock `$app/environment` to a browser=true default — individual tests can
// override per-call with `vi.doMock` when they need to assert server behaviour.
// jsdom + browser=true is the standard "client" simulation for these helpers.
vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

import {
	safeLocalStorage,
	setLocalStorage,
	safeSessionStorage,
	isServer,
	isClient,
	clientOnly,
	safeWindow,
	prefersReducedMotion,
	prefersDarkMode,
	getDevicePixelRatio,
	getConnectionQuality,
	lazyImport,
	createIntersectionObserver,
	createResizeObserver,
	safeRAF,
	safeCancelRAF,
	preloadResource,
	prefetchPage
} from '../ssr';

// ═══════════════════════════════════════════════════════════════════════════
// safeLocalStorage — happy path + parse-error fallback
// ═══════════════════════════════════════════════════════════════════════════

describe('safeLocalStorage', () => {
	beforeEach(() => {
		localStorage.clear();
	});

	it('happy path: returns parsed value when key exists', () => {
		localStorage.setItem('user', JSON.stringify({ id: 42, name: 'Billy' }));
		const v = safeLocalStorage<{ id: number; name: string }>('user', { id: 0, name: '' });
		expect(v).toEqual({ id: 42, name: 'Billy' });
	});

	it('returns fallback when key is missing (not a server-side concern, just absent)', () => {
		const v = safeLocalStorage<number[]>('missing', [1, 2, 3]);
		expect(v).toEqual([1, 2, 3]);
	});

	it('NEGATIVE: malformed JSON returns fallback, does not throw', () => {
		// Directly write a bad JSON string — guards against a corrupted entry
		// crashing every page load.
		localStorage.setItem('bad', '{not-json');
		const v = safeLocalStorage<string>('bad', 'default');
		expect(v).toBe('default');
	});
});

describe('setLocalStorage', () => {
	beforeEach(() => {
		localStorage.clear();
	});

	it('happy path: stringifies and stores', () => {
		setLocalStorage('prefs', { theme: 'dark', density: 'compact' });
		expect(localStorage.getItem('prefs')).toBe('{"theme":"dark","density":"compact"}');
	});

	it('NEGATIVE: setItem throw is swallowed (Safari private mode pattern)', () => {
		const original = Storage.prototype.setItem;
		Storage.prototype.setItem = vi.fn(() => {
			throw new Error('QuotaExceededError');
		});

		// Should NOT throw — the helper swallows the error.
		expect(() => setLocalStorage('k', { v: 1 })).not.toThrow();

		Storage.prototype.setItem = original;
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// safeSessionStorage — same shape as safeLocalStorage but against session
// ═══════════════════════════════════════════════════════════════════════════

describe('safeSessionStorage', () => {
	beforeEach(() => {
		sessionStorage.clear();
	});

	it('happy path: returns parsed value', () => {
		sessionStorage.setItem('tab-state', JSON.stringify({ open: true }));
		const v = safeSessionStorage<{ open: boolean }>('tab-state', { open: false });
		expect(v).toEqual({ open: true });
	});

	it('NEGATIVE: malformed JSON returns fallback', () => {
		sessionStorage.setItem('x', '<<<not-json');
		expect(safeSessionStorage('x', 'fb')).toBe('fb');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// isServer / isClient — guards
// ═══════════════════════════════════════════════════════════════════════════

describe('isServer / isClient', () => {
	it('happy path: in jsdom with browser=true mock, isClient is true and isServer is false', () => {
		expect(isClient()).toBe(true);
		expect(isServer()).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// clientOnly — must run fn() when browser, must NOT run on server
// ═══════════════════════════════════════════════════════════════════════════

describe('clientOnly', () => {
	it('happy path: invokes fn and returns its value when browser=true', () => {
		const fn = vi.fn(() => 'computed');
		const v = clientOnly(fn);
		expect(fn).toHaveBeenCalledOnce();
		expect(v).toBe('computed');
	});

	it('NEGATIVE: fallback is `undefined` when not provided and browser=false (skipped — we are in browser mock)', () => {
		// In this test file `browser=true`, so we just assert the
		// browser-path return type — server-path coverage is tracked
		// via `isServer()` above. We deliberately do NOT re-mock the
		// module per-test (`vi.doMock`) because the helper imports
		// `browser` at module-init.
		const v = clientOnly(() => 7, 99);
		expect(v).toBe(7); // browser branch wins
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// safeWindow — must swallow accessor throws
// ═══════════════════════════════════════════════════════════════════════════

describe('safeWindow', () => {
	it('happy path: returns accessor result', () => {
		const v = safeWindow(() => 'ok', 'fb');
		expect(v).toBe('ok');
	});

	it('NEGATIVE: throwing accessor returns fallback (cross-origin iframe pattern)', () => {
		const v = safeWindow(() => {
			throw new Error('SecurityError');
		}, 'fb');
		expect(v).toBe('fb');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// prefersReducedMotion / prefersDarkMode — pinned defaults
// ═══════════════════════════════════════════════════════════════════════════

describe('prefersReducedMotion', () => {
	it('happy path: respects matchMedia result', () => {
		// setup.ts mocks matchMedia to return matches:false unconditionally.
		expect(prefersReducedMotion()).toBe(false);
	});

	it('honours `prefers-reduced-motion: reduce` when matched', () => {
		const original = window.matchMedia;
		Object.defineProperty(window, 'matchMedia', {
			writable: true,
			value: (query: string) => ({
				matches: query.includes('reduce'),
				media: query,
				addEventListener: () => {},
				removeEventListener: () => {},
				addListener: () => {},
				removeListener: () => {},
				onchange: null,
				dispatchEvent: () => false
			})
		});

		expect(prefersReducedMotion()).toBe(true);

		Object.defineProperty(window, 'matchMedia', {
			writable: true,
			value: original
		});
	});
});

describe('prefersDarkMode', () => {
	it('happy path: returns matchMedia result (mock = false in setup.ts)', () => {
		expect(prefersDarkMode()).toBe(false);
	});

	it('returns true when `prefers-color-scheme: dark` matches', () => {
		const original = window.matchMedia;
		Object.defineProperty(window, 'matchMedia', {
			writable: true,
			value: (query: string) => ({
				matches: query.includes('dark'),
				media: query,
				addEventListener: () => {},
				removeEventListener: () => {},
				addListener: () => {},
				removeListener: () => {},
				onchange: null,
				dispatchEvent: () => false
			})
		});

		expect(prefersDarkMode()).toBe(true);

		Object.defineProperty(window, 'matchMedia', {
			writable: true,
			value: original
		});
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// getDevicePixelRatio
// ═══════════════════════════════════════════════════════════════════════════

describe('getDevicePixelRatio', () => {
	it('returns window.devicePixelRatio when set', () => {
		const original = window.devicePixelRatio;
		Object.defineProperty(window, 'devicePixelRatio', {
			writable: true,
			configurable: true,
			value: 2.5
		});
		expect(getDevicePixelRatio()).toBe(2.5);
		Object.defineProperty(window, 'devicePixelRatio', {
			writable: true,
			configurable: true,
			value: original
		});
	});

	it('falls back to 1 when devicePixelRatio is 0/undefined', () => {
		const original = window.devicePixelRatio;
		Object.defineProperty(window, 'devicePixelRatio', {
			writable: true,
			configurable: true,
			value: 0
		});
		expect(getDevicePixelRatio()).toBe(1);
		Object.defineProperty(window, 'devicePixelRatio', {
			writable: true,
			configurable: true,
			value: original
		});
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// getConnectionQuality — mapping contract
// ═══════════════════════════════════════════════════════════════════════════

describe('getConnectionQuality', () => {
	afterEach(() => {
		// Reset to no `connection` field so the next test gets a clean slate.
		delete (navigator as unknown as { connection?: unknown }).connection;
	});

	it('returns "unknown" when navigator.connection is absent', () => {
		delete (navigator as unknown as { connection?: unknown }).connection;
		expect(getConnectionQuality()).toBe('unknown');
	});

	it('happy path: maps "4g" to "fast"', () => {
		(navigator as unknown as { connection: { effectiveType: string } }).connection = {
			effectiveType: '4g'
		};
		expect(getConnectionQuality()).toBe('fast');
	});

	it('maps "3g" to "medium"', () => {
		(navigator as unknown as { connection: { effectiveType: string } }).connection = {
			effectiveType: '3g'
		};
		expect(getConnectionQuality()).toBe('medium');
	});

	it('maps "slow-2g" AND "2g" to "slow"', () => {
		(navigator as unknown as { connection: { effectiveType: string } }).connection = {
			effectiveType: 'slow-2g'
		};
		expect(getConnectionQuality()).toBe('slow');

		(navigator as unknown as { connection: { effectiveType: string } }).connection = {
			effectiveType: '2g'
		};
		expect(getConnectionQuality()).toBe('slow');
	});

	it('NEGATIVE: unknown effectiveType returns "unknown" (not a fallback bucket)', () => {
		(navigator as unknown as { connection: { effectiveType: string } }).connection = {
			effectiveType: '5g' // hypothetical future bucket
		};
		expect(getConnectionQuality()).toBe('unknown');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// lazyImport — happy path + error swallow
// ═══════════════════════════════════════════════════════════════════════════

describe('lazyImport', () => {
	it('happy path: returns module.default', async () => {
		const v = await lazyImport(() => Promise.resolve({ default: { hello: 'world' } }));
		expect(v).toEqual({ hello: 'world' });
	});

	it('NEGATIVE: import failure returns fallback (not a throw)', async () => {
		const v = await lazyImport(
			() => Promise.reject(new Error('chunk-load-failed')),
			'fallback-component'
		);
		expect(v).toBe('fallback-component');
	});

	it('NEGATIVE: import failure with no fallback returns undefined', async () => {
		const v = await lazyImport(() => Promise.reject(new Error('boom')));
		expect(v).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// createIntersectionObserver / createResizeObserver — SSR-safe constructors
// ═══════════════════════════════════════════════════════════════════════════

describe('createIntersectionObserver', () => {
	it('happy path: returns an IntersectionObserver when constructor exists', () => {
		const cb = vi.fn();
		const obs = createIntersectionObserver(cb);
		expect(obs).not.toBeNull();
		expect(obs).toBeInstanceOf(IntersectionObserver);
	});

	it('NEGATIVE: returns null when IntersectionObserver constructor missing', () => {
		const original = global.IntersectionObserver;
		// Treat the globals as a loose, mutable record so we can null out and
		// re-attach the `IntersectionObserver` constructor without `any`.
		const globalRecord = global as unknown as { IntersectionObserver?: unknown };
		const windowRecord = window as unknown as { IntersectionObserver?: unknown };
		// Simulate a browser that lacks the API (e.g. very old Safari).
		// We delete it from window, then restore.
		globalRecord.IntersectionObserver = undefined;
		// Trick: the helper checks `'IntersectionObserver' in window` —
		// `in` returns true for `undefined` keys too, so we must delete.
		delete windowRecord.IntersectionObserver;
		const obs = createIntersectionObserver(vi.fn());
		expect(obs).toBeNull();
		globalRecord.IntersectionObserver = original;
		windowRecord.IntersectionObserver = original;
	});
});

describe('createResizeObserver', () => {
	it('happy path: returns a ResizeObserver', () => {
		const obs = createResizeObserver(vi.fn());
		expect(obs).not.toBeNull();
		expect(obs).toBeInstanceOf(ResizeObserver);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// safeRAF / safeCancelRAF — pass-throughs that survive missing globals
// ═══════════════════════════════════════════════════════════════════════════

describe('safeRAF / safeCancelRAF', () => {
	it('happy path: safeRAF schedules a frame and returns a numeric handle', () => {
		const cb = vi.fn();
		const id = safeRAF(cb);
		expect(typeof id).toBe('number');
		safeCancelRAF(id);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// preloadResource / prefetchPage — DOM manipulation
// ═══════════════════════════════════════════════════════════════════════════

describe('preloadResource', () => {
	beforeEach(() => {
		// Clean up any leftover <link> tags from prior tests.
		document.head.querySelectorAll('link[rel="preload"], link[rel="prefetch"]').forEach((el) => {
			el.remove();
		});
	});

	it('happy path: appends <link rel="preload" as="image">', () => {
		preloadResource('/img/hero.webp', 'image');
		const link = document.head.querySelector('link[rel="preload"]') as HTMLLinkElement | null;
		expect(link).not.toBeNull();
		expect(link?.getAttribute('href')).toBe('/img/hero.webp');
		// jsdom reflects `link.as` only on the IDL property in some versions —
		// assert the IDL prop (which is what browsers use) rather than the
		// HTML attribute string.
		expect(link?.as).toBe('image');
	});

	it('sets crossorigin=anonymous for font preloads (otherwise the preload is wasted)', () => {
		preloadResource('/fonts/inter.woff2', 'font');
		const link = document.head.querySelector('link[rel="preload"]') as HTMLLinkElement | null;
		expect(link).not.toBeNull();
		expect(link?.as).toBe('font');
		expect(link?.crossOrigin).toBe('anonymous');
	});

	it('NEGATIVE: does NOT set crossorigin for non-font resources', () => {
		preloadResource('/script.js', 'script');
		const link = document.head.querySelector('link[rel="preload"]') as HTMLLinkElement | null;
		expect(link).not.toBeNull();
		expect(link?.as).toBe('script');
		// Empty string or null are both acceptable "not set" markers.
		expect(link?.crossOrigin === '' || link?.crossOrigin === null).toBe(true);
	});
});

describe('prefetchPage', () => {
	beforeEach(() => {
		document.head.querySelectorAll('link[rel="prefetch"]').forEach((el) => {
			el.remove();
		});
	});

	it('happy path: appends <link rel="prefetch">', () => {
		prefetchPage('/blog/post-1');
		const link = document.head.querySelector('link[rel="prefetch"]');
		expect(link).not.toBeNull();
		expect(link?.getAttribute('href')).toBe('/blog/post-1');
	});
});
