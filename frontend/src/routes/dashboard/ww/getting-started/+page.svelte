<script lang="ts">
	/**
	 * Weekly Watchlist - Getting Started
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Learn how to use the Weekly Watchlist
	 * @version 1.0.0 - December 2025
	 */
</script>

<svelte:head>
	<title>Getting Started - Weekly Watchlist | Revolution Trading Pros</title>
	<meta name="description" content="Learn how to best use the Weekly Watchlist and get the most out of it." />
</svelte:head>

<!-- Dashboard Layout -->
<div class="dashboard">
	<!-- Sidebar Navigation -->
	<aside class="dashboard__sidebar">
		<nav class="dashboard__nav-secondary">
			<ul>
				<li>
					<a href="/dashboard/ww/">
						<span class="dashboard__nav-item-icon st-icon-dashboard"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist</span>
					</a>
				</li>
				<li>
					<a href="/dashboard/ww/watchlist-rundown-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Watchlist Rundown Archive</span>
					</a>
				</li>
				<li>
					<a href="/dashboard/ww/weekly-watchlist-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist Archive</span>
					</a>
				</li>
			</ul>
		</nav>
	</aside>

	<!-- Main Content -->
	<main class="dashboard__main">
		<header class="dashboard__header">
			<div class="dashboard__header-left">
				<h1 class="dashboard__page-title">Getting Started with Weekly Watchlist</h1>
			</div>
		</header>

		<div class="dashboard__content">
			<div class="dashboard__content-main">
				<section class="dashboard__content-section">
					<h2 class="section-title">How to Use the Weekly Watchlist</h2>

					<div class="getting-started-content">
						<div class="step">
							<div class="step-number">1</div>
							<div class="step-content">
								<h3>Watch the Weekly Rundown Video</h3>
								<p>Each week, one of our expert traders creates a video rundown explaining the watchlist picks for the week. This video covers the reasoning behind each stock selection and what to look for.</p>
								<a href="/dashboard/ww/watchlist-rundown-archive/" class="btn btn-primary">View Rundown Archive</a>
							</div>
						</div>

						<div class="step">
							<div class="step-number">2</div>
							<div class="step-content">
								<h3>Download the Weekly Watchlist</h3>
								<p>Access the complete watchlist in a convenient spreadsheet format. The watchlist includes entry points, targets, and stop levels for each stock pick.</p>
								<a href="/dashboard/ww/weekly-watchlist-archive/" class="btn btn-primary">View Watchlist Archive</a>
							</div>
						</div>

						<div class="step">
							<div class="step-number">3</div>
							<div class="step-content">
								<h3>Follow Along Throughout the Week</h3>
								<p>Use the watchlist as a guide for potential trading opportunities. Remember to always use proper risk management and do your own due diligence before entering any trades.</p>
							</div>
						</div>

						<div class="step">
							<div class="step-number">4</div>
							<div class="step-content">
								<h3>Check Back Each Week</h3>
								<p>New watchlists are published every week. Check back regularly to stay up to date with the latest stock picks and trading opportunities.</p>
								<a href="/dashboard/ww/" class="btn btn-default">Back to Dashboard</a>
							</div>
						</div>
					</div>
				</section>
			</div>
		</div>
	</main>
</div>

<style>
	/* Dashboard Layout */
	.dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* Sidebar - WordPress dashboard.8f78208b.css: 280px width, #0f2d41 background */
	.dashboard__sidebar {
		width: 280px;
		background: #0f2d41;
		flex-shrink: 0;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 20px 0;
	}

	.dashboard__nav-secondary li {
		margin: 0;
	}

	.dashboard__nav-secondary a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 24px;
		color: #999;
		text-decoration: none;
		font-size: 14px;
		transition: all 0.2s;
	}

	.dashboard__nav-secondary a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	.dashboard__main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__header {
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 30px;
	}

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 30px;
		padding-bottom: 15px;
		border-bottom: 2px solid #0984ae;
	}

	.getting-started-content {
		max-width: 800px;
	}

	.step {
		display: flex;
		gap: 24px;
		margin-bottom: 40px;
	}

	.step:last-child {
		margin-bottom: 0;
	}

	.step-number {
		flex-shrink: 0;
		width: 48px;
		height: 48px;
		background: #0984ae;
		color: #fff;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 20px;
		font-weight: 700;
	}

	.step-content h3 {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 12px;
	}

	.step-content p {
		font-size: 15px;
		color: #666;
		line-height: 1.6;
		margin: 0 0 16px;
	}

	.btn {
		display: inline-block;
		padding: 10px 20px;
		text-decoration: none;
		border-radius: 4px;
		font-weight: 600;
		font-size: 14px;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e8e8e8;
	}

	@media (max-width: 992px) {
		.dashboard__sidebar {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.dashboard__content {
			padding: 15px;
		}

		.dashboard__content-section {
			padding: 20px;
		}

		.step {
			flex-direction: column;
			gap: 16px;
		}
	}
</style>
