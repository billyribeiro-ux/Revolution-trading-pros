<script lang="ts">
	/**
	 * Options Day Trading Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main dashboard for Options Day Trading membership
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';

	// Article data - matches WordPress structure
	const articles = [
		{
			id: 1,
			type: 'Daily Video',
			title: 'Options Market Analysis & Strategies',
			date: 'December 23, 2025',
			excerpt: 'Today we analyze key options setups and discuss strategies for the end of year trading. Let\'s look at SPY, QQQ, and individual stock opportunities.',
			href: '/daily/options-day-trading/market-analysis',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			isVideo: true
		},
		{
			id: 2,
			type: 'Chatroom Archive',
			title: 'December 23, 2025',
			date: 'December 23, 2025',
			excerpt: 'With Expert Trader',
			href: '/chatroom-archive/options-day-trading/12232025',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: false
		},
		{
			id: 3,
			type: 'Daily Video',
			title: 'Options Flow & Unusual Activity',
			date: 'December 22, 2025',
			excerpt: 'We\'re seeing some interesting options flow heading into the holidays. Let\'s break down the unusual activity and what it means for our trades.',
			href: '/daily/options-day-trading/options-flow',
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			isVideo: true
		}
	];

	let currentSlide = $state(0);
	let isLoading = $state(true);

	onMount(() => {
		isLoading = false;
	});

	function nextSlide() {
		currentSlide = (currentSlide + 1) % articles.length;
	}

	function prevSlide() {
		currentSlide = (currentSlide - 1 + articles.length) % articles.length;
	}
</script>

<svelte:head>
	<title>Options Day Trading | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Options Day Trading Dashboard</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Featured Carousel -->
		<section class="featured-carousel">
			<div class="carousel-container">
				{#each articles as article, index}
					<div
						class="carousel-slide"
						class:active={index === currentSlide}
						style="transform: translateX({(index - currentSlide) * 100}%)"
					>
						<div class="slide-image" style="background-image: url({article.image})">
							{#if article.isVideo}
								<div class="play-button">
									<svg width="60" height="60" viewBox="0 0 60 60" fill="none">
										<circle cx="30" cy="30" r="30" fill="rgba(0,0,0,0.5)"/>
										<path d="M24 18L44 30L24 42V18Z" fill="white"/>
									</svg>
								</div>
							{/if}
						</div>
						<div class="slide-content">
							<span class="slide-type">{article.type}</span>
							<h2 class="slide-title">{article.title}</h2>
							<p class="slide-date">{article.date}</p>
							<p class="slide-excerpt">{article.excerpt}</p>
							<a href={article.href} class="slide-link">
								{article.isVideo ? 'Watch Now' : 'View Archive'} →
							</a>
						</div>
					</div>
				{/each}
			</div>
			<button class="carousel-nav prev" onclick={prevSlide}>‹</button>
			<button class="carousel-nav next" onclick={nextSlide}>›</button>
			<div class="carousel-dots">
				{#each articles as _, index}
					<button
						class="dot"
						class:active={index === currentSlide}
						onclick={() => currentSlide = index}
					></button>
				{/each}
			</div>
		</section>

		<!-- Quick Links -->
		<section class="quick-links">
			<a href="/dashboard/options-day-trading/start-here" class="quick-link-card">
				<h3>Start Here</h3>
				<p>New to Options Day Trading? Start here for orientation and setup guides.</p>
			</a>
			<a href="/dashboard/options-day-trading/learning-center" class="quick-link-card">
				<h3>Learning Center</h3>
				<p>Access courses, tutorials, and educational content.</p>
			</a>
			<a href="/dashboard/options-day-trading/resources" class="quick-link-card">
				<h3>Resources</h3>
				<p>Trading tools, cheat sheets, and reference materials.</p>
			</a>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		padding: 30px;
		background: #f5f5f5;
		min-height: calc(100vh - 200px);
	}

	.featured-carousel {
		position: relative;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
		overflow: hidden;
		margin-bottom: 30px;
	}

	.carousel-container {
		position: relative;
		height: 400px;
		overflow: hidden;
	}

	.carousel-slide {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		transition: transform 0.5s ease-in-out;
		opacity: 0;
	}

	.carousel-slide.active {
		opacity: 1;
	}

	.slide-image {
		flex: 0 0 50%;
		background-size: cover;
		background-position: center;
		position: relative;
	}

	.play-button {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		cursor: pointer;
		transition: transform 0.2s;
	}

	.play-button:hover {
		transform: translate(-50%, -50%) scale(1.1);
	}

	.slide-content {
		flex: 0 0 50%;
		padding: 40px;
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	.slide-type {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		color: #0984ae;
		margin-bottom: 10px;
	}

	.slide-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.slide-date {
		font-size: 14px;
		color: #666;
		margin-bottom: 15px;
	}

	.slide-excerpt {
		font-size: 16px;
		color: #555;
		line-height: 1.6;
		margin-bottom: 20px;
	}

	.slide-link {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.slide-link:hover {
		text-decoration: underline;
	}

	.carousel-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		background: rgba(0,0,0,0.5);
		color: #fff;
		border: none;
		width: 50px;
		height: 50px;
		font-size: 30px;
		cursor: pointer;
		z-index: 10;
		transition: background 0.2s;
	}

	.carousel-nav:hover {
		background: rgba(0,0,0,0.7);
	}

	.carousel-nav.prev {
		left: 10px;
	}

	.carousel-nav.next {
		right: 10px;
	}

	.carousel-dots {
		position: absolute;
		bottom: 20px;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		gap: 10px;
	}

	.dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: rgba(255,255,255,0.5);
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.dot.active {
		background: #0984ae;
	}

	.quick-links {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
	}

	.quick-link-card {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
		text-decoration: none;
		transition: box-shadow 0.2s, transform 0.2s;
	}

	.quick-link-card:hover {
		box-shadow: 0 4px 12px rgba(0,0,0,0.15);
		transform: translateY(-2px);
	}

	.quick-link-card h3 {
		font-size: 18px;
		font-weight: 700;
		color: #0984ae;
		margin: 0 0 10px;
	}

	.quick-link-card p {
		font-size: 14px;
		color: #666;
		margin: 0;
		line-height: 1.5;
	}

	@media (max-width: 1024px) {
		.carousel-container {
			height: auto;
		}

		.carousel-slide {
			flex-direction: column;
			position: relative;
		}

		.carousel-slide:not(.active) {
			display: none;
		}

		.slide-image {
			flex: none;
			height: 250px;
		}

		.slide-content {
			flex: none;
			padding: 20px;
		}

		.quick-links {
			grid-template-columns: 1fr;
		}
	}
</style>
