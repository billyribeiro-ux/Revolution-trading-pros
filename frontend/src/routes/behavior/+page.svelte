<!--
	URL: /behavior
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { dashboardData, isLoading } from '$lib/stores/behavior.svelte';
	import { behaviorAPI } from '$lib/api/behavior';
	import {
		IconBrain,
		IconAlertTriangle,
		IconTarget,
		IconTrendingUp,
		IconUsers,
		IconChartBar
	} from '$lib/icons';

	let selectedPeriod = $state('7d');

	// Local derived from store getters
	let dashboard = $derived(dashboardData.value);
	let loading = $derived(isLoading.value);

	onMount(() => {
		loadDashboard();
	});

	async function loadDashboard() {
		isLoading.set(true);
		try {
			const data = await behaviorAPI.getDashboard(selectedPeriod);
			dashboardData.set(data);
		} catch (error) {
			console.error('Failed to load behavior dashboard:', error);
		} finally {
			isLoading.set(false);
		}
	}

	function handlePeriodChange(period: string) {
		selectedPeriod = period;
		loadDashboard();
	}
</script>

<svelte:head>
	<title>Behavior Analytics | Revolution Trading Pros</title>
	<meta name="description" content="Enterprise behavioral analytics dashboard" />
</svelte:head>

<div class="behavior-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-content">
			<div class="header-title">
				<IconBrain size={32} class="text-purple-400" />
				<div>
					<h1 class="text-3xl font-bold text-white">Behavior Analytics</h1>
					<p class="text-gray-400 mt-1">RevolutionBehavior-L8-System</p>
				</div>
			</div>

			<div class="period-selector">
				<button
					class="period-btn"
					class:active={selectedPeriod === '24h'}
					onclick={() => handlePeriodChange('24h')}
				>
					24h
				</button>
				<button
					class="period-btn"
					class:active={selectedPeriod === '7d'}
					onclick={() => handlePeriodChange('7d')}
				>
					7d
				</button>
				<button
					class="period-btn"
					class:active={selectedPeriod === '30d'}
					onclick={() => handlePeriodChange('30d')}
				>
					30d
				</button>
				<button
					class="period-btn"
					class:active={selectedPeriod === '90d'}
					onclick={() => handlePeriodChange('90d')}
				>
					90d
				</button>
			</div>
		</div>
	</div>

	<!-- Content -->
	{#if loading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p class="text-gray-400 mt-4">Loading behavior data...</p>
		</div>
	{:else if dashboard}
		<!-- Overview KPIs -->
		<div class="kpi-grid">
			<div class="kpi-card">
				<div class="kpi-icon bg-blue-500/20">
					<IconUsers size={24} class="text-blue-400" />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Total Sessions</div>
					<div class="kpi-value">{dashboard.overview.total_sessions.toLocaleString()}</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon bg-green-500/20">
					<IconTrendingUp size={24} class="text-green-400" />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Engagement</div>
					<div class="kpi-value">{dashboard.overview.avg_engagement_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon bg-yellow-500/20">
					<IconTarget size={24} class="text-yellow-400" />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Intent</div>
					<div class="kpi-value">{dashboard.overview.avg_intent_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon bg-orange-500/20">
					<IconAlertTriangle size={24} class="text-orange-400" />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Friction</div>
					<div class="kpi-value">{dashboard.overview.avg_friction_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card critical">
				<div class="kpi-icon bg-red-500/20">
					<IconAlertTriangle size={24} class="text-red-400" />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">High Churn Risk</div>
					<div class="kpi-value">{dashboard.overview.high_churn_risk_count}</div>
				</div>
			</div>
		</div>

		<!-- Friction Heatmap -->
		{#if dashboard.friction_heatmap && dashboard.friction_heatmap.length > 0}
			<div class="section">
				<div class="section-header">
					<h2 class="section-title">Friction Heatmap</h2>
					<p class="section-subtitle">Pages with highest friction</p>
				</div>

				<div class="friction-list">
					{#each dashboard.friction_heatmap as item}
						<div class="friction-item">
							<div class="friction-info">
								<div class="friction-url">{item.page_url}</div>
								<div class="friction-meta">
									<span class="badge badge-orange">{item.top_friction_type}</span>
									<span class="text-gray-400">{item.friction_count} issues</span>
								</div>
							</div>
							<div class="friction-bar">
								<div
									class="friction-fill"
									style="width: {Math.min((item.friction_count / 10) * 100, 100)}%"
								></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Session Timeline -->
		{#if dashboard.session_timeline && dashboard.session_timeline.length > 0}
			<div class="section">
				<div class="section-header">
					<h2 class="section-title">Session Activity</h2>
					<p class="section-subtitle">Daily session volume and engagement</p>
				</div>

				<div class="timeline-chart">
					{#each dashboard.session_timeline as point}
						<div class="timeline-bar">
							<div
								class="bar-fill"
								style="height: {(point.sessions /
									Math.max(...dashboard.session_timeline.map((p) => p.sessions))) *
									100}%"
								title="{point.sessions} sessions, {point.avg_engagement.toFixed(1)}% engagement"
							></div>
							<div class="bar-label">
								{new Date(point.timestamp).toLocaleDateString('en-US', {
									month: 'short',
									day: 'numeric'
								})}
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	{:else}
		<div class="empty-state">
			<IconChartBar size={64} class="text-gray-600" />
			<h3 class="text-xl font-semibold text-gray-400 mt-4">No Behavior Data</h3>
			<p class="text-gray-500 mt-2">Start tracking user behavior to see insights</p>
		</div>
	{/if}
</div>

<style lang="postcss">
	@reference "../../app.css";
	.behavior-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
	}

	.dashboard-header {
		@apply mb-8;
	}

	.header-content {
		@apply flex items-center justify-between flex-wrap gap-4;
	}

	.header-title {
		@apply flex items-center gap-4;
	}

	.period-selector {
		@apply flex gap-2;
	}

	.period-btn {
		@apply px-4 py-2 rounded-lg font-medium transition-all;
		@apply bg-gray-800/50 text-gray-400 hover:bg-gray-700/50 hover:text-white;
		@apply border border-gray-700/50;
	}

	.period-btn.active {
		@apply bg-purple-500/20 text-purple-400 border-purple-500/50;
	}

	.loading-state {
		@apply flex flex-col items-center justify-center py-20;
	}

	.spinner {
		@apply w-12 h-12 border-4 border-gray-700 border-t-purple-400 rounded-full animate-spin;
	}

	.kpi-grid {
		@apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4 mb-8;
	}

	.kpi-card {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50;
		@apply flex items-center gap-4;
	}

	.kpi-card.critical {
		@apply border-red-500/30 bg-red-500/5;
	}

	.kpi-icon {
		@apply w-12 h-12 rounded-lg flex items-center justify-center;
	}

	.kpi-content {
		@apply flex-1;
	}

	.kpi-label {
		@apply text-sm text-gray-400 mb-1;
	}

	.kpi-value {
		@apply text-2xl font-bold text-white;
	}

	.section {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50 mb-8;
	}

	.section-header {
		@apply mb-6;
	}

	.section-title {
		@apply text-2xl font-bold text-white;
	}

	.section-subtitle {
		@apply text-sm text-gray-400 mt-1;
	}

	.friction-list {
		@apply space-y-4;
	}

	.friction-item {
		@apply bg-gray-900/50 rounded-lg p-4;
	}

	.friction-info {
		@apply flex items-start justify-between mb-3;
	}

	.friction-url {
		@apply text-white font-medium truncate flex-1;
	}

	.friction-meta {
		@apply flex items-center gap-2 text-sm;
	}

	.badge {
		@apply px-2 py-1 rounded text-xs font-semibold;
	}

	.badge-orange {
		@apply bg-orange-500/20 text-orange-400;
	}

	.friction-bar {
		@apply h-2 bg-gray-700 rounded-full overflow-hidden;
	}

	.friction-fill {
		@apply h-full bg-gradient-to-r from-orange-500 to-red-500;
	}

	.timeline-chart {
		@apply flex items-end gap-2 h-64;
	}

	.timeline-bar {
		@apply flex-1 flex flex-col items-center gap-2;
	}

	.bar-fill {
		@apply w-full bg-gradient-to-t from-purple-500 to-purple-400 rounded-t;
		@apply transition-all hover:from-purple-400 hover:to-purple-300;
	}

	.bar-label {
		@apply text-xs text-gray-400 rotate-45 origin-left;
	}

	.empty-state {
		@apply flex flex-col items-center justify-center py-20 text-center;
	}
</style>
