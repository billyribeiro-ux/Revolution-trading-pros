<script lang="ts">
	/**
	 * Analytics Dashboard - Apple ICT9+ Enterprise Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Premium analytics dashboard with:
	 * - Connection status awareness
	 * - Real data only (no fake numbers)
	 * - Apple-level design and animations
	 * - KPIs, funnels, cohorts, attribution
	 */

	import { browser } from '$app/environment';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { analyticsApi, type DashboardData } from '$lib/api/analytics';
	import {
		connections,
		getIsAnalyticsConnected,
		FEATURE_SERVICES,
		SERVICE_KEYS
	} from '$lib/stores/connections.svelte';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import KpiGrid from '$lib/components/analytics/KpiGrid.svelte';
	import FunnelChart from '$lib/components/analytics/FunnelChart.svelte';
	import CohortMatrix from '$lib/components/analytics/CohortMatrix.svelte';
	import TimeSeriesChart from '$lib/components/analytics/TimeSeriesChart.svelte';
	import RealTimeWidget from '$lib/components/analytics/RealTimeWidget.svelte';
	import AttributionChart from '$lib/components/analytics/AttributionChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	import ExportButton from '$lib/components/ExportButton.svelte';
	import { IconPlugConnected, IconRefresh, IconArrowRight } from '$lib/icons';

	// State
	let dashboardData: DashboardData | null = $state(null);
	let loading = $state(true);
	let connectionsLoading = $state(true);
	let error: string | null = $state(null);
	let selectedPeriod = $state('30d');
	let attributionModel = $state('linear');
	let isConnected = $state(false);

	// Helper to get KPI value by key
	function getKpiValue(
		kpis: import('$lib/api/analytics').KpiValue[] | undefined,
		key: string
	): number {
		if (!kpis) return 0;
		const kpi = kpis.find((k) => k.kpi_key === key);
		return kpi?.value || 0;
	}

	// Derived export data
	let exportData = $derived.by(() => {
		if (!dashboardData) return [];

		const data = dashboardData;
		return [
			{
				metric: 'Total Users',
				value: getKpiValue(data.kpis, 'total_users'),
				period: selectedPeriod
			},
			{
				metric: 'Total Revenue',
				value: getKpiValue(data.kpis, 'total_revenue'),
				period: selectedPeriod
			},
			{
				metric: 'Conversion Rate',
				value: getKpiValue(data.kpis, 'conversion_rate'),
				period: selectedPeriod
			},
			{
				metric: 'Average Order Value',
				value: getKpiValue(data.kpis, 'avg_order_value'),
				period: selectedPeriod
			},
			...(data.time_series?.revenue || []).map((item: { date: string; value: number }) => ({
				type: 'Revenue',
				date: item.date,
				value: item.value
			})),
			...(data.time_series?.users || []).map((item: { date: string; value: number }) => ({
				type: 'Users',
				date: item.date,
				value: item.value
			}))
		];
	});

	// Navigation tabs
	const tabs = [
		{ id: 'overview', label: 'Overview', icon: '📊' },
		{ id: 'funnels', label: 'Funnels', icon: '🔻' },
		{ id: 'cohorts', label: 'Cohorts', icon: '👥' },
		{ id: 'attribution', label: 'Attribution', icon: '🎯' }
	];
	let activeTab = $state('overview');

	// Analytics services that can be connected - dynamically generated from FEATURE_SERVICES
	const analyticsServices =
		FEATURE_SERVICES['analytics']?.map((key) => {
			const serviceConfig: Record<string, { name: string; icon: string; color: string }> = {
				[SERVICE_KEYS.GOOGLE_ANALYTICS]: { name: 'Google Analytics', icon: '📊', color: '#E6B800' },
				[SERVICE_KEYS.MIXPANEL]: { name: 'Mixpanel', icon: '📈', color: '#B38F00' },
				[SERVICE_KEYS.AMPLITUDE]: { name: 'Amplitude', icon: '📉', color: '#3b82f6' },
				[SERVICE_KEYS.SEGMENT]: { name: 'Segment', icon: '🔗', color: '#10b981' },
				[SERVICE_KEYS.PLAUSIBLE]: { name: 'Plausible', icon: '🌿', color: '#14b8a6' }
			};
			return { key, ...serviceConfig[key] };
		}) || [];

	async function loadDashboard() {
		loading = true;
		error = null;
		try {
			dashboardData = await analyticsApi.getDashboard(selectedPeriod);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load dashboard';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadDashboard();
	}

	// Svelte 5: Initialize on mount with cleanup
	$effect(() => {
		if (!browser) return;

		// Check analytics connection status
		isConnected = getIsAnalyticsConnected();

		// Load connection status
		const initDashboard = async () => {
			connectionsLoading = true;
			await connections.load();
			connectionsLoading = false;

			// Re-check after load
			isConnected = getIsAnalyticsConnected();

			// Load dashboard if connected
			if (isConnected) {
				loadDashboard();
			}
		};
		initDashboard();
	});

	// Format time series data
	let revenueTimeSeries = $derived.by(() => {
		if (!dashboardData?.time_series?.revenue) return [];
		return dashboardData.time_series.revenue.map((item: { date: string; value: number }) => ({
			date: item.date,
			value: item.value,
			label: '$' + item.value.toLocaleString()
		}));
	});

	let usersTimeSeries = $derived.by(() => {
		if (!dashboardData?.time_series?.users) return [];
		return dashboardData.time_series.users.map((item: { date: string; value: number }) => ({
			date: item.date,
			value: item.value
		}));
	});
</script>

<svelte:head>
	<title>Analytics Dashboard | Revolution Trading</title>
</svelte:head>

<div class="analytics-dashboard">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header -->
		<header class="page-header" in:fly={{ y: -20, duration: 500, easing: quintOut }}>
			<h1>Analytics Dashboard</h1>
			<p class="subtitle">Enterprise insights and performance metrics</p>
			{#if isConnected}
				<div class="header-actions">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<ExportButton
						data={exportData}
						filename="analytics-report-{selectedPeriod}"
						formats={['csv', 'json']}
						label="Export"
						disabled={loading || !dashboardData}
					/>
					<a href="/admin/analytics/events" class="btn-primary">
						Event Explorer
						<IconArrowRight size={18} />
					</a>
				</div>
			{/if}

			<!-- Connection Status Badge -->
			<div class="connection-status" class:connected={isConnected}>
				<IconPlugConnected size={16} />
				<span>{isConnected ? 'Analytics Connected' : 'Not Connected'}</span>
			</div>

			<!-- Tab Navigation (only show when connected) -->
			{#if isConnected}
				<nav class="tab-nav" in:fade={{ duration: 300, delay: 200 }}>
					{#each tabs as tab}
						<button
							class="tab-btn"
							class:active={activeTab === tab.id}
							onclick={() => (activeTab = tab.id)}
						>
							<span class="tab-icon">{tab.icon}</span>
							{tab.label}
						</button>
					{/each}
				</nav>
			{/if}
		</header>

		<!-- Main Content -->
		<section class="dashboard-content">
			{#if connectionsLoading}
				<!-- Loading Connections -->
				<div class="loading-state" in:fade={{ duration: 300 }}>
					<div class="loading-spinner"></div>
					<p>Checking connections...</p>
				</div>
			{:else if !isConnected}
				<!-- Not Connected State -->
				<div class="not-connected-state" in:scale={{ duration: 400, start: 0.95 }}>
					<ApiNotConnected
						serviceName="Analytics"
						description="Connect an analytics service to view real-time data, track user behavior, and measure conversions."
						icon="📊"
						color="#E6B800"
						features={[
							'Real-time visitor tracking',
							'User behavior analytics',
							'Conversion funnel analysis',
							'Cohort retention reports',
							'Channel attribution'
						]}
					/>

					<!-- Available Services -->
					<div class="available-services" in:fly={{ y: 20, duration: 400, delay: 200 }}>
						<h3>Available Analytics Services</h3>
						<div class="services-grid">
							{#each analyticsServices as service}
								<a
									href="/admin/connections?connect={service.key}"
									class="service-card"
									style="--service-color: {service.color}"
								>
									<span class="service-icon">{service.icon}</span>
									<span class="service-name">{service.name}</span>
									<IconArrowRight size={16} class="service-arrow" />
								</a>
							{/each}
						</div>
					</div>
				</div>
			{:else if loading}
				<!-- Loading Dashboard -->
				<div class="loading-state" in:fade={{ duration: 300 }}>
					<div class="loading-spinner"></div>
					<p>Loading analytics data...</p>
				</div>
			{:else if error}
				<!-- Error State -->
				<div class="error-state" in:fade={{ duration: 300 }}>
					<div class="error-icon">⚠️</div>
					<h3>Unable to Load Analytics</h3>
					<p>{error}</p>
					<button class="btn-retry" onclick={loadDashboard}>
						<IconRefresh size={18} />
						Try Again
					</button>
				</div>
			{:else if dashboardData}
				<!-- Dashboard Content - Layout Shift Free Pattern -->
				<div class="dashboard-grid" in:fade={{ duration: 400 }}>
					<!-- Overview Panel -->
					<div
						class="tab-panel"
						class:active={activeTab === 'overview'}
						inert={activeTab !== 'overview' ? true : undefined}
					>
						<div class="tab-content">
							<!-- Real-time Widget -->
							<div class="grid-row">
								<div class="widget-large">
									<RealTimeWidget />
								</div>
								<div class="widget-small">
									<div class="quick-actions">
										<h3>Quick Actions</h3>
										<a href="/admin/analytics/segments" class="action-link">
											<span class="action-icon">👥</span>
											<div class="action-text">
												<strong>Segments</strong>
												<span>Manage user segments</span>
											</div>
										</a>
										<a href="/admin/analytics/goals" class="action-link">
											<span class="action-icon">🎯</span>
											<div class="action-text">
												<strong>Goals</strong>
												<span>Track conversion goals</span>
											</div>
										</a>
										<a href="/admin/analytics/reports" class="action-link">
											<span class="action-icon">📈</span>
											<div class="action-text">
												<strong>Reports</strong>
												<span>Custom report builder</span>
											</div>
										</a>
									</div>
								</div>
							</div>

							<!-- KPI Grid -->
							{#if dashboardData.kpis && dashboardData.kpis.length > 0}
								<KpiGrid kpis={dashboardData.kpis} />
							{/if}

							<!-- Time Series Charts -->
							{#if revenueTimeSeries.length > 0 || usersTimeSeries.length > 0}
								<div class="charts-row">
									{#if revenueTimeSeries.length > 0}
										<TimeSeriesChart
											data={revenueTimeSeries}
											title="Revenue Trend"
											color="#10B981"
											formatValue={(v) => '$' + v.toLocaleString()}
										/>
									{/if}
									{#if usersTimeSeries.length > 0}
										<TimeSeriesChart
											data={usersTimeSeries}
											title="Active Users"
											color="#3B82F6"
											formatValue={(v) => v.toLocaleString()}
										/>
									{/if}
								</div>
							{/if}

							<!-- Top Pages & Events -->
							<div class="data-tables">
								<div class="data-table">
									<h3>Top Pages</h3>
									{#if dashboardData.top_pages && dashboardData.top_pages.length > 0}
										<div class="table-rows">
											{#each dashboardData.top_pages.slice(0, 10) as page, i}
												<div class="table-row">
													<span class="row-rank">{i + 1}</span>
													<span class="row-label">{page.page_path}</span>
													<span class="row-value">{page.views.toLocaleString()}</span>
												</div>
											{/each}
										</div>
									{:else}
										<p class="no-data">No page data available yet</p>
									{/if}
								</div>

								<div class="data-table">
									<h3>Top Events</h3>
									{#if dashboardData.top_events && dashboardData.top_events.length > 0}
										<div class="table-rows">
											{#each dashboardData.top_events.slice(0, 10) as event, i}
												<div class="table-row">
													<span class="row-rank">{i + 1}</span>
													<span class="row-label">{event.event_name}</span>
													<span class="row-value">{event.count.toLocaleString()}</span>
												</div>
											{/each}
										</div>
									{:else}
										<p class="no-data">No event data available yet</p>
									{/if}
								</div>
							</div>
						</div>
					</div>

					<!-- Funnels Panel -->
					<div
						class="tab-panel"
						class:active={activeTab === 'funnels'}
						inert={activeTab !== 'funnels' ? true : undefined}
					>
						<div class="tab-content">
							<div class="tab-header">
								<h2>Conversion Funnels</h2>
								<a href="/admin/analytics/funnels/create" class="btn-primary"> Create Funnel </a>
							</div>

							{#if dashboardData.funnels && dashboardData.funnels.length > 0}
								<div class="funnels-grid">
									{#each dashboardData.funnels as funnel}
										<FunnelChart steps={funnel.steps} title={funnel.name} showDropOff={true} />
									{/each}
								</div>
							{:else}
								<div class="empty-state">
									<span class="empty-icon">🔻</span>
									<h3>No Funnels Yet</h3>
									<p>Create your first funnel to track user journeys</p>
									<a href="/admin/analytics/funnels/create" class="btn-primary">
										Create First Funnel
									</a>
								</div>
							{/if}
						</div>
					</div>

					<!-- Cohorts Panel -->
					<div
						class="tab-panel"
						class:active={activeTab === 'cohorts'}
						inert={activeTab !== 'cohorts' ? true : undefined}
					>
						<div class="tab-content">
							<div class="tab-header">
								<h2>Cohort Analysis</h2>
								<div class="tab-actions">
									<select class="select-input">
										<option value="weekly">Weekly Cohorts</option>
										<option value="monthly">Monthly Cohorts</option>
										<option value="daily">Daily Cohorts</option>
									</select>
									<a href="/admin/analytics/cohorts/create" class="btn-primary"> Create Cohort </a>
								</div>
							</div>

							{#if dashboardData.cohorts && dashboardData.cohorts.length > 0}
								{#each dashboardData.cohorts as cohort}
									<CohortMatrix data={cohort.retention_matrix} title={cohort.name} />
								{/each}
							{:else}
								<div class="empty-state">
									<span class="empty-icon">👥</span>
									<h3>No Cohort Data</h3>
									<p>Start tracking user retention with cohort analysis</p>
									<a href="/admin/analytics/cohorts/create" class="btn-primary">
										Create First Cohort
									</a>
								</div>
							{/if}
						</div>
					</div>

					<!-- Attribution Panel -->
					<div
						class="tab-panel"
						class:active={activeTab === 'attribution'}
						inert={activeTab !== 'attribution' ? true : undefined}
					>
						<div class="tab-content">
							<div class="tab-header">
								<h2>Channel Attribution</h2>
								<select bind:value={attributionModel} class="select-input">
									<option value="first_touch">First Touch</option>
									<option value="last_touch">Last Touch</option>
									<option value="linear">Linear</option>
									<option value="time_decay">Time Decay</option>
									<option value="position_based">Position Based</option>
								</select>
							</div>

							{#if dashboardData.attribution && dashboardData.attribution.channels}
								<AttributionChart
									channels={dashboardData.attribution.channels}
									model={attributionModel}
								/>

								<div class="attribution-table">
									<h3>Model Comparison</h3>
									<div class="table-wrapper">
										<table>
											<thead>
												<tr>
													<th>Channel</th>
													<th>First Touch</th>
													<th>Last Touch</th>
													<th>Linear</th>
													<th>Time Decay</th>
													<th>Position Based</th>
												</tr>
											</thead>
											<tbody>
												{#each dashboardData.attribution.channels as channel}
													<tr>
														<td class="channel-name">{channel.channel}</td>
														<td>{channel.first_touch_share?.toFixed(1) || '-'}%</td>
														<td>{channel.last_touch_share?.toFixed(1) || '-'}%</td>
														<td>{channel.linear_share?.toFixed(1) || '-'}%</td>
														<td>{channel.time_decay_share?.toFixed(1) || '-'}%</td>
														<td>{channel.position_based_share?.toFixed(1) || '-'}%</td>
													</tr>
												{/each}
											</tbody>
										</table>
									</div>
								</div>
							{:else}
								<div class="empty-state">
									<span class="empty-icon">🎯</span>
									<h3>No Attribution Data</h3>
									<p>Attribution data will appear once conversions are tracked</p>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/if}
		</section>
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.analytics-dashboard {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		color: white;
		position: relative;
		overflow: hidden;
	}

	/* Background Effects */
	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, #3b82f6, var(--primary-600));
		animation: float 25s ease-in-out infinite reverse;
	}

	.bg-blob-3 {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, #10b981, #14b8a6);
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Header */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
		position: relative;
		z-index: 10;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin-bottom: 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	/* Connection Status */
	.connection-status {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		color: #f87171;
		margin-top: 1rem;
		max-width: 1400px;
		width: fit-content;
	}

	.connection-status.connected {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	/* Tab Navigation */
	.tab-nav {
		display: flex;
		gap: 0.25rem;
		margin-top: 1.5rem;
		max-width: 1400px;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
		padding-bottom: 0;
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: -1px;
	}

	.tab-btn:hover {
		color: #f1f5f9;
	}

	.tab-btn.active {
		color: var(--primary-500);
		border-bottom-color: var(--primary-500);
	}

	.tab-icon {
		font-size: 1rem;
	}

	/* Main Content */
	.dashboard-content {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 6rem 2rem;
		gap: 1.5rem;
	}

	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Not Connected State */
	.not-connected-state {
		padding: 2rem 0;
	}

	.available-services {
		margin-top: 3rem;
	}

	.available-services h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #94a3b8;
		margin-bottom: 1.25rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.services-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.service-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		text-decoration: none;
		color: #f1f5f9;
		transition: all 0.2s;
	}

	.service-card:hover {
		background: rgba(30, 41, 59, 0.8);
		border-color: var(--service-color, var(--primary-500));
		transform: translateY(-2px);
	}

	.service-icon {
		font-size: 1.5rem;
	}

	.service-name {
		flex: 1;
		font-weight: 500;
	}

	.service-card :global(.service-arrow) {
		color: #64748b;
		transition: all 0.2s;
	}

	.service-card:hover :global(.service-arrow) {
		color: var(--service-color, var(--primary-500));
		transform: translateX(4px);
	}

	/* Error State */
	.error-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(239, 68, 68, 0.05);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 16px;
	}

	.error-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.error-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.error-state p {
		color: #94a3b8;
		margin: 0 0 1.5rem;
	}

	.btn-retry {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #f87171;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-retry:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	/* Dashboard Grid - Layout Shift Prevention */
	.dashboard-grid {
		position: relative;
		min-height: 500px;
		contain: layout style;
		isolation: isolate;
	}

	/* Tab Panels - CSS visibility toggling eliminates layout shift */
	.tab-panel {
		position: absolute;
		inset: 0;
		width: 100%;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition:
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	.tab-panel.active {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		.tab-panel {
			transition: none;
			transform: none;
		}
		.tab-panel:not(.active) {
			display: none;
		}
	}

	.tab-content {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.tab-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.tab-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.tab-actions {
		display: flex;
		gap: 1rem;
	}

	.select-input {
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.grid-row {
		display: grid;
		grid-template-columns: 2fr 1fr;
		gap: 1.5rem;
	}

	.widget-large,
	.widget-small {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.quick-actions {
		padding: 1.5rem;
	}

	.quick-actions h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 1rem;
		color: #f1f5f9;
	}

	.action-link {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem;
		border-radius: 10px;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}

	.action-link:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.action-icon {
		font-size: 1.5rem;
	}

	.action-text strong {
		display: block;
		color: #f1f5f9;
		font-weight: 500;
	}

	.action-text span {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.charts-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	.data-tables {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	.data-table {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.data-table h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 1rem;
	}

	.table-rows {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.table-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.table-row:last-child {
		border-bottom: none;
	}

	.row-rank {
		width: 24px;
		font-size: 0.8125rem;
		color: #64748b;
	}

	.row-label {
		flex: 1;
		font-size: 0.875rem;
		color: #f1f5f9;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.row-value {
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.no-data {
		color: #64748b;
		font-size: 0.875rem;
		text-align: center;
		padding: 2rem;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.empty-icon {
		font-size: 3rem;
		display: block;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 0.5rem;
	}

	.empty-state p {
		color: #64748b;
		margin: 0 0 1.5rem;
	}

	/* Funnels Grid */
	.funnels-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	/* Attribution Table */
	.attribution-table {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.attribution-table h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 1rem;
	}

	.table-wrapper {
		overflow-x: auto;
	}

	.attribution-table table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.attribution-table th {
		text-align: right;
		padding: 0.75rem;
		color: #64748b;
		font-weight: 500;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.attribution-table th:first-child {
		text-align: left;
	}

	.attribution-table td {
		padding: 0.75rem;
		text-align: right;
		color: #94a3b8;
		border-bottom: 1px solid rgba(148, 163, 184, 0.05);
	}

	.attribution-table td.channel-name {
		text-align: left;
		color: #f1f5f9;
		font-weight: 500;
		text-transform: capitalize;
	}

	.attribution-table tr:hover {
		background: rgba(255, 255, 255, 0.02);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple ICT7 Principal Engineer Grade
	 * Mobile-first approach with progressive enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet Landscape (< 1024px) */
	@media (max-width: 1024px) {
		.grid-row {
			grid-template-columns: 1fr;
		}

		.charts-row,
		.data-tables,
		.funnels-grid {
			grid-template-columns: 1fr;
		}
	}

	/* Mobile Landscape (< 768px) */
	@media (max-width: 768px) {
		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.dashboard-content {
			padding: 1rem;
		}

		.services-grid {
			grid-template-columns: 1fr;
		}
	}

	/* Mobile Portrait (< 640px) */
	@media (max-width: 640px) {
		.dashboard-content {
			padding: 0.75rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.widget-large,
		.widget-small,
		.data-table {
			padding: 1rem;
		}

		.btn-primary,
		.btn-retry {
			padding: 0.5rem 0.75rem;
			font-size: 0.8125rem;
		}

		.attribution-table {
			font-size: 0.8125rem;
		}
	}

	/* Extra Small Mobile (< 380px) - iPhone SE, Galaxy Fold */
	@media (max-width: 380px) {
		.dashboard-content {
			padding: 0.5rem;
		}

		.page-header h1 {
			font-size: 1.25rem;
		}

		.widget-large,
		.widget-small {
			padding: 0.75rem;
		}

		.data-table {
			padding: 0.75rem;
		}

		.attribution-table th,
		.attribution-table td {
			padding: 0.5rem;
		}
	}

	/* Touch Device Optimizations - Apple HIG 44pt minimum */
	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-retry,
		.tab-btn {
			min-height: 44px;
			min-width: 44px;
		}

		.widget-large,
		.widget-small,
		.data-table {
			min-height: 80px;
		}

		.attribution-table td,
		.attribution-table th {
			padding: 0.75rem 1rem;
		}

		select {
			min-height: 48px;
			font-size: 16px; /* Prevents iOS zoom */
		}
	}

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.btn-primary,
		.btn-retry,
		.service-card,
		.action-link {
			transition: none;
		}

		.service-card:hover,
		.action-link:hover {
			transform: none;
		}
	}

	/* High Contrast Mode - Accessibility */
	@media (prefers-contrast: high) {
		.data-table,
		.widget-large,
		.page-header,
		.attribution-table {
			border-width: 2px;
		}

		.page-header h1 {
			font-weight: 800;
		}
	}

	/* Print Styles */
	@media print {
		.page-header,
		.dashboard-content {
			background: white;
			color: black;
		}

		.header-actions {
			display: none !important;
		}

		.widget-large,
		.widget-small,
		.data-table {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid #ccc;
		}

		.attribution-table {
			box-shadow: none;
			border: 1px solid #ccc;
		}
	}

	/* Landscape Mode - Short viewport */
	@media (max-height: 500px) and (orientation: landscape) {
		.page-header,
		.dashboard-content {
			padding: 0.5rem 1rem;
		}

		.grid-row,
		.charts-row {
			gap: 0.5rem;
		}

		.widget-large,
		.widget-small {
			padding: 0.75rem;
		}
	}
</style>
