<script lang="ts">
	/**
	 * Weekly Watchlist Single Item Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Displays individual weekly watchlist with Rundown (video) and Watchlist (spreadsheet) tabs
	 * Matches Simpler Trading's exact layout and functionality
	 * @version 1.0.0 - December 2025
	 */

	// Define the WatchlistItem interface to match server data
	interface WatchlistItem {
		slug: string;
		title: string;
		subtitle: string;
		trader: string;
		datePosted: string;
		video: {
			src: string;
			poster: string;
			title: string;
		};
		spreadsheet: {
			src: string;
		};
		description: string;
		previous: {
			slug: string;
			title: string;
		} | null;
		next: {
			slug: string;
			title: string;
		} | null;
	}

	interface PageData {
		watchlist: WatchlistItem;
	}

	// Props from server load
	let { data }: { data: PageData } = $props();

	// Tab state using Svelte 5 runes
	let activeTab = $state<'rundown' | 'watchlist'>('rundown');

	// Watchlist data from server
	const watchlistData = $derived(data.watchlist);

	function setActiveTab(tab: 'rundown' | 'watchlist') {
		activeTab = tab;
	}
</script>

<svelte:head>
	<title>{watchlistData.title} | Revolution Trading Pros</title>
	<meta name="description" content="Weekly Watchlist: Stock market analysis with Revolution Trading Pros" />
</svelte:head>

<!-- Breadcrumbs - Fixed to avoid double slash -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
			</li>
			<li class="separator separator-home"> / </li>
			<li class="item-cat item-custom-post-type-watchlist">
				<a class="breadcrumb-link breadcrumb-custom-post-type-watchlist" href="/dashboard/ww/watchlist-rundown-archive" title="Weekly Watchlist">Weekly Watchlist</a>
			</li>
			<li class="separator"> / </li>
			<li class="item-current">
				<strong class="breadcrumb-current" title={watchlistData.title}>{watchlistData.title}</strong>
			</li>
		</ul>
	</div>
</nav>

<!-- Page Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">

		<!-- Title Section with Previous/Next Navigation -->
		<section id="ww-title" class="cpost-section cpost-title-section">
			<div class="section-inner">
				<!-- Previous Button -->
				<div id="w-previous" class="cpost-previous">
					{#if watchlistData.previous}
						<a href="/watchlist/{watchlistData.previous.slug}" title={watchlistData.previous.title}>
							<i class="fa fa-chevron-circle-left"></i>
							<span> Previous</span>
						</a>
					{/if}
				</div>

				<!-- Title -->
				<h1 class="cpost-title">{watchlistData.title}</h1>

				<!-- Next Button -->
				<div id="ww-next" class="cpost-next">
					{#if watchlistData.next}
						<a href="/watchlist/{watchlistData.next.slug}" title={watchlistData.next.title}>
							<span>Next </span>
							<i class="fa fa-chevron-circle-right"></i>
						</a>
					{/if}
				</div>

				<!-- Subtitle -->
				<h2 class="cpost-subtitle sub-lg-up">{watchlistData.subtitle}</h2>
			</div>
		</section>

		<!-- Tabs Section -->
		<section class="cpost-section">
			<div class="section-inner">
				<div class="tr_tabs ww-single">
					<!-- Tab Buttons -->
					<div class="tab">
						<button
							id="tab-link-1"
							data-tab="tab-1"
							class="tablinks"
							class:active={activeTab === 'rundown'}
							onclick={() => setActiveTab('rundown')}
						>
							Rundown
						</button>
						<button
							id="tab-link-2"
							data-tab="tab-2"
							class="tablinks"
							class:active={activeTab === 'watchlist'}
							onclick={() => setActiveTab('watchlist')}
						>
							Watchlist
						</button>
					</div>

					<!-- Tab 1: Rundown (Video) -->
					<div id="tab-1" class="tabcontent" class:active={activeTab === 'rundown'}>
						<div class="ww-content-block cpost-content-block">
							<div class="current-vid">
								<h3 class="current-title">{watchlistData.video.title}</h3>
								<div class="video-container current">
									<video
										id="ww-t2-player"
										controls
										width="100%"
										poster={watchlistData.video.poster}
										style="aspect-ratio: 16/9;"
										title={watchlistData.video.title}
									>
										<source src={watchlistData.video.src} type="video/mp4">
										Your browser does not support the video tag.
									</video>
								</div>
							</div>
						</div>
						<div class="ww-description">
							<p>{watchlistData.description}</p>
						</div>
					</div>

					<!-- Tab 2: Watchlist (Spreadsheet) -->
					<div id="tab-2" class="tabcontent" class:active={activeTab === 'watchlist'}>
						<div id="ww-spreadsheet">
							<iframe
								src={watchlistData.spreadsheet.src}
								width="100%"
								height="700"
								title="Weekly Watchlist Spreadsheet"
								frameborder="0"
							></iframe>
						</div>
					</div>
				</div>
			</div>
		</section>

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Weekly Watchlist Single Item Styles - Matching Simpler Trading
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f4f4f4;
		padding: 12px 0;
		border-bottom: 1px solid #dbdbdb;
	}

	.breadcrumbs .container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.breadcrumbs li {
		font-size: 14px;
		color: #666;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		margin: 0 8px;
	}

	.breadcrumbs .breadcrumb-current {
		color: #333;
		font-weight: 600;
	}

	/* Page Container */
	#page {
		background: #f4f4f4;
		min-height: 100vh;
	}

	#content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Title Section */
	.cpost-title-section {
		padding: 30px 0 20px;
	}

	.cpost-title-section .section-inner {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		text-align: center;
		position: relative;
	}

	.cpost-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 32px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0;
		text-transform: uppercase;
		flex: 1 1 100%;
		order: 1;
	}

	.cpost-subtitle {
		font-size: 18px;
		font-weight: 400;
		color: #666;
		margin: 10px 0 0;
		flex: 1 1 100%;
		order: 4;
	}

	/* Previous/Next Navigation */
	.cpost-previous,
	.cpost-next {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
	}

	.cpost-previous {
		left: 0;
	}

	.cpost-next {
		right: 0;
	}

	.cpost-previous a,
	.cpost-next a {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		padding: 10px 16px;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		background: rgba(9, 132, 174, 0.1);
		color: #076787;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 20px;
	}

	/* Font Awesome Icons (fallback) */
	.fa-chevron-circle-left::before {
		content: '\2039';
		font-style: normal;
		font-size: 24px;
	}

	.fa-chevron-circle-right::before {
		content: '\203A';
		font-style: normal;
		font-size: 24px;
	}

	/* Tabs Section */
	.cpost-section {
		padding-bottom: 40px;
	}

	.tr_tabs {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		overflow: hidden;
	}

	.tab {
		display: flex;
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.tablinks {
		flex: 1;
		padding: 16px 24px;
		font-size: 16px;
		font-weight: 600;
		color: #666;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.tablinks:hover {
		background: #f0f0f0;
		color: #333;
	}

	.tablinks.active {
		background: #fff;
		color: #0984ae;
		border-bottom: 3px solid #0984ae;
		margin-bottom: -1px;
	}

	/* Tab Content */
	.tabcontent {
		display: none;
		padding: 30px;
	}

	.tabcontent.active {
		display: block;
	}

	/* Video Container */
	.current-vid {
		margin-bottom: 20px;
	}

	.current-title {
		font-size: 22px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 16px;
	}

	.video-container {
		background: #000;
		border-radius: 8px;
		overflow: hidden;
	}

	.video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	/* Description */
	.ww-description {
		padding: 20px 0 0;
		color: #666;
		font-size: 15px;
		line-height: 1.6;
	}

	.ww-description p {
		margin: 0;
	}

	/* Spreadsheet Container */
	#ww-spreadsheet {
		width: 100%;
	}

	#ww-spreadsheet iframe {
		display: block;
		border: none;
		border-radius: 4px;
		background: #f9f9f9;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.cpost-title {
			font-size: 24px;
			padding: 0 50px;
		}

		.cpost-previous a span,
		.cpost-next a span {
			display: none;
		}

		.cpost-previous a,
		.cpost-next a {
			padding: 8px;
		}

		.tablinks {
			padding: 12px 16px;
			font-size: 14px;
		}

		.tabcontent {
			padding: 20px;
		}

		#ww-spreadsheet iframe {
			height: 500px;
		}
	}

	@media (max-width: 480px) {
		.cpost-title {
			font-size: 20px;
			padding: 0 40px;
		}

		.cpost-subtitle {
			font-size: 14px;
		}

		.current-title {
			font-size: 18px;
		}
	}
</style>
