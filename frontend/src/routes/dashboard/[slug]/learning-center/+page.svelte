<script lang="ts">
	/**
	 * Dynamic Room Learning Center
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 100% PIXEL-PERFECT match to WordPress reference: frontend/Do's/Learning-Center
	 * Now fetches content from the Learning Center API.
	 *
	 * @version 3.1.0 - API Integration
	 */
	import { untrack } from 'svelte';
	import { browser } from '$app/environment';
	import { learningCenterApi, type RoomContent } from '$lib/api/learning-center';

	let { data }: { data: any } = $props();
	const room = $derived(data.room);
	const slug = $derived(data.slug);

	let selectedCategory = $state('0');

	// API state
	let videos = $state<RoomContent[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let apiCategories = $state<Array<{ id: string; label: string; color?: string }>>([]);

	// Fallback categories from WordPress reference (used if API doesn't return categories)
	const defaultCategories = [
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

	// Use API categories if available, otherwise use defaults
	const categories = $derived(apiCategories.length > 0 ? apiCategories : defaultCategories);

	// Category color map for special categories (from reference CSS)
	const categoryColors: Record<string, string> = {
		'2927': '#A3C2F4', // Options Strategies (Level 2 & 3)
		'2928': '#EA9999', // Pricing/Volatility
		'2929': '#FFBE02', // Charting/Indicators/Tools
		'2932': '#B5D7A8', // Trade & Money Management
		'2930': '#BC7831', // Earnings & Options Expiration
		'2931': '#b4a7d6'  // Fibonacci & Options Trading
	};

	// Fetch content from API when slug changes
	$effect(() => {
		const currentSlug = slug;
		if (!browser || !currentSlug) return;

		untrack(() => {
			isLoading = true;
			error = null;
		});

		learningCenterApi.getRoomContent(currentSlug, { limit: 50 })
			.then((response) => {
				if (response.success && response.data.content.length > 0) {
					videos = response.data.content;
					if (response.data.categories.length > 0) {
						apiCategories = response.data.categories;
					}
				} else {
					// Keep empty - will show "no content" message
					videos = [];
				}
				isLoading = false;
			})
			.catch((err) => {
				console.error('Failed to fetch learning center content:', err);
				error = err.message || 'Failed to load content';
				isLoading = false;
			});
	});

	// Filter videos by selected category
	const filteredVideos = $derived(
		selectedCategory === '0'
			? videos
			: videos.filter(v => v.categories.includes(selectedCategory) || v.category_id === selectedCategory)
	);

	function getCategoryColor(catId: string): string {
		// Check API categories first for custom color
		const apiCat = apiCategories.find(c => c.id === catId);
		if (apiCat?.color) return apiCat.color;
		return categoryColors[catId] || '#0984ae';
	}

	function getCategoryLabel(catId: string): string {
		const cat = categories.find(c => c.id === catId);
		return cat?.label || catId;
	}

	function resetFilter() {
		selectedCategory = '0';
	}
</script>

<svelte:head>
	<title>{room.name} Learning Center - Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- FORM TO FILTER BY TERM - Exact match to reference -->
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
			<h2 class="section-title">{room.name} Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
			<div id="response">
				{#if isLoading}
					<div class="loading-state">
						<div class="loading-spinner"></div>
						<p>Loading learning content...</p>
					</div>
				{:else if error}
					<div class="error-state">
						<p>Failed to load content: {error}</p>
						<button type="button" onclick={() => location.reload()}>Try Again</button>
					</div>
				{:else if filteredVideos.length === 0}
					<div class="empty-state">
						<p>No learning content available for this room yet.</p>
					</div>
				{:else}
					<div class="article-cards row flex-grid">
						{#each filteredVideos as video (video.id)}
							<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
								<article class="article-card">
									<figure class="article-card__image" style="background-image: url({video.thumbnail_url || ''});">
										<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="" />
									</figure>
									<div class="article-card__type">
										{#each video.categories as catId (catId)}
											<span
												id={catId}
												class="label label--info"
												style="background-color: {getCategoryColor(catId)}"
											>{getCategoryLabel(catId)}</span>
										{/each}
									</div>
									<h4 class="h5 article-card__title">
										<a href="/dashboard/{slug}/learning-center/{video.slug}">{video.title}</a>
									</h4>
									<div class="u--margin-top-0">
										<span class="trader_name"><i>With {video.instructor?.name || 'Expert Trader'}</i></span>
									</div>
									<div class="article-card__excerpt u--hide-read-more">
										<p>{video.excerpt || video.description}</p>
									</div>
									<a href="/dashboard/{slug}/learning-center/{video.slug}" class="btn btn-tiny btn-default">Watch Now</a>
								</article>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DYNAMIC ROOM LEARNING CENTER - 100% PIXEL-PERFECT MATCH
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

	/* Apply filter button */
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

	/* Section Title */
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
	   ARTICLE CARDS GRID
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

	/* Responsive columns */
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
	   ARTICLE CARD - Pixel-perfect from reference
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		width: 100%;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.article-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
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

	/* Category type labels - OUTSIDE figure */
	.article-card__type {
		display: flex;
		flex-wrap: wrap;
		gap: 5px;
		padding: 15px 15px 0;
		margin: 0 !important;
	}

	/* Label styling */
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

	/* Watch Now Button - Orange style */
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

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #666;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e0e0e0;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #666;
	}

	.error-state button {
		margin-top: 16px;
		padding: 8px 20px;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 14px;
		font-weight: 600;
		transition: background-color 0.2s;
	}

	.error-state button:hover {
		background: #076787;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
		font-size: 16px;
	}
</style>
