<script lang="ts">
	/**
	 * Blog Layout - Error Boundary and Shared Context
	 *
	 * Provides:
	 * - Consistent blog structure
	 * - Error boundary for blog routes
	 * - Future: shared blog context (categories, tags)
	 *
	 * @version 1.1.0 - January 2026
	 */
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';

	// Svelte 5 props pattern with typed interface
	interface Props {
		children: Snippet;
	}
	let { children }: Props = $props();

	// Track if component has mounted (for hydration-safe rendering)
	let _mounted = $state(false);

	// onMount only runs client-side; browser guard is redundant here
	onMount(() => {
		_mounted = true;

		// Track blog section visit for analytics
		try {
			if ('gtag' in window) {
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
