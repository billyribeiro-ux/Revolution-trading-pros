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
	import type { Trade as ApiTrade } from '$lib/types/trading';

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

	// Admin state
	let isAdmin = $state(false);
	let showCloseTradeModal = $state(false);
	let closingTrade = $state<Trade | null>(null);
	let isClosingTrade = $state(false);
	let closeTradeForm = $state({
		exit_price: '',
		exit_date: new Date().toISOString().split('T')[0],
		notes: ''
	});
	let successMessage = $state('');
	let errorMessage = $state('');

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
		isLoading = true;
		try {
			const response = await fetch(`/api/trades/${ROOM_SLUG}?limit=100`);
			const data = await response.json();
			if (data.success) {
				apiTrades = data.data;
				apiStats = data.stats;
			}
		} catch (err) {
			console.error('Failed to fetch trades:', err);
		} finally {
			isLoading = false;
		}
	}

	// Open close trade modal
	function openCloseTrade(trade: Trade) {
		closingTrade = trade;
		closeTradeForm = {
			exit_price: '',
			exit_date: new Date().toISOString().split('T')[0],
			notes: trade.notes || ''
		};
		showCloseTradeModal = true;
	}

	// Close trade handler
	async function closeTrade() {
		if (!closingTrade || !closeTradeForm.exit_price) {
			errorMessage = 'Exit price is required';
			return;
		}

		isClosingTrade = true;
		errorMessage = '';

		try {
			// Use PUT to the trade endpoint with exit_price to close it
			const response = await fetch(`/api/trades/${ROOM_SLUG}/${closingTrade.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					exit_price: parseFloat(closeTradeForm.exit_price),
					exit_date: closeTradeForm.exit_date,
					notes: closeTradeForm.notes,
					status: 'closed'
				})
			});

			const data = await response.json();

			if (data.success) {
				successMessage = `Trade closed: ${closingTrade.ticker} at $${closeTradeForm.exit_price}`;
				showCloseTradeModal = false;
				closingTrade = null;
				await fetchTrades();
			} else {
				errorMessage = data.error || 'Failed to close trade';
			}
		} catch (err) {
			errorMessage = 'Failed to close trade';
			console.error(err);
		} finally {
			isClosingTrade = false;
		}
	}

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
	const displayTrades = $derived<Trade[]>(
		apiTrades.length > 0
			? apiTrades.map((t) => ({
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
				}))
			: trades
	);

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
			onclick={() => (filterStatus = 'all')}
		>
			All Trades
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'active'}
			onclick={() => (filterStatus = 'active')}
		>
			Active
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'win'}
			onclick={() => (filterStatus = 'win')}
		>
			Wins
		</button>
		<button
			class="filter-btn"
			class:active={filterStatus === 'loss'}
			onclick={() => (filterStatus = 'loss')}
		>
			Losses
		</button>
	</div>

	<!-- Success/Error Messages -->
	{#if successMessage}
		<div class="message success-message">{successMessage}</div>
	{/if}
	{#if errorMessage}
		<div class="message error-message">{errorMessage}</div>
	{/if}

	<!-- Trades Table -->
	<div class="trades-container">
		<div class="trades-table">
			<div class="table-header" class:has-actions={isAdmin}>
				<div>Ticker</div>
				<div>Entry</div>
				<div>Exit</div>
				<div>Entry $</div>
				<div>Exit $</div>
				<div>Profit</div>
				<div>%</div>
				<div>Days</div>
				<div>Setup</div>
				<div>Result</div>
				{#if isAdmin}<div>Actions</div>{/if}
			</div>
			{#each filteredTrades as trade}
				<div class="table-row" class:active={trade.result === 'ACTIVE'} class:has-actions={isAdmin}>
					<div class="ticker-cell">{trade.ticker}</div>
					<div>{trade.entryDate}</div>
					<div>{trade.exitDate || 'Active'}</div>
					<div>${(trade.entryPrice ?? 0).toFixed(2)}</div>
					<div>{trade.exitPrice ? `$${trade.exitPrice.toFixed(2)}` : '-'}</div>
					<div class:green={trade.profit > 0} class:red={trade.profit < 0}>
						{trade.profit !== 0 ? `$${trade.profit.toLocaleString()}` : '-'}
					</div>
					<div class:green={trade.profitPercent > 0} class:red={trade.profitPercent < 0}>
						{trade.profitPercent !== 0
							? `${trade.profitPercent > 0 ? '+' : ''}${trade.profitPercent}%`
							: '-'}
					</div>
					<div>{trade.duration || '-'}</div>
					<div><span class="setup-badge">{trade.setup}</span></div>
					<div>
						<span class="result-badge result--{trade.result.toLowerCase()}">{trade.result}</span>
					</div>
					{#if isAdmin}
						<div class="actions-cell">
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
{#if showCloseTradeModal && closingTrade}
	<div 
		class="modal-overlay" 
		onclick={() => (showCloseTradeModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showCloseTradeModal = false)}
		role="dialog"
		aria-modal="true"
		aria-labelledby="close-trade-title"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div 
			class="modal" 
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Close Trade: {closingTrade.ticker}</h2>
				<button class="modal-close" onclick={() => (showCloseTradeModal = false)}>&times;</button>
			</div>

			<div class="modal-body">
				<div class="trade-summary">
					<div class="summary-row">
						<span>Entry Price:</span>
						<strong>${closingTrade.entryPrice.toFixed(2)}</strong>
					</div>
					<div class="summary-row">
						<span>Entry Date:</span>
						<strong>{closingTrade.entryDate}</strong>
					</div>
					<div class="summary-row">
						<span>Shares:</span>
						<strong>{closingTrade.shares}</strong>
					</div>
				</div>

				<div class="form-group">
					<label for="exit_price">Exit Price *</label>
					<input
						type="number"
						id="exit_price"
						step="0.01"
						placeholder="e.g., 155.00"
						bind:value={closeTradeForm.exit_price}
						required
					/>
				</div>

				<div class="form-group">
					<label for="exit_date">Exit Date</label>
					<input
						type="date"
						id="exit_date"
						bind:value={closeTradeForm.exit_date}
					/>
				</div>

				<div class="form-group">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						rows="3"
						placeholder="Exit notes..."
						bind:value={closeTradeForm.notes}
					></textarea>
				</div>

				{#if closeTradeForm.exit_price}
					{@const exitPrice = parseFloat(closeTradeForm.exit_price)}
					{@const pnl = (exitPrice - closingTrade.entryPrice) * closingTrade.shares}
					{@const pnlPercent = ((exitPrice - closingTrade.entryPrice) / closingTrade.entryPrice) * 100}
					<div class="pnl-preview" class:profit={pnl > 0} class:loss={pnl < 0}>
						<span>Estimated P&L:</span>
						<strong>{pnl > 0 ? '+' : ''}${pnl.toFixed(2)} ({pnlPercent > 0 ? '+' : ''}{pnlPercent.toFixed(2)}%)</strong>
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-cancel" onclick={() => (showCloseTradeModal = false)}>
					Cancel
				</button>
				<button class="btn-close-trade" onclick={closeTrade} disabled={isClosingTrade || !closeTradeForm.exit_price}>
					{isClosingTrade ? 'Closing...' : 'Close Trade'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.trade-tracker {
		background: #f5f7fa;
		min-height: 100vh;
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

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		background: white;
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e7eb;
	}

	.modal-header h2 {
		margin: 0;
		font-size: 20px;
		color: #143e59;
	}

	.modal-close {
		background: none;
		border: none;
		font-size: 28px;
		color: #999;
		cursor: pointer;
		line-height: 1;
	}

	.modal-close:hover {
		color: #333;
	}

	.modal-body {
		padding: 24px;
	}

	.trade-summary {
		background: #f8fafc;
		border-radius: 8px;
		padding: 16px;
		margin-bottom: 20px;
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		padding: 6px 0;
	}

	.summary-row span {
		color: #666;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #666;
		margin-bottom: 6px;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 12px;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 14px;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f69532;
	}

	.pnl-preview {
		display: flex;
		justify-content: space-between;
		padding: 16px;
		border-radius: 8px;
		margin-top: 16px;
	}

	.pnl-preview.profit {
		background: #dcfce7;
		color: #166534;
	}

	.pnl-preview.loss {
		background: #fee2e2;
		color: #991b1b;
	}

	.modal-footer {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
		padding: 20px 24px;
		border-top: 1px solid #e5e7eb;
	}

	.btn-cancel {
		padding: 12px 24px;
		background: #e5e7eb;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
	}

	.btn-cancel:hover {
		background: #d1d5db;
	}

	.btn-close-trade {
		padding: 12px 24px;
		background: #f69532;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
	}

	.btn-close-trade:hover:not(:disabled) {
		background: #e8852d;
	}

	.btn-close-trade:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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
