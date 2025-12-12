<script lang="ts">
	/**
	 * Login Layout - Standalone Auth Page Layout
	 *
	 * Removes navbar/footer for clean auth experience.
	 * Theme-aware background for light/dark modes.
	 */
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="auth-page">
	{@render children()}
</div>

<style>
	.auth-page {
		min-height: 100vh;
		background: var(--color-rtp-bg);
	}

	/* Ensure proper dark/light theme handling */
	:global(html.light) .auth-page,
	:global(body.light) .auth-page {
		background: #f5f5f7;
	}
</style>
