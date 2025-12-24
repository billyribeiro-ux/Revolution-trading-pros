<script lang="ts">
	/**
	 * Options Day Trading Room Dashboard - Exact Match to Mastering the Trade Structure
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This is the main dashboard page for the Options Day Trading Room membership.
	 * Structure matches SimplerMasteringTheTradeDashboard exactly, just with renamed content.
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	let dropdownOpen = $state(false);

	function handleDropdownToggle(event: MouseEvent): void {
		event.stopPropagation();
		dropdownOpen = !dropdownOpen;
	}

	function closeDropdown(): void {
		dropdownOpen = false;
	}
</script>

<svelte:window on:click={closeDropdown} />

<!-- DASHBOARD HEADER -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Options Day Trading Room Dashboard</h1>
		<a href="/dashboard/options-day-trading-room/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>
	<div class="dashboard__header-right">
		<ul class="trading-room-rules">
			<li><a href="/trading-room-rules.pdf" target="_blank" class="rules-link">Trading Room Rules</a></li>
			<li class="rules-disclaimer">By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.</li>
		</ul>

		<div class="dropdown" class:is-open={dropdownOpen}>
			<button
				type="button"
				class="btn btn-orange btn-tradingroom"
				onclick={handleDropdownToggle}
				aria-expanded={dropdownOpen}
				aria-haspopup="true"
			>
				<strong>Enter a Trading Room</strong>
				<IconChevronDown size={16} />
			</button>

			{#if dropdownOpen}
				<div class="dropdown-menu">
					<ul>
						<li>
							<a href="/trading-room/options-day-trading-room" target="_blank">
								<span class="room-icon st-icon-options-day-trading"></span>
								Options Day Trading Room
							</a>
						</li>
						<li>
							<a href="/trading-room/simpler-showcase" target="_blank">
								<span class="room-icon st-icon-simpler-showcase"></span>
								Simpler Showcase Room
							</a>
						</li>
					</ul>
				</div>
			{/if}
		</div>
	</div>
</header>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- LATEST UPDATES SECTION -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				
				<!-- Daily Video Card 1 -->
				<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure class="article-card__image" style="background-image: url('https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg');">
							<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="Daily Video" />
						</figure>
						<div class="article-card__type">
							<span class="label label--info">Daily Video</span>
						</div>
						<h4 class="h5 article-card__title"><a href="/daily/options-day-trading-room/latest">Weekend Market Review</a></h4>
						<span class="article-card__meta"><small>Latest market analysis</small></span>
						<div class="article-card__excerpt u--hide-read-more">
							<p>Get the latest market analysis and trading insights from our expert traders.</p>
						</div>
						<a href="/daily/options-day-trading-room/latest" class="btn btn-tiny btn-default">Watch Now</a>
					</article>
				</div>

				<!-- Trading Room Archive Card -->
				<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure class="card-media article-card__image card-media--video" style="background-image: url('https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg');">
							<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="Trading Room Archive" />
						</figure>
						<h4 class="h5 article-card__title"><a href="/chatroom-archive/options-day-trading-room">Recent Trading Room Session</a></h4>
						<div class="article-card__excerpt u--hide-read-more">
							<p class="u--margin-bottom-0 u--font-size-sm"><i>With our expert traders</i></p>
						</div>
						<span class="article-card__meta"><small>Latest session</small></span>
						<a href="/chatroom-archive/options-day-trading-room" class="btn btn-tiny btn-default">Watch Now</a>
					</article>
				</div>

				<!-- Daily Video Card 2 -->
				<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure class="article-card__image" style="background-image: url('https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg');">
							<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="Daily Video" />
						</figure>
						<div class="article-card__type">
							<span class="label label--info">Daily Video</span>
						</div>
						<h4 class="h5 article-card__title"><a href="/daily/options-day-trading-room">Market Analysis</a></h4>
						<span class="article-card__meta"><small>Expert trading insights</small></span>
						<div class="article-card__excerpt u--hide-read-more">
							<p>Professional analysis and trade ideas from our trading team.</p>
						</div>
						<a href="/daily/options-day-trading-room" class="btn btn-tiny btn-default">Watch Now</a>
					</article>
				</div>

			</div>
		</section>

		<!-- WEEKLY WATCHLIST FEATURED SECTION -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<div class="hidden-md d-lg-none pb-2">
							<a href="/watchlist/latest">
								<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">Weekly Watchlist with TG Watkins</h4>
						<div class="u--hide-read-more">
							<p>Week of December 22, 2025.</p>
						</div>
						<a href="/watchlist/latest" class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
						<a href="/watchlist/latest">
							<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
						</a>
					</div>
				</div>
			</section>
		</div>

	</div>

	<!-- PANEL 2: SECONDARY SIDEBAR - Visible on membership pages -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Trading Room Schedule
				<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;">Schedule is subject to change.</p>
			</h4>
			<div class="script-container">
				<p style="padding: 10px; color: #666; font-size: 13px;">Trading room schedule will be displayed here.</p>
			</div>
		</section>
		
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
				<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank">Simpler Blog</a></li>
			</ul>
		</section>
	</aside>
</div>

<style>
	/* All CSS inherited from +layout.svelte and +page.svelte */
	/* This page uses the same styling as the main dashboard */
</style>
