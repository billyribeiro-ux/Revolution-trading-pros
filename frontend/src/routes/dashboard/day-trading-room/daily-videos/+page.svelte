<script lang="ts">
	/**
	 * Day Trading Room - Premium Daily Videos
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to core 2 reference
	 * URL: /dashboard/mastering-the-trade/daily-videos
	 *
	 * Exact HTML structure from core 2:
	 * - dashboard__content > dashboard__content-main > dashboard__content-section
	 * - section-title
	 * - dashboard-filters with facetwp-counts and facetwp-facet
	 * - facetwp-template > card-grid flex-grid row
	 * - card-grid-spacer flex-grid-item articles
	 * - facetwp-pagination > facetwp-pager > facetwp-page buttons
	 *
	 * @version 3.0.0 - 100% Pixel Perfect
	 */

	// State for search and pagination
	let searchQuery = $state('');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	// Video data matching core 2 reference exactly
	const allVideos = [
		{
			id: 1,
			title: 'A Strong End Of Year',
			date: 'December 24, 2025',
			trader: 'TG',
			thumbnail: 'https://cdn.simplertrading.com/2025/09/29170752/MTT-TG.jpg',
			description: 'Christmas week started with the strong turn around at the end of the prior week, with the mega caps and indexes leading the way. Since the initial pop, there has been some participation drop off as volume has slowed due to traders starting to focus on time off.',
			slug: 'a-strong-end-of-year'
		},
		{
			id: 2,
			title: "Santa's On His Way",
			date: 'December 23, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: "Things can always change, but given how the market closed on Tuesday, it looks like Santa's on his way. Let's look at the facts, then also some preferences and opinions as we get into the end of 2025.",
			slug: 'santas-on-his-way'
		},
		{
			id: 3,
			title: 'Setting Up for the Santa Rally',
			date: 'December 22, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: "Everything looks good for a potential rally, as the indexes are consolidating and breaking higher, along with a lot of key stocks. Let's take a look at TSLA, GOOGL, AMZN, AVGO, MSFT, and more.",
			slug: 'setting-up-santa-rally'
		},
		{
			id: 4,
			title: 'Holiday Weekend Market Review',
			date: 'December 19, 2025',
			trader: 'Sam',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: "Indexes continue to churn sideways as we approach next week's holiday trade. Bulls usually take over in low volume. Can they do it again?",
			slug: 'holiday-weekend-market-review'
		},
		{
			id: 5,
			title: 'Ho Ho Whoa!',
			date: 'December 18, 2025',
			trader: 'Bruce Marshall',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			description: "In this video, Bruce discusses today's market action and the outlook for the end of the year. After CPI this morning, we got a nice bounce and relief rally after a long and messy chop phase since Thanksgiving. We have Quad Witching tomorrow after today's monthly (AM) expiration in the indices and then PCE next Tuesday, so lots more volatility to come. We are clinging onto the 50 SMA on the ES, and if this can hold, we still have a shot at seeing our Santa Rally this year. Bruce will continue to be a cautious Bull until we get more clarity, but don't load the sleigh too heavy just yet, as we don't have clear skies to fly in as of now.",
			slug: 'ho-ho-whoa'
		},
		{
			id: 6,
			title: 'A Moment For The VIX',
			date: 'December 17, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: "Today's action in stocks wasn't just out of the blue. We'd been seeing weakness across the board, but it really came through today after the VIX expiration. That and one other thing really aided in today's break. Let's talk a bit about that and what we may have to look for tomorrow.",
			slug: 'a-moment-for-the-vix'
		},
		{
			id: 7,
			title: 'Next Move For SPX, TSLA, and Gold',
			date: 'December 16, 2025',
			trader: 'JC',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134307/SimplerCentral_JC.jpg',
			description: 'With the indexes consolidating, gold and silver pausing post explosion, crude oil hitting new 52-week lows — does it make sense to put it all in TSLA?',
			slug: 'next-move-spx-tsla-gold'
		},
		{
			id: 8,
			title: "A Stock Picker's Market",
			date: 'December 15, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: "The Mag 7 isn't moving together, which means you have to be more selective as far as which tech stocks you look at. Let's take a look at MSFT, TSLA, AVGO, and more, and Danielle will point out relative strength vs. weakness.",
			slug: 'stock-pickers-market-12152025'
		},
		{
			id: 9,
			title: 'What Happened After FED?',
			date: 'December 12, 2025',
			trader: 'Allison Ostrander',
			thumbnail: 'https://cdn.simplertrading.com/2025/02/07135336/SimplerCentral_AO.jpg',
			description: 'Allison walks through SPX and discusses how the market reacted around the FED announcement going into the end of the Week and levels to be aware of on SPX for both support and resistance.',
			slug: 'what-happened-after-fed'
		},
		{
			id: 10,
			title: 'One Thing Missing For A Bull Run?',
			date: 'December 11, 2025',
			trader: 'Taylor Horton',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07134838/SimplerCentral_TH.jpg',
			description: "Taylor reviews the rankings on the Big 3 Scorecard to review what's bullish, what's bearish, and the pieces missing to kick-off the next big bull-run.",
			slug: 'one-thing-missing-for-a-bull-run'
		},
		{
			id: 11,
			title: 'Processing The Latest Cut',
			date: 'December 10, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: "Today's FOMC meeting brought what most traders were expecting with a 0.25 rate cut. The markets received this generally pretty well, but are down a bit in sympathy with ORCL. Henry looks at some macro things to kick things off, then dives into how he played earnings.",
			slug: 'processing-the-latest-cut'
		},
		{
			id: 12,
			title: 'FOMC and ORCL Earnings ... Do We Explode?',
			date: 'December 09, 2025',
			trader: 'JC',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134307/SimplerCentral_JC.jpg',
			description: "There's an assumption that everything will move higher once the rate cut is announced. But, like marriage, it's not about the announcement, but the first fight (press conference) after the announcement.",
			slug: 'fomc-orcl-earnings'
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

	// Pagination calculations
	let totalPages = $derived(Math.ceil(filteredVideos.length / itemsPerPage));
	let paginatedVideos = $derived(
		filteredVideos.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
	);

	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}

	function handleSearch() {
		currentPage = 1;
	}
</script>

<svelte:head>
	<title>Premium Daily Videos - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from core 2 reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Premium Daily Videos</h2>
			<p></p>
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <div class="facetwp-counts">{filteredVideos.length} results</div>
				</div>
				<div class="dashboard-filters__search">
					<div class="facetwp-facet facetwp-facet-better_search facetwp-type-autocomplete" data-name="better_search" data-type="autocomplete">
						<input
							type="text"
							placeholder="Search..."
							bind:value={searchQuery}
							oninput={handleSearch}
						/>
					</div>
				</div>
			</div>
			<div id="products-list" class="facetwp-template">
				<div class="card-grid flex-grid row">
					{#each paginatedVideos as video (video.id)}
						<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
							<div class="card flex-grid-panel">
								<figure class="card-media card-media--video">
									<a href="/daily/day-trading-room/{video.slug}" class="card-image" style="background-image: url({video.thumbnail});">
										<img class="default-background" width="325px" height="183px" alt={video.title} />
									</a>
								</figure>
								<section class="card-body">
									<h4 class="h5 card-title">
										<a href="/daily/day-trading-room/{video.slug}">
											{video.title}
										</a>
									</h4>
									<span class="article-card__meta"><small>{video.date} with {video.trader}</small></span><br>

									<div class="card-description">
										<div class="u--hide-read-more u--squash"><p>{video.description}</p>
</div>
									</div>
								</section>
								<footer class="card-footer">
									<a class="btn btn-tiny btn-default" href="/daily/day-trading-room/{video.slug}">Watch Now</a>
								</footer>
							</div>
						</article>
					{/each}
				</div>
				{#if totalPages > 1}
					<div class="facetwp-pagination">
						<div class="facetwp-pager">
							{#if currentPage > 1}
								<button class="facetwp-page" onclick={() => goToPage(currentPage - 1)}>&laquo;</button>
							{/if}
							{#each Array(totalPages) as _, i}
								<button
									class="facetwp-page"
									class:active={currentPage === i + 1}
									onclick={() => goToPage(i + 1)}
								>
									{i + 1}
								</button>
							{/each}
							{#if currentPage < totalPages}
								<button class="facetwp-page" onclick={() => goToPage(currentPage + 1)}>&raquo;</button>
							{/if}
						</div>
					</div>
				{/if}
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching core 2 reference exactly
	   All styles use exact class names from WordPress reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* FacetwWP Counts - inline display like WordPress */
	:global(.facetwp-counts) {
		display: inline;
	}

	/* FacetwWP Facet Search Input - exact match */
	:global(.facetwp-facet) input {
		padding: 8px 12px;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		border: 1px solid #ccc;
		border-radius: 4px;
		min-width: 200px;
		transition: border-color 0.15s ease-in-out;
	}

	:global(.facetwp-facet) input:focus {
		border-color: #0984ae;
		outline: none;
	}

	:global(.facetwp-facet) input::placeholder {
		color: #999;
	}

	/* FacetwWP Pagination - exact match */
	:global(.facetwp-pager) {
		display: flex;
		gap: 5px;
	}

	:global(.facetwp-page) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 36px;
		height: 36px;
		padding: 0 10px;
		font-size: 14px;
		font-weight: 500;
		font-family: 'Open Sans', sans-serif;
		color: #666;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		text-decoration: none;
	}

	:global(.facetwp-page:hover) {
		color: #0984ae;
		border-color: #0984ae;
		background-color: #f4f4f4;
	}

	:global(.facetwp-page.active) {
		color: #fff;
		background-color: #0984ae;
		border-color: #0984ae;
	}

	/* Empty paragraph hidden - matches WordPress */
	p:empty {
		display: none;
	}
</style>
