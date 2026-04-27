<script lang="ts">
	/**
	 * Admin Dashboard — DEPRECATED route, redirects to /admin.
	 *
	 * FIX P0-1 (audits/admin-2026-04-26/01-shell-and-dashboard.md):
	 * Two parallel "Admin Dashboard" routes shipped simultaneously
	 * (`/admin` and `/admin/dashboard`) — both fetched the same overlapping
	 * endpoints (`/api/admin/connections/summary`, `/api/analytics/realtime`,
	 * `/api/payments/summary`, `/api/admin/rooms/stats`) on a 60s refresh
	 * interval, doubling load whenever any user landed on the orphan.
	 *
	 * The canonical dashboard is `/admin` (the "Analytics Dashboard" page
	 * with period selector and 1500-line layout). The sidebar's `Overview`
	 * link points there; nothing in the canonical nav points here. So:
	 *
	 *  - Per CREATE-not-DELETE rule, we DO NOT delete this file.
	 *  - We replace its body with a redirect (meta-refresh + onMount goto)
	 *    so any deep link or bookmark is forwarded to `/admin`, and the
	 *    body never mounts the duplicate fetcher.
	 *  - The previous implementation is preserved in git history; if a
	 *    future product decision re-splits the two surfaces, recover from
	 *    the prior commit.
	 *
	 * TODO(2026-04-26-audit): Remove this redirect entirely once external
	 * links to `/admin/dashboard` are confirmed gone (3 release cycles).
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';

	onMount(() => {
		if (!browser) return;
		// replaceState so the back button doesn't trap users in a redirect loop.
		goto('/admin', { replaceState: true });
	});
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
	<!-- Meta-refresh as a no-JS fallback. Fires after 0s. -->
	<meta http-equiv="refresh" content="0; url=/admin" />
</svelte:head>

<div class="redirect-shell">
	<p>Redirecting to <a href="/admin">Admin Dashboard</a>…</p>
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
