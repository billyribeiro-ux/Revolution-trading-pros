<script lang="ts">
	/**
	 * Trade Tracker - Explosive Swings
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Complete trade history with admin controls for closing trades
	 *
	 * @version 2.0.0 - ICT 11 Principal Engineer Grade
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import ClosePositionModal from '../components/ClosePositionModal.svelte';
	import type { Trade as ApiTrade } from '$lib/types/trading';
	import type { ActivePosition } from '../types';

	// TYPE DEFINITIONS
	interface Trade {
		id: number;
		ticker: string;
		entryDate: string;
		exitDate: string | null;
		entryPrice: number;
		exitPrice: number | null;
		shares: number;
		profit: number;
		profitPercent: number;
		duration: number;
		setup: 'Breakout' | 'Momentum' | 'Reversal' | 'Earnings' | 'Pullback';
		result: 'WIN' | 'LOSS' | 'ACTIVE';
		notes: string;
		tradeType?: 'shares' | 'options';
	}

	interface TradeStats {
		totalTrades: number;
		wins: number;
		losses: number;
		winRate: string;
		totalProfit: number;
		avgWin: string;
		avgLoss: string;
		profitFactor: string;
	}

	type FilterStatus = 'all' | 'active' | 'win' | 'loss';

	// REACTIVE STATE
	let filterStatus = $state<FilterStatus>('all');
	let apiTrades = $state<ApiTrade[]>([]);
	let apiStats = $state<{
		total_pnl: number;
		win_rate: number;
		wins: number;
		losses: number;
		avg_win: number;
		avg_loss: number;
		profit_factor: number;
	} | null>(null);
	let isLoading = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');

	/**
	 * ═══════════════════════════════════════════════════════════════════════════
	 * DEVELOPMENT MODE
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Set true to use mock data during implementation/testing
	 */
	const USE_MOCK_DATA = false;

	// Admin state
	let isAdmin = $state(false);
	let showCloseTradeModal = $state(false);
	let closingTrade = $state<Trade | null>(null);

	const ROOM_SLUG = 'explosive-swings';

	// Check admin status
	async function checkAdminStatus() {
		try {
			const response = await fetch('/api/auth/me');
			if (response.ok) {
				const data = await response.json();
				isAdmin = data.user?.role === 'admin' || data.user?.is_admin === true;
			}
		} catch {
			isAdmin = false;
		}
	}

	// Fetch trades from API
	async function fetchTrades() {
		if (USE_MOCK_DATA) {
			isLoading = false;
			return;
		}
		
		isLoading = true;
		errorMessage = '';
		
		try {
			const response = await fetch(`/api/trades/${ROOM_SLUG}?limit=100`, {
				credentials: 'include'
			});
			
			if (!response.ok) {
				throw new Error(`HTTP ${response.status}`);
			}
			
			const data = await response.json();
			
			if (data.success) {
				apiTrades = data.data;
				apiStats = data.stats;
			} else {
				throw new Error(data.error || 'Failed to load trades');
			}
		} catch (err) {
			errorMessage = 'Failed to load trades. Please try again.';
			console.error('Failed to fetch trades:', err);
		} finally {
			isLoading = false;
		}
	}

	// Open close trade modal
	function openCloseTrade(trade: Trade) {
		closingTrade = trade;
		showCloseTradeModal = true;
	}

	// Transform Trade to ActivePosition for ClosePositionModal
	const closingPosition = $derived.by((): ActivePosition | null => {
		if (!closingTrade) return null;
		return {
			id: closingTrade.id,
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

	onMount(() => {
		checkAdminStatus();
		fetchTrades();
	});

	// Clear messages after delay
	$effect(() => {
		if (successMessage) {
			const timeout = setTimeout(() => (successMessage = ''), 3000);
			return () => clearTimeout(timeout);
		}
		return undefined;
	});

	// Format date for display
	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	const trades: Trade[] = [
		{
			id: 1,
			ticker: 'MSFT',
			entryDate: 'Jan 5, 2026',
			exitDate: 'Jan 10, 2026',
			entryPrice: 425.0,
			exitPrice: 460.0,
			shares: 100,
			profit: 2450,
			profitPercent: 8.2,
			duration: 5,
			setup: 'Breakout',
			result: 'WIN',
			notes: 'Perfect breakout setup. Held through consolidation and exited at T2.'
		},
		{
			id: 2,
			ticker: 'META',
			entryDate: 'Jan 8, 2026',
			exitDate: 'Jan 12, 2026',
			entryPrice: 575.0,
			exitPrice: 610.0,
			shares: 50,
			profit: 1820,
			profitPercent: 6.1,
			duration: 3,
			setup: 'Momentum',
			result: 'WIN',
			notes: 'Strong momentum play. Quick move to target.'
		},
		{
			id: 3,
			ticker: 'AMD',
			entryDate: 'Jan 9, 2026',
			exitDate: 'Jan 11, 2026',
			entryPrice: 130.0,
			exitPrice: 127.0,
			shares: 200,
			profit: -680,
			profitPercent: -2.3,
			duration: 2,
			setup: 'Reversal',
			result: 'LOSS',
			notes: "Stopped out. Reversal didn't materialize."
		},
		{
			id: 4,
			ticker: 'NFLX',
			entryDate: 'Jan 2, 2026',
			exitDate: 'Jan 9, 2026',
			entryPrice: 520.0,
			exitPrice: 570.0,
			shares: 60,
			profit: 3100,
			profitPercent: 9.5,
			duration: 7,
			setup: 'Earnings',
			result: 'WIN',
			notes: 'Earnings catalyst worked perfectly. Held for runner target.'
		},
		{
			id: 5,
			ticker: 'NVDA',
			entryDate: 'Jan 13, 2026',
			exitDate: null,
			entryPrice: 142.5,
			exitPrice: null,
			shares: 150,
			profit: 0,
			profitPercent: 0,
			duration: 0,
			setup: 'Breakout',
			result: 'ACTIVE',
			notes: 'Currently holding. Watching for T1 at $148.'
		},
		{
			id: 6,
			ticker: 'TSLA',
			entryDate: 'Dec 28, 2025',
			exitDate: 'Jan 5, 2026',
			entryPrice: 235.0,
			exitPrice: 265.0,
			shares: 80,
			profit: 2400,
			profitPercent: 12.8,
			duration: 8,
			setup: 'Momentum',
			result: 'WIN',
			notes: 'Great momentum trade. Hit T3 and exited.'
		},
		{
			id: 7,
			ticker: 'AMZN',
			entryDate: 'Jan 11, 2026',
			exitDate: null,
			entryPrice: 185.0,
			exitPrice: null,
			shares: 100,
			profit: 0,
			profitPercent: 0,
			duration: 2,
			setup: 'Breakout',
			result: 'ACTIVE',
			notes: 'Active position. Currently at T1.'
		},
		{
			id: 8,
			ticker: 'GOOGL',
			entryDate: 'Dec 20, 2025',
			exitDate: 'Dec 30, 2025',
			entryPrice: 168.0,
			exitPrice: 175.0,
			shares: 120,
			profit: 840,
			profitPercent: 4.2,
			duration: 10,
			setup: 'Momentum',
			result: 'WIN',
			notes: 'Slow grind higher. Took profits at T1.'
		}
	];

	// Transform API trades to display format or use fallback
	const displayTrades = $derived.by((): Trade[] => {
		// Development mode: use mock data
		if (USE_MOCK_DATA) {
			return trades;
		}
		
		// Production: transform API data
		return apiTrades.map((t) => ({
			id: t.id,
			ticker: t.ticker,
			entryDate: formatDate(t.entry_date),
			exitDate: t.exit_date ? formatDate(t.exit_date) : null,
			entryPrice: t.entry_price,
			exitPrice: t.exit_price,
			shares: t.quantity,
			profit: t.pnl || 0,
			profitPercent: t.pnl_percent || 0,
			duration: t.holding_days || 0,
			setup: (t.setup || 'Breakout') as Trade['setup'],
			result: t.status === 'open' ? 'ACTIVE' : (t.pnl || 0) >= 0 ? 'WIN' : 'LOSS',
			notes: t.notes || '',
			tradeType: t.trade_type
		}));
	});

	const filteredTrades = $derived(
		filterStatus === 'all'
			? displayTrades
			: filterStatus === 'active'
				? displayTrades.filter((t) => t.result === 'ACTIVE')
				: displayTrades.filter((t) => t.result === filterStatus.toUpperCase())
	);

	const stats = $derived.by((): TradeStats => {
		// Use API stats if available
		if (apiStats) {
			return {
				totalTrades: (apiStats.wins ?? 0) + (apiStats.losses ?? 0),
				wins: apiStats.wins ?? 0,
				losses: apiStats.losses ?? 0,
				winRate: (apiStats.win_rate ?? 0).toFixed(1),
				totalProfit: apiStats.total_pnl ?? 0,
				avgWin: (apiStats.avg_win ?? 0).toFixed(0),
				avgLoss: (apiStats.avg_loss ?? 0).toFixed(0),
				profitFactor: (apiStats.profit_factor ?? 0).toFixed(2)
			};
		}

		// Fallback to calculating from displayTrades
		const closedTrades = displayTrades.filter((t) => t.result !== 'ACTIVE');
		const wins = closedTrades.filter((t) => t.result === 'WIN').length;
		const losses = closedTrades.filter((t) => t.result === 'LOSS').length;
		const totalProfit = closedTrades.reduce((sum, t) => sum + t.profit, 0);
		const avgWin =
			wins > 0
				? closedTrades.filter((t) => t.result === 'WIN').reduce((sum, t) => sum + t.profit, 0) /
					wins
				: 0;
		const avgLoss =
			losses > 0
				? Math.abs(
						closedTrades.filter((t) => t.result === 'LOSS').reduce((sum, t) => sum + t.profit, 0) /
							losses
					)
				: 0;

		return {
			totalTrades: closedTrades.length,
			wins,
			losses,
			winRate: closedTrades.length > 0 ? ((wins / closedTrades.length) * 100).toFixed(1) : '0.0',
			totalProfit,
			avgWin: avgWin.toFixed(0),
			avgLoss: avgLoss.toFixed(0),
			profitFactor: avgLoss > 0 ? (avgWin / avgLoss).toFixed(2) : '0.00'
		};
	});
</script>

<svelte:head>
	<title>Trade Tracker | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="trade-tracker">
	<div class="page-header">
		<h1>Trade Tracker</h1>
		<p>Complete history of all swing trades with performance metrics</p>
	</div>

	<!-- Stats Overview -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-value">{stats.totalTrades}</div>
			<div class="stat-label">Total Trades</div>
		</div>
		<div class="stat-card">
			<div class="stat-value green">{stats.wins}</div>
			<div class="stat-label">Wins</div>
		</div>
		<div class="stat-card">
			<div class="stat-value red">{stats.losses}</div>
			<div class="stat-label">Losses</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.winRate}%</div>
			<div class="stat-label">Win Rate</div>
		</div>
		<div class="stat-card">
			<div class="stat-value green">${stats.totalProfit.toLocaleString()}</div>
			<div class="stat-label">Total Profit</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">${stats.avgWin}</div>
			<div class="stat-label">Avg Win</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">${stats.avgLoss}</div>
			<div class="stat-label">Avg Loss</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.profitFactor}</div>
			<div class="stat-label">Profit Factor</div>
		</div>
	</div>

	<!-- Filter Buttons -->
	<div class="filter-section">
		<button
			class="filter-btn"
			class:active={filterStatus === 'all'}
			aria-pressed={filterStatus === 'all'}
			onclick={() => (filterStatus = 'all')}
		>
			All Trades
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'active'}
			aria-pressed={filterStatus === 'active'}
			onclick={() => (filterStatus = 'active')}
		>
			Active
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'win'}
			aria-pressed={filterStatus === 'win'}
			onclick={() => (filterStatus = 'win')}
		>
			Wins
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'loss'}
			aria-pressed={filterStatus === 'loss'}
			onclick={() => (filterStatus = 'loss')}
		>
			Losses
		</button>
	</div>

	<!-- Success/Error Messages -->
	{#if successMessage}
		<div class="message success-message">{successMessage}</div>
	{/if}

	<!-- Loading State -->
	{#if isLoading}
		<div class="trades-container">
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading trades...</p>
			</div>
		</div>
	{:else if errorMessage && !USE_MOCK_DATA}
		<div class="trades-container">
			<div class="error-state">
				<p>{errorMessage}</p>
				<button class="retry-btn" onclick={fetchTrades}>Try Again</button>
			</div>
		</div>
	{:else}
		<!-- Trades Table -->
		<div class="trades-container">
		<div class="trades-table" role="table" aria-label="Trade history">
				<div class="table-header" class:has-actions={isAdmin} role="row">
					<div role="columnheader">Ticker</div>
					<div role="columnheader">Entry</div>
					<div role="columnheader">Exit</div>
					<div role="columnheader">Entry $</div>
					<div role="columnheader">Exit $</div>
					<div role="columnheader">Profit</div>
					<div role="columnheader">%</div>
					<div role="columnheader">Days</div>
					<div role="columnheader">Setup</div>
					<div role="columnheader">Result</div>
					{#if isAdmin}<div role="columnheader">Actions</div>{/if}
				</div>
				{#each filteredTrades as trade (trade.id)}
					<div class="table-row" class:active={trade.result === 'ACTIVE'} class:has-actions={isAdmin} role="row">
					<div class="ticker-cell" role="cell">{trade.ticker}</div>
						<div role="cell">{trade.entryDate}</div>
						<div role="cell">{trade.exitDate || 'Active'}</div>
						<div role="cell">${(trade.entryPrice ?? 0).toFixed(2)}</div>
						<div role="cell">{trade.exitPrice ? `$${trade.exitPrice.toFixed(2)}` : '-'}</div>
						<div role="cell" class:green={trade.profit > 0} class:red={trade.profit < 0}>
							{trade.profit !== 0 ? `$${trade.profit.toLocaleString()}` : '-'}
						</div>
						<div role="cell" class:green={trade.profitPercent > 0} class:red={trade.profitPercent < 0}>
							{trade.profitPercent !== 0
								? `${trade.profitPercent > 0 ? '+' : ''}${trade.profitPercent}%`
								: '-'}
						</div>
						<div role="cell">{trade.duration || '-'}</div>
						<div role="cell"><span class="setup-badge">{trade.setup}</span></div>
						<div role="cell">
							<span class="result-badge result--{trade.result.toLowerCase()}">{trade.result}</span>
						</div>
					{#if isAdmin}
							<div class="actions-cell" role="cell">
								{#if trade.result === 'ACTIVE'}
									<button class="close-trade-btn" onclick={() => openCloseTrade(trade)}>
										Close Trade
									</button>
								{:else}
									<span class="closed-indicator">Closed</span>
								{/if}
							</div>
						{/if}
					</div>
					<div class="notes-row">
						<div class="notes-content">
							<strong>Notes:</strong>
							{trade.notes}
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Admin Link -->
	{#if isAdmin}
		<div class="admin-link">
			<a href="/admin/trading-rooms/explosive-swings">
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
					<circle cx="12" cy="12" r="3"/>
				</svg>
				Manage All Content in Admin
			</a>
		</div>
	{/if}
</div>

<!-- Close Trade Modal -->
<ClosePositionModal
	isOpen={showCloseTradeModal}
	position={closingPosition}
	roomSlug={ROOM_SLUG}
	onClose={() => {
		showCloseTradeModal = false;
		closingTrade = null;
	}}
	onSuccess={() => {
		successMessage = `Trade closed: ${closingTrade?.ticker}`;
		fetchTrades();
	}}
/>

<style>
	.trade-tracker {
		background: #f5f7fa;
		padding: 40px 30px;
	}

	.page-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 40px;
	}

	.page-header h1 {
		font-size: 36px;
		font-weight: 700;
		margin: 0 0 12px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.page-header p {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
		max-width: 1400px;
		margin: 0 auto 40px;
	}

	@media (min-width: 640px) {
		.stats-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(8, 1fr);
		}
	}

	.stat-card {
		background: #fff;
		border-radius: 12px;
		padding: 20px;
		text-align: center;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.06);
	}

	.stat-value {
		font-size: 28px;
		font-weight: 700;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
		margin-bottom: 5px;
	}

	.stat-value.green {
		color: #22c55e;
	}

	.stat-value.red {
		color: #ef4444;
	}

	.stat-label {
		font-size: 12px;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-section {
		display: flex;
		gap: 10px;
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: 30px;
	}

	.filter-btn {
		background: #fff;
		border: 2px solid #e5e7eb;
		padding: 10px 20px;
		border-radius: 25px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		border-color: #143e59;
		color: #143e59;
	}

	.filter-btn.active {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.trades-container {
		max-width: 1400px;
		margin: 0 auto;
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		overflow-x: auto;
	}

	.trades-table {
		min-width: 900px;
	}

	.table-header {
		display: grid;
		grid-template-columns: 80px 100px 100px 90px 90px 100px 80px 60px 100px 90px;
		gap: 10px;
		padding: 15px 10px;
		background: #f8fafc;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		margin-bottom: 10px;
	}

	.table-row {
		display: grid;
		grid-template-columns: 80px 100px 100px 90px 90px 100px 80px 60px 100px 90px;
		gap: 10px;
		padding: 15px 10px;
		border-bottom: 1px solid #f0f0f0;
		font-size: 14px;
		align-items: center;
	}

	.table-row.active {
		background: #fffbf5;
	}

	.ticker-cell {
		font-weight: 700;
		color: #143e59;
		font-size: 16px;
	}

	.green {
		color: #22c55e;
		font-weight: 600;
	}

	.red {
		color: #ef4444;
		font-weight: 600;
	}

	.setup-badge {
		display: inline-block;
		background: #e5e7eb;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
	}

	.result-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.result--win {
		background: #dcfce7;
		color: #166534;
	}

	.result--loss {
		background: #fee2e2;
		color: #991b1b;
	}

	/* Messages */
	.message {
		max-width: 1400px;
		margin: 0 auto 20px;
		padding: 12px 20px;
		border-radius: 8px;
		font-weight: 500;
	}

	.success-message {
		background: #dcfce7;
		color: #166534;
	}

	.error-message {
		background: #fee2e2;
		color: #991b1b;
	}

	/* Table with actions */
	.table-header.has-actions,
	.table-row.has-actions {
		grid-template-columns: 80px 100px 100px 90px 90px 100px 80px 60px 100px 90px 100px;
	}

	.actions-cell {
		display: flex;
		align-items: center;
	}

	.close-trade-btn {
		background: #f69532;
		color: white;
		border: none;
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-trade-btn:hover {
		background: #e8852d;
	}

	.closed-indicator {
		color: #999;
		font-size: 12px;
	}

	/* Admin Link */
	.admin-link {
		max-width: 1400px;
		margin: 30px auto 0;
		text-align: center;
	}

	.admin-link a {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #666;
		text-decoration: none;
		padding: 12px 24px;
		background: #f8fafc;
		border-radius: 8px;
		font-weight: 500;
		transition: all 0.2s;
	}

	.admin-link a:hover {
		background: #e5e7eb;
		color: #143e59;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #666;
	}

	.loading-state .spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.loading-state p {
		margin: 0;
		font-size: 14px;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #dc2626;
	}

	.error-state p {
		margin: 0 0 16px;
		font-size: 14px;
		font-weight: 500;
	}

	.retry-btn {
		padding: 10px 20px;
		background: #dc2626;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
	}

	.retry-btn:hover {
		background: #b91c1c;
	}

	/* Focus Visible */
	.filter-btn:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.close-trade-btn:focus-visible,
	.retry-btn:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.admin-link a:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.result--active {
		background: #fef3c7;
		color: #92400e;
	}

	.notes-row {
		padding: 10px 10px 20px;
		border-bottom: 2px solid #e5e7eb;
	}

	.notes-content {
		font-size: 13px;
		color: #666;
		line-height: 1.6;
	}

	.notes-content strong {
		color: #333;
	}
</style>
