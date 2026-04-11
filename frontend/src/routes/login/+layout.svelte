<script lang="ts">
	/**
	 * Login Layout - Standalone Auth Page Layout
	 *
	 * Removes navbar/footer for clean auth experience.
	 * Theme-aware background for light/dark modes.
	 *
	 * @version 2.1.0 - Svelte 5.55+ sweep (April 2026)
	 */
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<div class="auth-page">
	{@render children()}
</div>

<style>
	.auth-page {
		background: var(--color-rtp-bg);
	}

	/* Ensure proper dark/light theme handling */
	:global(html.light) .auth-page,
	:global(body.light) .auth-page {
		background: #f5f5f7;
	}
</style>
