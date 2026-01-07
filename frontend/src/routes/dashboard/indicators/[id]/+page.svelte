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
		const id = page.params.id;
		return {
			id: id,
		name: 'Volume Max Tool Kit (formerly VWAP)',
		description: 'Professional volume analysis tools for advanced trading',
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
				logo: '/wp-content/themes/simpler-trading/assets/images/thinkorswim.png',
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
				logo: '/wp-content/themes/simpler-trading/assets/images/tradingview.png',
				files: [],
				notes: 'Please email your TradingView Username to support@simplertrading.com. The TradingView chart indicator is very easy to get set up in your online charting profile. Once you are logged into TradingView, locate your Notifications area. Once we receive your Username, you should have a notification letting you know the new chart study has been made available.'
			}
		],
		supportDocs: [
			{ title: 'TOS Installation Guide', url: 'https://intercom.help/simpler-trading/en/articles/3263969', type: 'view' },
			{ title: 'Troubleshooting within TOS', url: 'https://intercom.help/simpler-trading/en/articles/3481530-tos-indicator-will-not-import', type: 'view' },
			{ title: 'TradingView - Installing an Indicator', url: 'https://intercom.help/simpler-trading/en/articles/3498380', type: 'view' },
			{ title: 'TradingView - What is my Username', url: 'https://intercom.help/simpler-trading/en/articles/3606861', type: 'view' },
			{ title: 'What is Volume Max Tool Kit (formerly VWAP)?', url: 'https://intercom.help/simpler-trading/en/articles/3160130', type: 'view' }
		]
	} as Indicator;
	});
</script>

<svelte:head>
	<title>{indicator.name} - Simpler Trading</title>
	<meta name="description" content="Download and install {indicator.name} for your trading platform" />
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
								width="250"
								src={platformDownload.logo}
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
											<a
												class="orng_btn"
												href={doc.url}
												target="_blank"
												rel="noopener noreferrer"
											>
												<span class="fa fa-{doc.type === 'view' ? 'external-link' : 'download'}"></span>
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
	 * Indicator Detail Page Styles
	 * Matches WordPress implementation exactly
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicators {
		max-width: 1200px;
		margin: 0 auto;
		padding: 20px;
	}

	.indicator-detail {
		background: #fff;
		border-radius: 8px;
		padding: 40px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Header
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicator-detail__header h1 {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Platforms Section
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicator-detail__platforms {
		margin-bottom: 30px;
	}

	.platforms {
		font-size: 16px;
		color: #666;
		margin: 0 0 15px;
	}

	.platforms strong {
		color: #333;
		font-weight: 600;
	}

	hr {
		border: none;
		border-top: 1px solid #e5e5e5;
		margin: 20px 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Training Videos
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicator-detail__videos {
		margin-bottom: 40px;
	}

	.indicator-detail__videos h2 {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 30px;
		margin-bottom: 30px;
	}

	.video-container {
		background: #000;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-title {
		padding: 15px;
		background: #f9f9f9;
		color: #333;
		font-size: 14px;
		font-weight: 600;
		margin: 0;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Download Sections (st_box)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.st_box {
		background: #f9f9f9;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 30px;
		margin-bottom: 30px;
	}

	.st_box h2 {
		font-size: 22px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.platform-header {
		text-align: center;
		margin-bottom: 25px;
	}

	.platform-header img {
		max-width: 250px;
		height: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Downloads Table
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.downloads-table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 6px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.downloads-table thead th {
		background: linear-gradient(135deg, #143E59 0%, #0984ae 100%);
		color: #fff;
		padding: 15px 20px;
		text-align: left;
		font-weight: 600;
		font-size: 14px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.downloads-table tbody tr {
		border-bottom: 1px solid #e5e5e5;
		transition: background-color 0.2s ease;
	}

	.downloads-table tbody tr:last-child {
		border-bottom: none;
	}

	.downloads-table tbody tr:hover {
		background-color: #f5f5f5;
	}

	.downloads-table td {
		padding: 15px 20px;
		color: #333;
		font-size: 14px;
	}

	.downloads-table td.text-right {
		text-align: right;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Orange Button (orng_btn)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.orng_btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #F69532;
		color: #fff;
		text-decoration: none;
		border-radius: 6px;
		font-weight: 600;
		font-size: 14px;
		transition: all 0.2s ease;
		border: none;
		cursor: pointer;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.orng_btn:hover {
		background: #dc7309;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(246, 149, 50, 0.3);
	}

	.orng_btn .fa {
		font-size: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Platform Notes
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.platform_notes {
		background: #fff;
		border-left: 4px solid #F69532;
		padding: 20px;
		margin-top: 20px;
		border-radius: 4px;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
	}

	.platform_notes a {
		color: #143E59;
		text-decoration: underline;
	}

	.platform_notes a:hover {
		color: #F69532;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Footer
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.indicator-detail__footer {
		margin-top: 40px;
		padding-top: 30px;
		border-top: 1px solid #e5e5e5;
		text-align: center;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 16px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 6px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.btn-secondary {
		background: #fff;
		color: #143E59;
		border: 2px solid #143E59;
	}

	.btn-secondary:hover {
		background: #143E59;
		color: #fff;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.indicator-detail {
			padding: 20px;
		}

		.indicator-detail__header h1 {
			font-size: 24px;
		}

		.videos-grid {
			grid-template-columns: 1fr;
			gap: 20px;
		}

		.st_box {
			padding: 20px;
		}

		.downloads-table {
			font-size: 13px;
		}

		.downloads-table td {
			padding: 12px 15px;
		}

		.downloads-table thead th {
			padding: 12px 15px;
			font-size: 12px;
		}

		.orng_btn {
			padding: 8px 16px;
			font-size: 13px;
		}

		.platform-header img {
			max-width: 180px;
		}
	}

	@media (max-width: 480px) {
		.indicators {
			padding: 10px;
		}

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
			border-radius: 6px;
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
			justify-content: center;
		}
	}
</style>
