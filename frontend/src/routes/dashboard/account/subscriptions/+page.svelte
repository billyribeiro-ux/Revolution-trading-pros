<script lang="ts">
	/**
	 * My Subscriptions - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT 11+ Principal Engineer Grade - December 2025
	 * Real API integration with cancel functionality
	 *
	 * @version 3.0.0 - Production Ready
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { authStore } from '$lib/stores/auth';
	import { LoadingState } from '$lib/components/dashboard';

	// API Configuration
	const isDev = import.meta.env.DEV;
	const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.fly.dev/api';
	const API_BASE = browser
		? isDev
			? '/api'
			: (import.meta.env['VITE_API_URL'] || PRODUCTION_API_URL)
		: '/api';

	// Types
	interface Subscription {
		id: number;
		plan_id: number;
		plan_name: string;
		status: string;
		starts_at: string;
		expires_at: string | null;
		cancelled_at: string | null;
		cancel_at_period_end: boolean;
		current_period_start: string | null;
		current_period_end: string | null;
		stripe_subscription_id: string | null;
		billing_cycle: string;
		price: number;
		currency: string;
	}

	// State using Svelte 5 runes
	let subscriptions = $state<Subscription[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let cancellingId = $state<number | null>(null);

	// Fetch subscriptions from API
	async function fetchSubscriptions(): Promise<void> {
		if (!browser) return;

		isLoading = true;
		error = null;

		try {
			const token = authStore.getToken();

			const response = await fetch(`${API_BASE}/subscriptions/my`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include'
			});

			if (!response.ok) {
				if (response.status === 401) {
					error = 'Please log in to view your subscriptions';
					return;
				}
				throw new Error('Failed to fetch subscriptions');
			}

			const data = await response.json();
			subscriptions = data.subscriptions || data || [];
		} catch (e) {
			console.error('Error fetching subscriptions:', e);
			error = e instanceof Error ? e.message : 'Failed to load subscriptions';
		} finally {
			isLoading = false;
		}
	}

	// Cancel subscription
	async function cancelSubscription(id: number, immediately: boolean = false): Promise<void> {
		if (!confirm(`Are you sure you want to cancel this subscription${immediately ? ' immediately' : ' at the end of the billing period'}?`)) {
			return;
		}

		cancellingId = id;

		try {
			const token = authStore.getToken();

			const response = await fetch(`${API_BASE}/api/subscriptions/${id}/cancel`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					...(token ? { Authorization: `Bearer ${token}` } : {})
				},
				credentials: 'include',
				body: JSON.stringify({ immediately })
			});

			if (!response.ok) {
				const err = await response.json();
				throw new Error(err.message || 'Failed to cancel subscription');
			}

			// Refresh subscriptions
			await fetchSubscriptions();
		} catch (e) {
			console.error('Error cancelling subscription:', e);
			alert(e instanceof Error ? e.message : 'Failed to cancel subscription');
		} finally {
			cancellingId = null;
		}
	}

	// Format date for display
	function formatDate(dateString: string | null): string {
		if (!dateString) return '-';
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	// Format currency
	function formatCurrency(amount: number, currency: string = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	// Get status display info
	function getStatusInfo(sub: Subscription): { label: string; class: string } {
		if (sub.status === 'cancelled') {
			return { label: 'Cancelled', class: 'label--error' };
		}
		if (sub.cancel_at_period_end) {
			return { label: 'Pending Cancellation', class: 'label--info' };
		}
		if (sub.status === 'past_due') {
			return { label: 'Past Due', class: 'label--warning' };
		}
		if (sub.status === 'active') {
			return { label: 'Active', class: 'label--success' };
		}
		return { label: sub.status, class: '' };
	}

	// Get billing interval display
	function getBillingInterval(cycle: string): string {
		switch (cycle?.toLowerCase()) {
			case 'monthly':
				return '/ month';
			case 'quarterly':
				return '/ quarter';
			case 'yearly':
			case 'annual':
				return '/ year';
			default:
				return '';
		}
	}

	onMount(() => {
		fetchSubscriptions();
	});
</script>

<svelte:head>
	<title>My Subscriptions - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- Subscriptions Page - Real API Integration -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">My Subscriptions</h2>

			{#if isLoading}
				<LoadingState message="Loading your subscriptions..." />
			{:else if error}
				<div class="error-state">
					<p>{error}</p>
					<button class="btn btn-primary" onclick={() => fetchSubscriptions()}>
						Try Again
					</button>
				</div>
			{:else if subscriptions.length === 0}
				<div class="empty-state">
					<div class="empty-icon">📋</div>
					<p>No active subscriptions found.</p>
					<a href="/pricing" class="btn btn-primary">Browse Memberships</a>
				</div>
			{:else}
				<div class="woocommerce_account_subscriptions">
					<table class="table">
						<thead>
							<tr>
								<th class="col-xs-3"><span class="nobr">Subscription</span></th>
								<th class="col-xs-3"><span class="nobr">Status</span></th>
								<th class="col-xs-2"><span class="nobr">Plan</span></th>
								<th class="col-xs-2"><span class="nobr">Next Payment</span></th>
								<th class="col-xs-2"><span class="nobr">Total</span></th>
								<th class="col-xs-2">&nbsp;</th>
							</tr>
						</thead>
						<tbody class="u--font-size-sm">
							{#each subscriptions as sub (sub.id)}
								{@const statusInfo = getStatusInfo(sub)}
								<tr class="order">
									<td class="col-xs-3" data-title="ID">
										<a href="/dashboard/account/view-subscription/{sub.id}">
											#{sub.id}
										</a>
									</td>
									<td class="col-xs-3" data-title="Status">
										<span class="label {statusInfo.class}">
											{statusInfo.label}
										</span>
									</td>
									<td class="col-xs-2" data-title="Plan">
										<span>{sub.plan_name}</span>
									</td>
									<td class="col-xs-2" data-title="Next Payment">
										{#if sub.status === 'active' && !sub.cancel_at_period_end}
											{formatDate(sub.current_period_end)}
										{:else}
											-
										{/if}
									</td>
									<td class="col-xs-2" data-title="Total">
										<span class="woocommerce-Price-amount amount">
											{formatCurrency(sub.price, sub.currency)}
										</span>
										{getBillingInterval(sub.billing_cycle)}
									</td>
									<td class="col-xs-2 text-right actions-cell">
										<a href="/dashboard/account/view-subscription/{sub.id}" class="btn btn-default btn-xs">
											View
										</a>
										{#if sub.status === 'active' && !sub.cancel_at_period_end}
											<button
												class="btn btn-danger btn-xs"
												onclick={() => cancelSubscription(sub.id, false)}
												disabled={cancellingId === sub.id}
											>
												{cancellingId === sub.id ? 'Cancelling...' : 'Cancel'}
											</button>
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Apple ICT 11+ Grade Styles - Production Ready
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #d32f2f;
		background: #ffebee;
		border-radius: 4px;
	}

	.error-state p {
		margin-bottom: 16px;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
		background: #f9f9f9;
		border-radius: 4px;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.empty-state p {
		margin-bottom: 20px;
		font-size: 16px;
	}

	/* Table */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
	}

	.table thead {
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.table th {
		padding: 15px 15px;
		font-size: 12px;
		font-weight: 700;
		color: #333;
		text-align: left;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.table td {
		padding: 15px 15px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
		vertical-align: middle;
	}

	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.table tbody tr:hover {
		background: #f9f9f9;
	}

	/* Links */
	.table td a:not(.btn) {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.table td a:not(.btn):hover {
		color: #0e6ac4;
		text-decoration: underline;
	}

	/* Status Labels */
	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	.label--success {
		background-color: #d4edda;
		color: #155724;
	}

	.label--info {
		background-color: #d1ecf1;
		color: #0c5460;
	}

	.label--warning {
		background-color: #fff3cd;
		color: #856404;
	}

	.label--danger,
	.label--error {
		background-color: #f8d7da;
		color: #721c24;
	}

	/* Price */
	.woocommerce-Price-amount {
		font-weight: 600;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.15s ease;
		border: none;
		cursor: pointer;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #077a9e;
	}

	.btn-default {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
		border-color: #999;
	}

	.btn-danger {
		background: #dc3545;
		color: #fff;
	}

	.btn-danger:hover {
		background: #c82333;
	}

	.btn-danger:disabled {
		background: #e9a4ab;
		cursor: not-allowed;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.text-right {
		text-align: right;
	}

	.actions-cell {
		display: flex;
		gap: 8px;
		justify-content: flex-end;
	}

	@media (max-width: 992px) {
		.table {
			display: block;
			overflow-x: auto;
		}

		.actions-cell {
			flex-direction: column;
			gap: 4px;
		}
	}
</style>
