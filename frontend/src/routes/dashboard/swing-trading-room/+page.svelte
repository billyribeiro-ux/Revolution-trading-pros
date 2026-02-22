<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * Swing Trading Room Dashboard - Pixel-Perfect WordPress Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Based on Day Trading Room layout structure
	 * Customized for Swing Trading Room content
	 *
	 * @version 1.0.0 - January 2026
	 * @svelte5 Fully compliant with Svelte 5 runes and SvelteKit best practices
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import WeeklyWatchlist from '$lib/components/dashboard/WeeklyWatchlist.svelte';
	import LatestUpdates from '$lib/components/dashboard/LatestUpdates.svelte';
	import TradingRoomSidebar from '$lib/components/dashboard/TradingRoomSidebar.svelte';
	import type { WatchlistResponse } from '$lib/types/watchlist';

	// Props interface for SSR data - Svelte 5 best practice
	interface Props {
		data: {
			watchlist?: WatchlistResponse;
		};
	}

	// SSR data from +page.server.ts
	let props: Props = $props();
	let data = $derived(props.data);

	// Article data - matches WordPress structure
	const articles = [
		{
			id: 1,
			type: 'Daily Video',
			title: 'Swing Trading Market Analysis',
			date: 'January 14, 2026',
			excerpt:
				'Analyzing key swing trading opportunities in the current market environment. Looking at multi-day setups and position management strategies.',
			href: '/daily/swing-trading-room/market-analysis',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		{
			id: 2,
			type: 'Chatroom Archive',
			title: 'January 13, 2026',
			date: 'January 13, 2026',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/swing-trading-room/01132026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 3,
			type: 'Daily Video',
			title: 'Weekly Swing Trade Setups',
			date: 'January 12, 2026',
			excerpt:
				'Reviewing the best swing trading setups for the week ahead. Focus on technical analysis and risk management for multi-day positions.',
			href: '/daily/swing-trading-room/weekly-setups',
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			isVideo: true
		},
		{
			id: 4,
			type: 'Chatroom Archive',
			title: 'January 12, 2026',
			date: 'January 12, 2026',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/swing-trading-room/01122026',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 5,
			type: 'Daily Video',
			title: 'Position Management Strategies',
			date: 'January 9, 2026',
			excerpt:
				'How to manage swing trades effectively. Discussing entry points, stop losses, and profit targets for longer-term positions.',
			href: '/daily/swing-trading-room/position-management',
			image: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			isVideo: true
		},
		{
			id: 6,
			type: 'Chatroom Archive',
			title: 'January 9, 2026',
			date: 'January 9, 2026',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/swing-trading-room/01092026',
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
			logger.warn('Failed to load Google Calendar API');
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
			logger.warn('Google API not loaded yet');
			return;
		}

		try {
			// @ts-ignore
			gapi.load('client', () => {
				// @ts-ignore
				gapi.client
					.init({
						apiKey: API_KEY,
						clientId: CLIENT_ID,
						discoveryDocs: DISCOVERY_DOCS,
						scope: SCOPES
					})
					.then(() => {
						// @ts-ignore
						return gapi.client.calendar.events.list({
							calendarId: 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com',
							timeMin: new Date().toISOString(),
							showDeleted: false,
							singleEvents: true,
							maxResults: 10,
							orderBy: 'startTime',
							fields: 'items(summary,start/dateTime)'
						});
					})
					.then((response: any) => {
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
					})
					.catch((error: any) => {
						logger.warn('Calendar Error:', error);
					});
			});
		} catch (error) {
			logger.warn('Failed to initialize Google Calendar:', error);
		}
	}
</script>

<svelte:head>
	<title>Swing Trading Room | Revolution Trading Pros</title>
	<script src="https://apis.google.com/js/api.js"></script>
</svelte:head>

<TradingRoomHeader
	roomName="Swing Trading Room"
	startHereUrl="/dashboard/swing-trading-room/start-here"
/>

<!-- DASHBOARD CONTENT - Exact WordPress Structure -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- VIDEO TUTORIAL SECTION -->
		<section class="dashboard__content-section-member">
			<video
				controls
				width="100%"
				poster="https://cdn.simplertrading.com/2025/06/03161600/SCR-20250603-nmuc.jpeg"
				style="aspect-ratio: 2 / 1;"
			>
				<source
					src="https://simpler-options.s3.amazonaws.com/tutorials/SwingTrading_tutorial2026.mp4"
					type="video/mp4"
				/>
				Your browser does not support the video tag.
			</video>
		</section>

		<!-- LATEST UPDATES SECTION -->
		<LatestUpdates items={articles} title="Latest Updates" roomSlug="swing-trading-room" />

		<!-- WEEKLY WATCHLIST SECTION - SSR pre-fetched for 0ms loading -->
		<div class="dashboard__content-section u--background-color-white">
			<WeeklyWatchlist data={(data as { watchlist?: any }).watchlist} />
		</div>
	</div>

	<!-- SIDEBAR - Using TradingRoomSidebar Component -->
	<TradingRoomSidebar planSlug="swing-trading-room" />
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch Targets: 44x44px minimum
	   Safe Areas: env(safe-area-inset-*) for notched devices
	   Dynamic Viewport: 100dvh for full-screen trading views
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-First Base - Stack sidebar below content on mobile */
	.dashboard__content {
		display: flex;
		flex-direction: column;
		min-height: 100dvh;
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
		border-right: none;
		border-bottom: 1px solid #dbdbdb;
	}

	/* Video Tutorial Section - Mobile-First */
	.dashboard__content-section-member {
		padding: 16px;
		padding-left: max(16px, env(safe-area-inset-left, 16px));
		padding-right: max(16px, env(safe-area-inset-right, 16px));
	}

	.dashboard__content-section-member video {
		width: 100%;
		aspect-ratio: 16 / 9;
		border-radius: 8px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
		background: #000;
		object-fit: contain;
	}

	/* Content Sections - Mobile-First */
	.dashboard__content-section {
		padding: 16px;
		padding-left: max(16px, env(safe-area-inset-left, 16px));
		padding-right: max(16px, env(safe-area-inset-right, 16px));
		overflow-x: auto;
		overflow-y: hidden;
		-webkit-overflow-scrolling: touch;
	}

	/* Weekly Watchlist Section */
	.u--background-color-white {
		background-color: #fff !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Progressive Enhancement
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media screen and (min-width: 360px) {
		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 20px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media screen and (min-width: 640px) {
		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 24px;
		}
	}

	/* md: 768px+ - Tablets */
	@media screen and (min-width: 768px) {
		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 28px;
		}
	}

	/* lg: 1024px+ - Desktop - Switch to row layout */
	@media screen and (min-width: 1024px) {
		.dashboard__content {
			flex-direction: row;
			flex-wrap: nowrap;
		}

		.dashboard__content-main {
			border-right: 1px solid #dbdbdb;
			border-bottom: none;
		}

		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 30px;
		}
	}

	/* xl: 1280px+ - Large desktop */
	@media screen and (min-width: 1280px) {
		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 36px;
		}
	}

	/* xxl: 1440px+ - Extra large desktop */
	@media screen and (min-width: 1440px) {
		.dashboard__content-section-member,
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* Landscape orientation on mobile - optimize video viewing */
	@media screen and (max-width: 1023px) and (orientation: landscape) {
		.dashboard__content-section-member {
			padding: 12px 24px;
		}

		.dashboard__content-section-member video {
			max-height: 70dvh;
			width: auto;
			margin: 0 auto;
			display: block;
		}
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.dashboard__content-section-member video {
			transition: none;
		}
	}
</style>
