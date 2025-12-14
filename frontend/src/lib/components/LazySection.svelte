<script lang="ts">
	/**
	 * LazySection - ICT11+ Apple-Grade Lazy Loading Component
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Delays rendering of below-the-fold sections until they're near viewport
	 * Reduces initial bundle size and improves Time to Interactive (TTI)
	 *
	 * ICT11+ Fixes (December 2025):
	 * - Uses $effect for reliable IntersectionObserver setup
	 * - Removed aggressive CSS containment that broke observer
	 * - Added fallback timeout for browsers with IO issues
	 * - SSR-safe with proper hydration handling
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, tick } from 'svelte';
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
		fallbackTimeout = 3000,
		placeholderHeight = '400px',
		children
	}: Props = $props();

	let isVisible = $state(false);
	let container = $state<HTMLElement | null>(null);
	let observer: IntersectionObserver | null = null;
	let fallbackTimer: ReturnType<typeof setTimeout> | null = null;

	// ICT11+ Fix: Use $effect for reliable observer setup after binding
	$effect(() => {
		if (!browser || !container || isVisible) return;

		// Clean up any existing observer
		observer?.disconnect();
		if (fallbackTimer) clearTimeout(fallbackTimer);

		// Create new IntersectionObserver
		observer = new IntersectionObserver(
			(entries) => {
				const entry = entries[0];
				if (entry?.isIntersecting) {
					isVisible = true;
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

		// ICT11+ Fix: Fallback timeout for edge cases where IO doesn't fire
		if (fallbackTimeout > 0) {
			fallbackTimer = setTimeout(() => {
				if (!isVisible) {
					isVisible = true;
					observer?.disconnect();
					observer = null;
				}
			}, fallbackTimeout);
		}

		// Cleanup function
		return () => {
			observer?.disconnect();
			observer = null;
			if (fallbackTimer) {
				clearTimeout(fallbackTimer);
				fallbackTimer = null;
			}
		};
	});

	// ICT11+ Fix: Immediate visibility check for elements already in viewport
	onMount(async () => {
		if (!browser) return;

		// Wait for binding to complete
		await tick();

		// Check if element is already in viewport (handles fast scrollers)
		if (container && !isVisible) {
			const rect = container.getBoundingClientRect();
			const margin = parseInt(rootMargin) || 200;
			const viewportHeight = window.innerHeight || document.documentElement.clientHeight;

			// Element is in or near viewport
			if (rect.top < viewportHeight + margin) {
				isVisible = true;
			}
		}
	});
</script>

<div bind:this={container} class="lazy-section">
	{#if isVisible}
		{@render children?.()}
	{:else}
		<!-- Placeholder maintains layout stability without hiding from observers -->
		<div
			class="lazy-placeholder"
			style="min-height: {placeholderHeight};"
			aria-hidden="true"
		></div>
	{/if}
</div>

<style>
	.lazy-section {
		/* ICT11+ Fix: Use content-visibility for performance without breaking IO */
		content-visibility: auto;
		contain-intrinsic-size: auto 400px;
	}

	.lazy-placeholder {
		/* ICT11+ Fix: Use transparent background instead of opacity:0 */
		/* This ensures the element takes up space and triggers IntersectionObserver */
		background: transparent;
		pointer-events: none;
	}
</style>
