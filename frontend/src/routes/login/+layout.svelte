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
	/* PRINCIPAL-2026-04-26: comment-out, verify, delete in follow-up.
	   `--color-rtp-bg` is undefined repo-wide (rtp-* token namespace removed
	   in earlier refactor). The inner LoginLayout component paints its own
	   background via `--auth-bg`. The body.light/html.light selectors never
	   match — no code applies those classes. Concealed dead style. */
	/*
	.auth-page {
		background: var(--color-rtp-bg);
	}
	:global(html.light) .auth-page,
	:global(body.light) .auth-page {
		background: #f5f5f7;
	}
	*/
</style>
