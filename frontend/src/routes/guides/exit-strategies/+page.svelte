<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
</script>

<svelte:head>
	<title>Exit Strategies Guide | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="guide-page">
	<div class="guide-header">
		<h1>🎯 Exit Strategies for Swing Trading</h1>
		<p>Know when to take profits and protect gains</p>
	</div>

	<div class="guide-content">
		<section class="guide-section">
			<h2>The Scaling Out Method</h2>
			<p>The most effective exit strategy for swing trades is scaling out at multiple targets.</p>

			<div class="strategy-box">
				<h3>Standard Scaling Plan:</h3>
				<ul>
					<li><strong>33% at Target 1:</strong> Lock in quick profit, move stop to breakeven</li>
					<li>
						<strong>33% at Target 2:</strong> Secure more profit, trail stop below recent swing low
					</li>
					<li><strong>34% at Target 3 or Runner:</strong> Let winners run or take final profit</li>
				</ul>
			</div>

			<div class="example-box">
				<h4>Example: MSFT Trade</h4>
				<p>Entry: $425 (100 shares) | Stop: $415</p>
				<p>T1: $435 - Sell 33 shares (+$330) | Move stop to $425</p>
				<p>T2: $445 - Sell 33 shares (+$660) | Trail stop to $435</p>
				<p>T3: $460 - Sell 34 shares (+$1,190) | Total: +$2,180</p>
			</div>
		</section>

		<section class="guide-section">
			<h2>Trailing Stop Strategy</h2>
			<p>After hitting T1, use trailing stops to protect profits while allowing for upside.</p>

			<div class="strategy-box">
				<h3>Trailing Stop Methods:</h3>
				<ul>
					<li><strong>Percentage Trail:</strong> Trail stop 5-8% below current price</li>
					<li><strong>Swing Low Trail:</strong> Place stop below most recent swing low</li>
					<li><strong>Moving Average Trail:</strong> Trail below 20-day or 50-day MA</li>
					<li><strong>ATR Trail:</strong> Trail 1.5-2x ATR below current price</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Time-Based Exits</h2>
			<p>Sometimes the best exit is based on time, not price.</p>

			<div class="warning-box">
				<h3>Exit If:</h3>
				<ul>
					<li>Trade hasn't reached T1 within expected timeframe (5-7 days)</li>
					<li>Stock consolidates for too long without progress</li>
					<li>Momentum clearly shifts (volume dries up, lower highs)</li>
					<li>Better opportunity presents itself</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Breakeven Stop Management</h2>
			<p>Once T1 is hit, immediately move your stop to breakeven (or slightly above entry).</p>

			<div class="strategy-box">
				<h3>Benefits:</h3>
				<ul>
					<li>Eliminates risk of loss on remaining position</li>
					<li>Allows you to hold for bigger gains stress-free</li>
					<li>Protects profits already taken at T1</li>
					<li>Gives trade room to breathe</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Runner Strategy</h2>
			<p>For exceptional setups, let a small portion run for outsized gains.</p>

			<div class="example-box">
				<h4>Runner Rules:</h4>
				<p>• Only use 10-20% of position for runner</p>
				<p>• Must have already taken profits at T1 and T2</p>
				<p>• Trail stop aggressively (below each swing low)</p>
				<p>• Don't get greedy - take profit if momentum clearly shifts</p>
			</div>
		</section>

		<section class="guide-section">
			<h2>Common Exit Mistakes</h2>
			<div class="warning-box">
				<ul>
					<li>❌ Exiting entire position too early</li>
					<li>❌ Holding losers, cutting winners</li>
					<li>❌ Moving stop loss further away</li>
					<li>❌ Not taking profits at predetermined targets</li>
					<li>❌ Getting emotional about "round numbers"</li>
					<li>❌ Ignoring technical breakdown signals</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Exit Decision Flowchart</h2>
			<div class="flowchart-box">
				<div class="flow-step">1. Has T1 been hit? → Yes: Take 1/3 off, move stop to BE</div>
				<div class="flow-step">2. Has T2 been hit? → Yes: Take another 1/3 off, trail stop</div>
				<div class="flow-step">3. Has T3 been hit? → Yes: Take final 1/3 or let runner</div>
				<div class="flow-step">4. Is stop hit? → Yes: Exit immediately, no questions</div>
				<div class="flow-step">5. Time exceeded? → Yes: Evaluate and consider exit</div>
			</div>
		</section>
	</div>
</div>

<style>
	.guide-page {
		background: #f5f7fa;
		min-height: 100vh;
		padding: 40px 30px;
	}

	.guide-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 50px;
	}

	.guide-header h1 {
		font-size: 36px;
		font-weight: 700;
		margin: 0 0 12px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.guide-header p {
		font-size: 18px;
		color: #666;
		margin: 0;
	}

	.guide-content {
		max-width: 900px;
		margin: 0 auto;
		background: #fff;
		border-radius: 16px;
		padding: 40px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
	}

	.guide-section {
		margin-bottom: 50px;
	}

	.guide-section:last-child {
		margin-bottom: 0;
	}

	.guide-section h2 {
		font-size: 28px;
		font-weight: 700;
		color: #143e59;
		margin: 0 0 20px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.guide-section p {
		font-size: 16px;
		color: #555;
		line-height: 1.8;
		margin: 0 0 20px 0;
	}

	.strategy-box {
		background: #f8fafc;
		border-left: 4px solid #143e59;
		padding: 25px;
		border-radius: 8px;
		margin: 20px 0;
	}

	.strategy-box h3 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px 0;
	}

	.strategy-box ul {
		margin: 0;
		padding-left: 20px;
	}

	.strategy-box li {
		font-size: 15px;
		color: #555;
		line-height: 1.8;
		margin-bottom: 10px;
	}

	.strategy-box li strong {
		color: #143e59;
	}

	.example-box {
		background: #dcfce7;
		border-left: 4px solid #22c55e;
		padding: 20px;
		border-radius: 8px;
		margin: 20px 0;
	}

	.example-box h4 {
		font-size: 16px;
		font-weight: 700;
		color: #166534;
		margin: 0 0 10px 0;
	}

	.example-box p {
		font-size: 15px;
		color: #166534;
		line-height: 1.7;
		margin: 5px 0;
	}

	.warning-box {
		background: #fef3c7;
		border-left: 4px solid #f59e0b;
		padding: 25px;
		border-radius: 8px;
		margin: 20px 0;
	}

	.warning-box h3 {
		font-size: 16px;
		font-weight: 700;
		color: #92400e;
		margin: 0 0 15px 0;
	}

	.warning-box ul {
		margin: 0;
		padding-left: 20px;
	}

	.warning-box li {
		font-size: 15px;
		color: #92400e;
		line-height: 1.8;
		margin-bottom: 10px;
	}

	.flowchart-box {
		background: #dbeafe;
		border-left: 4px solid #3b82f6;
		padding: 25px;
		border-radius: 8px;
		margin: 20px 0;
	}

	.flow-step {
		font-size: 15px;
		color: #1e40af;
		padding: 12px 0;
		border-bottom: 1px solid #bfdbfe;
		font-weight: 600;
	}

	.flow-step:last-child {
		border-bottom: none;
	}

	@media (max-width: 768px) {
		.guide-content {
			padding: 25px;
		}

		.guide-header h1 {
			font-size: 28px;
		}

		.guide-section h2 {
			font-size: 22px;
		}
	}
</style>
