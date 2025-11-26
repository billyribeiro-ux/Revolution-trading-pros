<script lang="ts">
	/**
	 * LazySection - Google L11+ Lazy Loading Component
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Delays rendering of below-the-fold sections until they're near viewport
	 * Reduces initial bundle size and improves Time to Interactive (TTI)
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Props
	export let rootMargin = '200px'; // Start loading 200px before entering viewport
	export let threshold = 0.01;

	// State
	let isVisible = false;
	let container: HTMLElement;

	onMount(() => {
		if (!browser || !container) return;

		// Use IntersectionObserver for efficient viewport detection
		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0]?.isIntersecting) {
					isVisible = true;
					observer.disconnect(); // Stop observing once loaded
				}
			},
			{
				rootMargin,
				threshold
			}
		);

		observer.observe(container);

		return () => observer.disconnect();
	});
</script>

<div bind:this={container} class="lazy-section">
	{#if isVisible}
		<slot />
	{:else}
		<!-- Placeholder to maintain layout stability -->
		<div class="lazy-placeholder" style="min-height: 400px;" aria-hidden="true"></div>
	{/if}
</div>

<style>
	.lazy-section {
		/* Prevent layout shift */
		contain: layout style paint;
	}

	.lazy-placeholder {
		/* Invisible placeholder to prevent CLS */
		opacity: 0;
		pointer-events: none;
	}
</style>
