<script lang="ts">
	/**
	 * LazySection - ICT11+ Apple-Grade Lazy Loading Component
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Delays rendering of below-the-fold sections until they're near viewport
	 * Reduces initial bundle size and improves Time to Interactive (TTI)
	 *
	 * ICT11+ Fixes (December 2025):
	 * - Uses $effect() for IntersectionObserver with cleanup return function
	 * - Removed aggressive CSS containment that broke observer
	 * - Added fallback timeout for browsers with IO issues
	 * - SSR-safe with proper hydration handling via browser guard
	 * - PRODUCTION FIX: Always render content on SSR for SEO, lazy load animations only
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import type { Snippet } from 'svelte';
	import { browser } from '$app/environment';

	interface Props {
		/** Distance from viewport to start loading */
		rootMargin?: string;
		/** Intersection ratio to trigger (0-1) */
		threshold?: number;
		/** Fallback timeout in ms (0 = disabled) */
		fallbackTimeout?: number;
		/** Minimum placeholder height */
		placeholderHeight?: string;
		/** Children snippet */
		children?: Snippet;
	}

	let {
		rootMargin = '200px',
		threshold = 0,
		fallbackTimeout = 2000,
		placeholderHeight: _placeholderHeight = '400px',
		children
	}: Props = $props();

	// ICT11+ PRODUCTION FIX: Always start visible for SSR/SEO
	// Content is always rendered, only animations are deferred
	let hasAnimated = $state(false);
	let container = $state<HTMLElement | null>(null);
	let observer: IntersectionObserver | null = null;
	let fallbackTimer: ReturnType<typeof setTimeout> | null = null;

	// Svelte 5 $effect() for lifecycle management with cleanup
	$effect(() => {
		if (!browser) return;

		let rafId: number | null = null;

		// Use requestAnimationFrame to ensure DOM is ready
		rafId = requestAnimationFrame(() => {
			if (!container) {
				hasAnimated = true;
				return;
			}

			// Check if element is already in viewport - trigger animations immediately
			const rect = container.getBoundingClientRect();
			const margin = parseInt(rootMargin) || 200;
			const viewportHeight = window.innerHeight || document.documentElement.clientHeight;

			if (rect.top < viewportHeight + margin) {
				hasAnimated = true;
				return;
			}

			// Create IntersectionObserver for animation triggering
			observer = new IntersectionObserver(
				(entries) => {
					const entry = entries[0];
					if (entry?.isIntersecting && !hasAnimated) {
						hasAnimated = true;
						observer?.disconnect();
						observer = null;
						if (fallbackTimer) {
							clearTimeout(fallbackTimer);
							fallbackTimer = null;
						}
					}
				},
				{
					rootMargin,
					threshold
				}
			);

			observer.observe(container);

			// Fallback timeout - trigger animations after timeout regardless
			if (fallbackTimeout > 0) {
				fallbackTimer = setTimeout(() => {
					if (!hasAnimated) {
						hasAnimated = true;
						observer?.disconnect();
						observer = null;
					}
				}, fallbackTimeout);
			}
		});

		// Cleanup function returned from $effect
		return () => {
			if (rafId !== null) {
				cancelAnimationFrame(rafId);
			}
			observer?.disconnect();
			observer = null;
			if (fallbackTimer) {
				clearTimeout(fallbackTimer);
				fallbackTimer = null;
			}
		};
	});
</script>

<div bind:this={container} class="lazy-section" data-animated={hasAnimated}>
	{@render children?.()}
</div>

<style>
	.lazy-section {
		/* ICT11+ Fix: Removed content-visibility as it breaks IntersectionObserver in some browsers */
		/* The lazy loading is handled by the component logic instead */
		display: block;
		width: 100%;
	}
</style>
