<script lang="ts">
	/**
	 * Account Page - Redirect to Orders
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main account page that redirects to the orders section
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		goto('/dashboard/account/orders', { replaceState: true });
	});
</script>

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<p>Redirecting to your account...</p>
		</section>
	</div>
</div>
