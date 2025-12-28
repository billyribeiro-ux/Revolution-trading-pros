<script lang="ts">
	/**
	 * Premium Daily Videos Dashboard Page
	 * Reference: frontend/Do's/Premium-Daily-Videos
	 */
	let searchQuery = $state('');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	const allVideos = [
		{
			id: 1,
			title: 'Signal & Noise',
			date: 'December 26, 2025',
			trader: 'Sam',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: 'The signal we have been tracking in the Nasdaq continues to build.',
			slug: 'signal-noise'
		},
		{
			id: 2,
			title: 'A Strong End Of Year',
			date: 'December 24, 2025',
			trader: 'TG',
			thumbnail: 'https://cdn.simplertrading.com/2025/09/29170752/MTT-TG.jpg',
			description: 'Christmas week started with the strong turn around at the end of the prior week.',
			slug: 'a-strong-end-of-year'
		},
		{
			id: 3,
			title: "Santa's On His Way",
			date: 'December 23, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: "Santa's on his way based on Tuesday's close.",
			slug: 'santas-on-his-way'
		},
		{
			id: 4,
			title: 'Setting Up for the Santa Rally',
			date: 'December 22, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Everything looks good for a potential rally.',
			slug: 'setting-up-santa-rally'
		},
		{
			id: 5,
			title: 'Holiday Weekend Market Review',
			date: 'December 19, 2025',
			trader: 'Sam',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			description: 'Indexes continue to churn sideways as we approach holiday trade.',
			slug: 'holiday-weekend-market-review'
		},
		{
			id: 6,
			title: 'Ho Ho Whoa!',
			date: 'December 18, 2025',
			trader: 'Bruce Marshall',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			description: 'Market action and outlook for end of year.',
			slug: 'ho-ho-whoa'
		},
		{
			id: 7,
			title: 'A Moment For The VIX',
			date: 'December 17, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'VIX expiration and market weakness analysis.',
			slug: 'a-moment-for-the-vix'
		},
		{
			id: 8,
			title: 'Next Move For SPX, TSLA, and Gold',
			date: 'December 16, 2025',
			trader: 'JC',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134307/SimplerCentral_JC.jpg',
			description: 'Analysis of key markets and assets.',
			slug: 'next-move-spx-tsla-gold'
		},
		{
			id: 9,
			title: 'Triple Threat Setup',
			date: 'December 15, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Setting up for multiple opportunities.',
			slug: 'triple-threat-setup'
		},
		{
			id: 10,
			title: 'Year End Rally Potential',
			date: 'December 12, 2025',
			trader: 'Allison Ostrander',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			description: 'Analyzing year-end rally potential.',
			slug: 'year-end-rally-potential'
		},
		{
			id: 11,
			title: 'Market Momentum Check',
			date: 'December 11, 2025',
			trader: 'Taylor Horton',
			thumbnail: 'https://cdn.simplertrading.com/2021/06/23152021/MemberWebinar-TaylorH.jpg',
			description: 'Checking market momentum indicators.',
			slug: 'market-momentum-check'
		},
		{
			id: 12,
			title: 'Tech Sector Analysis',
			date: 'December 10, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Deep dive into tech sector performance.',
			slug: 'tech-sector-analysis'
		}
	];

	let filteredVideos = $derived(
		searchQuery
			? allVideos.filter(
					(v) =>
						v.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
						v.trader.toLowerCase().includes(searchQuery.toLowerCase())
				)
			: allVideos
	);
	let totalPages = $derived(Math.ceil(filteredVideos.length / itemsPerPage));
	let paginatedVideos = $derived(
		filteredVideos.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
	);
</script>

<svelte:head>
	<title>Premium Daily Videos | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Premium Daily Videos</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Mastering the Trade Premium Daily Videos</h2>
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{filteredVideos.length} results</span>
				</div>
				<div class="dashboard-filters__search">
					<input
						type="text"
						placeholder="Search..."
						bind:value={searchQuery}
						oninput={() => (currentPage = 1)}
					/>
				</div>
			</div>
			<div id="products-list" class="facetwp-template">
				<div class="card-grid flex-grid row">
					{#each paginatedVideos as video (video.id)}
						<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
							<div class="card flex-grid-panel">
								<figure class="card-media card-media--video">
									<a
										href="/daily/mastering-the-trade/{video.slug}"
										class="card-image"
										style="background-image: url({video.thumbnail});"
									>
										<img class="default-background" width="325px" height="183px" alt={video.title} />
									</a>
								</figure>
								<section class="card-body">
									<h4 class="h5 card-title">
										<a href="/daily/mastering-the-trade/{video.slug}">
											{video.title}
										</a>
									</h4>
									<span class="article-card__meta"><small>{video.date} with {video.trader}</small></span>
									<div class="card-description">
										<div class="u--hide-read-more u--squash">
											<p>{video.description}</p>
										</div>
									</div>
								</section>
								<footer class="card-footer">
									<a class="btn btn-tiny btn-default" href="/daily/mastering-the-trade/{video.slug}"
										>Watch Now</a
									>
								</footer>
							</div>
						</article>
					{/each}
				</div>
			</div>
			{#if totalPages > 1}
				<div class="facetwp-pagination">
					<div class="facetwp-pager">
						{#each Array(totalPages) as _, i}
							<button
								class="facetwp-page"
								class:active={currentPage === i + 1}
								onclick={() => (currentPage = i + 1)}>{i + 1}</button
							>
						{/each}
					</div>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		max-width: 100%;
		padding: 20px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
	}

	@media screen and (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		background-color: #efefef;
		min-width: 0;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	.section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.dashboard-filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
		flex-wrap: wrap;
		gap: 15px;
	}

	.dashboard-filters__count {
		color: #666;
		font-size: 14px;
	}

	.dashboard-filters__search input {
		padding: 8px 12px;
		border: 1px solid #ddd;
		border-radius: 4px;
		font-size: 14px;
		min-width: 250px;
	}

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	.card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		height: 100%;
		transition: all 0.2s ease-in-out;
	}

	.card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.card-media {
		position: relative;
		height: 180px;
		overflow: hidden;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='white'%3E%3Cpath d='M8 5v14l11-7z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: center;
		background-size: 30px;
		transition: all 0.2s ease-in-out;
	}

	.card-media--video:hover::after {
		background-color: rgba(246, 149, 50, 0.9);
		transform: translate(-50%, -50%) scale(1.1);
	}

	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
	}

	.card-image img {
		opacity: 0;
	}

	.card-body {
		padding: 15px 20px;
		flex: 1;
	}

	.card-title {
		margin: 0 0 8px;
		font-size: 18px;
		font-weight: 700;
		line-height: 1.3;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.card-title a:hover {
		color: #0984ae;
	}

	.article-card__meta {
		display: block;
		color: #999;
		font-size: 13px;
		margin-bottom: 10px;
	}

	.card-description {
		color: #666;
		font-size: 14px;
		line-height: 1.5;
	}

	.card-description p {
		margin: 0;
	}

	.card-footer {
		padding: 15px 20px;
		border-top: 1px solid #eee;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		text-decoration: none;
		border-radius: 4px;
		font-size: 13px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
	}

	.btn-default:hover {
		background: #e8e8e8;
	}

	.facetwp-pagination {
		margin-top: 30px;
		text-align: center;
	}

	.facetwp-pager {
		display: flex;
		justify-content: center;
		gap: 8px;
	}

	.facetwp-page {
		padding: 8px 14px;
		border: 1px solid #ddd;
		background: #fff;
		border-radius: 4px;
		cursor: pointer;
		font-size: 14px;
		transition: all 0.2s;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
	}

	.facetwp-page.active {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}
</style>
