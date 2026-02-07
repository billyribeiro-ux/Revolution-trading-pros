<!--
	Trading Plan Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific trading plan content. Uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';

	interface TradingPlanSection {
		title: string;
		content: string[];
	}

	let trader = $derived(getTraderBySlug(page.params.slug ?? ''));

	// Trader-specific trading plan - in production, fetch from API
	const tradingPlanSections: TradingPlanSection[] = [
		{
			title: 'Trading Philosophy',
			content: [
				'Focus on high-probability setups with defined risk',
				'Never risk more than 2% of account on a single trade',
				'Trade with the trend, not against it',
				'Patience is the key to consistent profitability'
			]
		},
		{
			title: 'Pre-Market Routine',
			content: [
				'Review overnight futures and international markets',
				'Check economic calendar for scheduled events',
				'Identify key support and resistance levels',
				'Set up watchlist with potential trade candidates'
			]
		},
		{
			title: 'Entry Criteria',
			content: [
				'Wait for confirmation before entering trades',
				'Use multiple timeframe analysis',
				'Look for confluence of technical indicators',
				'Ensure proper risk-to-reward ratio (minimum 1:2)'
			]
		},
		{
			title: 'Risk Management',
			content: [
				'Always use stop losses on every trade',
				'Position size based on account risk tolerance',
				'Scale out of winning positions',
				'Never add to losing positions'
			]
		},
		{
			title: 'Post-Market Review',
			content: [
				'Journal all trades with screenshots',
				"Analyze what worked and what didn't",
				'Track performance metrics weekly',
				'Continuously refine and improve the plan'
			]
		}
	];
</script>

<!-- Page Content -->
{#if trader}
	<div class="content-block">
		<h2 class="section-heading">{trader.name.split(' ')[0]}'s Trading Plan</h2>
	</div>
	<div class="content-block">
		<div class="content-text">
			<p>
				A comprehensive look at {trader.name}'s trading plan and methodology. Learn the principles
				and routines that guide successful trading.
			</p>
		</div>
	</div>

	<!-- Trading Plan Sections -->
	<div class="trading-plan-sections">
		{#each tradingPlanSections as section, index}
			<div class="plan-section">
				<div class="section-number">{index + 1}</div>
				<div class="section-content">
					<h3 class="plan-section-title">{section.title}</h3>
					<ul class="section-list">
						{#each section.content as item}
							<li>{item}</li>
						{/each}
					</ul>
				</div>
			</div>
		{/each}
	</div>

	<div class="plan-cta">
		<h3>Want to learn more about {trader.name.split(' ')[0]}'s trading approach?</h3>
		<p>Check out the courses and resources available in the Trader Store.</p>
		<a
			href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}/trader-store"
			class="btn btn-orange">Visit Trader Store</a
		>
	</div>
{/if}

<style>
	.content-block {
		margin-bottom: 20px;
	}
	.content-block:last-child {
		margin-bottom: 0;
	}

	.section-heading {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		line-height: 1.2;
	}

	.content-text {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
		margin-bottom: 30px;
	}
	.content-text p {
		margin: 0;
	}

	.trading-plan-sections {
		display: flex;
		flex-direction: column;
		gap: 25px;
		margin-bottom: 40px;
	}

	.plan-section {
		display: flex;
		gap: 20px;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		padding: 25px;
		transition: box-shadow 0.3s ease;
	}
	.plan-section:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.08);
	}

	.section-number {
		flex-shrink: 0;
		width: 50px;
		height: 50px;
		background: #143e59;
		color: #fff;
		font-size: 24px;
		font-weight: 700;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.section-content {
		flex: 1;
	}
	.plan-section-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
	}

	.section-list {
		margin: 0;
		padding: 0 0 0 20px;
	}
	.section-list li {
		font-size: 15px;
		color: #555;
		line-height: 1.8;
		margin-bottom: 8px;
	}
	.section-list li:last-child {
		margin-bottom: 0;
	}

	.plan-cta {
		background: linear-gradient(135deg, #143e59 0%, #0c2638 100%);
		color: #fff;
		padding: 40px;
		border-radius: 8px;
		text-align: center;
	}
	.plan-cta h3 {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 10px;
	}
	.plan-cta p {
		font-size: 16px;
		opacity: 0.9;
		margin: 0 0 20px;
	}

	.btn {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}
	.btn-orange {
		background: #f69532;
		color: #fff;
		border: 1px solid #f69532;
	}
	.btn-orange:hover {
		background: #dc7309;
		border-color: #dc7309;
	}

	/* Mobile-first: stacked by default, row on md+ */
	.plan-section {
		flex-direction: column;
		align-items: center;
		text-align: center;
	}
	.section-list {
		padding-left: 0;
		list-style-position: inside;
	}

	@media (min-width: 768px) {
		.plan-section {
			flex-direction: row;
			align-items: flex-start;
			text-align: left;
		}
		.section-list {
			padding-left: 20px;
			list-style-position: outside;
		}
	}
</style>
