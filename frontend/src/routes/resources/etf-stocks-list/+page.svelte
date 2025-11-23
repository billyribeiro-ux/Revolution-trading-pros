<script lang="ts">
	import { IconTrendingUp, IconChartLine } from '@tabler/icons-svelte';
</script>

<svelte:head>
	<title>ETF Stocks List | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Comprehensive list of ETF stocks for professional traders. Track and analyze the most traded ETFs in the market."
	/>
</svelte:head>

<div class="page-container">
	<header class="page-header">
		<div class="header-content">
			<div class="icon-wrapper">
				<IconChartLine size={48} class="header-icon" />
			</div>
			<h1 class="page-title">ETF Stocks List</h1>
			<p class="page-subtitle">
				Comprehensive list of Exchange-Traded Funds for professional traders. Track performance,
				analyze trends, and make informed decisions.
			</p>
		</div>
	</header>

	<section class="content-section">
		<div class="content-wrapper">
			<div class="info-card">
				<div class="card-header">
					<IconTrendingUp size={32} class="card-icon" />
					<h2>Popular ETFs</h2>
				</div>
				<div class="card-body">
					<p>
						Stay tuned for our curated list of the most popular and actively traded ETFs across
						various sectors including technology, healthcare, energy, and more.
					</p>

					<div class="placeholder-grid">
						<div class="placeholder-item">
							<div class="placeholder-title">SPY</div>
							<div class="placeholder-desc">S&P 500 ETF Trust</div>
						</div>
						<div class="placeholder-item">
							<div class="placeholder-title">QQQ</div>
							<div class="placeholder-desc">Invesco QQQ Trust</div>
						</div>
						<div class="placeholder-item">
							<div class="placeholder-title">IWM</div>
							<div class="placeholder-desc">iShares Russell 2000 ETF</div>
						</div>
						<div class="placeholder-item">
							<div class="placeholder-title">VTI</div>
							<div class="placeholder-desc">Vanguard Total Stock Market ETF</div>
						</div>
					</div>
				</div>
			</div>

			<div class="info-card">
				<div class="card-header">
					<h2>Coming Soon</h2>
				</div>
				<div class="card-body">
					<ul class="feature-list">
						<li>Real-time ETF price tracking</li>
						<li>Sector-based ETF categorization</li>
						<li>Performance analytics and charts</li>
						<li>Institutional ownership data</li>
						<li>Dividend yield information</li>
						<li>Expense ratio comparisons</li>
					</ul>
				</div>
			</div>
		</div>
	</section>
</div>

<style>
	.page-container {
		min-height: 100vh;
		background: linear-gradient(135deg, #050e1f 0%, #0a1628 50%, #0f1e33 100%);
		padding-top: 120px;
	}

	.page-header {
		padding: 4rem 1.5rem;
		text-align: center;
		background: linear-gradient(to bottom, rgba(46, 142, 255, 0.1), transparent);
	}

	.header-content {
		max-width: 800px;
		margin: 0 auto;
	}

	.icon-wrapper {
		display: inline-flex;
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.2), rgba(46, 142, 255, 0.05));
		border-radius: 20px;
		margin-bottom: 2rem;
	}

	:global(.header-icon) {
		color: #2e8eff;
	}

	.page-title {
		font-size: clamp(2.5rem, 5vw, 4rem);
		font-weight: 800;
		color: #ffffff;
		margin-bottom: 1rem;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.page-subtitle {
		font-size: 1.25rem;
		color: #cbd5e1;
		line-height: 1.8;
		max-width: 600px;
		margin: 0 auto;
	}

	.content-section {
		padding: 2rem 1.5rem 4rem;
	}

	.content-wrapper {
		max-width: 1200px;
		margin: 0 auto;
		display: grid;
		gap: 2rem;
	}

	.info-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(46, 142, 255, 0.2);
		border-radius: 16px;
		padding: 2rem;
		backdrop-filter: blur(10px);
	}

	.card-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	:global(.card-icon) {
		color: #2e8eff;
	}

	.card-header h2 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0;
	}

	.card-body {
		color: #cbd5e1;
		font-size: 1.1rem;
		line-height: 1.8;
	}

	.placeholder-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.placeholder-item {
		background: rgba(46, 142, 255, 0.1);
		border: 1px solid rgba(46, 142, 255, 0.3);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.3s ease;
	}

	.placeholder-item:hover {
		background: rgba(46, 142, 255, 0.15);
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-2px);
	}

	.placeholder-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #2e8eff;
		margin-bottom: 0.5rem;
	}

	.placeholder-desc {
		color: #94a3b8;
		font-size: 0.95rem;
	}

	.feature-list {
		list-style: none;
		padding: 0;
		margin: 1.5rem 0 0 0;
	}

	.feature-list li {
		padding: 0.75rem 0;
		padding-left: 1.5rem;
		position: relative;
		font-size: 1.1rem;
	}

	.feature-list li::before {
		content: '✓';
		position: absolute;
		left: 0;
		color: #2e8eff;
		font-weight: bold;
	}

	@media (max-width: 768px) {
		.page-container {
			padding-top: 80px;
		}

		.page-header {
			padding: 2rem 1rem;
		}

		.placeholder-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
