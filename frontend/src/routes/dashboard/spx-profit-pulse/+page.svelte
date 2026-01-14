<script lang="ts">
	/**
	 * SPX Profit Pulse Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches: Tr3ndy SPX Alerts Service Dashboard
	 * Reference: frontend/Do's/SPX Tr3ndy/SPX-Profit-Pulse
	 *
	 * @version 2.1.0 - Added Latest Updates section per WordPress reference
	 */
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';

	// SSR data from +page.server.ts
	let { data } = $props();

	// Latest Updates data - matches WordPress reference exactly
	const latestUpdatesItems = [
		{
			id: 1,
			type: 'Daily Video',
			title: 'January 13 2026',
			date: 'January 13, 2026',
			excerpt: 'With Blly Ribeiro',
			href: '/dashboard/spx-profit-pulse/alerts/january-13-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		},
		{
			id: 2,
			type: 'Daily Video',
			title: 'January 12 2026',
			date: 'January 12, 2026',
			excerpt: 'With Freddie Ferber',
			href: '/dashboard/spx-profit-pulse/alerts/january-12-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		},
		{
			id: 3,
			type: 'Daily Video',
			title: 'January 9 2026',
			date: 'January 09, 2026',
			excerpt: 'With Shao Wan',
			href: '/dashboard/spx-profit-pulse/alerts/january-9-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		},
		{
			id: 4,
			type: 'Daily Video',
			title: 'January 8 2026',
			date: 'January 08, 2026',
			excerpt: 'With Melissa Beegle',
			href: '/dashboard/spx-profit-pulse/alerts/january-8-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		},
		{
			id: 5,
			type: 'Daily Video',
			title: 'January 7 2026',
			date: 'January 07, 2026',
			excerpt: 'With Jonathan McKeever',
			href: '/dashboard/spx-profit-pulse/alerts/january-7-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		},
		{
			id: 6,
			type: 'Daily Video',
			title: 'January 6 2026',
			date: 'January 06, 2026',
			excerpt: 'With Jonathan McKeever',
			href: '/dashboard/spx-profit-pulse/alerts/january-6-2026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true
		}
	];
</script>

<svelte:head>
	<title>SPX Profit Pulse Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader 
	roomName="SPX Profit Pulse" 
	startHereUrl="/dashboard/spx-profit-pulse/start-here" 
/>

<!-- DASHBOARD CONTENT - Exact WordPress Structure -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- MAIN CONTENT SECTION - Video + Featured Cards -->
		<section class="dashboard__content-section-member">
			
			<!-- Welcome Video -->
			<div class="video-container">
				<video 
					controls 
					poster="https://cdn.simplertrading.com/2025/07/02134158/FTR-Jonathan.png"
					class="welcome-video"
				>
					<source 
						src="https://simpler-options.s3.amazonaws.com/Tr3ndy/TrendySPXQuickstart2025.mp4" 
						type="video/mp4"
					>
					Your browser doesn't support HTML5 video.
				</video>
			</div>

			<!-- Featured Cards - Exact WordPress Match -->
			<div class="featured_cards">
				<div class="card-col">
					<div class="img_1">
						<h2 class="card_title">Start Here</h2>
						<p class="featured-desc">Key steps for getting started with your SPX Profit Pulse membership</p>
						<a href="/dashboard/spx-profit-pulse/start-here">CHECK IT OUT</a>
					</div>
				</div>
				<div class="card-col">
					<div class="img_2">
						<h2 class="card_title_2">Meet the Traders</h2>
						<p>Here you'll find Strategies and more!</p>
						<div class="buttons">
							<a class="aclass" href="/dashboard/spx-profit-pulse/billy-ribeiro/">Meet Billy</a>
						</div>
					</div>
				</div>
				<div class="card-col">
					<div class="img_3">
						<h2 class="card_title_3">Continued Education</h2>
						<p class="featured-desc">Grow your trading skills and learn to read the market better.</p>
						<a target="_blank" href="https://www.youtube.com/@Tr3ndyOptions/featured">LEARN MORE</a>
					</div>
				</div>
			</div>

		</section>

		<!-- Latest Updates Section - Exact WordPress Match -->
		<section class="dashboard__content-section">
			<LatestUpdates 
				items={latestUpdatesItems}
				title="Latest Updates"
				roomSlug="spx-profit-pulse"
				buttonText="Watch Now"
			/>
		</section>

		<!-- Weekly Watchlist Section -->
		<div class="dashboard__content-section u--background-color-white">
			<WeeklyWatchlist />
		</div>

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SPX PROFIT PULSE - Pixel-Perfect WordPress Match
	   Reference: Tr3ndy SPX Alerts Service Dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	.dashboard__content {
		display: block;
		width: 100%;
	}

	.dashboard__content-main {
		width: 100%;
		max-width: 100%;
	}

	/* Main Content Section */
	.dashboard__content-section-member {
		padding: 20px;
	}

	@media screen and (min-width: 768px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1200px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}

	/* Video Container - Max width 1024px on large devices */
	.video-container {
		margin-bottom: 30px;
		max-width: 1024px;
		margin-left: auto;
		margin-right: auto;
	}

	.welcome-video {
		width: 100%;
		height: auto;
		display: block;
		margin: 0 auto;
		border-radius: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FEATURED CARDS - 2 on top, 1 centered on bottom
	   ═══════════════════════════════════════════════════════════════════════════ */
	.featured_cards {
		display: grid;
		grid-template-columns: 1fr;
		gap: 20px;
		max-width: 1024px;
		margin: 0 auto;
	}

	/* Mobile: 1 column */
	.card-col {
		display: flex;
		width: 100%;
	}

	/* Tablet and up: 2 columns for first 2 cards */
	@media screen and (min-width: 768px) {
		.featured_cards {
			grid-template-columns: repeat(2, 1fr);
		}
		
		/* Third card spans both columns and centers */
		.card-col:nth-child(3) {
			grid-column: 1 / -1;
			max-width: 485px;
			margin: 0 auto;
			width: 100%;
		}
	}

	/* Card Title Styles - Exact WordPress Match */
	.card_title {
		background-color: #2E88E2;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		margin: 0;
		color: #fff;
	}

	.card_title_2 {
		background-color: #284556;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		margin: 0;
		color: #fff;
	}

	.card_title_3 {
		background-color: #062946;
		border-radius: 8px 8px 0 0;
		opacity: 0.81;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		margin: 0;
		color: #fff;
	}

	/* Card Content Paragraphs */
	.featured_cards p {
		padding: 0 20px;
		margin: 15px 0;
		color: #fff;
	}

	.featured-desc {
		min-height: 48px;
	}

	/* Card Buttons - Exact WordPress Match */
	.featured_cards a {
		color: #fff !important;
		display: inline-block;
		font-family: "Open Sans Condensed", sans-serif;
		font-weight: 700;
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		border-radius: 50px;
		padding: 5px 30px;
		margin-bottom: 20px;
		text-decoration: none;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.featured_cards a:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(198, 128, 0, 0.4);
	}

	.buttons {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
	}

	.buttons .aclass {
		margin-bottom: 20px;
	}

	/* Card Background Images - Exact WordPress Match */
	.img_1 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/08/12114451/start-here-top-card-bg.png');
		text-align: center;
		display: flex;
		flex-direction: column;
		min-height: 280px;
	}

	.img_2 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/08/12123709/meet-the-traders-top-card-bg.png');
		text-align: center;
		display: flex;
		flex-direction: column;
		min-height: 280px;
	}

	.img_3 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/08/12123711/continued-ed-top-card-bg.png');
		text-align: center;
		display: flex;
		flex-direction: column;
		min-height: 280px;
	}

	/* Weekly Watchlist Section */
	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media screen and (min-width: 768px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1200px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.u--background-color-white {
		background-color: #fff !important;
	}
</style>
