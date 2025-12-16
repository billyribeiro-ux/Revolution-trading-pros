<script lang="ts">
	/**
	 * LazySection - ICT11+ Apple-Grade Lazy Loading Component
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Delays rendering of below-the-fold sections until they're near viewport
	 * Reduces initial bundle size and improves Time to Interactive (TTI)
	 *
	 * ICT11+ Fixes (December 2025):
	 * - Uses onMount for IntersectionObserver (NOT $effect - bindings not ready)
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

	// ICT11+ Fix: Start visible on SSR, let client handle lazy loading
	// This ensures content renders even if JS fails
	let isVisible = $state(!browser);
	let container = $state<HTMLElement | null>(null);
	let observer: IntersectionObserver | null = null;
	let fallbackTimer: ReturnType<typeof setTimeout> | null = null;

	onMount(() => {
		if (!browser) return;

		// Reset to false on client, then check visibility
		isVisible = false;

		// Use requestAnimationFrame to ensure DOM is ready
		requestAnimationFrame(() => {
			if (!container) {
				isVisible = true;
				return;
			}

			// Check if element is already in viewport
			const rect = container.getBoundingClientRect();
			const margin = parseInt(rootMargin) || 200;
			const viewportHeight = window.innerHeight || document.documentElement.clientHeight;

			if (rect.top < viewportHeight + margin) {
				isVisible = true;
				return;
			}

			// Create IntersectionObserver
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

			// Fallback timeout - show content after timeout regardless
			if (fallbackTimeout > 0) {
				fallbackTimer = setTimeout(() => {
					if (!isVisible) {
						isVisible = true;
						observer?.disconnect();
						observer = null;
					}
				}, fallbackTimeout);
			}
		});

		// Cleanup
		return () => {
			observer?.disconnect();
			observer = null;
			if (fallbackTimer) {
				clearTimeout(fallbackTimer);
				fallbackTimer = null;
			}
		};
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
		/* ICT11+ Fix: Removed content-visibility as it breaks IntersectionObserver in some browsers */
		/* The lazy loading is handled by the component logic instead */
		display: block;
		width: 100%;
	}

	.lazy-placeholder {
		/* ICT11+ Fix: Use transparent background instead of opacity:0 */
		/* This ensures the element takes up space and triggers IntersectionObserver */
		background: transparent;
		pointer-events: none;
	}
</style>
