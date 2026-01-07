<!--
	Trading Strategies Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific trading strategies content. Uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';

	interface Strategy {
		id: string;
		name: string;
		description: string;
		difficulty: 'Beginner' | 'Intermediate' | 'Advanced';
		timeframe: string;
		markets: string[];
		keyPoints: string[];
	}

	let trader = $derived(getTraderBySlug(page.params.slug));

	// Trader-specific strategies - in production, fetch from API
	const strategies: Strategy[] = [
		{
			id: '1',
			name: 'Squeeze Pro Momentum',
			description: 'Identify explosive momentum moves using the Squeeze Pro indicator. This strategy focuses on catching breakouts after periods of low volatility consolidation.',
			difficulty: 'Intermediate',
			timeframe: 'Daily / 4-Hour',
			markets: ['SPY', 'QQQ', 'Individual Stocks'],
			keyPoints: [
				'Wait for squeeze to fire (dots change color)',
				'Confirm with momentum histogram direction',
				'Enter on first pullback after breakout',
				'Use ATR-based stops for position sizing'
			]
		},
		{
			id: '2',
			name: 'Voodoo Lines Support/Resistance',
			description: 'Use proprietary Voodoo Lines to identify key support and resistance levels that institutional traders watch.',
			difficulty: 'Beginner',
			timeframe: 'All Timeframes',
			markets: ['All Markets'],
			keyPoints: [
				'Identify major Voodoo Line levels',
				'Watch for price reaction at these levels',
				'Look for confluence with other indicators',
				'Trade bounces or breaks with confirmation'
			]
		},
		{
			id: '3',
			name: 'Options Momentum Swing',
			description: 'Capture multi-day momentum moves using options for leveraged exposure with defined risk.',
			difficulty: 'Advanced',
			timeframe: 'Daily',
			markets: ['High Beta Stocks', 'ETFs'],
			keyPoints: [
				'Select options 30-45 DTE for optimal theta decay',
				'Use delta 0.50-0.70 for good leverage',
				'Scale into positions on pullbacks',
				'Take profits at 50-100% gain'
			]
		}
	];

	function getDifficultyColor(difficulty: string): string {
		switch (difficulty) {
			case 'Beginner': return '#28a745';
			case 'Intermediate': return '#ffc107';
			case 'Advanced': return '#dc3545';
			default: return '#666';
		}
	}
</script>

<!-- Page Content -->
{#if trader}
	<div class="fl-module fl-module-heading">
		<div class="fl-module-content fl-node-content">
			<h2 class="fl-heading">
				<span class="fl-heading-text section-title">{trader.name.split(' ')[0]}'s Trading Strategies</span>
			</h2>
		</div>
	</div>
	<div class="fl-module fl-module-rich-text">
		<div class="fl-module-content fl-node-content">
			<div class="fl-rich-text">
				<p>Explore the trading strategies developed and refined by {trader.name}. Each strategy includes detailed entry and exit criteria, risk management rules, and best practices.</p>
			</div>
		</div>
	</div>

	<!-- Strategies Grid -->
	<div class="strategies-grid">
		{#each strategies as strategy (strategy.id)}
			<article class="strategy-card">
				<header class="card-header">
					<h3 class="strategy-name">{strategy.name}</h3>
					<span class="difficulty-badge" style="background-color: {getDifficultyColor(strategy.difficulty)}">
						{strategy.difficulty}
					</span>
				</header>
				<div class="card-body">
					<p class="strategy-description">{strategy.description}</p>
					
					<div class="strategy-meta">
						<div class="meta-item">
							<span class="meta-label">Timeframe</span>
							<span class="meta-value">{strategy.timeframe}</span>
						</div>
						<div class="meta-item">
							<span class="meta-label">Markets</span>
							<span class="meta-value">{strategy.markets.join(', ')}</span>
						</div>
					</div>

					<div class="key-points">
						<h4>Key Points</h4>
						<ul>
							{#each strategy.keyPoints as point}
								<li>{point}</li>
							{/each}
						</ul>
					</div>
				</div>
				<footer class="card-footer">
					<a href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}/trader-store" class="btn btn-xs btn-default">Learn This Strategy</a>
				</footer>
			</article>
		{/each}
	</div>

	{#if strategies.length === 0}
		<div class="empty-state">
			<p>No strategies available at this time. Check back soon!</p>
		</div>
	{/if}
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

	.strategies-grid { display: grid; grid-template-columns: repeat(1, 1fr); gap: 25px; margin-top: 20px; }

	.strategy-card { background: #fff; border: 1px solid #e6e6e6; border-radius: 8px; overflow: hidden; transition: box-shadow 0.3s ease; }
	.strategy-card:hover { box-shadow: 0 5px 20px rgba(0, 0, 0, 0.08); }

	.card-header { display: flex; justify-content: space-between; align-items: center; padding: 20px; background: #f9f9f9; border-bottom: 1px solid #e6e6e6; }
	.strategy-name { font-size: 20px; font-weight: 700; color: #333; margin: 0; }
	.difficulty-badge { display: inline-block; padding: 5px 12px; color: #fff; font-size: 11px; font-weight: 600; border-radius: 3px; text-transform: uppercase; }

	.card-body { padding: 20px; }
	.strategy-description { font-size: 15px; color: #555; line-height: 1.7; margin: 0 0 20px; }

	.strategy-meta { display: flex; gap: 30px; margin-bottom: 20px; padding-bottom: 20px; border-bottom: 1px solid #e6e6e6; }
	.meta-item { display: flex; flex-direction: column; gap: 4px; }
	.meta-label { font-size: 11px; text-transform: uppercase; color: #999; font-weight: 600; }
	.meta-value { font-size: 14px; color: #333; font-weight: 500; }

	.key-points h4 { font-size: 14px; font-weight: 700; color: #333; margin: 0 0 10px; text-transform: uppercase; }
	.key-points ul { margin: 0; padding: 0 0 0 20px; }
	.key-points li { font-size: 14px; color: #555; line-height: 1.8; margin-bottom: 5px; }
	.key-points li:last-child { margin-bottom: 0; }

	.card-footer { padding: 15px 20px; background: #fafafa; border-top: 1px solid #e6e6e6; }

	.btn { display: inline-block; padding: 10px 20px; font-size: 14px; font-weight: 600; text-align: center; text-decoration: none; border-radius: 4px; transition: all 0.2s ease; cursor: pointer; }
	.btn-xs { padding: 8px 16px; font-size: 12px; }
	.btn-default { background: #143E59; color: #fff; border: 1px solid #143E59; }
	.btn-default:hover { background: #0c2638; border-color: #0c2638; }

	.empty-state { text-align: center; padding: 60px 20px; color: #666; }

	/* Mobile-first: stacked by default, row on md+ */
	.card-header { flex-direction: column; gap: 10px; text-align: center; }
	.strategy-meta { flex-direction: column; gap: 15px; }

	@media (min-width: 768px) {
		.card-header { flex-direction: row; gap: 20px; text-align: left; }
		.strategy-meta { flex-direction: row; gap: 30px; }
	}
</style>
