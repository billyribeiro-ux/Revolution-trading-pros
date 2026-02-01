<!--
	URL: /dashboard/day-trading-room/meet-the-traders

	Meet the Traders Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Directory of all traders matching WordPress implementation exactly.

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import TradingRoomDropdown from '$lib/components/dashboard/TradingRoomDropdown.svelte';
	import { traders } from '$lib/data/traders';
</script>

<svelte:head>
	<title>Meet the Traders | Day Trading Room | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Meet our expert traders and learn from their years of experience in the markets."
	/>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Meet the Traders</h1>
	</div>
	<div class="dashboard__header-right">
		<TradingRoomDropdown />
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<p class="intro-text">
			Learn from our team of experienced traders. Each trader brings unique expertise and strategies
			to help you improve your trading.
		</p>

		<div class="traders-grid">
			{#each traders as trader (trader.id)}
				<article class="trader-card">
					<figure class="trader-card__image">
						<img src={trader.image} alt={trader.name} loading="lazy" />
					</figure>

					<div class="trader-card__content">
						<h3 class="trader-card__name">{trader.name}</h3>
						<p class="trader-card__title">{trader.title}</p>

						{#if trader.specialties}
							<div class="trader-card__specialties">
								{#each trader.specialties as specialty}
									<span class="specialty-tag">{specialty}</span>
								{/each}
							</div>
						{/if}

						<p class="trader-card__bio">{trader.whoIs.substring(0, 150)}...</p>

						<a
							href="/dashboard/day-trading-room/meet-the-traders/{trader.slug}"
							class="btn btn-primary"
						>
							View Profile
						</a>
					</div>
				</article>
			{/each}
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * MEET THE TRADERS - 2026 Mobile-First Responsive Design
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch Targets: 44x44px minimum
	 * Safe Areas: env(safe-area-inset-*) for notched devices
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-First Base */
	.intro-text {
		font-size: 15px;
		color: #666;
		margin-bottom: 24px;
		line-height: 1.6;
	}

	/* Mobile-First Grid - 1 column */
	.traders-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 16px;
	}

	.trader-card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 12px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.trader-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.trader-card__image {
		width: 100%;
		height: 180px;
		margin: 0;
		overflow: hidden;
	}

	.trader-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.trader-card__content {
		padding: 16px;
	}

	.trader-card__name {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 4px;
		line-height: 1.3;
	}

	.trader-card__title {
		font-size: 13px;
		color: #0984ae;
		font-weight: 600;
		margin: 0 0 12px;
	}

	.trader-card__specialties {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-bottom: 12px;
	}

	.specialty-tag {
		display: inline-block;
		padding: 4px 8px;
		background: #e8f4fc;
		color: #0984ae;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.trader-card__bio {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 16px;
	}

	/* Button - Touch-Friendly (44px minimum) */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		min-width: 44px;
		padding: 12px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-primary {
		background: #f69532;
		color: #fff;
		border: none;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	.btn-primary:focus-visible {
		outline: 2px solid #f69532;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Progressive Enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.traders-grid {
			gap: 20px;
		}

		.trader-card__content {
			padding: 18px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media (min-width: 640px) {
		.intro-text {
			font-size: 16px;
			margin-bottom: 28px;
		}

		.traders-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 24px;
		}

		.trader-card__image {
			height: 200px;
		}

		.trader-card__name {
			font-size: 20px;
		}

		.trader-card__title {
			font-size: 14px;
			margin-bottom: 15px;
		}

		.specialty-tag {
			font-size: 11px;
			padding: 4px 10px;
		}

		.trader-card__bio {
			margin-bottom: 20px;
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.intro-text {
			margin-bottom: 30px;
		}

		.traders-grid {
			gap: 28px;
		}

		.trader-card__content {
			padding: 20px;
		}
	}

	/* lg: 1024px+ - Desktop */
	@media (min-width: 1024px) {
		.traders-grid {
			grid-template-columns: repeat(3, 1fr);
			gap: 30px;
		}
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.btn,
		.trader-card {
			transition: none;
		}

		.trader-card:hover {
			box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
		}
	}
</style>
