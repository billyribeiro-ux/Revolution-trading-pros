<script lang="ts">
	/**
	 * Day Trading Room Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Rebuilt to match Frontend/2 (Mastering the Trade) exactly
	 * Renamed: "Mastering the Trade" → "Day Trading Room"
	 *
	 * @version 2.0.0
	 */
	import { onMount } from 'svelte';
	// DEACTIVATED: Sidebar commented out for layout optimization
	// import TradingRoomSidebar from '$lib/components/dashboard/TradingRoomSidebar.svelte';

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
							<figure class="article-card__image" style="background-image: url({article.image});">
								<img src={article.image} alt={article.title} />
							</figure>
							{#if article.isVideo}
								<div class="article-card__type">
									<span class="label label--info">{article.type}</span>
								</div>
							{/if}
							<h4 class="h5 article-card__title"><a href={article.href}>{article.title}</a></h4>
							<span class="article-card__meta"><small>{article.date}</small></span>
							<div class="article-card__excerpt u--hide-read-more">
								<p>{article.excerpt}</p>
							</div>
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

	<!-- DEACTIVATED: New TradingRoomSidebar component - Commented out for layout optimization
	<TradingRoomSidebar planSlug="day-trading-room" />
	-->
</div>

<style>
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
		border-radius: 5px;
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

	.article-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		height: 200px;
		background-size: cover;
		background-position: center;
		margin: 0;
	}

	.article-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 12px 20px 0;
	}

	.label {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 3px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.label--info {
		background: #0984ae;
		color: #fff;
	}

	.article-card__title {
		padding: 12px 20px 0;
		margin: 0;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		font-size: 18px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: color 0.2s;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.article-card__meta {
		display: block;
		padding: 8px 20px 0;
		color: #999;
		font-size: 13px;
	}

	.article-card__excerpt {
		padding: 12px 20px;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
	}

	.article-card__excerpt p {
		margin: 0;
	}

	.u--hide-read-more {
		display: block;
	}

	.article-card .btn {
		margin: 0 20px 20px;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		box-shadow: none;
		display: inline-block;
		text-decoration: none;
		border-radius: 5px;
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
