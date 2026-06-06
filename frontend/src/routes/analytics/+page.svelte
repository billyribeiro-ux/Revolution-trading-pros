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
				<IconChartBar size={32} class="analytics-header-icon" />
				<div>
					<h1 class="analytics-title">Analytics Dashboard</h1>
					<p class="analytics-subtitle">Enterprise-grade analytics and insights</p>
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
			class={['tab', { active: selectedTab === 'overview' }]}
			onclick={() => (selectedTab = 'overview')}
		>
			<IconChartBar size={20} />
			Overview
		</button>
		<button
			class={['tab', { active: selectedTab === 'funnels' }]}
			onclick={() => (selectedTab = 'funnels')}
		>
			<IconTarget size={20} />
			Funnels
		</button>
		<button
			class={['tab', { active: selectedTab === 'cohorts' }]}
			onclick={() => (selectedTab = 'cohorts')}
		>
			<IconUsers size={20} />
			Cohorts
		</button>
		<button
			class={['tab', { active: selectedTab === 'attribution' }]}
			onclick={() => (selectedTab = 'attribution')}
		>
			<IconTrendingUp size={20} />
			Attribution
		</button>
		<button
			class={['tab', { active: selectedTab === 'behavior' }]}
			onclick={() => (selectedTab = 'behavior')}
		>
			<IconBolt size={20} />
			Behavior
		</button>
		<button
			class={['tab', { active: selectedTab === 'revenue' }]}
			onclick={() => (selectedTab = 'revenue')}
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
				<p class="state-text state-text--spaced">Loading analytics data...</p>
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
								<div class="anomaly-heading">
									<IconAlertTriangle size={24} class="anomaly-icon" />
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
								{#each dashboard.funnels.slice(0, 2) as funnel (funnel.name)}
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
							{#each dashboard.funnels as funnel (funnel.name)}
								<FunnelChart steps={funnel.steps} title={funnel.name} />
							{/each}
						</div>
					{:else}
						<div class="empty-state">
							<IconTarget size={64} class="empty-icon" />
							<h3 class="empty-title">No Funnels Yet</h3>
							<p class="empty-copy">Create your first funnel to track conversion paths</p>
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
							<IconUsers size={18} />
							Create Cohort
						</button>
					</div>

					{#if dashboard.cohorts && dashboard.cohorts.length > 0}
						<div class="cohorts-list">
							{#each dashboard.cohorts as cohort (cohort.name)}
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
							<IconUsers size={64} class="empty-icon" />
							<h3 class="empty-title">No Cohorts Yet</h3>
							<p class="empty-copy">Create cohorts to analyze user retention and behavior</p>
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
							<IconTrendingUp size={64} class="empty-icon" />
							<h3 class="empty-title">No Attribution Data</h3>
							<p class="empty-copy">Attribution data will appear once conversions are tracked</p>
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
									{#each dashboard.top_pages as page (page.page_path)}
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
									{#each dashboard.top_events as event (event.event_name)}
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
							{#each dashboard.kpis.filter((k) => k.category === 'revenue') as kpi (kpi.name)}
								<div class="revenue-kpi-card">
									<div class="kpi-label">{kpi.name}</div>
									<div class="kpi-value">{kpi.formatted_value}</div>
									<div
										class={[
											'kpi-change',
											{
												positive: kpi.trend === 'up',
												negative: kpi.trend === 'down'
											}
										]}
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
				<IconBrain size={64} class="empty-icon" />
				<h3 class="empty-title">No Data Available</h3>
				<p class="empty-copy">Analytics data will appear once events are tracked</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.analytics-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
		min-height: 100%;
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
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-title :global(.analytics-header-icon) {
		color: #facc15;
	}

	.analytics-title {
		margin: 0;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
		color: #ffffff;
	}

	.analytics-subtitle {
		margin: 0.25rem 0 0;
		color: #9ca3af;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.realtime-section {
		margin-bottom: 2rem;
	}

	.dashboard-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
		scrollbar-width: thin;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(31, 41, 55, 0.5);
		color: #9ca3af;
		font: inherit;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease,
			color 0.2s ease;
		white-space: nowrap;
	}

	.tab:hover {
		background: rgba(55, 65, 81, 0.5);
		color: #ffffff;
	}

	.tab.active {
		border-color: rgba(234, 179, 8, 0.5);
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}

	.dashboard-content {
		min-height: 400px;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
	}

	.state-text {
		margin: 0;
		color: #9ca3af;
	}

	.state-text--spaced {
		margin-top: 1rem;
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		border: 4px solid #374151;
		border-top-color: #facc15;
		border-radius: 999px;
		animation: analytics-spin 1s linear infinite;
	}

	.overview-grid {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.section {
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.section-title {
		margin: 0;
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
		color: #ffffff;
	}

	.section-subtitle {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.view-all-link {
		color: #facc15;
		font-weight: 500;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.view-all-link:hover {
		color: #fde047;
	}

	.anomaly-section {
		border-color: rgba(249, 115, 22, 0.3);
		background: rgba(249, 115, 22, 0.05);
	}

	.anomaly-heading {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.anomaly-heading :global(.anomaly-icon) {
		color: #fb923c;
	}

	.anomaly-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.anomaly-card {
		padding: 1.5rem;
		border: 1px solid rgba(249, 115, 22, 0.2);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
		text-align: center;
	}

	.anomaly-card.critical {
		border-color: rgba(239, 68, 68, 0.5);
		background: rgba(239, 68, 68, 0.1);
	}

	.anomaly-value {
		font-size: 2.25rem;
		line-height: 2.5rem;
		font-weight: 700;
		color: #fb923c;
	}

	.anomaly-card.critical .anomaly-value {
		color: #f87171;
	}

	.anomaly-label {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.charts-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.funnels-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.funnels-list,
	.cohorts-list {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
	}

	.empty-state :global(.empty-icon) {
		color: #4b5563;
	}

	.empty-title {
		margin: 1rem 0 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #9ca3af;
	}

	.empty-copy {
		margin: 0.5rem 0 0;
		color: #6b7280;
	}

	.empty-action {
		margin-top: 1rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: #eab308;
		color: #111827;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-primary:hover {
		background: #facc15;
	}

	.model-selector {
		padding: 0.5rem 1rem;
		border: 1px solid #4b5563;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
	}

	.model-selector:focus {
		outline: 2px solid #eab308;
		outline-offset: 2px;
	}

	.behavior-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.behavior-card {
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.card-title {
		margin: 0 0 1rem;
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.page-list,
	.event-list {
		display: flex;
		flex-direction: column;
	}

	.page-item,
	.event-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding-block: 0.5rem;
		border-bottom: 1px solid rgba(55, 65, 81, 0.5);
	}

	.page-item:last-child,
	.event-item:last-child {
		border-bottom: 0;
	}

	.page-path,
	.event-name {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		color: #d1d5db;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.page-views,
	.event-count {
		color: #facc15;
		font-weight: 600;
		white-space: nowrap;
	}

	.revenue-grid {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.revenue-kpis {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.revenue-kpi-card {
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.kpi-label {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.kpi-value {
		margin-bottom: 0.5rem;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
		color: #ffffff;
	}

	.kpi-change {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.kpi-change.positive {
		color: #4ade80;
	}

	.kpi-change.negative {
		color: #f87171;
	}

	@media (min-width: 768px) {
		.anomaly-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.revenue-kpis {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.charts-grid,
		.funnels-grid,
		.behavior-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.revenue-kpis {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@keyframes analytics-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
