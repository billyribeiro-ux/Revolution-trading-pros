<script lang="ts">
	interface Coupon {
		id: number;
		code: string;
		type: string;
		value: number;
		amount: string;
		description: string | null;
		display_name: string | null;
		expiry_date: string | null;
		min_purchase_amount: number;
		max_discount_amount: number | null;
		usage_count: number;
		usage_limit: number;
		is_expired: boolean;
	}

	interface PageData {
		coupons: Coupon[];
	}

	let props: { data: PageData } = $props();
	let data = $derived(props.data);

	const coupons = $derived(data.coupons || []);
	let copiedCode = $state<string | null>(null);

	function formatDate(dateString: string | null): string {
		if (!dateString) return 'No expiration';
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function isExpired(coupon: Coupon): boolean {
		if (!coupon.expiry_date) return false;
		return coupon.is_expired || new Date(coupon.expiry_date) < new Date();
	}

	function isAvailable(coupon: Coupon): boolean {
		if (isExpired(coupon)) return false;
		if (coupon.usage_limit > 0 && coupon.usage_count >= coupon.usage_limit) return false;
		return true;
	}

	async function copyCouponCode(code: string): Promise<void> {
		try {
			await navigator.clipboard.writeText(code);
			copiedCode = code;
			setTimeout(() => {
				copiedCode = null;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy code:', err);
		}
	}
</script>

<svelte:head>
	<title>Coupons - Revolution Trading Pros</title>
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<div class="coupons-card">
			<div class="woocommerce-notices-wrapper"></div>

			<h2 class="section-title">My Coupons</h2>

			{#if coupons.length === 0}
				<div class="woocommerce-message woocommerce-message--info">
					<p>You have no available coupons.</p>
				</div>
			{:else}
				<div class="coupons-grid">
					{#each coupons as coupon (coupon.id)}
						<div class="coupon-card" class:expired={isExpired(coupon)}>
							<div class="coupon-header">
								<div class="coupon-code">
									<span class="code-label">Code:</span>
									<span class="code-value">{coupon.code}</span>
								</div>
								<div class="coupon-amount">
									{coupon.amount}
								</div>
							</div>

							<div class="coupon-body">
								{#if coupon.description || coupon.display_name}
									<p class="coupon-description">{coupon.display_name || coupon.description}</p>
								{/if}

								{#if coupon.min_purchase_amount > 0}
									<p class="coupon-min-purchase">
										Minimum purchase: ${coupon.min_purchase_amount.toFixed(2)}
									</p>
								{/if}

								<div class="coupon-meta">
									{#if coupon.expiry_date}
										<div class="coupon-expiry">
											<i class="fa fa-calendar"></i>
											<span>
												{#if isExpired(coupon)}
													Expired: {formatDate(coupon.expiry_date)}
												{:else}
													Expires: {formatDate(coupon.expiry_date)}
												{/if}
											</span>
										</div>
									{/if}

									{#if coupon.usage_limit > 0}
										<div class="coupon-usage">
											<i class="fa fa-check-circle"></i>
											<span>Used {coupon.usage_count} of {coupon.usage_limit} times</span>
										</div>
									{/if}
								</div>
							</div>

							<div class="coupon-footer">
								{#if isAvailable(coupon)}
									<button
										class="btn btn-primary btn-sm"
										class:btn-success={copiedCode === coupon.code}
										onclick={() => copyCouponCode(coupon.code)}
									>
										{#if copiedCode === coupon.code}
											<i class="fa fa-check"></i> Copied!
										{:else}
											<i class="fa fa-copy"></i> Copy Code
										{/if}
									</button>
								{:else}
									<span class="coupon-status-expired">Not Available</span>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 28px;
		font-weight: 700;
		font-style: italic;
		color: #333333;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content */
	/* ICT11+ Fix: Removed min-height: calc(100vh - 60px) - let parent flex container handle height */
	.dashboard__content {
		background: #f5f5f5;
		padding: 40px 30px;
	}

	.dashboard__content-main {
		max-width: 1100px;
		margin: 0 auto;
	}

	/* Professional Card Container */
	.coupons-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

	.section-title {
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 30px;
		color: #333;
	}

	.coupons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
	}

	.coupon-card {
		background: #fff;
		border: 2px solid #e9ecef;
		border-radius: 12px;
		overflow: hidden;
		transition: all 0.3s ease;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.coupon-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	.coupon-card.expired {
		opacity: 0.6;
		border-color: #dc3545;
	}

	.coupon-header {
		background: linear-gradient(135deg, #143e59 0%, #0f2f43 100%);
		padding: 20px;
		color: #fff;
	}

	.coupon-code {
		margin-bottom: 12px;
	}

	.code-label {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 1px;
		opacity: 0.8;
		display: block;
		margin-bottom: 4px;
	}

	.code-value {
		font-size: 24px;
		font-weight: 700;
		font-family: 'Courier New', monospace;
		letter-spacing: 2px;
	}

	.coupon-amount {
		font-size: 32px;
		font-weight: 700;
		text-align: right;
	}

	.coupon-body {
		padding: 20px;
	}

	.coupon-description {
		font-size: 14px;
		color: #495057;
		margin-bottom: 16px;
		line-height: 1.6;
	}

	.coupon-meta {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.coupon-expiry,
	.coupon-usage {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: #6c757d;
	}

	.coupon-expiry i,
	.coupon-usage i {
		width: 16px;
		text-align: center;
	}

	.coupon-footer {
		padding: 16px 20px;
		border-top: 1px solid #e9ecef;
		background: #f8f9fa;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s ease;
		width: 100%;
		justify-content: center;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0f2f43;
	}

	.btn-success {
		background: #28a745;
		color: #fff;
	}

	.btn-success:hover {
		background: #218838;
	}

	.btn-sm {
		padding: 8px 16px;
		font-size: 13px;
	}

	.coupon-min-purchase {
		font-size: 13px;
		color: #6c757d;
		margin-bottom: 12px;
		font-style: italic;
	}

	.coupon-status-expired {
		display: block;
		text-align: center;
		color: #dc3545;
		font-weight: 600;
		font-size: 14px;
	}

	.woocommerce-message {
		padding: 16px 20px;
		background: #e7f3ff;
		border-left: 4px solid #143e59;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.woocommerce-message p {
		margin: 0;
		color: #495057;
		font-size: 14px;
	}

	@media (max-width: 768px) {
		.coupons-grid {
			grid-template-columns: 1fr;
		}

		.coupon-amount {
			font-size: 24px;
		}

		.code-value {
			font-size: 20px;
		}
	}
</style>
