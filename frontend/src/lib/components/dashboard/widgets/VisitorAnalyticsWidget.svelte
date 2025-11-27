<script lang="ts">
	import { onMount } from 'svelte';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';

	export let data: {
		unique_visitors: number;
		total_sessions: number;
		total_pageviews: number;
		avg_session_duration: number;
		avg_pages_per_session: number;
		visitor_growth: number;
		bounce_rate: number;
		daily_trend: Array<{ date: string; visitors: number; sessions: number }>;
		period: string;
	};
	// Config available for widget customization
	export const config: Record<string, unknown> = {};

	const visitors = tweened(0, { duration: 1500, easing: cubicOut });
	const sessions = tweened(0, { duration: 1500, easing: cubicOut });
	const pageviews = tweened(0, { duration: 1500, easing: cubicOut });

	$: if (data) {
		visitors.set(data.unique_visitors || 0);
		sessions.set(data.total_sessions || 0);
		pageviews.set(data.total_pageviews || 0);
	}

	$: maxValue = Math.max(...(data?.daily_trend?.map((d) => d.visitors) || [1]));

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return Math.round(num).toString();
	}

	function formatDuration(seconds: number): string {
		if (seconds < 60) return `${Math.round(seconds)}s`;
		const mins = Math.floor(seconds / 60);
		const secs = Math.round(seconds % 60);
		return `${mins}m ${secs}s`;
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}
</script>

<div class="visitor-analytics-widget">
	<!-- Hero Stats -->
	<div class="hero-stats">
		<div class="hero-stat main">
			<div class="stat-icon visitors">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
					<circle cx="9" cy="7" r="4" />
					<path d="M23 21v-2a4 4 0 0 0-3-3.87" />
					<path d="M16 3.13a4 4 0 0 1 0 7.75" />
				</svg>
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber($visitors)}</span>
				<span class="stat-label">Unique Visitors</span>
				{#if data?.visitor_growth !== undefined}
					<span class="stat-trend {data.visitor_growth >= 0 ? 'positive' : 'negative'}">
						{data.visitor_growth >= 0 ? '↑' : '↓'}
						{Math.abs(data.visitor_growth).toFixed(1)}%
					</span>
				{/if}
			</div>
		</div>

		<div class="hero-stat">
			<div class="stat-icon sessions">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="3" y="3" width="7" height="7" />
					<rect x="14" y="3" width="7" height="7" />
					<rect x="14" y="14" width="7" height="7" />
					<rect x="3" y="14" width="7" height="7" />
				</svg>
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber($sessions)}</span>
				<span class="stat-label">Sessions</span>
			</div>
		</div>

		<div class="hero-stat">
			<div class="stat-icon pageviews">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
					<polyline points="14 2 14 8 20 8" />
					<line x1="16" y1="13" x2="8" y2="13" />
					<line x1="16" y1="17" x2="8" y2="17" />
					<polyline points="10 9 9 9 8 9" />
				</svg>
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber($pageviews)}</span>
				<span class="stat-label">Pageviews</span>
			</div>
		</div>
	</div>

	<!-- Mini Chart -->
	<div class="chart-section">
		<div class="chart-header">
			<h4>Visitor Trend</h4>
			<span class="period-badge">{data?.period || '30d'}</span>
		</div>
		<div class="mini-chart">
			{#if data?.daily_trend?.length > 0}
				<div class="chart-bars">
					{#each data.daily_trend.slice(-14) as day, i}
						<div class="chart-bar-container" title="{formatDate(day.date)}: {day.visitors} visitors">
							<div
								class="chart-bar"
								style="height: {(day.visitors / maxValue) * 100}%"
								class:highlight={i === data.daily_trend.slice(-14).length - 1}
							></div>
							{#if i % 3 === 0 || i === data.daily_trend.slice(-14).length - 1}
								<span class="chart-label">{formatDate(day.date)}</span>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<div class="no-data">No trend data available</div>
			{/if}
		</div>
	</div>

	<!-- Engagement Metrics -->
	<div class="engagement-section">
		<h4>Engagement Metrics</h4>
		<div class="metrics-grid">
			<div class="metric-card">
				<div class="metric-icon bounce">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="23 6 13.5 15.5 8.5 10.5 1 18" />
						<polyline points="17 6 23 6 23 12" />
					</svg>
				</div>
				<div class="metric-info">
					<span class="metric-value">{data?.bounce_rate?.toFixed(1) || 0}%</span>
					<span class="metric-label">Bounce Rate</span>
				</div>
				<div class="metric-bar">
					<div
						class="metric-fill bounce"
						style="width: {Math.min(data?.bounce_rate || 0, 100)}%"
					></div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon duration">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<polyline points="12 6 12 12 16 14" />
					</svg>
				</div>
				<div class="metric-info">
					<span class="metric-value">{formatDuration(data?.avg_session_duration || 0)}</span>
					<span class="metric-label">Avg Duration</span>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon pages">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
						<path
							d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
						/>
					</svg>
				</div>
				<div class="metric-info">
					<span class="metric-value">{data?.avg_pages_per_session?.toFixed(1) || 0}</span>
					<span class="metric-label">Pages/Session</span>
				</div>
			</div>
		</div>
	</div>

	<div class="widget-footer">
		<a href="/admin/analytics" class="view-all-link">View Full Analytics →</a>
	</div>
</div>

<style>
	.visitor-analytics-widget {
		height: 100%;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		padding: 0.25rem;
	}

	.hero-stats {
		display: grid;
		grid-template-columns: 1.5fr 1fr 1fr;
		gap: 1rem;
	}

	.hero-stat {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 12px;
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.hero-stat:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.hero-stat.main {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
	}

	.hero-stat.main .stat-content .stat-value,
	.hero-stat.main .stat-content .stat-label {
		color: white;
	}

	.hero-stat.main .stat-icon {
		background: rgba(255, 255, 255, 0.2);
		color: white;
	}

	.stat-icon {
		width: 44px;
		height: 44px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon svg {
		width: 22px;
		height: 22px;
	}

	.stat-icon.visitors {
		background: #e0e7ff;
		color: #4f46e5;
	}
	.stat-icon.sessions {
		background: #dbeafe;
		color: #2563eb;
	}
	.stat-icon.pageviews {
		background: #dcfce7;
		color: #16a34a;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		line-height: 1.2;
	}

	.stat-label {
		font-size: 0.7rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.stat-trend {
		font-size: 0.75rem;
		font-weight: 600;
		margin-top: 0.25rem;
	}

	.stat-trend.positive {
		color: #10b981;
	}
	.stat-trend.negative {
		color: #ef4444;
	}

	.hero-stat.main .stat-trend.positive {
		color: #a7f3d0;
	}
	.hero-stat.main .stat-trend.negative {
		color: #fca5a5;
	}

	.chart-section {
		background: #f9fafb;
		border-radius: 12px;
		padding: 1rem;
	}

	.chart-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.chart-header h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.period-badge {
		font-size: 0.65rem;
		background: #e5e7eb;
		color: #6b7280;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-weight: 500;
	}

	.mini-chart {
		height: 100px;
	}

	.chart-bars {
		display: flex;
		align-items: flex-end;
		gap: 4px;
		height: 100%;
	}

	.chart-bar-container {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100%;
		cursor: pointer;
	}

	.chart-bar {
		width: 100%;
		background: linear-gradient(180deg, #a78bfa 0%, #7c3aed 100%);
		border-radius: 3px 3px 0 0;
		transition: all 0.3s ease;
		min-height: 4px;
	}

	.chart-bar.highlight {
		background: linear-gradient(180deg, #34d399 0%, #10b981 100%);
	}

	.chart-bar-container:hover .chart-bar {
		opacity: 0.8;
		transform: scaleY(1.05);
	}

	.chart-label {
		font-size: 0.55rem;
		color: #9ca3af;
		margin-top: 4px;
		white-space: nowrap;
	}

	.no-data {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.engagement-section h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.75rem 0;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.metric-card {
		background: #f9fafb;
		border-radius: 10px;
		padding: 0.875rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.metric-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-icon svg {
		width: 18px;
		height: 18px;
	}

	.metric-icon.bounce {
		background: #fee2e2;
		color: #dc2626;
	}
	.metric-icon.duration {
		background: #dbeafe;
		color: #2563eb;
	}
	.metric-icon.pages {
		background: #dcfce7;
		color: #16a34a;
	}

	.metric-info {
		display: flex;
		flex-direction: column;
	}

	.metric-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #1f2937;
	}

	.metric-label {
		font-size: 0.65rem;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.metric-bar {
		height: 4px;
		background: #e5e7eb;
		border-radius: 2px;
		overflow: hidden;
	}

	.metric-fill {
		height: 100%;
		border-radius: 2px;
		transition: width 1s ease-out;
	}

	.metric-fill.bounce {
		background: linear-gradient(90deg, #fca5a5 0%, #ef4444 100%);
	}

	.widget-footer {
		display: flex;
		justify-content: flex-end;
		padding-top: 0.75rem;
		border-top: 1px solid #e5e7eb;
		margin-top: auto;
	}

	.view-all-link {
		font-size: 0.75rem;
		color: #3b82f6;
		text-decoration: none;
		font-weight: 500;
		transition: color 0.2s;
	}

	.view-all-link:hover {
		color: #2563eb;
	}

	@media (max-width: 640px) {
		.hero-stats {
			grid-template-columns: 1fr;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
