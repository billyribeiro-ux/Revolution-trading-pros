<script lang="ts">
	/**
	 * Day Trading Room - Learning Center
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerLearningCenter reference
	 * Uses article-cards structure with filter form
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// State for category filter
	let selectedCategory = $state('0');

	// Categories matching SimplerLearningCenter reference
	const categories = [
		{ id: '529', label: 'Trade Setups & Strategies' },
		{ id: '528', label: 'Methodology' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '2932', label: 'Trade & Money Management/Trading Plan' },
		{ id: '531', label: 'Indicators' },
		{ id: '3260', label: 'Options' },
		{ id: '469', label: 'Foundation' },
		{ id: '527', label: 'Fundamentals' },
		{ id: '522', label: 'Simpler Tech' },
		{ id: '2929', label: 'Charting/Indicators/Tools' },
		{ id: '530', label: 'Charting' },
		{ id: '537', label: 'Psychology' }
	];

	// Learning center videos matching SimplerLearningCenter reference
	const allVideos = [
		{
			id: 1,
			title: 'Q3 Market Cycles Update with John Carter',
			trader: 'John Carter',
			category: '529',
			categoryLabel: 'Trade Setups & Strategies',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'In this session, John walked through key cycle dates and what they mean for your trading plan.',
			slug: 'q3-market-cycles-update-john-carter-08282025'
		},
		{
			id: 2,
			title: 'Q3 Market Outlook July 2025',
			trader: 'John Carter',
			category: '529',
			categoryLabel: 'Trade Setups & Strategies',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: "Using the economic cycle, John Carter will share insights on what's next in the stock market, commodities, Treasury yields, bonds, and more.",
			slug: 'market-outlook-jul2025-john-carter'
		},
		{
			id: 3,
			title: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			trader: 'Kody Ashmore',
			category: '528',
			categoryLabel: 'Methodology',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			slug: 'kody-ashmore-daily-sessions-results'
		},
		{
			id: 4,
			title: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			trader: 'Chris Brecher',
			category: '329',
			categoryLabel: 'Member Webinar',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			slug: '15-50-trade'
		},
		{
			id: 5,
			title: 'How Mike Teeto Builds His Watchlist',
			trader: 'Mike Teeto',
			category: '329',
			categoryLabel: 'Member Webinar',
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			description: 'How Mike Teeto Builds His Watchlist',
			slug: 'mike-teeto-watchlist'
		},
		{
			id: 6,
			title: 'Understanding Market Indicators',
			trader: 'Henry Gambell',
			category: '531',
			categoryLabel: 'Indicators',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'A deep dive into the indicators that move the market and how to use them effectively.',
			slug: 'understanding-market-indicators'
		},
		{
			id: 7,
			title: 'Options Trading Fundamentals',
			trader: 'Danielle Shay',
			category: '3260',
			categoryLabel: 'Options',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Learn the fundamentals of options trading from entry to exit strategies.',
			slug: 'options-trading-fundamentals'
		},
		{
			id: 8,
			title: 'Psychology of Trading',
			trader: 'Bruce Marshall',
			category: '537',
			categoryLabel: 'Psychology',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			description: 'Master your emotions and develop the mental discipline needed for successful trading.',
			slug: 'psychology-of-trading'
		},
		{
			id: 9,
			title: 'Advanced Charting Techniques',
			trader: 'Taylor Horton',
			category: '530',
			categoryLabel: 'Charting',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07134838/SimplerCentral_TH.jpg',
			description: 'Take your charting skills to the next level with advanced pattern recognition.',
			slug: 'advanced-charting-techniques'
		}
	];

	// Filter videos based on selected category
	let filteredVideos = $derived(
		selectedCategory === '0'
			? allVideos
			: allVideos.filter(v => v.category === selectedCategory)
	);

	function handleCategoryChange(categoryId: string) {
		selectedCategory = categoryId;
	}
</script>

<svelte:head>
	<title>Learning Center - Day Trading Room | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerLearningCenter reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main" style="min-width: 100%">

		<!-- FORM TO FILTER BY TERM - Exact from SimplerLearningCenter lines 3066-3080 -->
		<form class="term_filter" id="term_filter">
			<div class="reset_filter">
				<input type="radio" id="0" value="0" name="categoryfilter" checked={selectedCategory === '0'} onchange={() => handleCategoryChange('0')} />
				<label for="0">
					<svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="undo" class="svg-inline--fa fa-undo fa-w-16" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
						<path fill="currentColor" d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"></path>
					</svg>
				</label>
			</div>
			{#each categories as cat}
				<div class="filter_btn">
					<input type="radio" id={cat.id} value={cat.id} name="categoryfilter" checked={selectedCategory === cat.id} onchange={() => handleCategoryChange(cat.id)} />
					<label for={cat.id}>{cat.label}</label>
				</div>
			{/each}
		</form>

		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
			<div id="response">
				<div class="article-cards row flex-grid">
					{#each filteredVideos as video (video.id)}
						<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
							<article class="article-card">
								<figure class="article-card__image" style="background-image: url({video.thumbnail});">
									<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt={video.title} />
								</figure>
								<div class="article-card__type">
									<span id={video.category} class="label label--info">{video.categoryLabel}</span>
								</div>
								<h4 class="h5 article-card__title"><a href="/learning-center/{video.slug}">{video.title}</a></h4>
								<div class="u--margin-top-0">
									<span class="trader_name"><i>With {video.trader}</i></span>
								</div>
								<div class="article-card__excerpt u--hide-read-more">
									<p>{video.description}</p>
								</div>
								<a href="/learning-center/{video.slug}" class="btn btn-tiny btn-default">Watch Now</a>
							</article>
						</div>
					{/each}
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerLearningCenter reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Filter Form - Exact from SimplerLearningCenter */
	.term_filter {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-bottom: 30px;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 8px;
	}

	.reset_filter,
	.filter_btn {
		display: inline-block;
	}

	.reset_filter input,
	.filter_btn input {
		display: none;
	}

	.reset_filter label,
	.filter_btn label {
		display: inline-block;
		padding: 8px 16px;
		font-size: 12px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		color: #666;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.reset_filter label {
		padding: 8px 12px;
	}

	.reset_filter label:hover,
	.filter_btn label:hover {
		border-color: #0984ae;
		color: #0984ae;
	}

	.reset_filter input:checked + label,
	.filter_btn input:checked + label {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}

	.reset_filter svg {
		width: 14px;
		height: 14px;
	}

	/* Section Title - Exact from reference */
	.section-title span {
		color: #999;
		font-weight: 400;
	}

	/* Article Cards Grid - Exact from SimplerLearningCenter lines 3085 */
	:global(.article-cards.row.flex-grid) {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	/* Flex Grid Items - Exact from reference */
	:global(.flex-grid-item) {
		display: flex;
		padding: 0 15px;
		margin-bottom: 30px;
	}

	/* Column sizing - Exact from reference */
	:global(.col-xs-12) {
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		:global(.col-sm-6) {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		:global(.col-md-6) {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		:global(.col-xl-4) {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	/* Article Card - Exact from SimplerLearningCenter */
	:global(.article-card) {
		position: relative;
		width: 100%;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		padding-bottom: 15px;
	}

	:global(.article-card:hover) {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	:global(.article-card__image) {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		margin: 0;
	}

	:global(.article-card__image img) {
		display: none;
	}

	:global(.article-card__type) {
		position: absolute;
		top: 10px;
		left: 10px;
	}

	:global(.article-card__type .label) {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
	}

	:global(.article-card__type .label--info) {
		background-color: #0984ae;
		color: #fff;
	}

	:global(.article-card__title) {
		margin: 0;
		padding: 15px 15px 5px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.3;
		font-family: 'Open Sans', sans-serif;
	}

	:global(.article-card__title a) {
		color: #333;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	:global(.article-card__title a:hover) {
		color: #0984ae;
	}

	:global(.u--margin-top-0) {
		margin-top: 0;
		padding: 0 15px;
	}

	:global(.trader_name) {
		font-size: 13px;
		color: #666;
	}

	:global(.article-card__excerpt) {
		padding: 10px 15px;
		font-size: 14px;
		line-height: 1.5;
		color: #666;
	}

	:global(.article-card__excerpt p) {
		margin: 0;
	}

	:global(.article-card .btn) {
		margin: 0 15px;
	}

	/* Empty paragraph hidden */
	p:empty {
		display: none;
	}
</style>
