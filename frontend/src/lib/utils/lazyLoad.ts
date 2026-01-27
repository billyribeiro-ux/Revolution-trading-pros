/**
 * Lazy Loading Utilities
 * Performance-optimized component and module loading
 */

import { browser } from '$app/environment';
import type { ComponentType } from 'svelte';

/**
 * Lazy load a Svelte component with optional delay
 * @param importFn - Dynamic import function
 * @param delay - Optional delay in ms (default: 0)
 * @returns Promise resolving to component
 */
export async function lazyLoadComponent<T = ComponentType>(
	importFn: () => Promise<{ default: T }>,
	delay = 0
): Promise<T> {
	if (!browser) {
		// SSR: return immediately
		const module = await importFn();
		return module.default;
	}

	if (delay > 0) {
		await new Promise((resolve) => setTimeout(resolve, delay));
	}

	const module = await importFn();
	return module.default;
}

/**
 * Lazy load multiple components in parallel
 * @param importFns - Array of dynamic import functions
 * @returns Promise resolving to array of components
 */
export async function lazyLoadComponents<T = ComponentType>(
	importFns: Array<() => Promise<{ default: T }>>
): Promise<T[]> {
	const modules = await Promise.all(importFns.map((fn) => fn()));
	return modules.map((m) => m.default);
}

/**
 * Load component when idle (uses requestIdleCallback)
 * Falls back to setTimeout for Safari
 * @param importFn - Dynamic import function
 * @param timeout - Maximum wait time in ms (default: 2000)
 * @returns Promise resolving to component
 */
export function lazyLoadOnIdle<T = ComponentType>(
	importFn: () => Promise<{ default: T }>,
	timeout = 2000
): Promise<T> {
	if (!browser) {
		return importFn().then((m) => m.default);
	}

	return new Promise((resolve) => {
		if ('requestIdleCallback' in window) {
			requestIdleCallback(
				async () => {
					const module = await importFn();
					resolve(module.default);
				},
				{ timeout }
			);
		} else {
			// Fallback for Safari
			setTimeout(async () => {
				const module = await importFn();
				resolve(module.default);
			}, 100);
		}
	});
}

/**
 * Load component when it enters viewport (Intersection Observer)
 * @param element - Target element to observe
 * @param importFn - Dynamic import function
 * @param threshold - Intersection threshold (default: 0.1)
 * @returns Promise resolving to component
 */
export function lazyLoadOnVisible<T = ComponentType>(
	element: HTMLElement,
	importFn: () => Promise<{ default: T }>,
	threshold = 0.1
): Promise<T> {
	if (!browser) {
		return importFn().then((m) => m.default);
	}

	return new Promise((resolve) => {
		const observer = new IntersectionObserver(
			async (entries) => {
				if (entries[0]?.isIntersecting) {
					observer.disconnect();
					const module = await importFn();
					resolve(module.default);
				}
			},
			{ threshold }
		);

		observer.observe(element);
	});
}

/**
 * Preload a component without rendering it
 * Useful for prefetching on hover/focus
 * @param importFn - Dynamic import function
 */
export function preloadComponent(importFn: () => Promise<{ default: unknown }>): void {
	if (!browser) return;

	// Fire and forget - just trigger the import
	importFn().catch(() => {
		// Silently fail - preload is optional
	});
}

/**
 * Create a lazy load store for component state management
 */
export function createLazyLoadStore<T = ComponentType>() {
	let component: T | null = null;
	let loading = false;
	let error: Error | null = null;

	return {
		get component() {
			return component;
		},
		get loading() {
			return loading;
		},
		get error() {
			return error;
		},
		async load(importFn: () => Promise<{ default: T }>) {
			if (component) return component;
			if (loading) return null;

			loading = true;
			error = null;

			try {
				const module = await importFn();
				component = module.default;
				return component;
			} catch (e) {
				error = e as Error;
				return null;
			} finally {
				loading = false;
			}
		}
	};
}
