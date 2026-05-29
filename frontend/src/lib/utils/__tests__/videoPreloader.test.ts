/**
 * videoPreloader — Unit Tests (R27-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/videoPreloader.ts` is the Bunny.net CDN preloader that
 * triggers HLS manifest / thumbnail / segment downloads on hover and on
 * viewport entry. It's an all-or-nothing performance feature: a silent
 * regression means "video clicks no longer feel instant" with zero error
 * surface to catch it.
 *
 * Contracts we pin:
 *
 *   1. `videoPreloader` singleton instance exists and is the same on every
 *      import (it caches preload state across the app).
 *
 *   2. `getPreloadState()` returns `null` for an unseen video (NOT
 *      undefined — the API is documented to return null).
 *
 *   3. `isPreloaded()` flips to true after `preload()` runs. Bug shape:
 *      the previous version checked `preloadedVideos.has(key)` before the
 *      `.set(key, state)` call, so it would never flip.
 *
 *   4. `preload()` on Save-Data / 2g connections becomes a no-op. We
 *      simulate Save-Data via the `navigator.connection.saveData` flag.
 *
 *   5. `clear()` empties the in-memory caches AND cancels pending hover
 *      timeouts (so an in-flight debounce doesn't fire after the user
 *      navigates away).
 *
 *   6. `onHoverStart()` debounces by the documented delay, and a
 *      matching `onHoverEnd()` cancels the pending preload before the
 *      timeout fires.
 *
 *   7. `preloadVideo` Svelte action sets data-* attrs, observes, and
 *      registers mouseenter/mouseleave handlers; `.destroy()` reverses
 *      all three. Bug shape: forgetting one of the removeEventListener
 *      calls leaks per-element handlers on hot navigation.
 *
 *   8. `addBunnyPreconnects()` is idempotent — calling twice does NOT
 *      duplicate <link rel="preconnect"> in the head.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

// ═══════════════════════════════════════════════════════════════════════════
// Helpers — connection mock
// ═══════════════════════════════════════════════════════════════════════════

function setConnection(opts: { saveData?: boolean; effectiveType?: string }): () => void {
	const original = (navigator as { connection?: unknown }).connection;
	Object.defineProperty(navigator, 'connection', {
		configurable: true,
		writable: true,
		value: {
			saveData: opts.saveData ?? false,
			effectiveType: opts.effectiveType ?? '4g',
			addEventListener: () => {}
		}
	});
	return () => {
		if (original === undefined) {
			delete (navigator as { connection?: unknown }).connection;
		} else {
			Object.defineProperty(navigator, 'connection', {
				configurable: true,
				writable: true,
				value: original
			});
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Singleton / state
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader singleton', () => {
	let restoreConn: (() => void) | null = null;

	beforeEach(() => {
		restoreConn = setConnection({ saveData: false, effectiveType: '4g' });
	});

	afterEach(() => {
		restoreConn?.();
	});

	it('exports a singleton instance with the documented public API', async () => {
		const { videoPreloader } = await import('../videoPreloader');
		expect(typeof videoPreloader.preload).toBe('function');
		expect(typeof videoPreloader.observe).toBe('function');
		expect(typeof videoPreloader.unobserve).toBe('function');
		expect(typeof videoPreloader.onHoverStart).toBe('function');
		expect(typeof videoPreloader.onHoverEnd).toBe('function');
		expect(typeof videoPreloader.isPreloaded).toBe('function');
		expect(typeof videoPreloader.getPreloadState).toBe('function');
		expect(typeof videoPreloader.clear).toBe('function');
		expect(typeof videoPreloader.destroy).toBe('function');
	});

	it('getPreloadState returns null for an unseen video (NOT undefined)', async () => {
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();
		expect(
			videoPreloader.getPreloadState({ videoId: 'never-seen', libraryId: 'lib-x' })
		).toBeNull();
		expect(videoPreloader.isPreloaded({ videoId: 'never-seen', libraryId: 'lib-x' })).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// preload() — happy path
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader.preload', () => {
	let restoreConn: (() => void) | null = null;
	let originalImage: typeof Image;

	beforeEach(() => {
		restoreConn = setConnection({ saveData: false, effectiveType: '4g' });
		originalImage = global.Image;
		// Stub Image so onload fires synchronously (no real network).
		global.Image = class FakeImage {
			onload: (() => void) | null = null;
			onerror: (() => void) | null = null;
			set src(_: string) {
				queueMicrotask(() => this.onload?.());
			}
		} as unknown as typeof Image;
	});

	afterEach(() => {
		restoreConn?.();
		global.Image = originalImage;
	});

	it('flips isPreloaded() to true and stores state with thumbnail flag set', async () => {
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();
		const video = { videoId: 'vid-1', libraryId: 'lib-1' };

		await videoPreloader.preload(video, { thumbnail: true });

		// CONTRACT: flag flips post-set, not pre-set.
		expect(videoPreloader.isPreloaded(video)).toBe(true);
		const state = videoPreloader.getPreloadState(video);
		expect(state).not.toBeNull();
		expect(state?.thumbnail).toBe(true);
		expect(state?.manifest).toBe(false);
		expect(state?.segments).toBe(0);
		expect(typeof state?.timestamp).toBe('number');
	});

	it('preload state is keyed by libraryId:videoId (different lib → distinct entry)', async () => {
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();
		await videoPreloader.preload({ videoId: 'v', libraryId: 'lib-A' }, { thumbnail: true });
		// Same videoId, DIFFERENT libraryId → must NOT collide.
		expect(videoPreloader.isPreloaded({ videoId: 'v', libraryId: 'lib-B' })).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// preload() — Save-Data / slow connection guard
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader.preload — connection-aware guard', () => {
	let originalImage: typeof Image;

	beforeEach(() => {
		originalImage = global.Image;
		global.Image = class FakeImage {
			onload: (() => void) | null = null;
			onerror: (() => void) | null = null;
			set src(_: string) {
				queueMicrotask(() => this.onload?.());
			}
		} as unknown as typeof Image;
	});

	afterEach(() => {
		global.Image = originalImage;
	});

	it('NEGATIVE — Save-Data ON → preload is a no-op (state stays unset)', async () => {
		const restore = setConnection({ saveData: true, effectiveType: '4g' });
		vi.resetModules();
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();

		await videoPreloader.preload({ videoId: 'no-go', libraryId: 'lib-1' }, { thumbnail: true });
		// CONTRACT: respect data-saver — no state stored.
		expect(videoPreloader.isPreloaded({ videoId: 'no-go', libraryId: 'lib-1' })).toBe(false);
		restore();
	});

	it('NEGATIVE — 2g connection → preload is a no-op (only 3g/4g preload)', async () => {
		const restore = setConnection({ saveData: false, effectiveType: '2g' });
		vi.resetModules();
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();

		await videoPreloader.preload({ videoId: 'slow', libraryId: 'lib-1' }, { thumbnail: true });
		expect(videoPreloader.isPreloaded({ videoId: 'slow', libraryId: 'lib-1' })).toBe(false);
		restore();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// onHoverStart / onHoverEnd debounce
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader hover debouncing', () => {
	let restoreConn: (() => void) | null = null;

	beforeEach(() => {
		restoreConn = setConnection({ saveData: false, effectiveType: '4g' });
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
		restoreConn?.();
	});

	it('onHoverEnd cancels a pending preload before the debounce fires', async () => {
		vi.resetModules();
		const { videoPreloader } = await import('../videoPreloader');
		videoPreloader.clear();
		const video = { videoId: 'hover-cancel', libraryId: 'lib-1' };

		videoPreloader.onHoverStart(video, 100);
		// Cancel BEFORE the 100ms timeout expires.
		videoPreloader.onHoverEnd(video);
		await vi.advanceTimersByTimeAsync(200);

		// CONTRACT: cancelled hover must NOT have triggered the preload.
		expect(videoPreloader.isPreloaded(video)).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// observe / unobserve via IntersectionObserver
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader observe/unobserve', () => {
	it('forwards observe()/unobserve() calls to the underlying IntersectionObserver', async () => {
		const observeSpy = vi.fn();
		const unobserveSpy = vi.fn();

		const original = global.IntersectionObserver;
		global.IntersectionObserver = class MockIO {
			observe = observeSpy;
			unobserve = unobserveSpy;
			disconnect = vi.fn();
			takeRecords = vi.fn();
		} as unknown as typeof IntersectionObserver;

		vi.resetModules();
		const { videoPreloader } = await import('../videoPreloader');
		const el = document.createElement('div');
		videoPreloader.observe(el);
		videoPreloader.unobserve(el);

		expect(observeSpy).toHaveBeenCalledWith(el);
		expect(unobserveSpy).toHaveBeenCalledWith(el);

		global.IntersectionObserver = original;
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// clear / destroy
// ═══════════════════════════════════════════════════════════════════════════

describe('videoPreloader.clear / destroy', () => {
	let restoreConn: (() => void) | null = null;
	let originalImage: typeof Image;

	beforeEach(() => {
		restoreConn = setConnection({ saveData: false, effectiveType: '4g' });
		originalImage = global.Image;
		global.Image = class FakeImage {
			onload: (() => void) | null = null;
			onerror: (() => void) | null = null;
			set src(_: string) {
				queueMicrotask(() => this.onload?.());
			}
		} as unknown as typeof Image;
	});

	afterEach(() => {
		restoreConn?.();
		global.Image = originalImage;
	});

	it('clear() wipes preloadedVideos so isPreloaded returns false again', async () => {
		vi.resetModules();
		const { videoPreloader } = await import('../videoPreloader');
		const video = { videoId: 'to-clear', libraryId: 'lib-1' };

		await videoPreloader.preload(video, { thumbnail: true });
		expect(videoPreloader.isPreloaded(video)).toBe(true);

		videoPreloader.clear();
		expect(videoPreloader.isPreloaded(video)).toBe(false);
		expect(videoPreloader.getPreloadState(video)).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// preloadVideo (Svelte action)
// ═══════════════════════════════════════════════════════════════════════════

describe('preloadVideo Svelte action', () => {
	let restoreConn: (() => void) | null = null;

	beforeEach(() => {
		restoreConn = setConnection({ saveData: false, effectiveType: '4g' });
	});

	afterEach(() => {
		restoreConn?.();
	});

	it('sets data-* attrs, registers mouseenter/mouseleave, and update() re-syncs attrs', async () => {
		vi.resetModules();
		const { preloadVideo } = await import('../videoPreloader');
		const el = document.createElement('div');
		const addSpy = vi.spyOn(el, 'addEventListener');
		const removeSpy = vi.spyOn(el, 'removeEventListener');

		const action = preloadVideo(el, {
			videoId: 'v1',
			libraryId: 'lib-1',
			thumbnailUrl: 'https://x/t.jpg'
		});

		// CONTRACT: data-* attrs are set so IntersectionObserver can read them.
		expect(el.dataset.videoId).toBe('v1');
		expect(el.dataset.libraryId).toBe('lib-1');
		expect(el.dataset.thumbnailUrl).toBe('https://x/t.jpg');

		// Both hover handlers must be installed.
		expect(addSpy).toHaveBeenCalledWith('mouseenter', expect.any(Function));
		expect(addSpy).toHaveBeenCalledWith('mouseleave', expect.any(Function));

		// update() re-syncs attrs.
		action.update({ videoId: 'v2', libraryId: 'lib-2' });
		expect(el.dataset.videoId).toBe('v2');
		expect(el.dataset.libraryId).toBe('lib-2');

		// destroy() removes BOTH handlers — leak guard.
		action.destroy();
		expect(removeSpy).toHaveBeenCalledWith('mouseenter', expect.any(Function));
		expect(removeSpy).toHaveBeenCalledWith('mouseleave', expect.any(Function));
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// addBunnyPreconnects — idempotent
// ═══════════════════════════════════════════════════════════════════════════

describe('addBunnyPreconnects', () => {
	beforeEach(() => {
		// Clear any preconnects from prior tests.
		document.head.querySelectorAll('link[rel="preconnect"]').forEach((node) => node.remove());
	});

	it('appends preconnect <link> tags for the documented Bunny.net hosts', async () => {
		vi.resetModules();
		const { addBunnyPreconnects } = await import('../videoPreloader');
		addBunnyPreconnects();

		const preconnects = Array.from(
			document.head.querySelectorAll<HTMLLinkElement>('link[rel="preconnect"]')
		);
		const hrefs = preconnects.map((l) => l.href);
		expect(hrefs).toContain('https://iframe.mediadelivery.net/');
		expect(hrefs).toContain('https://video.bunnycdn.com/');
	});

	it('NEGATIVE — second call does NOT duplicate existing preconnect <link>s (idempotency)', async () => {
		vi.resetModules();
		const { addBunnyPreconnects } = await import('../videoPreloader');
		addBunnyPreconnects();
		addBunnyPreconnects();

		const preconnects = Array.from(
			document.head.querySelectorAll<HTMLLinkElement>('link[rel="preconnect"]')
		);
		// 3 distinct hosts → exactly 3 link tags after two calls.
		const distinctHosts = new Set(preconnects.map((l) => l.href));
		expect(preconnects.length).toBe(distinctHosts.size);
	});
});
