<!--
	Trader Overview Page (Default Landing)
	═══════════════════════════════════════════════════════════════════════════
	Overview content for trader profile - uses shared layout.
	
	@version 2.0.0
-->
<script lang="ts">
	import { page } from '$app/stores';
	import { getTraderBySlug } from '$lib/data/traders';

	$: trader = getTraderBySlug($page.params.slug);
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
	<div class="weekly-watchlist-section">
		<div class="row">
			<div class="col-left">
				<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
				<div class="mobile-image">
					<a href="/dashboard/day-trading-room/weekly-watchlist">
						<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
					</a>
				</div>
				<h4 class="h5 u--font-weight-bold">Weekly Watchlist with David Starr</h4>
				<div class="u--hide-read-more">
					<p>Week of December 29, 2025.</p>
				</div>
				<a href="/dashboard/day-trading-room/weekly-watchlist" class="btn btn-tiny btn-default">Watch Now</a>
			</div>
			<div class="col-right desktop-only">
				<a href="/dashboard/day-trading-room/weekly-watchlist">
					<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
				</a>
			</div>
		</div>
	</div>
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

	/* Weekly Watchlist Section */
	.weekly-watchlist-section {
		margin-top: 40px;
		padding-top: 40px;
		border-top: 1px solid #e6e6e6;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-left {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
		padding: 0 15px;
	}

	.col-right {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
		padding: 0 15px;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 3px solid #F69532;
	}

	.h5 {
		font-size: 18px;
		margin: 0 0 10px;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
		height: auto;
	}

	.u--hide-read-more p {
		color: #666;
		font-size: 14px;
		margin: 0 0 15px;
	}

	.mobile-image {
		display: none;
		margin-bottom: 15px;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.btn-default {
		background: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	@media (max-width: 991px) {
		.col-left,
		.col-right {
			flex: 0 0 50%;
			max-width: 50%;
		}

		.desktop-only {
			display: none;
		}

		.mobile-image {
			display: block;
		}
	}

	@media (max-width: 767px) {
		.col-left,
		.col-right {
			flex: 0 0 100%;
			max-width: 100%;
			margin-bottom: 20px;
		}

		.col-right:last-child {
			margin-bottom: 0;
		}
	}
</style>
