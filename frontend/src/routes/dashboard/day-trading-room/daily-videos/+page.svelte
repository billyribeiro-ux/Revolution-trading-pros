<!--
	Day Trading Room - Premium Daily Videos
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Pixel-perfect match: frontend/Implementation/daily-videos
	
	Svelte 5 / SvelteKit 2.0 Best Practices:
	- $props() rune for component props
	- $derived() rune for reactive computed values
	- $state() rune for local state
	- Type imports from server load function
	
	@version 1.0.0 - January 2026
-->
<script lang="ts">
	import type { PageData } from './$types';
	import type { DailyVideo } from './+page.server';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	// Svelte 5 props with SvelteKit typing
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	// Derived state for reactive computed values
	let videos = $derived(data.videos ?? []);
	let pagination = $derived(data.pagination);
	let hasVideos = $derived(videos.length > 0);

	// Local state for search input
	let searchQuery = $state('');

	// Form submission handler - navigates with search params
	function handleSearch(e: Event): void {
		e.preventDefault();
		const url = new URL(window.location.href);
		if (searchQuery) {
			url.searchParams.set('search', searchQuery);
		} else {
			url.searchParams.delete('search');
		}
		url.searchParams.set('page', '1');
		window.location.href = url.toString();
	}

	// Generate pagination URL
	function getPageUrl(page: number): string {
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		return url.toString();
	}

	// Generate video detail URL
	function getVideoUrl(video: DailyVideo): string {
		return `/dashboard/day-trading-room/daily-videos/${video.slug}`;
	}
</script>

<svelte:head>
	<title>Day Trading Room Premium Daily Videos - Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader 
	roomName="Day Trading Room" 
	pageTitle="Premium Daily Videos"
	startHereUrl="/dashboard/day-trading-room/start-here" 
/>

<section class="dashboard__content-section">
	
	<!-- Dashboard Filters -->
	<div class="dashboard-filters">
		<div class="dashboard-filters__count">
			Showing <span class="facetwp-counts">{pagination.page * pagination.perPage - pagination.perPage + 1}-{Math.min(pagination.page * pagination.perPage, pagination.total)} of {pagination.total}</span>
		</div>
		<div class="dashboard-filters__search">
			<form onsubmit={handleSearch}>
				<input 
					type="text" 
					class="facetwp-autocomplete" 
					bind:value={searchQuery}
					placeholder="Search" 
					autocomplete="off"
				/>
				<button type="submit" class="facetwp-autocomplete-update">🔍</button>
			</form>
		</div>
	</div>

	<!-- Video Grid -->
	{#if hasVideos}
		<div id="products-list" class="facetwp-template">
			<div class="card-grid flex-grid row">
				{#each videos as video (video.id)}
					<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
						<div class="card flex-grid-panel">
							<figure class="card-media card-media--video">
								<a href={getVideoUrl(video)} class="card-image" style="background-image: url({video.thumbnail});">
									<img class="default-background" width="325" height="183" alt={video.title} />
								</a>
							</figure>
							<section class="card-body">
								<h4 class="h5 card-title">
									<a href={getVideoUrl(video)}>
										{video.title}
									</a>
								</h4>
								<span class="article-card__meta"><small>{video.date} with {video.trader}</small></span><br>
								<div class="card-description">
									<div class="u--hide-read-more u--squash">
										<p>{video.excerpt}</p>
									</div>
								</div>
							</section>
							<footer class="card-footer">
								<a class="btn btn-tiny btn-default" href={getVideoUrl(video)}>Watch Now</a>
							</footer>
						</div>
					</article>
				{/each}
			</div>
		</div>

		<!-- Pagination -->
		{#if pagination.totalPages > 1}
			<nav class="facetwp-pager" aria-label="Pagination">
				{#each Array.from({ length: pagination.totalPages }, (_, i) => i + 1) as pageNum}
					{#if pageNum === pagination.page}
						<a class="facetwp-page active" href={getPageUrl(pageNum)} data-page={pageNum} aria-current="page">{pageNum}</a>
					{:else if pageNum === 1 || pageNum === pagination.totalPages || Math.abs(pageNum - pagination.page) <= 2}
						<a class="facetwp-page" href={getPageUrl(pageNum)} data-page={pageNum}>{pageNum}</a>
					{:else if Math.abs(pageNum - pagination.page) === 3}
						<span class="facetwp-page-ellipsis">...</span>
					{/if}
				{/each}
			</nav>
		{/if}
	{:else}
		<div class="empty-state">
			<p>No daily videos available at this time.</p>
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DAILY VIDEOS - Pixel-Perfect WordPress Match
	 * Matches: frontend/Implementation/daily-videos
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Filters */
	.dashboard-filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
		flex-wrap: wrap;
		gap: 15px;
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: #666;
	}

	.dashboard-filters__count .facetwp-counts {
		display: inline;
	}

	.dashboard-filters__search {
		display: flex;
		align-items: center;
	}

	.dashboard-filters__search form {
		display: flex;
		align-items: center;
	}

	.dashboard-filters__search .facetwp-autocomplete {
		padding: 8px 12px;
		border: 1px solid #ddd;
		border-radius: 4px 0 0 4px;
		font-size: 14px;
		width: 200px;
		outline: none;
		color: #333;
		background-color: #fff;
	}

	.dashboard-filters__search .facetwp-autocomplete:focus {
		border-color: #0e2433;
	}

	.dashboard-filters__search .facetwp-autocomplete-update {
		padding: 8px 12px;
		background: #0e2433;
		border: 1px solid #0e2433;
		border-left: none;
		border-radius: 0 4px 4px 0;
		cursor: pointer;
		font-size: 14px;
	}

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid-item {
		padding-left: 15px;
		padding-right: 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			width: 33.333333%;
		}
	}

	/* Card Component */
	.card {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.3s ease, transform 0.2s ease;
	}

	.card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.flex-grid-panel {
		height: 100%;
	}

	/* Card Media */
	.card-media {
		margin: 0;
		position: relative;
	}

	.card-media--video .card-image {
		display: block;
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.card-media--video .card-image::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		z-index: 2;
		transition: background 0.3s ease;
	}

	.card-media--video .card-image::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-40%, -50%);
		border-style: solid;
		border-width: 10px 0 10px 18px;
		border-color: transparent transparent transparent #fff;
		z-index: 3;
	}

	.card-media--video .card-image:hover::before {
		background: rgba(247, 148, 29, 0.9);
	}

	.card-media--video .default-background {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	/* Card Body */
	.card-body {
		padding: 20px;
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.card-title {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 8px 0;
		line-height: 1.4;
	}

	.card-title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card-title a:hover {
		color: #f7941d;
	}

	.article-card__meta {
		display: block;
		margin-bottom: 10px;
	}

	.article-card__meta small {
		font-size: 0.8125rem;
		color: #666;
	}

	.card-description {
		flex: 1;
	}

	.card-description p {
		font-size: 0.875rem;
		line-height: 1.6;
		color: #666;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.u--hide-read-more {
		margin-top: 8px;
	}

	/* Card Footer */
	.card-footer {
		padding: 0 20px 20px;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		border-radius: 4px;
		text-decoration: none;
		font-weight: 600;
		font-size: 0.8125rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 0.75rem;
	}

	.btn-default {
		background: #f7941d;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #e5850f;
		color: #fff;
	}

	/* Pagination */
	.facetwp-pager {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 5px;
		margin-top: 30px;
		padding: 20px 0;
	}

	.facetwp-page {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 36px;
		padding: 0 10px;
		border: 1px solid #ddd;
		border-radius: 4px;
		background: #fff;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		transition: all 0.2s ease;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.facetwp-page.active {
		background: #0e2433;
		border-color: #0e2433;
		color: #fff;
	}

	.facetwp-page-ellipsis {
		padding: 0 5px;
		color: #666;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		background: #f9f9f9;
		border-radius: 8px;
	}

	.empty-state p {
		color: #666;
		font-size: 1rem;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard-filters {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard-filters__search .facetwp-autocomplete {
			width: 100%;
		}
	}
</style>