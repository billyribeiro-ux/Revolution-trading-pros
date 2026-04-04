<!--
	URL: /behavior
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { dashboardData, isLoading } from '$lib/stores/behavior.svelte';
	import { behaviorAPI } from '$lib/api/behavior';
	import { Icon, IconAlertTriangle, IconBrain, IconChartBar, IconTarget, IconTrendingUp, IconUsers } from '$lib/icons';

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
			logger.error('Failed to load behavior dashboard:', error);
		} finally {
			isLoading.set(false);
		}
	}

	function handlePeriodChange(period: string) {
		selectedPeriod = period;
		loadDashboard();
	}
</script>

<div class="behavior-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-content">
			<div class="header-title">
				<Icon icon={IconBrain} size={32} />
				<div>
					<h1 class="page-heading">Behavior Analytics</h1>
					<p class="page-subheading">RevolutionBehavior-L8-System</p>
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
			<p class="loading-text">Loading behavior data...</p>
		</div>
	{:else if dashboard}
		<!-- Overview KPIs -->
		<div class="kpi-grid">
			<div class="kpi-card">
				<div class="kpi-icon" data-color="blue">
					<Icon icon={IconUsers} size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Total Sessions</div>
					<div class="kpi-value">{dashboard.overview.total_sessions.toLocaleString()}</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon" data-color="green">
					<Icon icon={IconTrendingUp} size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Engagement</div>
					<div class="kpi-value">{dashboard.overview.avg_engagement_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon" data-color="yellow">
					<Icon icon={IconTarget} size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Intent</div>
					<div class="kpi-value">{dashboard.overview.avg_intent_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon" data-color="orange">
					<Icon icon={IconAlertTriangle} size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Friction</div>
					<div class="kpi-value">{dashboard.overview.avg_friction_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card" data-variant="critical">
				<div class="kpi-icon" data-color="red">
					<Icon icon={IconAlertTriangle} size={24} />
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
									<span class="friction-count">{item.friction_count} issues</span>
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
			<Icon icon={IconChartBar} size={64} />
			<h3 class="empty-title">No Behavior Data</h3>
			<p class="empty-subtitle">Start tracking user behavior to see insights</p>
		</div>
	{/if}
</div>

<style>
	.behavior-dashboard {
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
		color: oklch(0.7 0.18 300);
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

	.period-selector { display: flex; gap: var(--space-2); }

	.period-btn {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		border-radius: var(--radius-lg);
		font-weight: var(--weight-medium);
		background-color: oklch(0.25 0.01 250 / 50%);
		color: oklch(0.65 0.01 250);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover { background-color: oklch(0.38 0.01 250 / 50%); color: oklch(1 0 0); }
		&.active { background-color: oklch(0.55 0.2 300 / 20%); color: oklch(0.7 0.18 300); border-color: oklch(0.55 0.2 300 / 50%); }
	}

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
		border-block-start-color: oklch(0.7 0.18 300);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin { to { transform: rotate(360deg); } }

	.kpi-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);
		margin-block-end: var(--space-8);
		@media (min-width: 768px) { grid-template-columns: repeat(2, 1fr); }
		@media (min-width: 1024px) { grid-template-columns: repeat(5, 1fr); }
	}

	.kpi-card {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		display: flex;
		align-items: center;
		gap: var(--space-4);

		&[data-variant='critical'] { border-color: oklch(0.55 0.22 25 / 30%); background-color: oklch(0.55 0.22 25 / 5%); }
	}

	.kpi-icon {
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;

		&[data-color='blue'] { background-color: oklch(0.6 0.2 260 / 20%); color: oklch(0.7 0.18 260); }
		&[data-color='green'] { background-color: oklch(0.6 0.18 160 / 20%); color: oklch(0.7 0.18 160); }
		&[data-color='yellow'] { background-color: oklch(0.8 0.18 90 / 20%); color: oklch(0.8 0.18 90); }
		&[data-color='orange'] { background-color: oklch(0.7 0.18 55 / 20%); color: oklch(0.75 0.16 55); }
		&[data-color='red'] { background-color: oklch(0.55 0.22 25 / 20%); color: oklch(0.7 0.2 25); }
	}

	.kpi-content { flex: 1; }

	.kpi-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-1);
	}

	.kpi-value {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.section {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		margin-block-end: var(--space-8);
	}

	.section-header { margin-block-end: var(--space-6); }

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

	.friction-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.friction-item {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
	}

	.friction-info {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-block-end: var(--space-3);
	}

	.friction-url {
		color: oklch(1 0 0);
		font-weight: var(--weight-medium);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		flex: 1;
	}

	.friction-meta {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
	}

	.friction-count { color: oklch(0.65 0.01 250); }

	.badge {
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
	}

	.badge-orange {
		background-color: oklch(0.7 0.18 55 / 20%);
		color: oklch(0.75 0.16 55);
	}

	.friction-bar {
		block-size: 0.5rem;
		background-color: oklch(0.38 0.01 250);
		border-radius: 9999px;
		overflow: hidden;
	}

	.friction-fill {
		block-size: 100%;
		background: linear-gradient(to right, oklch(0.7 0.18 55), oklch(0.55 0.22 25));
	}

	.timeline-chart {
		display: flex;
		align-items: flex-end;
		gap: var(--space-2);
		block-size: 16rem;
	}

	.timeline-bar {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
	}

	.bar-fill {
		inline-size: 100%;
		background: linear-gradient(to top, oklch(0.55 0.2 300), oklch(0.65 0.18 300));
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
		transition: background var(--duration-fast) var(--ease-default);
		&:hover { background: linear-gradient(to top, oklch(0.65 0.18 300), oklch(0.75 0.16 300)); }
	}

	.bar-label {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		transform: rotate(45deg);
		transform-origin: left;
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
</style>
