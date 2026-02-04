<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Explosive Swings - Analytics Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Comprehensive trading performance analytics with charts and metrics
	 * @version 2.0.0 - Phase 4 Analytics Dashboard Update
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */
	import { browser } from '$app/environment';
	import { createAnalyticsState, type TimePeriod } from './analytics.state.svelte';

	// Layout Components
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

	// Analytics Components
	import PerformanceOverview from './components/PerformanceOverview.svelte';
	import EquityCurveChart from './components/EquityCurveChart.svelte';
	import TickerHeatmap from './components/TickerHeatmap.svelte';
	import MonthlyReturnsChart from './components/MonthlyReturnsChart.svelte';
	import DrawdownChart from './components/DrawdownChart.svelte';
	import SetupBreakdown from './components/SetupBreakdown.svelte';
	import DateRangePicker from './components/DateRangePicker.svelte';
	import TickerPerformanceTable from './components/TickerPerformanceTable.svelte';

	const analytics = createAnalyticsState();

	const periods: { value: TimePeriod; label: string }[] = [
		{ value: '30d', label: '30D' },
		{ value: '90d', label: '90D' },
		{ value: '180d', label: '6M' },
		{ value: '365d', label: '1Y' },
		{ value: 'ytd', label: 'YTD' },
		{ value: 'all', label: 'All' }
	];

	$effect(() => {
		if (browser) {
			analytics.initializeData();
		}
	});
</script>

<svelte:head>
	<title>Analytics | Explosive Swings | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Comprehensive trading performance analytics for Explosive Swings trading room"
	/>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
	showTradingRoomControls={false}
/>

<div class="analytics-page">
	<!-- Page Header -->
	<header class="page-header">
		<div class="header-content">
			<div class="title-section">
				<h1>Performance Analytics</h1>
				<p class="subtitle">{analytics.periodLabel} Performance Overview</p>
			</div>

			<!-- Period Selector -->
			<div class="period-selector" role="group" aria-label="Time period selection">
				{#each periods as period}
					<button
						class="period-btn"
						class:active={analytics.selectedPeriod === period.value}
						onclick={() => analytics.setPeriod(period.value)}
						aria-pressed={analytics.selectedPeriod === period.value}
					>
						{period.label}
					</button>
				{/each}
			</div>
		</div>

		<!-- Back Link -->
		<a href="/dashboard/explosive-swings" class="back-link">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M19 12H5M12 19l-7-7 7-7" />
			</svg>
			Back to Dashboard
		</a>
	</header>

	{#if analytics.error}
		<div class="error-banner" role="alert">
			<p>{analytics.error}</p>
			<button onclick={() => analytics.fetchAll()}>Retry</button>
		</div>
	{/if}

	<!-- Main Content Grid -->
	<div class="analytics-grid">
		<!-- Performance Overview - Full Width -->
		<section class="grid-item full-width">
			<PerformanceOverview
				summary={analytics.summary}
				streakAnalysis={analytics.streakAnalysis}
				alertEffectiveness={analytics.alertEffectiveness}
				isLoading={analytics.isLoading}
			/>
		</section>

		<!-- Equity Curve Chart - Wide -->
		<section class="grid-item wide">
			<EquityCurveChart data={analytics.equityCurve} isLoading={analytics.isLoadingEquityCurve} />
		</section>

		<!-- Setup Breakdown -->
		<section class="grid-item narrow">
			<SetupBreakdown data={analytics.setupPerformance} isLoading={analytics.isLoading} />
		</section>

		<!-- Monthly Returns Chart -->
		<section class="grid-item half">
			<MonthlyReturnsChart data={analytics.monthlyPerformance} isLoading={analytics.isLoading} />
		</section>

		<!-- Drawdown Chart -->
		<section class="grid-item half">
			<DrawdownChart data={analytics.equityCurve} isLoading={analytics.isLoadingEquityCurve} />
		</section>

		<!-- Ticker Heatmap - Full Width -->
		<section class="grid-item full-width">
			<TickerHeatmap
				data={analytics.tickerPerformance}
				topPerformers={analytics.topTickers}
				worstPerformers={analytics.worstTickers}
				isLoading={analytics.isLoading}
			/>
		</section>

		<!-- Ticker Performance Table - Full Width -->
		<section class="grid-item full-width">
			<TickerPerformanceTable data={analytics.tickerPerformance} isLoading={analytics.isLoading} />
		</section>
	</div>

	<!-- Empty State -->
	{#if !analytics.isLoading && !analytics.hasData}
		<div class="empty-state">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M3 3v18h18" />
				<path d="M18.7 8l-5.1 5.2-2.8-2.7L7 14.3" />
			</svg>
			<h2>No Trading Data Yet</h2>
			<p>Start trading to see your performance analytics here.</p>
			<a href="/dashboard/explosive-swings" class="cta-btn">Go to Dashboard</a>
		</div>
	{/if}
</div>

<style>
	.analytics-page {
		background: var(--color-bg-page);
		min-height: 100vh;
		padding-bottom: 48px;
	}

	.page-header {
		background: var(--color-bg-card);
		border-bottom: 1px solid var(--color-border-default);
		padding: 24px;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		max-width: 1400px;
		margin: 0 auto;
		flex-wrap: wrap;
		gap: 16px;
	}

	.title-section h1 {
		font-size: 24px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 4px 0;
	}

	.subtitle {
		font-size: 14px;
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.period-selector {
		display: flex;
		gap: 4px;
		background: var(--color-bg-subtle);
		padding: 4px;
		border-radius: 8px;
	}

	.period-btn {
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		background: transparent;
		color: var(--color-text-secondary);
		transition:
			background 0.15s,
			color 0.15s;
	}

	.period-btn:hover {
		background: var(--color-bg-muted);
	}

	.period-btn.active {
		background: var(--color-bg-card);
		color: var(--color-text-primary);
		box-shadow: var(--shadow-sm);
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		margin-top: 12px;
		max-width: 1400px;
		margin-left: auto;
		margin-right: auto;
	}

	.back-link:hover {
		color: var(--color-brand-primary);
	}

	.error-banner {
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-border);
		border-radius: 8px;
		padding: 12px 16px;
		margin: 24px;
		max-width: 1400px;
		margin-left: auto;
		margin-right: auto;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.error-banner p {
		margin: 0;
		color: var(--color-loss);
		font-size: 14px;
	}

	.error-banner button {
		padding: 6px 12px;
		background: var(--color-loss);
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 13px;
	}

	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(12, 1fr);
		gap: 20px;
		padding: 24px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.grid-item {
		grid-column: span 6;
	}

	.grid-item.full-width {
		grid-column: span 12;
	}

	.grid-item.wide {
		grid-column: span 8;
	}

	.grid-item.narrow {
		grid-column: span 4;
	}

	.grid-item.half {
		grid-column: span 6;
	}

	.empty-state {
		text-align: center;
		padding: 80px 24px;
		color: var(--color-text-tertiary);
	}

	.empty-state svg {
		margin-bottom: 16px;
		opacity: 0.5;
	}

	.empty-state h2 {
		font-size: 20px;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin: 0 0 8px 0;
	}

	.empty-state p {
		margin: 0 0 24px 0;
		font-size: 14px;
	}

	.cta-btn {
		display: inline-block;
		padding: 12px 24px;
		background: var(--color-brand-primary);
		color: white;
		font-weight: 600;
		font-size: 14px;
		text-decoration: none;
		border-radius: 8px;
		transition: background 0.15s;
	}

	.cta-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.grid-item.wide {
			grid-column: span 12;
		}

		.grid-item.narrow {
			grid-column: span 12;
		}
	}

	@media (max-width: 1024px) {
		.grid-item {
			grid-column: span 12;
		}

		.grid-item.half {
			grid-column: span 12;
		}
	}

	@media (max-width: 768px) {
		.page-header {
			padding: 16px;
		}

		.header-content {
			flex-direction: column;
			align-items: flex-start;
		}

		.title-section h1 {
			font-size: 20px;
		}

		.analytics-grid {
			padding: 16px;
			gap: 16px;
		}

		.period-selector {
			width: 100%;
			justify-content: space-between;
		}

		.period-btn {
			flex: 1;
			padding: 8px 12px;
		}
	}
</style>
