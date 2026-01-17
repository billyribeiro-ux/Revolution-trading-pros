<!--
	URL: /dashboard/spx-profit-pulse/premium-videos/[slug]

	SPX Profit Pulse Video Detail Page - 100% WordPress Pixel-Perfect Match
	═══════════════════════════════════════════════════════════════════════════
	Reference: frontend/Do's/SPX Tr3ndy/latest-updates-video-clicked
	
	Displays individual video with HTML5 player, navigation, and metadata.
	
	@version 1.0.0 - January 2026
	@author Revolution Trading Pros
-->
<script lang="ts">
	import type { PageData } from './$types';
	import type { VideoDetail } from './+page.server';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
	let video = $derived(data.video as VideoDetail);
</script>

<svelte:head>
	<title>{video.title} | SPX Profit Pulse | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Watch {video.title} with {video.trader} from SPX Profit Pulse"
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="SPX Profit Pulse"
	startHereUrl="/dashboard/spx-profit-pulse/start-here"
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Breadcrumbs -->
		<nav class="breadcrumbs">
			<ul>
				<li><a href="/dashboard">Home</a></li>
				<li class="separator">/</li>
				<li><a href="/dashboard/spx-profit-pulse">SPX Profit Pulse</a></li>
				<li class="separator">/</li>
				<li class="current">{video.title}</li>
			</ul>
		</nav>

		<!-- Video Title Section -->
		<section class="cpost-title-section">
			<div class="section-inner">
				{#if video.previousVideo}
					<div class="cpost-previous">
						<a
							href="/dashboard/spx-profit-pulse/premium-videos/{video.previousVideo.slug}"
							title={video.previousVideo.title}
						>
							<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
						</a>
					</div>
				{/if}

				<h1 class="cpost-title">{video.title}</h1>

				{#if video.nextVideo}
					<div class="cpost-next">
						<a
							href="/dashboard/spx-profit-pulse/premium-videos/{video.nextVideo.slug}"
							title={video.nextVideo.title}
						>
							<span>Next </span><i class="fa fa-chevron-circle-right"></i>
						</a>
					</div>
				{/if}

				{#if video.chatLogUrl}
					<h2 class="cpost-subtitle">
						<a href={video.chatLogUrl} target="_blank" rel="nofollow">View Chat Log</a>
					</h2>
				{/if}
			</div>
		</section>

		<!-- Video Player Section -->
		<section class="cpost-section">
			<div class="section-inner">
				<div class="ca-content-block">
					<div class="current-vid">
						<h2 class="current-title">{video.title}</h2>
						<p class="current-trader">{video.trader}</p>
						<div class="video-container current">
							<div class="video-overlay">
								<h3 class="d-none">{video.title}</h3>
								<h4>{video.trader}</h4>
								<p class="d-none">{video.trader}</p>
							</div>
							<div class="video-player">
								<video
									controls
									width="100%"
									poster={video.traderImage}
									style="aspect-ratio: 16/9;"
									title={video.title}
								>
									<source src={video.videoUrl} type="video/mp4" />
									Your browser does not support the video tag.
								</video>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SPX PROFIT PULSE VIDEO DETAIL - 100% WordPress Pixel-Perfect Match
	   Reference: frontend/Do's/SPX Tr3ndy/latest-updates-video-clicked
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: block;
		width: 100%;
	}

	.dashboard__content-main {
		width: 100%;
		max-width: 100%;
	}

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f5f5f5;
		padding: 12px 20px;
		border-bottom: 1px solid #e0e0e0;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 8px;
		font-size: 13px;
	}

	.breadcrumbs li {
		display: inline;
	}

	.breadcrumbs a {
		color: #143e59;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.breadcrumbs a:hover {
		color: #0c2638;
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		color: #999;
	}

	.breadcrumbs .current {
		color: #666;
		font-weight: 600;
	}

	/* Title Section */
	.cpost-title-section {
		background: #fff;
		padding: 30px 20px;
		border-bottom: 1px solid #e0e0e0;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		position: relative;
	}

	.cpost-title {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0;
		text-align: center;
		font-family: 'Open Sans', sans-serif;
	}

	@media screen and (min-width: 768px) {
		.cpost-title {
			font-size: 2.5rem;
		}
	}

	.cpost-subtitle {
		text-align: center;
		margin-top: 15px;
		font-size: 1rem;
		font-weight: 400;
	}

	.cpost-subtitle a {
		color: #143e59;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.cpost-subtitle a:hover {
		color: #0c2638;
		text-decoration: underline;
	}

	/* Navigation Arrows */
	.cpost-previous,
	.cpost-next {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
	}

	.cpost-previous {
		left: 0;
	}

	.cpost-next {
		right: 0;
	}

	.cpost-previous a,
	.cpost-next a {
		display: flex;
		align-items: center;
		gap: 8px;
		color: #143e59;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s ease;
		padding: 8px 12px;
		border-radius: 4px;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #0c2638;
		background: #f5f5f5;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 20px;
	}

	@media screen and (max-width: 768px) {
		.cpost-previous,
		.cpost-next {
			position: static;
			transform: none;
			margin-bottom: 15px;
		}

		.section-inner {
			display: flex;
			flex-direction: column;
			align-items: center;
		}
	}

	/* Video Section */
	.cpost-section {
		background: #fff;
		padding: 40px 20px;
	}

	.ca-content-block {
		max-width: 1200px;
		margin: 0 auto;
	}

	.current-vid {
		margin-bottom: 40px;
	}

	.current-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px 0;
		font-family: 'Open Sans', sans-serif;
	}

	.current-trader {
		font-size: 1rem;
		color: #666;
		margin: 0 0 20px 0;
		font-style: italic;
	}

	/* Video Container */
	.video-container {
		position: relative;
		width: 100%;
		max-width: 100%;
		background: #000;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.video-container.current {
		aspect-ratio: 16/9;
	}

	.video-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0.3) 0%, rgba(0, 0, 0, 0.7) 100%);
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		color: #fff;
		opacity: 0;
		transition: opacity 0.3s ease;
		pointer-events: none;
		z-index: 1;
	}

	.video-container:hover .video-overlay {
		opacity: 1;
	}

	.video-overlay h4 {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 0;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
	}

	.d-none {
		display: none;
	}

	/* Video Player */
	.video-player {
		position: relative;
		width: 100%;
		height: 100%;
	}

	.video-player video {
		width: 100%;
		height: 100%;
		display: block;
		background: #000;
	}

	/* Font Awesome Icons */
	.fa {
		display: inline-block;
		font: normal normal normal 14px/1 FontAwesome;
		font-size: inherit;
		text-rendering: auto;
		-webkit-font-smoothing: antialiased;
	}

	.fa-chevron-circle-left:before {
		content: '\f137';
	}

	.fa-chevron-circle-right:before {
		content: '\f138';
	}

	/* CRITICAL: Hide sidebar on SPX Profit Pulse pages */
	:global(.dashboard__content-sidebar) {
		display: none !important;
	}

	/* Ensure main content takes full width without sidebar */
	.dashboard__content {
		display: block !important;
		width: 100% !important;
	}

	.dashboard__content-main {
		width: 100% !important;
		max-width: 100% !important;
		flex: none !important;
	}
</style>
