<script lang="ts">
	import { onMount } from 'svelte';
	import type { WorkflowAnalytics } from '$lib/types/workflow';

	interface Props {
		workflowId: number;
	}

	let { workflowId }: Props = $props();

	let analytics: WorkflowAnalytics | null = $state(null);
	let isLoading = $state(true);
	let timeRange: '7d' | '30d' | '90d' = $state('30d');

	async function loadAnalytics() {
		isLoading = true;
		try {
			const token = localStorage.getItem('access_token');
			const response = await fetch(
				`/api/admin/workflows/${workflowId}/analytics?range=${timeRange}`,
				{
					headers: {
						Authorization: token ? `Bearer ${token}` : '',
						Accept: 'application/json'
					}
				}
			);

			if (response.ok) {
				const data = await response.json();
				analytics = {
					workflow_id: workflowId,
					total_runs: data.total_runs || 0,
					successful_runs: data.successful_runs || 0,
					failed_runs: data.failed_runs || 0,
					success_rate: data.success_rate || 0,
					avg_duration_ms: data.avg_duration_ms || 0,
					runs_by_day: data.runs_by_day || generateEmptyData(),
					failure_reasons: data.failure_reasons || []
				};
			} else {
				// Show empty state when no data available
				analytics = getEmptyAnalytics();
			}
		} catch (error) {
			console.error('Failed to load analytics:', error);
			analytics = getEmptyAnalytics();
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Generate empty data array for the time range
	 */
	function generateEmptyData() {
		const data = [];
		const days = timeRange === '7d' ? 7 : timeRange === '30d' ? 30 : 90;
		for (let i = days; i >= 0; i--) {
			const date = new Date();
			date.setDate(date.getDate() - i);
			data.push({
				date: date.toISOString().split('T')[0],
				count: 0
			});
		}
		return data;
	}

	/**
	 * Get empty analytics state
	 */
	function getEmptyAnalytics(): WorkflowAnalytics {
		return {
			workflow_id: workflowId,
			total_runs: 0,
			successful_runs: 0,
			failed_runs: 0,
			success_rate: 0,
			avg_duration_ms: 0,
			runs_by_day: generateEmptyData(),
			failure_reasons: []
		};
	}

	function formatDuration(ms: number): string {
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(1)}s`;
	}

	onMount(() => {
		loadAnalytics();
	});

	$effect(() => {
		if (timeRange) {
			loadAnalytics();
		}
	});
</script>

<div class="workflow-analytics">
	<div class="analytics-header">
		<h2>Workflow Analytics</h2>
		<div class="time-range-selector">
			<button class:active={timeRange === '7d'} onclick={() => (timeRange = '7d')}>7 Days</button>
			<button class:active={timeRange === '30d'} onclick={() => (timeRange = '30d')}>30 Days</button
			>
			<button class:active={timeRange === '90d'} onclick={() => (timeRange = '90d')}>90 Days</button
			>
		</div>
	</div>

	{#if isLoading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading analytics...</p>
		</div>
	{:else if analytics}
		{@const successPercent = (analytics.successful_runs / analytics.total_runs) * 100}
		<div class="metrics-grid">
			<div class="metric-card">
				<div class="metric-icon" style="background: #dbeafe; color: #1e40af;">
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M22 12h-4l-3 9L9 3l-3 9H2" />
					</svg>
				</div>
				<div class="metric-content">
					<span class="metric-label">Total Runs</span>
					<span class="metric-value">{analytics.total_runs.toLocaleString()}</span>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon" style="background: #d1fae5; color: #065f46;">
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
						<polyline points="22 4 12 14.01 9 11.01" />
					</svg>
				</div>
				<div class="metric-content">
					<span class="metric-label">Success Rate</span>
					<span class="metric-value success">{analytics.success_rate.toFixed(1)}%</span>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon" style="background: #fef3c7; color: #92400e;">
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="12" cy="12" r="10" />
						<polyline points="12 6 12 12 16 14" />
					</svg>
				</div>
				<div class="metric-content">
					<span class="metric-label">Avg Duration</span>
					<span class="metric-value">{formatDuration(analytics.avg_duration_ms)}</span>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon" style="background: #fee2e2; color: #991b1b;">
					<svg
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="12" cy="12" r="10" />
						<line x1="12" y1="8" x2="12" y2="12" />
						<line x1="12" y1="16" x2="12.01" y2="16" />
					</svg>
				</div>
				<div class="metric-content">
					<span class="metric-label">Failed Runs</span>
					<span class="metric-value error">{analytics.failed_runs.toLocaleString()}</span>
				</div>
			</div>
		</div>

		<div class="charts-section">
			<div class="chart-card">
				<h3>Execution Trend</h3>
				<div class="line-chart">
					{#each analytics.runs_by_day as day, _i}
						{@const maxCount = Math.max(...analytics.runs_by_day.map((d) => d.count))}
						{@const height = (day.count / maxCount) * 100}
						<div class="chart-bar" style="height: {height}%;" title="{day.date}: {day.count} runs">
							<span class="bar-value">{day.count}</span>
						</div>
					{/each}
				</div>
				<div class="chart-labels">
					<span>Start</span>
					<span>End</span>
				</div>
			</div>

			<div class="chart-card">
				<h3>Top Failure Reasons</h3>
				<div class="failure-list">
					{#each analytics.failure_reasons as failure}
						{@const percentage = (failure.count / analytics.failed_runs) * 100}
						<div class="failure-item">
							<div class="failure-info">
								<span class="failure-reason">{failure.reason}</span>
								<span class="failure-count">{failure.count} ({percentage.toFixed(1)}%)</span>
							</div>
							<div class="failure-bar">
								<div class="failure-bar-fill" style="width: {percentage}%"></div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<div class="success-breakdown">
			<h3>Success vs Failure</h3>
			<div class="pie-chart">
				<svg viewBox="0 0 100 100">
					<circle cx="50" cy="50" r="40" fill="#d1fae5" />
					<circle
						cx="50"
						cy="50"
						r="40"
						fill="transparent"
						stroke="#fee2e2"
						stroke-width="80"
						stroke-dasharray="{(100 - successPercent) * 2.51} 251"
						transform="rotate(-90 50 50)"
					/>
				</svg>
				<div class="pie-center">
					<span class="pie-value">{successPercent.toFixed(1)}%</span>
					<span class="pie-label">Success</span>
				</div>
			</div>
			<div class="legend">
				<div class="legend-item">
					<span class="legend-color" style="background: #10b981;"></span>
					<span>Successful: {analytics.successful_runs}</span>
				</div>
				<div class="legend-item">
					<span class="legend-color" style="background: #ef4444;"></span>
					<span>Failed: {analytics.failed_runs}</span>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.workflow-analytics {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.analytics-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.analytics-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0;
	}

	.time-range-selector {
		display: flex;
		gap: 0.5rem;
	}

	.time-range-selector button {
		padding: 0.5rem 1rem;
		background: #f3f4f6;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.time-range-selector button:hover {
		background: #e5e7eb;
	}

	.time-range-selector button.active {
		background: #3b82f6;
		color: white;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 3rem;
		gap: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.metric-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 12px;
		border: 1px solid #e5e7eb;
	}

	.metric-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 12px;
		flex-shrink: 0;
	}

	.metric-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metric-label {
		font-size: 0.875rem;
		color: #6b7280;
		font-weight: 500;
	}

	.metric-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
	}

	.metric-value.success {
		color: #10b981;
	}

	.metric-value.error {
		color: #ef4444;
	}

	.charts-section {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.chart-card {
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 12px;
		border: 1px solid #e5e7eb;
	}

	.chart-card h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 1.5rem 0;
	}

	.line-chart {
		display: flex;
		align-items: flex-end;
		gap: 2px;
		height: 200px;
		padding: 1rem 0;
	}

	.chart-bar {
		flex: 1;
		background: linear-gradient(180deg, #3b82f6, #1e40af);
		border-radius: 4px 4px 0 0;
		position: relative;
		min-height: 10px;
		transition: all 0.3s;
		cursor: pointer;
	}

	.chart-bar:hover {
		opacity: 0.8;
	}

	.bar-value {
		position: absolute;
		top: -20px;
		left: 50%;
		transform: translateX(-50%);
		font-size: 0.75rem;
		font-weight: 600;
		color: #1f2937;
		display: none;
	}

	.chart-bar:hover .bar-value {
		display: block;
	}

	.chart-labels {
		display: flex;
		justify-content: space-between;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.failure-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.failure-item {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.failure-info {
		display: flex;
		justify-content: space-between;
		font-size: 0.875rem;
	}

	.failure-reason {
		color: #1f2937;
		font-weight: 500;
	}

	.failure-count {
		color: #6b7280;
	}

	.failure-bar {
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.failure-bar-fill {
		height: 100%;
		background: linear-gradient(90deg, #ef4444, #dc2626);
		border-radius: 4px;
		transition: width 0.3s;
	}

	.success-breakdown {
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 12px;
		border: 1px solid #e5e7eb;
	}

	.success-breakdown h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 1.5rem 0;
	}

	.pie-chart {
		position: relative;
		width: 200px;
		height: 200px;
		margin: 0 auto 1.5rem;
	}

	.pie-chart svg {
		width: 100%;
		height: 100%;
	}

	.pie-center {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.pie-value {
		display: block;
		font-size: 2rem;
		font-weight: 700;
		color: #1f2937;
	}

	.pie-label {
		display: block;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.legend {
		display: flex;
		justify-content: center;
		gap: 2rem;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.legend-color {
		width: 16px;
		height: 16px;
		border-radius: 4px;
	}
</style>
