<!--
	URL: /admin/behavior
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconEye,
		IconCursor,
		IconPointer,
		IconClock,
		IconChartBar,
		IconRefresh,
		IconArrowUpRight,
		IconPlay,
		IconClick
	} from '$lib/icons';
	import { api } from '$lib/api/config';
	import { connections, isBehaviorConnected } from '$lib/stores/connections.svelte';
	import ApiNotConnected from '$lib/components/ApiNotConnected.svelte';
	import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

	let isLoading = true;
	let connectionLoading = true;
	let error = '';
	let selectedPeriod = '7d';

	// Behavior metrics
	let metrics = {
		totalSessions: 0,
		avgSessionDuration: '0s',
		pagesPerSession: 0,
		scrollDepth: 0,
		clickRate: 0,
		heatmapViews: 0
	};

	// Top interactions
	let topClicks: { element: string; count: number; page: string }[] = [];
	let topScrolls: { page: string; avgDepth: number; views: number }[] = [];
	let recordings: { id: string; duration: string; pages: number; date: string }[] = [];

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const response = await api.get(`/api/admin/analytics/behavior?period=${selectedPeriod}`);
			const data = response?.data || response;

			if (data) {
				metrics = {
					totalSessions: data.total_sessions || 0,
					avgSessionDuration: data.avg_session_duration || '0s',
					pagesPerSession: data.pages_per_session || 0,
					scrollDepth: data.avg_scroll_depth || 0,
					clickRate: data.click_rate || 0,
					heatmapViews: data.heatmap_views || 0
				};

				topClicks = data.top_clicks || [];
				topScrolls = data.top_scrolls || [];
				recordings = data.recordings || [];
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load behavior data';
		} finally {
			isLoading = false;
		}
	}

	function changePeriod(period: string) {
		selectedPeriod = period;
		loadData();
	}

	onMount(async () => {
		// Load connection status first
		await connections.load();
		connectionLoading = false;

		// Only load data if behavior tracking is connected
		if ($isBehaviorConnected) {
			await loadData();
		} else {
			isLoading = false;
		}
	});
</script>

<svelte:head>
	<title>Behavior Tracking - Admin Dashboard</title>
</svelte:head>

<div class="admin-behavior">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header - Centered Style -->
		<header class="page-header">
			<h1>Behavior Tracking</h1>
			<p class="subtitle">
				Analyze user interactions, clicks, scrolls, and session recordings
			</p>
			{#if $isBehaviorConnected}
				<div class="header-actions">
					<div class="period-selector">
						<button class:active={selectedPeriod === '24h'} onclick={() => changePeriod('24h')}
							>24H</button
						>
						<button class:active={selectedPeriod === '7d'} onclick={() => changePeriod('7d')}
							>7D</button
						>
						<button class:active={selectedPeriod === '30d'} onclick={() => changePeriod('30d')}
							>30D</button
						>
					</div>
					<button class="btn-refresh" onclick={loadData} disabled={isLoading}>
						<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<SkeletonLoader variant="dashboard" />
		{:else if !$isBehaviorConnected}
			<ApiNotConnected
				serviceName="Behavior Analytics"
				description="Connect an analytics platform to track user behavior, record sessions, and generate heatmaps for deeper insights."
				serviceKey="google_analytics"
				icon="🔍"
				color="#10b981"
				features={[
					'Track user sessions and interactions',
					'Record and replay user sessions',
					'Generate click and scroll heatmaps',
					'Analyze page engagement metrics',
					'Identify UX issues and opportunities'
				]}
			/>
		{:else}
			{#if error}
				<div class="error-banner">{error}</div>
			{/if}

			<!-- Metrics Grid -->
			<div class="metrics-grid">
				<div class="metric-card">
					<div class="metric-icon blue">
						<IconEye size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.totalSessions.toLocaleString()}</span>
						<span class="metric-label">Total Sessions</span>
					</div>
					<div class="metric-trend positive">
						<IconArrowUpRight size={14} />
						<span>12.5%</span>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon purple">
						<IconClock size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.avgSessionDuration}</span>
						<span class="metric-label">Avg. Session Duration</span>
					</div>
					<div class="metric-trend positive">
						<IconArrowUpRight size={14} />
						<span>8.3%</span>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon cyan">
						<IconChartBar size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.pagesPerSession.toFixed(1)}</span>
						<span class="metric-label">Pages / Session</span>
					</div>
					<div class="metric-trend positive">
						<IconArrowUpRight size={14} />
						<span>5.2%</span>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon green">
						<IconPointer size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.scrollDepth}%</span>
						<span class="metric-label">Avg. Scroll Depth</span>
					</div>
					<div class="metric-trend positive">
						<IconArrowUpRight size={14} />
						<span>3.1%</span>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon orange">
						<IconClick size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.clickRate}%</span>
						<span class="metric-label">Click Rate</span>
					</div>
					<div class="metric-trend negative">
						<IconArrowUpRight size={14} />
						<span>1.2%</span>
					</div>
				</div>

				<div class="metric-card">
					<div class="metric-icon pink">
						<IconCursor size={24} />
					</div>
					<div class="metric-content">
						<span class="metric-value">{metrics.heatmapViews.toLocaleString()}</span>
						<span class="metric-label">Heatmap Views</span>
					</div>
					<div class="metric-trend positive">
						<IconArrowUpRight size={14} />
						<span>15.7%</span>
					</div>
				</div>
			</div>

			<!-- Content Grid -->
			<div class="content-grid">
				<!-- Click Heatmap Section -->
				<div class="panel">
					<div class="panel-header">
						<h3>
							<IconClick size={20} />
							Top Click Elements
						</h3>
						<a href="/admin/analytics/heatmaps" class="panel-link">View Heatmaps</a>
					</div>
					<div class="panel-content">
						{#if topClicks.length === 0}
							<div class="empty-state">
								<IconClick size={32} />
								<p>No click data available yet</p>
							</div>
						{:else}
							<div class="click-list">
								{#each topClicks as click, i}
									<div class="click-item">
										<span class="click-rank">{i + 1}</span>
										<div class="click-info">
											<span class="click-element">{click.element}</span>
											<span class="click-page">{click.page}</span>
										</div>
										<span class="click-count">{click.count}</span>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>

				<!-- Scroll Depth Section -->
				<div class="panel">
					<div class="panel-header">
						<h3>
							<IconPointer size={20} />
							Scroll Depth by Page
						</h3>
					</div>
					<div class="panel-content">
						{#if topScrolls.length === 0}
							<div class="empty-state">
								<IconPointer size={32} />
								<p>No scroll data available yet</p>
							</div>
						{:else}
							<div class="scroll-list">
								{#each topScrolls as scroll}
									<div class="scroll-item">
										<div class="scroll-info">
											<span class="scroll-page">{scroll.page}</span>
											<span class="scroll-views">{scroll.views} views</span>
										</div>
										<div class="scroll-bar-wrap">
											<div class="scroll-bar" style="width: {scroll.avgDepth}%"></div>
										</div>
										<span class="scroll-depth">{scroll.avgDepth}%</span>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>

				<!-- Session Recordings -->
				<div class="panel full-width">
					<div class="panel-header">
						<h3>
							<IconPlay size={20} />
							Recent Session Recordings
						</h3>
						<a href="/admin/analytics/recordings" class="panel-link">View All</a>
					</div>
					<div class="panel-content">
						{#if recordings.length === 0}
							<div class="empty-state">
								<IconPlay size={32} />
								<p>No session recordings available</p>
								<span class="empty-hint">Session recordings help you understand user behavior</span>
							</div>
						{:else}
							<div class="recordings-grid">
								{#each recordings as recording}
									<div class="recording-card">
										<div class="recording-preview">
											<IconPlay size={24} />
										</div>
										<div class="recording-info">
											<span class="recording-duration">{recording.duration}</span>
											<span class="recording-pages">{recording.pages} pages</span>
											<span class="recording-date">{recording.date}</span>
										</div>
										<button class="btn-play">
											<IconPlay size={16} />
											Watch
										</button>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN BEHAVIOR - Analytics Dashboard Layout Pattern
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-behavior {
		background: linear-gradient(135deg, var(--bg-base) 0%, var(--bg-elevated) 50%, var(--bg-base) 100%);
		color: var(--text-primary);
		position: relative;
		overflow: hidden;
	}

	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
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
		background: linear-gradient(135deg, var(--secondary-500), var(--primary-600));
		animation: float 25s ease-in-out infinite reverse;
	}

	.bg-blob-3 {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, var(--success-base), var(--success-emphasis));
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(30px, -30px) scale(1.05); }
		66% { transform: translate(-20px, 20px) scale(0.95); }
	}

	/* Page Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.period-selector {
		display: flex;
		background: rgba(15, 23, 42, 0.8);
		border-radius: 10px;
		padding: 4px;
		border: 1px solid rgba(230, 184, 0, 0.2);
	}

	.period-selector button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.period-selector button:hover {
		color: #e2e8f0;
	}

	.period-selector button.active {
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		color: white;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #e6b800;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.error-banner {
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1400px) {
		.metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.metric-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		position: relative;
	}

	.metric-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.metric-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}
	.metric-icon.cyan {
		background: rgba(6, 182, 212, 0.15);
		color: #22d3ee;
	}
	.metric-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.metric-icon.orange {
		background: rgba(251, 146, 60, 0.15);
		color: #fb923c;
	}
	.metric-icon.pink {
		background: rgba(236, 72, 153, 0.15);
		color: #f472b6;
	}

	.metric-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metric-value {
		font-size: 1.75rem;
		font-weight: 800;
		color: #f1f5f9;
	}

	.metric-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.metric-trend {
		position: absolute;
		top: 1rem;
		right: 1rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
	}

	.metric-trend.positive {
		color: #4ade80;
		background: rgba(34, 197, 94, 0.15);
	}

	.metric-trend.negative {
		color: #f87171;
		background: rgba(239, 68, 68, 0.15);
	}

	/* Content Grid */
	.content-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}

	.panel {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel.full-width {
		grid-column: span 2;
	}

	@media (max-width: 1024px) {
		.panel.full-width {
			grid-column: span 1;
		}
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.panel-header h3 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.panel-header h3 :global(svg) {
		color: #e6b800;
	}

	.panel-link {
		font-size: 0.85rem;
		color: #e6b800;
		text-decoration: none;
	}

	.panel-link:hover {
		color: #ffd11a;
	}

	.panel-content {
		padding: 1.25rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state p {
		margin: 0;
	}

	.empty-hint {
		font-size: 0.8rem;
		margin-top: 0.5rem;
	}

	/* Click List */
	.click-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.click-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(230, 184, 0, 0.05);
		border-radius: 10px;
	}

	.click-rank {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		font-size: 0.8rem;
		font-weight: 600;
		color: #e6b800;
	}

	.click-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.click-element {
		font-weight: 500;
		color: #e2e8f0;
	}

	.click-page {
		font-size: 0.8rem;
		color: #64748b;
	}

	.click-count {
		font-weight: 600;
		color: #e6b800;
	}

	/* Scroll List */
	.scroll-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.scroll-item {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.scroll-info {
		width: 150px;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.scroll-page {
		font-weight: 500;
		color: #e2e8f0;
		font-size: 0.9rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.scroll-views {
		font-size: 0.75rem;
		color: #64748b;
	}

	.scroll-bar-wrap {
		flex: 1;
		height: 8px;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.scroll-bar {
		height: 100%;
		background: linear-gradient(90deg, #e6b800, #b38f00);
		border-radius: 4px;
		transition: width 0.5s ease-out;
	}

	.scroll-depth {
		width: 50px;
		text-align: right;
		font-weight: 600;
		color: #e6b800;
	}

	/* Recordings */
	.recordings-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.recording-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(230, 184, 0, 0.05);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 12px;
	}

	.recording-preview {
		width: 60px;
		height: 45px;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #e6b800;
	}

	.recording-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.recording-duration {
		font-weight: 600;
		color: #e2e8f0;
	}

	.recording-pages {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.recording-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.btn-play {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.2);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 8px;
		color: #e6b800;
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-play:hover {
		background: rgba(230, 184, 0, 0.3);
	}
</style>
