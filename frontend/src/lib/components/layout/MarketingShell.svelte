<script lang="ts">
	/**
	 * MarketingShell — owns marketing CSS and marketing chrome (NavBar).
	 * ══════════════════════════════════════════════════════════════════════════
	 * Imported by routes/+layout.svelte's marketing branch ONLY. The
	 * marketing.css import below is what makes marketing CSS independent of
	 * app.css — Vite code-splits this into the marketing chunk; admin /
	 * dashboard / cms / embed bundles do not load it.
	 * ══════════════════════════════════════════════════════════════════════════
	 */
	import '../../../marketing.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import ClientOnly from '$lib/components/ssr/ClientOnly.svelte';
	import { NavBar } from '$lib/components/nav';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		showAdminToolbar: boolean;
	}

	let props: Props = $props();
</script>

<div class={['marketing-shell', { 'has-admin-toolbar': props.showAdminToolbar }]}>
	<ClientOnly>
		{#if props.showAdminToolbar}
			<AdminToolbar />
		{/if}
	</ClientOnly>

	<NavBar />

	<main id="main-content" class="flex-1 min-w-0 overflow-x-clip">
		{@render props.children()}
	</main>
</div>
