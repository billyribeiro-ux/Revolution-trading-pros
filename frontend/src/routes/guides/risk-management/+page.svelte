<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
</script>

<svelte:head>
	<title>Risk Management Guide | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="guide-page">
	<div class="guide-header">
		<h1>🛡️ Risk Management for Swing Trading</h1>
		<p>Protect your capital and trade with confidence</p>
	</div>

	<div class="guide-content">
		<section class="guide-section">
			<h2>The 2% Rule</h2>
			<p>
				Never risk more than 2% of your total account on any single trade. This is the foundation of
				proper risk management.
			</p>

			<div class="strategy-box">
				<h3>Example Calculation:</h3>
				<ul>
					<li>Account Size: $50,000</li>
					<li>Max Risk Per Trade: $1,000 (2%)</li>
					<li>Entry: $142.50</li>
					<li>Stop Loss: $136.00</li>
					<li>Risk Per Share: $6.50</li>
					<li>Position Size: $1,000 ÷ $6.50 = 153 shares</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Position Sizing Formula</h2>
			<div class="formula-box">
				<p><strong>Shares to Buy = (Account Size × Risk %) ÷ (Entry Price - Stop Loss)</strong></p>
			</div>
		</section>

		<section class="guide-section">
			<h2>Stop Loss Placement</h2>
			<div class="strategy-box">
				<h3>Key Principles:</h3>
				<ul>
					<li><strong>Technical Stops:</strong> Below support, consolidation, or swing lows</li>
					<li><strong>Percentage Stops:</strong> 5-8% for swing trades</li>
					<li><strong>ATR Stops:</strong> 1.5-2x Average True Range</li>
					<li><strong>Time Stops:</strong> Exit if setup doesn't work within expected timeframe</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Risk/Reward Ratios</h2>
			<p>Always aim for minimum 2:1 risk/reward. Preferably 3:1 or higher.</p>
			<div class="example-box">
				<h4>Example:</h4>
				<p>Entry: $142.50 | Stop: $136.00 | Risk: $6.50</p>
				<p>Target 1: $148.00 | Reward: $5.50 | R/R: 0.85:1 ❌</p>
				<p>Target 2: $152.00 | Reward: $9.50 | R/R: 1.46:1 ⚠️</p>
				<p>Target 3: $158.00 | Reward: $15.50 | R/R: 2.38:1 ✅</p>
			</div>
		</section>

		<section class="guide-section">
			<h2>Scaling Out Strategy</h2>
			<div class="strategy-box">
				<h3>Recommended Approach:</h3>
				<ul>
					<li><strong>Target 1 (T1):</strong> Take 1/3 position off, move stop to breakeven</li>
					<li><strong>Target 2 (T2):</strong> Take another 1/3 off, trail stop</li>
					<li><strong>Target 3 (T3):</strong> Take final 1/3 off or let runner</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Maximum Exposure Rules</h2>
			<div class="warning-box">
				<ul>
					<li>Never have more than 6-8 positions open simultaneously</li>
					<li>Limit sector exposure to 30% of portfolio</li>
					<li>Don't add to losing positions</li>
					<li>Take a break after 3 consecutive losses</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Risk Management Checklist</h2>
			<div class="checklist-box">
				<div class="checklist-item">✓ Position size calculated using 2% rule</div>
				<div class="checklist-item">✓ Stop loss placed immediately after entry</div>
				<div class="checklist-item">✓ Risk/reward ratio is minimum 2:1</div>
				<div class="checklist-item">✓ Total portfolio risk under 10%</div>
				<div class="checklist-item">✓ Exit strategy defined before entry</div>
				<div class="checklist-item">✓ No emotional attachment to position</div>
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

	.formula-box {
		background: #143e59;
		color: #fff;
		padding: 25px;
		border-radius: 8px;
		text-align: center;
		margin: 20px 0;
	}

	.formula-box p {
		font-size: 18px;
		font-weight: 600;
		margin: 0;
		color: #fff;
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
		background: #fee2e2;
		border-left: 4px solid #ef4444;
		padding: 25px;
		border-radius: 8px;
	}

	.warning-box ul {
		margin: 0;
		padding-left: 20px;
	}

	.warning-box li {
		font-size: 15px;
		color: #991b1b;
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
