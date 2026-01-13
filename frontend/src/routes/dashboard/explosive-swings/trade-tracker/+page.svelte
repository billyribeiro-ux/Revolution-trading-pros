<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	let filterStatus = $state('all');

	const trades = [
		{
			id: 1,
			ticker: 'MSFT',
			entryDate: 'Jan 5, 2026',
			exitDate: 'Jan 10, 2026',
			entryPrice: 425.00,
			exitPrice: 460.00,
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
			entryPrice: 575.00,
			exitPrice: 610.00,
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
			entryPrice: 130.00,
			exitPrice: 127.00,
			shares: 200,
			profit: -680,
			profitPercent: -2.3,
			duration: 2,
			setup: 'Reversal',
			result: 'LOSS',
			notes: 'Stopped out. Reversal didn\'t materialize.'
		},
		{
			id: 4,
			ticker: 'NFLX',
			entryDate: 'Jan 2, 2026',
			exitDate: 'Jan 9, 2026',
			entryPrice: 520.00,
			exitPrice: 570.00,
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
			entryPrice: 142.50,
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
			entryPrice: 235.00,
			exitPrice: 265.00,
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
			entryPrice: 185.00,
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
			entryPrice: 168.00,
			exitPrice: 175.00,
			shares: 120,
			profit: 840,
			profitPercent: 4.2,
			duration: 10,
			setup: 'Momentum',
			result: 'WIN',
			notes: 'Slow grind higher. Took profits at T1.'
		}
	];

	const filteredTrades = $derived(
		filterStatus === 'all'
			? trades
			: filterStatus === 'active'
			? trades.filter(t => t.result === 'ACTIVE')
			: trades.filter(t => t.result === filterStatus.toUpperCase())
	);

	const stats = $derived.by(() => {
		const closedTrades = trades.filter(t => t.result !== 'ACTIVE');
		const wins = closedTrades.filter(t => t.result === 'WIN').length;
		const losses = closedTrades.filter(t => t.result === 'LOSS').length;
		const totalProfit = closedTrades.reduce((sum, t) => sum + t.profit, 0);
		const avgWin = closedTrades.filter(t => t.result === 'WIN').reduce((sum, t) => sum + t.profit, 0) / wins;
		const avgLoss = Math.abs(closedTrades.filter(t => t.result === 'LOSS').reduce((sum, t) => sum + t.profit, 0) / losses);
		
		return {
			totalTrades: closedTrades.length,
			wins,
			losses,
			winRate: ((wins / closedTrades.length) * 100).toFixed(1),
			totalProfit,
			avgWin: avgWin.toFixed(0),
			avgLoss: avgLoss.toFixed(0),
			profitFactor: (avgWin / avgLoss).toFixed(2)
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
		<button class="filter-btn" class:active={filterStatus === 'all'} onclick={() => filterStatus = 'all'}>
			All Trades
		</button>
		<button class="filter-btn" class:active={filterStatus === 'active'} onclick={() => filterStatus = 'active'}>
			Active
		</button>
		<button class="filter-btn" class:active={filterStatus === 'win'} onclick={() => filterStatus = 'win'}>
			Wins
		</button>
		<button class="filter-btn" class:active={filterStatus === 'loss'} onclick={() => filterStatus = 'loss'}>
			Losses
		</button>
	</div>

	<!-- Trades Table -->
	<div class="trades-container">
		<div class="trades-table">
			<div class="table-header">
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
			</div>
			{#each filteredTrades as trade}
				<div class="table-row" class:active={trade.result === 'ACTIVE'}>
					<div class="ticker-cell">{trade.ticker}</div>
					<div>{trade.entryDate}</div>
					<div>{trade.exitDate || 'Active'}</div>
					<div>${trade.entryPrice.toFixed(2)}</div>
					<div>{trade.exitPrice ? `$${trade.exitPrice.toFixed(2)}` : '-'}</div>
					<div class:green={trade.profit > 0} class:red={trade.profit < 0}>
						{trade.profit !== 0 ? `$${trade.profit.toLocaleString()}` : '-'}
					</div>
					<div class:green={trade.profitPercent > 0} class:red={trade.profitPercent < 0}>
						{trade.profitPercent !== 0 ? `${trade.profitPercent > 0 ? '+' : ''}${trade.profitPercent}%` : '-'}
					</div>
					<div>{trade.duration || '-'}</div>
					<div><span class="setup-badge">{trade.setup}</span></div>
					<div><span class="result-badge result--{trade.result.toLowerCase()}">{trade.result}</span></div>
				</div>
				<div class="notes-row">
					<div class="notes-content">
						<strong>Notes:</strong> {trade.notes}
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>

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
		box-shadow: 0 2px 10px rgba(0,0,0,0.06);
	}

	.stat-value {
		font-size: 28px;
		font-weight: 700;
		color: #143E59;
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
		border-color: #143E59;
		color: #143E59;
	}

	.filter-btn.active {
		background: #143E59;
		border-color: #143E59;
		color: #fff;
	}

	.trades-container {
		max-width: 1400px;
		margin: 0 auto;
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
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
		color: #143E59;
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
