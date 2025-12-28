<script lang="ts">
	/**
	 * Start Here - Options Day Trading Onboarding
	 * Mockup page using Mastering the Trade content structure
	 */
</script>

<svelte:head>
	<title>Start Here | Options Day Trading</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Welcome to Options Day Trading</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2>Getting Started</h2>
			<p>Welcome to Options Day Trading! This page will help you get started with our trading room services.</p>

			<h3>Quick Start Guide</h3>
			<ul>
				<li>Review the <a href="/trading-room-rules.pdf" target="_blank">Trading Room Rules</a></li>
				<li>Explore the <a href="/dashboard/options-day-trading/learning-center">Learning Center</a></li>
				<li>Check out <a href="/dashboard/options-day-trading/resources">Resources</a></li>
				<li>Return to the <a href="/dashboard/options-day-trading">Dashboard</a></li>
			</ul>
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
	}

	.dashboard__content-section {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
	}

	h2 {
		font-size: 24px;
		margin-bottom: 20px;
		color: #333;
	}

	h3 {
		font-size: 18px;
		margin: 20px 0 10px;
		color: #333;
	}

	ul {
		list-style: disc;
		margin-left: 20px;
	}

	li {
		margin-bottom: 10px;
	}

	a {
		color: #0984ae;
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}
</style>
