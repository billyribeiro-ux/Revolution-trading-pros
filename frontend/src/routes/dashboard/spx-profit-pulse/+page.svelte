<script lang="ts">
	/**
	 * SPX Profit Pulse Dashboard - Trading Room
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Alert service dashboard for SPX Profit Pulse members
	 * Follows Day Trading Room pattern with reusable components
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';

	// SSR data from +page.server.ts
	let { data } = $props();

	// Latest alerts and updates
	const articles = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'SPX 0DTE Call Spread',
			date: 'January 10, 2026 at 9:45 AM ET',
			excerpt: 'Opening bullish call spread on SPX expiring today. Entry at market open with tight risk management.',
			href: '/dashboard/spx-profit-pulse/alerts/spx-call-spread-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'SPX Technical Analysis',
			date: 'January 10, 2026 at 8:30 AM ET',
			excerpt: 'Key levels to watch for today\'s 0DTE trading session. Support and resistance zones identified.',
			href: '/dashboard/spx-profit-pulse/alerts/spx-analysis-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'SPX Put Spread Closed',
			date: 'January 9, 2026 at 2:15 PM ET',
			excerpt: 'Closing bearish put spread for profit. Target reached ahead of schedule.',
			href: '/dashboard/spx-profit-pulse/alerts/spx-put-close-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'SPX Iron Condor Setup',
			date: 'January 9, 2026 at 10:00 AM ET',
			excerpt: 'Neutral strategy for range-bound market conditions. High probability setup with defined risk.',
			href: '/dashboard/spx-profit-pulse/alerts/spx-condor-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 5,
			type: 'Market Update',
			title: 'Weekly SPX Outlook',
			date: 'January 8, 2026 at 4:00 PM ET',
			excerpt: 'Market analysis and key levels for the upcoming week. Economic calendar highlights.',
			href: '/dashboard/spx-profit-pulse/alerts/weekly-outlook-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 6,
			type: 'Trade Alert',
			title: 'SPX Butterfly Spread',
			date: 'January 8, 2026 at 11:30 AM ET',
			excerpt: 'Advanced options strategy targeting specific price level. Limited risk with high reward potential.',
			href: '/dashboard/spx-profit-pulse/alerts/spx-butterfly-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		}
	];

	// Google Calendar integration for alert schedule
	onMount(() => {
		const script = document.createElement('script');
		script.src = 'https://apis.google.com/js/api.js';
		script.onload = () => {
			setTimeout(initCalendar, 100);
		};
		script.onerror = () => {
			console.warn('Failed to load Google Calendar API');
		};
		document.head.appendChild(script);
	});

	function initCalendar() {
		const CLIENT_ID = '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
		const API_KEY = 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';
		const DISCOVERY_DOCS = ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'];
		const SCOPES = 'https://www.googleapis.com/auth/calendar.readonly';

		// @ts-ignore - gapi is loaded from external script
		if (typeof gapi === 'undefined' || !gapi.client) {
			console.warn('Google API not loaded yet');
			return;
		}

		try {
			// @ts-ignore
			gapi.load('client', () => {
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
					console.warn('Calendar Error:', error);
				});
			});
		} catch (error) {
			console.warn('Failed to initialize Google Calendar:', error);
		}
	}
</script>

<svelte:head>
	<title>SPX Profit Pulse | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<TradingRoomHeader 
	roomName="SPX Profit Pulse" 
	startHereUrl="/dashboard/spx-profit-pulse/start-here" 
/>

<!-- DASHBOARD CONTENT - Exact WordPress Structure -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- WELCOME SECTION -->
		<section class="dashboard__content-section-member">
			<div class="welcome-banner">
				<h2>Welcome to SPX Profit Pulse</h2>
				<p>Get real-time 0DTE SPX options alerts and market analysis from our expert traders. All trades include entry, exit, and risk management details.</p>
			</div>
		</section>

		<!-- LATEST ALERTS SECTION -->
		<LatestUpdates items={articles} title="Latest Alerts & Updates" roomSlug="spx-profit-pulse" />

		<!-- PERFORMANCE METRICS SECTION -->
		<section class="dashboard__content-section u--background-color-white">
			<h3 class="section-heading">Recent Performance</h3>
			<div class="metrics-grid">
				<div class="metric-card">
					<div class="metric-value">87%</div>
					<div class="metric-label">Win Rate (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">+$12,450</div>
					<div class="metric-label">Total Profit (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">42</div>
					<div class="metric-label">Total Trades</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">1.8:1</div>
					<div class="metric-label">Avg Risk/Reward</div>
				</div>
			</div>
		</section>

	</div>

	<!-- SIDEBAR (PANEL 2) -->
	<aside class="dashboard__content-sidebar">
		<!-- ALERT SCHEDULE -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				Alert Schedule
				<p class="pssubject" style="font-size: 10px;margin-top: 15px;text-transform: initial;text-align: center;">Schedule is subject to change.</p>
			</h4>
			<div class="script-container">
				<div class="room-sched"></div>
			</div>
		</section>

		<!-- QUICK LINKS -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Quick Links</h4>
			<ul class="link-list">
				<li><a href="/dashboard/spx-profit-pulse/alerts">View All Alerts</a></li>
				<li><a href="https://intercom.help/simpler-trading/en/" target="_blank">Support</a></li>
				<li><a href="/tutorials" target="_blank">Platform Tutorials</a></li>
				<li><a href="/blog" target="_blank">Trading Blog</a></li>
			</ul>
		</section>

		<!-- ALERT SETTINGS -->
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">Alert Settings</h4>
			<ul class="link-list">
				<li><a href="/dashboard/account">Notification Preferences</a></li>
				<li><a href="/dashboard/account">SMS Alerts</a></li>
				<li><a href="/dashboard/account">Email Alerts</a></li>
			</ul>
		</section>
	</aside>
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

	/* Welcome Section */
	.dashboard__content-section-member {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}

	.welcome-banner {
		background: linear-gradient(135deg, #143E59 0%, #0984ae 100%);
		color: #fff;
		padding: 40px;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	.welcome-banner h2 {
		margin: 0 0 15px 0;
		font-size: 32px;
		font-weight: 600;
		font-family: 'Montserrat', sans-serif;
	}

	.welcome-banner p {
		margin: 0;
		font-size: 16px;
		line-height: 1.6;
		opacity: 0.95;
	}

	/* Content Sections */
	.dashboard__content-section {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	.u--background-color-white {
		background-color: #fff !important;
	}

	.section-heading {
		margin: 0 0 25px 0;
		font-size: 24px;
		font-weight: 600;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	/* Performance Metrics */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 20px;
	}

	@media screen and (min-width: 640px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (min-width: 1024px) {
		.metrics-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.metric-card {
		background: #f8f9fa;
		padding: 25px;
		border-radius: 8px;
		text-align: center;
		border: 1px solid #e0e0e0;
		transition: all 0.2s ease;
	}

	.metric-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.metric-value {
		font-size: 32px;
		font-weight: 700;
		color: #143E59;
		margin-bottom: 8px;
		font-family: 'Montserrat', sans-serif;
	}

	.metric-label {
		font-size: 14px;
		color: #666;
		font-weight: 500;
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

	/* Mobile-first: sidebar hidden by default, shown on lg (1024px+) */
	.dashboard__content-sidebar {
		display: none;
	}

	@media (min-width: 1080px) {
		.dashboard__content-sidebar {
			display: block;
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

	/* Mobile-first: content stacked by default, row on lg+ */
	@media (min-width: 993px) {
		.dashboard__content {
			flex-direction: row;
		}
	}
</style>
