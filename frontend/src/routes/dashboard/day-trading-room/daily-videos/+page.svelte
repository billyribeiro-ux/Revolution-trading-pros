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

	function updateDisplayedVideos() {
		const startIndex = (currentPage - 1) * itemsPerPage;
		const endIndex = startIndex + itemsPerPage;
		displayedVideos = allVideos.slice(startIndex, endIndex);
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
					Showing <div class="facetwp-counts">{(currentPage - 1) * itemsPerPage + 1}-{Math.min(currentPage * itemsPerPage, totalItems)} of {totalItems}</div>
				</div>
				<div class="dashboard-filters__search">
					<div class="facetwp-facet facetwp-facet-better_search facetwp-type-autocomplete" data-name="better_search" data-type="autocomplete"></div>
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
			<ul class="page-numbers">
				{#if currentPage > 1}
					<li>
						<button 
							class="page-numbers" 
							on:click={() => goToPage(currentPage - 1)}
							type="button"
							aria-label="Previous page"
						>
							&laquo;
						</button>
					</li>
				{/if}
				
				{#each getPaginationRange() as pageNum}
					<li>
						{#if pageNum === '...'}
							<span class="page-numbers dots">…</span>
						{:else if pageNum === currentPage}
							<span class="page-numbers current" aria-current="page">{pageNum}</span>
						{:else}
							<button 
								class="page-numbers" 
								on:click={() => goToPage(Number(pageNum))}
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
							on:click={() => goToPage(currentPage + 1)}
							type="button"
							aria-label="Next page"
						>
							&raquo;
						</button>
					</li>
				{/if}
					</ul>
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
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: #666;
		display: flex;
		align-items: center;
		gap: 5px;
	}

	.dashboard-filters__search {
		margin-top: 15px;
	}

	.facetwp-counts {
		font-weight: 600;
		color: #333;
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
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.7);
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
	}

	.facetwp-pagination ul.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		text-align: center;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 5px;
		flex-wrap: wrap;
	}

	.facetwp-pagination li {
		display: inline-block;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.facetwp-pagination li button.page-numbers,
	.facetwp-pagination li span.page-numbers {
		border: 1px solid #e6e6e6;
		display: inline-block;
		padding: 8px 12px;
		margin: 0;
		min-width: 40px;
		text-align: center;
		text-decoration: none;
		color: #333;
		background: #fff;
		transition: all 0.2s ease;
		cursor: pointer;
		font-family: inherit;
		font-size: inherit;
	}

	.facetwp-pagination li button.page-numbers:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.facetwp-pagination li span.current {
		background: #F69532;
		color: #fff;
		border-color: #F69532;
		font-weight: 600;
	}

	.facetwp-pagination li span.dots {
		border: none;
		background: transparent;
		cursor: default;
	}

	/* Responsive Grid */
	@media (max-width: 991px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (max-width: 767px) {
		.col-sm-6 {
			width: 50%;
		}
		
		.card-title {
			font-size: 16px;
		}
	}

	@media (max-width: 575px) {
		.col-xs-12 {
			width: 100%;
		}
		
		.card-grid {
			margin: 0 -10px;
		}
		
		.card-grid-spacer {
			padding: 0 10px 20px;
		}
	}

	@media (min-width: 1200px) {
		.col-lg-4 {
			width: 33.333%;
		}
	}
</style>
