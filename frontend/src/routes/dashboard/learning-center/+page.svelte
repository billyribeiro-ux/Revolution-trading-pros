<script lang="ts">
	/**
	 * Learning Center Dashboard Page
	 * Reference: frontend/Do's/Learning-Center
	 */
	let selectedCategory = $state('all');
	let searchQuery = $state('');

	const categories = [
		{ id: 'all', name: 'All' },
		{ id: 'trade-setups', name: 'Trade Setups & Strategies' },
		{ id: 'methodology', name: 'Methodology' },
		{ id: 'member-webinar', name: 'Member Webinar' },
		{ id: 'money-management', name: 'Trade & Money Management' },
		{ id: 'indicators', name: 'Indicators' },
		{ id: 'options', name: 'Options' },
		{ id: 'fundamentals', name: 'Fundamentals' },
		{ id: 'charting', name: 'Charting' },
		{ id: 'psychology', name: 'Psychology' }
	];

	const videos = [
		{
			id: 1,
			title: 'Q3 Market Cycles Update with John Carter',
			category: 'trade-setups',
			categoryName: 'Trade Setups & Strategies',
			trader: 'John Carter',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'In this session, John walked through key cycle dates and what they mean for your trading plan.',
			slug: 'q3-market-cycles-update-john-carter-08282025'
		},
		{
			id: 2,
			title: 'Q3 Market Outlook July 2025',
			category: 'trade-setups',
			categoryName: 'Trade Setups & Strategies',
			trader: 'John Carter',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'Using the economic cycle, John Carter will share insights on what\'s next in the stock market.',
			slug: 'market-outlook-jul2025-john-carter'
		},
		{
			id: 3,
			title: "Intro to Kody Ashmore's Daily Sessions",
			category: 'methodology',
			categoryName: 'Methodology',
			trader: 'Kody Ashmore',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			slug: 'kody-ashmore-daily-sessions-results'
		},
		{
			id: 4,
			title: 'The 15:50 Trade',
			category: 'member-webinar',
			categoryName: 'Member Webinar',
			trader: 'Chris Brecher',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'How Buybacks Matter the Last 10 Minutes Every Day',
			slug: '15-50-trade'
		},
		{
			id: 5,
			title: 'How Mike Teeto Builds His Watchlist',
			category: 'member-webinar',
			categoryName: 'Member Webinar',
			trader: 'Mike Teeto',
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			description: 'Learn how to build an effective trading watchlist.',
			slug: 'mike-teeto-watchlist'
		},
		{
			id: 6,
			title: 'Mastering the Trade Room FAQs',
			category: 'member-webinar',
			categoryName: 'Member Webinar',
			trader: 'Simpler Trading Team',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			description: 'Frequently asked questions about the trading room.',
			slug: 'mastering-the-trade-room-faqs'
		}
	];

	let filteredVideos = $derived(() => {
		let result = videos;
		if (selectedCategory !== 'all') {
			result = result.filter((v) => v.category === selectedCategory);
		}
		if (searchQuery) {
			result = result.filter(
				(v) =>
					v.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
					v.trader.toLowerCase().includes(searchQuery.toLowerCase())
			);
		}
		return result;
	});
</script>

<svelte:head>
	<title>Learning Center | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Learning Center</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filters -->
		<div class="category-filters">
			<button class="reset-filter" onclick={() => (selectedCategory = 'all')} title="Reset">
				<svg viewBox="0 0 512 512" width="16" height="16" fill="currentColor">
					<path d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z" />
				</svg>
			</button>
			{#each categories as cat (cat.id)}
				<button
					class="filter-btn"
					class:active={selectedCategory === cat.id}
					onclick={() => (selectedCategory = cat.id)}
				>
					{cat.name}
				</button>
			{/each}
		</div>

		<section class="dashboard__content-section">
			<h2 class="section-title">Learning Center <span>|</span> <span>Overview</span></h2>

			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{filteredVideos().length} results</span>
				</div>
				<div class="dashboard-filters__search">
					<input type="text" placeholder="Search..." bind:value={searchQuery} />
				</div>
			</div>

			<div class="article-cards row flex-grid">
				{#each filteredVideos() as video (video.id)}
					<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
						<article class="article-card">
							<figure
								class="article-card__image"
								style="background-image: url({video.thumbnail});"
							>
								<img src={video.thumbnail} alt={video.title} />
								<div class="article-card__type">
									<span class="label label--info">{video.categoryName}</span>
								</div>
							</figure>
							<h4 class="h5 article-card__title">
								<a href="/learning-center/{video.slug}">{video.title}</a>
							</h4>
							<div class="u--margin-top-0">
								<span class="trader_name"><i>With {video.trader}</i></span>
							</div>
							<div class="article-card__excerpt u--hide-read-more">
								<p>{video.description}</p>
							</div>
							<a href="/learning-center/{video.slug}" class="btn btn-tiny btn-default"
								>Watch Now</a
							>
						</article>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	@media (min-width: 1280px) {
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
	}

	/* Content */
	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		background-color: #efefef;
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
	}

	.section-title span {
		color: #999;
		font-weight: 400;
	}

	/* Category Filters */
	.category-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		padding: 20px 40px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.reset-filter {
		width: 36px;
		height: 36px;
		border: 1px solid #ddd;
		background: #f5f5f5;
		border-radius: 4px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #666;
		transition: all 0.2s;
	}

	.reset-filter:hover {
		background: #e8e8e8;
		color: #333;
	}

	.filter-btn {
		padding: 8px 16px;
		border: 1px solid #ddd;
		background: #fff;
		border-radius: 4px;
		font-size: 13px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: #f5f5f5;
	}

	.filter-btn.active {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}

	/* Filters */
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
		min-width: 200px;
	}

	/* Grid */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.flex-grid-item {
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

	@media (min-width: 1200px) {
		.col-xl-4 {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - Pixel-perfect match to Simpler Trading reference
	   Reference: dashboard-globals.css lines 931-1021
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		margin-bottom: 30px;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		background-color: #0984ae; /* Fallback color */
		margin: 0;
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

	/* Type label - Absolute positioned overlay on image */
	.article-card__type {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 2;
		margin: 0;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px; /* Pill shape */
	}

	.label--info {
		background: #0984ae;
		color: #fff;
	}

	.article-card__title {
		margin: 0;
		padding: 15px 15px 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.3;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.u--margin-top-0 {
		padding: 0 15px;
	}

	.trader_name {
		color: #999;
		font-size: 12px;
	}

	.trader_name i {
		font-style: italic;
	}

	.article-card__excerpt {
		display: none;
	}

	.article-card .btn {
		margin: auto 15px 15px;
	}

	.btn {
		display: inline-block;
		padding: 5px 10px;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		border-radius: 3px;
		text-decoration: none;
		font-size: 11px;
		font-weight: 700;
		line-height: 1.5;
		transition: all 0.2s;
	}

	/* Watch Now button - Orange style from reference */
	.article-card .btn.btn-tiny.btn-default {
		background: transparent;
		color: #F3911B;
		padding-left: 0;
		font-size: 17px;
		border: none;
	}

	.article-card .btn.btn-tiny.btn-default:hover {
		color: #F3911B;
		background: #e7e7e7;
		padding-left: 8px;
	}

	.btn:hover {
		color: #333;
		background-color: #e6e6e6;
		border-color: #adadad;
	}
</style>
