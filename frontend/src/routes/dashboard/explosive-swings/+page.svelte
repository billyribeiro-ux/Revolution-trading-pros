<script lang="ts">
	/**
	 * Explosive Swings Dashboard - Trading Room
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Alert service dashboard for Explosive Swings members
	 * Follows Day Trading Room pattern with reusable components
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';
	import TradingRoomSidebar from '$lib/components/dashboard/TradingRoomSidebar.svelte';

	// SSR data from +page.server.ts
	let { data } = $props();

	// Latest alerts and updates
	const articles = [
		{
			id: 1,
			type: 'Trade Alert',
			title: 'NVDA Swing Trade Setup',
			date: 'January 10, 2026 at 2:30 PM ET',
			excerpt: 'Opening swing position on NVDA with bullish momentum. Multi-day hold targeting key resistance levels.',
			href: '/dashboard/explosive-swings/alerts/nvda-swing-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 2,
			type: 'Market Update',
			title: 'Weekly Swing Trade Outlook',
			date: 'January 10, 2026 at 9:00 AM ET',
			excerpt: 'Key swing trade setups for the week ahead. Technical analysis on major tech stocks and market leaders.',
			href: '/dashboard/explosive-swings/alerts/weekly-outlook-011026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 3,
			type: 'Trade Alert',
			title: 'TSLA Position Closed',
			date: 'January 9, 2026 at 3:45 PM ET',
			excerpt: 'Closing TSLA swing trade for profit. Target reached after 5-day hold.',
			href: '/dashboard/explosive-swings/alerts/tsla-close-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 4,
			type: 'Trade Alert',
			title: 'AMZN Breakout Play',
			date: 'January 9, 2026 at 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Swing trade entry with momentum confirmation.',
			href: '/dashboard/explosive-swings/alerts/amzn-breakout-010926',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 5,
			type: 'Market Update',
			title: 'Sector Rotation Analysis',
			date: 'January 8, 2026 at 4:30 PM ET',
			excerpt: 'Identifying the strongest sectors for swing trading opportunities. Technology and healthcare showing strength.',
			href: '/dashboard/explosive-swings/alerts/sector-analysis-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 6,
			type: 'Trade Alert',
			title: 'GOOGL Options Swing',
			date: 'January 8, 2026 at 10:00 AM ET',
			excerpt: 'Multi-week options swing trade on GOOGL. Targeting earnings catalyst with defined risk.',
			href: '/dashboard/explosive-swings/alerts/googl-options-010826',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		}
	];

	// Google Calendar integration
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

		// Cleanup function
		return () => {
			if (script.parentNode) {
				script.parentNode.removeChild(script);
			}
		};
	});

	function initCalendar() {
		const CLIENT_ID = '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
		const API_KEY = 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';
		const DISCOVERY_DOCS = ['https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest'];
		const SCOPES = 'https://www.googleapis.com/auth/calendar.readonly';

		// @ts-ignore
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
	<title>Explosive Swings | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<TradingRoomHeader 
	roomName="Explosive Swings" 
	startHereUrl="/dashboard/explosive-swings/start-here" 
/>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- WELCOME SECTION -->
		<section class="dashboard__content-section-member">
			<div class="welcome-banner">
				<h2>Welcome to Explosive Swings</h2>
				<p>Get high-probability swing trade alerts on stocks and options with explosive momentum. All trades include detailed entry, exit, and risk management strategies for multi-day holds.</p>
			</div>
		</section>

		<!-- LATEST ALERTS SECTION -->
		<LatestUpdates items={articles} title="Latest Alerts & Updates" roomSlug="explosive-swings" />

		<!-- PERFORMANCE METRICS SECTION -->
		<section class="dashboard__content-section u--background-color-white">
			<h3 class="section-heading">Recent Performance</h3>
			<div class="metrics-grid">
				<div class="metric-card">
					<div class="metric-value">82%</div>
					<div class="metric-label">Win Rate (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">+$18,750</div>
					<div class="metric-label">Total Profit (30 Days)</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">28</div>
					<div class="metric-label">Total Trades</div>
				</div>
				<div class="metric-card">
					<div class="metric-value">3.2:1</div>
					<div class="metric-label">Avg Risk/Reward</div>
				</div>
			</div>
		</section>

	</div>

	<!-- SIDEBAR - Using TradingRoomSidebar Component -->
	<TradingRoomSidebar planSlug="explosive-swings" />
</div>

<style>
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

	/* Sidebar styling handled by TradingRoomSidebar component */

	@media (min-width: 993px) {
		.dashboard__content {
			flex-direction: row;
		}
	}
</style>
