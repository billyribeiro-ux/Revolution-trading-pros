<!--
	Trader Layout
	═══════════════════════════════════════════════════════════════════════════
	Shared layout for all trader sub-pages.
	Displays header and navigation consistently across all pages.
	
	@version 1.0.0
-->
<script lang="ts">
	import { page } from '$app/stores';
	import { getTraderBySlug, type Trader } from '$lib/data/traders';
	import TraderHeader from '$lib/components/traders/TraderHeader.svelte';
	import TraderNav from '$lib/components/traders/TraderNav.svelte';

	let trader: Trader | undefined;
	let currentPath: string = '';

	$: {
		const slug = $page.params.slug;
		trader = getTraderBySlug(slug);
		
		// Determine current sub-path
		const fullPath = $page.url.pathname;
		const basePath = `/dashboard/day-trading-room/meet-the-traders/${slug}`;
		currentPath = fullPath.replace(basePath, '') || '';
	}
</script>

<svelte:head>
	{#if trader}
		<title>{trader.name} | Meet the Traders | Revolution Trading Pros</title>
		<meta name="description" content="{trader.name} - {trader.title}. {trader.quote}" />
	{:else}
		<title>Trader Not Found | Revolution Trading Pros</title>
	{/if}
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if trader}
				<!-- Trader Header -->
				<TraderHeader {trader} />

				<!-- Navigation & Content Section -->
				<div class="fl-row fl-row-full-width pills-section">
					<div class="fl-row-content-wrap">
						<div class="fl-row-content fl-row-fixed-width fl-node-content">
							<div class="fl-col-group">
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-html">
											<div class="fl-module-content fl-node-content">
												<div class="fl-html">
													<TraderNav traderSlug={trader.slug} {currentPath} />
												</div>
											</div>
										</div>
									</div>
								</div>
							</div>

							<!-- Page Content Slot -->
							<div class="fl-col-group">
								<div class="fl-col">
									<div class="fl-col-content fl-node-content">
										<slot />
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			{:else}
				<div class="not-found">
					<h1>Trader Not Found</h1>
					<p>The trader you're looking for could not be found.</p>
					<a href="/dashboard/day-trading-room/meet-the-traders" class="btn btn-default">
						← Back to All Traders
					</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.dashboard__content {
		padding: 0;
	}

	.dashboard__content-main {
		padding: 0;
	}

	.dashboard__content-section {
		padding: 0;
	}

	.fl-row {
		margin: 0;
		padding: 0;
		width: 100%;
	}

	.fl-row-full-width {
		width: 100%;
	}

	.pills-section {
		background-color: #fff;
	}

	.fl-row-content-wrap {
		max-width: 1200px;
		margin: 0 auto;
		padding: 30px 20px 40px;
	}

	.fl-row-content {
		width: 100%;
	}

	.fl-row-fixed-width {
		max-width: 1100px;
		margin: 0 auto;
	}

	.fl-col-group {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.fl-col {
		flex: 1;
		padding: 0 15px;
		min-width: 0;
	}

	.fl-col-content {
		height: 100%;
	}

	.fl-node-content {
		position: relative;
	}

	.fl-module {
		margin-bottom: 20px;
	}

	.fl-module:last-child {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	.not-found {
		text-align: center;
		padding: 60px 20px;
		background: #fff;
	}

	.not-found h1 {
		font-size: 28px;
		margin-bottom: 15px;
		color: #333;
	}

	.not-found p {
		color: #666;
		margin-bottom: 25px;
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

	@media (max-width: 767px) {
		.fl-col-group {
			flex-direction: column;
		}

		.fl-col {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}
</style>
