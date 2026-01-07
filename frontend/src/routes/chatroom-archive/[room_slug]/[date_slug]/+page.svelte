<!--
	URL: /chatroom-archive/[room_slug]/[date_slug]
	
	Chatroom Archive Video Detail Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	
	Displays all recorded sessions from a specific date in the trading room.
	Includes prev/next navigation, chat log link, and video players.
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { IconChevronLeft, IconChevronRight, IconFileText, IconArrowLeft } from '$lib/icons';
	import type { ArchiveDetailPageData } from './+page.server';

	// Server data
	interface Props {
		data: ArchiveDetailPageData;
	}

	let { data }: Props = $props();

	// Get trader poster image or fallback
	function getTraderPoster(traderSlug: string | undefined): string {
		if (!traderSlug) return '/images/traders/default-poster.jpg';
		return `https://s3.amazonaws.com/simpler-cdn/azure-blob-files/chatroom/chatroom-${traderSlug}.jpg`;
	}
</script>

<svelte:head>
	<title>{data.displayDate} | {data.roomName} Archives | Revolution Trading Pros</title>
	<meta name="description" content="Watch recorded trading sessions from {data.displayDate} in the {data.roomName}." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Breadcrumb Navigation -->
<nav class="breadcrumbs" aria-label="Breadcrumb">
	<div class="container">
		<ul class="breadcrumb-list">
			<li><a href="/dashboard">Home</a></li>
			<li class="separator">/</li>
			<li><a href={data.archiveUrl}>Chat Archives</a></li>
			<li class="separator">/</li>
			<li class="current">{data.displayDate}</li>
		</ul>
	</div>
</nav>

<!-- Title Section with Navigation -->
<section class="ca-title-section">
	<div class="section-inner">
		<!-- Previous Link -->
		<div class="nav-prev">
			{#if data.previousDate}
				<a href="/chatroom-archive/{data.roomSlug}/{data.previousDate}" class="nav-link">
					<IconChevronLeft size={20} />
					<span>Previous</span>
				</a>
			{/if}
		</div>

		<!-- Title -->
		<h1 class="page-title">{data.displayDate}</h1>

		<!-- Next Link -->
		<div class="nav-next">
			{#if data.nextDate}
				<a href="/chatroom-archive/{data.roomSlug}/{data.nextDate}" class="nav-link">
					<span>Next</span>
					<IconChevronRight size={20} />
				</a>
			{/if}
		</div>
	</div>

	<!-- Chat Log Link -->
	{#if data.chatLogUrl}
		<div class="chat-log-link">
			<a href={data.chatLogUrl} target="_blank" rel="noopener noreferrer">
				<IconFileText size={16} />
				<span>View Chat Log</span>
			</a>
		</div>
	{/if}
</section>

<!-- Video Content Section -->
<section class="ca-main-section">
	<div class="section-inner">
		{#if data.error}
			<div class="error-message">
				<p>{data.error}</p>
				<a href={data.archiveUrl} class="btn btn-default">
					<IconArrowLeft size={16} />
					Back to Archives
				</a>
			</div>
		{:else if data.videos.length === 0}
			<div class="empty-state">
				<h3>No recordings available</h3>
				<p>There are no recorded sessions for this date.</p>
				<a href={data.archiveUrl} class="btn btn-default">
					<IconArrowLeft size={16} />
					Back to Archives
				</a>
			</div>
		{:else}
			<div class="videos-container">
				{#each data.videos as video (video.id)}
					<article class="video-card">
						<header class="video-header">
							<h2 class="video-title">{video.title}</h2>
							{#if video.trader}
								<p class="video-trader">{video.trader.name}</p>
							{/if}
						</header>

						<div class="video-player-container">
							<video 
								controls 
								width="100%" 
								poster={video.thumbnail_url || getTraderPoster(video.trader?.slug)}
								title={video.title}
								preload="metadata"
							>
								<source src={video.video_url} type="video/mp4" />
								Your browser does not support the video tag.
							</video>
						</div>

						{#if video.description}
							<div class="video-description">
								<p>{video.description}</p>
							</div>
						{/if}
					</article>
				{/each}
			</div>
		{/if}
	</div>
</section>

<!-- Back to Archive Link -->
<section class="back-section">
	<div class="section-inner">
		<a href={data.archiveUrl} class="back-link">
			<IconArrowLeft size={18} />
			<span>Back to {data.roomName} Archives</span>
		</a>
	</div>
</section>

<style>
	/* Breadcrumbs */
	.breadcrumbs {
		background: #143E59;
		padding: 12px 0;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.breadcrumb-list {
		display: flex;
		align-items: center;
		gap: 8px;
		list-style: none;
		margin: 0;
		padding: 0;
		font-size: 14px;
	}

	.breadcrumb-list a {
		color: rgba(255, 255, 255, 0.8);
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.breadcrumb-list a:hover {
		color: #fff;
	}

	.breadcrumb-list .separator {
		color: rgba(255, 255, 255, 0.5);
	}

	.breadcrumb-list .current {
		color: #fff;
		font-weight: 600;
	}

	/* Title Section */
	.ca-title-section {
		background: #0a2335;
		padding: 30px 20px;
		text-align: center;
	}

	.ca-title-section .section-inner {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 20px;
	}

	.nav-prev, .nav-next {
		flex: 0 0 120px;
	}

	.nav-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: color 0.2s ease;
	}

	.nav-link:hover {
		color: #0db4e8;
	}

	.page-title {
		flex: 1;
		margin: 0;
		font-size: 28px;
		font-weight: 700;
		color: #fff;
		font-family: 'Open Sans', sans-serif;
	}

	@media (min-width: 768px) {
		.page-title {
			font-size: 36px;
		}
	}

	.chat-log-link {
		margin-top: 15px;
	}

	.chat-log-link a {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: color 0.2s ease;
	}

	.chat-log-link a:hover {
		color: #0db4e8;
		text-decoration: underline;
	}

	/* Main Section */
	.ca-main-section {
		background: #f5f5f5;
		padding: 30px 20px;
	}

	@media (min-width: 768px) {
		.ca-main-section {
			padding: 40px 20px;
		}
	}

	.ca-main-section .section-inner {
		max-width: 1000px;
		margin: 0 auto;
	}

	/* Videos Container */
	.videos-container {
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	/* Video Card */
	.video-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.video-header {
		padding: 20px;
		border-bottom: 1px solid #e6e6e6;
	}

	.video-title {
		margin: 0 0 5px;
		font-size: 20px;
		font-weight: 700;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	.video-trader {
		margin: 0;
		font-size: 14px;
		color: #666;
	}

	.video-player-container {
		position: relative;
		background: #000;
	}

	.video-player-container video {
		display: block;
		width: 100%;
		aspect-ratio: 16 / 9;
		object-fit: contain;
	}

	.video-description {
		padding: 15px 20px;
		border-top: 1px solid #e6e6e6;
	}

	.video-description p {
		margin: 0;
		font-size: 14px;
		color: #666;
		line-height: 1.6;
	}

	/* Error & Empty States */
	.error-message,
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		background: #fff;
		border-radius: 8px;
	}

	.error-message {
		background: #fee;
		border: 1px solid #fcc;
	}

	.error-message p,
	.empty-state p {
		margin: 0 0 20px;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 10px;
		font-size: 18px;
		color: #333;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
	}

	.btn-default {
		background: #143E59;
		color: #fff;
	}

	.btn-default:hover {
		background: #0c2638;
	}

	/* Back Section */
	.back-section {
		background: #f5f5f5;
		padding: 0 20px 40px;
	}

	.back-section .section-inner {
		max-width: 1000px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: color 0.2s ease;
	}

	.back-link:hover {
		color: #0db4e8;
	}

	/* Mobile Responsive */
	@media (max-width: 640px) {
		.ca-title-section .section-inner {
			flex-direction: column;
		}

		.nav-prev, .nav-next {
			flex: 0 0 auto;
		}

		.page-title {
			order: -1;
		}
	}
</style>
