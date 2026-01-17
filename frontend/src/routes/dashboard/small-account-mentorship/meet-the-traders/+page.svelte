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
	<title>Meet the Traders | Small Account Mentorship | Revolution Trading Pros</title>
	<meta name="description" content="Meet our expert traders and learn from their years of experience in the markets." />
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
			Learn from our team of experienced traders. Each trader brings unique expertise and strategies to help you improve your trading.
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
						
						<a href="/dashboard/small-account-mentorship/meet-the-traders/{trader.slug}" class="btn btn-primary">
							View Profile
						</a>
					</div>
				</article>
			{/each}
		</div>
	</div>
</div>

<style>
	.intro-text {
		font-size: 16px;
		color: #666;
		margin-bottom: 30px;
		line-height: 1.6;
	}

	.traders-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 30px;
	}

	.trader-card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.trader-card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.trader-card__image {
		width: 100%;
		height: 200px;
		margin: 0;
		overflow: hidden;
	}

	.trader-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.trader-card__content {
		padding: 20px;
	}

	.trader-card__name {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
	}

	.trader-card__title {
		font-size: 14px;
		color: #0984ae;
		font-weight: 600;
		margin: 0 0 15px;
	}

	.trader-card__specialties {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-bottom: 15px;
	}

	.specialty-tag {
		display: inline-block;
		padding: 4px 10px;
		background: #e8f4fc;
		color: #0984ae;
		font-size: 11px;
		font-weight: 600;
		border-radius: 3px;
		text-transform: uppercase;
	}

	.trader-card__bio {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 20px;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: #F69532;
		color: #fff;
		border: none;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	/* Mobile-first: 1 column by default, 2 on md+, 3 on lg+ */
	.traders-grid {
		grid-template-columns: 1fr;
		gap: 20px;
	}

	@media (min-width: 768px) {
		.traders-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 30px;
		}
	}

	@media (min-width: 1200px) {
		.traders-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
