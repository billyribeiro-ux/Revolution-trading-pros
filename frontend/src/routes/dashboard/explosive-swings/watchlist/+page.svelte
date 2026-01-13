<script lang="ts">
	/**
	 * Watchlist - Explosive Swings
	 * @version 1.0.0
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	// TYPE DEFINITIONS
	interface WatchlistEntry {
		ticker: string;
		company: string;
		bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		optionsStrike: string;
		optionsExp: string;
		currentPrice: string;
		change: string;
		status: 'active' | 'watching' | 'closed';
		notes: string;
		technicals: string;
		catalyst: string;
	}

	const watchlist: WatchlistEntry[] = [
		{
			ticker: 'NVDA',
			company: 'NVIDIA Corporation',
			bias: 'BULLISH',
			entry: '$142.50',
			target1: '$148.00',
			target2: '$152.00',
			target3: '$158.00',
			runner: '$165.00+',
			stop: '$136.00',
			optionsStrike: '$145 Call',
			optionsExp: 'Jan 24, 2026',
			currentPrice: '$145.20',
			change: '+1.9%',
			status: 'active',
			notes: 'Breakout above consolidation. Wait for pullback to entry zone. Strong momentum with volume confirmation.',
			technicals: 'RSI: 62, MACD: Bullish crossover, Volume: Above average',
			catalyst: 'AI chip demand, Q4 earnings upcoming'
		},
		{
			ticker: 'TSLA',
			company: 'Tesla Inc',
			bias: 'BULLISH',
			entry: '$248.00',
			target1: '$255.00',
			target2: '$265.00',
			target3: '$275.00',
			runner: '$290.00+',
			stop: '$235.00',
			optionsStrike: '$250 Call',
			optionsExp: 'Jan 31, 2026',
			currentPrice: '$251.30',
			change: '+1.3%',
			status: 'active',
			notes: 'Momentum building. Earnings catalyst ahead. Watch for continuation above $250.',
			technicals: 'RSI: 58, MACD: Positive, Volume: Increasing',
			catalyst: 'Q4 deliveries beat, FSD updates'
		},
		{
			ticker: 'AMZN',
			company: 'Amazon.com Inc',
			bias: 'BULLISH',
			entry: '$185.00',
			target1: '$190.00',
			target2: '$195.00',
			target3: '$198.00',
			runner: '$205.00+',
			stop: '$178.00',
			optionsStrike: '$185 Call',
			optionsExp: 'Jan 24, 2026',
			currentPrice: '$187.50',
			change: '+1.4%',
			status: 'active',
			notes: 'Breaking above key resistance. Strong volume confirmation. Cloud growth story intact.',
			technicals: 'RSI: 65, MACD: Bullish, Volume: High',
			catalyst: 'AWS growth, holiday sales data'
		},
		{
			ticker: 'GOOGL',
			company: 'Alphabet Inc',
			bias: 'NEUTRAL',
			entry: '$175.50',
			target1: '$180.00',
			target2: '$185.00',
			target3: '$188.00',
			runner: '$195.00+',
			stop: '$168.00',
			optionsStrike: '$177.50 Call',
			optionsExp: 'Feb 7, 2026',
			currentPrice: '$174.80',
			change: '-0.4%',
			status: 'watching',
			notes: 'Watching for breakout. Not triggered yet. Need to see price action above $176.',
			technicals: 'RSI: 52, MACD: Neutral, Volume: Average',
			catalyst: 'AI integration, search revenue'
		},
		{
			ticker: 'META',
			company: 'Meta Platforms Inc',
			bias: 'BULLISH',
			entry: '$585.00',
			target1: '$600.00',
			target2: '$615.00',
			target3: '$630.00',
			runner: '$650.00+',
			stop: '$565.00',
			optionsStrike: '$590 Call',
			optionsExp: 'Jan 24, 2026',
			currentPrice: '$592.40',
			change: '+1.3%',
			status: 'active',
			notes: 'Strong trend. Buy dips to support. AI monetization story gaining traction.',
			technicals: 'RSI: 68, MACD: Strong bullish, Volume: Above average',
			catalyst: 'AI revenue growth, metaverse updates'
		},
		{
			ticker: 'AMD',
			company: 'Advanced Micro Devices',
			bias: 'BEARISH',
			entry: '$125.00',
			target1: '$120.00',
			target2: '$115.00',
			target3: '$110.00',
			runner: '$100.00',
			stop: '$132.00',
			optionsStrike: '$122 Put',
			optionsExp: 'Jan 31, 2026',
			currentPrice: '$123.50',
			change: '-1.2%',
			status: 'active',
			notes: 'Breakdown in progress. Short on bounces. Losing market share concerns.',
			technicals: 'RSI: 38, MACD: Bearish, Volume: Increasing on down days',
			catalyst: 'Competition pressure, margin concerns'
		}
	];
</script>

<svelte:head>
	<title>This Week's Watchlist | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="watchlist-page">
	<div class="page-header">
		<h1>This Week's Watchlist</h1>
		<p>Complete trade plan with entries, targets, stops, and options plays</p>
	</div>

	<div class="watchlist-grid">
		{#each watchlist as stock}
			<div class="stock-card">
				<div class="stock-header">
					<div class="stock-title">
						<h2>{stock.ticker}</h2>
						<span class="company-name">{stock.company}</span>
					</div>
					<div class="stock-price">
						<div class="current-price">{stock.currentPrice}</div>
						<div class="price-change" class:positive={stock.change.startsWith('+')} class:negative={stock.change.startsWith('-')}>
							{stock.change}
						</div>
					</div>
				</div>

				<div class="bias-badge bias--{stock.bias.toLowerCase()}">
					{stock.bias}
				</div>

				<div class="trade-plan">
					<div class="plan-row">
						<span class="label">Entry:</span>
						<span class="value entry">{stock.entry}</span>
					</div>
					<div class="plan-row">
						<span class="label">Target 1:</span>
						<span class="value target">{stock.target1}</span>
					</div>
					<div class="plan-row">
						<span class="label">Target 2:</span>
						<span class="value target">{stock.target2}</span>
					</div>
					<div class="plan-row">
						<span class="label">Target 3:</span>
						<span class="value target">{stock.target3}</span>
					</div>
					<div class="plan-row">
						<span class="label">Runner:</span>
						<span class="value runner">{stock.runner}</span>
					</div>
					<div class="plan-row">
						<span class="label">Stop:</span>
						<span class="value stop">{stock.stop}</span>
					</div>
				</div>

				<div class="options-section">
					<h4>Options Play</h4>
					<div class="options-info">
						<span>{stock.optionsStrike}</span>
						<span>Exp: {stock.optionsExp}</span>
					</div>
				</div>

				<div class="status-badge status--{stock.status}">
					{stock.status.toUpperCase()}
				</div>

				<div class="notes-section">
					<h4>Trade Notes</h4>
					<p>{stock.notes}</p>
				</div>

				<div class="technicals-section">
					<h4>Technicals</h4>
					<p>{stock.technicals}</p>
				</div>

				<div class="catalyst-section">
					<h4>Catalyst</h4>
					<p>{stock.catalyst}</p>
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.watchlist-page {
		background: #f5f7fa;
		min-height: 100vh;
		padding: 40px 30px;
	}

	.page-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 50px;
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

	.watchlist-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (min-width: 768px) {
		.watchlist-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1200px) {
		.watchlist-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.stock-card {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
		transition: all 0.3s ease;
		position: relative;
	}

	.stock-card:hover {
		transform: translateY(-5px);
		box-shadow: 0 12px 40px rgba(0,0,0,0.15);
	}

	.stock-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 15px;
	}

	.stock-title h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 5px 0;
		color: #143E59;
		font-family: 'Montserrat', sans-serif;
	}

	.company-name {
		font-size: 13px;
		color: #666;
	}

	.stock-price {
		text-align: right;
	}

	.current-price {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.price-change {
		font-size: 14px;
		font-weight: 600;
		margin-top: 4px;
	}

	.price-change.positive {
		color: #22c55e;
	}

	.price-change.negative {
		color: #ef4444;
	}

	.bias-badge {
		display: inline-block;
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 700;
		margin-bottom: 20px;
	}

	.bias--bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias--bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias--neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.trade-plan {
		background: #f8fafc;
		border-radius: 12px;
		padding: 15px;
		margin-bottom: 20px;
	}

	.plan-row {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #e5e7eb;
	}

	.plan-row:last-child {
		border-bottom: none;
	}

	.plan-row .label {
		font-size: 13px;
		color: #666;
		font-weight: 600;
	}

	.plan-row .value {
		font-size: 14px;
		font-weight: 700;
	}

	.plan-row .entry {
		color: #143E59;
	}

	.plan-row .target {
		color: #22c55e;
	}

	.plan-row .runner {
		color: #0ea5e9;
	}

	.plan-row .stop {
		color: #ef4444;
	}

	.options-section {
		background: #f3f4f6;
		border-radius: 8px;
		padding: 12px;
		margin-bottom: 15px;
	}

	.options-section h4 {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		margin: 0 0 8px 0;
	}

	.options-info {
		display: flex;
		justify-content: space-between;
		font-size: 14px;
		font-weight: 600;
		color: #7c3aed;
	}

	.status-badge {
		position: absolute;
		top: 20px;
		right: 20px;
		padding: 5px 12px;
		border-radius: 20px;
		font-size: 10px;
		font-weight: 700;
	}

	.status--active {
		background: #dcfce7;
		color: #166534;
	}

	.status--watching {
		background: #fef3c7;
		color: #92400e;
	}

	.status--closed {
		background: #f3f4f6;
		color: #6b7280;
	}

	.notes-section,
	.technicals-section,
	.catalyst-section {
		margin-bottom: 15px;
	}

	.notes-section h4,
	.technicals-section h4,
	.catalyst-section h4 {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #666;
		margin: 0 0 8px 0;
	}

	.notes-section p,
	.technicals-section p,
	.catalyst-section p {
		font-size: 13px;
		color: #555;
		line-height: 1.6;
		margin: 0;
	}
</style>
