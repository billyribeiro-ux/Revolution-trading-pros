<!--
	Chart Setups Sub-Page
	═══════════════════════════════════════════════════════════════════════════
	Trader-specific chart setups content. Uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';

	interface ChartSetup {
		id: string;
		title: string;
		description: string;
		image: string;
		date: string;
		symbol: string;
		type: 'Bullish' | 'Bearish' | 'Neutral';
	}

	let trader = $derived(getTraderBySlug(page.params.slug));

	// Trader-specific chart setups - in production, fetch from API
	const chartSetups: ChartSetup[] = [
		{
			id: '1',
			title: 'SPY Squeeze Pro Setup',
			description: 'Looking for a breakout above resistance with squeeze firing.',
			image: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/08/22154121/Squeeze-Pro-Trailer-Cardxx-300x169.jpg',
			date: 'January 3, 2026',
			symbol: 'SPY',
			type: 'Bullish'
		},
		{
			id: '2',
			title: 'QQQ Momentum Play',
			description: 'Tech sector showing strength, watching for continuation.',
			image: 'https://cdn.simplertrading.com/2022/08/07092727/Quick-Hits-Strategy-Video-bg-300x169.jpg',
			date: 'January 2, 2026',
			symbol: 'QQQ',
			type: 'Bullish'
		},
		{
			id: '3',
			title: 'TSLA Consolidation Break',
			description: 'Tesla forming a tight consolidation pattern near key levels.',
			image: 'https://cdn.simplertrading.com/2023/03/21162031/Micro-voodoo-lines-video-bg-300x169.jpg',
			date: 'January 2, 2026',
			symbol: 'TSLA',
			type: 'Neutral'
		}
	];
</script>

<!-- Page Content -->
{#if trader}
	<div class="fl-module fl-module-heading">
		<div class="fl-module-content fl-node-content">
			<h2 class="fl-heading">
				<span class="fl-heading-text section-title">{trader.name.split(' ')[0]}'s Chart Setups</span>
			</h2>
		</div>
	</div>
	<div class="fl-module fl-module-rich-text">
		<div class="fl-module-content fl-node-content">
			<div class="fl-rich-text">
				<p>Explore the latest chart setups and trading ideas from {trader.name}. These setups showcase key levels, patterns, and opportunities in the markets.</p>
			</div>
		</div>
	</div>

	<!-- Chart Setups Grid -->
	<div class="chart-setups-grid">
		{#each chartSetups as setup (setup.id)}
			<article class="chart-setup-card">
				<figure class="card-media">
					<div class="card-image" style="background-image: url({setup.image});">
						<span class="setup-type" class:bullish={setup.type === 'Bullish'} class:bearish={setup.type === 'Bearish'}>{setup.type}</span>
					</div>
				</figure>
				<div class="card-body">
					<div class="card-meta">
						<span class="symbol">{setup.symbol}</span>
						<span class="date">{setup.date}</span>
					</div>
					<h3 class="card-title">{setup.title}</h3>
					<p class="card-description">{setup.description}</p>
					<button class="btn btn-xs btn-default">View Setup</button>
				</div>
			</article>
		{/each}
	</div>

	{#if chartSetups.length === 0}
		<div class="empty-state">
			<p>No chart setups available at this time. Check back soon!</p>
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

	.chart-setups-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 25px; margin-top: 20px; }

	.chart-setup-card { background: #fff; border: 1px solid #e6e6e6; border-radius: 8px; overflow: hidden; transition: box-shadow 0.3s ease; }
	.chart-setup-card:hover { box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1); }

	.card-media { margin: 0; padding: 0; position: relative; height: 160px; overflow: hidden; }
	.card-image { display: block; width: 100%; height: 100%; background-size: cover; background-position: center; background-repeat: no-repeat; position: relative; }

	.setup-type { position: absolute; top: 10px; left: 10px; padding: 4px 12px; background: #666; color: #fff; font-size: 11px; font-weight: 600; border-radius: 3px; text-transform: uppercase; }
	.setup-type.bullish { background: #28a745; }
	.setup-type.bearish { background: #dc3545; }

	.card-body { padding: 20px; }
	.card-meta { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; font-size: 12px; }
	.symbol { background: #143E59; color: #fff; padding: 3px 8px; border-radius: 3px; font-weight: 600; }
	.date { color: #999; }
	.card-title { font-size: 18px; font-weight: 700; color: #333; margin: 0 0 10px; }
	.card-description { font-size: 14px; color: #666; line-height: 1.5; margin: 0 0 15px; }

	.btn { display: inline-block; padding: 10px 20px; font-size: 14px; font-weight: 600; text-align: center; text-decoration: none; border-radius: 4px; transition: all 0.2s ease; cursor: pointer; }
	.btn-xs { padding: 8px 16px; font-size: 12px; }
	.btn-default { background: #143E59; color: #fff; border: 1px solid #143E59; }
	.btn-default:hover { background: #0c2638; border-color: #0c2638; }

	.empty-state { text-align: center; padding: 60px 20px; color: #666; }

	/* Mobile-first: 1 column by default, 2 on md+, 3 on lg+ */
	.chart-setups-grid { grid-template-columns: 1fr; }

	@media (min-width: 768px) {
		.chart-setups-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (min-width: 992px) {
		.chart-setups-grid { grid-template-columns: repeat(3, 1fr); }
	}
</style>
