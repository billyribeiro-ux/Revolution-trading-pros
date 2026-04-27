<!--
	URL: /admin/analytics/funnels/create

	Stub create page for funnels.

	PRINCIPAL-2026-04-26 (audit 08-analytics §P3-6):
	The empty-state CTAs at `/admin/analytics/+page.svelte:412,426` linked here
	but the route did not exist (404 on click). The actual create flow lives
	inside the modal at `/admin/analytics/funnels/+page.svelte` (`showCreateModal`).
	Per the project's CREATE-not-DELETE rule, we build the missing route as a
	thin redirect into the funnels list page with `?create=1` so the existing
	modal can open itself.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';

	onMount(() => {
		if (!browser) return;
		// replaceState so the back button doesn't trap users in a redirect loop.
		goto('/admin/analytics/funnels?create=1', { replaceState: true });
	});
</script>

<svelte:head>
	<title>Create Funnel | Analytics</title>
	<!-- Meta-refresh as a no-JS fallback. Fires after 0s. -->
	<meta http-equiv="refresh" content="0; url=/admin/analytics/funnels?create=1" />
</svelte:head>

<div class="redirect-shell">
	<p>Redirecting to <a href="/admin/analytics/funnels?create=1">funnel creator</a>…</p>
</div>

<style>
	.redirect-shell {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 40vh;
		padding: 2rem;
		color: var(--admin-text-muted, #888);
		font-size: 0.875rem;
	}
</style>
