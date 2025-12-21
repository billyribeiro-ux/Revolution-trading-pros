<script lang="ts">
	/**
	 * Dashboard - My Subscriptions Page
	 * Shows user's active subscriptions with management options
	 * WordPress Revolution Trading Exact Match
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authStore } from '$lib/stores/auth';
	import {
		IconCreditCard,
		IconCalendar,
		IconRefresh,
		IconPlayerPause,
		IconPlayerPlay,
		IconX,
		IconChevronRight,
		IconAlertCircle
	} from '$lib/icons';

	// Redirect if not authenticated
	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/subscriptions', { replaceState: true });
		}
	});

	type SubscriptionStatus = 'active' | 'paused' | 'cancelled' | 'expired';

	interface Subscription {
		id: string;
		name: string;
		plan: string;
		status: SubscriptionStatus;
		price: number;
		billingCycle: 'monthly' | 'quarterly' | 'annual';
		nextBillingDate: string;
		startDate: string;
		paymentMethod: string;
	}

	// Sample subscriptions (would come from API)
	const subscriptions: Subscription[] = [
		{
			id: 'sub_1',
			name: 'Mastering the Trade',
			plan: 'Monthly',
			status: 'active',
			price: 497,
			billingCycle: 'monthly',
			nextBillingDate: 'January 5, 2026',
			startDate: 'November 5, 2025',
			paymentMethod: 'Visa ending in 4242'
		},
		{
			id: 'sub_2',
			name: 'Small Accounts Room',
			plan: 'Monthly',
			status: 'active',
			price: 197,
			billingCycle: 'monthly',
			nextBillingDate: 'January 1, 2026',
			startDate: 'October 1, 2025',
			paymentMethod: 'Visa ending in 4242'
		},
		{
			id: 'sub_3',
			name: 'Explosive Swings Alerts',
			plan: 'Annual',
			status: 'active',
			price: 927,
			billingCycle: 'annual',
			nextBillingDate: 'November 15, 2026',
			startDate: 'November 15, 2025',
			paymentMethod: 'PayPal'
		}
	];

	function getStatusInfo(status: SubscriptionStatus) {
		const statusMap = {
			active: { label: 'Active', class: 'status--active' },
			paused: { label: 'Paused', class: 'status--paused' },
			cancelled: { label: 'Cancelled', class: 'status--cancelled' },
			expired: { label: 'Expired', class: 'status--expired' }
		};
		return statusMap[status];
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function getBillingLabel(cycle: string, price: number): string {
		const formatted = formatCurrency(price);
		switch (cycle) {
			case 'monthly': return `${formatted}/month`;
			case 'quarterly': return `${formatted}/quarter`;
			case 'annual': return `${formatted}/year`;
			default: return formatted;
		}
	}
</script>

<svelte:head>
	<title>My Subscriptions | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if $isAuthenticated}
	<!-- Dashboard Header -->
	<header class="dashboard__header">
		<div class="dashboard__header-content">
			<h1 class="dashboard__page-title">Subscriptions</h1>
			<p class="dashboard__page-subtitle">Manage your active subscriptions and billing</p>
		</div>
	</header>

	<!-- Dashboard Content -->
	<div class="dashboard__content">
		<div class="dashboard__content-main">
			{#if subscriptions.length > 0}
				<!-- Subscriptions List -->
				<div class="subscriptions-list">
					{#each subscriptions as sub (sub.id)}
						{@const statusInfo = getStatusInfo(sub.status)}
						<div class="subscription-card">
							<div class="subscription-card__header">
								<div class="subscription-info">
									<h3 class="subscription-name">{sub.name}</h3>
									<span class="subscription-plan">{sub.plan}</span>
								</div>
								<span class="subscription-status {statusInfo.class}">
									{statusInfo.label}
								</span>
							</div>

							<div class="subscription-card__body">
								<div class="subscription-details">
									<div class="detail-item">
										<IconCreditCard size={18} />
										<div>
											<span class="detail-label">Price</span>
											<span class="detail-value">{getBillingLabel(sub.billingCycle, sub.price)}</span>
										</div>
									</div>
									<div class="detail-item">
										<IconCalendar size={18} />
										<div>
											<span class="detail-label">Next Billing</span>
											<span class="detail-value">{sub.nextBillingDate}</span>
										</div>
									</div>
									<div class="detail-item">
										<IconRefresh size={18} />
										<div>
											<span class="detail-label">Payment Method</span>
											<span class="detail-value">{sub.paymentMethod}</span>
										</div>
									</div>
								</div>
							</div>

							<div class="subscription-card__actions">
								{#if sub.status === 'active'}
									<button class="action-btn action-btn--secondary">
										<IconPlayerPause size={16} />
										Pause
									</button>
									<button class="action-btn action-btn--danger">
										<IconX size={16} />
										Cancel
									</button>
								{:else if sub.status === 'paused'}
									<button class="action-btn action-btn--primary">
										<IconPlayerPlay size={16} />
										Resume
									</button>
								{/if}
								<button class="action-btn action-btn--link">
									View Details
									<IconChevronRight size={16} />
								</button>
							</div>
						</div>
					{/each}
				</div>

				<!-- Summary Card -->
				<div class="summary-card">
					<div class="summary-card__header">
						<IconAlertCircle size={20} />
						<h4>Billing Summary</h4>
					</div>
					<div class="summary-card__content">
						<p>Your next payment of <strong>{formatCurrency(subscriptions.filter(s => s.status === 'active').reduce((acc, s) => {
							if (s.billingCycle === 'monthly') return acc + s.price;
							if (s.billingCycle === 'quarterly') return acc + s.price / 3;
							if (s.billingCycle === 'annual') return acc + s.price / 12;
							return acc;
						}, 0))}</strong> is scheduled for your next billing date.</p>
					</div>
				</div>
			{:else}
				<!-- Empty State -->
				<div class="empty-state">
					<IconCreditCard size={64} class="empty-icon" />
					<h3>No active subscriptions</h3>
					<p>Subscribe to a trading room or service to get started.</p>
					<a href="/live-trading-rooms" class="browse-btn">Browse Trading Rooms</a>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="loading-state">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		padding: 30px;
		border-bottom: 1px solid #ededed;
	}

	.dashboard__page-title {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 8px;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
	}

	.dashboard__page-subtitle {
		font-size: 15px;
		color: #666;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 800px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTIONS LIST
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscriptions-list {
		display: flex;
		flex-direction: column;
		gap: 20px;
		margin-bottom: 30px;
	}

	.subscription-card {
		background: #fff;
		border: 1px solid #ededed;
		border-radius: 8px;
		overflow: hidden;
	}

	.subscription-card__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 20px;
		background: #f8f8f8;
		border-bottom: 1px solid #ededed;
	}

	.subscription-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.subscription-name {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0;
	}

	.subscription-plan {
		font-size: 13px;
		color: #666;
	}

	.subscription-status {
		padding: 6px 12px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status--active {
		background: #dcfce7;
		color: #166534;
	}

	.status--paused {
		background: #fef3c7;
		color: #92400e;
	}

	.status--cancelled {
		background: #fee2e2;
		color: #991b1b;
	}

	.status--expired {
		background: #f3f4f6;
		color: #4b5563;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTION DETAILS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscription-card__body {
		padding: 20px;
	}

	.subscription-details {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
	}

	.detail-item {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		color: #666;
	}

	.detail-item div {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.detail-label {
		font-size: 12px;
		color: #999;
		text-transform: uppercase;
	}

	.detail-value {
		font-size: 14px;
		color: #333;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTION ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscription-card__actions {
		display: flex;
		gap: 12px;
		padding: 16px 20px;
		border-top: 1px solid #ededed;
		background: #fafafa;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn--primary {
		background: #0984ae;
		border: 1px solid #0984ae;
		color: #fff;
	}

	.action-btn--primary:hover {
		background: #076787;
		border-color: #076787;
	}

	.action-btn--secondary {
		background: #f4f4f4;
		border: 1px solid #dbdbdb;
		color: #333;
	}

	.action-btn--secondary:hover {
		background: #eee;
	}

	.action-btn--danger {
		background: #fff;
		border: 1px solid #ef4444;
		color: #ef4444;
	}

	.action-btn--danger:hover {
		background: #fef2f2;
	}

	.action-btn--link {
		background: transparent;
		border: none;
		color: #1e73be;
		margin-left: auto;
	}

	.action-btn--link:hover {
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUMMARY CARD
	   ═══════════════════════════════════════════════════════════════════════════ */

	.summary-card {
		background: #f0f9ff;
		border: 1px solid #bae6fd;
		border-radius: 8px;
		overflow: hidden;
	}

	.summary-card__header {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 16px 20px;
		background: #e0f2fe;
		border-bottom: 1px solid #bae6fd;
		color: #0369a1;
	}

	.summary-card__header h4 {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
	}

	.summary-card__content {
		padding: 16px 20px;
	}

	.summary-card__content p {
		margin: 0;
		color: #0369a1;
		font-size: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
	}

	:global(.empty-icon) {
		color: #ccc;
		margin-bottom: 20px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state p {
		color: #666;
		margin: 0 0 24px;
	}

	.browse-btn {
		display: inline-block;
		padding: 12px 24px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 6px;
		font-weight: 600;
		transition: background 0.15s ease;
	}

	.browse-btn:hover {
		background: #076787;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 20px;
		}

		.dashboard__page-title {
			font-size: 26px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.subscription-details {
			grid-template-columns: 1fr;
			gap: 16px;
		}

		.subscription-card__actions {
			flex-wrap: wrap;
		}

		.action-btn--link {
			margin-left: 0;
			width: 100%;
			justify-content: center;
		}
	}
</style>
