<!--
	URL: /learning-center/[slug]

	Learning Center Video Detail Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Individual video/resource detail page matching WordPress implementation.

	@version 1.0.1
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { sanitizeHtml } from '$lib/utils/sanitize';

	interface LearningResource {
		id: number;
		title: string;
		trader: string;
		excerpt: string;
		slug: string;
		thumbnail: string;
		categories: string[];
		videoUrl?: string;
		content?: string;
	}

	// All resources data (would come from API in production)
	const allResources: LearningResource[] = [
		{
			id: 1,
			title: 'Q3 Market Outlook July 2025',
			trader: 'John Carter',
			excerpt:
				"Using the economic cycle, John Carter will share insights on what's next in the stock market, commodities, Treasury yields, bonds, and more.",
			slug: 'market-outlook-jul2025-john-carter',
			thumbnail:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['trade-setups'],
			videoUrl:
				'https://simpler-options.s3.amazonaws.com/webinars/MasteringTheTrade_JC_08282025.mp4',
			content: `<p>In this session, John walked through key cycle dates and what they mean for your trading plan:</p>
<ul>
<li>Aug 25–26: Timing for a short 1–2 week market peak (a "buy the pullback" opportunity).</li>
<li>Sept 28–Oct 2: Next larger market cycle.</li>
<li>Bitcoin: July's large cycle top is still intact. Targeting $100,000 soon, with $80,000 on tap over the next few months.</li>
<li>Crude Oil (Sept 5–8): Setting up for a multi-month high.</li>
<li>The Bigger Macro Picture: Updated outlook on global liquidity — and how it continues to shape everything.</li>
</ul>`
		},
		{
			id: 2,
			title: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			trader: 'Kody Ashmore',
			excerpt: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			slug: 'kody-ashmore-daily-sessions-results',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			categories: ['methodology'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>In this comprehensive session, Kody Ashmore breaks down his daily trading routine and shares real results from following his methodology.</p>
<ul>
<li>Morning preparation and market analysis</li>
<li>Key indicators and setups to watch for</li>
<li>Live trade examples with entry and exit points</li>
<li>Risk management strategies</li>
<li>Q&A session addressing common questions</li>
</ul>`
		},
		{
			id: 3,
			title: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			trader: 'Chris Brecher',
			excerpt: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			slug: '15-50-trade',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			categories: ['member-webinar', 'trade-management'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Chris Brecher reveals the powerful 15:50 trade strategy that takes advantage of end-of-day buyback activity.</p>
<ul>
<li>Understanding corporate buyback programs</li>
<li>Why the last 10 minutes matter</li>
<li>Setting up the 15:50 trade</li>
<li>Entry timing and position sizing</li>
<li>Historical performance data</li>
</ul>`
		},
		{
			id: 4,
			title: 'How Mike Teeto Builds His Watchlist',
			trader: 'Mike Teeto',
			excerpt: 'How Mike Teeto Builds His Watchlist',
			slug: 'mike-teeto-watchlist',
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			categories: ['member-webinar'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Mike Teeto shares his systematic approach to building a winning watchlist.</p>
<ul>
<li>Screening criteria for stock selection</li>
<li>Technical and fundamental filters</li>
<li>Sector rotation analysis</li>
<li>Organizing your watchlist for maximum efficiency</li>
<li>Weekly maintenance routine</li>
</ul>`
		},
		{
			id: 5,
			title: 'Understanding Market Structure',
			trader: 'Henry Gambell',
			excerpt: 'Learn how to identify key market structure levels and use them in your trading.',
			slug: 'understanding-market-structure',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			categories: ['methodology', 'trade-setups'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Henry Gambell explains the fundamentals of market structure analysis.</p>
<ul>
<li>Identifying support and resistance levels</li>
<li>Trend structure and momentum</li>
<li>Price action patterns</li>
<li>Using structure for entry and exit timing</li>
<li>Real chart examples</li>
</ul>`
		},
		{
			id: 6,
			title: 'Options Trading Fundamentals',
			trader: 'John Carter',
			excerpt: 'Master the basics of options trading with this comprehensive guide.',
			slug: 'options-trading-fundamentals',
			thumbnail:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['methodology'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>John Carter covers everything you need to know to start trading options.</p>
<ul>
<li>Call and put options explained</li>
<li>Understanding the Greeks</li>
<li>Option chain analysis</li>
<li>Basic strategies: covered calls, protective puts</li>
<li>Managing risk in options trading</li>
</ul>`
		},
		{
			id: 7,
			title: 'Using Squeeze Pro Indicator',
			trader: 'John Carter',
			excerpt:
				'Learn how to use the Squeeze Pro indicator to identify high-probability trade setups.',
			slug: 'squeeze-pro-indicator',
			thumbnail:
				'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['indicators'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Master the Squeeze Pro indicator with John Carter's expert guidance.</p>
<ul>
<li>Understanding the squeeze concept</li>
<li>Reading the indicator signals</li>
<li>Combining with other technical tools</li>
<li>Entry and exit strategies</li>
<li>Live chart demonstrations</li>
</ul>`
		},
		{
			id: 8,
			title: 'Risk Management Essentials',
			trader: 'Chris Brecher',
			excerpt: 'Protect your capital with proven risk management strategies.',
			slug: 'risk-management-essentials',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			categories: ['trade-management'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Chris Brecher teaches the critical skill of risk management.</p>
<ul>
<li>Position sizing fundamentals</li>
<li>Setting stop losses effectively</li>
<li>Risk/reward ratios</li>
<li>Portfolio risk management</li>
<li>Psychological aspects of risk</li>
</ul>`
		},
		{
			id: 9,
			title: 'Futures Trading 101',
			trader: 'Kody Ashmore',
			excerpt: 'Get started with futures trading - from basics to advanced strategies.',
			slug: 'futures-trading-101',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			categories: ['methodology', 'trade-setups'],
			videoUrl: 'https://simpler-options.s3.amazonaws.com/webinars/sample-video.mp4',
			content: `<p>Kody Ashmore introduces futures trading for beginners and intermediate traders.</p>
<ul>
<li>What are futures contracts</li>
<li>Popular futures markets (ES, NQ, CL)</li>
<li>Margin and leverage explained</li>
<li>Day trading vs swing trading futures</li>
<li>Platform setup and order types</li>
</ul>`
		}
	];

	let resource = $state<LearningResource | null>(null);
	let loading = $state(true);

	onMount(() => {
		const slug = page.params.slug;
		resource = allResources.find((r) => r.slug === slug) || null;
		loading = false;
	});

	const categories: { [key: string]: string } = {
		methodology: 'Methodology',
		'trade-setups': 'Trade Setups & Strategies',
		'member-webinar': 'Member Webinar',
		'trade-management': 'Trade & Money Management',
		indicators: 'Indicators & Tools'
	};

	function getCategoryLabel(categoryId: string): string {
		return categories[categoryId] || categoryId;
	}
</script>

<svelte:head>
	{#if resource}
		<title>{resource.title} | Learning Center | Revolution Trading Pros</title>
	{/if}
</svelte:head>

<!-- Breadcrumbs -->
<nav class="breadcrumbs" aria-label="Breadcrumb">
	<div class="breadcrumbs-inner">
		<ul>
			<li><a href="/dashboard">Home</a></li>
			<li class="separator">/</li>
			<li><a href="/dashboard/day-trading-room/learning-center">Learning Center</a></li>
			<li class="separator">/</li>
			{#if resource}
				<li class="current">{resource.title}</li>
			{:else}
				<li class="current">Not Found</li>
			{/if}
		</ul>
	</div>
</nav>

<div id="page" class="site">
	<div id="content" class="site-content">
		{#if loading}
			<div class="loading-container">
				<div class="loading-spinner"></div>
				<p>Loading...</p>
			</div>
		{:else if resource}
			<!-- Title Section -->
			<section id="ca-title" class="ca-section cpost-title-section">
				<div class="section-inner">
					<h1 class="cpost-title">{resource.title}</h1>
				</div>
			</section>

			<!-- Main Content Section -->
			<section id="ca-main" class="ca-section cpost-section">
				<div class="section-inner">
					<div class="ca-content-block">
						<!-- Video Player -->
						<div class="video-container">
							<video controls width="100%" poster={resource.thumbnail} preload="metadata">
								<source src={resource.videoUrl} type="video/mp4" />
								<track kind="captions" />
								Your browser does not support HTML5 video.
							</video>
						</div>

						<!-- Category Labels -->
						<div class="resource-meta">
							<div class="resource-categories">
								{#each resource.categories as cat}
									<span class="label label--info">{getCategoryLabel(cat)}</span>
								{/each}
							</div>
							<div class="resource-trader">
								<span>With <strong>{resource.trader}</strong></span>
							</div>
						</div>

						<!-- Content -->
						<div class="resource-content">
							{@html sanitizeHtml(resource.content, 'rich')}
						</div>

						<!-- Back Link -->
						<div class="back-link">
							<a href="/dashboard/day-trading-room/learning-center" class="btn btn-default">
								← Back to Learning Center
							</a>
						</div>
					</div>
				</div>
			</section>
		{:else}
			<!-- Not Found -->
			<section class="ca-section not-found">
				<div class="section-inner">
					<h1>Resource Not Found</h1>
					<p>The learning resource you're looking for could not be found.</p>
					<a href="/dashboard/day-trading-room/learning-center" class="btn btn-default">
						← Back to Learning Center
					</a>
				</div>
			</section>
		{/if}
	</div>
</div>

<style>
	/* Breadcrumbs */
	.breadcrumbs {
		background: #f5f5f5;
		padding: 12px 0;
		font-size: 13px;
		border-bottom: 1px solid #e6e6e6;
	}

	.breadcrumbs-inner {
		max-width: 1160px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 8px;
	}

	.breadcrumbs li {
		color: #666;
	}

	.breadcrumbs li a {
		color: #1e73be;
		text-decoration: none;
	}

	.breadcrumbs li a:hover {
		text-decoration: underline;
	}

	.breadcrumbs .separator {
		color: #999;
	}

	.breadcrumbs .current {
		color: #333;
		font-weight: 600;
	}

	/* Page Layout */
	.site {
		background: #efefef;
	}

	.site-content {
		max-width: 1160px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Title Section */
	.cpost-title-section {
		padding: 30px 0 20px;
	}

	.section-inner {
		background: #fff;
		padding: 30px 40px;
		border-radius: 4px;
	}

	.cpost-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #333;
		margin: 0;
		line-height: 1.2;
	}

	/* Main Content Section */
	.cpost-section {
		padding: 0 0 40px;
	}

	.cpost-section .section-inner {
		margin-top: 20px;
	}

	.ca-content-block {
		margin-top: 20px;
	}

	/* Video Container */
	.video-container {
		margin-bottom: 25px;
		background: #000;
		border-radius: 4px;
		overflow: hidden;
	}

	.video-container video {
		display: block;
		width: 100%;
		max-width: 100%;
	}

	/* Resource Meta */
	.resource-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 15px;
		align-items: center;
		margin-bottom: 25px;
		padding-bottom: 20px;
		border-bottom: 1px solid #e6e6e6;
	}

	.resource-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.label {
		display: inline-block;
		padding: 5px 12px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 3px;
	}

	.label--info {
		background: #e8f4fc;
		color: #0984ae;
	}

	.resource-trader {
		font-size: 14px;
		color: #666;
	}

	.resource-trader strong {
		color: #333;
	}

	/* Content */
	.resource-content {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.resource-content :global(p) {
		margin-bottom: 1em;
	}

	.resource-content :global(ul) {
		margin: 0 0 1.5em 0;
		padding-left: 25px;
	}

	.resource-content :global(li) {
		margin-bottom: 8px;
	}

	/* Back Link */
	.back-link {
		margin-top: 40px;
		padding-top: 30px;
		border-top: 1px solid #e6e6e6;
	}

	.btn {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-default {
		background: #f69532;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Loading */
	.loading-container {
		text-align: center;
		padding: 60px 20px;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f3f3f3;
		border-top: 3px solid #1e73be;
		border-radius: 50%;
		margin: 0 auto 15px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	/* Not Found */
	.not-found {
		padding: 60px 0;
	}

	.not-found h1 {
		font-size: 28px;
		margin-bottom: 15px;
	}

	.not-found p {
		margin-bottom: 25px;
		color: #666;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.cpost-title {
			font-size: 28px;
		}

		.section-inner {
			padding: 20px;
		}

		.breadcrumbs-inner {
			padding: 0 15px;
		}

		.resource-meta {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
