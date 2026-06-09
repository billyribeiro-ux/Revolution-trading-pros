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

	function getTimelineBarHeight(sessions: number): string {
		const maxSessions = Math.max(
			...(dashboard?.session_timeline.map((point) => point.sessions) ?? [0])
		);
		if (maxSessions === 0) return '0%';
		return `${(sessions / maxSessions) * 100}%`;
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
				<IconBrain size={32} class="behavior-header-icon" />
				<div>
					<h1 class="behavior-title">Behavior Analytics</h1>
					<p class="behavior-subtitle">RevolutionBehavior-L8-System</p>
				</div>
			</div>

			<div class="period-selector">
				<button
					class={['period-btn', { active: selectedPeriod === '24h' }]}
					onclick={() => handlePeriodChange('24h')}
				>
					24h
				</button>
				<button
					class={['period-btn', { active: selectedPeriod === '7d' }]}
					onclick={() => handlePeriodChange('7d')}
				>
					7d
				</button>
				<button
					class={['period-btn', { active: selectedPeriod === '30d' }]}
					onclick={() => handlePeriodChange('30d')}
				>
					30d
				</button>
				<button
					class={['period-btn', { active: selectedPeriod === '90d' }]}
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
			<p class="state-text state-text--spaced">Loading behavior data...</p>
		</div>
	{:else if dashboard}
		<!-- Overview KPIs -->
		<div class="kpi-grid">
			<div class="kpi-card">
				<div class="kpi-icon kpi-icon--blue">
					<IconUsers size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Total Sessions</div>
					<div class="kpi-value">{dashboard.overview.total_sessions.toLocaleString()}</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon kpi-icon--green">
					<IconTrendingUp size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Engagement</div>
					<div class="kpi-value">{dashboard.overview.avg_engagement_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon kpi-icon--yellow">
					<IconTarget size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Intent</div>
					<div class="kpi-value">{dashboard.overview.avg_intent_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card">
				<div class="kpi-icon kpi-icon--orange">
					<IconAlertTriangle size={24} />
				</div>
				<div class="kpi-content">
					<div class="kpi-label">Avg Friction</div>
					<div class="kpi-value">{dashboard.overview.avg_friction_score.toFixed(1)}%</div>
				</div>
			</div>

			<div class="kpi-card critical">
				<div class="kpi-icon kpi-icon--red">
					<IconAlertTriangle size={24} />
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
					{#each dashboard.friction_heatmap as item (item.page_url)}
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
									style:width={`${Math.min((item.friction_count / 10) * 100, 100)}%`}
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
					{#each dashboard.session_timeline as point (point.timestamp)}
						<div class="timeline-bar">
							<div
								class="bar-fill"
								style:height={getTimelineBarHeight(point.sessions)}
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
			<IconChartBar size={64} class="empty-icon" />
			<h3 class="empty-title">No Behavior Data</h3>
			<p class="empty-copy">Start tracking user behavior to see insights</p>
		</div>
	{/if}
</div>

<style>
	.behavior-dashboard {
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

	.header-title :global(.behavior-header-icon) {
		color: #c084fc;
	}

	.behavior-title {
		margin: 0;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
		color: #ffffff;
	}

	.behavior-subtitle {
		margin: 0.25rem 0 0;
		color: #9ca3af;
	}

	.period-selector {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.period-btn {
		padding: 0.5rem 1rem;
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
	}

	.period-btn:hover {
		background: rgba(55, 65, 81, 0.5);
		color: #ffffff;
	}

	.period-btn.active {
		border-color: rgba(168, 85, 247, 0.5);
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
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
		border-top-color: #c084fc;
		border-radius: 999px;
		animation: behavior-spin 1s linear infinite;
	}

	.kpi-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.kpi-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.kpi-card.critical {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.05);
	}

	.kpi-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		border-radius: 0.5rem;
		flex-shrink: 0;
	}

	.kpi-icon--blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.kpi-icon--green {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.kpi-icon--yellow {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}

	.kpi-icon--orange {
		background: rgba(249, 115, 22, 0.2);
		color: #fb923c;
	}

	.kpi-icon--red {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.kpi-content {
		flex: 1;
		min-width: 0;
	}

	.kpi-label {
		margin-bottom: 0.25rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.kpi-value {
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
		color: #ffffff;
	}

	.section {
		margin-bottom: 2rem;
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.section-header {
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

	.friction-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.friction-item {
		padding: 1rem;
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
	}

	.friction-info {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.friction-url {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		color: #ffffff;
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.friction-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		white-space: nowrap;
	}

	.badge {
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.badge-orange {
		background: rgba(249, 115, 22, 0.2);
		color: #fb923c;
	}

	.friction-count {
		color: #9ca3af;
	}

	.friction-bar {
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: #374151;
	}

	.friction-fill {
		height: 100%;
		background: linear-gradient(to right, #f97316, #ef4444);
	}

	.timeline-chart {
		display: flex;
		align-items: flex-end;
		gap: 0.5rem;
		height: 16rem;
	}

	.timeline-bar {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.bar-fill {
		width: 100%;
		min-height: 0.25rem;
		border-radius: 0.25rem 0.25rem 0 0;
		background: linear-gradient(to top, #a855f7, #c084fc);
		transition: background 0.2s ease;
	}

	.bar-fill:hover {
		background: linear-gradient(to top, #c084fc, #d8b4fe);
	}

	.bar-label {
		color: #9ca3af;
		font-size: 0.75rem;
		transform: rotate(45deg);
		transform-origin: left;
		white-space: nowrap;
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

	@media (min-width: 768px) {
		.kpi-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.kpi-grid {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}
	}

	@media (max-width: 767px) {
		.friction-info {
			flex-direction: column;
		}

		.timeline-chart {
			overflow-x: auto;
		}

		.timeline-bar {
			min-width: 3rem;
		}
	}

	@keyframes behavior-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
