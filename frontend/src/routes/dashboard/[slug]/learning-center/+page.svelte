<script lang="ts">
	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	let selectedCategory = $state('0');

	// All categories from reference - exact match
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

	// Sample videos - matching reference format exactly
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
			description: 'Using the economic cycle, John Carter will share insights on what\'s next in the stock market, commodities, Treasury yields, bonds, and more.',
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
			title: 'Understanding Market Indicators',
			trader: 'Henry Gambell',
			categories: ['531'],
			categoryLabels: ['Indicators'],
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			description: 'Deep dive into market indicators and how to use them effectively.',
			videoSlug: 'market-indicators'
		},
		{
			id: 9,
			title: 'Options Trading Fundamentals',
			trader: 'Danielle Shay',
			categories: ['3260'],
			categoryLabels: ['Options'],
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			description: 'Learn options trading fundamentals from the ground up.',
			videoSlug: 'options-fundamentals'
		}
	];

	let filteredVideos = $derived(
		selectedCategory === '0'
			? allVideos
			: allVideos.filter(v => v.categories.includes(selectedCategory))
	);

	function resetFilter() {
		selectedCategory = '0';
	}
</script>

<svelte:head>
	<title>{room.name} Learning Center - Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main" style="min-width: 100%">

		<!-- FORM TO FILTER BY TERM - Exact match to reference -->
		<form class="term_filter" id="term_filter">
			<div class="reset_filter">
				<input type="radio" id="cat-0" value="0" name="categoryfilter" checked={selectedCategory === '0'} onchange={resetFilter} />
				<label for="cat-0">
					<svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="undo" class="svg-inline--fa fa-undo fa-w-16" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
						<path fill="currentColor" d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"></path>
					</svg>
				</label>
			</div>
			{#each categories as cat (cat.id)}
				<div class="filter_btn">
					<input
						type="radio"
						id="cat-{cat.id}"
						value={cat.id}
						name="categoryfilter"
						checked={selectedCategory === cat.id}
						onchange={() => selectedCategory = cat.id}
					/>
					<label for="cat-{cat.id}">{cat.label}</label>
				</div>
			{/each}
		</form>

		<section class="dashboard__content-section">
			<h2 class="section-title">{room.name} Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
			<div id="response">
				<div class="article-cards row flex-grid">
					{#each filteredVideos as video (video.id)}
						<!-- Weekly Watchlist Traders Images -->
						<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
							<article class="article-card">
								<!-- DAILY VIDEOS -->
								<figure class="article-card__image" style="background-image: url({video.thumbnail});">
									<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="" />
								</figure>
								<div class="article-card__type">
									{#each video.categoryLabels as label, i (i)}
										<span id={video.categories[i]} class="label label--info">{label}</span>
									{/each}
								</div>
								<h4 class="h5 article-card__title">
									<a href="/learning-center/{video.videoSlug}">{video.title}</a>
								</h4>
								<div class="u--margin-top-0">
									<span class="trader_name"><i>With {video.trader}</i></span>
								</div>
								<div class="article-card__excerpt u--hide-read-more">
									<p>{video.description}</p>
								</div>
								<a href="/learning-center/{video.videoSlug}" class="btn btn-tiny btn-default">Watch Now</a>
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
	   CATEGORY FILTER - Exact match to Simpler Trading reference
	   ═══════════════════════════════════════════════════════════════════════════ */
	.term_filter {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 0;
		margin-bottom: 20px;
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

	.reset_filter input:checked + label {
		background: #0984ae;
		border-color: #0984ae;
	}

	.reset_filter input:checked + label svg {
		color: #fff;
	}

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

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECTION TITLE - Exact match to reference
	   ═══════════════════════════════════════════════════════════════════════════ */
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARDS GRID - Exact match to Simpler Trading reference
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
	}

	/* Responsive columns */
	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - Exact match to Simpler Trading reference
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.article-card__image {
		position: relative;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		background-repeat: no-repeat;
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

	.article-card__type {
		padding: 15px 15px 0;
		display: flex;
		flex-wrap: wrap;
		gap: 5px;
	}

	.label {
		display: inline-block;
		padding: 4px 8px;
		font-size: 10px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		border-radius: 3px;
		line-height: 1.2;
	}

	.label--info {
		background: #d1ecf1;
		color: #0c5460;
	}

	.h5.article-card__title {
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #333;
		margin: 10px 15px 0;
		line-height: 1.4;
	}

	.article-card__title a {
		color: inherit;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.u--margin-top-0 {
		margin-top: 0 !important;
		padding: 5px 15px 0;
	}

	.trader_name {
		font-size: 13px;
		color: #666;
		font-family: 'Open Sans', sans-serif;
	}

	.trader_name i {
		font-style: italic;
	}

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

	.u--hide-read-more :global(.read-more) {
		display: none;
	}

	/* Watch Now Button - Exact match */
	.btn {
		display: inline-block;
		font-family: 'Open Sans', sans-serif;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		text-align: center;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 12px;
		margin: 0 15px 15px;
	}

	.btn-default {
		background: #f4f4f4;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e9e9e9;
		color: #333;
	}

	/* Dashboard Content Section */
	.dashboard__content-section {
		padding: 20px;
	}

	#response {
		margin-top: 0;
	}
</style>
