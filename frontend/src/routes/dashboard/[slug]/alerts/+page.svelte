<script lang="ts">
	/**
	 * Membership Alerts Page - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Displays trade alerts for a specific membership.
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte/icons/trending-down';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import { getTradeAlerts, type TradeAlert, type AlertType } from '$lib/api/trade-alerts';

	let slug = $derived($page.params.slug!);
	let loading = $state(true);
	let alerts = $state<TradeAlert[]>([]);
	let activeFilter = $state<'all' | AlertType>('all');
	let error = $state<string | null>(null);

	let filteredAlerts = $derived(
		activeFilter === 'all'
			? alerts
			: alerts.filter((alert) => alert.type === activeFilter)
	);

	function formatTime(timestamp: string): string {
		const date = new Date(timestamp);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
		const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

		if (diffHours < 1) {
			const diffMins = Math.floor(diffMs / (1000 * 60));
			return `${diffMins}m ago`;
		} else if (diffHours < 24) {
			return `${diffHours}h ago`;
		} else if (diffDays < 7) {
			return `${diffDays}d ago`;
		} else {
			return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
		}
	}

	function formatPrice(price: number): string {
		return `$${price.toFixed(2)}`;
	}

	function getSlugName(s: string | undefined): string {
		if (!s) return '';
		return s.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
	}

	async function loadAlerts() {
		loading = true;
		error = null;
		try {
			const response = await getTradeAlerts(slug || '', {
				status: 'active',
				per_page: 50
			});
			alerts = response.alerts;
		} catch (err) {
			console.error('Failed to load alerts:', err);
			error = 'Failed to load trade alerts. Please try again.';
		} finally {
			loading = false;
		}
	}

	function refreshAlerts() {
		loadAlerts();
	}

	onMount(() => {
		loadAlerts();
	});
</script>

<svelte:head>
	<title>Alerts - {getSlugName(slug)} | Revolution Trading Pros</title>
</svelte:head>

<div class="alerts-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<a href="/dashboard/{slug || ''}" class="back-link">
				<IconArrowLeft size={18} />
				Back to {getSlugName(slug)}
			</a>
			<h1>
				<IconBell size={24} />
				Trade Alerts
			</h1>
		</div>
		<div class="header-right">
			<button class="btn-icon" onclick={refreshAlerts} disabled={loading}>
				<IconRefresh size={20} class={loading ? 'spinning' : ''} />
			</button>
		</div>
	</header>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="filter-group">
			<IconFilter size={16} />
			<button
				class="filter-btn"
				class:active={activeFilter === 'all'}
				onclick={() => activeFilter = 'all'}
			>
				All
			</button>
			<button
				class="filter-btn buy"
				class:active={activeFilter === 'buy'}
				onclick={() => activeFilter = 'buy'}
			>
				Buy Alerts
			</button>
			<button
				class="filter-btn sell"
				class:active={activeFilter === 'sell'}
				onclick={() => activeFilter = 'sell'}
			>
				Sell Alerts
			</button>
			<button
				class="filter-btn update"
				class:active={activeFilter === 'update'}
				onclick={() => activeFilter = 'update'}
			>
				Updates
			</button>
		</div>
	</div>

	<!-- Alerts List -->
	<div class="alerts-container">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading alerts...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>{error}</p>
				<button class="btn-retry" onclick={refreshAlerts}>Try Again</button>
			</div>
		{:else if filteredAlerts.length === 0}
			<div class="empty-state">
				<IconBell size={48} />
				<h3>No Alerts Found</h3>
				<p>
					{#if activeFilter === 'all'}
						No trade alerts have been posted yet.
					{:else}
						No {activeFilter} alerts found. Try changing the filter.
					{/if}
				</p>
			</div>
		{:else}
			<div class="alerts-list">
				{#each filteredAlerts as alert (alert.id)}
					<article class="alert-card {alert.type}">
						<div class="alert-header">
							<div class="alert-type-badge {alert.type}">
								{#if alert.type === 'buy'}
									<IconTrendingUp size={16} />
								{:else if alert.type === 'sell'}
									<IconTrendingDown size={16} />
								{:else}
									<IconBell size={16} />
								{/if}
								{alert.type.toUpperCase()}
							</div>
							<div class="alert-time">
								<IconClock size={14} />
								{formatTime(alert.created_at)}
							</div>
						</div>

						<div class="alert-symbol">{alert.symbol}</div>

						{#if alert.entry_price || alert.target_price || alert.stop_loss}
							<div class="alert-details">
								{#if alert.entry_price}
									<div class="detail-item">
										<span class="label">Entry</span>
										<span class="value">{formatPrice(alert.entry_price)}</span>
									</div>
								{/if}
								{#if alert.target_price}
									<div class="detail-item target">
										<span class="label">Target</span>
										<span class="value">{formatPrice(alert.target_price)}</span>
									</div>
								{/if}
								{#if alert.stop_loss}
									<div class="detail-item stop">
										<span class="label">Stop</span>
										<span class="value">{formatPrice(alert.stop_loss)}</span>
									</div>
								{/if}
							</div>
						{/if}

						{#if alert.notes}
							<div class="alert-notes">{alert.notes}</div>
						{/if}
					</article>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.alerts-page {
		min-height: 100vh;
		background: #0f172a;
		padding: 1.5rem 2rem;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #a5b4fc;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Filters */
	.filters-bar {
		margin-bottom: 1.5rem;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
	}

	.filter-group > :global(svg) {
		color: #64748b;
	}

	.filter-btn {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #f1f5f9;
	}

	.filter-btn.active {
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
	}

	.filter-btn.buy.active {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.filter-btn.sell.active {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.filter-btn.update.active {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	/* Loading & Empty */
	.loading-state,
	.empty-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		color: #64748b;
	}

	.empty-state p,
	.error-state p {
		margin-top: 16px;
		font-size: 16px;
	}

	.error-state {
		color: #ef4444;
	}

	.btn-retry {
		margin-top: 16px;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
	}

	.btn-retry:hover {
		background: #076787;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	/* Alerts List */
	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.alert-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
		transition: all 0.2s;
	}

	.alert-card:hover {
		border-color: rgba(148, 163, 184, 0.2);
	}

	.alert-card.buy {
		border-left: 3px solid #10b981;
	}

	.alert-card.sell {
		border-left: 3px solid #ef4444;
	}

	.alert-card.update {
		border-left: 3px solid #f59e0b;
	}

	.alert-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.alert-type-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.alert-type-badge.buy {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.alert-type-badge.sell {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.alert-type-badge.update {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.alert-time {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #64748b;
		font-size: 0.8125rem;
	}

	.alert-symbol {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.alert-details {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 0.75rem;
	}

	.detail-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.detail-item .label {
		font-size: 0.6875rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.detail-item .value {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.detail-item.target .value {
		color: #34d399;
	}

	.detail-item.stop .value {
		color: #f87171;
	}

	.alert-notes {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 8px;
		color: #cbd5e1;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	@media (max-width: 768px) {
		.alerts-page {
			padding: 1rem;
		}

		.filter-group {
			flex-wrap: wrap;
		}
	}
</style>
