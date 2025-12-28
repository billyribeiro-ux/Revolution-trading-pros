<script lang="ts">
	/**
	 * Dynamic Trading Room Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Renders trading room dashboard based on configuration
	 * Pixel-perfect match to WordPress reference
	 *
	 * @version 2.0.0 - Dynamic Routes
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Tabler Icons for dropdown menu
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';

	let { data }: { data: any } = $props();

	// Trading Room dropdown toggle state - WordPress style click-based toggle
	let dropdownOpen = $state(false);
	let dropdownRef = $state<HTMLElement | null>(null);

	// Toggle dropdown on button click
	function toggleDropdown(e: Event) {
		e.preventDefault();
		e.stopPropagation();
		dropdownOpen = !dropdownOpen;
	}

	// Close dropdown when clicking outside
	function handleClickOutside(e: MouseEvent) {
		if (dropdownRef && !dropdownRef.contains(e.target as Node)) {
			dropdownOpen = false;
		}
	}

	// Close dropdown on Escape key
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && dropdownOpen) {
			dropdownOpen = false;
		}
	}

	// Setup click-outside listener
	$effect(() => {
		if (browser && dropdownOpen) {
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleKeydown);
			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleKeydown);
			};
		}
	});
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	// Fallback placeholder for video cards (WordPress reference)
	const FALLBACK_PLACEHOLDER = 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';

	// Article data - will be fetched from API later
	// Using $derived to properly reference reactive slug value in closures
	const articles = $derived([
		{
			id: 1,
			type: 'daily-video',
			label: 'Daily Video',
			title: "Santa's On His Way",
			date: 'December 23, 2025 with HG',
			excerpt: 'Things can always change, but given how the market closed on Tuesday, it looks like Santa\'s on his way. Let\'s look at the facts, then also some preferences and opinions as we get into the end of 2025.',
			href: `/daily/${slug}/santas-on-his-way`,
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg'
		},
		{
			id: 2,
			type: 'chatroom-archive',
			title: 'December 23, 2025',
			date: 'December 23, 2025',
			traderName: 'Danielle Shay',
			href: `/chatroom-archive/${slug}/12232025`,
			image: FALLBACK_PLACEHOLDER
		},
		{
			id: 3,
			type: 'daily-video',
			label: 'Daily Video',
			title: 'Setting Up for the Santa Rally',
			date: 'December 22, 2025 with Danielle Shay',
			excerpt: 'Everything looks good for a potential rally, as the indexes are consolidating and breaking higher, along with a lot of key stocks. Let\'s take a look at TSLA, GOOGL, AMZN, AVGO, MSFT, and more.',
			href: `/daily/${slug}/setting-up-santa-rally`,
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg'
		},
		{
			id: 4,
			type: 'chatroom-archive',
			title: 'December 22, 2025',
			date: 'December 22, 2025',
			traderName: 'Henry Gambell',
			href: `/chatroom-archive/${slug}/12222025`,
			image: FALLBACK_PLACEHOLDER
		},
		{
			id: 5,
			type: 'daily-video',
			label: 'Daily Video',
			title: 'Holiday Weekend Market Review',
			date: 'December 19, 2025 with Sam',
			excerpt: 'Indexes continue to churn sideways as we approach next week\'s holiday trade. Bulls usually take over in low volume. Can they do it again?',
			href: `/daily/${slug}/holiday-weekend-market-review`,
			image: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg'
		},
		{
			id: 6,
			type: 'chatroom-archive',
			title: 'December 19, 2025',
			date: 'December 19, 2025',
			traderName: 'Bruce Marshall',
			href: `/chatroom-archive/${slug}/12192025`,
			image: FALLBACK_PLACEHOLDER
		}
	]);

	// Weekly Watchlist - Dynamic date calculation
	const weeklyWatchlistDate = $derived(() => {
		const now = new Date();
		// Get the Monday of the current week
		const day = now.getDay();
		const diff = now.getDate() - day + (day === 0 ? -6 : 1);
		const monday = new Date(now.setDate(diff));
		return monday.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
	});

	// Weekly Watchlist - Dynamic URL based on date
	const weeklyWatchlistUrl = $derived(() => {
		const now = new Date();
		const day = now.getDay();
		const diff = now.getDate() - day + (day === 0 ? -6 : 1);
		const monday = new Date(now.setDate(diff));
		const month = String(monday.getMonth() + 1).padStart(2, '0');
		const dayNum = String(monday.getDate()).padStart(2, '0');
		const year = monday.getFullYear();
		return `/watchlist/${month}${dayNum}${year}-tg-watkins`;
	});

	// Google Calendar integration
	onMount(() => {
		if (room.calendarId) {
			const script = document.createElement('script');
			script.src = 'https://apis.google.com/js/api.js';
			script.onload = initCalendar;
			document.head.appendChild(script);
		}
	});

	function initCalendar() {
		if (!room.calendarId || !room.calendarApiKey || !room.calendarClientId) return;

		const DISCOVERY_DOCS = ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'];
		const SCOPES = 'https://www.googleapis.com/auth/calendar.readonly';

		// @ts-ignore - gapi is loaded from external script
		if (typeof gapi !== 'undefined') {
			// @ts-ignore
			gapi.client.init({
				apiKey: room.calendarApiKey,
				clientId: room.calendarClientId,
				discoveryDocs: DISCOVERY_DOCS,
				scope: SCOPES
			}).then(() => {
				// @ts-ignore
				return gapi.client.calendar.events.list({
					'calendarId': room.calendarId,
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
	<title>{room.name} | Revolution Trading Pros</title>
	{#if room.calendarId}
		<script src="https://apis.google.com/js/api.js"></script>
	{/if}
</svelte:head>

<!-- DASHBOARD HEADER -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{room.title}</h1>
		{#if room.features.startHere}
			<a href="/dashboard/{slug}/start-here" class="btn btn-xs btn-default">
				New? Start Here
			</a>
		{/if}
	</div>
	<div class="dashboard__header-right">
		{#if room.type === 'trading-room'}
			<ul class="ultradingroom">
				<li class="litradingroom">
					<a href="https://cdn.simplertrading.com/2024/02/07192341/Simpler-Tradings-Rules-of-the-Room.pdf" target="_blank" class="btn btn-xs btn-link">Trading Room Rules</a>
				</li>
				<li class="litradingroomhind">
					By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
				</li>
			</ul>
			<div class="dropdown display-inline-block" class:show={dropdownOpen} bind:this={dropdownRef}>
				<button
					type="button"
					class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
					id="dLabel"
					data-bs-toggle="dropdown"
					aria-expanded={dropdownOpen}
					onclick={toggleDropdown}
				>
					<strong>Enter a Trading Room</strong>
				</button>
				<nav class="dropdown-menu dropdown-menu--full-width" class:show={dropdownOpen} aria-labelledby="dLabel">
					<ul class="dropdown-menu__menu">
						<li>
							<a href="/dashboard/{slug}" target="_blank" rel="nofollow">
								<span class="dropdown-icon dropdown-icon--day-trading">
									<IconChartLine size={20} />
								</span>
								{room.name}
							</a>
						</li>
						<li>
							<a href="/dashboard/simpler-showcase" target="_blank" rel="nofollow">
								<span class="dropdown-icon dropdown-icon--showcase">
									<IconTrophy size={20} />
								</span>
								Simpler Showcase Room
							</a>
						</li>
					</ul>
				</nav>
			</div>
		{/if}
	</div>
</header>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- VIDEO TUTORIAL SECTION -->
		{#if room.tutorialVideo}
			<section class="dashboard__content-section-member">
				<video controls width="100%" poster={room.tutorialVideo.poster} style="aspect-ratio: 2 / 1;">
					<source src={room.tutorialVideo.src} type="video/mp4">
					Your browser does not support the video tag.
				</video>
			</section>
		{/if}

		<!-- LATEST UPDATES SECTION - WordPress reference core 1:3143-3300 -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Latest Updates</h2>
			<div class="article-cards row flex-grid">
				{#each articles as article (article.id)}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							{#if article.type === 'daily-video'}
								<!-- DAILY VIDEO CARD - WordPress core 1:3148-3167 -->
								<figure class="article-card__image" style="background-image: url({article.image});">
									<img src={FALLBACK_PLACEHOLDER} alt={article.title} />
									<div class="article-card__type">
										<span class="label label--info">{article.label}</span>
									</div>
								</figure>
								<h4 class="h5 article-card__title"><a href={article.href}>{article.title}</a></h4>
								<span class="article-card__meta"><small>{article.date}</small></span>
								<div class="article-card__excerpt u--hide-read-more">
									<p>{article.excerpt}</p>
								</div>
								<a href={article.href} class="btn btn-tiny btn-default">Watch Now</a>
							{:else}
								<!-- CHATROOM ARCHIVE CARD - WordPress core 1:3169-3189 -->
								<figure class="card-media article-card__image card-media--video" style="background-image: url({article.image});">
									<a href={article.href}>
										<img src={article.image} alt={article.title} />
									</a>
								</figure>
								<h4 class="h5 article-card__title"><a href={article.href}>{article.title}</a></h4>
								<div class="article-card__excerpt u--hide-read-more">
									<p class="u--margin-bottom-0 u--font-size-sm"><i>With {article.traderName}</i></p>
								</div>
								<span class="article-card__meta"><small>{article.date}</small></span>
								<a href={article.href} class="btn btn-tiny btn-default">Watch Now</a>
							{/if}
						</article>
					</div>
				{/each}
			</div>
		</section>

		<!-- WEEKLY WATCHLIST SECTION - WordPress reference core:2952-2976 -->
		{#if room.watchlistImage}
			<div class="dashboard__content-section u--background-color-white">
				<section>
					<div class="row">
						<div class="col-sm-6 col-lg-5">
							<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
							<div class="hidden-md d-lg-none pb-2">
								<a href={weeklyWatchlistUrl()}>
									<img src={room.watchlistImage} alt="Weekly Watchlist" class="u--border-radius">
								</a>
							</div>
							<h4 class="h5 u--font-weight-bold">Weekly Watchlist with TG Watkins</h4>
							<div class="u--hide-read-more">
								<p>Week of {weeklyWatchlistDate()}.</p>
							</div>
							<a href={weeklyWatchlistUrl()} class="btn btn-tiny btn-default">Watch Now</a>
						</div>
						<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
							<a href={weeklyWatchlistUrl()}>
								<img src={room.watchlistImage} alt="Weekly Watchlist" class="u--border-radius">
							</a>
						</div>
					</div>
				</section>
			</div>
		{/if}

	</div>

	<!-- SIDEBAR (PANEL 2) -->
	<aside class="dashboard__content-sidebar">
		{#if room.calendarId}
			<section class="content-sidebar__section">
				<h4 class="content-sidebar__heading">
					Trading Room Schedule
					<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;">Schedule is subject to change.</p>
				</h4>
				<div class="script-container">
					<div class="room-sched"></div>
				</div>
			</section>
		{/if}

		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				{#each room.quickLinks as link}
					<li>
						<a href={link.href} target={link.external ? '_blank' : undefined}>
							{link.label}
						</a>
					</li>
				{/each}
			</ul>
		</section>
	</aside>
</div>

<style>
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

	/* WordPress reference: font-size: 36px (core 1:3087) */
	.dashboard__page-title {
		font-size: 36px;
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

	.btn-orange,
	.btn-tradingroom {
		background-color: #F69532;
		color: #fff;
		border-color: #F69532;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.15s ease-in-out;
	}

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
		/* Phase 5: WordPress reference box-shadow: 0 10px 30px rgba(0,0,0,0.15) */
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		min-width: 200px;
		display: none;
		font-size: 14px;
		/* Animation for smooth open/close */
		opacity: 0;
		transform: translateY(-5px);
		transition: opacity 0.15s ease-in-out, transform 0.15s ease-in-out;
	}

	/* Click-based toggle (WordPress style with data-bs-toggle) */
	.dropdown-menu.show {
		display: block;
		opacity: 1;
		transform: translateY(0);
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

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
	}

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
	   ARTICLE CARD - Pixel-perfect match to Simpler Trading reference
	   Reference: dashboard-globals.css lines 931-1021
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

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		background-color: #0984ae; /* Fallback color */
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

	/* Chatroom Archive: Show image in figure with link */
	.article-card__image.card-media--video img {
		opacity: 1;
	}

	.article-card__image.card-media--video a {
		position: absolute;
		top: 0;
		left: 0;
		display: block;
		width: 100%;
		height: 100%;
	}

	/* Video Play Icon Overlay */
	.card-media--video {
		position: relative;
		cursor: pointer;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background-color: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='white'%3E%3Cpath d='M8 5v14l11-7z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: center;
		background-size: 28px 28px;
		transition: all 0.2s ease-in-out;
		pointer-events: none;
	}

	.card-media--video:hover::after {
		background-color: rgba(9, 132, 174, 0.8);
		transform: translate(-50%, -50%) scale(1.1);
	}

	/* Type label - Absolute positioned overlay on image */
	.article-card__type {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 2;
		margin: 0;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px; /* Pill shape */
	}

	.label--info {
		background: #0984ae;
		color: #fff;
	}

	.article-card__title {
		margin: 0;
		padding: 15px 15px 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.3;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
		transition: color 0.2s;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.h5 {
		font-size: 16px;
		font-weight: 700;
	}

	.article-card__meta {
		display: block;
		padding: 0 15px;
		color: #999;
		font-size: 12px;
	}

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

	.article-card .btn {
		margin: 0 15px 15px;
	}

	.btn-tiny {
		padding: 5px 10px;
		font-size: 11px;
		line-height: 1.5;
		border-radius: 3px;
	}

	/* Watch Now button - Orange style from Learning-Center reference */
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
	}

	.u--background-color-white {
		background-color: #fff !important;
	}

	/* Utility classes for chatroom archive cards - WordPress reference */
	.u--margin-bottom-0 {
		margin-bottom: 0 !important;
	}

	.u--font-size-sm {
		font-size: 13px !important;
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

	@media (max-width: 992px) {
		.dashboard__content {
			flex-direction: column;
		}
	}
</style>
