<script lang="ts">
	/**
	 * Dashboard - My Coupons Page
	 * WordPress Revolution Trading Exact Match
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authStore } from '$lib/stores/auth';
	import { IconTicket, IconCopy, IconCheck, IconClock, IconGift } from '@tabler/icons-svelte';

	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/coupons', { replaceState: true });
		}
	});

	interface Coupon {
		id: string;
		code: string;
		description: string;
		discount: string;
		expiresAt?: string;
		usedAt?: string;
		status: 'available' | 'used' | 'expired';
		minPurchase?: number;
	}

	const coupons: Coupon[] = [
		{ id: 'c_1', code: 'WELCOME20', description: 'Welcome discount for new members', discount: '20% off', expiresAt: 'January 31, 2026', status: 'available' },
		{ id: 'c_2', code: 'HOLIDAY25', description: 'Holiday special discount', discount: '$25 off', expiresAt: 'December 31, 2025', status: 'available', minPurchase: 100 },
		{ id: 'c_3', code: 'LOYALTY10', description: 'Loyalty reward', discount: '10% off', usedAt: 'November 15, 2025', status: 'used' }
	];

	let copiedCode = $state<string | null>(null);

	async function copyCode(code: string) {
		await navigator.clipboard.writeText(code);
		copiedCode = code;
		setTimeout(() => copiedCode = null, 2000);
	}

	function getStatusInfo(status: Coupon['status']) {
		const map = {
			available: { label: 'Available', class: 'status--available' },
			used: { label: 'Used', class: 'status--used' },
			expired: { label: 'Expired', class: 'status--expired' }
		};
		return map[status];
	}
</script>

<svelte:head>
	<title>My Coupons | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if $isAuthenticated}
	<header class="dashboard__header">
		<h1 class="dashboard__page-title">Coupons</h1>
		<p class="dashboard__page-subtitle">View and manage your discount coupons</p>
	</header>

	<div class="dashboard__content">
		<div class="dashboard__content-main">
			{#if coupons.length > 0}
				<div class="coupons-list">
					{#each coupons as coupon (coupon.id)}
						{@const statusInfo = getStatusInfo(coupon.status)}
						<div class="coupon-card" class:is-used={coupon.status === 'used'} class:is-expired={coupon.status === 'expired'}>
							<div class="coupon-card__icon">
								<IconTicket size={28} />
							</div>
							<div class="coupon-card__info">
								<div class="coupon-card__header">
									<span class="coupon-code">{coupon.code}</span>
									<span class="coupon-discount">{coupon.discount}</span>
								</div>
								<p class="coupon-description">{coupon.description}</p>
								<div class="coupon-meta">
									{#if coupon.status === 'available' && coupon.expiresAt}
										<span class="coupon-expiry">
											<IconClock size={14} />
											Expires: {coupon.expiresAt}
										</span>
									{:else if coupon.status === 'used' && coupon.usedAt}
										<span class="coupon-used">Used on {coupon.usedAt}</span>
									{/if}
									{#if coupon.minPurchase}
										<span class="coupon-min">Min. purchase: ${coupon.minPurchase}</span>
									{/if}
								</div>
							</div>
							<div class="coupon-card__actions">
								<span class="coupon-status {statusInfo.class}">{statusInfo.label}</span>
								{#if coupon.status === 'available'}
									<button class="copy-btn" onclick={() => copyCode(coupon.code)}>
										{#if copiedCode === coupon.code}
											<IconCheck size={16} />
											Copied!
										{:else}
											<IconCopy size={16} />
											Copy Code
										{/if}
									</button>
								{/if}
							</div>
						</div>
					{/each}
				</div>

				<!-- Redeem Code Section -->
				<div class="redeem-section">
					<h3>
						<IconGift size={20} />
						Have a coupon code?
					</h3>
					<form class="redeem-form" onsubmit={(e) => e.preventDefault()}>
						<input type="text" placeholder="Enter coupon code" />
						<button type="submit">Apply</button>
					</form>
				</div>
			{:else}
				<div class="empty-state">
					<IconTicket size={64} class="empty-icon" />
					<h3>No coupons available</h3>
					<p>Check back later for special offers and discounts.</p>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="loading-state"><p>Redirecting to login...</p></div>
{/if}

<style>
	.dashboard__header { background: #fff; padding: 30px; border-bottom: 1px solid #ededed; }
	.dashboard__page-title { font-size: 32px; font-weight: 700; color: #333; margin: 0 0 8px; font-family: 'Open Sans Condensed', 'Open Sans', sans-serif; }
	.dashboard__page-subtitle { font-size: 15px; color: #666; margin: 0; }
	.dashboard__content { padding: 30px; background: #fff; min-height: 400px; }
	.dashboard__content-main { max-width: 800px; }

	.coupons-list { display: flex; flex-direction: column; gap: 16px; margin-bottom: 30px; }

	.coupon-card { display: flex; gap: 16px; padding: 20px; border: 2px dashed #0984ae; border-radius: 12px; background: linear-gradient(135deg, #f0f9ff 0%, #fff 100%); }
	.coupon-card.is-used, .coupon-card.is-expired { border-color: #dbdbdb; background: #f8f8f8; opacity: 0.7; }
	.coupon-card__icon { color: #0984ae; flex-shrink: 0; }
	.coupon-card.is-used .coupon-card__icon, .coupon-card.is-expired .coupon-card__icon { color: #999; }
	.coupon-card__info { flex: 1; }
	.coupon-card__header { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
	.coupon-code { font-family: monospace; font-size: 18px; font-weight: 700; color: #0984ae; background: #e0f2fe; padding: 4px 10px; border-radius: 4px; }
	.coupon-card.is-used .coupon-code, .coupon-card.is-expired .coupon-code { color: #666; background: #eee; }
	.coupon-discount { font-weight: 700; color: #166534; background: #dcfce7; padding: 4px 10px; border-radius: 4px; font-size: 14px; }
	.coupon-card.is-used .coupon-discount, .coupon-card.is-expired .coupon-discount { color: #666; background: #eee; }
	.coupon-description { margin: 0 0 8px; color: #333; font-size: 14px; }
	.coupon-meta { display: flex; gap: 16px; flex-wrap: wrap; }
	.coupon-expiry, .coupon-used, .coupon-min { display: inline-flex; align-items: center; gap: 4px; font-size: 12px; color: #666; }
	.coupon-card__actions { display: flex; flex-direction: column; align-items: flex-end; gap: 8px; }
	.coupon-status { padding: 4px 10px; border-radius: 20px; font-size: 11px; font-weight: 600; text-transform: uppercase; }
	.status--available { background: #dcfce7; color: #166534; }
	.status--used { background: #f3f4f6; color: #4b5563; }
	.status--expired { background: #fee2e2; color: #991b1b; }
	.copy-btn { display: inline-flex; align-items: center; gap: 6px; padding: 8px 14px; background: #0984ae; border: none; border-radius: 6px; color: #fff; font-size: 13px; font-weight: 500; cursor: pointer; transition: background 0.15s; }
	.copy-btn:hover { background: #076787; }

	.redeem-section { padding: 24px; background: #f8f8f8; border-radius: 12px; }
	.redeem-section h3 { display: flex; align-items: center; gap: 10px; margin: 0 0 16px; font-size: 16px; color: #333; }
	.redeem-form { display: flex; gap: 12px; }
	.redeem-form input { flex: 1; padding: 12px 16px; border: 1px solid #dbdbdb; border-radius: 6px; font-size: 14px; }
	.redeem-form input:focus { outline: none; border-color: #0984ae; }
	.redeem-form button { padding: 12px 24px; background: #0984ae; border: none; border-radius: 6px; color: #fff; font-weight: 600; cursor: pointer; }
	.redeem-form button:hover { background: #076787; }

	.empty-state { display: flex; flex-direction: column; align-items: center; padding: 60px 20px; text-align: center; }
	:global(.empty-icon) { color: #ccc; margin-bottom: 20px; }
	.empty-state h3 { font-size: 20px; color: #333; margin: 0 0 8px; }
	.empty-state p { color: #666; margin: 0; }

	.loading-state { display: flex; align-items: center; justify-content: center; min-height: 300px; color: #666; }

	@media (max-width: 768px) {
		.dashboard__header { padding: 20px; }
		.dashboard__page-title { font-size: 26px; }
		.dashboard__content { padding: 20px; }
		.coupon-card { flex-direction: column; }
		.coupon-card__actions { flex-direction: row; width: 100%; justify-content: space-between; }
		.redeem-form { flex-direction: column; }
	}
</style>
