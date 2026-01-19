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
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	let trader = $derived(getTraderBySlug($page.params.slug ?? ''));
	let currentPath = $derived.by(() => {
		const slug = $page.params.slug;
		const fullPath = $page.url.pathname;
		const basePath = `/dashboard/small-account-mentorship/meet-the-traders/${slug}`;
		return fullPath.replace(basePath, '') || '';
	});
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
				<div class="trader-section">
					<div class="trader-section__wrapper">
						<div class="trader-section__content">
							<div class="trader-nav-container">
								<TraderNav traderSlug={trader.slug} {currentPath} />
							</div>

							<!-- Page Content Slot -->
							<div class="trader-page-content">
								{@render children()}
							</div>
						</div>
					</div>
				</div>
			{:else}
				<div class="not-found">
					<h1>Trader Not Found</h1>
					<p>The trader you're looking for could not be found.</p>
					<a href="/dashboard/small-account-mentorship/meet-the-traders" class="btn btn-default">
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

	/* Trader Section - Semantic naming */
	.trader-section {
		width: 100%;
		background-color: #fff;
	}

	.trader-section__wrapper {
		max-width: 1200px;
		margin: 0 auto;
		padding: 30px 20px 40px;
	}

	.trader-section__content {
		max-width: 1100px;
		margin: 0 auto;
		width: 100%;
	}

	.trader-nav-container {
		margin-bottom: 20px;
	}

	.trader-page-content {
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
		background: #143e59;
		color: #fff;
		border: 1px solid #143e59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}
</style>
