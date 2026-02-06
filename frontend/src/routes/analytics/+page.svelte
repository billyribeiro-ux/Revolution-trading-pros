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
	import {
		IconTrendingUp,
		IconUsers,
		IconCurrencyDollar,
		IconChartBar,
		IconBolt,
		IconTarget,
		IconBrain,
		IconAlertTriangle
	} from '$lib/icons';

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
				<IconChartBar size={32} class="text-yellow-400" />
				<div>
					<h1 class="text-3xl font-bold text-white">Analytics Dashboard</h1>
					<p class="text-gray-400 mt-1">Enterprise-grade analytics and insights</p>
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
			<IconChartBar size={20} />
			Overview
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'funnels'}
			onclick={() => (selectedTab = 'funnels')}
		>
			<IconTarget size={20} />
			Funnels
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'cohorts'}
			onclick={() => (selectedTab = 'cohorts')}
		>
			<IconUsers size={20} />
			Cohorts
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'attribution'}
			onclick={() => (selectedTab = 'attribution')}
		>
			<IconTrendingUp size={20} />
			Attribution
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'behavior'}
			onclick={() => (selectedTab = 'behavior')}
		>
			<IconBolt size={20} />
			Behavior
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'revenue'}
			onclick={() => selectedTab === 'revenue'}
		>
			<IconCurrencyDollar size={20} />
			Revenue
		</button>
	</div>

	<!-- Content -->
	<div class="dashboard-content">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="text-gray-400 mt-4">Loading analytics data...</p>
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
						<div class="section anomaly-section">
							<div class="section-header">
								<div class="flex items-center gap-2">
									<IconAlertTriangle size={24} class="text-orange-400" />
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
								<div class="anomaly-card critical">
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
							<IconTarget size={18} />
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
							<IconTarget size={64} class="text-gray-600" />
							<h3 class="text-xl font-semibold text-gray-400 mt-4">No Funnels Yet</h3>
							<p class="text-gray-500 mt-2">Create your first funnel to track conversion paths</p>
							<button class="btn-primary mt-4">Create Funnel</button>
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
							<IconUsers size={18} />
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
							<IconUsers size={64} class="text-gray-600" />
							<h3 class="text-xl font-semibold text-gray-400 mt-4">No Cohorts Yet</h3>
							<p class="text-gray-500 mt-2">
								Create cohorts to analyze user retention and behavior
							</p>
							<button class="btn-primary mt-4">Create Cohort</button>
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
							<IconTrendingUp size={64} class="text-gray-600" />
							<h3 class="text-xl font-semibold text-gray-400 mt-4">No Attribution Data</h3>
							<p class="text-gray-500 mt-2">
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
				<IconBrain size={64} class="text-gray-600" />
				<h3 class="text-xl font-semibold text-gray-400 mt-4">No Data Available</h3>
				<p class="text-gray-500 mt-2">Analytics data will appear once events are tracked</p>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	@reference "../../app.css";
	.analytics-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
	}

	.dashboard-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.header-title {
		@apply flex items-center gap-4;
	}

	.header-actions {
		@apply flex items-center gap-3;
	}

	.realtime-section {
		@apply mb-8;
	}

	.dashboard-tabs {
		@apply flex gap-2 mb-8 overflow-x-auto pb-2;
		scrollbar-width: thin;
	}

	.tab {
		@apply flex items-center gap-2 px-6 py-3 rounded-lg font-medium transition-all;
		@apply bg-gray-800/50 text-gray-400 hover:bg-gray-700/50 hover:text-white;
		@apply border border-gray-700/50;
		white-space: nowrap;
	}

	.tab.active {
		@apply bg-yellow-500/20 text-yellow-400 border-yellow-500/50;
	}

	.dashboard-content {
		@apply min-h-[400px];
	}

	.loading-state {
		@apply flex flex-col items-center justify-center py-20;
	}

	.spinner {
		@apply w-12 h-12 border-4 border-gray-700 border-t-yellow-400 rounded-full animate-spin;
	}

	.overview-grid {
		@apply space-y-8;
	}

	.section {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50;
	}

	.section-header {
		@apply flex items-center justify-between mb-6;
	}

	.section-title {
		@apply text-2xl font-bold text-white;
	}

	.section-subtitle {
		@apply text-sm text-gray-400 mt-1;
	}

	.view-all-link {
		@apply text-yellow-400 hover:text-yellow-300 font-medium transition-colors;
	}

	.anomaly-section {
		@apply border-orange-500/30 bg-orange-500/5;
	}

	.anomaly-grid {
		@apply grid grid-cols-1 md:grid-cols-3 gap-4;
	}

	.anomaly-card {
		@apply bg-gray-900/50 rounded-lg p-6 text-center border border-orange-500/20;
	}

	.anomaly-card.critical {
		@apply border-red-500/50 bg-red-500/10;
	}

	.anomaly-value {
		@apply text-4xl font-bold text-orange-400;
	}

	.anomaly-card.critical .anomaly-value {
		@apply text-red-400;
	}

	.anomaly-label {
		@apply text-sm text-gray-400 mt-2;
	}

	.charts-grid {
		@apply grid grid-cols-1 lg:grid-cols-2 gap-6;
	}

	.funnels-grid {
		@apply grid grid-cols-1 lg:grid-cols-2 gap-6;
	}

	.funnels-list,
	.cohorts-list {
		@apply space-y-6;
	}

	.empty-state {
		@apply flex flex-col items-center justify-center py-20 text-center;
	}

	.btn-primary {
		@apply flex items-center gap-2 px-6 py-3 bg-yellow-500 text-gray-900 font-semibold rounded-lg;
		@apply hover:bg-yellow-400 transition-colors;
	}

	.model-selector {
		@apply px-4 py-2 bg-gray-700 text-white rounded-lg border border-gray-600;
		@apply focus:outline-none focus:ring-2 focus:ring-yellow-500;
	}

	.behavior-grid {
		@apply grid grid-cols-1 lg:grid-cols-2 gap-6;
	}

	.behavior-card {
		@apply bg-gray-900/50 rounded-lg p-6 border border-gray-700/50;
	}

	.card-title {
		@apply text-lg font-semibold text-white mb-4;
	}

	.page-list,
	.event-list {
		@apply space-y-3;
	}

	.page-item,
	.event-item {
		@apply flex items-center justify-between py-2 border-b border-gray-700/50 last:border-0;
	}

	.page-path,
	.event-name {
		@apply text-gray-300 truncate flex-1;
	}

	.page-views,
	.event-count {
		@apply text-yellow-400 font-semibold ml-4;
	}

	.revenue-grid {
		@apply space-y-6;
	}

	.revenue-kpis {
		@apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4;
	}

	.revenue-kpi-card {
		@apply bg-gray-900/50 rounded-lg p-6 border border-gray-700/50;
	}

	.kpi-label {
		@apply text-sm text-gray-400 mb-2;
	}

	.kpi-value {
		@apply text-3xl font-bold text-white mb-2;
	}

	.kpi-change {
		@apply text-sm font-semibold;
	}

	.kpi-change.positive {
		@apply text-green-400;
	}

	.kpi-change.negative {
		@apply text-red-400;
	}
</style>
