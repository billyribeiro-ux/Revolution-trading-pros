<!--
	URL: /dashboard/spx-profit-pulse/learning-center

	SPX Profit Pulse Learning Center Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Reference: frontend/Do's/SPX Tr3ndy/LearningCenter

	Educational resources and training materials for SPX Profit Pulse members.
	Fetches data with server-side rendering.

	@version 1.0.0 - January 2026
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import type { VideoResponse } from './+page.server';

	interface PageData {
		videos: VideoResponse[];
		meta: {
			current_page: number;
			per_page: number;
			total: number;
			last_page: number;
		};
		activeFilter: string;
		error: string | null;
	}

	// Server-side data
	let props: { data: PageData } = $props();
	let data = $derived(props.data);

	// Reactive state from server data
	let videos = $derived(data.videos || []);
	let meta = $derived(data.meta || { current_page: 1, per_page: 9, total: 0, last_page: 1 });
	let activeFilter = $derived(data.activeFilter || 'all');
	let error = $derived(data.error);

	// Pagination derived values
	let currentPage = $derived(meta.current_page);
	let totalPages = $derived(meta.last_page);
	let totalItems = $derived(meta.total);

	// Category options matching WordPress SPX reference - using WordPress category IDs
	const categories = [
		{ id: '2929', label: 'Charting/Indicators/Tools' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '528', label: 'Methodology' },
		{ id: '529', label: 'Trade Setups & Strategies' }
	];

	// Filter resources by navigating to new URL with query params
	function filterResources(categoryId: string) {
		const url = new URL($page.url);
		if (categoryId === 'all' || categoryId === '0') {
			url.searchParams.delete('category');
		} else {
			url.searchParams.set('category', categoryId);
		}
		url.searchParams.delete('page'); // Reset to page 1
		goto(url.toString(), { replaceState: true });
	}

	// Navigate to page
	function goToPage(pageNum: number) {
		if (!browser) return;
		if (pageNum >= 1 && pageNum <= totalPages) {
			const url = new URL($page.url);
			if (pageNum === 1) {
				url.searchParams.delete('page');
			} else {
				url.searchParams.set('page', pageNum.toString());
			}
			goto(url.toString(), { replaceState: true });
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function getPaginationRange(): (number | string)[] {
		const range: (number | string)[] = [];
		const delta = 1;

		if (totalPages <= 7) {
			for (let i = 1; i <= totalPages; i++) {
				range.push(i);
			}
		} else {
			// Always show first page
			range.push(1);

			if (currentPage > 3) {
				range.push('...');
			}

			// Pages around current
			for (
				let i = Math.max(2, currentPage - delta);
				i <= Math.min(totalPages - 1, currentPage + delta);
				i++
			) {
				if (!range.includes(i)) {
					range.push(i);
				}
			}

			if (currentPage < totalPages - 2) {
				range.push('...');
			}

			// Always show last page
			if (!range.includes(totalPages)) {
				range.push(totalPages);
			}
		}

		return range;
	}

	function getCategoryLabel(categoryId: string): string {
		const category = categories.find((c) => c.id === categoryId);
		return category?.label || categoryId;
	}
</script>

<svelte:head>
	<title>Learning Center | SPX Profit Pulse | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access educational resources, training materials, and courses for SPX trading mastery."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="SPX Profit Pulse"
	startHereUrl="/dashboard/spx-profit-pulse/start-here"
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filter Form - matches WordPress exactly -->
		<form action="#" method="POST" id="term_filter" onsubmit={(e) => e.preventDefault()}>
			<div class="reset_filter">
				<input
					type="radio"
					id="0"
					value="0"
					name="categoryfilter"
					checked={activeFilter === 'all'}
					onchange={() => filterResources('all')}
				/>
				<label for="0">
					<svg
						aria-hidden="true"
						focusable="false"
						data-prefix="fas"
						data-icon="undo"
						class="svg-inline--fa fa-undo fa-w-16"
						role="img"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 512 512"
					>
						<path
							fill="currentColor"
							d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"
						></path>
					</svg>
				</label>
			</div>
			{#each categories as category}
				<div class="filter_btn">
					<input
						type="radio"
						id={category.id}
						value={category.id}
						name="categoryfilter"
						checked={activeFilter === category.id}
						onchange={() => filterResources(category.id)}
					/>
					<label for={category.id}>{category.label}</label>
				</div>
			{/each}
			<input type="hidden" name="page_id" value="2055757" />
			<input
				type="hidden"
				name="pagination_base"
				value="/dashboard/spx-profit-pulse/learning-center/page/%#%"
			/>

			<button class="apply_filter">Apply filter</button>
			<input type="hidden" name="action" value="myfilter" />
		</form>

		<!-- Section Title - matches WordPress exactly -->
		<section class="dashboard__content-section">
			<h2 class="section-title">
				SPX Profit Pulse Learning Center<span> | </span><span>Overview</span>
			</h2>
			<p></p>
		</section>

		<!-- Error State -->
		{#if error}
			<div class="error-message">
				<p>Unable to load videos. Please try again later.</p>
				<p class="error-detail">{error}</p>
			</div>
		{/if}

		<!-- Videos Grid -->
		{#if videos.length > 0}
			<div id="response">
				<div class="article-cards row flex-grid">
					{#each videos as video}
						<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
							<article class="article-card">
								<figure
									class="article-card__image"
									style="background-image: url({video.thumbnail_url ||
										'https://cdn.simplertrading.com/2024/09/23132611/Jon-Generic.jpg'});"
								>
									<img
										src={video.thumbnail_url ||
											'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg'}
										alt={video.title}
									/>
								</figure>
								{#if video.tag_details && video.tag_details.length > 0}
									<div class="article-card__type">
										{#each video.tag_details as tag}
											<span id={tag.slug} class="label label--info">{tag.name}</span>
										{/each}
									</div>
								{/if}
								<h4 class="h5 article-card__title">
									<a href="/dashboard/spx-profit-pulse/learning-center/{video.slug}"
										>{video.title}</a
									>
								</h4>
								{#if video.trader}
									<div class="u--margin-top-0">
										<span class="trader_name"><i>With {video.trader.name}</i></span>
									</div>
								{/if}
								<div class="article-card__excerpt u--hide-read-more">
									<p>{video.description || ''}</p>
								</div>
								<a
									href="/dashboard/spx-profit-pulse/learning-center/{video.slug}"
									class="btn btn-tiny btn-default">Watch Now</a
								>
							</article>
						</div>
					{/each}
				</div>
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="facetwp-pager">
					{#each getPaginationRange() as item}
						{#if typeof item === 'number'}
							<button
								type="button"
								class="facetwp-page {item === currentPage ? 'active' : ''}"
								onclick={() => goToPage(item)}
								data-page={item}
							>
								{item}
							</button>
						{:else}
							<span class="facetwp-page dots">{item}</span>
						{/if}
					{/each}
				</div>
			{/if}
		{:else}
			<div class="empty-state">
				<p>No learning resources available at this time.</p>
			</div>
		{/if}
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * SPX PROFIT PULSE LEARNING CENTER - Pixel-Perfect WordPress Match
	 * Reference: frontend/Do's/SPX Tr3ndy/LearningCenter
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Full width layout - no sidebar */
	.dashboard__content {
		display: block;
		width: 100%;
	}

	.dashboard__content-main {
		width: 100%;
		max-width: 100%;
		min-width: 100%;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	/* md: 768px - Tablets */
	@media screen and (min-width: 768px) {
		.dashboard__content-section {
			padding: 40px 30px 20px;
		}
	}

	.section-title {
		font-size: 1.75rem;
		font-weight: 700;
		margin: 0 0 20px 0;
		color: #1a1a1a;
	}

	.section-title span {
		color: #666;
	}

	/* Category Filter Form */
	#term_filter {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 8px;
		margin-bottom: 30px;
		align-items: center;
	}

	.reset_filter {
		margin-right: 10px;
	}

	.reset_filter input[type='radio'] {
		display: none;
	}

	.reset_filter label {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #fff;
		border: 2px solid #ddd;
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.reset_filter label:hover {
		border-color: #143e59;
		background: #f0f0f0;
	}

	.reset_filter input[type='radio']:checked + label {
		border-color: #143e59;
		background: #143e59;
	}

	.reset_filter input[type='radio']:checked + label svg {
		fill: #fff;
	}

	.reset_filter svg {
		width: 20px;
		height: 20px;
		fill: #666;
		transition: fill 0.2s ease;
	}

	.filter_btn {
		display: inline-block;
	}

	.filter_btn input[type='radio'] {
		display: none;
	}

	.filter_btn label {
		display: inline-block;
		padding: 10px 20px;
		background: #fff;
		border: 2px solid #ddd;
		border-radius: 25px;
		cursor: pointer;
		font-size: 14px;
		font-weight: 500;
		color: #333;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.filter_btn label:hover {
		border-color: #143e59;
		background: #f0f0f0;
	}

	.filter_btn input[type='radio']:checked + label {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.apply_filter {
		padding: 10px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 25px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
		margin-left: auto;
	}

	.apply_filter:hover {
		background: #0c2638;
	}

	/* Article Cards Grid */
	.article-cards {
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

	/* sm: 392px - Large phones: 2 columns */
	@media (min-width: 392px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	/* md: 768px - Tablets: 2 columns */
	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	/* xl: 1280px - Large desktops: 3 columns */
	@media (min-width: 1280px) {
		.col-xl-4 {
			width: 33.333333%;
		}
	}

	/* Article Card */
	.article-card {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition:
			box-shadow 0.3s ease,
			transform 0.2s ease;
	}

	.article-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.article-card__image {
		margin: 0;
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 10px 15px;
		display: flex;
		flex-wrap: wrap;
		gap: 5px;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 12px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.label--info {
		background: #143e59;
		color: #fff;
	}

	.article-card__title {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 15px 10px;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #143e59;
	}

	.u--margin-top-0 {
		margin-top: 0;
		padding: 0 15px 10px;
	}

	.trader_name {
		font-size: 0.875rem;
		color: #666;
	}

	.article-card__excerpt {
		padding: 0 15px 15px;
		flex: 1;
	}

	.article-card__excerpt p {
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
		margin-top: 0;
	}

	/* Button */
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
		margin: 0 15px 15px;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 12px;
		height: 40px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.btn-default {
		background: #143e59;
		color: #fff;
		border: 1px solid #143e59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	/* Pagination */
	.facetwp-pager {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 5px;
		margin: 30px 0;
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
		cursor: pointer;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.facetwp-page.active {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.facetwp-page.dots {
		border: none;
		cursor: default;
	}

	.facetwp-page.dots:hover {
		background: transparent;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		background: #f9f9f9;
		border-radius: 8px;
		margin: 20px;
	}

	.empty-state p {
		color: #666;
		font-size: 1rem;
	}

	/* Error Message */
	.error-message {
		background: #fee;
		border: 1px solid #fcc;
		border-radius: 8px;
		padding: 20px;
		margin: 20px;
	}

	.error-message p {
		color: #c33;
		margin: 5px 0;
	}

	.error-detail {
		font-size: 0.875rem;
		opacity: 0.8;
	}

	/* Responsive */
	@media (max-width: 768px) {
		#term_filter {
			padding: 15px;
		}

		.apply_filter {
			width: 100%;
			margin-left: 0;
			margin-top: 10px;
		}
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
