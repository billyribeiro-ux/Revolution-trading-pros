<script lang="ts">
	import { onMount } from 'svelte';
	import { toastStore } from '$lib/stores/toast';
	import {
		IconGauge,
		IconServer,
		IconDatabase,
		IconRefresh,
		IconCpu,
		IconActivity,
		IconUpload,
		IconBolt,
		IconCheck,
		IconAlertTriangle,
		IconX,
		IconChartBar,
		IconClock,
		IconFlame
	} from '@tabler/icons-svelte';
	import {
		bingSeoApi,
		type PerformanceDashboard,
		type CoreWebVitals,
		type OptimizationRecommendation
	} from '$lib/api/bing-seo';

	// State
	let loading = true;
	let dashboard: PerformanceDashboard | null = null;
	let coreWebVitals: CoreWebVitals | null = null;
	let recommendations: OptimizationRecommendation[] = [];
	let warmingCaches = false;

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const [dashboardData, recommendationsData] = await Promise.all([
				bingSeoApi.getPerformanceDashboard().catch(() => null),
				bingSeoApi.getRecommendations().catch(() => ({ recommendations: [], cdn: null, core_web_vitals: null }))
			]);

			dashboard = dashboardData;
			recommendations = recommendationsData.recommendations || [];
			coreWebVitals = dashboardData?.core_web_vitals || null;
		} catch (err) {
			console.error('Failed to load performance data:', err);
		} finally {
			loading = false;
		}
	}

	async function handleWarmCaches() {
		warmingCaches = true;
		try {
			const result = await bingSeoApi.warmCaches();
			toastStore.success(
				`Caches warmed! Views: ${result.views}, Routes: ${result.routes}, Configs: ${result.configs}`
			);
			await loadData();
		} catch (err: any) {
			toastStore.error(err.message || 'Failed to warm caches');
		} finally {
			warmingCaches = false;
		}
	}

	function getVitalRating(rating: string | undefined) {
		switch (rating) {
			case 'good':
				return { color: 'emerald', icon: IconCheck, label: 'Good' };
			case 'needs_improvement':
				return { color: 'yellow', icon: IconAlertTriangle, label: 'Needs Work' };
			case 'poor':
				return { color: 'red', icon: IconX, label: 'Poor' };
			default:
				return { color: 'slate', icon: IconClock, label: 'N/A' };
		}
	}

	function formatMs(ms: number | undefined): string {
		if (!ms) return '-';
		if (ms < 1000) return `${ms.toFixed(0)}ms`;
		return `${(ms / 1000).toFixed(2)}s`;
	}

	function formatPercent(value: number | undefined): string {
		if (value === undefined) return '-';
		return `${value.toFixed(1)}%`;
	}

	function getPriorityColor(priority: string) {
		switch (priority) {
			case 'critical':
				return 'red';
			case 'high':
				return 'orange';
			case 'medium':
				return 'yellow';
			default:
				return 'slate';
		}
	}
</script>

<svelte:head>
	<title>Performance Dashboard | Admin</title>
</svelte:head>

<div class="performance-page">
	<!-- Header -->
	<div class="page-header">
		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconGauge size={28} />
				</div>
				<div>
					<h1>Performance Dashboard</h1>
					<p class="subtitle">Monitor and optimize your website performance</p>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-secondary" onclick={loadData}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-primary" onclick={handleWarmCaches} disabled={warmingCaches}>
					{#if warmingCaches}
						Warming...
					{:else}
						<IconFlame size={18} />
						Warm Caches
					{/if}
				</button>
			</div>
		</div>
	</div>

	{#if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _}
				<div class="skeleton skeleton-metric"></div>
			{/each}
		</div>
	{:else}
		<!-- Core Web Vitals -->
		<div class="section">
			<h2 class="section-title">
				<IconBolt size={20} />
				Core Web Vitals
			</h2>
			<p class="section-description">
				Google's metrics for user experience. Aim for "Good" on all metrics to boost search rankings.
			</p>

			<div class="vitals-grid">
				<div class="vital-card">
					<div class="vital-header">
						<span class="vital-name">LCP</span>
						<span class="vital-label">Largest Contentful Paint</span>
					</div>
					<div class="vital-value">{formatMs(coreWebVitals?.lcp)}</div>
					<div class="vital-rating {getVitalRating(coreWebVitals?.lcp_rating).color}">
						<svelte:component this={getVitalRating(coreWebVitals?.lcp_rating).icon} size={16} />
						{getVitalRating(coreWebVitals?.lcp_rating).label}
					</div>
					<div class="vital-target">Target: &lt; 2.5s</div>
				</div>

				<div class="vital-card">
					<div class="vital-header">
						<span class="vital-name">FID</span>
						<span class="vital-label">First Input Delay</span>
					</div>
					<div class="vital-value">{formatMs(coreWebVitals?.fid)}</div>
					<div class="vital-rating {getVitalRating(coreWebVitals?.fid_rating).color}">
						<svelte:component this={getVitalRating(coreWebVitals?.fid_rating).icon} size={16} />
						{getVitalRating(coreWebVitals?.fid_rating).label}
					</div>
					<div class="vital-target">Target: &lt; 100ms</div>
				</div>

				<div class="vital-card">
					<div class="vital-header">
						<span class="vital-name">CLS</span>
						<span class="vital-label">Cumulative Layout Shift</span>
					</div>
					<div class="vital-value">{coreWebVitals?.cls?.toFixed(3) || '-'}</div>
					<div class="vital-rating {getVitalRating(coreWebVitals?.cls_rating).color}">
						<svelte:component this={getVitalRating(coreWebVitals?.cls_rating).icon} size={16} />
						{getVitalRating(coreWebVitals?.cls_rating).label}
					</div>
					<div class="vital-target">Target: &lt; 0.1</div>
				</div>

				<div class="vital-card">
					<div class="vital-header">
						<span class="vital-name">INP</span>
						<span class="vital-label">Interaction to Next Paint</span>
					</div>
					<div class="vital-value">{formatMs(coreWebVitals?.inp)}</div>
					<div class="vital-rating {getVitalRating(coreWebVitals?.inp_rating).color}">
						<svelte:component this={getVitalRating(coreWebVitals?.inp_rating).icon} size={16} />
						{getVitalRating(coreWebVitals?.inp_rating).label}
					</div>
					<div class="vital-target">Target: &lt; 200ms</div>
				</div>
			</div>
		</div>

		<!-- Server Metrics -->
		<div class="metrics-grid">
			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon server">
						<IconServer size={24} />
					</div>
					<h3>Server</h3>
				</div>
				<div class="metric-content">
					<div class="metric-row">
						<span>CPU Usage</span>
						<div class="metric-bar-container">
							<div
								class="metric-bar"
								class:warning={(dashboard?.server?.cpu_usage || 0) > 70}
								class:danger={(dashboard?.server?.cpu_usage || 0) > 90}
								style="width: {dashboard?.server?.cpu_usage || 0}%"
							></div>
						</div>
						<span class="metric-value">{formatPercent(dashboard?.server?.cpu_usage)}</span>
					</div>
					<div class="metric-row">
						<span>Memory</span>
						<div class="metric-bar-container">
							<div
								class="metric-bar"
								class:warning={(dashboard?.server?.memory_usage || 0) > 70}
								class:danger={(dashboard?.server?.memory_usage || 0) > 90}
								style="width: {dashboard?.server?.memory_usage || 0}%"
							></div>
						</div>
						<span class="metric-value">{formatPercent(dashboard?.server?.memory_usage)}</span>
					</div>
					<div class="metric-row">
						<span>Disk</span>
						<div class="metric-bar-container">
							<div
								class="metric-bar"
								class:warning={(dashboard?.server?.disk_usage || 0) > 70}
								class:danger={(dashboard?.server?.disk_usage || 0) > 90}
								style="width: {dashboard?.server?.disk_usage || 0}%"
							></div>
						</div>
						<span class="metric-value">{formatPercent(dashboard?.server?.disk_usage)}</span>
					</div>
					<div class="metric-row">
						<span>Uptime</span>
						<span class="metric-value">{dashboard?.server?.uptime || '-'}</span>
					</div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon database">
						<IconDatabase size={24} />
					</div>
					<h3>Database</h3>
				</div>
				<div class="metric-content">
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.database?.query_count?.toLocaleString() || 0}</span>
						<span class="stat-label">Total Queries</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.database?.slow_queries || 0}</span>
						<span class="stat-label">Slow Queries</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.database?.connection_pool || 0}</span>
						<span class="stat-label">Connection Pool</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{formatMs(dashboard?.database?.avg_query_time)}</span>
						<span class="stat-label">Avg Query Time</span>
					</div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-header">
					<div class="metric-icon cache">
						<IconActivity size={24} />
					</div>
					<h3>Cache</h3>
				</div>
				<div class="metric-content">
					<div class="metric-row">
						<span>Hit Rate</span>
						<div class="metric-bar-container">
							<div
								class="metric-bar success"
								style="width: {dashboard?.cache?.hit_rate || 0}%"
							></div>
						</div>
						<span class="metric-value">{formatPercent(dashboard?.cache?.hit_rate)}</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.cache?.memory_used || '-'}</span>
						<span class="stat-label">Memory Used</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.cache?.keys_count?.toLocaleString() || 0}</span>
						<span class="stat-label">Cached Keys</span>
					</div>
					<div class="metric-stat">
						<span class="stat-value">{dashboard?.cache?.evictions?.toLocaleString() || 0}</span>
						<span class="stat-label">Evictions</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Recommendations -->
		<div class="section">
			<h2 class="section-title">
				<IconChartBar size={20} />
				Optimization Recommendations
			</h2>

			{#if recommendations.length === 0}
				<div class="empty-state">
					<IconCheck size={48} stroke={1} />
					<h3>All Good!</h3>
					<p>No immediate optimizations recommended</p>
				</div>
			{:else}
				<div class="recommendations-list">
					{#each recommendations as rec}
						<div class="recommendation-card">
							<div class="rec-priority {getPriorityColor(rec.priority)}">
								{rec.priority}
							</div>
							<div class="rec-content">
								<div class="rec-header">
									<h4>{rec.title}</h4>
									<span class="rec-category">{rec.category}</span>
								</div>
								<p class="rec-description">{rec.description}</p>
								<div class="rec-details">
									<div class="rec-impact">
										<strong>Impact:</strong> {rec.impact}
									</div>
									{#if rec.implementation}
										<div class="rec-implementation">
											<strong>How to fix:</strong> {rec.implementation}
										</div>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.performance-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #f97316, #fb923c);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Sections */
	.section {
		margin-bottom: 2rem;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.section-description {
		color: #64748b;
		margin: 0 0 1.5rem;
	}

	/* Vitals Grid */
	.vitals-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.vital-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
		text-align: center;
	}

	.vital-header {
		margin-bottom: 1rem;
	}

	.vital-name {
		display: block;
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.vital-label {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	.vital-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.vital-rating {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.vital-rating.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.vital-rating.yellow {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.vital-rating.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.vital-rating.slate {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.vital-target {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.75rem;
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.metric-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.metric-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.metric-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-icon.server {
		background: rgba(99, 102, 241, 0.15);
		color: #818cf8;
	}
	.metric-icon.database {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.metric-icon.cache {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.metric-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.metric-content {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.metric-row {
		display: grid;
		grid-template-columns: 80px 1fr 60px;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.metric-bar-container {
		height: 8px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 4px;
		overflow: hidden;
	}

	.metric-bar {
		height: 100%;
		background: linear-gradient(90deg, #6366f1, #8b5cf6);
		border-radius: 4px;
		transition: width 0.3s;
	}

	.metric-bar.success {
		background: linear-gradient(90deg, #10b981, #34d399);
	}

	.metric-bar.warning {
		background: linear-gradient(90deg, #f59e0b, #fbbf24);
	}

	.metric-bar.danger {
		background: linear-gradient(90deg, #ef4444, #f87171);
	}

	.metric-value {
		text-align: right;
		font-weight: 600;
		color: #f1f5f9;
	}

	.metric-stat {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metric-stat .stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.metric-stat .stat-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Recommendations */
	.recommendations-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.recommendation-card {
		display: flex;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.rec-priority {
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		height: fit-content;
	}

	.rec-priority.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.rec-priority.orange {
		background: rgba(249, 115, 22, 0.15);
		color: #fb923c;
	}
	.rec-priority.yellow {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.rec-priority.slate {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.rec-content {
		flex: 1;
	}

	.rec-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.rec-header h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.rec-category {
		font-size: 0.6875rem;
		padding: 0.125rem 0.5rem;
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
		border-radius: 4px;
	}

	.rec-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 0.75rem;
	}

	.rec-details {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: #64748b;
	}

	.rec-details strong {
		color: #94a3b8;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
		background: rgba(30, 41, 59, 0.3);
		border-radius: 16px;
		border: 2px dashed rgba(148, 163, 184, 0.2);
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin: 0;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-metric {
		height: 180px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #fb923c);
		color: white;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-primary:disabled,
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 1024px) {
		.vitals-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.loading-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
