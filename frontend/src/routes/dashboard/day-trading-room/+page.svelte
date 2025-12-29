<script lang="ts">
	/**
	 * Day Trading Room Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Rebuilt to match Frontend/2 (Mastering the Trade) exactly
	 * Renamed: "Mastering the Trade" → "Day Trading Room"
	 *
	 * @version 2.1.0 - Added WordPress-identical structure with JWT SSO documentation
	 *
	 * ═══════════════════════════════════════════════════════════════════════════
	 * API KEYS & CUSTOM LINKS CONFIGURATION GUIDE
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 1. GOOGLE CALENDAR API (Trading Room Schedule)
	 *    ─────────────────────────────────────────────────────────────────────────
	 *    Current Keys (from WordPress - may need rotation):
	 *    - CLIENT_ID: 656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com
	 *    - API_KEY: AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw
	 *    - CALENDAR_ID: simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com
	 *
	 *    SETUP REQUIREMENTS:
	 *    a) Go to Google Cloud Console (console.cloud.google.com)
	 *    b) Create/select project → Enable Google Calendar API
	 *    c) Create OAuth 2.0 credentials → Add authorized domains
	 *    d) Create API Key → Restrict to Calendar API & your domain
	 *    e) Replace keys below with your credentials
	 *
	 *    SECURITY NOTE: These keys should be environment variables in production
	 *    - VITE_GOOGLE_CLIENT_ID
	 *    - VITE_GOOGLE_API_KEY
	 *    - VITE_GOOGLE_CALENDAR_ID
	 *
	 * 2. JWT SSO TRADING ROOM AUTHENTICATION
	 *    ─────────────────────────────────────────────────────────────────────────
	 *    WordPress uses ProTradingRoom.com for live chat rooms with JWT SSO:
	 *
	 *    Original WordPress URLs:
	 *    - Day Trading Room: https://chat.protradingroom.com/rooms/mastering-the-trade?jwt={TOKEN}
	 *    - Showcase Room: https://chat.protradingroom.com/rooms/simpler-showcase?jwt={TOKEN}
	 *
	 *    BACKEND IMPLEMENTATION REQUIRED:
	 *    a) Create endpoint: GET /api/trading-room/sso?room={room-slug}
	 *    b) Validate user authentication & room access permissions
	 *    c) Generate JWT with payload: { user_id, email, name, room_access, exp }
	 *    d) Sign JWT with shared secret (coordinate with ProTradingRoom)
	 *    e) Return: { url: "https://chat.protradingroom.com/rooms/{room}?jwt={token}" }
	 *
	 *    FRONTEND IMPLEMENTATION:
	 *    - Update dropdown links to call SSO endpoint before redirect
	 *    - Handle loading states and error cases
	 *    - Consider redirect middleware for seamless UX
	 *
	 * 3. CUSTOM LINKS THAT NEED CONFIGURATION
	 *    ─────────────────────────────────────────────────────────────────────────
	 *    - /trading-room-rules.pdf        → Host PDF on your CDN or S3
	 *    - /dashboard/day-trading-room/start-here → Create this route
	 *    - /watchlist/latest              → Create watchlist route
	 *    - /tutorials                     → Create tutorials page
	 *    - /blog                          → Create blog route
	 *    - /chatroom-archive/*            → Create archive routes
	 *    - /daily/*                       → Create daily video routes
	 *
	 * 4. CDN/ASSET URLS
	 *    ─────────────────────────────────────────────────────────────────────────
	 *    Currently using Simpler Trading CDN:
	 *    - cdn.simplertrading.com (images)
	 *    - simpler-cdn.s3.amazonaws.com (watchlist images)
	 *    - simpler-options.s3.amazonaws.com (tutorial videos)
	 *
	 *    FOR PRODUCTION: Migrate assets to your own CDN and update URLs
	 *
	 * 5. CUSTOM ICON FONT (OPTIONAL)
	 *    ─────────────────────────────────────────────────────────────────────────
	 *    WordPress uses custom icon font classes:
	 *    - st-icon-mastering-the-trade
	 *    - st-icon-simpler-showcase
	 *
	 *    Currently using Tabler Icons as alternative. If exact match needed:
	 *    a) Extract icon font from WordPress theme
	 *    b) Convert to modern icon format (SVG sprite or icon components)
	 *    c) Replace Tabler icon imports with custom icons
	 *
	 * ═══════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';

	// Tabler Icons for dropdown menu
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';

	// Article data - matches WordPress structure
	const articles = [
		{
			id: 1,
			type: 'Daily Video',
			title: 'Market Analysis & Trading Strategies',
			date: 'December 23, 2025 with HG',
			excerpt: 'Things can always change, but given how the market closed on Tuesday, it looks like Santa\'s on his way. Let\'s look at the facts, then also some preferences and opinions as we get into the end of 2025.',
			href: '/daily/day-trading-room/market-analysis',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		{
			id: 2,
			type: 'Chatroom Archive',
			title: 'December 23, 2025',
			date: 'December 23, 2025',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/day-trading-room/12232025',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 3,
			type: 'Daily Video',
			title: 'Setting Up for Success',
			date: 'December 22, 2025 with Expert',
			excerpt: 'Everything looks good for a potential rally, as the indexes are consolidating and breaking higher, along with a lot of key stocks. Let\'s take a look at TSLA, GOOGL, AMZN, AVGO, MSFT, and more.',
			href: '/daily/day-trading-room/setting-up-success',
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			isVideo: true
		},
		{
			id: 4,
			type: 'Chatroom Archive',
			title: 'December 22, 2025',
			date: 'December 22, 2025',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/day-trading-room/12222025',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 5,
			type: 'Daily Video',
			title: 'Holiday Weekend Market Review',
			date: 'December 19, 2025 with Expert',
			excerpt: 'Indexes continue to churn sideways as we approach next week\'s holiday trade. Bulls usually take over in low volume. Can they do it again?',
			href: '/daily/day-trading-room/holiday-weekend-review',
			image: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			isVideo: true
		},
		{
			id: 6,
			type: 'Chatroom Archive',
			title: 'December 19, 2025',
			date: 'December 19, 2025',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/day-trading-room/12192025',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		}
	];

	// Google Calendar integration
	onMount(() => {
		// Load Google Calendar API
		const script = document.createElement('script');
		script.src = 'https://apis.google.com/js/api.js';
		script.onload = initCalendar;
		document.head.appendChild(script);
	});

	function initCalendar() {
		const CLIENT_ID = '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
		const API_KEY = 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';
		const DISCOVERY_DOCS = ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'];
		const SCOPES = 'https://www.googleapis.com/auth/calendar.readonly';

		// @ts-ignore - gapi is loaded from external script
		if (typeof gapi !== 'undefined') {
			// @ts-ignore
			gapi.client.init({
				apiKey: API_KEY,
				clientId: CLIENT_ID,
				discoveryDocs: DISCOVERY_DOCS,
				scope: SCOPES
			}).then(() => {
				// @ts-ignore
				return gapi.client.calendar.events.list({
					'calendarId': 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
					'timeMin': (new Date()).toISOString(),
					'showDeleted': false,
					'singleEvents': true,
					'maxResults': 10,
					'orderBy': 'startTime',
					'fields': 'items(summary,start/dateTime)'
				});
			}).then((response: any) => {
				const dateOptions: Intl.DateTimeFormatOptions = {
					month: 'short',
					day: 'numeric',
					year: 'numeric',
					hour: 'numeric',
					minute: 'numeric',
					timeZoneName: 'short'
				};
				const container = document.querySelector('.room-sched');
				if (container && response.result.items) {
					for (let i = 0; i < response.result.items.length; i++) {
						const eventStart = new Date(response.result.items[i].start.dateTime);
						const eventHtml = `<h4>${response.result.items[i].summary}</h4><span>${eventStart.toLocaleString('en-US', dateOptions)}</span>`;
						container.innerHTML += eventHtml;
					}
				}
			}).catch((error: any) => {
				console.log('Calendar Error:', error);
			});
		}
	}
</script>

<svelte:head>
	<title>Day Trading Room | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<!-- DASHBOARD HEADER -->
	<header class="dashboard__header">
		<div class="dashboard__header-left">
			<h1 class="dashboard__page-title">Day Trading Room Dashboard</h1>
			<a href="/dashboard/day-trading-room/start-here" class="btn btn-xs btn-default">
				New? Start Here
			</a>
		</div>
		<div class="dashboard__header-right">
			<ul class="ultradingroom">
				<li class="litradingroom">
					<a href="/trading-room-rules.pdf" target="_blank" class="btn btn-xs btn-link">Trading Room Rules</a>
				</li>
				<li class="litradingroomhind">
					By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
				</li>
			</ul>
			<div class="dropdown display-inline-block">
				<!-- WordPress uses <a> tag instead of <button> for dropdown toggle -->
				<a href="#" class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle" id="dLabel" aria-expanded="false">
					<strong>Enter a Trading Room</strong>
				</a>
				<nav class="dropdown-menu dropdown-menu--full-width" aria-labelledby="dLabel">
					<ul class="dropdown-menu__menu">
						<li>
							<!--
								═══════════════════════════════════════════════════════════════════════
								JWT SSO AUTHENTICATION REQUIRED
								═══════════════════════════════════════════════════════════════════════
								WordPress URL: https://chat.protradingroom.com/rooms/mastering-the-trade?jwt={DYNAMIC_JWT_TOKEN}

								BACKEND REQUIREMENTS:
								1. JWT token must be generated server-side for authenticated user
								2. Token payload should include: user_id, email, name, room_access
								3. Token must be signed with shared secret between your backend and ProTradingRoom
								4. Implement endpoint: GET /api/trading-room/sso?room=day-trading-room
								   Returns: { url: "https://chat.protradingroom.com/rooms/mastering-the-trade?jwt=..." }

								IMPLEMENTATION:
								- Replace href with dynamic URL from SSO endpoint
								- Add on:click handler to fetch JWT URL before navigation
								- Consider adding loading state while JWT is being fetched
								═══════════════════════════════════════════════════════════════════════
							-->
							<a href="/api/trading-room/sso?room=day-trading-room" target="_blank" rel="nofollow">
								<span class="dropdown-icon dropdown-icon--day-trading">
									<!-- WordPress uses: <span class="st-icon-mastering-the-trade icon icon--md"></span>
									     Custom icon font required. Using Tabler as alternative. -->
									<IconChartLine size={20} />
								</span>
								Day Trading Room
							</a>
						</li>
						<li>
							<!--
								═══════════════════════════════════════════════════════════════════════
								JWT SSO AUTHENTICATION REQUIRED
								═══════════════════════════════════════════════════════════════════════
								WordPress URL: https://chat.protradingroom.com/rooms/simpler-showcase?jwt={DYNAMIC_JWT_TOKEN}

								Same backend requirements as Day Trading Room above.
								Endpoint: GET /api/trading-room/sso?room=simpler-showcase
								═══════════════════════════════════════════════════════════════════════
							-->
							<a href="/api/trading-room/sso?room=simpler-showcase" target="_blank" rel="nofollow">
								<span class="dropdown-icon dropdown-icon--showcase">
									<!-- WordPress uses: <span class="st-icon-simpler-showcase icon icon--md"></span>
									     Custom icon font required. Using Tabler as alternative. -->
									<IconTrophy size={20} />
								</span>
								Simpler Showcase Room
							</a>
						</li>
					</ul>
				</nav>
			</div>
		</div>
	</header>

<!-- DASHBOARD CONTENT - Exact WordPress Structure -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- VIDEO TUTORIAL SECTION -->
		<section class="dashboard__content-section-member">
			<video controls width="100%" poster="https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg" style="aspect-ratio: 2 / 1;">
				<source src="https://simpler-options.s3.amazonaws.com/tutorials/MTT_tutorial2025.mp4" type="video/mp4">
				Your browser does not support the video tag.
			</video>
		</section>

		<!-- LATEST UPDATES SECTION -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				{#each articles as article (article.id)}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<!-- WordPress: card-media base class + card-media--video for video content -->
							<figure class="card-media article-card__image{article.isVideo || article.type === 'Chatroom Archive' ? ' card-media--video' : ''}" style="background-image: url({article.image});">
								<a href={article.href}><img src={article.image} alt={article.title} /></a>
								{#if article.isVideo}
									<div class="article-card__type">
										<span class="label label--info">{article.type}</span>
									</div>
								{/if}
							</figure>
							<h4 class="h5 article-card__title"><a href={article.href}>{article.title}</a></h4>
							<!-- WordPress Order: excerpt BEFORE meta -->
							<div class="article-card__excerpt u--hide-read-more">
								<p class="u--margin-bottom-0 u--font-size-sm">{#if !article.isVideo}<i>{article.excerpt}</i>{:else}{article.excerpt}{/if}</p>
							</div>
							<span class="article-card__meta"><small>{article.date}</small></span>
							<a href={article.href} class="btn btn-tiny btn-default">Watch Now</a>
						</article>
					</div>
				{/each}
			</div>
		</section>

		<!-- WEEKLY WATCHLIST SECTION -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<div class="hidden-md d-lg-none pb-2">
							<a href="/watchlist/latest">
								<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius">
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
							<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius">
						</a>
					</div>
				</div>
			</section>
		</div>

	</div>

	<!-- SIDEBAR (PANEL 2) -->
	<aside class="dashboard__content-sidebar">
		<!-- TRADING ROOM SCHEDULE -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Trading Room Schedule
				<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;">Schedule is subject to change.</p>
			</h4>
			<div class="script-container">
				<div class="room-sched"></div>
			</div>
		</section>

		<!-- QUICK LINKS -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
				<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank">Trading Blog</a></li>
			</ul>
		</section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		gap: 20px;
	}

	@media (max-width: 991px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 15px;
		flex-wrap: wrap;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.dashboard__header-right {
		display: flex;
		align-items: center;
		gap: 20px;
		flex-wrap: wrap;
	}

	.ultradingroom {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 5px;
		text-align: right;
	}

	.litradingroom {
		list-style: none;
	}

	.litradingroomhind {
		font-size: 11px;
		color: #666;
		list-style: none;
	}

	.display-inline-block {
		display: inline-block;
		position: relative;
	}

	/* Base button - Bootstrap 3 standard */
	.btn {
		display: inline-block;
		padding: 6px 12px;
		margin-bottom: 0;
		font-size: 14px;
		font-weight: 400;
		line-height: 1.42857143;
		text-align: center;
		white-space: nowrap;
		vertical-align: middle;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: 4px;
		text-decoration: none;
		font-family: 'Open Sans', sans-serif;
	}

	/* btn-xs - smaller button variant */
	.btn-xs {
		padding: 1px 5px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	.btn-link {
		background: transparent;
		color: #0984ae;
		border: none;
		text-decoration: none;
		font-weight: 700 !important;
	}

	.btn-link:hover {
		text-decoration: underline;
	}

	/* Orange button colors - NO padding override */
	.btn-orange,
	.btn-tradingroom {
		background-color: #F69532;
		color: #fff;
		border-color: #F69532;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.15s ease-in-out;
	}

	/* EXACT from Simpler Trading style.css - btn-tradingroom override */
	.btn.btn-xs.btn-orange.btn-tradingroom {
		width: 280px;
		padding: 12px 18px;
	}

	.btn-orange:hover,
	.btn-tradingroom:hover {
		background-color: #dc7309;
		border-color: #dc7309;
		color: #fff;
	}

	.dropdown-toggle::after {
		content: '';
		display: none;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin: 5px 0 0;
		padding: 10px;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		z-index: 1000;
		min-width: 200px;
		display: none;
		font-size: 14px;
	}

	.dropdown:hover .dropdown-menu {
		display: block;
	}

	.dropdown-menu--full-width {
		min-width: 280px;
	}

	.dropdown-menu__menu {
		list-style: none;
		margin: -10px;
		padding: 0;
	}

	.dropdown-menu__menu li {
		margin: 0;
	}

	.dropdown-menu__menu a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 15px;
		color: #666;
		font-size: 14px;
		line-height: 1.4;
		text-decoration: none;
		white-space: nowrap;
		transition: all 0.15s ease-in-out;
	}

	.dropdown-menu__menu a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	/* Dropdown Icon Styling - Pixel-perfect match */
	.dropdown-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.dropdown-icon--day-trading {
		background-color: #0984ae;
		color: #fff;
	}

	.dropdown-icon--showcase {
		background-color: #000;
		color: #F69532;
	}

	.dropdown-icon :global(svg) {
		width: 18px;
		height: 18px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - Exact WordPress Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - Exact WordPress Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
	}

	/* Video Tutorial Section */
	.dashboard__content-section-member {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}

	.dashboard__content-section-member video {
		width: 100%;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	/* Content Sections */
	.dashboard__content-section {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.dashboard__content-section + .dashboard__content-section {
		border-top: 1px solid #dbdbdb;
	}

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* Article Cards */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
	}

	.article-cards.row {
		margin: 0 -10px;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		display: flex;
	}

	.col-xs-12 {
		width: 100%;
		padding: 0 10px;
		box-sizing: border-box;
		margin-bottom: 20px;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.col-md-6 {
		flex: 0 0 50%;
		max-width: 50%;
	}

	.col-xl-4 {
		flex: 0 0 33.333%;
		max-width: 33.333%;
	}

	@media (max-width: 992px) {
		.col-xl-4 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 641px) {
		.col-sm-6,
		.col-md-6,
		.col-xl-4 {
			flex: 0 0 100%;
			max-width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - ICT11+ PIXEL-PERFECT MATCH to dashboard-globals.css
	   Reference lines 931-1021
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		margin-bottom: 30px;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CARD MEDIA - WordPress Match (card-media base class)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		margin: 0;
		overflow: hidden;
	}

	/* card-media--video: Adds play button overlay indicator for video content */
	.card-media--video::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		z-index: 3;
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%) translateX(3px);
		width: 0;
		height: 0;
		border-left: 18px solid #fff;
		border-top: 12px solid transparent;
		border-bottom: 12px solid transparent;
		z-index: 4;
		opacity: 0;
		transition: opacity 0.2s ease-in-out;
	}

	.article-card:hover .card-media--video::before,
	.article-card:hover .card-media--video::after {
		opacity: 1;
	}

	/* Image with 16:9 aspect ratio using padding-top trick */
	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		margin: 0;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__image a {
		display: block;
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 2;
	}

	/* Type label positioned OVER the image */
	.article-card__type {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 2;
		margin: 0;
	}

	/* Label badge - PILL SHAPE 25px radius */
	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	.label--info {
		background-color: #0984ae;
		color: #fff;
	}

	/* Title */
	.article-card__title {
		margin: 0;
		padding: 15px 15px 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.3;
		font-family: 'Open Sans', sans-serif;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.h5 {
		font-size: 16px;
		font-weight: 700;
	}

	/* Meta */
	.article-card__meta {
		display: block;
		padding: 0 15px;
		color: #999;
		font-size: 12px;
	}

	.article-card__meta small {
		font-size: 12px;
	}

	/* Excerpt */
	.article-card__excerpt {
		padding: 10px 15px;
		font-size: 14px;
		line-height: 1.5;
		color: #666;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	.u--hide-read-more {
		display: block;
	}

	/* WordPress utility classes */
	.u--margin-bottom-0 {
		margin-bottom: 0 !important;
	}

	.u--font-size-sm {
		font-size: 14px !important;
	}

	/* Button in article card */
	.article-card .btn {
		margin: 0 15px 15px;
	}

	/* btn-tiny */
	.btn-tiny {
		padding: 5px 10px;
		font-size: 11px;
		line-height: 1.5;
		border-radius: 3px;
	}

	/* btn-tiny.btn-default - Watch Now button ORANGE */
	.article-card .btn.btn-tiny.btn-default {
		background: transparent;
		color: #F3911B;
		padding-left: 0;
		font-size: 17px;
		border: none;
	}

	.article-card .btn.btn-tiny.btn-default:hover {
		color: #F3911B;
		background: #e7e7e7;
		padding-left: 8px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		box-shadow: none;
		display: inline-block;
		text-decoration: none;
		border-radius: 4px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #ccc;
		box-shadow: none;
	}

	/* Weekly Watchlist Section */
	.u--background-color-white {
		background-color: #fff !important;
	}

	.section-title-alt {
		color: #0984ae;
		font-weight: 700;
		font-size: 14px;
		letter-spacing: 0.2em;
		margin-bottom: 30px;
		text-transform: uppercase;
		font-family: 'Open Sans', sans-serif;
	}

	.section-title-alt--underline {
		padding-bottom: 30px;
		position: relative;
	}

	.section-title-alt--underline::after {
		background-color: #e8e8e8;
		bottom: 2px;
		content: ' ';
		display: block;
		height: 2px;
		position: absolute;
		left: 0;
		width: 50px;
	}

	.u--border-radius {
		border-radius: 8px !important;
	}

	.u--font-weight-bold {
		font-weight: 700 !important;
	}

	.hidden-md {
		display: block;
	}

	.d-lg-none {
		display: block;
	}

	@media (min-width: 992px) {
		.d-lg-none {
			display: none !important;
		}
	}

	.pb-2 {
		padding-bottom: 0.5rem;
	}

	.hidden-xs,
	.hidden-sm {
		display: none;
	}

	.d-none {
		display: none !important;
	}

	.d-lg-block {
		display: none !important;
	}

	@media (min-width: 992px) {
		.d-lg-block {
			display: block !important;
		}

		.hidden-xs,
		.hidden-sm {
			display: block;
		}
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-right: -15px;
		margin-left: -15px;
	}

	.col-sm-6,
	.col-lg-5,
	.col-lg-7 {
		position: relative;
		width: 100%;
		padding-right: 15px;
		padding-left: 15px;
	}

	@media (min-width: 641px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-5 {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-lg-7 {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}

	.dashboard__content-section section {
		margin: 0;
	}

	.dashboard__content-section section img {
		width: 100%;
		height: auto;
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR (PANEL 2) - Exact WordPress Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-sidebar {
		display: block;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	@media (max-width: 1079px) {
		.dashboard__content-sidebar {
			display: none;
		}
	}

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	.content-sidebar__heading {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 15px;
		text-transform: initial;
	}

	.script-container {
		margin: 0;
	}

	.room-sched {
		margin: 0;
	}

	/* Calendar event styles - dynamically generated by Google Calendar API */
	:global(.room-sched h4) {
		font-size: 14px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px 0;
		font-family: 'Open Sans', sans-serif;
	}

	:global(.room-sched span) {
		display: block;
		font-size: 13px;
		color: #666;
		margin-bottom: 20px;
		line-height: 1.4;
	}

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin-bottom: 12px;
	}

	.link-list a {
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.2s;
	}

	.link-list a:hover {
		color: #076787;
		text-decoration: underline;
	}

	/* Responsive */
	@media (max-width: 992px) {
		.dashboard__content {
			flex-direction: column;
		}
	}
</style>
