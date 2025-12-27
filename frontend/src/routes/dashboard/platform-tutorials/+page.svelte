<script lang="ts">
	/**
	 * Platform Tutorials Dashboard Page
	 * Reference: frontend/Do's/Platform-Tutorials
	 */
	let selectedPlatform = $state('all');

	const platforms = [
		{ id: 'all', name: 'All Platforms' },
		{ id: 'thinkorswim', name: 'ThinkorSwim' },
		{ id: 'tradestation', name: 'TradeStation' },
		{ id: 'tradingview', name: 'TradingView' },
		{ id: 'tastyworks', name: 'Tastyworks' },
		{ id: 'webull', name: 'Webull' }
	];

	const tutorials = [
		{
			id: 1,
			title: 'ThinkorSwim Setup Guide',
			platform: 'thinkorswim',
			platformName: 'ThinkorSwim',
			thumbnail: 'https://cdn.simplertrading.com/2021/06/23152021/MemberWebinar-TaylorH.jpg',
			description: 'Complete setup guide for ThinkorSwim platform.',
			duration: '45 min',
			slug: 'thinkorswim-setup-guide'
		},
		{
			id: 2,
			title: 'TradeStation Chart Configuration',
			platform: 'tradestation',
			platformName: 'TradeStation',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27120614/MemberWebinar-Generic1.jpg',
			description: 'How to configure charts in TradeStation.',
			duration: '30 min',
			slug: 'tradestation-chart-config'
		},
		{
			id: 3,
			title: 'Installing Simpler Indicators on TOS',
			platform: 'thinkorswim',
			platformName: 'ThinkorSwim',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			description: 'Step-by-step indicator installation guide.',
			duration: '20 min',
			slug: 'installing-indicators-tos'
		},
		{
			id: 4,
			title: 'TradingView Basics',
			platform: 'tradingview',
			platformName: 'TradingView',
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			description: 'Getting started with TradingView charting.',
			duration: '25 min',
			slug: 'tradingview-basics'
		},
		{
			id: 5,
			title: 'Setting Up Alerts in ThinkorSwim',
			platform: 'thinkorswim',
			platformName: 'ThinkorSwim',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			description: 'Create and manage alerts in TOS.',
			duration: '15 min',
			slug: 'alerts-thinkorswim'
		},
		{
			id: 6,
			title: 'Tastyworks Order Entry',
			platform: 'tastyworks',
			platformName: 'Tastyworks',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			description: 'Mastering order entry in Tastyworks.',
			duration: '35 min',
			slug: 'tastyworks-order-entry'
		}
	];

	let filteredTutorials = $derived(() => {
		if (selectedPlatform === 'all') return tutorials;
		return tutorials.filter((t) => t.platform === selectedPlatform);
	});
</script>

<svelte:head>
	<title>Platform Tutorials | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Platform Tutorials</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Platform Filters -->
		<div class="platform-filters">
			{#each platforms as platform (platform.id)}
				<button
					class="filter-btn"
					class:active={selectedPlatform === platform.id}
					onclick={() => (selectedPlatform = platform.id)}
				>
					{platform.name}
				</button>
			{/each}
		</div>

		<section class="dashboard__content-section">
			<h2 class="section-title">Platform Tutorials</h2>

			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{filteredTutorials().length} tutorials</span>
				</div>
			</div>

			<div class="card-grid flex-grid row">
				{#each filteredTutorials() as tutorial (tutorial.id)}
					<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
						<div class="card flex-grid-panel">
							<figure class="card-media card-media--video">
								<a
									href="/platform-tutorials/{tutorial.slug}"
									class="card-image"
									style="background-image: url({tutorial.thumbnail});"
								>
									<img alt={tutorial.title} />
								</a>
							</figure>
							<section class="card-body">
								<div class="card-meta">
									<span class="platform-badge">{tutorial.platformName}</span>
									<span class="duration">{tutorial.duration}</span>
								</div>
								<h4 class="h5 card-title">
									<a href="/platform-tutorials/{tutorial.slug}">{tutorial.title}</a>
								</h4>
								<div class="card-description">
									<p>{tutorial.description}</p>
								</div>
							</section>
							<footer class="card-footer">
								<a class="btn btn-tiny btn-default" href="/platform-tutorials/{tutorial.slug}"
									>Watch Tutorial</a
								>
							</footer>
						</div>
					</article>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}

	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		background-color: #efefef;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	.section-title {
		color: #333;
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 30px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Platform Filters */
	.platform-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		padding: 20px 40px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.filter-btn {
		padding: 10px 20px;
		border: 1px solid #ddd;
		background: #fff;
		border-radius: 4px;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: #f5f5f5;
	}

	.filter-btn.active {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}

	.dashboard-filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.dashboard-filters__count {
		color: #666;
		font-size: 14px;
	}

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	.card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		height: 100%;
		transition: all 0.2s;
	}

	.card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.card-media {
		position: relative;
		height: 180px;
		overflow: hidden;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='white'%3E%3Cpath d='M8 5v14l11-7z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: center;
		background-size: 30px;
		transition: all 0.2s;
	}

	.card-media--video:hover::after {
		background-color: rgba(246, 149, 50, 0.9);
		transform: translate(-50%, -50%) scale(1.1);
	}

	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
	}

	.card-image img {
		opacity: 0;
	}

	.card-body {
		padding: 15px 20px;
		flex: 1;
	}

	.card-meta {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 10px;
	}

	.platform-badge {
		background: #28a745;
		color: #fff;
		padding: 3px 8px;
		border-radius: 3px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.duration {
		color: #999;
		font-size: 13px;
	}

	.card-title {
		margin: 0 0 10px;
		font-size: 18px;
		font-weight: 700;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.card-title a:hover {
		color: #0984ae;
	}

	.card-description {
		color: #666;
		font-size: 14px;
		line-height: 1.5;
	}

	.card-description p {
		margin: 0;
	}

	.card-footer {
		padding: 15px 20px;
		border-top: 1px solid #eee;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		border-radius: 4px;
		text-decoration: none;
		font-size: 13px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn:hover {
		background: #e8e8e8;
	}
</style>
