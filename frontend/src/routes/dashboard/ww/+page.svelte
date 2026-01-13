<!--
	Weekly Watchlist Dashboard - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Matches frontend/Implementation/wwlDashboard exactly.
	Three featured cards with background images and archive section below.
	
	@version 1.0.0 - January 2026 - WordPress pixel-perfect match
-->
<script lang="ts">
	import { onMount } from 'svelte';

	// Featured cards data matching WordPress
	const featuredCards = [
		{
			id: 1,
			title: 'Get Started',
			description: 'Learn how to best use the Weekly Watchlist and get the most out of it.',
			href: '/dashboard/ww/getting-started/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png',
			cardClass: 'img_1',
			titleClass: 'card_title'
		},
		{
			id: 2,
			title: 'Watchlist Rundown',
			description: 'Video reviewing the Watchlist of the Week by the Trader who created it.',
			href: '/dashboard/ww/watchlist-rundown-archive/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png',
			cardClass: 'img_2',
			titleClass: 'card_title_2'
		},
		{
			id: 3,
			title: 'Weekly Watchlist',
			description: 'Get the complete Watchlist of the Week in a downloadable format.',
			href: '/dashboard/ww/weekly-watchlist-archive/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134813/weekly-watchlist-top-card-bg.png',
			cardClass: 'img_3',
			titleClass: 'card_title_3'
		}
	];

	// Archive items (placeholder - would come from API)
	const archiveItems: any[] = [];
</script>

<svelte:head>
	<title>Weekly Watchlist Dashboard | Revolution Trading Pros</title>
</svelte:head>

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		
		<!-- FEATURED CARDS SECTION -->
		<section class="dashboard__content-section">
			<div class="row featured_cards">
				{#each featuredCards as card (card.id)}
					<div class="col-md-6 col-lg-4 text-center">
						<div class="{card.cardClass}">
							<h2 class="{card.titleClass}">{card.title}</h2>
							<p>{card.description}</p>
							<a href="{card.href}">Check It Out</a>
						</div>
					</div>
				{/each}
			</div>
		</section>

		<!-- WATCHLIST RUNDOWN ARCHIVE SECTION -->
		<div class="dashboard__content-section">
			<section>
				<h2 class="section-title">Watchlist Rundown Archive</h2>
				<div class="article-cards row flex-grid">
					{#if archiveItems.length === 0}
						<div class="col-12">
							<p class="text-muted">Archive items will appear here.</p>
						</div>
					{:else}
						{#each archiveItems as item}
							<!-- Archive cards would go here -->
						{/each}
					{/if}
				</div>
			</section>
		</div>

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * FEATURED CARDS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.featured_cards {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
	}

	.featured_cards .text-center {
		display: flex;
		margin-bottom: 20px;
		min-width: 384px;
		max-width: 485px;
	}

	.featured_cards p {
		padding: 0 20px;
		color: #ffffff;
		font-size: 14px;
		line-height: 1.6;
	}

	.featured_cards a {
		color: #fff !important;
		display: inline-block;
		font-family: "Open Sans Condensed", sans-serif;
		font-weight: 700;
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		border-radius: 50px;
		padding: 5px 30px;
		margin-bottom: 20px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.featured_cards a:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	/* Card Titles */
	.card_title {
		background-color: #19416b;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	.card_title_2 {
		background-color: #45770c;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	.card_title_3 {
		background-color: #1f9eab;
		border-radius: 8px 8px 0 0;
		opacity: 0.81;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	/* Card Background Images */
	.featured_cards .img_1 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png');
	}

	.featured_cards .img_2 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png');
	}

	.featured_cards .img_3 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134813/weekly-watchlist-top-card-bg.png');
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE GRID
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.col-md-6,
	.col-lg-4,
	.col-12 {
		padding-left: 15px;
		padding-right: 15px;
		width: 100%;
	}

	/* Mobile first - full width */
	.text-center {
		text-align: center;
		width: 100%;
	}

	/* Tablet - 2 columns */
	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Desktop - 3 columns */
	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * ARCHIVE SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.article-cards {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	.text-muted {
		color: #666;
		padding: 20px;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT STRUCTURE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-wrap: wrap;
		width: 100%;
	}

	.dashboard__content-main {
		flex: 1 1 100%;
		max-width: 100%;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}
</style>
