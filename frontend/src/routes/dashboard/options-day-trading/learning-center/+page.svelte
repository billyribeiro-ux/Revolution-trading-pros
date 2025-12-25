<script lang="ts">
	/**
	 * Learning Center - Options Day Trading
	 * Mockup page using Mastering the Trade content structure
	 */
</script>

<svelte:head>
	<title>Learning Center | Options Day Trading</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Learning Center</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2>Options Day Trading Courses</h2>
			<p>Access our comprehensive library of trading education materials.</p>

			<div class="course-grid">
				<div class="course-card">
					<h3>Options Basics</h3>
					<p>Learn the fundamentals of options trading, including calls, puts, and basic strategies.</p>
					<button type="button" class="course-link" onclick={() => alert('Course coming soon')}>Start Course →</button>
				</div>
				<div class="course-card">
					<h3>Day Trading Strategies</h3>
					<p>Master intraday options trading techniques for consistent profits.</p>
					<button type="button" class="course-link" onclick={() => alert('Course coming soon')}>Start Course →</button>
				</div>
				<div class="course-card">
					<h3>Risk Management</h3>
					<p>Learn how to protect your capital and manage position sizing.</p>
					<button type="button" class="course-link" onclick={() => alert('Course coming soon')}>Start Course →</button>
				</div>
				<div class="course-card">
					<h3>Technical Analysis</h3>
					<p>Chart patterns, indicators, and price action for options traders.</p>
					<button type="button" class="course-link" onclick={() => alert('Course coming soon')}>Start Course →</button>
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
	}

	.dashboard__content-section {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
	}

	h2 {
		font-size: 24px;
		margin-bottom: 10px;
		color: #333;
	}

	.dashboard__content-section > p {
		color: #666;
		margin-bottom: 30px;
	}

	.course-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}

	.course-card {
		background: #f9f9f9;
		padding: 25px;
		border-radius: 4px;
		border: 1px solid #eee;
	}

	.course-card h3 {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.course-card p {
		font-size: 14px;
		color: #666;
		margin: 0 0 15px;
		line-height: 1.5;
	}

	.course-link {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		font-family: inherit;
		font-size: inherit;
	}

	.course-link:hover {
		text-decoration: underline;
	}

	@media (max-width: 768px) {
		.course-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
