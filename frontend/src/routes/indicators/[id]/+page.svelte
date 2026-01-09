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

<!-- Main Content - Matches WordPress exactly -->
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
								width="200" 
								src={platformDownload.logo}
								srcset={platformDownload.platform === 'TradingView' ? '/logos/platforms/tradingview@2x.png 2x' : ''}
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

<!-- Have Questions Section - WordPress Footer -->
<div class="fl-builder-content">
	<div class="fl-row fl-row-full-width fl-row-bg-color" style="background-color: #f7f7f7;">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width" style="max-width: 800px; margin: 0 auto; padding: 60px 20px;">
				<div class="fl-col-group">
					<div class="fl-col">
						<div class="fl-col-content">
							<div class="fl-module fl-module-rich-text">
								<div class="fl-module-content">
									<div class="fl-rich-text">
										<h2 style="text-align: center; font-size: 42px; margin: 0 0 20px;"><strong>Have Questions?</strong></h2>
									</div>
								</div>
							</div>
							<div class="fl-module fl-module-rich-text">
								<div class="fl-module-content">
									<div class="fl-rich-text">
										<p style="text-align: center; font-size: 22px; margin: 0;">
											Our support staff is the best by far! You can email 
											<a href="mailto:support@simplertrading.com" style="color: #0984ae;">support@simplertrading.com</a> 
											or call us at 
											<a href="tel:5122668659" style="color: #0984ae;">(512) 266-8659</a>
										</p>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * WordPress Exact Match Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */

	:global(body) {
		background-color: #efefef !important;
	}
	
	:global(html),
	:global(body),
	:global(main) {
		background-color: #efefef !important;
	}

	.indicators {
		max-width: 1160px;
		margin: 0 auto;
		padding: 40px 20px;
		background-color: #ffffff;
	}

	.indicators main {
		max-width: 100%;
		margin: 0;
		padding: 0;
	}

	.indicators h1 {
		font-size: 32px;
		font-weight: 400;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
		text-align: center;
	}

	.platforms {
		font-size: 16px;
		color: #666666;
		margin: 0 0 15px;
		font-family: 'Open Sans', sans-serif;
		text-align: center;
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

	/* Video Section */
	.ca-section {
		margin-bottom: 40px;
	}

	.section-inner {
		width: 100%;
	}

	.current-vid {
		padding: 25px;
	}

	.video-container {
		position: relative;
		background: #000;
		border-radius: 0;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-overlay {
		display: none;
	}

	/* Platform Download Boxes */
	.st_box {
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		padding: 40px 30px;
		margin-bottom: 20px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.st_box img {
		display: block;
		margin: 0 auto 30px;
		max-width: 200px;
		height: auto;
	}

	.st_box h2 {
		font-size: 22px;
		font-weight: 700;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Tables */
	.st_box table {
		width: 100%;
		border-collapse: collapse;
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 0;
	}

	.st_box table th {
		background: #f5f5f5;
		color: #666666;
		padding: 12px 15px;
		text-align: left;
		font-weight: 700;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		border-bottom: 1px solid #e5e5e5;
	}

	.st_box table tr {
		border-bottom: 1px solid #e5e5e5;
	}

	.st_box table tr:last-child {
		border-bottom: none;
	}

	.st_box table tr:hover {
		background-color: #f5f5f5;
	}

	.st_box table td {
		padding: 15px;
		color: #666666;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	.st_box table td.text-right {
		text-align: right;
	}

	/* Orange Button */
	.orng_btn {
		display: inline-block;
		padding: 8px 20px;
		background: #F69532;
		color: #ffffff;
		text-decoration: none;
		border-radius: 20px;
		font-weight: 600;
		font-size: 13px;
		transition: all 0.2s ease;
		border: none;
		cursor: pointer;
		white-space: nowrap;
	}

	.orng_btn:hover {
		background: #dc7309;
		text-decoration: none;
	}

	/* Platform Notes */
	.platform_notes {
		background: #f9f9f9;
		border: 1px solid #e5e5e5;
		padding: 20px;
		margin-top: 20px;
		border-radius: 4px;
		color: #666666;
		font-size: 14px;
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

	/* Have Questions Section */
	.fl-builder-content {
		width: 100%;
		margin-top: 60px;
		background-color: #f7f7f7;
	}

	.fl-row-bg-color {
		background-color: #f7f7f7;
		padding: 60px 20px;
	}

	.fl-rich-text {
		font-family: 'Open Sans', sans-serif;
	}

	.fl-rich-text a {
		color: #1e73be;
		text-decoration: none;
	}

	.fl-rich-text a:hover {
		color: #000000;
		text-decoration: underline;
	}

	/* Responsive */
	@media (max-width: 992px) {
		.fl-row-content {
			padding: 40px 20px !important;
		}
		
		.fl-rich-text h2 {
			font-size: 32px !important;
		}
		
		.fl-rich-text p {
			font-size: 18px !important;
		}
	}

	@media (max-width: 768px) {
		.indicators {
			padding: 30px 20px;
		}

		.indicators h1 {
			font-size: 24px;
		}

		.st_box {
			padding: 20px;
		}

		.st_box img {
			max-width: 180px;
		}

		.fl-row-content {
			padding: 30px 20px !important;
		}
		
		.fl-rich-text h2 {
			font-size: 24px !important;
		}
		
		.fl-rich-text p {
			font-size: 16px !important;
		}

		.st_box table td,
		.st_box table th {
			padding: 10px;
			font-size: 13px;
		}

		.orng_btn {
			padding: 8px 16px;
			font-size: 13px;
		}
	}

	@media (max-width: 480px) {
		.st_box table {
			font-size: 12px;
		}

		.st_box table tr:first-child {
			display: none;
		}

		.st_box table tr {
			display: block;
			margin-bottom: 15px;
			border: 1px solid #e5e5e5;
			border-radius: 4px;
			padding: 10px;
		}

		.st_box table td {
			display: block;
			text-align: left !important;
			padding: 8px 0;
			border-bottom: none;
		}

		.st_box table td.text-right {
			margin-top: 10px;
		}

		.orng_btn {
			width: 100%;
			text-align: center;
		}
	}
</style>
