/**
 * Revolution Trading Pros - SSR/SSG Utilities
 * =============================================
 * Utilities for proper server-side rendering and hydration
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

import { browser } from '$app/environment';

/**
 * Safe localStorage access with SSR support
 */
export function safeLocalStorage<T>(key: string, fallback: T): T {
	if (!browser) return fallback;

	try {
		const item = localStorage.getItem(key);
		return item ? JSON.parse(item) : fallback;
	} catch {
		return fallback;
	}
}

/**
 * Set localStorage safely
 */
export function setLocalStorage<T>(key: string, value: T): void {
	if (!browser) return;

	try {
		localStorage.setItem(key, JSON.stringify(value));
	} catch (error) {
		console.error('Failed to set localStorage:', error);
	}
}

/**
 * Safe sessionStorage access with SSR support
 */
export function safeSessionStorage<T>(key: string, fallback: T): T {
	if (!browser) return fallback;

	try {
		const item = sessionStorage.getItem(key);
		return item ? JSON.parse(item) : fallback;
	} catch {
		return fallback;
	}
}

/**
 * Check if running on server
 */
export function isServer(): boolean {
	return !browser;
}

/**
 * Check if running on client
 */
export function isClient(): boolean {
	return browser;
}

/**
 * Run function only on client
 */
export function clientOnly<T>(fn: () => T, fallback?: T): T | undefined {
	if (browser) {
		return fn();
	}
	return fallback;
}

/**
 * Create a deferred promise that resolves after hydration
 */
export function afterHydration(): Promise<void> {
	if (!browser) {
		return Promise.resolve();
	}

	return new Promise((resolve) => {
		if (document.readyState === 'complete') {
			// Use requestIdleCallback for better performance
			if ('requestIdleCallback' in window) {
				(
					window as unknown as { requestIdleCallback: (cb: () => void) => void }
				).requestIdleCallback(() => resolve());
			} else {
				setTimeout(resolve, 0);
			}
		} else {
			window.addEventListener('load', () => {
				if ('requestIdleCallback' in window) {
					(
						window as unknown as { requestIdleCallback: (cb: () => void) => void }
					).requestIdleCallback(() => resolve());
				} else {
					setTimeout(resolve, 0);
				}
			});
		}
	});
}

/**
 * Safe window access
 */
export function safeWindow<T>(accessor: () => T, fallback: T): T {
	if (!browser) return fallback;

	try {
		return accessor();
	} catch {
		return fallback;
	}
}

/**
 * Preload critical resources
 */
export function preloadResource(href: string, as: 'image' | 'script' | 'style' | 'font'): void {
	if (!browser) return;

	const link = document.createElement('link');
	link.rel = 'preload';
	link.href = href;
	link.as = as;

	if (as === 'font') {
		link.crossOrigin = 'anonymous';
	}

	document.head.appendChild(link);
}

/**
 * Prefetch a page for faster navigation
 */
export function prefetchPage(href: string): void {
	if (!browser) return;

	const link = document.createElement('link');
	link.rel = 'prefetch';
	link.href = href;
	document.head.appendChild(link);
}

/**
 * Check if user prefers reduced motion
 */
export function prefersReducedMotion(): boolean {
	if (!browser) return false;

	return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Check if user prefers dark mode
 */
export function prefersDarkMode(): boolean {
	if (!browser) return true; // Default to dark

	return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

/**
 * Get device pixel ratio for responsive images
 */
export function getDevicePixelRatio(): number {
	if (!browser) return 1;
	return window.devicePixelRatio || 1;
}

/**
 * Check network connection quality
 */
export function getConnectionQuality(): 'slow' | 'medium' | 'fast' | 'unknown' {
	if (!browser) return 'unknown';

	const connection = (navigator as unknown as { connection?: { effectiveType?: string } })
		.connection;

	if (!connection) return 'unknown';

	switch (connection.effectiveType) {
		case 'slow-2g':
		case '2g':
			return 'slow';
		case '3g':
			return 'medium';
		case '4g':
			return 'fast';
		default:
			return 'unknown';
	}
}

/**
 * Lazy load a module on the client
 */
export async function lazyImport<T>(
	importFn: () => Promise<{ default: T }>,
	fallback?: T
): Promise<T | undefined> {
	if (!browser) return fallback;

	try {
		const module = await importFn();
		return module.default;
	} catch (error) {
		console.error('Failed to lazy load module:', error);
		return fallback;
	}
}

/**
 * Create intersection observer with SSR safety
 */
export function createIntersectionObserver(
	callback: IntersectionObserverCallback,
	options?: IntersectionObserverInit
): IntersectionObserver | null {
	if (!browser || !('IntersectionObserver' in window)) {
		return null;
	}

	return new IntersectionObserver(callback, options);
}

/**
 * Create resize observer with SSR safety
 */
export function createResizeObserver(callback: ResizeObserverCallback): ResizeObserver | null {
	if (!browser || !('ResizeObserver' in window)) {
		return null;
	}

	return new ResizeObserver(callback);
}

/**
 * Safe requestAnimationFrame
 */
export function safeRAF(callback: FrameRequestCallback): number {
	if (!browser) return 0;
	return requestAnimationFrame(callback);
}

/**
 * Safe cancelAnimationFrame
 */
export function safeCancelRAF(id: number): void {
	if (!browser) return;
	cancelAnimationFrame(id);
}
