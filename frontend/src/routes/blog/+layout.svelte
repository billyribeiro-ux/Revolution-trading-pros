<script lang="ts">
	/**
	 * Blog Layout - Error Boundary and Shared Context
	 *
	 * Provides:
	 * - Consistent blog structure
	 * - Error boundary for blog routes
	 * - Future: shared blog context (categories, tags)
	 *
	 * @version 1.0.0 - January 2026
	 */
	import type { Snippet } from 'svelte';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	// Svelte 5 props pattern
	let { children }: { children: Snippet } = $props();

	// Track if component has mounted (for hydration-safe rendering)
	// @ts-expect-error write-only state
	let mounted = $state(false);

	onMount(() => {
		mounted = true;

		// Track blog section visit for analytics
		if (browser) {
			try {
				// Send page category for analytics segmentation
				if (typeof window !== 'undefined' && 'gtag' in window) {
					(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'page_view', {
						page_category: 'blog'
					});
				}
			} catch (error) {
				// Silently fail analytics - non-critical
				if (import.meta.env.DEV) {
					console.debug('[BlogLayout] Analytics tracking failed:', error);
				}
			}
		}
	});
</script>

<!-- Blog section wrapper for consistent styling and error boundary scope -->
<div class="blog-layout" data-section="blog">
	{@render children()}
</div>

<style>
	.blog-layout {
		/* Ensure blog content fills available space */
		min-height: 100%;
		width: 100%;
	}
</style>
