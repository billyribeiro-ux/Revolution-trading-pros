<script lang="ts">
	interface Coupon {
		id: number;
		code: string;
		amount: string;
		description: string;
		expiryDate: string;
		usageCount: number;
		usageLimit: number;
	}

	interface PageData {
		coupons: Coupon[];
	}

	let { data }: { data: PageData } = $props();

	const coupons = $derived(data.coupons || []);

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function isExpired(dateString: string): boolean {
		return new Date(dateString) < new Date();
	}
</script>

<svelte:head>
	<title>Coupons - Revolution Trading Pros</title>
</svelte:head>

<div class="woocommerce">
	<div class="woocommerce-MyAccount-content">
		<div class="woocommerce-notices-wrapper"></div>
		
		<h2 class="section-title">My Coupons</h2>

		{#if coupons.length === 0}
			<div class="woocommerce-message woocommerce-message--info">
				<p>You have no available coupons.</p>
			</div>
		{:else}
			<div class="coupons-grid">
				{#each coupons as coupon (coupon.id)}
					<div class="coupon-card" class:expired={isExpired(coupon.expiryDate)}>
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
							{#if coupon.description}
								<p class="coupon-description">{coupon.description}</p>
							{/if}
							
							<div class="coupon-meta">
								{#if coupon.expiryDate}
									<div class="coupon-expiry">
										<i class="fa fa-calendar"></i>
										<span>
											{#if isExpired(coupon.expiryDate)}
												Expired: {formatDate(coupon.expiryDate)}
											{:else}
												Expires: {formatDate(coupon.expiryDate)}
											{/if}
										</span>
									</div>
								{/if}
								
								{#if coupon.usageLimit}
									<div class="coupon-usage">
										<i class="fa fa-check-circle"></i>
										<span>Used {coupon.usageCount} of {coupon.usageLimit} times</span>
									</div>
								{/if}
							</div>
						</div>
						
						<div class="coupon-footer">
							{#if !isExpired(coupon.expiryDate) && coupon.usageCount < coupon.usageLimit}
								<button class="btn btn-primary btn-sm" onclick={() => navigator.clipboard.writeText(coupon.code)}>
									<i class="fa fa-copy"></i> Copy Code
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

<style>
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
		background: linear-gradient(135deg, #143E59 0%, #0f2f43 100%);
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
		background: #143E59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0f2f43;
	}

	.btn-sm {
		padding: 8px 16px;
		font-size: 13px;
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
		border-left: 4px solid #143E59;
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
