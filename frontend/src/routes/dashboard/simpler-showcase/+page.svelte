<script lang="ts">
	/**
	 * Simpler Showcase Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main dashboard for Simpler Showcase membership
	 *
	 * @version 1.0.0
	 */
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';

	// Featured content
	const featured = [
		{
			id: 1,
			title: 'Weekly Trading Showcase',
			date: 'December 23, 2025',
			description: 'Join us for this week\'s trading showcase where our top traders share their best setups and winning strategies.',
			image: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			href: '/showcase/weekly'
		},
		{
			id: 2,
			title: 'Monthly Winners Circle',
			date: 'December 2025',
			description: 'Celebrating our top performers of the month. See their strategies and learn from the best.',
			image: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
			href: '/showcase/winners'
		}
	];

	// Highlights
	const highlights = [
		{ title: 'Live Trading Sessions', count: '50+', description: 'Monthly live sessions' },
		{ title: 'Expert Traders', count: '12', description: 'Professional mentors' },
		{ title: 'Success Stories', count: '500+', description: 'Member testimonials' },
		{ title: 'Trading Strategies', count: '100+', description: 'Proven methods' }
	];
</script>

<svelte:head>
	<title>Simpler Showcase | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<div class="header-icon">
			<IconTrophy size={28} />
		</div>
		<h1 class="dashboard__page-title">Simpler Showcase</h1>
	</div>
	<div class="dashboard__header-right">
		<a href="/dashboard/simpler-showcase/start-here" class="btn btn-default">New? Start Here</a>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Highlights Grid -->
		<section class="highlights-section">
			<div class="highlights-grid">
				{#each highlights as highlight}
					<div class="highlight-card">
						<span class="highlight-count">{highlight.count}</span>
						<h3 class="highlight-title">{highlight.title}</h3>
						<p class="highlight-desc">{highlight.description}</p>
					</div>
				{/each}
			</div>
		</section>

		<!-- Featured Content -->
		<section class="featured-section">
			<h2 class="section-title">Featured Content</h2>
			<div class="featured-grid">
				{#each featured as item}
					<article class="featured-card">
						<div class="featured-image" style="background-image: url({item.image})">
							<div class="play-overlay">
								<IconPlayerPlay size={48} />
							</div>
						</div>
						<div class="featured-info">
							<span class="featured-date">
								<IconCalendar size={14} />
								{item.date}
							</span>
							<h3 class="featured-title">{item.title}</h3>
							<p class="featured-description">{item.description}</p>
							<a href={item.href} class="watch-btn">Watch Now</a>
						</div>
					</article>
				{/each}
			</div>
		</section>

		<!-- About Section -->
		<section class="about-section">
			<div class="about-content">
				<h2 class="about-title">Welcome to Simpler Showcase</h2>
				<p class="about-text">
					Simpler Showcase brings together the best trading content, strategies, and insights from our community of expert traders.
					Whether you're looking to learn new techniques or get inspiration for your next trade, you'll find it here.
				</p>
				<div class="about-features">
					<div class="feature">
						<strong>Weekly Live Sessions</strong>
						<span>Join our traders live every week</span>
					</div>
					<div class="feature">
						<strong>Strategy Breakdowns</strong>
						<span>Detailed analysis of winning trades</span>
					</div>
					<div class="feature">
						<strong>Community Spotlight</strong>
						<span>Featured member success stories</span>
					</div>
				</div>
			</div>
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
		gap: 20px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 15px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: #000;
		color: #F69532;
		border-radius: 10px;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.btn-default {
		display: inline-block;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		text-decoration: none;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		border: 1px solid #ddd;
		transition: all 0.2s;
	}

	.btn-default:hover {
		background: #e8e8e8;
	}

	.dashboard__content {
		padding: 30px;
		background: #f5f5f5;
	}

	.dashboard__content-main {
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Highlights */
	.highlights-section {
		margin-bottom: 40px;
	}

	.highlights-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 20px;
	}

	.highlight-card {
		background: #fff;
		padding: 24px;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
		text-align: center;
	}

	.highlight-count {
		display: block;
		font-size: 36px;
		font-weight: 700;
		color: #0984ae;
		margin-bottom: 8px;
	}

	.highlight-title {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0 0 4px;
	}

	.highlight-desc {
		font-size: 13px;
		color: #666;
		margin: 0;
	}

	/* Featured Section */
	.featured-section {
		margin-bottom: 40px;
	}

	.section-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.featured-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 24px;
	}

	.featured-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: all 0.2s;
	}

	.featured-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.featured-image {
		position: relative;
		width: 100%;
		height: 200px;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
	}

	.play-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
		color: #fff;
		opacity: 0;
		transition: opacity 0.2s;
		cursor: pointer;
	}

	.featured-card:hover .play-overlay {
		opacity: 1;
	}

	.featured-info {
		padding: 20px;
	}

	.featured-date {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		color: #666;
		margin-bottom: 8px;
	}

	.featured-title {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 8px;
		font-family: 'Open Sans', sans-serif;
	}

	.featured-description {
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		margin: 0 0 16px;
	}

	.watch-btn {
		display: inline-block;
		padding: 10px 20px;
		background: #F69532;
		color: #fff;
		text-decoration: none;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		transition: background 0.2s;
	}

	.watch-btn:hover {
		background: #dc7309;
	}

	/* About Section */
	.about-section {
		background: #fff;
		border-radius: 8px;
		padding: 40px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
	}

	.about-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 16px;
		font-family: 'Open Sans', sans-serif;
	}

	.about-text {
		font-size: 16px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 24px;
		max-width: 800px;
	}

	.about-features {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	.feature {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.feature strong {
		font-size: 15px;
		color: #333;
	}

	.feature span {
		font-size: 14px;
		color: #666;
	}

	@media (max-width: 1024px) {
		.highlights-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.featured-grid {
			grid-template-columns: 1fr;
		}

		.about-features {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.dashboard__header {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard__content {
			padding: 20px;
		}

		.highlights-grid {
			grid-template-columns: 1fr;
		}

		.about-section {
			padding: 24px;
		}
	}
</style>
