<!--
	Indicator Detail & Download Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: volume-max-model file
	URL: /indicators/volume-max-i
	
	Features:
	- Indicator name and details
	- Available platforms display
	- Training videos section
	- Platform-specific download sections (ThinkorSwim, TradingView)
	- Supporting documentation links
	- "Have Questions?" footer section
	- Matches original WordPress structure IDENTICALLY
-->
<script lang="ts">
	import { page } from '$app/state';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';
	import IndicatorHeader from '$lib/components/indicators/IndicatorHeader.svelte';
	import TrainingVideosSection from '$lib/components/indicators/TrainingVideosSection.svelte';
	import PlatformDownloads from '$lib/components/indicators/PlatformDownloads.svelte';
	import SupportDocsSection from '$lib/components/indicators/SupportDocsSection.svelte';

	interface DownloadFile {
		name: string;
		downloadUrl: string;
	}

	interface PlatformDownloads {
		platform: string;
		logo: string;
		files: DownloadFile[];
		notes?: string;
	}

	interface TrainingVideo {
		id: string;
		title: string;
		videoUrl: string;
		posterUrl: string;
	}

	interface SupportDoc {
		title: string;
		url: string;
	}

	interface Indicator {
		id: string;
		name: string;
		platforms: string[];
		trainingVideos: TrainingVideo[];
		downloads: PlatformDownloads[];
		supportDocs: SupportDoc[];
	}

	// Get indicator ID from URL
	let indicator = $derived.by(() => {
		const id = page.params.id;
		// TODO: Replace with actual API call to your backend
		// Example: const response = await fetch(`/api/indicators/${id}`);
		// const data = await response.json();
		return {
			id: id,
			name: 'Volume Max Tool Kit (formerly VWAP)',
			platforms: ['ThinkorSwim', 'TradingView'],
			trainingVideos: [
				{
					id: '1',
					title: 'Indicator Setup and Q&A, with Eric Purdy',
					videoUrl:
						'https://cloud-streaming.s3.amazonaws.com/classes/SubmarketSonar-IndicatorSetup-EP.mp4',
					posterUrl:
						'https://cdn.simplertrading.com/2020/03/25163022/simpler-geenric-video-bg-768x432.jpg'
				},
				{
					id: '2',
					title: 'VWAP Training and Live Trading Recorded Session with Raghee Horner',
					videoUrl:
						'https://s3.amazonaws.com/cloud-streaming/chatrecordings%2FSTWebinars/1176-RH-06-12-2019__09.01.394_AM.mp4',
					posterUrl:
						'https://cdn.simplertrading.com/2020/03/25163022/simpler-geenric-video-bg-768x432.jpg'
				}
			],
			downloads: [
				{
					platform: 'ThinkorSwim',
					logo: '/logos/platforms/thinkorswim.png',
					files: [
						{
							name: 'Volume Max Indicator',
							downloadUrl: '/?st-download-file=452914b18dc78691e6c98731b9e094fe'
						},
						{
							name: 'VScore EOD Study',
							downloadUrl: '/?st-download-file=12ab53eb7e85a005a6a21f800db57777'
						},
						{
							name: 'VScore Intraday Signals Study',
							downloadUrl: '/?st-download-file=4bd68f7bbfec21d14c099258f6833a9c'
						},
						{
							name: 'VScore Bands EOD Study',
							downloadUrl: '/?st-download-file=4fb2197916c551be4acb037afccc607f'
						},
						{
							name: 'VScore Bands Intraday Study',
							downloadUrl: '/?st-download-file=560ebea65a1229c04c0c268107fdb693'
						},
						{
							name: 'VProfile EOD Study',
							downloadUrl: '/?st-download-file=da8f1193496d5cf4bed2cdd049a2d70b'
						},
						{
							name: 'VProfile Intraday Study',
							downloadUrl: '/?st-download-file=ab0357f17694826819f632d50f6cfccc'
						},
						{
							name: 'Volume Labels',
							downloadUrl: '/?st-download-file=7aa9537ae79f082d9b8ff2ede9c2561b'
						}
					]
				},
				{
					platform: 'TradingView',
					logo: '/logos/platforms/tradingview.png',
					files: [],
					notes:
						'Please email your TradingView Username to <a href="mailto:support@simplertrading.com">support@simplertrading.com</a>. The TradingView chart indicator is very easy to get set up in your online charting profile. Once you are logged into TradingView, locate your Notifications area. Once we receive your Username, you should have a notification letting you know the new chart study has been made available. Clicking on the notification will bring up a chart for the study to be displayed.'
				}
			],
			supportDocs: [
				{
					title: 'TOS Installation Guide',
					url: 'https://intercom.help/simpler-trading/en/articles/3263969'
				},
				{
					title: 'Troubleshooting within TOS',
					url: 'https://intercom.help/simpler-trading/en/articles/3481530-tos-indicator-will-not-import'
				},
				{
					title: 'TradingView - Installing an Indicator',
					url: 'https://intercom.help/simpler-trading/en/articles/3498380'
				},
				{
					title: 'TradingView - What is my Username',
					url: 'https://intercom.help/simpler-trading/en/articles/3606861'
				},
				{
					title: 'What is Volume Max Tool Kit (formerly VWAP)?',
					url: 'https://intercom.help/simpler-trading/en/articles/3160130'
				},
				{
					title: 'What are Volume Max Tool Kit (formerly VWAP) Settings?',
					url: 'https://intercom.help/simpler-trading/en/articles/3311644'
				},
				{
					title: 'What is VScore?',
					url: 'https://intercom.help/simpler-trading/en/articles/3210561'
				},
				{
					title: 'VScore Settings and Configuration',
					url: 'https://intercom.help/simpler-trading/en/articles/3210408'
				},
				{
					title: 'What is VProfile?',
					url: 'https://intercom.help/simpler-trading/en/articles/3341484'
				},
				{
					title: 'What are VProfile Settings?',
					url: 'https://intercom.help/simpler-trading/en/articles/3341233'
				}
			]
		} as Indicator;
	});
</script>

<svelte:head>
	<title>{indicator.name} | Revolution Trading Pros</title>
</svelte:head>

<!-- Breadcrumbs - Auto-generated from path -->
<DashboardBreadcrumbs />

<!-- Main Content - Matches WordPress exactly -->
<!-- WordPress structure: #page > #content.site-content > .indicators > main -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<div class="indicators">
			<!-- ICT11+ Fix: Changed from <main> to <div> - root layout provides <main> -->
			<div class="indicator-content">
				<!-- Indicator Header Component -->
				<IndicatorHeader name={indicator.name} platforms={indicator.platforms} />

				<!-- Training Videos Component -->
				<TrainingVideosSection videos={indicator.trainingVideos} />

				<!-- Platform Downloads Components -->
				{#each indicator.downloads as platformDownload}
					<PlatformDownloads
						platform={platformDownload.platform}
						logo={platformDownload.logo}
						files={platformDownload.files}
						notes={platformDownload.notes}
					/>
				{/each}

				<!-- Support Documentation Component -->
				<SupportDocsSection docs={indicator.supportDocs} />
			</div>
		</div>
	</div>
</div>

<!-- Have Questions Section -->
<HaveQuestionsSection
	email="support@revolutiontradingpros.com"
	phone="8002668659"
	phoneDisplay="(800) 266-8659"
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Indicator Detail Page - 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Page-scoped link colors - touch-friendly */
	.indicators :global(a),
	.indicators :global(a:visited) {
		color: #1e73be;
		/* Touch-friendly link padding */
		padding: 2px 0;
		text-decoration-thickness: 1px;
		text-underline-offset: 2px;
	}

	.indicators :global(a:hover),
	.indicators :global(a:focus),
	.indicators :global(a:active) {
		color: #000000;
	}

	/* Focus state for accessibility */
	.indicators :global(a:focus-visible) {
		outline: 2px solid #1e73be;
		outline-offset: 2px;
		border-radius: 2px;
	}

	/* Mobile-first base styles */
	.indicators {
		background-color: #fff;
		/* Responsive font size */
		font-size: clamp(16px, 4vw, 24px);
		/* Mobile-first padding with safe areas */
		padding: 20px max(12px, env(safe-area-inset-left));
		padding-bottom: max(20px, env(safe-area-inset-bottom));
		max-width: 1200px;
		margin: auto;
		/* Better text rendering */
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	.indicators .indicator-content {
		max-width: 100%;
		margin: 0;
		padding: 0;
	}

	/* Site content wrapper */
	#content.site-content {
		min-height: 50vh;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Breakpoints - Mobile First (min-width)
	 * xs: 360px, sm: 640px, md: 768px, lg: 1024px, xl: 1280px
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.indicators {
			padding: 24px max(16px, env(safe-area-inset-left));
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.indicators {
			padding: 32px 20px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.indicators {
			padding: 50px 15px;
			font-size: 24px;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.indicators {
			padding: 50px 30px;
		}
	}

	/* xl: Extra large devices (≥ 1280px) */
	@media (min-width: 1280px) {
		.indicators {
			padding: 50px 40px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.indicators :global(a) {
			/* Larger touch target for links */
			padding: 4px 2px;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.indicators,
		.indicators :global(*) {
			transition: none !important;
			animation: none !important;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.indicators :global(a) {
			text-decoration: underline;
			text-decoration-thickness: 2px;
		}
	}

	/* Print styles */
	@media print {
		.indicators {
			padding: 0;
			font-size: 12pt;
		}
	}
</style>
