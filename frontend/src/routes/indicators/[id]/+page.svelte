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
		// Mock data - replace with API call
		return {
			id: id,
			name: 'Volume Max Tool Kit (formerly VWAP)',
			platforms: ['ThinkorSwim', 'TradingView'],
			trainingVideos: [
				{
					id: '1',
					title: 'Indicator Setup and Q&A, with Eric Purdy',
					videoUrl: 'https://cloud-streaming.s3.amazonaws.com/classes/SubmarketSonar-IndicatorSetup-EP.mp4',
					posterUrl: 'https://cdn.simplertrading.com/2020/03/25163022/simpler-geenric-video-bg-768x432.jpg'
				},
				{
					id: '2',
					title: 'VWAP Training and Live Trading Recorded Session with Raghee Horner',
					videoUrl: 'https://s3.amazonaws.com/cloud-streaming/chatrecordings%2FSTWebinars/1176-RH-06-12-2019__09.01.394_AM.mp4',
					posterUrl: 'https://cdn.simplertrading.com/2020/03/25163022/simpler-geenric-video-bg-768x432.jpg'
				}
			],
			downloads: [
				{
					platform: 'ThinkorSwim',
					logo: '/logos/platforms/thinkorswim.png',
					files: [
						{ name: 'Volume Max Indicator', downloadUrl: '/?st-download-file=452914b18dc78691e6c98731b9e094fe' },
						{ name: 'VScore EOD Study', downloadUrl: '/?st-download-file=12ab53eb7e85a005a6a21f800db57777' },
						{ name: 'VScore Intraday Signals Study', downloadUrl: '/?st-download-file=4bd68f7bbfec21d14c099258f6833a9c' },
						{ name: 'VScore Bands EOD Study', downloadUrl: '/?st-download-file=4fb2197916c551be4acb037afccc607f' },
						{ name: 'VScore Bands Intraday Study', downloadUrl: '/?st-download-file=560ebea65a1229c04c0c268107fdb693' },
						{ name: 'VProfile EOD Study', downloadUrl: '/?st-download-file=da8f1193496d5cf4bed2cdd049a2d70b' },
						{ name: 'VProfile Intraday Study', downloadUrl: '/?st-download-file=ab0357f17694826819f632d50f6cfccc' },
						{ name: 'Volume Labels', downloadUrl: '/?st-download-file=7aa9537ae79f082d9b8ff2ede9c2561b' }
					]
				},
				{
					platform: 'TradingView',
					logo: '/logos/platforms/tradingview.png',
					files: [],
					notes: 'Please email your TradingView Username to <a href="mailto:support@simplertrading.com">support@simplertrading.com</a>. The TradingView chart indicator is very easy to get set up in your online charting profile. Once you are logged into TradingView, locate your Notifications area. Once we receive your Username, you should have a notification letting you know the new chart study has been made available. Clicking on the notification will bring up a chart for the study to be displayed.'
				}
			],
			supportDocs: [
				{ title: 'TOS Installation Guide', url: 'https://intercom.help/simpler-trading/en/articles/3263969' },
				{ title: 'Troubleshooting within TOS', url: 'https://intercom.help/simpler-trading/en/articles/3481530-tos-indicator-will-not-import' },
				{ title: 'TradingView - Installing an Indicator', url: 'https://intercom.help/simpler-trading/en/articles/3498380' },
				{ title: 'TradingView - What is my Username', url: 'https://intercom.help/simpler-trading/en/articles/3606861' },
				{ title: 'What is Volume Max Tool Kit (formerly VWAP)?', url: 'https://intercom.help/simpler-trading/en/articles/3160130' },
				{ title: 'What are Volume Max Tool Kit (formerly VWAP) Settings?', url: 'https://intercom.help/simpler-trading/en/articles/3311644' },
				{ title: 'What is VScore?', url: 'https://intercom.help/simpler-trading/en/articles/3210561' },
				{ title: 'VScore Settings and Configuration', url: 'https://intercom.help/simpler-trading/en/articles/3210408' },
				{ title: 'What is VProfile?', url: 'https://intercom.help/simpler-trading/en/articles/3341484' },
				{ title: 'What are VProfile Settings?', url: 'https://intercom.help/simpler-trading/en/articles/3341233' }
			]
		} as Indicator;
	});
</script>

<svelte:head>
	<title>{indicator.name} - Simpler Trading</title>
	<meta name="description" content="Download and install {indicator.name} for your trading platform" />
</svelte:head>

<!-- Breadcrumbs - Auto-generated from path -->
<DashboardBreadcrumbs />

<!-- Main Content - Matches WordPress exactly -->
<!-- WordPress structure: #page > #content.site-content > .indicators > main -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<div class="indicators">
			<main>
				<!-- Page Title -->
				<h1>{indicator.name}</h1>
				
				<!-- Platforms Section -->
				<section>
					<p class="platforms">
						<strong>Available Platforms:</strong>
						{indicator.platforms.join(', ')}
					</p>
					<hr>
					
					<!-- Training Videos Section - WordPress Structure -->
					<section id="ca-main" class="ca-section cpost-section">
						<div class="section-inner">
							<div class="ca-content-block cpost-content-block">
								{#each indicator.trainingVideos as video (video.id)}
									<div class="current-vid">
										<div class="video-container current">
											<div class="video-overlay"></div>
											<div id="{video.id}" class="video-player"></div>
											<video 
												id="{video.id}" 
												controls 
												width="100%" 
												poster="{video.posterUrl}"
												style="aspect-ratio: 16/9;" 
												title="{video.title}"
											>
												<source src="{video.videoUrl}" type="video/mp4">
												Your browser does not support the video tag.
											</video>
										</div>
									</div>
								{/each}
							</div>
						</div>
					</section>
					
					<!-- ThinkorSwim Downloads -->
					{#each indicator.downloads as platformDownload}
						<div class="st_box {platformDownload.platform.toLowerCase()}">
							<img 
								width="250" 
								src={platformDownload.logo}
								alt={platformDownload.platform}
							>
							
							{#if platformDownload.files.length > 0}
								<table>
									<tbody>
										<tr>
											<th>{platformDownload.platform} Install File:</th>
											<th></th>
										</tr>
										{#each platformDownload.files as file}
											<tr>
												<td>{file.name}</td>
												<td class="text-right">
													<a class="orng_btn" href={file.downloadUrl}>
														Click to Download
													</a>
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							{/if}
							
							{#if platformDownload.notes}
								<div class="platform_notes">
									{@html platformDownload.notes}
								</div>
							{/if}
						</div>
					{/each}
					
					<!-- Supporting Documentation -->
					<div class="st_box">
						<h2>
							<strong>Supporting Documentation</strong>
						</h2>
						<table>
							<tbody>
								{#each indicator.supportDocs as doc}
									<tr>
										<td>{doc.title}</td>
										<td class="text-right">
											<a class="orng_btn" href={doc.url} target="_blank" rel="noopener noreferrer">
												Click to View
											</a>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</section>
			</main>
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
	 * WORDPRESS EXACT MATCH - from INDICATOR_PAGE_REFERENCE.md
	 * Source: https://my.simplertrading.com/indicators/volume-max-i
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* 1. BODY BACKGROUND - CRITICAL */
	:global(html),
	:global(body) {
		background-color: #efefef !important;
		color: #666666;
		font-family: "Open Sans", sans-serif;
	}

	:global(a), :global(a:visited) {
		color: #1e73be;
	}

	:global(a:hover), :global(a:focus), :global(a:active) {
		color: #000000;
	}

	/* 2. PAGE WRAPPER - WordPress structure */
	#page {
		padding: 0;
		max-width: 100%;
	}

	#content.site-content {
		width: 100%;
	}

	/* 3. INDICATORS CONTAINER - Mobile-first */
	.indicators {
		background-color: #fff;
		font-size: 24px;
		padding: 30px 15px;
		max-width: 1200px;
		margin: auto;
	}

	.indicators main {
		max-width: 100%;
		margin: 0;
		padding: 0;
	}

	/* 4. H1 TITLE - Mobile-first */
	.indicators h1 {
		color: #0c2434;
		font-weight: 700;
		text-align: center;
		margin-bottom: 10px;
		font-size: 30px;
		font-family: "Open Sans Condensed", sans-serif;
		line-height: 1.1em;
	}

	/* 4. PLATFORMS - Centered */
	.platforms {
		text-align: center;
		margin-bottom: 40px;
		color: #666666;
		font-family: "Open Sans", sans-serif;
	}

	.platforms strong {
		color: #666666;
		font-weight: 700;
	}

	hr {
		border: none;
		border-top: 1px solid #dddddd;
		margin: 20px 0 40px;
	}

	/* 5. VIDEO SECTION - Mobile-first */
	.ca-section {
		padding-bottom: 0;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0;
	}

	.ca-content-block {
		margin: 0;
		padding: 0;
	}

	.current-vid {
		width: 100%;
		background-color: #f4f4f4;
		padding: 10px 10px 0;
	}

	.video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid #999;
		cursor: pointer;
	}

	.video-container.current {
		width: 100%;
		display: flex;
		z-index: 1;
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-overlay {
		background-color: rgba(0, 0, 0, 0.269);
	}

	/* 6. ST_BOX - 24px solid #f4f4f4 border */
	.st_box {
		border: 24px solid #f4f4f4;
		padding: 20px;
		margin-top: 30px;
		background: #ffffff;
	}

	.st_box img {
		display: block;
		margin: 0 auto 20px;
		height: auto;
	}

	.st_box h2 {
		margin-bottom: 20px;
		font-weight: 700;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
	}

	/* 7. TABLES - No border, specific styling */
	.st_box table {
		width: 100%;
		margin: 0;
		border: 0;
		border-collapse: collapse;
	}

	.st_box table th {
		border: 0;
		padding: 12px 15px;
		text-align: left;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #666666;
	}

	.st_box table th:first-child {
		padding-left: 0;
	}

	.st_box table td {
		border: 0;
		padding: 20px 0;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		font-family: 'Open Sans', sans-serif;
	}

	.st_box table td.text-right {
		text-align: right;
	}

	/* 8. ORANGE BUTTON - Mobile-first */
	.orng_btn {
		background-color: #f8ac00;
		border-radius: 80px;
		font-weight: 700;
		color: #fff;
		display: inline-block;
		text-align: center;
		padding: 8px 20px;
		min-width: 150px;
		font-size: 16px;
		text-decoration: none;
		transition: background-color 0.2s ease;
	}

	.orng_btn:hover {
		background-color: #df9c00;
		color: #fff;
		text-decoration: none;
	}

	/* 9. PLATFORM NOTES */
	.platform_notes {
		padding-top: 20px;
		border-top: 1px solid #f4f4f4;
		color: #666666;
		line-height: 1.7;
		font-family: 'Open Sans', sans-serif;
	}

	.platform_notes :global(a) {
		color: #1e73be;
		text-decoration: none;
	}

	.platform_notes :global(a:hover) {
		color: #000000;
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Mobile-First (min-width breakpoints)
	 * Base styles above are mobile, these scale UP
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet (416px+) */
	@media (min-width: 416px) {
		.current-vid {
			padding: 25px 25px 0;
		}

		.orng_btn {
			min-width: 200px;
			font-size: 18px;
			padding: 10px 25px;
		}
	}

	/* Tablet Large (768px+) */
	@media (min-width: 768px) {
		.indicators {
			padding: 50px 15px;
		}

		.indicators h1 {
			font-size: 54px;
		}

		.orng_btn {
			min-width: 250px;
			font-size: 20px;
			padding: 10px 35px;
		}
	}
</style>
