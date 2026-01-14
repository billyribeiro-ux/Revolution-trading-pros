<script lang="ts">
	/**
	 * Day Trading Room Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Rebuilt to match Frontend/2 (Mastering the Trade) exactly
	 * Renamed: "Mastering the Trade" → "Day Trading Room"
	 *
	 * @version 2.1.0 - SSR WeeklyWatchlist
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';
	import TradingRoomSidebar from '$lib/components/dashboard/TradingRoomSidebar.svelte';

	// SSR data from +page.server.ts
	let { data } = $props();

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
		// Load Google Calendar API with error handling
		const script = document.createElement('script');
		script.src = 'https://apis.google.com/js/api.js';
		script.onload = () => {
			// Wait for gapi to be fully loaded
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
	<title>Day Trading Room | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<TradingRoomHeader 
	roomName="Day Trading Room" 
	startHereUrl="/dashboard/day-trading-room/start-here" 
/>

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
		<LatestUpdates items={articles} title="Latest Updates" roomSlug="day-trading-room" />

		<!-- WEEKLY WATCHLIST SECTION - SSR pre-fetched for 0ms loading -->
		<div class="dashboard__content-section u--background-color-white">
			<WeeklyWatchlist data={(data as { watchlist?: any }).watchlist} />
		</div>

	</div>

	<!-- SIDEBAR - Using TradingRoomSidebar Component -->
	<TradingRoomSidebar planSlug="day-trading-room" />
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

	/* Weekly Watchlist Section */
	.u--background-color-white {
		background-color: #fff !important;
	}

	/* Sidebar styling handled by TradingRoomSidebar component */

	/* Mobile-first: content stacked by default, row on lg+ */
	@media (min-width: 993px) {
		.dashboard__content {
			flex-direction: row;
		}
	}
</style>
