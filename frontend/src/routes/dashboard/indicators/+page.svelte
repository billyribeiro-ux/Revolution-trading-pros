<script lang="ts">
	/**
	 * Dashboard - My Indicators Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Shows user's purchased indicators or empty state.
	 * Matches Simpler Trading's My Indicators page exactly.
	 *
	 * @version 4.0.0 (Simpler Trading Exact / December 2025)
	 */

	// ═══════════════════════════════════════════════════════════════════════════
	// USER INDICATORS (Mock data - will be fetched from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface UserIndicator {
		id: number;
		name: string;
		platform: string;
		downloadUrl: string;
		purchaseDate: string;
	}

	// Empty array for now - matches the screenshot showing empty state
	const userIndicators: UserIndicator[] = [];
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Indicators | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER - WordPress: .dashboard__header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Indicators</h1>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD CONTENT - WordPress: .dashboard__content
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			{#if userIndicators.length > 0}
				<!-- Indicators Grid (for when user has indicators) -->
				<div class="indicators-grid">
					{#each userIndicators as indicator (indicator.id)}
						<div class="indicator-card">
							<h4>{indicator.name}</h4>
							<p>Platform: {indicator.platform}</p>
							<p>Purchased: {indicator.purchaseDate}</p>
							<a href={indicator.downloadUrl} class="btn btn-default btn-tiny">Download</a>
						</div>
					{/each}
				</div>
			{:else}
				<!-- Empty State - Simpler Trading EXACT -->
				<div class="empty-state">
					<h4 class="pb-2">You don't have any Indicators.</h4>
					<a target="_blank" class="btn btn-orange" href="/indicators">See All Indicators</a>
				</div>
			{/if}
		</section>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD HEADER - WordPress: .dashboard__header
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD CONTENT - WordPress: .dashboard__content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: calc(100vh - 80px);
	}

	.dashboard__content-main {
		width: 100%;
	}

	.dashboard__content-section {
		/* No additional styling needed */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		padding: 20px 0;
	}

	.empty-state h4 {
		font-size: 16px;
		font-weight: 400;
		color: #333;
		margin: 0 0 15px;
	}

	.pb-2 {
		padding-bottom: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTONS - Simpler Trading Style
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn {
		display: inline-block;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
		text-align: center;
	}

	.btn-orange {
		background: #f99e31;
		border: 1px solid #f99e31;
		color: #fff;
	}

	.btn-orange:hover {
		background: #dc7309;
		border-color: #dc7309;
	}

	.btn-default {
		background: transparent;
		border: 1px solid #0984ae;
		color: #0984ae;
	}

	.btn-default:hover {
		background: #0984ae;
		color: #fff;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   INDICATORS GRID (for when user has indicators)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(min(100%, 280px), 1fr));
		gap: 20px;
	}

	.indicator-card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		padding: 20px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
	}

	.indicator-card h4 {
		font-size: 16px;
		font-weight: 600;
		color: #333;
		margin: 0 0 10px;
	}

	.indicator-card p {
		font-size: 14px;
		color: #666;
		margin: 0 0 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}
	}
</style>
