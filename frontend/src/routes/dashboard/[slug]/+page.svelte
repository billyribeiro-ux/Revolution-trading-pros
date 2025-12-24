<script lang="ts">
	/**
	 * Trading Room Dashboard 
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This page displays the full trading room dashboard matching the Jesus file.
	 * Structure copied from SimplerMasteringTheTradeDashboard lines 2965-3214.
	 *
	 * @version 1.0.0
	 */
	import { page } from '$app/stores';
	
	let slug = $derived($page.params.slug);
	
	// Convert slug to display name (e.g., "options-day-trading-room" -> "Options Day Trading Room")
	let displayName = $derived(
		slug
			? slug.split('-')
				.map(word => word.charAt(0).toUpperCase() + word.slice(1))
				.join(' ')
			: 'Trading Room'
	)
</script>

<svelte:head>
	<title>{displayName} Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- DASHBOARD HEADER - Matches SimplerMasteringTheTradeDashboard line 2965-2994 -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{displayName} Dashboard</h1>
		<a href="/dashboard/{slug}/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>
	<div class="dashboard__header-right">
		<ul class="ultradingroom" style="text-align: right;list-style: none;">
			<li class="litradingroom">
				<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="btn btn-xs btn-link" style="font-weight: 700 !important;">Trading Room Rules</a>
			</li>
			<li style="font-size: 11px;" class="btn btn-xs btn-link litradingroomhind">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</li>
		</ul>

		<div class="dropdown display-inline-block">
			<button type="button" class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle" id="dLabel" data-bs-toggle="dropdown" aria-expanded="false">
				<strong>Enter a Trading Room</strong>
			</button>
			
			<nav class="dropdown-menu dropdown-menu--full-width" aria-labelledby="dLabel">
				<ul class="dropdown-menu__menu">
					<li>
						<a href="/trading-room/{slug}" target="_blank" rel="nofollow">
							<span class="st-icon-{slug} icon icon--md"></span>
							{displayName}
						</a>
					</li>
				</ul>
			</nav>
		</div>
	</div>
</header>

<!-- DASHBOARD CONTENT - Matches SimplerMasteringTheTradeDashboard line 2997-3183 -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		
		<!-- LATEST UPDATES SECTION - Matches SimplerMasteringTheTradeDashboard line 3024-3157 -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				
				<!-- Article cards will go here -->
				<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure class="article-card__image" style="background-image: url(https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg);">
							<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="Daily Video" />
						</figure>
						<div class="article-card__type">
							<span class="label label--info">Daily Video</span>
						</div>
						<h4 class="h5 article-card__title">
							<a href="/daily/{slug}">Latest Market Analysis</a>
						</h4>
						<span class="article-card__meta"><small>Latest updates</small></span>
						<div class="article-card__excerpt u--hide-read-more">
							<p>Get the latest market analysis and trading insights.</p>
						</div>
						<a href="/daily/{slug}" class="btn btn-tiny btn-default">Watch Now</a>
					</article>
				</div>

			</div>
		</section>

		<!-- WEEKLY WATCHLIST SECTION - Matches SimplerMasteringTheTradeDashboard line 3158-3182 -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
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

	<!-- PANEL 2: SECONDARY SIDEBAR - Matches SimplerMasteringTheTradeDashboard line 3185-3208 -->
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
	/* All CSS inherited from +layout.svelte */
</style>
