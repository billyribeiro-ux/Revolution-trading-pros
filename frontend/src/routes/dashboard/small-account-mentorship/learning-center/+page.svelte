<!--
	URL: /dashboard/small-account-mentorship/learning-center

	Learning Center Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Educational resources and training materials for Small Account Mentorship members.
	Fetches data with server-side rendering.

	@version 4.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
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
		dataUnavailable?: boolean;
		reason?: string;
	}

	const fallbackThumbnailUrl =
		'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';

	const categoryFilters = [
		{ id: '529', label: 'Trade Setups & Strategies' },
		{ id: '528', label: 'Methodology' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '2932', label: 'Trade & Money Management/Trading Plan' },
		{ id: '531', label: 'Indicators' },
		{ id: '3260', label: 'Options' },
		{ id: '469', label: 'foundation' },
		{ id: '527', label: 'Fundamentals' },
		{ id: '522', label: 'Simpler Tech' },
		{ id: '2929', label: 'Charting/Indicators/Tools' },
		{ id: '530', label: 'Charting' },
		{ id: '3515', label: 'Drama Free Daytrades' },
		{ id: '3516', label: 'Quick Hits Daytrades' },
		{ id: '537', label: 'Psychology' },
		{ id: '775', label: 'Trading Platform' },
		{ id: '3055', label: 'Calls' },
		{ id: '447', label: 'ThinkorSwim' },
		{ id: '446', label: 'TradeStation' },
		{ id: '776', label: 'Charting Software' },
		{ id: '772', label: 'Trading Computer' },
		{ id: '3057', label: 'Calls Puts Credit Spreads' },
		{ id: '3056', label: 'Puts' },
		{ id: '3514', label: 'Profit Recycling' },
		{ id: '791', label: 'Trade Strategies' },
		{ id: '774', label: 'Website Support' },
		{ id: '2927', label: 'Options Strategies (Level 2 & 3)' },
		{ id: '457', label: 'Crypto' },
		{ id: '2931', label: 'Fibonacci & Options Trading' },
		{ id: '2928', label: 'Pricing/Volatility' },
		{ id: '459', label: 'Crypto Indicators & Trading' },
		{ id: '771', label: 'Browser Support' },
		{ id: '2930', label: 'Earnings & Options Expiration' }
	];

	// Server-side data
	let props: { data: PageData } = $props();
	let data = $derived(props.data);

	// Reactive state from server data
	let videos = $derived(data.videos || []);
	let meta = $derived(data.meta || { current_page: 1, per_page: 9, total: 0, last_page: 1 });
	let activeFilter = $derived(data.activeFilter || 'all');
	let error = $derived(data.error);
	// FIX-2026-04-26: surface backend-unavailable state instead of pretending
	// "no videos found" when the request failed.
	let dataUnavailable = $derived(Boolean(data.dataUnavailable));
	let unavailableReason = $derived(data.reason);

	// Pagination derived values
	let currentPage = $derived(meta.current_page);
	let totalPages = $derived(meta.last_page);

	// Filter resources by navigating to new URL with query params
	function filterResources(categoryId: string) {
		const url = new URL(page.url);
		if (categoryId === 'all' || categoryId === '0') {
			url.searchParams.delete('category');
		} else {
			url.searchParams.set('category', categoryId);
		}
		url.searchParams.delete('page'); // Reset to page 1
		void goto(url.toString(), { replaceState: true });
	}

	// Navigate to page
	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			const url = new URL(page.url);
			if (pageNum === 1) {
				url.searchParams.delete('page');
			} else {
				url.searchParams.set('page', pageNum.toString());
			}
			void goto(url.toString(), { replaceState: true });
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
</script>

<svelte:head>
	<title>Learning Center | Small Account Mentorship | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Access educational resources, training materials, and courses for day trading mastery."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader
	roomName="Small Account Mentorship"
	startHereUrl="/dashboard/small-account-mentorship/start-here"
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
			{#each categoryFilters as category (category.id)}
				<div class="filter_btn">
					<input
						type="radio"
						id={category.id}
						value={category.id}
						name="categoryfilter"
						checked={activeFilter === category.id}
						onchange={() => filterResources(category.id)}
					/><label for={category.id}>{category.label}</label>
				</div>
			{/each}
			<input type="hidden" name="page_id" value="402087" />
			<input
				type="hidden"
				name="pagination_base"
				value="/dashboard/small-account-mentorship/learning-center/page/%#%"
			/>

			<button class="apply_filter">Apply filter</button>
			<input type="hidden" name="action" value="myfilter" />
		</form>

		<!-- Section Title - matches WordPress exactly -->
		<section class="dashboard__content-section">
			<h2 class="section-title">
				Small Account Mentorship Learning Center<span> | </span><span>Overview</span>
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

		<!-- FIX-2026-04-26: data-unavailable banner so empty grid isn't mistaken for "no content". -->
		{#if dataUnavailable}
			<div class="data-unavailable" role="status" aria-live="polite">
				<p>Video data temporarily unavailable. Check back soon.</p>
				{#if unavailableReason}
					<p class="data-unavailable__reason">({unavailableReason})</p>
				{/if}
			</div>
		{/if}

		<!-- Learning Resources Grid -->
		<div id="response">
			{#if videos.length === 0 && !error}
				<div class="empty-state">
					<h3>No videos found</h3>
					<p>Try adjusting your filter or check back later for new content.</p>
				</div>
			{:else}
				<div class="learning-grid">
					{#each videos as video (video.id)}
						<div>
							<article class="article-card">
								<figure
									class="article-card__image"
									style:background-image={`url(${video.thumbnail_url || fallbackThumbnailUrl})`}
								>
									<img
										src={fallbackThumbnailUrl}
										alt={video.title}
										loading="lazy"
										width="325"
										height="183"
									/>
								</figure>

								<div class="article-card__type">
									{#each video.tag_details as tag (tag.name)}
										<span class="label label--info" style:background-color={tag.color}>
											{tag.name}
										</span>
									{/each}
								</div>

								<h4 class="h5 article-card__title">
									<a href="/learning-center/{video.slug}">{video.title}</a>
								</h4>

								<div class="u--margin-top-0">
									{#if video.trader}
										<span class="trader_name"><i>With {video.trader.name}</i></span>
									{/if}
								</div>

								<div class="article-card__excerpt u--hide-read-more">
									<p>{video.description || ''}</p>
								</div>

								<a
									href="/learning-center/{video.slug}"
									class="btn btn-tiny btn-default watch-now-btn"
								>
									Watch Now
								</a>
							</article>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="pagination-container">
				<ul class="page-numbers">
					{#if currentPage > 1}
						<li>
							<button
								class="page-numbers"
								onclick={() => goToPage(currentPage - 1)}
								type="button"
								aria-label="Previous page"
							>
								&laquo;
							</button>
						</li>
					{/if}

					{#each getPaginationRange() as pageNum, _pi (_pi)}
						<li>
							{#if pageNum === '...'}
								<span class="page-numbers dots">…</span>
							{:else if pageNum === currentPage}
								<span class="page-numbers current" aria-current="page">{pageNum}</span>
							{:else}
								<button
									class="page-numbers"
									onclick={() => goToPage(Number(pageNum))}
									type="button"
									aria-label="Go to page {pageNum}"
								>
									{pageNum}
								</button>
							{/if}
						</li>
					{/each}

					{#if currentPage < totalPages}
						<li>
							<button
								class="page-numbers"
								onclick={() => goToPage(currentPage + 1)}
								type="button"
								aria-label="Next page"
							>
								&raquo;
							</button>
						</li>
					{/if}
				</ul>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Dashboard Content Main - Padding for cards */
	.dashboard__content-main {
		padding-left: 10px;
		padding-right: 10px;
	}

	/* Section Title */
	.dashboard__content-section {
		margin-bottom: 20px;
	}

	.section-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.section-title span {
		color: #666;
		font-weight: 400;
	}

	/* Term Filter Form - Exact WordPress CSS from dashboard.8f78208b.css */
	#term_filter {
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		overflow-x: scroll;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
		background: #fff;
		padding: 20px;
		-webkit-box-shadow: 0 3px 6px #00000029;
		box-shadow: 0 3px 6px #00000029;
	}

	#term_filter .apply_filter {
		display: none;
	}

	/* Reset Filter Button */
	#term_filter .reset_filter {
		margin-right: 10px;
	}

	#term_filter .reset_filter :global(svg) {
		min-width: 20px;
		color: #666;
	}

	#term_filter .reset_filter label {
		border-radius: 25px;
		cursor: pointer;
		padding: 13px 32px;
		border: 2px solid;
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		margin-bottom: 0;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
		font-family: 'Montserrat', sans-serif;
		color: #666;
	}

	#term_filter .reset_filter input {
		display: none;
	}

	/* Filter Buttons */
	#term_filter .filter_btn {
		margin-right: 10px;
	}

	#term_filter .filter_btn input {
		display: none;
	}

	#term_filter .filter_btn input:checked + label {
		background: #333;
		color: #fff;
	}

	#term_filter .filter_btn label {
		cursor: pointer;
		padding: 11px 32px;
		border-radius: 25px;
		border: 2px solid;
		white-space: nowrap;
		font-weight: 700;
		color: #666;
		display: block;
		margin-bottom: 0;
		font-family: 'Montserrat', sans-serif;
	}

	/* Error and Empty States */
	.error-message {
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		padding: 20px;
		margin-bottom: 20px;
		text-align: center;
	}

	.error-message p {
		margin: 0;
		color: #dc2626;
	}

	.error-detail {
		font-size: 12px;
		color: #6b7280;
		margin-top: 8px;
	}

	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 10px;
		font-size: 18px;
	}

	/* FIX-2026-04-26: info-style banner for backend-unavailable state. */
	.data-unavailable {
		background: #eff6ff;
		border: 1px solid #bfdbfe;
		border-radius: 8px;
		padding: 16px 20px;
		margin-bottom: 20px;
		text-align: center;
	}

	.data-unavailable p {
		margin: 0;
		color: #1e40af;
	}

	.data-unavailable__reason {
		font-size: 12px;
		color: #6b7280;
		margin-top: 6px;
	}

	.learning-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	@media (min-width: 768px) {
		.learning-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.learning-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	/* Article Card */
	.article-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
		padding: 0 0 20px;
	}

	.article-card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		margin: 0;
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
		padding: 15px 20px 10px;
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 3px;
	}

	.label--info {
		background: #e8f4fc;
		color: #0984ae;
	}

	.article-card__title {
		padding: 0 20px;
		margin: 0 0 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #f69532;
	}

	.u--margin-top-0 {
		padding: 0 20px;
		margin-top: 0;
	}

	.trader_name {
		font-size: 13px;
		color: #666;
	}

	.trader_name i {
		font-style: italic;
	}

	.article-card__excerpt {
		padding: 10px 20px;
		flex-grow: 1;
	}

	.article-card__excerpt p {
		margin: 0;
		font-size: 14px;
		color: #666;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
		line-height: 1.5;
	}

	.u--hide-read-more {
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
	}

	/* Button */
	.btn {
		display: inline-block;
		margin: 10px 20px 0;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		align-self: flex-start;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f69532;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Pagination */
	.pagination-container {
		margin-top: 40px;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 4px;
		text-align: center;
	}

	.pagination-container ul.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: inline-flex;
		flex-wrap: wrap;
		gap: 0;
	}

	.pagination-container li {
		display: inline-block;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.pagination-container li button.page-numbers,
	.pagination-container li span.page-numbers {
		border: 1px solid #e6e6e6;
		display: inline-block;
		padding: 10px 16px;
		margin: 0;
		min-width: 45px;
		text-align: center;
		text-decoration: none;
		color: #1e73be;
		background: #fff;
		transition: all 0.2s ease;
		cursor: pointer;
		font-family: inherit;
		font-size: 16px;
		font-weight: 400;
	}

	.pagination-container li button.page-numbers:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.pagination-container li span.current {
		background: #1e73be;
		color: #fff;
		border-color: #1e73be;
		font-weight: 600;
	}

	.pagination-container li span.dots {
		background: #fff;
		cursor: default;
		color: #333;
	}

	/* Mobile-first: smaller title by default, larger on md+ */
	.section-title {
		font-size: 20px;
	}

	@media (min-width: 768px) {
		.section-title {
			font-size: 24px;
		}
	}
</style>
