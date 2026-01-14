<!--
	FeaturedCards Component - Reusable Dashboard Section
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Svelte 5 Best Practices - Latest Nov/Dec 2024 Syntax
	
	Displays a grid of featured cards with background images.
	Used across dashboard pages (Weekly Watchlist, etc.)
	
	@version 1.0.0 - January 2026
-->
<script lang="ts">
	interface Card {
		id: number | string;
		title: string;
		description: string;
		href: string;
		bgImage: string;
		cardClass: string;
		titleClass: string;
	}

	interface Props {
		/** Array of card items to display */
		cards: Card[];
		/** Optional CSS class for container */
		className?: string;
		/** Layout mode: '3x1' (default 3 columns) or '2x1' (2 top, 1 centered bottom) */
		layout?: '3x1' | '2x1';
	}

	let { cards = [], className = '', layout = '3x1' }: Props = $props();
</script>

<section class="dashboard__content-section-member {className}">
	<div class="row featured_cards" class:layout-2x1={layout === '2x1'}>
		{#each cards as card, index (card.id)}
			<div 
				class="text-center"
				class:col-md-6={layout === '3x1' || (layout === '2x1' && index < 2)}
				class:col-lg-4={layout === '3x1'}
				class:col-lg-6={layout === '2x1' && index < 2}
				class:col-centered={layout === '2x1' && index === 2}
			>
				<div class={card.cardClass}>
					<h2 class={card.titleClass}>{card.title}</h2>
					<p>{card.description}</p>
					<a href={card.href}>Check It Out</a>
				</div>
			</div>
		{/each}
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * FEATURED CARDS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.featured_cards {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
	}

	/* 2x1 Layout - 2 cards on top, 1 centered on bottom */
	.featured_cards.layout-2x1 {
		justify-content: center;
	}

	.col-centered {
		flex: 0 0 100%;
		max-width: 485px;
		margin: 0 auto;
		display: flex;
		justify-content: center;
	}

	/* Apple ICT 7: Equal width and height for all cards */
	.featured_cards .text-center {
		display: flex;
		margin-bottom: 20px;
		width: 100%;
	}

	/* Tablet+ - fixed equal dimensions */
	@media (min-width: 768px) {
		.featured_cards .text-center {
			width: 485px;
			min-width: 485px;
			max-width: 485px;
		}
	}

	.featured_cards p {
		padding: 0 20px;
		color: #ffffff;
		font-size: 14px;
		line-height: 1.6;
		text-align: center;
		margin: 15px 0;
	}

	.featured_cards a {
		color: #fff !important;
		display: inline-block;
		font-family: "Open Sans Condensed", sans-serif;
		font-weight: 700;
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		border-radius: 50px;
		padding: 5px 30px;
		margin: 0 auto 20px auto;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.featured_cards a:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	/* Card Titles */
	:global(.card_title) {
		background-color: #19416b;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
		text-align: center;
	}

	:global(.card_title_2) {
		background-color: #45770c;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
		text-align: center;
	}

	:global(.card_title_3) {
		background-color: #1f9eab;
		border-radius: 8px 8px 0 0;
		opacity: 0.81;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
		text-align: center;
	}

	/* Card Background Images - Equal dimensions */
	:global(.featured_cards .img_1),
	:global(.featured_cards .img_2),
	:global(.featured_cards .img_3) {
		width: 100%;
		height: 220px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		padding-bottom: 20px;
		text-align: center;
	}

	:global(.featured_cards .img_1) {
		background-image: url('https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png');
	}

	:global(.featured_cards .img_2) {
		background-image: url('https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png');
	}

	:global(.featured_cards .img_3) {
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
	.col-lg-4 {
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

	/* Desktop - 3 columns for 3x1 layout */
	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}

		/* 2x1 layout - 2 columns on desktop */
		.col-lg-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION PADDING - Matches WordPress
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-section-member {
		padding: 30px 20px;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section-member {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section-member {
			padding: 40px;
		}
	}
</style>
