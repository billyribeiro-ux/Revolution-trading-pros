<script lang="ts">
	/**
	 * Dashboard - Coupons Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/account/coupons
	 * Shows available coupons and store credits
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account/coupons', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Coupon {
		id: string;
		code: string;
		description: string;
		discount: string;
		expiresAt: string | null;
	}

	// Sample coupons data (empty for now as shown in screenshot)
	const coupons: Coupon[] = [];
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Coupons | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Info Box -->
		<div class="info-box">
			<p>
				List of coupons which are valid & available for use. Click on the coupon to use it.
				The coupon discount will be visible only when at least one product is present in the cart.
			</p>
		</div>

		<!-- Coupons List -->
		{#if coupons.length > 0}
			<div class="coupons-list">
				{#each coupons as coupon (coupon.id)}
					<button type="button" class="coupon-card" onclick={() => {}}>
						<div class="coupon-code">{coupon.code}</div>
						<div class="coupon-discount">{coupon.discount}</div>
						{#if coupon.description}
							<div class="coupon-description">{coupon.description}</div>
						{/if}
						{#if coupon.expiresAt}
							<div class="coupon-expires">Expires: {coupon.expiresAt}</div>
						{/if}
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   INFO BOX - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.info-box {
		background: #f8f9fa;
		border: 1px solid #e9ebed;
		border-radius: 4px;
		padding: 20px 24px;
		margin-bottom: 24px;
	}

	.info-box p {
		margin: 0;
		font-size: 14px;
		color: #333;
		line-height: 1.6;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COUPONS LIST
	   ═══════════════════════════════════════════════════════════════════════════ */

	.coupons-list {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
	}

	.coupon-card {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		min-width: 200px;
		max-width: 280px;
		padding: 20px;
		background: #fff;
		border: 2px dashed #ddd;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s ease;
		text-align: left;
	}

	.coupon-card:hover {
		border-color: #1e73be;
		border-style: solid;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.coupon-code {
		font-family: 'Courier New', monospace;
		font-size: 16px;
		font-weight: 700;
		color: #1e73be;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}

	.coupon-discount {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin-bottom: 8px;
	}

	.coupon-description {
		font-size: 13px;
		color: #666;
		line-height: 1.4;
		margin-bottom: 8px;
	}

	.coupon-expires {
		font-size: 12px;
		color: #999;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.info-box {
			padding: 16px 20px;
		}

		.coupons-list {
			gap: 16px;
		}

		.coupon-card {
			min-width: 100%;
			max-width: 100%;
		}
	}
</style>
