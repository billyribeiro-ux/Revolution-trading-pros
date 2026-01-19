<!--
	Indicator Detail & Download Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyIndicatorDownload HTML file
	
	Features:
	- Indicator name and details
	- Available platforms display
	- Training videos
	- Platform-specific download sections
	- Supporting documentation links
	- Breadcrumb navigation
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import { page } from '$app/stores';

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
		type: 'view' | 'download';
	}

	interface Indicator {
		id: string;
		name: string;
		description: string;
		platforms: string[];
		trainingVideos: TrainingVideo[];
		downloads: PlatformDownloads[];
		supportDocs: SupportDoc[];
	}

	// Get indicator ID from URL - access directly in derived context
	let indicator = $derived.by(() => {
		const id = $page.params.id;
		return {
			id: id,
			name: 'Volume Max Tool Kit (formerly VWAP)',
			description: 'Professional volume analysis tools for advanced trading',
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
						'Please email your TradingView Username to support@simplertrading.com. The TradingView chart indicator is very easy to get set up in your online charting profile. Once you are logged into TradingView, locate your Notifications area. Once we receive your Username, you should have a notification letting you know the new chart study has been made available.'
				}
			],
			supportDocs: [
				{
					title: 'TOS Installation Guide',
					url: 'https://intercom.help/simpler-trading/en/articles/3263969',
					type: 'view'
				},
				{
					title: 'Troubleshooting within TOS',
					url: 'https://intercom.help/simpler-trading/en/articles/3481530-tos-indicator-will-not-import',
					type: 'view'
				},
				{
					title: 'TradingView - Installing an Indicator',
					url: 'https://intercom.help/simpler-trading/en/articles/3498380',
					type: 'view'
				},
				{
					title: 'TradingView - What is my Username',
					url: 'https://intercom.help/simpler-trading/en/articles/3606861',
					type: 'view'
				},
				{
					title: 'What is Volume Max Tool Kit (formerly VWAP)?',
					url: 'https://intercom.help/simpler-trading/en/articles/3160130',
					type: 'view'
				}
			]
		} as Indicator;
	});
</script>

<svelte:head>
	<title>{indicator.name} - Simpler Trading</title>
	<meta
		name="description"
		content="Download and install {indicator.name} for your trading platform"
	/>
	<meta property="og:title" content={indicator.name} />
	<meta property="og:type" content="article" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<div class="indicators">
			<main class="indicator-detail">
				<!-- Page Header -->
				<header class="indicator-detail__header">
					<h1>{indicator.name}</h1>
				</header>

				<!-- Available Platforms -->
				<section class="indicator-detail__platforms">
					<p class="platforms">
						<strong>Available Platforms:</strong>
						{indicator.platforms.join(', ')}
					</p>
					<hr />
				</section>

				<!-- Training Videos -->
				{#if indicator.trainingVideos.length > 0}
					<section class="indicator-detail__videos">
						<h2>Training Videos</h2>
						<div class="videos-grid">
							{#each indicator.trainingVideos as video (video.id)}
								<div class="video-container">
									<video
										id={video.id}
										controls
										width="100%"
										poster={video.posterUrl}
										style="aspect-ratio: 16/9;"
										title={video.title}
									>
										<source src={video.videoUrl} type="video/mp4" />
										Your browser does not support the video tag.
									</video>
									<p class="video-title">{video.title}</p>
								</div>
							{/each}
						</div>
					</section>
				{/if}

				<!-- Download Sections by Platform -->
				{#each indicator.downloads as platformDownload}
					<section class="st_box {platformDownload.platform.toLowerCase().replace(/\s+/g, '')}">
						<div class="platform-header">
							<img
								width="200"
								src={platformDownload.logo}
								srcset={platformDownload.platform === 'TradingView'
									? '/logos/platforms/tradingview@2x.png 2x'
									: ''}
								alt={platformDownload.platform}
								onerror={(e) => {
									const img = e.currentTarget as HTMLImageElement;
									img.style.display = 'none';
								}}
							/>
						</div>

						{#if platformDownload.files.length > 0}
							<table class="downloads-table">
								<thead>
									<tr>
										<th>{platformDownload.platform} Install File:</th>
										<th></th>
									</tr>
								</thead>
								<tbody>
									{#each platformDownload.files as file}
										<tr>
											<td>{file.name}</td>
											<td class="text-right">
												<a class="orng_btn" href={file.downloadUrl}>
													<span class="fa fa-download"></span>
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
					</section>
				{/each}

				<!-- Supporting Documentation -->
				{#if indicator.supportDocs.length > 0}
					<section class="st_box">
						<h2>
							<strong>Supporting Documentation</strong>
						</h2>
						<table class="downloads-table">
							<tbody>
								{#each indicator.supportDocs as doc}
									<tr>
										<td>{doc.title}</td>
										<td class="text-right">
											<a class="orng_btn" href={doc.url} target="_blank" rel="noopener noreferrer">
												<span class="fa fa-{doc.type === 'view' ? 'external-link' : 'download'}"
												></span>
												Click to {doc.type === 'view' ? 'View' : 'Download'}
											</a>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</section>
				{/if}

				<!-- Back to Indicators -->
				<div class="indicator-detail__footer">
					<a href="/dashboard/indicators" class="btn btn-secondary">
						<span class="fa fa-arrow-left"></span>
						Back to My Indicators
					</a>
				</div>
			</main>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * WordPress Exact Match Styles - NO CUSTOM STYLING
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicators {
		max-width: 1160px;
		margin: 0 auto;
		padding: 40px 20px;
		background-color: #ffffff;
	}

	.indicator-detail {
		max-width: 100%;
		margin: 0;
		padding: 0;
		background: transparent;
		border-radius: 0;
		box-shadow: none;
	}

	.indicator-detail__header h1 {
		font-size: 32px;
		font-weight: 400;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
		text-align: center;
	}

	.indicator-detail__platforms {
		margin-bottom: 0;
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
	.indicator-detail__videos {
		margin-bottom: 40px;
	}

	.indicator-detail__videos h2 {
		display: none;
	}

	.videos-grid {
		display: block;
	}

	.video-container {
		position: relative;
		background: #000;
		border-radius: 0;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		margin-bottom: 25px;
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-title {
		display: none;
	}

	/* Platform Download Boxes - WordPress Exact */
	.st_box {
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		padding: 40px 30px;
		margin-bottom: 20px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.st_box h2 {
		font-size: 22px;
		font-weight: 700;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.platform-header {
		text-align: center;
		margin-bottom: 30px;
	}

	.platform-header img {
		max-width: 200px;
		height: auto;
	}

	/* Downloads Table - WordPress Exact */
	.downloads-table {
		width: 100%;
		border-collapse: collapse;
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 0;
		overflow: visible;
		box-shadow: none;
	}

	.downloads-table thead th {
		background: #f5f5f5;
		color: #666666;
		padding: 15px 20px;
		text-align: left;
		font-weight: 700;
		font-size: 14px;
		text-transform: none;
		letter-spacing: 0;
		border-bottom: 1px solid #e5e5e5;
		font-family: 'Open Sans', sans-serif;
	}

	.downloads-table tbody tr {
		border-bottom: 1px solid #e5e5e5;
	}

	.downloads-table tbody tr:last-child {
		border-bottom: none;
	}

	.downloads-table tbody tr:hover {
		background-color: #fafafa;
	}

	.downloads-table td {
		padding: 15px 20px;
		color: #666666;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	.downloads-table td.text-right {
		text-align: right;
	}

	/* Orange Button - WordPress Exact */
	.orng_btn {
		display: inline-block;
		padding: 8px 20px;
		background: #f69532;
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

	.orng_btn .fa {
		display: none;
	}

	/* Platform Notes - WordPress Exact */
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

	.platform_notes a {
		color: #1e73be;
		text-decoration: none;
	}

	.platform_notes a:hover {
		color: #000000;
		text-decoration: underline;
	}

	/* Footer */
	.indicator-detail__footer {
		margin-top: 40px;
		padding-top: 30px;
		border-top: 1px solid #dddddd;
		text-align: center;
	}

	.btn {
		display: inline-block;
		padding: 10px 24px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		font-family: 'Open Sans', sans-serif;
	}

	.btn-secondary {
		background: #f5f5f5;
		color: #666666;
		border: 1px solid #dddddd;
	}

	.btn-secondary:hover {
		background: #e5e5e5;
		color: #333333;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.indicators {
			padding: 20px 15px;
		}

		.indicator-detail__header h1 {
			font-size: 24px;
		}

		.st_box {
			padding: 20px;
		}

		.downloads-table td,
		.downloads-table thead th {
			padding: 12px 15px;
		}

		.platform-header img {
			max-width: 150px;
		}
	}

	@media (max-width: 480px) {
		.downloads-table td,
		.downloads-table thead th {
			display: block;
			width: 100%;
			text-align: left !important;
		}

		.downloads-table thead {
			display: none;
		}

		.downloads-table tbody tr {
			display: block;
			margin-bottom: 15px;
			border: 1px solid #e5e5e5;
			padding: 10px;
		}

		.downloads-table td {
			padding: 8px 0;
			border-bottom: none;
		}

		.downloads-table td.text-right {
			text-align: left !important;
			margin-top: 10px;
		}

		.orng_btn {
			width: 100%;
			text-align: center;
		}
	}
</style>
