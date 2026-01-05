<!--
	URL: /dashboard/day-trading-room/daily-videos
	
	Premium Daily Videos Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Daily video analysis and market commentary for Day Trading Room members.
	Includes pagination matching WordPress implementation.
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	interface DailyVideo {
		id: number;
		title: string;
		date: string;
		trader: string;
		excerpt: string;
		slug: string;
		thumbnail: string;
		isVideo: boolean;
	}

	// Pagination state
	let currentPage = 1;
	let totalPages = 63;
	let itemsPerPage = 12;
	let totalItems = 750;
	let displayedVideos: DailyVideo[] = [];
	
	// Search state
	let searchQuery = '';
	let filteredVideos: DailyVideo[] = [];

	// Sample data - matches WordPress structure
	const allVideos: DailyVideo[] = [
		{
			id: 1,
			title: 'How to use Bookmap to make more informed trades',
			date: 'January 02, 2026',
			trader: 'Kody Ashmore',
			excerpt: "You asked for it, you got it. Here are Kody's Bookmap tools and how he uses them to make better informed trades.",
			slug: 'bookmap',
			thumbnail: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			isVideo: true
		},
		{
			id: 2,
			title: 'Cautious entry into 2026',
			date: 'December 31, 2025',
			trader: 'Henry Gambell',
			excerpt: 'As we head into the new year, here are some key considerations for your trading strategy.',
			slug: 'cautious-entry-into-2026',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		{
			id: 3,
			title: 'Market Analysis & Trading Strategies',
			date: 'December 23, 2025',
			trader: 'HG',
			excerpt: "Things can always change, but given how the market closed on Tuesday, it looks like Santa's on his way.",
			slug: 'market-analysis',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		// Add more sample videos to demonstrate pagination
		...Array.from({ length: 9 }, (_, i) => ({
			id: i + 4,
			title: `Daily Market Update ${i + 4}`,
			date: `December ${20 - i}, 2025`,
			trader: i % 2 === 0 ? 'Kody Ashmore' : 'Henry Gambell',
			excerpt: 'Expert analysis and trading insights for today\'s market conditions.',
			slug: `daily-update-${i + 4}`,
			thumbnail: i % 2 === 0 
				? 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg'
				: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		}))
	];

	function filterVideos() {
		if (!searchQuery.trim()) {
			filteredVideos = allVideos;
		} else {
			const query = searchQuery.toLowerCase();
			filteredVideos = allVideos.filter(video => 
				video.title.toLowerCase().includes(query) ||
				video.trader.toLowerCase().includes(query) ||
				video.excerpt.toLowerCase().includes(query)
			);
		}
		
		// Update total items and pages based on filtered results
		totalItems = filteredVideos.length;
		totalPages = Math.ceil(totalItems / itemsPerPage);
		
		// Reset to page 1 when search changes
		if (currentPage > totalPages) {
			currentPage = 1;
		}
		
		updateDisplayedVideos();
	}
	
	function updateDisplayedVideos() {
		const startIndex = (currentPage - 1) * itemsPerPage;
		const endIndex = startIndex + itemsPerPage;
		displayedVideos = filteredVideos.slice(startIndex, endIndex);
	}
	
	function handleSearch(event: Event) {
		const target = event.target as HTMLInputElement;
		searchQuery = target.value;
		filterVideos();
	}

	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			currentPage = pageNum;
			updateDisplayedVideos();
			// Scroll to top
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function getPaginationRange(): (number | string)[] {
		const range: (number | string)[] = [];
		
		// Always show first page
		range.push(1);
		
		if (currentPage > 3) {
			range.push('...');
		}
		
		// Show pages around current page
		for (let i = Math.max(2, currentPage - 1); i <= Math.min(totalPages - 1, currentPage + 1); i++) {
			if (!range.includes(i)) {
				range.push(i);
			}
		}
		
		if (currentPage < totalPages - 2) {
			range.push('...');
		}
		
		// Always show last page
		if (totalPages > 1 && !range.includes(totalPages)) {
			range.push(totalPages);
		}
		
		return range;
	}

	onMount(() => {
		filteredVideos = allVideos;
		totalItems = filteredVideos.length;
		totalPages = Math.ceil(totalItems / itemsPerPage);
		updateDisplayedVideos();
	});
</script>

<svelte:head>
	<title>Premium Daily Videos | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access daily video analysis and market commentary from our expert traders." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Premium Daily Videos</h2>
			<p></p>
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{(currentPage - 1) * itemsPerPage + 1}-{Math.min(currentPage * itemsPerPage, totalItems)} of {totalItems}</span>
				</div>
				<div class="dashboard-filters__search">
					<input 
						type="text" 
						class="facetwp-autocomplete" 
						placeholder="Search" 
						value={searchQuery}
						oninput={handleSearch}
						aria-label="Search premium daily videos"
						autocomplete="off"
					/>
					<button 
						type="button" 
						class="facetwp-autocomplete-update"
						aria-label="Search"
					>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="11" cy="11" r="8"></circle>
							<path d="m21 21-4.35-4.35"></path>
						</svg>
					</button>
				</div>
			</div>
			<div id="products-list" class="facetwp-template">

				<div class="card-grid flex-grid row">
			{#each displayedVideos as video (video.id)}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a 
								href="/daily/day-trading-room/{video.slug}" 
								class="card-image" 
								style="background-image: url({video.thumbnail});"
							>
								<img 
									class="default-background" 
									width="325" 
									height="183" 
									alt={video.title}
									loading="lazy"
								/>
							</a>
						</figure>
						
						<section class="card-body">
							<h4 class="h5 card-title">
								<a href="/daily/day-trading-room/{video.slug}">
									{video.title}
								</a>
							</h4>
							<span class="article-card__meta">
								<small>{video.date} with {video.trader}</small>
							</span>
							<br>
							<div class="card-description">
								<div class="u--hide-read-more u--squash">
									<p>{video.excerpt}</p>
								</div>
							</div>
						</section>
						
						<footer class="card-footer">
							<a class="btn btn-tiny btn-default" href="/daily/day-trading-room/{video.slug}">
								Watch Now
							</a>
						</footer>
					</div>
				</article>
			{/each}
				</div>
				<div class="facetwp-pagination">
					<div class="facetwp-pager">
						{#each getPaginationRange() as pageNum}
							{#if pageNum === currentPage}
								<span class="facetwp-page active" data-page="{pageNum}" aria-current="page">{pageNum}</span>
							{:else if pageNum === totalPages && pageNum !== currentPage}
								<a 
									href="#page-{pageNum}"
									class="facetwp-page last-page" 
									data-page="{pageNum}"
									onclick={(e) => { e.preventDefault(); goToPage(Number(pageNum)); }}
									aria-label="Go to last page {pageNum}"
								>
									<span class="fa fa-angle-double-right"></span>
								</a>
							{:else if pageNum !== '...'}
								<a 
									href="#page-{pageNum}"
									class="facetwp-page" 
									data-page="{pageNum}"
									onclick={(e) => { e.preventDefault(); goToPage(Number(pageNum)); }}
									aria-label="Go to page {pageNum}"
								>
									{pageNum}
								</a>
							{/if}
						{/each}
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* Dashboard Filters */
	.dashboard-filters {
		margin-bottom: 30px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 4px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 20px;
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: #666;
		display: flex;
		align-items: center;
		gap: 5px;
		flex-shrink: 0;
	}

	.dashboard-filters__search {
		flex: 0 0 auto;
		min-width: 250px;
		max-width: 320px;
		display: flex;
		gap: 0;
	}
	
	.facetwp-autocomplete {
		flex: 1;
		padding: 8px 12px;
		font-size: 13px;
		border: 1px solid #ddd;
		border-radius: 4px 0 0 4px;
		background: #fff;
		color: #333;
		font-family: 'Montserrat', sans-serif;
		transition: border-color 0.2s ease;
	}
	
	.facetwp-autocomplete:focus {
		outline: none;
		border-color: #F69532;
		box-shadow: 0 0 0 2px rgba(246, 149, 50, 0.1);
	}
	
	.facetwp-autocomplete::placeholder {
		color: #999;
	}

	.facetwp-autocomplete-update {
		padding: 8px 12px;
		font-size: 14px;
		border: 1px solid #ddd;
		border-left: none;
		border-radius: 0 4px 4px 0;
		background: #F69532;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Montserrat', sans-serif;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.facetwp-autocomplete-update:hover {
		background: #dc7309;
		color: #fff;
	}

	.facetwp-counts {
		font-weight: 600;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px 30px;
	}

	.flex-grid-item {
		display: flex;
	}

	.card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		overflow: hidden;
		margin: 0;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 0;
		height: 0;
		border-style: solid;
		border-width: 20px 0 20px 35px;
		border-color: transparent transparent transparent rgba(255, 255, 255, 0.95);
		pointer-events: none;
		filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
	}

	.card-media--video::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 70px;
		height: 70px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		pointer-events: none;
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
		opacity: 0;
		width: 100%;
		height: 100%;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.card-title {
		margin: 0 0 10px;
		font-size: 18px;
		font-weight: 700;
		line-height: 1.3;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card-title a:hover {
		color: #F69532;
	}

	.article-card__meta {
		display: block;
		margin-bottom: 10px;
	}

	.article-card__meta small {
		font-size: 12px;
		color: #999;
	}

	.card-description {
		margin-top: 10px;
		font-size: 14px;
		color: #666;
		line-height: 1.6;
	}

	.u--hide-read-more p {
		margin: 0;
	}

	.card-footer {
		padding: 15px 20px;
		border-top: 1px solid #e6e6e6;
		text-align: center;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #F69532;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Section Title */
	.section-title {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 20px;
		color: #333;
	}

	/* Pagination */
	.facetwp-pagination {
		padding: 40px 0;
		text-align: center;
	}

	.facetwp-pager {
		display: inline-flex;
		gap: 5px;
		align-items: center;
	}

	.facetwp-page {
		display: inline-block;
		padding: 8px 12px;
		border: 1px solid #e6e6e6;
		background: #fff;
		color: #666;
		text-decoration: none;
		font-size: 14px;
		font-weight: 400;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Montserrat', sans-serif;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
		border-color: #ddd;
	}

	.facetwp-page.active {
		background: #F69532;
		color: #fff;
		border-color: #F69532;
		cursor: default;
		pointer-events: none;
	}

	.facetwp-page.last-page {
		font-size: 16px;
	}

	.facetwp-page.last-page .fa {
		display: inline-block;
	}

	/* Search Button SVG */
	.facetwp-autocomplete-update svg {
		display: block;
	}

	/* Responsive Grid - Mobile First */
	/* WordPress Breakpoints:
	 * xs: 0-575px (mobile)
	 * sm: 576px-767px (small tablets)
	 * md: 768px-991px (tablets)
	 * lg: 992px+ (desktop)
	 */

	/* Mobile First - Default (xs: 0-575px) */
	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	/* Small devices (sm: 576px and up) */
	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Medium devices (md: 768px and up) */
	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Large devices (lg: 900px and up) - Adjusted for 14" laptops */
	@media (min-width: 900px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* Mobile: Stack filters vertically */
	.dashboard-filters {
		flex-direction: column;
		align-items: flex-start;
	}

	.dashboard-filters__search {
		width: 100%;
		max-width: 100%;
	}

	/* Tablet and up: Horizontal filters */
	@media (min-width: 768px) {
		.dashboard-filters {
			flex-direction: row;
			align-items: center;
		}

		.dashboard-filters__search {
			min-width: 250px;
			max-width: 320px;
		}
	}

	/* Mobile: Smaller card grid margins */
	.card-grid {
		margin: 0 -10px;
	}

	.card-grid-spacer {
		padding: 0 10px 20px;
	}

	/* Tablet and up: Larger margins */
	@media (min-width: 768px) {
		.card-grid {
			margin: 0 -15px;
		}

		.card-grid-spacer {
			padding: 0 15px 30px;
		}
	}

	/* Mobile: Smaller card title */
	.card-title {
		font-size: 16px;
	}

	/* Tablet and up: Larger card title */
	@media (min-width: 768px) {
		.card-title {
			font-size: 18px;
		}
	}

	/* Mobile: Wrap pagination */
	.facetwp-pager {
		flex-wrap: wrap;
		justify-content: center;
	}

</style>
