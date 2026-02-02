<script lang="ts">
	/**
	 * Login Layout - Standalone Auth Page Layout
	 *
	 * Removes navbar/footer for clean auth experience.
	 * Theme-aware background for light/dark modes.
	 *
	 * @version 2.0.0 - Svelte 5 January 2026
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let props: Props = $props();
</script>

<div class="auth-page">
	{@render props.children()}
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
