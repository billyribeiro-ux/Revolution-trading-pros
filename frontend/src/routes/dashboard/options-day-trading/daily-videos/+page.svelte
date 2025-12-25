<script lang="ts">
	/**
	 * Options Day Trading - Premium Daily Videos
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Pixel-perfect match to core 2 reference
	 * Uses card-grid layout with flex-grid-panel structure
	 *
	 * @version 2.0.0
	 */
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconChevronLeft from '@tabler/icons-svelte/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';

	// State for search and pagination
	let searchQuery = $state('');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	// Mock video data
	const allVideos = [
		{
			id: 1,
			title: 'Options Market Analysis & Strategies',
			date: 'December 23, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Today we analyze key options setups for SPY, QQQ, and individual stock opportunities.',
			slug: 'options-market-analysis-strategies'
		},
		{
			id: 2,
			title: 'Options Flow & Unusual Activity',
			date: 'December 22, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Breaking down the unusual options activity and what it means for our trades.',
			slug: 'options-flow-unusual-activity'
		},
		{
			id: 3,
			title: 'Weekly Options Wrap-Up',
			date: 'December 20, 2025',
			trader: 'Sam',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: "Reviewing this week's options plays and setting up for next week.",
			slug: 'weekly-options-wrap-up'
		},
		{
			id: 4,
			title: 'SPY Options Deep Dive',
			date: 'December 19, 2025',
			trader: 'Bruce Marshall',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			description: 'Bruce breaks down SPY options levels and key strikes for the holiday week.',
			slug: 'spy-options-deep-dive'
		},
		{
			id: 5,
			title: 'QQQ Options Setup',
			date: 'December 18, 2025',
			trader: 'JC',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134307/SimplerCentral_JC.jpg',
			description: 'Looking at QQQ options for year-end positioning and momentum plays.',
			slug: 'qqq-options-setup'
		},
		{
			id: 6,
			title: 'NVDA Options Earnings Play',
			date: 'December 17, 2025',
			trader: 'Taylor Horton',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07134838/SimplerCentral_TH.jpg',
			description: 'Taylor reviews NVDA options strategies ahead of potential market moves.',
			slug: 'nvda-options-earnings-play'
		}
	];

	// Filter videos based on search query
	let filteredVideos = $derived(
		searchQuery
			? allVideos.filter(v =>
				v.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				v.trader.toLowerCase().includes(searchQuery.toLowerCase()) ||
				v.description.toLowerCase().includes(searchQuery.toLowerCase())
			)
			: allVideos
	);

	// Pagination
	let totalPages = $derived(Math.ceil(filteredVideos.length / itemsPerPage));
	let paginatedVideos = $derived(
		filteredVideos.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
	);

	// Count display
	let startItem = $derived((currentPage - 1) * itemsPerPage + 1);
	let endItem = $derived(Math.min(currentPage * itemsPerPage, filteredVideos.length));

	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}
</script>

<svelte:head>
	<title>Premium Daily Videos - Options Day Trading | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Options Day Trading Premium Daily Videos</h2>
			<p></p>

			<!-- Dashboard Filters - Exact match to core 2 -->
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{startItem} - {endItem} of {filteredVideos.length}</span>
				</div>
				<div class="dashboard-filters__search">
					<div class="search-input-wrapper">
						<IconSearch size={16} />
						<input
							type="text"
							placeholder="Search videos..."
							bind:value={searchQuery}
							oninput={() => currentPage = 1}
						/>
					</div>
				</div>
			</div>

			<!-- Card Grid - Exact structure from core 2 -->
			<div id="products-list" class="facetwp-template">
				<div class="card-grid flex-grid row">
					{#each paginatedVideos as video (video.id)}
						<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
							<div class="card flex-grid-panel">
								<figure class="card-media card-media--video">
									<a
										href="/daily/options-day-trading/{video.slug}"
										class="card-image"
										style="background-image: url({video.thumbnail});"
									>
										<img
											class="default-background"
											width="325"
											height="183"
											alt={video.title}
										/>
									</a>
								</figure>
								<section class="card-body">
									<h4 class="h5 card-title">
										<a href="/daily/options-day-trading/{video.slug}">
											{video.title}
										</a>
									</h4>
									<span class="article-card__meta"><small>{video.date} with {video.trader}</small></span><br>
									<div class="card-description">
										<div class="u--hide-read-more u--squash">
											<p>{video.description}</p>
										</div>
									</div>
								</section>
								<footer class="card-footer">
									<a class="btn btn-tiny btn-default" href="/daily/options-day-trading/{video.slug}">Watch Now</a>
								</footer>
							</div>
						</article>
					{/each}
				</div>

				<!-- Pagination -->
				{#if totalPages > 1}
					<div class="facetwp-pagination">
						<div class="facetwp-pager">
							<button
								class="pagination-item"
								class:disabled={currentPage === 1}
								onclick={() => goToPage(currentPage - 1)}
								aria-label="Previous page"
							>
								<IconChevronLeft size={16} />
							</button>

							{#each Array(totalPages) as _, i}
								<button
									class="pagination-item"
									class:active={currentPage === i + 1}
									onclick={() => goToPage(i + 1)}
								>
									{i + 1}
								</button>
							{/each}

							<button
								class="pagination-item"
								class:disabled={currentPage === totalPages}
								onclick={() => goToPage(currentPage + 1)}
								aria-label="Next page"
							>
								<IconChevronRight size={16} />
							</button>
						</div>
					</div>
				{/if}
			</div>
		</section>
	</div>
</div>

<style>
	/* Search input wrapper with icon */
	.search-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.search-input-wrapper :global(svg) {
		position: absolute;
		left: 12px;
		color: #999;
		pointer-events: none;
	}

	.search-input-wrapper input {
		padding: 8px 12px 8px 36px;
		font-size: 14px;
		border: 1px solid #ccc;
		border-radius: 4px;
		min-width: 200px;
		transition: border-color 0.15s ease-in-out;
		font-family: 'Open Sans', sans-serif;
	}

	.search-input-wrapper input:focus {
		border-color: #0984ae;
		outline: none;
	}

	.search-input-wrapper input::placeholder {
		color: #999;
	}

	/* Pagination button styles */
	.pagination-item {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 36px;
		padding: 0 10px;
		font-size: 14px;
		font-weight: 500;
		color: #666;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		font-family: 'Open Sans', sans-serif;
	}

	.pagination-item:hover:not(.disabled) {
		color: #0984ae;
		border-color: #0984ae;
		background-color: #f4f4f4;
	}

	.pagination-item.active {
		color: #fff;
		background-color: #0984ae;
		border-color: #0984ae;
	}

	.pagination-item.disabled {
		color: #ccc;
		cursor: not-allowed;
		pointer-events: none;
	}

	/* Override facetwp-pager for flex display */
	.facetwp-pager {
		display: flex;
		gap: 5px;
	}

	/* Empty paragraph fix */
	p:empty {
		display: none;
	}
</style>
