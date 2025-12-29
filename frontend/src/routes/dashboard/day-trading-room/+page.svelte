<script lang="ts">
	/**
	 * Day Trading Room Dashboard - Svelte 5 Component-Based Architecture
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Refactored to use modular Svelte 5 components with scoped CSS.
	 *
	 * @version 3.0.0 - Svelte 5 with component-based architecture
	 */
	import { onMount } from 'svelte';

	// Import Svelte 5 Dashboard Components
	import {
		DashboardHeader,
		ArticleCard,
		WeeklyWatchlistSection,
		SectionTitle
	} from '$lib/components/dashboard';

	// Tabler Icons for dropdown menu
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';

	// Trading rooms for header dropdown
	const tradingRooms = [
		{ id: 1, name: 'Day Trading Room', slug: 'day-trading-room', roomLabel: 'Day Trading Room' },
		{ id: 2, name: 'Simpler Showcase', slug: 'simpler-showcase', roomLabel: 'Simpler Showcase Room' }
	];

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
<DashboardHeader
	title="Day Trading Room Dashboard"
	tradingRooms={tradingRooms}
>
	{#snippet rightContent()}
		<a href="/dashboard/day-trading-room/start-here" class="btn-start-here">
			New? Start Here
		</a>
	{/snippet}
</DashboardHeader>

<!-- DASHBOARD CONTENT -->
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
			<SectionTitle title="Latest Updates" />
			<div class="article-cards">
				{#each articles as article (article.id)}
					<div class="article-cards__item">
						<ArticleCard
							title={article.title}
							href={article.href}
							image={article.image}
							label={article.isVideo ? article.type : undefined}
							meta={article.date}
							excerpt={article.isVideo ? article.excerpt : undefined}
							buttonText="Watch Now"
						/>
					</div>
				{/each}
			</div>
		</section>

		<!-- WEEKLY WATCHLIST SECTION -->
		<WeeklyWatchlistSection
			title="Weekly Watchlist"
			featuredTitle="Weekly Watchlist with TG Watkins"
			description="Week of December 22, 2025."
			href="/watchlist/latest"
		/>

	</div>

	<!-- SIDEBAR -->
	<aside class="dashboard__content-sidebar">
		<!-- TRADING ROOM SCHEDULE -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Trading Room Schedule
				<p class="schedule-note">Schedule is subject to change.</p>
			</h4>
			<div class="room-sched"></div>
		</section>

		<!-- QUICK LINKS -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener noreferrer">Support</a></li>
				<li><a href="/tutorials" target="_blank" rel="noopener noreferrer">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank" rel="noopener noreferrer">Trading Blog</a></li>
			</ul>
		</section>
	</aside>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE-SPECIFIC STYLES
	   Component styles (header, cards, watchlist) come from components
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Start Here Button */
	.btn-start-here {
		display: inline-block;
		padding: 4px 10px;
		font-size: 12px;
		font-weight: 700;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		border-radius: 3px;
		text-decoration: none;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
	}

	.btn-start-here:hover {
		background: #e8e8e8;
		border-color: #ccc;
	}

	/* Dashboard Content Layout */
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

	@media (min-width: 1280px) {
		.dashboard__content-section-member { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.dashboard__content-section-member { padding: 40px; }
	}

	.dashboard__content-section-member video {
		width: 100%;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	/* Content Section */
	.dashboard__content-section {
		padding: 30px 20px;
		background-color: #fff;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.dashboard__content-section { padding: 40px; }
	}

	/* Article Cards Grid */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.article-cards__item {
		display: flex;
		min-width: 0;
	}

	/* Sidebar */
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
		.dashboard__content-sidebar { display: none; }
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

	.schedule-note {
		font-size: 10px;
		margin-top: 15px;
		text-transform: initial;
		letter-spacing: normal;
		font-weight: 400;
		color: #666;
	}

	/* Calendar event styles */
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

	/* Quick Links */
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
