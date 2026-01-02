<!--
	URL: /chatroom-archive/[slug]/[date]
-->

<script lang="ts">
	/**
	 * Chatroom Archive - Dynamic Route
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match to Simpler Trading reference (LatestUpdates)
	 * URL structure: /chatroom-archive/[slug]/[date]
	 * Example: /chatroom-archive/mastering-the-trade/12242025
	 */
	import { page } from '$app/stores';

	const slug = $derived($page.params.slug);
	const dateSlug = $derived($page.params.date);

	// Parse date from slug (MMDDYYYY format)
	function parseDate(dateStr: string): { display: string; prevDate: string; nextDate: string } {
		if (!dateStr || dateStr.length !== 8) {
			return { display: 'Unknown Date', prevDate: '', nextDate: '' };
		}
		const month = parseInt(dateStr.substring(0, 2));
		const day = parseInt(dateStr.substring(2, 4));
		const year = parseInt(dateStr.substring(4, 8));

		const date = new Date(year, month - 1, day);
		const months = ['January', 'February', 'March', 'April', 'May', 'June',
						'July', 'August', 'September', 'October', 'November', 'December'];

		// Calculate prev/next dates
		const prevDay = new Date(date);
		prevDay.setDate(prevDay.getDate() - 1);
		const nextDay = new Date(date);
		nextDay.setDate(nextDay.getDate() + 1);

		const formatDateSlug = (d: Date) => {
			return String(d.getMonth() + 1).padStart(2, '0') +
				   String(d.getDate()).padStart(2, '0') +
				   d.getFullYear();
		};

		return {
			display: `${months[month - 1]} ${day}, ${year}`,
			prevDate: formatDateSlug(prevDay),
			nextDate: formatDateSlug(nextDay)
		};
	}

	const dateInfo = $derived(parseDate(dateSlug));

	// Room name from slug
	const roomName = $derived(
		slug?.split('-').map((w: string) => w.charAt(0).toUpperCase() + w.slice(1)).join(' ') || 'Trading Room'
	);

	// Sample videos - matching reference structure exactly
	const videos = [
		{
			id: 'taylor-horton1',
			title: 'Nail the Open',
			trader: 'Taylor Horton',
			poster: 'https://s3.amazonaws.com/simpler-cdn/azure-blob-files/chatroom/chatroom-th.jpg',
			source: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/SOGold/sample-video.mp4'
		},
		{
			id: 'melissa-beegle2',
			title: 'Day Trading Options',
			trader: 'Melissa Beegle',
			poster: 'https://cdn.simplertrading.com/2024/01/25025702/melissa.png',
			source: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/SOGold/sample-video.mp4'
		},
		{
			id: 'tg-watkins3',
			title: 'Moxie',
			trader: 'TG Watkins',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/chatroom/chatroom-tgw.jpg',
			source: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/SOGold/sample-video.mp4'
		},
		{
			id: 'lorna-st-george4',
			title: 'Simpler Tech',
			trader: 'Lorna St. George',
			poster: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/chatroom/chatroom-lorna.jpg',
			source: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/SOGold/sample-video.mp4'
		}
	];
</script>

<svelte:head>
	<title>{dateInfo.display} - Revolution Trading Pros</title>
</svelte:head>

<!-- Breadcrumbs - Exact match to reference -->
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
			</li>
			<li class="separator separator-home"> / </li>
			<li class="item-cat item-custom-post-type-chat-archives">
				<a class="breadcrumb-cat breadcrumb-custom-post-type-chat-archives" href="/chatroom-archive" title="Chat Archives">Chat Archives</a>
			</li>
			<li class="separator"> / </li>
			<li class="item-current">
				<strong class="breadcrumb-current" title={dateInfo.display}>{dateInfo.display}</strong>
			</li>
		</ul>
	</div>
</nav>

<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<!-- Title Section with Previous/Next Navigation -->
		<section id="ca-title" class="ca-section cpost-title-section cpost-section">
			<div class="section-inner">
				<div id="ca-previous" class="cpost-previous">
					<a href="/chatroom-archive/{slug}/{dateInfo.prevDate}" title="Previous Day">
						<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
					</a>
				</div>
				<h1 class="cpost-title">{dateInfo.display}</h1>
				<div id="ca-next" class="cpost-next">
					<a href="/chatroom-archive/{slug}/{dateInfo.nextDate}" title="Next Day">
						<span>Next </span><i class="fa fa-chevron-circle-right"></i>
					</a>
				</div>
				<h2 class="cpost-subtitle">
					<a href="#" target="_blank" rel="nofollow">View Chat Log</a>
				</h2>
			</div>
		</section>

		<!-- Main Content Section with Videos -->
		<section id="ca-main" class="ca-section cpost-section">
			<div class="section-inner">
				<div class="ca-content-block cpost-content-block">
					{#each videos as video (video.id)}
						<div class="current-vid">
							<h2 class="current-title">{video.title}</h2>
							<p class="current-trader">{video.trader}</p>
							<div class="video-container current">
								<div class="video-overlay">
									<h3 class="d-none">{video.title}</h3>
									<h4>{video.trader}</h4>
									<p class="d-none">{video.trader}</p>
								</div>
								<div id={video.id} class="video-player">
									<video
										id="{video.id}-video"
										controls
										width="100%"
										poster={video.poster}
										style="aspect-ratio: 16/9;"
										title={video.title}
									>
										<source src={video.source} type="video/mp4" />
										Your browser does not support the video tag.
									</video>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CHATROOM ARCHIVE STYLES - Exact match to Simpler Trading reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f5f5f5;
		padding: 12px 0;
		font-size: 13px;
		font-family: 'Open Sans', sans-serif;
	}

	.breadcrumbs .container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
	}

	.breadcrumbs li {
		display: inline;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		color: #666;
		margin: 0 5px;
	}

	.breadcrumbs .breadcrumb-current {
		color: #333;
	}

	/* Page Container */
	#page {
		background: #efefef;
		min-height: 100vh;
	}

	#content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 20px;
	}

	/* Title Section */
	.ca-section {
		background: #fff;
		border-radius: 4px;
		margin-bottom: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
	}

	.cpost-title-section .section-inner {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		padding: 30px 20px;
		position: relative;
		gap: 20px;
	}

	.cpost-previous,
	.cpost-next {
		flex: 0 0 auto;
	}

	.cpost-previous a,
	.cpost-next a {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.15s ease-in-out;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #065f7c;
	}

	.cpost-previous .fa,
	.cpost-next .fa {
		font-size: 24px;
	}

	/* Font Awesome fallback using CSS */
	.fa-chevron-circle-left::before {
		content: '◀';
		font-style: normal;
	}

	.fa-chevron-circle-right::before {
		content: '▶';
		font-style: normal;
	}

	.cpost-title {
		flex: 1;
		text-align: center;
		font-size: 32px;
		font-weight: 700;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		color: #333;
		margin: 0;
		line-height: 1.2;
	}

	.cpost-subtitle {
		width: 100%;
		text-align: center;
		margin: 10px 0 0;
		font-size: 14px;
		font-weight: 400;
	}

	.cpost-subtitle a {
		color: #0984ae;
		text-decoration: none;
	}

	.cpost-subtitle a:hover {
		text-decoration: underline;
	}

	/* Main Content Section */
	#ca-main .section-inner {
		padding: 30px 20px;
	}

	.ca-content-block {
		display: flex;
		flex-direction: column;
		gap: 40px;
	}

	/* Individual Video Block */
	.current-vid {
		border-bottom: 1px solid #ededed;
		padding-bottom: 40px;
	}

	.current-vid:last-child {
		border-bottom: none;
		padding-bottom: 0;
	}

	.current-title {
		font-size: 24px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		margin: 0 0 5px;
	}

	.current-trader {
		font-size: 14px;
		color: #666;
		margin: 0 0 15px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Video Container */
	.video-container {
		position: relative;
		width: 100%;
		max-width: 800px;
	}

	.video-overlay {
		display: none;
	}

	.d-none {
		display: none !important;
	}

	.video-player {
		width: 100%;
	}

	.video-player video {
		width: 100%;
		height: auto;
		border-radius: 4px;
		background: #000;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.cpost-title-section .section-inner {
			flex-direction: column;
			gap: 15px;
		}

		.cpost-previous,
		.cpost-next {
			order: 2;
		}

		.cpost-title {
			order: 1;
			font-size: 24px;
		}

		.cpost-subtitle {
			order: 3;
		}

		.current-title {
			font-size: 20px;
		}
	}
</style>
