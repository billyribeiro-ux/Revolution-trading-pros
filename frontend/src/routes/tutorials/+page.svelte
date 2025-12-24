<script lang="ts">
	/**
	 * Platform Tutorials
	 */
</script>

<svelte:head>
	<title>Platform Tutorials | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Platform Tutorials</h1>
		<a href="/dashboard/day-trading-room" class="btn btn-xs btn-default">
			← Back to Dashboard
		</a>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2>Platform Tutorials</h2>
			<p>Learn how to use our trading platform and tools effectively.</p>
			
			<div class="tutorials-grid">
				<div class="tutorial-card">
					<h3>Getting Started</h3>
					<p>Learn the basics of our platform</p>
				</div>
				<div class="tutorial-card">
					<h3>Advanced Features</h3>
					<p>Master advanced trading tools</p>
				</div>
				<div class="tutorial-card">
					<h3>Trading Room Guide</h3>
					<p>How to use the live trading room</p>
				</div>
			</div>
			
			<a href="/dashboard/day-trading-room" class="btn btn-default">Return to Dashboard</a>
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
		gap: 15px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 15px;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.btn {
		padding: 8px 16px;
		border-radius: 3px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e0e0e0;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
	}

	h2 {
		font-size: 24px;
		margin-bottom: 20px;
		color: #333;
	}

	.tutorials-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 20px;
		margin: 30px 0;
	}

	.tutorial-card {
		background: #f5f5f5;
		padding: 20px;
		border-radius: 4px;
		border: 1px solid #dbdbdb;
	}

	.tutorial-card h3 {
		font-size: 18px;
		margin-bottom: 10px;
		color: #333;
	}

	.tutorial-card p {
		color: #666;
		margin: 0;
	}
</style>
