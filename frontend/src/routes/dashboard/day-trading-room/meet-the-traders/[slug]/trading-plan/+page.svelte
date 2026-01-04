<!--
	Trading Plan Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific trading plan content. Uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/stores';
	import { getTraderBySlug } from '$lib/data/traders';

	interface TradingPlanSection {
		title: string;
		content: string[];
	}

	$: trader = getTraderBySlug($page.params.slug);

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
				'Analyze what worked and what didn\'t',
				'Track performance metrics weekly',
				'Continuously refine and improve the plan'
			]
		}
	];
</script>

<!-- Page Content -->
{#if trader}
	<div class="fl-module fl-module-heading">
		<div class="fl-module-content fl-node-content">
			<h2 class="fl-heading">
				<span class="fl-heading-text section-title">{trader.name.split(' ')[0]}'s Trading Plan</span>
			</h2>
		</div>
	</div>
	<div class="fl-module fl-module-rich-text">
		<div class="fl-module-content fl-node-content">
			<div class="fl-rich-text">
				<p>A comprehensive look at {trader.name}'s trading plan and methodology. Learn the principles and routines that guide successful trading.</p>
			</div>
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
		<a href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}/trader-store" class="btn btn-orange">Visit Trader Store</a>
	</div>
{/if}

<style>
	.fl-module { margin-bottom: 20px; }
	.fl-module:last-child { margin-bottom: 0; }
	.fl-module-content { position: relative; }
	.fl-node-content { position: relative; }

	.fl-heading { margin: 0; padding: 0; line-height: 1.2; }
	.fl-heading-text { display: block; }
	.fl-heading-text.section-title { font-size: 24px; font-weight: 700; color: #333; margin-bottom: 15px; }

	.fl-rich-text { font-size: 16px; line-height: 1.7; color: #444; margin-bottom: 30px; }
	.fl-rich-text p { margin: 0; }

	.trading-plan-sections { display: flex; flex-direction: column; gap: 25px; margin-bottom: 40px; }

	.plan-section { display: flex; gap: 20px; background: #fff; border: 1px solid #e6e6e6; border-radius: 8px; padding: 25px; transition: box-shadow 0.3s ease; }
	.plan-section:hover { box-shadow: 0 5px 20px rgba(0, 0, 0, 0.08); }

	.section-number { flex-shrink: 0; width: 50px; height: 50px; background: #143E59; color: #fff; font-size: 24px; font-weight: 700; border-radius: 50%; display: flex; align-items: center; justify-content: center; }

	.section-content { flex: 1; }
	.plan-section-title { font-size: 20px; font-weight: 700; color: #333; margin: 0 0 15px; }

	.section-list { margin: 0; padding: 0 0 0 20px; }
	.section-list li { font-size: 15px; color: #555; line-height: 1.8; margin-bottom: 8px; }
	.section-list li:last-child { margin-bottom: 0; }

	.plan-cta { background: linear-gradient(135deg, #143E59 0%, #0c2638 100%); color: #fff; padding: 40px; border-radius: 8px; text-align: center; }
	.plan-cta h3 { font-size: 24px; font-weight: 700; margin: 0 0 10px; }
	.plan-cta p { font-size: 16px; opacity: 0.9; margin: 0 0 20px; }

	.btn { display: inline-block; padding: 12px 24px; font-size: 14px; font-weight: 600; text-align: center; text-decoration: none; border-radius: 4px; transition: all 0.2s ease; cursor: pointer; }
	.btn-orange { background: #F69532; color: #fff; border: 1px solid #F69532; }
	.btn-orange:hover { background: #dc7309; border-color: #dc7309; }

	@media (max-width: 767px) {
		.plan-section { flex-direction: column; align-items: center; text-align: center; }
		.section-list { padding-left: 0; list-style-position: inside; }
	}
</style>
