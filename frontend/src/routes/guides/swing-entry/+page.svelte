<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
</script>

<svelte:head>
	<title>Entry Strategies Guide | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<div class="guide-page">
	<div class="guide-header">
		<h1>📖 Entry Strategies for Swing Trading</h1>
		<p>Master the art of timing your entries for maximum profit potential</p>
	</div>

	<div class="guide-content">
		<section class="guide-section">
			<h2>1. Breakout Entry Strategy</h2>
			<p>The breakout entry is one of the most powerful swing trading strategies. Here's how to execute it properly:</p>
			
			<div class="strategy-box">
				<h3>Key Components:</h3>
				<ul>
					<li><strong>Consolidation Pattern:</strong> Look for 3-5 days of tight consolidation</li>
					<li><strong>Volume Confirmation:</strong> Entry volume should be 1.5x average</li>
					<li><strong>Price Action:</strong> Clean break above resistance with strong candle</li>
					<li><strong>Timing:</strong> Enter on the breakout candle or first pullback</li>
				</ul>
			</div>

			<div class="example-box">
				<h4>Example: NVDA Breakout</h4>
				<p>NVDA consolidated between $138-$142 for 4 days. On day 5, it broke above $142 with 2x average volume. Entry at $142.50 with stop at $136 (below consolidation).</p>
			</div>
		</section>

		<section class="guide-section">
			<h2>2. Pullback Entry Strategy</h2>
			<p>Entering on pullbacks in an uptrend offers better risk/reward ratios:</p>
			
			<div class="strategy-box">
				<h3>Key Components:</h3>
				<ul>
					<li><strong>Established Trend:</strong> Stock must be in clear uptrend</li>
					<li><strong>Support Levels:</strong> Identify key support zones (moving averages, prior highs)</li>
					<li><strong>Reversal Signals:</strong> Look for bullish candles at support</li>
					<li><strong>Volume:</strong> Decreasing on pullback, increasing on bounce</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>3. Momentum Entry Strategy</h2>
			<p>Riding strong momentum can lead to explosive gains:</p>
			
			<div class="strategy-box">
				<h3>Key Components:</h3>
				<ul>
					<li><strong>Strong Trend:</strong> Stock making higher highs and higher lows</li>
					<li><strong>Relative Strength:</strong> Outperforming the market/sector</li>
					<li><strong>Volume Surge:</strong> Consistent above-average volume</li>
					<li><strong>Catalyst:</strong> News, earnings, or sector rotation driving momentum</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>4. Earnings Play Entry</h2>
			<p>Capitalizing on earnings catalysts requires specific timing:</p>
			
			<div class="strategy-box">
				<h3>Key Components:</h3>
				<ul>
					<li><strong>Pre-Earnings Setup:</strong> Enter 1-3 days before earnings</li>
					<li><strong>Historical Pattern:</strong> Study past earnings reactions</li>
					<li><strong>Options Strategy:</strong> Use options to define risk</li>
					<li><strong>Exit Plan:</strong> Have clear plan for both scenarios</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Entry Checklist</h2>
			<div class="checklist-box">
				<div class="checklist-item">✓ Trend direction confirmed</div>
				<div class="checklist-item">✓ Volume confirms the move</div>
				<div class="checklist-item">✓ Entry price defined</div>
				<div class="checklist-item">✓ Stop loss placed</div>
				<div class="checklist-item">✓ Target prices set (T1, T2, T3)</div>
				<div class="checklist-item">✓ Position size calculated</div>
				<div class="checklist-item">✓ Risk/reward ratio acceptable (minimum 2:1)</div>
			</div>
		</section>

		<section class="guide-section">
			<h2>Common Entry Mistakes to Avoid</h2>
			<div class="warning-box">
				<ul>
					<li>❌ Chasing price after big move</li>
					<li>❌ Entering without stop loss</li>
					<li>❌ Ignoring volume confirmation</li>
					<li>❌ Oversizing position</li>
					<li>❌ Entering on hope vs. setup</li>
					<li>❌ Not waiting for confirmation</li>
				</ul>
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
		box-shadow: 0 4px 20px rgba(0,0,0,0.08);
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
		color: #143E59;
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
		border-left: 4px solid #143E59;
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
		color: #143E59;
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
		margin: 0;
	}

	.checklist-box {
		background: #f8fafc;
		border-radius: 8px;
		padding: 25px;
	}

	.checklist-item {
		font-size: 16px;
		color: #333;
		padding: 12px 0;
		border-bottom: 1px solid #e5e7eb;
	}

	.checklist-item:last-child {
		border-bottom: none;
	}

	.warning-box {
		background: #fef3c7;
		border-left: 4px solid #f59e0b;
		padding: 25px;
		border-radius: 8px;
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
