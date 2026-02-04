<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Trade Tracker - Explosive Swings
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Complete trade history with admin controls for closing trades
	 *
	 * @version 3.0.0 - Nuclear Refactor (ICT 7+ Standards)
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { browser } from '$app/environment';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import ClosePositionModal from '../components/ClosePositionModal.svelte';
	import { checkAdminStatus } from '$lib/utils/auth';
	import { fetchTrades, calculateStats } from './api';
	import type { Trade, TradeStats, FilterStatus } from './types';
	import type { ActivePosition } from '../types';

	// Components
	import StatsGrid from './components/StatsGrid.svelte';
	import FilterButtons from './components/FilterButtons.svelte';
	import TradesTable from './components/TradesTable.svelte';
	import EmptyState from './components/EmptyState.svelte';
	import ErrorState from './components/ErrorState.svelte';
	import TableSkeleton from './components/TableSkeleton.svelte';
	import ExportMenu from '../components/ExportMenu.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let filterStatus = $state<FilterStatus>('all');
	let trades = $state<Trade[]>([]);
	let stats = $state<TradeStats | null>(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isAdmin = $state(false);

	// Modal state
	let showCloseModal = $state(false);
	let closingTrade = $state<Trade | null>(null);

	// Toast state
	let successMessage = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredTrades = $derived(
		filterStatus === 'all'
			? trades
			: filterStatus === 'active'
				? trades.filter((t) => t.result === 'ACTIVE')
				: trades.filter((t) => t.result === filterStatus.toUpperCase())
	);

	const filterCounts = $derived({
		all: trades.length,
		active: trades.filter((t) => t.result === 'ACTIVE').length,
		win: trades.filter((t) => t.result === 'WIN').length,
		loss: trades.filter((t) => t.result === 'LOSS').length
	});

	const displayStats = $derived(stats ?? calculateStats(trades));

	// Transform Trade to ActivePosition for modal
	const closingPosition = $derived.by((): ActivePosition | null => {
		if (!closingTrade) return null;
		return {
			id: String(closingTrade.id),
			ticker: closingTrade.ticker,
			status: 'ACTIVE',
			entryPrice: closingTrade.entryPrice,
			currentPrice: closingTrade.entryPrice,
			unrealizedPercent: null,
			targets: [],
			stopLoss: { price: 0, percentFromEntry: 0 },
			progressToTarget1: 0,
			notes: closingTrade.notes
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadData() {
		isLoading = true;
		error = null;

		try {
			const result = await fetchTrades();
			trades = result.trades;
			stats = result.stats;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load trades';
			console.error('Failed to fetch trades:', err);
		} finally {
			isLoading = false;
		}
	}

	function handleFilterChange(filter: FilterStatus) {
		filterStatus = filter;
	}

	function handleCloseTrade(trade: Trade) {
		closingTrade = trade;
		showCloseModal = true;
	}

	function handleModalClose() {
		showCloseModal = false;
		closingTrade = null;
	}

	function handleTradeCloseSuccess() {
		successMessage = `Trade closed: ${closingTrade?.ticker}`;
		handleModalClose();
		loadData();
	}

	// Auto-dismiss success message
	$effect(() => {
		if (!successMessage) return;

		const timeout = setTimeout(() => {
			successMessage = null;
		}, 3000);
		return () => clearTimeout(timeout);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser) {
			(async () => {
				isAdmin = await checkAdminStatus();
				await loadData();
			})();
		}
	});
</script>

<svelte:head>
	<title>Trade Tracker | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
	showTradingRoomControls={false}
/>

<main class="trade-tracker">
	<!-- Page Header -->
	<header class="page-header">
		<div class="header-content">
			<div class="header-text">
				<h1>Trade Tracker</h1>
				<p>Complete history of all swing trades with performance metrics</p>
			</div>
			<ExportMenu
				roomSlug="explosive-swings"
				size="md"
				onexport={(detail) => {
					if (detail.success) {
						successMessage = `Export completed: ${detail.type}`;
					}
				}}
				onerror={(err) => {
					error = err.message;
				}}
			/>
		</div>
	</header>

	<!-- Stats Overview -->
	<StatsGrid stats={displayStats} {isLoading} />

	<!-- Filters -->
	<FilterButtons
		selected={filterStatus}
		onFilterChange={handleFilterChange}
		counts={filterCounts}
	/>

	<!-- Success Toast -->
	{#if successMessage}
		<div class="toast success" role="status" aria-live="polite">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width="20"
				height="20"
			>
				<path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			{successMessage}
		</div>
	{/if}

	<!-- Content Area -->
	<div class="content-area">
		{#if isLoading}
			<TableSkeleton rows={6} hasActions={isAdmin} />
		{:else if error}
			<div class="card-container">
				<ErrorState message={error} onRetry={loadData} />
			</div>
		{:else if filteredTrades.length === 0}
			<div class="card-container">
				<EmptyState filter={filterStatus} onReset={() => handleFilterChange('all')} />
			</div>
		{:else}
			<TradesTable trades={filteredTrades} {isAdmin} onCloseTrade={handleCloseTrade} />
		{/if}
	</div>

	<!-- Secondary Nav - Invalidated Trades Link -->
	<nav class="secondary-nav" aria-label="Secondary navigation">
		<a href="/dashboard/explosive-swings/invalidated">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width="16"
				height="16"
			>
				<circle cx="12" cy="12" r="10" />
				<path d="M12 8v4m0 4h.01" />
			</svg>
			View Invalidated Trades
		</a>
	</nav>

	<!-- Admin Link -->
	{#if isAdmin}
		<nav class="admin-nav" aria-label="Admin navigation">
			<a href="/admin/trading-rooms/explosive-swings">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="16"
					height="16"
				>
					<path
						d="M12.22 2h-.44a2 2 0 00-2 2v.18a2 2 0 01-1 1.73l-.43.25a2 2 0 01-2 0l-.15-.08a2 2 0 00-2.73.73l-.22.38a2 2 0 00.73 2.73l.15.1a2 2 0 011 1.72v.51a2 2 0 01-1 1.74l-.15.09a2 2 0 00-.73 2.73l.22.38a2 2 0 002.73.73l.15-.08a2 2 0 012 0l.43.25a2 2 0 011 1.73V20a2 2 0 002 2h.44a2 2 0 002-2v-.18a2 2 0 011-1.73l.43-.25a2 2 0 012 0l.15.08a2 2 0 002.73-.73l.22-.39a2 2 0 00-.73-2.73l-.15-.08a2 2 0 01-1-1.74v-.5a2 2 0 011-1.74l.15-.09a2 2 0 00.73-2.73l-.22-.38a2 2 0 00-2.73-.73l-.15.08a2 2 0 01-2 0l-.43-.25a2 2 0 01-1-1.73V4a2 2 0 00-2-2z"
					/>
					<circle cx="12" cy="12" r="3" />
				</svg>
				Manage All Content in Admin
			</a>
		</nav>
	{/if}
</main>

<!-- Close Trade Modal -->
<ClosePositionModal
	isOpen={showCloseModal}
	position={closingPosition}
	roomSlug="explosive-swings"
	onClose={handleModalClose}
	onSuccess={handleTradeCloseSuccess}
/>

<style>
	.trade-tracker {
		background: var(--color-bg-page);
		padding: var(--space-10) var(--space-8);
		min-height: 100vh;
	}

	.page-header {
		max-width: 1400px;
		margin: 0 auto var(--space-10);
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-6);
	}

	.header-text {
		flex: 1;
		text-align: center;
	}

	.header-text h1 {
		font-size: var(--text-3xl);
		font-weight: var(--font-bold);
		margin: 0 0 var(--space-3) 0;
		color: var(--color-text-primary);
		font-family: var(--font-display);
		letter-spacing: var(--tracking-tight);
	}

	.header-text p {
		font-size: var(--text-base);
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.content-area {
		margin-bottom: var(--space-8);
	}

	.card-container {
		max-width: 1400px;
		margin: 0 auto;
		background: var(--color-bg-card);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-md);
	}

	/* Toast */
	.toast {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		max-width: 400px;
		margin: 0 auto var(--space-6);
		padding: var(--space-3) var(--space-4);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		animation: slideIn 0.3s ease-out;
	}

	.toast.success {
		background: var(--color-profit-bg);
		color: var(--color-profit);
		border: 1px solid var(--color-profit-border);
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Secondary Nav */
	.secondary-nav {
		max-width: 1400px;
		margin: var(--space-4) auto 0;
		text-align: center;
	}

	.secondary-nav a {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		color: var(--color-text-muted);
		text-decoration: none;
		padding: var(--space-2) var(--space-4);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		transition: color 0.15s;
	}

	.secondary-nav a:hover {
		color: var(--color-text-secondary);
	}

	/* Admin Nav */
	.admin-nav {
		max-width: 1400px;
		margin: var(--space-4) auto 0;
		text-align: center;
	}

	.admin-nav a {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		color: var(--color-text-tertiary);
		text-decoration: none;
		padding: var(--space-3) var(--space-5);
		background: var(--color-bg-subtle);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		transition: var(--transition-colors);
	}

	.admin-nav a:hover {
		background: var(--color-bg-muted);
		color: var(--color-brand-primary);
	}

	.admin-nav a:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.trade-tracker {
			padding: var(--space-6) var(--space-4);
		}

		.header-content {
			flex-direction: column;
			align-items: stretch;
			gap: var(--space-4);
		}

		.header-text {
			text-align: center;
		}

		.header-text h1 {
			font-size: var(--text-2xl);
		}

		/* Center the export button on mobile */
		.header-content > :global(.export-menu) {
			align-self: center;
		}
	}
</style>
