<script lang="ts">
	/**
	 * Coupons - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerMyCoupons reference
	 * Available coupons and store credits
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Sample coupons data
	const coupons: { code: string; amount: string; type: string; expires: string }[] = [];
</script>

<svelte:head>
	<title>Coupons - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerMyCoupons reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Available Coupons &amp; Store Credits</h2>
			<p class="section-description">List of coupons which are valid &amp; available for use. Click on the coupon to use it. The coupon discount will be visible only when at least one product is present in the cart.</p>

			{#if coupons.length > 0}
				<div class="sc-coupons-list">
					{#each coupons as coupon}
						<div class="sc-coupon">
							<div class="coupon-code">{coupon.code}</div>
							<div class="coupon-amount">{coupon.amount}</div>
							<div class="coupon-expires">Expires: {coupon.expires}</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="woocommerce-Message woocommerce-Message--info woocommerce-info">
					Sorry, No coupons available for you.
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerMyCoupons reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.section-description {
		font-size: 14px;
		color: #666;
		margin: 0 0 30px;
		line-height: 1.6;
	}

	/* Coupons List */
	.sc-coupons-list {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 2rem;
	}

	.sc-coupon {
		position: relative;
		overflow: hidden;
		font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		cursor: pointer;
		min-width: 160px;
		max-width: 300px;
		padding: 20px;
		background: #fff;
		border: 2px dashed #0984ae;
		border-radius: 8px;
		transition: all 0.15s ease;
	}

	.sc-coupon:hover {
		border-style: solid;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.coupon-code {
		font-size: 18px;
		font-weight: 700;
		color: #0984ae;
		margin-bottom: 8px;
		font-family: monospace;
	}

	.coupon-amount {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin-bottom: 4px;
	}

	.coupon-expires {
		font-size: 12px;
		color: #666;
	}

	/* WooCommerce Message - Exact from reference */
	.woocommerce-Message {
		padding: 20px 25px;
		margin: 0;
		border-radius: 8px;
		font-size: 14px;
		line-height: 1.5;
	}

	.woocommerce-Message--info,
	.woocommerce-info {
		background: #f0f8ff;
		border: 1px solid #b3d7ff;
		color: #31708f;
	}

	@media (max-width: 768px) {
		.sc-coupons-list {
			gap: 1rem;
		}

		.sc-coupon {
			min-width: 100%;
		}
	}
</style>
