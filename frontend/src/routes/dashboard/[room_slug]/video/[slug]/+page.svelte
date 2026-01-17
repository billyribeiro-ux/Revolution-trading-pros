<!--
	URL: /dashboard/[room_slug]/video/[slug]
	
	Video Detail Page - Multi-Room Support
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Based on LatestUpdatesPage reference
	
	@version 3.0.0 - January 2026 - Multi-room support
	@author Revolution Trading Pros
-->
<script lang="ts">
	import type { PageData } from './+page.server';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Server-loaded data with Previous/Next navigation
	let { data }: { data: PageData } = $props();

	// Reactive video data from server
	const video = $derived(data.video);
	const roomSlug = $derived(data.roomSlug);
	const roomName = $derived(data.roomName);

	// Dropdown state
	let isDropdownOpen = $state(false);

	// Trading rooms for dropdown
	const tradingRooms = [
		{
			name: 'Day Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'chart-line'
		},
		{
			name: 'Swing Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'trending-up'
		},
		{
			name: 'Small Accounts Mentorship',
			href: '#', // TODO: Provide URL
			icon: 'dollar-sign'
		}
	];

	function toggleDropdown(event: Event): void {
		event.stopPropagation();
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}

	// Close dropdown when clicking outside
	$effect(() => {
		if (isDropdownOpen && typeof window !== 'undefined') {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown')) {
					closeDropdown();
				}
			};
			const handleEscape = (e: KeyboardEvent) => {
				if (e.key === 'Escape') {
					closeDropdown();
				}
			};
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleEscape);
			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleEscape);
			};
		}
		return undefined;
	});

	// Related videos (could also be fetched from server in the future)
	const relatedVideos = [
		{
			title: 'A Cautious Entry Into 2026',
			slug: 'cautious-entry-2026',
			author: 'HG',
			date: 'December 31, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			excerpt:
				"If Santa doesn't show up, the first bit of 2026 may be a little precarious. With that in mind, let's dive in to some of the most important charts for the new year."
		},
		{
			title: 'SPX Snoozefest',
			slug: 'spx-snoozefest',
			author: 'Heather',
			date: 'December 30, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
			excerpt:
				"We've had two days of some very narrow ranges in the indices. It's almost as though the market has had an amazing year and just needs to rest a bit before making its next move!"
		},
		{
			title: 'Signal & Noise',
			slug: 'signal-noise',
			author: 'Sam',
			date: 'December 26, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			excerpt:
				'The signal we have been tracking in the Nasdaq continues to build. As we approach the new year, where are the pockets of strength?'
		}
	];
</script>

<svelte:head>
	<title>{video.title} | {roomName} | Revolution Trading Pros</title>
	<meta name="description" content={video.description.replace(/<[^>]*>/g, '')} />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{roomName} Dashboard</h1>
	</div>
	<div class="dashboard__header-right">
		<div class="dropdown" class:is-open={isDropdownOpen}>
			<button
				class="btn btn-orange btn-tradingroom"
				onclick={toggleDropdown}
				aria-expanded={isDropdownOpen}
				aria-haspopup="true"
				type="button"
			>
				<strong>Enter the Trading Room</strong>
				<span class="dropdown-arrow">
					<RtpIcon name="chevron-down" size={14} />
				</span>
			</button>

			{#if isDropdownOpen}
				<div class="dropdown-menu" role="menu">
					{#each tradingRooms as room}
						<a href={room.href} class="dropdown-item" onclick={closeDropdown} role="menuitem">
							<span class="dropdown-item__icon">
								<RtpIcon name={room.icon} size={20} />
							</span>
							<span class="dropdown-item__text">{room.name}</span>
						</a>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</header>

<!-- Video Title Section -->
<section id="dv-title" class="dv-section cpost-title-section cpost-section">
	<div class="section-inner">
		{#if video.previousVideo}
			<div id="dv-previous" class="cpost-previous">
				<a
					href="/dashboard/{roomSlug}/video/{video.previousVideo.slug}"
					title={video.previousVideo.title}
				>
					<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
				</a>
			</div>
		{/if}

		<h1 class="cpost-title">{video.title}</h1>

		{#if video.nextVideo}
			<div id="dv-next" class="cpost-next">
				<a href="/dashboard/{roomSlug}/video/{video.nextVideo.slug}" title={video.nextVideo.title}>
					<span>Next </span><i class="fa fa-chevron-circle-right"></i>
				</a>
			</div>
		{/if}

		<h2 class="cpost-subtitle">With {video.author}</h2>
	</div>
</section>

<!-- Video Player Section -->
<section id="dv-main" class="dv-section cpost-section">
	<div class="section-inner">
		<div class="dv-content-block cpost-content-block w-desc">
			<div class="current-vid">
				<div class="video-container current">
					<video
						id="dv-player"
						controls
						width="100%"
						poster={video.thumbnailUrl}
						style="aspect-ratio: 16/9;"
					>
						<source src={video.videoUrl} type="video/mp4" />
						Your browser does not support the video tag.
					</video>
				</div>
			</div>
		</div>
		<div class="dv-description">
			{@html video.description}
		</div>
	</div>
</section>

<!-- Related Videos Section -->
<section id="dv-recent" class="dv-section cpost-recent-section cpost-section">
	<div class="section-inner">
		<h2>Recent Day Trading Room Daily Videos</h2>

		<div class="card-grid flex-grid row">
			{#each relatedVideos as relatedVideo}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a
								href="/dashboard/{roomSlug}/video/{relatedVideo.slug}"
								class="card-image"
								style="background-image: url({relatedVideo.thumbnail});"
							>
								<img src="https://placehold.it/325x183" alt={relatedVideo.title} />
							</a>
						</figure>
						<section class="card-body">
							<h4 class="h5 card-title">
								<a href="/dashboard/{roomSlug}/video/{relatedVideo.slug}">
									{relatedVideo.title}
								</a>
							</h4>
							<span class="article-card__meta"
								><small>{relatedVideo.date} with {relatedVideo.author}</small></span
							><br />
							<div class="card-description">
								<div class="u--hide-read-more u--squash">{relatedVideo.excerpt}</div>
							</div>
						</section>
						<footer class="card-footer">
							<a
								class="btn btn-tiny btn-default"
								href="/dashboard/{roomSlug}/video/{relatedVideo.slug}">Watch Now</a
							>
						</footer>
					</div>
				</article>
			{/each}
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}

	@media (min-width: 1024px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex: 1;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 28px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.dashboard__header-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		margin-top: 10px;
	}

	@media (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: column;
			align-items: flex-end;
			gap: 0;
			margin-top: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTONS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.btn-orange {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		background-color: #f69532;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background-color: #dc7309;
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		text-transform: none;
		width: 280px;
		padding: 12px 18px;
	}

	.dropdown-arrow {
		font-size: 10px;
		transition: transform 0.15s ease-in-out;
		display: flex;
		align-items: center;
	}

	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		padding: 15px;
		min-width: 260px;
		max-width: 280px;
		margin: 5px 0 0;
		font-size: 14px;
		background-color: #ffffff;
		border: none;
		border-radius: 5px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		transition: all 0.15s ease-in-out;
	}

	.dropdown:not(.is-open) .dropdown-menu {
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		pointer-events: none;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 15px 20px;
		color: #666;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
		border-radius: 5px;
		white-space: nowrap;
	}

	.dropdown-item:hover {
		background-color: #f4f4f4;
	}

	.dropdown-item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: #143e59;
	}

	.dropdown-item__text {
		flex: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * VIDEO DETAIL PAGE STYLES - Pixel-Perfect Match to MasterDash
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Title Section */
	.dv-section {
		padding: 30px 0;
	}

	.cpost-title-section {
		background: #f4f4f4;
		border-bottom: 1px solid #e5e5e5;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
		position: relative;
	}

	.cpost-title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #191717;
		text-align: center;
		margin: 0 0 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.cpost-subtitle {
		font-size: 1.3rem;
		font-weight: 400;
		color: #666;
		text-align: center;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.cpost-previous,
	.cpost-next {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
	}

	.cpost-previous {
		left: 20px;
	}

	.cpost-next {
		right: 20px;
	}

	.cpost-previous a,
	.cpost-next a {
		color: #143e59;
		text-decoration: none;
		font-size: 1rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #076a8a;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 1.5rem;
	}

	/* Video Player Section */
	.cpost-section {
		background: #fff;
	}

	.dv-content-block {
		margin-bottom: 30px;
	}

	.video-container {
		position: relative;
		width: 100%;
		max-width: 1000px;
		margin: 0 auto;
	}

	.video-container video {
		width: 100%;
		height: auto;
		display: block;
		background: #000;
	}

	.dv-description {
		max-width: 1000px;
		margin: 0 auto;
		font-size: 1.1rem;
		line-height: 1.8;
		color: #555;
	}

	.dv-description :global(p) {
		margin-bottom: 15px;
	}

	/* Related Videos Section */
	.cpost-recent-section {
		background: #f8f9fa;
		padding: 50px 0;
	}

	.cpost-recent-section h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #191717;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
	}

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -10px;
	}

	.card-grid-spacer {
		padding: 0 10px 20px;
	}

	.card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		overflow: hidden;
	}

	.card-image {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		display: block;
	}

	.card-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(9, 132, 174, 0.9);
		border-radius: 50%;
		pointer-events: none;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.card-title {
		font-size: 1.2rem;
		font-weight: 700;
		margin-bottom: 10px;
	}

	.card-title a {
		color: #191717;
		text-decoration: none;
	}

	.card-title a:hover {
		color: #143e59;
	}

	.card-description {
		margin-top: 10px;
		color: #666;
		line-height: 1.6;
	}

	.u--hide-read-more {
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-footer {
		padding: 0 20px 20px;
	}

	/* Mobile-first: smaller title by default, larger on md+ */
	.cpost-title {
		font-size: 1.8rem;
	}

	@media (min-width: 769px) {
		.cpost-title {
			font-size: 2.5rem;
		}

		.cpost-subtitle {
			font-size: 1.1rem;
		}

		.cpost-previous,
		.cpost-next {
			position: static;
			transform: none;
			text-align: center;
			margin: 10px 0;
		}

		.cpost-previous {
			order: -1;
		}

		.cpost-next {
			order: 1;
		}

		.section-inner {
			display: flex;
			flex-direction: column;
		}
	}
</style>
