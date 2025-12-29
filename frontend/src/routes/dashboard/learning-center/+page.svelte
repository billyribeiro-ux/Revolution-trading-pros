<script lang="ts">
	/**
	 * Learning Center Dashboard Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 100% PIXEL-PERFECT match to WordPress reference: frontend/Do's/Learning-Center
	 *
	 * @version 3.0.0 - Full Reference Compliance
	 */

	let selectedCategory = $state('0');

	// All 32 categories from WordPress reference - exact match
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

	// Category color map for special categories (from reference CSS)
	const categoryColors: Record<string, string> = {
		'2927': '#A3C2F4', // Options Strategies (Level 2 & 3)
		'2928': '#EA9999', // Pricing/Volatility
		'2929': '#FFBE02', // Charting/Indicators/Tools
		'2932': '#B5D7A8', // Trade & Money Management
		'2930': '#BC7831', // Earnings & Options Expiration
		'2931': '#b4a7d6'  // Fibonacci & Options Trading
	};

	// Videos with multi-category support - matching reference structure
	const allVideos = [
		{
			id: 1,
			title: 'Q3 Market Cycles Update with John Carter',
			trader: 'John Carter',
			categories: ['529'],
			categoryLabels: ['Trade Setups & Strategies'],
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'In this session, John walked through key cycle dates and what they mean for your trading plan.',
			videoSlug: 'q3-market-cycles-update-john-carter-08282025'
		},
		{
			id: 2,
			title: 'Q3 Market Outlook July 2025',
			trader: 'John Carter',
			categories: ['529'],
			categoryLabels: ['Trade Setups & Strategies'],
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: "Using the economic cycle, John Carter will share insights on what's next in the stock market, commodities, Treasury yields, bonds, and more.",
			videoSlug: 'market-outlook-jul2025-john-carter'
		},
		{
			id: 3,
			title: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			trader: 'Kody Ashmore',
			categories: ['528'],
			categoryLabels: ['Methodology'],
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			videoSlug: 'kody-ashmore-daily-sessions-results'
		},
		{
			id: 4,
			title: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			trader: 'Chris Brecher',
			categories: ['329', '2932'],
			categoryLabels: ['Member Webinar', 'Trade & Money Management/Trading Plan'],
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			videoSlug: '15-50-trade'
		},
		{
			id: 5,
			title: 'How Mike Teeto Builds His Watchlist',
			trader: 'Mike Teeto',
			categories: ['329'],
			categoryLabels: ['Member Webinar'],
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			description: 'How Mike Teeto Builds His Watchlist',
			videoSlug: 'mike-teeto-watchlist'
		},
		{
			id: 6,
			title: 'Mastering the Trade Room FAQs',
			trader: 'Simpler Trading Team',
			categories: ['329'],
			categoryLabels: ['Member Webinar'],
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			description: 'Mastering the Trade Room FAQs',
			videoSlug: 'mastering-the-trade-room-faqs'
		},
		{
			id: 7,
			title: 'How to Find 3 Reasons to do a Trade',
			trader: 'Chris Brecher',
			categories: ['329', '2932'],
			categoryLabels: ['Member Webinar', 'Trade & Money Management/Trading Plan'],
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'How to Find 3 Reasons to do a Trade',
			videoSlug: '3-reasons-to-do-a-trade'
		},
		{
			id: 8,
			title: 'Analysis of A Trade',
			trader: 'Taylor Horton',
			categories: ['329', '2932'],
			categoryLabels: ['Member Webinar', 'Trade & Money Management/Trading Plan'],
			thumbnail: 'https://cdn.simplertrading.com/2021/06/23152021/MemberWebinar-TaylorH.jpg',
			description: 'Analysis of A Trade',
			videoSlug: 'analysis-of-a-trade'
		},
		{
			id: 9,
			title: 'How to Use the ATR Trailing Stop Intraday',
			trader: 'Chris Brecher',
			categories: ['329', '2929'],
			categoryLabels: ['Member Webinar', 'Charting/Indicators/Tools'],
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'How to Use the ATR Trailing Stop Intraday',
			videoSlug: 'atr-trailing-stop-intraday'
		},
		{
			id: 10,
			title: 'Knowing Where To Look For Tells',
			trader: 'Chris Brecher',
			categories: ['329', '2929'],
			categoryLabels: ['Member Webinar', 'Charting/Indicators/Tools'],
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Knowing Where To Look For Tells',
			videoSlug: 'knowing-where-to-look-for-tells'
		},
		{
			id: 11,
			title: 'John Carter Bigger Picture Outlook 4/14/25',
			trader: 'John Carter',
			categories: ['529', '329'],
			categoryLabels: ['Trade Setups & Strategies', 'Member Webinar'],
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'John Carter Bigger Picture Outlook 4/14/25',
			videoSlug: 'john-carter-bigger-picture-outlook-04142025'
		},
		{
			id: 12,
			title: 'Macroeconomic Recap',
			trader: 'Sam Shames',
			categories: ['527'],
			categoryLabels: ['Fundamentals'],
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2019/07/15174557/MemberWebinar-SamShames.jpg',
			description: 'Macroeconomic Recap',
			videoSlug: 'macroeconomic-recap'
		}
	];

	// Filter videos - supports multi-category matching
	const filteredVideos = $derived(
		selectedCategory === '0'
			? allVideos
			: allVideos.filter(v => v.categories.includes(selectedCategory))
	);

	function getCategoryColor(catId: string): string {
		return categoryColors[catId] || '#0984ae';
	}

	function resetFilter() {
		selectedCategory = '0';
	}
</script>

<svelte:head>
	<title>Learning Center | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- FORM TO FILTER BY TERM - Exact match to reference lines 3072-3086 -->
		<form class="term_filter" id="term_filter">
			<div class="reset_filter">
				<input
					type="radio"
					id="cat-0"
					value="0"
					name="categoryfilter"
					checked={selectedCategory === '0'}
					onchange={resetFilter}
				/>
				<label for="cat-0">
					<svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="undo" class="svg-inline--fa fa-undo fa-w-16" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
						<path fill="currentColor" d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"></path>
					</svg>
				</label>
			</div>
			{#each categories as cat (cat.id)}
				<div class="filter_btn" style={categoryColors[cat.id] ? `--cat-color: ${categoryColors[cat.id]}` : ''}>
					<input
						type="radio"
						id="cat-{cat.id}"
						value={cat.id}
						name="categoryfilter"
						checked={selectedCategory === cat.id}
						onchange={() => selectedCategory = cat.id}
					/>
					<label
						for="cat-{cat.id}"
						class:has-custom-color={!!categoryColors[cat.id]}
					>{cat.label}</label>
				</div>
			{/each}
			<button type="button" class="apply_filter">Apply filter</button>
		</form>

		<section class="dashboard__content-section">
			<h2 class="section-title">Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
			<div id="response">
				<div class="article-cards row flex-grid">
					{#each filteredVideos as video (video.id)}
						<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
							<article class="article-card">
								<figure class="article-card__image" style="background-image: url({video.thumbnail});">
									<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="" />
								</figure>
								<div class="article-card__type">
									{#each video.categoryLabels as label, i (i)}
										<span
											id={video.categories[i]}
											class="label label--info"
											style="background-color: {getCategoryColor(video.categories[i])}"
										>{label}</span>
									{/each}
								</div>
								<h4 class="h5 article-card__title">
									<a href="/dashboard/learning-center/{video.videoSlug}">{video.title}</a>
								</h4>
								<div class="u--margin-top-0">
									<span class="trader_name"><i>With {video.trader}</i></span>
								</div>
								<div class="article-card__excerpt u--hide-read-more">
									<p>{video.description}</p>
								</div>
								<a href="/dashboard/learning-center/{video.videoSlug}" class="btn btn-tiny btn-default">Watch Now</a>
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
	   LEARNING CENTER - 100% PIXEL-PERFECT MATCH TO WORDPRESS REFERENCE
	   Reference: frontend/Do's/Learning-Center
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Content Layout */
	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		min-width: 100%;
		background-color: #efefef;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FILTER FORM - Exact match to reference
	   ═══════════════════════════════════════════════════════════════════════════ */
	.term_filter {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 0;
		margin-bottom: 0;
		align-items: center;
	}

	.reset_filter,
	.filter_btn {
		display: inline-block;
	}

	.reset_filter input,
	.filter_btn input {
		display: none;
	}

	/* Reset button - icon only */
	.reset_filter label {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.reset_filter label svg {
		width: 14px;
		height: 14px;
		color: #666;
	}

	.reset_filter label:hover {
		border-color: #0984ae;
	}

	.reset_filter label:hover svg {
		color: #0984ae;
	}

	.reset_filter input:checked + label {
		background: #0984ae;
		border-color: #0984ae;
	}

	.reset_filter input:checked + label svg {
		color: #fff;
	}

	/* Filter buttons */
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
		white-space: nowrap;
	}

	.filter_btn label:hover {
		border-color: #0984ae;
		color: #0984ae;
	}

	.filter_btn input:checked + label {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}

	/* Custom color category buttons */
	.filter_btn label.has-custom-color {
		border-color: var(--cat-color, #ddd);
	}

	.filter_btn input:checked + label.has-custom-color {
		background: var(--cat-color, #0984ae);
		border-color: var(--cat-color, #0984ae);
	}

	/* Apply filter button - from reference line 3084 */
	.apply_filter {
		display: inline-block;
		padding: 8px 16px;
		font-size: 12px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		color: #fff;
		background: #0984ae;
		border: 1px solid #0984ae;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		margin-left: auto;
	}

	.apply_filter:hover {
		background: #077a9e;
		border-color: #077a9e;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-section {
		padding: 20px;
		background: #fff;
	}

	@media (min-width: 768px) {
		.dashboard__content-section {
			padding: 30px 40px;
		}
	}

	/* Section Title - Exact from reference line 3088 */
	.section-title {
		font-size: 24px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		margin: 0 0 20px;
		line-height: 1.4;
	}

	.section-title span {
		font-weight: 400;
		color: #666;
	}

	/* Hide empty paragraphs */
	p:empty {
		display: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARDS GRID - Exact from reference lines 3091-3114
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.article-cards.row {
		margin-left: -15px;
		margin-right: -15px;
	}

	.flex-grid-item {
		padding: 0 15px;
		margin-bottom: 30px;
		display: flex;
	}

	/* Responsive columns - exact from reference */
	.col-xs-12 {
		width: 100%;
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333333%;
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - Pixel-perfect from reference lines 3095-3114
	   Reference CSS: lines 1334-1358
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		width: 100%;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	/* Article card image - 16:9 aspect ratio */
	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
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

	/* Category type labels - OUTSIDE figure, with margin-top per reference */
	.article-card__type {
		display: flex;
		flex-wrap: wrap;
		gap: 5px;
		padding: 15px 15px 0;
		margin: 0 !important;
	}

	/* Label styling - from reference lines 1334-1341 */
	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 10px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		border-radius: 25px;
		line-height: 1.2;
	}

	.label--info {
		background-color: #0984ae;
		color: #fff;
	}

	/* Article title */
	.h5.article-card__title {
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		margin: 0;
		padding: 10px 15px 5px;
		line-height: 1.3;
	}

	.article-card__title a {
		color: inherit;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	/* Trader name */
	.u--margin-top-0 {
		margin-top: 0 !important;
		padding: 0 15px;
	}

	.trader_name {
		font-size: 13px;
		color: #999;
		font-family: 'Open Sans', sans-serif;
	}

	.trader_name i {
		font-style: italic;
	}

	/* Excerpt */
	.article-card__excerpt {
		padding: 10px 15px;
		flex: 1;
	}

	.article-card__excerpt p {
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		color: #666;
		line-height: 1.5;
		margin: 0;
	}

	/* Watch Now Button - Orange style from reference lines 1343-1354 */
	.btn {
		display: inline-block;
		font-family: 'Open Sans', sans-serif;
		font-weight: 700;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		text-align: center;
	}

	.btn-tiny {
		padding: 5px 10px;
		font-size: 11px;
		line-height: 1.5;
		border-radius: 3px;
	}

	.article-card .btn.btn-tiny.btn-default {
		background: transparent;
		color: #F3911B;
		padding-left: 0;
		font-size: 17px;
		border: none;
		margin: 0 15px 15px;
	}

	.article-card .btn.btn-tiny.btn-default:hover {
		color: #F3911B;
		background: #e7e7e7;
		padding-left: 8px;
	}

	/* Response container */
	#response {
		margin-top: 0;
	}
</style>
