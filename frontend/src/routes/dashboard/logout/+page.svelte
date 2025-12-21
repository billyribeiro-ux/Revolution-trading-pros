<script lang="ts">
	/**
	 * Dashboard - Logout Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/logout
	 * Logs the user out and redirects to homepage
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { browser } from '$app/environment';
	import { authStore } from '$lib/stores/auth';

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			// Call logout and redirect to homepage
			authStore.logout('/');
		}
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Logging out... | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT - Simple loading state while logout processes
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="logout-page">
	<div class="logout-content">
		<p>Logging out...</p>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.logout-page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		background: #fff;
	}

	.logout-content {
		text-align: center;
	}

	.logout-content p {
		font-size: 16px;
		color: #666;
	}
</style>
