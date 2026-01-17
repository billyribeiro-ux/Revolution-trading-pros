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
	<div class="content-block">
		<h2 class="section-heading">Who is {trader.name.split(' ')[0]}?</h2>
	</div>
	<div class="content-block">
		<div class="content-text">
			<p>{trader.whoIs}</p>
			<p>{trader.tradingApproach}</p>
		</div>
	</div>

	<!-- Weekly Watchlist Section -->
	<WeeklyWatchlist />
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
	}

	.content-text p {
		margin: 0 0 15px;
	}

	.content-text p:last-child {
		margin-bottom: 0;
	}
</style>
