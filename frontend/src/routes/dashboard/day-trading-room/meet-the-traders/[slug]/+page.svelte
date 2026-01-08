<!--
	Trader Overview Page (Default Landing)
	═══════════════════════════════════════════════════════════════════════════
	Overview content for trader profile - uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { getTraderBySlug } from '$lib/data/traders';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';

	let trader = $derived(getTraderBySlug(page.params.slug));
</script>

<!-- Who is Section -->
{#if trader}
	<div class="fl-module fl-module-heading">
		<div class="fl-module-content fl-node-content">
			<h2 class="fl-heading">
				<span class="fl-heading-text section-title">Who is {trader.name.split(' ')[0]}?</span>
			</h2>
		</div>
	</div>
	<div class="fl-module fl-module-rich-text">
		<div class="fl-module-content fl-node-content">
			<div class="fl-rich-text">
				<p>{trader.whoIs}</p>
				<p>{trader.tradingApproach}</p>
			</div>
		</div>
	</div>

	<!-- Weekly Watchlist Section -->
	<WeeklyWatchlist />
{/if}

<style>
	.fl-module {
		margin-bottom: 20px;
	}

	.fl-module:last-child {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	.fl-node-content {
		position: relative;
	}

	.fl-heading {
		margin: 0;
		padding: 0;
		line-height: 1.2;
	}

	.fl-heading-text {
		display: block;
	}

	.fl-heading-text.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin-bottom: 15px;
	}

	.fl-rich-text {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.fl-rich-text p {
		margin: 0 0 15px;
	}

	.fl-rich-text p:last-child {
		margin-bottom: 0;
	}
</style>
