<script lang="ts">
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
</script>

<svelte:head>
	<title>Position Sizing Guide | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="guide-page">
	<div class="guide-header">
		<h1>💰 Position Sizing for Swing Trading</h1>
		<p>Calculate the perfect position size for every trade</p>
	</div>

	<div class="guide-content">
		<section class="guide-section">
			<h2>The Position Sizing Formula</h2>
			<div class="formula-box">
				<p><strong>Position Size = (Account Risk $) ÷ (Entry Price - Stop Loss Price)</strong></p>
			</div>
			<p>This formula ensures you never risk more than your predetermined amount per trade.</p>
		</section>

		<section class="guide-section">
			<h2>Step-by-Step Calculation</h2>

			<div class="strategy-box">
				<h3>Example Trade Setup:</h3>
				<ul>
					<li><strong>Account Size:</strong> $50,000</li>
					<li><strong>Risk Per Trade:</strong> 2% = $1,000</li>
					<li><strong>Entry Price:</strong> $142.50</li>
					<li><strong>Stop Loss:</strong> $136.00</li>
					<li><strong>Risk Per Share:</strong> $142.50 - $136.00 = $6.50</li>
				</ul>
				<div class="calculation">
					<p><strong>Position Size = $1,000 ÷ $6.50 = 153 shares</strong></p>
					<p><strong>Total Investment = 153 × $142.50 = $21,802.50</strong></p>
				</div>
			</div>
		</section>

		<section class="guide-section">
			<h2>Account Size Tiers</h2>
			<p>Position sizing varies based on account size. Here are recommended approaches:</p>

			<div class="tier-grid">
				<div class="tier-card">
					<h4>Small Account ($5,000 - $25,000)</h4>
					<ul>
						<li>Risk: 1-2% per trade</li>
						<li>Max Positions: 3-4</li>
						<li>Focus on higher probability setups</li>
						<li>Use options to control larger positions</li>
					</ul>
				</div>
				<div class="tier-card">
					<h4>Medium Account ($25,000 - $100,000)</h4>
					<ul>
						<li>Risk: 1.5-2% per trade</li>
						<li>Max Positions: 5-6</li>
						<li>Can diversify across sectors</li>
						<li>Mix of stocks and options</li>
					</ul>
				</div>
				<div class="tier-card">
					<h4>Large Account ($100,000+)</h4>
					<ul>
						<li>Risk: 1-2% per trade</li>
						<li>Max Positions: 6-8</li>
						<li>Full diversification possible</li>
						<li>Scale into positions</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="guide-section">
			<h2>Options Position Sizing</h2>
			<p>Options require different position sizing due to leverage and theta decay.</p>

			<div class="strategy-box">
				<h3>Options Sizing Rules:</h3>
				<ul>
					<li><strong>Max Risk:</strong> Same 2% rule applies to total premium paid</li>
					<li><strong>Contract Calculation:</strong> Risk $ ÷ (Premium × 100)</li>
					<li><strong>Time Decay:</strong> Account for theta - don't oversize</li>
					<li><strong>Expiration:</strong> Minimum 30-45 days for swing trades</li>
				</ul>
			</div>

			<div class="example-box">
				<h4>Options Example:</h4>
				<p>Account: $50,000 | Risk: $1,000 (2%)</p>
				<p>NVDA $145 Call @ $3.50 premium</p>
				<p>Contracts = $1,000 ÷ ($3.50 × 100) = 2.85 → Buy 2 contracts</p>
				<p>Total Risk = 2 × $3.50 × 100 = $700</p>
			</div>
		</section>

		<section class="guide-section">
			<h2>Scaling Into Positions</h2>
			<p>For larger accounts or volatile stocks, scale into positions to reduce risk.</p>

			<div class="strategy-box">
				<h3>Scaling Strategy:</h3>
				<ul>
					<li><strong>Initial Entry:</strong> 50% of calculated position</li>
					<li><strong>Add on Confirmation:</strong> 25% more if setup confirms</li>
					<li><strong>Final Add:</strong> Last 25% if momentum continues</li>
					<li><strong>Average Entry:</strong> Track your average entry price</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Portfolio Heat Management</h2>
			<p>Total portfolio risk should never exceed 10% at any given time.</p>

			<div class="warning-box">
				<h3>Portfolio Risk Rules:</h3>
				<ul>
					<li>Max 5 positions × 2% risk = 10% total portfolio risk</li>
					<li>If you have 5 open trades, don't open a 6th</li>
					<li>Close weakest position before opening new one</li>
					<li>Reduce position sizes if market volatility increases</li>
				</ul>
			</div>
		</section>

		<section class="guide-section">
			<h2>Position Sizing Calculator</h2>
			<div class="calculator-box">
				<h3>Quick Reference Table</h3>
				<table>
					<thead>
						<tr>
							<th>Account Size</th>
							<th>2% Risk</th>
							<th>$5 Risk/Share</th>
							<th>$10 Risk/Share</th>
						</tr>
					</thead>
					<tbody>
						<tr>
							<td>$10,000</td>
							<td>$200</td>
							<td>40 shares</td>
							<td>20 shares</td>
						</tr>
						<tr>
							<td>$25,000</td>
							<td>$500</td>
							<td>100 shares</td>
							<td>50 shares</td>
						</tr>
						<tr>
							<td>$50,000</td>
							<td>$1,000</td>
							<td>200 shares</td>
							<td>100 shares</td>
						</tr>
						<tr>
							<td>$100,000</td>
							<td>$2,000</td>
							<td>400 shares</td>
							<td>200 shares</td>
						</tr>
					</tbody>
				</table>
			</div>
		</section>

		<section class="guide-section">
			<h2>Common Position Sizing Mistakes</h2>
			<div class="warning-box">
				<ul>
					<li>❌ Risking same dollar amount regardless of setup quality</li>
					<li>❌ Oversizing because "you're confident"</li>
					<li>❌ Not adjusting for wider stops</li>
					<li>❌ Ignoring total portfolio exposure</li>
					<li>❌ Sizing based on how much you want to make</li>
					<li>❌ Not accounting for commissions and slippage</li>
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

	.formula-box {
		background: #143e59;
		color: #fff;
		padding: 30px;
		border-radius: 8px;
		text-align: center;
		margin: 20px 0;
	}

	.formula-box p {
		font-size: 20px;
		font-weight: 600;
		margin: 0;
		color: #fff;
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

	.calculation {
		background: #dcfce7;
		padding: 20px;
		border-radius: 8px;
		margin-top: 20px;
	}

	.calculation p {
		font-size: 16px;
		font-weight: 700;
		color: #166534;
		margin: 8px 0;
	}

	.tier-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
		margin: 20px 0;
	}

	@media (min-width: 768px) {
		.tier-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.tier-card {
		background: #f8fafc;
		border: 2px solid #e5e7eb;
		border-radius: 12px;
		padding: 20px;
	}

	.tier-card h4 {
		font-size: 16px;
		font-weight: 700;
		color: #143e59;
		margin: 0 0 15px 0;
	}

	.tier-card ul {
		margin: 0;
		padding-left: 20px;
	}

	.tier-card li {
		font-size: 14px;
		color: #555;
		line-height: 1.7;
		margin-bottom: 8px;
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

	.calculator-box {
		background: #f8fafc;
		border-radius: 12px;
		padding: 25px;
		margin: 20px 0;
		overflow-x: auto;
	}

	.calculator-box h3 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px 0;
		text-align: center;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	thead {
		background: #143e59;
		color: #fff;
	}

	th {
		padding: 12px;
		text-align: left;
		font-size: 13px;
		font-weight: 700;
	}

	td {
		padding: 12px;
		border-bottom: 1px solid #e5e7eb;
		font-size: 14px;
		color: #555;
	}

	tbody tr:hover {
		background: #f8fafc;
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

		.tier-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
