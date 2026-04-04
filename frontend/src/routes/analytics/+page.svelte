<!--
	Analytics Dashboard - Enterprise Analytics Overview
	═══════════════════════════════════════════════════════════════════════════
	
	Main analytics dashboard showing KPIs, real-time metrics, funnels,
	cohorts, attribution, and AI-powered insights.
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		analyticsStore,
		getDashboard,
		getIsAnalyticsLoading,
		getAnalyticsSelectedPeriod
	} from '$lib/stores/analytics.svelte';
	import {
		KpiGrid,
		RealTimeWidget,
		PeriodSelector,
		TimeSeriesChart,
		FunnelChart,
		CohortMatrix,
		AttributionChart
	} from '$lib/components/analytics';
	import { Icon, IconAlertTriangle, IconBolt, IconBrain, IconChartBar, IconCurrencyDollar, IconTarget, IconTrendingUp, IconUsers } from '$lib/icons';

	let selectedTab = $state<
		'overview' | 'funnels' | 'cohorts' | 'attribution' | 'behavior' | 'revenue'
	>('overview');

	// Local derived from getters
	const dashboard = $derived(getDashboard());
	const isLoading = $derived(getIsAnalyticsLoading());
	const selectedPeriod = $derived(getAnalyticsSelectedPeriod());

	onMount(() => {
		analyticsStore.loadDashboard(selectedPeriod);
		analyticsStore.startRealtimeUpdates(10000);
	});

	onDestroy(() => {
		analyticsStore.stopRealtimeUpdates();
	});

	function handlePeriodChange(period: string) {
		analyticsStore.setPeriod(period);
	}
</script>

<svelte:head>
	<title>Analytics Dashboard | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Enterprise analytics dashboard with real-time metrics, funnels, cohorts, and AI insights"
	/>
</svelte:head>

<div class="analytics-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-content">
			<div class="header-title">
				<Icon icon={IconChartBar} size={32} />
				<div>
					<h1 class="page-heading">Analytics Dashboard</h1>
					<p class="page-subheading">Enterprise-grade analytics and insights</p>
				</div>
			</div>

			<div class="header-actions">
				<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
			</div>
		</div>
	</div>

	<!-- Real-Time Widget -->
	<div class="realtime-section">
		<RealTimeWidget />
	</div>

	<!-- Navigation Tabs -->
	<div class="dashboard-tabs">
		<button
			class="tab"
			class:active={selectedTab === 'overview'}
			onclick={() => (selectedTab = 'overview')}
		>
			<Icon icon={IconChartBar} size={20} />
			Overview
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'funnels'}
			onclick={() => (selectedTab = 'funnels')}
		>
			<Icon icon={IconTarget} size={20} />
			Funnels
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'cohorts'}
			onclick={() => (selectedTab = 'cohorts')}
		>
			<Icon icon={IconUsers} size={20} />
			Cohorts
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'attribution'}
			onclick={() => (selectedTab = 'attribution')}
		>
			<Icon icon={IconTrendingUp} size={20} />
			Attribution
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'behavior'}
			onclick={() => (selectedTab = 'behavior')}
		>
			<Icon icon={IconBolt} size={20} />
			Behavior
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'revenue'}
			onclick={() => selectedTab === 'revenue'}
		>
			<Icon icon={IconCurrencyDollar} size={20} />
			Revenue
		</button>
	</div>

	<!-- Content -->
	<div class="dashboard-content">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="loading-text">Loading analytics data...</p>
			</div>
		{:else if dashboard}
			<!-- Overview Tab -->
			{#if selectedTab === 'overview'}
				<div class="overview-grid">
					<!-- KPI Grid -->
					<div class="section">
						<div class="section-header">
							<h2 class="section-title">Key Performance Indicators</h2>
							<p class="section-subtitle">Primary metrics for {selectedPeriod}</p>
						</div>
						<KpiGrid kpis={dashboard.kpis} />
					</div>

					<!-- Anomalies & Alerts -->
					{#if dashboard.anomalies && (dashboard.anomalies.anomalies_count > 0 || dashboard.anomalies.alerts_triggered > 0)}
						<div class="section" data-variant="anomaly">
							<div class="section-header">
								<div class="anomaly-header-row">
									<Icon icon={IconAlertTriangle} size={24} />
									<h2 class="section-title">Anomalies & Alerts</h2>
								</div>
							</div>
							<div class="anomaly-grid">
								<div class="anomaly-card">
									<div class="anomaly-value">{dashboard.anomalies.anomalies_count}</div>
									<div class="anomaly-label">Anomalies Detected</div>
								</div>
								<div class="anomaly-card">
									<div class="anomaly-value">{dashboard.anomalies.alerts_triggered}</div>
									<div class="anomaly-label">Alerts Triggered</div>
								</div>
								<div class="anomaly-card" data-variant="critical">
									<div class="anomaly-value">{dashboard.anomalies.critical_count}</div>
									<div class="anomaly-label">Critical Issues</div>
								</div>
							</div>
						</div>
					{/if}

					<!-- Time Series Charts -->
					{#if dashboard.time_series}
						<div class="section">
							<div class="section-header">
								<h2 class="section-title">Trends</h2>
							</div>
							<div class="charts-grid">
								{#if dashboard.time_series.revenue}
									<TimeSeriesChart
										data={dashboard.time_series.revenue}
										title="Revenue Trend"
										color="#fbbf24"
										formatValue={(v) => '$' + v.toLocaleString()}
									/>
								{/if}
								{#if dashboard.time_series.users}
									<TimeSeriesChart
										data={dashboard.time_series.users}
										title="User Growth"
										color="#60a5fa"
										formatValue={(v) => v.toLocaleString()}
									/>
								{/if}
							</div>
						</div>
					{/if}

					<!-- Quick Funnels -->
					{#if dashboard.funnels && dashboard.funnels.length > 0}
						<div class="section">
							<div class="section-header">
								<h2 class="section-title">Top Funnels</h2>
								<a href="/analytics/funnels" class="view-all-link">View All →</a>
							</div>
							<div class="funnels-grid">
								{#each dashboard.funnels.slice(0, 2) as funnel}
									<FunnelChart steps={funnel.steps} title={funnel.name} />
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Funnels Tab -->
			{#if selectedTab === 'funnels'}
				<div class="funnels-view">
					<div class="section-header">
						<h2 class="section-title">Conversion Funnels</h2>
						<button class="btn-primary">
							<Icon icon={IconTarget} size={18} />
							Create Funnel
						</button>
					</div>

					{#if dashboard.funnels && dashboard.funnels.length > 0}
						<div class="funnels-list">
							{#each dashboard.funnels as funnel}
								<FunnelChart steps={funnel.steps} title={funnel.name} />
							{/each}
						</div>
					{:else}
						<div class="empty-state">
							<Icon icon={IconTarget} size={64} />
							<h3 class="empty-title">No Funnels Yet</h3>
							<p class="empty-subtitle">Create your first funnel to track conversion paths</p>
							<button class="btn-primary empty-action">Create Funnel</button>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Cohorts Tab -->
			{#if selectedTab === 'cohorts'}
				<div class="cohorts-view">
					<div class="section-header">
						<h2 class="section-title">Cohort Analysis</h2>
						<button class="btn-primary">
							<Icon icon={IconUsers} size={18} />
							Create Cohort
						</button>
					</div>

					{#if dashboard.cohorts && dashboard.cohorts.length > 0}
						<div class="cohorts-list">
							{#each dashboard.cohorts as cohort}
								{@const transformedData = cohort.retention_matrix.map((row) => ({
									cohort_date: row.cohort,
									cohort_size: row.size,
									periods: Object.fromEntries(
										row.periods.map((retention, index) => [
											index,
											{
												active_users: Math.round((row.size * retention) / 100),
												retention_rate: retention,
												total_revenue: 0,
												avg_revenue_per_user: 0
											}
										])
									)
								}))}
								<CohortMatrix data={transformedData} title={cohort.name} />
							{/each}
						</div>
					{:else}
						<div class="empty-state">
							<Icon icon={IconUsers} size={64} />
							<h3 class="empty-title">No Cohorts Yet</h3>
							<p class="empty-subtitle">
								Create cohorts to analyze user retention and behavior
							</p>
							<button class="btn-primary empty-action">Create Cohort</button>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Attribution Tab -->
			{#if selectedTab === 'attribution'}
				<div class="attribution-view">
					<div class="section-header">
						<h2 class="section-title">Attribution Modeling</h2>
						<select class="model-selector">
							<option value="linear">Linear Attribution</option>
							<option value="first_click">First Click</option>
							<option value="last_click">Last Click</option>
							<option value="time_decay">Time Decay</option>
							<option value="position_based">Position Based</option>
						</select>
					</div>

					{#if dashboard.attribution}
						<AttributionChart channels={dashboard.attribution.channels} />
					{:else}
						<div class="empty-state">
							<Icon icon={IconTrendingUp} size={64} />
							<h3 class="empty-title">No Attribution Data</h3>
							<p class="empty-subtitle">
								Attribution data will appear once conversions are tracked
							</p>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Behavior Tab -->
			{#if selectedTab === 'behavior'}
				<div class="behavior-view">
					<div class="section-header">
						<h2 class="section-title">Behavior Analytics</h2>
					</div>

					<div class="behavior-grid">
						<!-- Top Pages -->
						{#if dashboard.top_pages}
							<div class="behavior-card">
								<h3 class="card-title">Top Pages</h3>
								<div class="page-list">
									{#each dashboard.top_pages as page}
										<div class="page-item">
											<span class="page-path">{page.page_path}</span>
											<span class="page-views">{page.views.toLocaleString()} views</span>
										</div>
									{/each}
								</div>
							</div>
						{/if}

						<!-- Top Events -->
						{#if dashboard.top_events}
							<div class="behavior-card">
								<h3 class="card-title">Top Events</h3>
								<div class="event-list">
									{#each dashboard.top_events as event}
										<div class="event-item">
											<span class="event-name">{event.event_name}</span>
											<span class="event-count">{event.count.toLocaleString()}</span>
										</div>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Revenue Tab -->
			{#if selectedTab === 'revenue'}
				<div class="revenue-view">
					<div class="section-header">
						<h2 class="section-title">Revenue Analytics</h2>
					</div>

					<div class="revenue-grid">
						<!-- Revenue KPIs -->
						<div class="revenue-kpis">
							{#each dashboard.kpis.filter((k) => k.category === 'revenue') as kpi}
								<div class="revenue-kpi-card">
									<div class="kpi-label">{kpi.name}</div>
									<div class="kpi-value">{kpi.formatted_value}</div>
									<div
										class="kpi-change"
										class:positive={kpi.trend === 'up'}
										class:negative={kpi.trend === 'down'}
									>
										{kpi.change_percentage > 0 ? '+' : ''}{kpi.change_percentage}%
									</div>
								</div>
							{/each}
						</div>

						<!-- Revenue Chart -->
						{#if dashboard.time_series?.revenue}
							<div class="revenue-chart">
								<TimeSeriesChart
									data={dashboard.time_series.revenue}
									title="Revenue Over Time"
									color="#10b981"
									formatValue={(v) => '$' + v.toLocaleString()}
								/>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		{:else}
			<div class="empty-state">
				<Icon icon={IconBrain} size={64} />
				<h3 class="empty-title">No Data Available</h3>
				<p class="empty-subtitle">Analytics data will appear once events are tracked</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.analytics-dashboard {
		background: linear-gradient(to bottom right, oklch(0.13 0.02 260), oklch(0.2 0.02 250), oklch(0.13 0.02 260));
		padding: var(--space-6);
	}

	.dashboard-header { margin-block-end: var(--space-8); }

	.header-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: var(--space-4);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		color: oklch(0.8 0.18 90);
	}

	.page-heading {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.page-subheading {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
	}

	.header-actions { display: flex; align-items: center; gap: var(--space-3); }

	.realtime-section { margin-block-end: var(--space-8); }

	.dashboard-tabs {
		display: flex;
		gap: var(--space-2);
		margin-block-end: var(--space-8);
		overflow-x: auto;
		padding-block-end: var(--space-2);
		scrollbar-width: thin;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		border-radius: var(--radius-lg);
		font-weight: var(--weight-medium);
		background-color: oklch(0.25 0.01 250 / 50%);
		color: oklch(0.65 0.01 250);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		white-space: nowrap;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover { background-color: oklch(0.38 0.01 250 / 50%); color: oklch(1 0 0); }
		&.active { background-color: oklch(0.8 0.18 90 / 20%); color: oklch(0.8 0.18 90); border-color: oklch(0.8 0.18 90 / 50%); }
	}

	.dashboard-content { min-block-size: 400px; }

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
	}

	.loading-text { color: oklch(0.65 0.01 250); margin-block-start: var(--space-4); }

	.spinner {
		inline-size: 3rem;
		block-size: 3rem;
		border: 4px solid oklch(0.38 0.01 250);
		border-block-start-color: oklch(0.8 0.18 90);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin { to { transform: rotate(360deg); } }

	.overview-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
	}

	.section {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);

		&[data-variant='anomaly'] {
			border-color: oklch(0.7 0.18 55 / 30%);
			background-color: oklch(0.7 0.18 55 / 5%);
		}
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.section-title {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
	}

	.view-all-link {
		color: oklch(0.8 0.18 90);
		font-weight: var(--weight-medium);
		text-decoration: none;
		transition: color var(--duration-fast) var(--ease-default);
		&:hover { color: oklch(0.85 0.16 90); }
	}

	.anomaly-header-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		color: oklch(0.75 0.16 55);
	}

	.anomaly-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
		@media (min-width: 768px) { grid-template-columns: repeat(3, 1fr); }
	}

	.anomaly-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-6);
		text-align: center;
		border: 1px solid oklch(0.7 0.18 55 / 20%);

		&[data-variant='critical'] {
			border-color: oklch(0.55 0.22 25 / 50%);
			background-color: oklch(0.55 0.22 25 / 10%);
		}
	}

	.anomaly-value {
		font-size: var(--text-4xl);
		font-weight: var(--weight-bold);
		color: oklch(0.75 0.16 55);
	}

	[data-variant='critical'] .anomaly-value { color: oklch(0.7 0.2 25); }

	.anomaly-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-2);
	}

	.charts-grid,
	.funnels-grid,
	.behavior-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);
		@media (min-width: 1024px) { grid-template-columns: repeat(2, 1fr); }
	}

	.funnels-list,
	.cohorts-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
		color: oklch(0.45 0.01 250);
	}

	.empty-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-4);
	}

	.empty-subtitle {
		color: oklch(0.55 0.01 250);
		margin-block-start: var(--space-2);
	}

	.empty-action { margin-block-start: var(--space-4); }

	.btn-primary {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.8 0.18 90);
		color: oklch(0.15 0.02 90);
		font-weight: var(--weight-semibold);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.85 0.16 90); }
	}

	.model-selector {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.45 0.01 250);
		&:focus { outline: none; box-shadow: 0 0 0 2px oklch(0.8 0.18 90); }
	}

	.behavior-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.card-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-4);
	}

	.page-list,
	.event-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.page-item,
	.event-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-block: var(--space-2);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
		&:last-child { border-block-end: none; }
	}

	.page-path,
	.event-name {
		color: oklch(0.75 0.01 250);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		flex: 1;
	}

	.page-views,
	.event-count {
		color: oklch(0.8 0.18 90);
		font-weight: var(--weight-semibold);
		margin-inline-start: var(--space-4);
	}

	.revenue-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.revenue-kpis {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
		@media (min-width: 768px) { grid-template-columns: repeat(2, 1fr); }
		@media (min-width: 1024px) { grid-template-columns: repeat(4, 1fr); }
	}

	.revenue-kpi-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.kpi-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-2);
	}

	.kpi-value {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-2);
	}

	.kpi-change {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		&.positive { color: oklch(0.7 0.18 160); }
		&.negative { color: oklch(0.7 0.2 25); }
	}
</style>
