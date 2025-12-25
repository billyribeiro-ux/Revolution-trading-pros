<script lang="ts">
	/**
	 * Resources - Options Day Trading
	 * Mockup page using Mastering the Trade content structure
	 */
</script>

<svelte:head>
	<title>Resources | Options Day Trading</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Resources</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2>Trading Resources & Tools</h2>
			<p>Download trading tools, cheat sheets, and reference materials.</p>

			<div class="resource-list">
				<div class="resource-item">
					<div class="resource-icon">📄</div>
					<div class="resource-info">
						<h3>Options Greeks Cheat Sheet</h3>
						<p>Quick reference guide for Delta, Gamma, Theta, and Vega.</p>
					</div>
					<button type="button" class="resource-download" onclick={() => alert('Download functionality coming soon')}>Download PDF</button>
				</div>
				<div class="resource-item">
					<div class="resource-icon">📊</div>
					<div class="resource-info">
						<h3>Trade Journal Template</h3>
						<p>Track your trades and analyze your performance.</p>
					</div>
					<button type="button" class="resource-download" onclick={() => alert('Download functionality coming soon')}>Download Excel</button>
				</div>
				<div class="resource-item">
					<div class="resource-icon">📈</div>
					<div class="resource-info">
						<h3>Position Sizing Calculator</h3>
						<p>Calculate optimal position sizes based on your risk tolerance.</p>
					</div>
					<button type="button" class="resource-download" onclick={() => alert('Download functionality coming soon')}>Download Excel</button>
				</div>
				<div class="resource-item">
					<div class="resource-icon">📋</div>
					<div class="resource-info">
						<h3>Pre-Market Checklist</h3>
						<p>Daily checklist to prepare for the trading day.</p>
					</div>
					<button type="button" class="resource-download" onclick={() => alert('Download functionality coming soon')}>Download PDF</button>
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

	.resource-list {
		display: flex;
		flex-direction: column;
		gap: 15px;
	}

	.resource-item {
		display: flex;
		align-items: center;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 4px;
		border: 1px solid #eee;
	}

	.resource-icon {
		font-size: 32px;
		margin-right: 20px;
	}

	.resource-info {
		flex: 1;
	}

	.resource-info h3 {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
	}

	.resource-info p {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.resource-download {
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
		font-weight: 600;
		font-size: 14px;
		transition: background 0.2s;
		border: none;
		cursor: pointer;
		font-family: inherit;
	}

	.resource-download:hover {
		background: #077a9e;
	}

	@media (max-width: 768px) {
		.resource-item {
			flex-direction: column;
			text-align: center;
		}

		.resource-icon {
			margin-right: 0;
			margin-bottom: 15px;
		}

		.resource-info {
			margin-bottom: 15px;
		}
	}
</style>
