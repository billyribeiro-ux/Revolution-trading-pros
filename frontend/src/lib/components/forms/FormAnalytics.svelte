<script lang="ts">
	import { onMount } from 'svelte';
	import type {
		Form,
		FormField,
		FormSubmission,
		SubmissionData,
		FormAnalytics as FormAnalyticsResponse,
		FormSubmissionTrendPoint
	} from '$lib/api/forms';
	import { getSubmissionStats, getSubmissions } from '$lib/api/forms';

	interface Props {
		form: Form;
	}

	let props: Props = $props();

	/**
	 * View-model shape the template reads. Derived from the real backend
	 * `FormAnalyticsResponse` envelope (see `forms.ts`). Pre-R24-B this
	 * component cast the response `as unknown as SubmissionStatsView` and
	 * read keys (`total_submissions` / `read_count` / `unread_count` / etc.)
	 * that don't exist on the response — every metric rendered as `0`. R24-B
	 * builds these fields from `response.submissions` + `status_breakdown` +
	 * `submission_trends`, which the backend has been emitting all along.
	 */
	interface SubmissionStatsView {
		total_submissions: number;
		recent_submissions: number;
		read_count: number;
		unread_count: number;
		starred_count: number;
		archived_count: number;
	}

	interface FieldCompletionRow {
		label: string;
		name: string;
		completionRate: number;
		filled: number;
		total: number;
	}

	let stats = $state<SubmissionStatsView | null>(null);
	let loading = $state(true);
	let submissionTrend = $state<FormSubmissionTrendPoint[]>([]);
	let fieldAnalytics = $state<FieldCompletionRow[]>([]);

	onMount(async () => {
		await loadAnalytics();
	});

	/**
	 * Project the backend `FormAnalyticsResponse` shape onto the view model
	 * the template renders. Counts are sourced from `status_breakdown` (one
	 * row per status); `recent_submissions` is the last 7 days of
	 * `submission_trends` summed (the backend already returns the trend).
	 */
	function toStatsView(resp: FormAnalyticsResponse): SubmissionStatsView {
		const statusCount = (s: string): number =>
			resp.status_breakdown?.find((row) => row.status === s)?.count ?? 0;

		const last7Days = (resp.submission_trends ?? []).slice(-7);
		const recent = last7Days.reduce((sum, p) => sum + p.count, 0);

		return {
			total_submissions: resp.submissions,
			recent_submissions: recent,
			unread_count: statusCount('unread'),
			read_count: statusCount('read'),
			starred_count: statusCount('starred'),
			archived_count: statusCount('archived')
		};
	}

	async function loadAnalytics() {
		loading = true;

		try {
			if (props.form.id) {
				// R24-B: use the typed backend envelope directly. `getSubmissionStats`
				// is an alias for `getFormAnalytics`; both hit GET /forms/:id/analytics.
				const resp = await getSubmissionStats(props.form.id);
				stats = toStatsView(resp);

				// Use the backend-provided 30-day trend directly. The client
				// previously re-computed this from `getSubmissions(...)`; for
				// forms with >100 submissions the client calc only counted the
				// most-recent 100 (a paged slice), under-reporting the trend.
				// LB-FA-1.
				submissionTrend = resp.submission_trends ?? [];

				// Field completion still requires submission rows (the backend
				// envelope doesn't include per-field response data).
				const submissions = await getSubmissions(props.form.id, 1, 100);
				fieldAnalytics = analyzeFieldCompletion(submissions.submissions);
			}
		} catch (err) {
			console.error('Failed to load analytics:', err);
		} finally {
			loading = false;
		}
	}

	function analyzeFieldCompletion(submissions: FormSubmission[]): FieldCompletionRow[] {
		if (!props.form.fields || submissions.length === 0) return [];

		const fieldStats: Record<string, { filled: number; total: number; field: FormField }> = {};

		// Initialize field stats
		props.form.fields.forEach((field) => {
			fieldStats[field.name] = {
				filled: 0,
				total: submissions.length,
				field
			};
		});

		// Count filled fields. `data.value` is a `JsonValue` — only call .trim()
		// when it's a string.
		submissions.forEach((submission) => {
			submission.data?.forEach((data: SubmissionData) => {
				const v = data.value;
				if (typeof v === 'string' ? v.trim() !== '' : v !== null && v !== undefined) {
					const stat = fieldStats[data.field_name];
					if (stat) {
						stat.filled++;
					}
				}
			});
		});

		return Object.values(fieldStats).map((stat) => ({
			label: stat.field.label,
			name: stat.field.name,
			completionRate: stat.total > 0 ? (stat.filled / stat.total) * 100 : 0,
			filled: stat.filled,
			total: stat.total
		}));
	}

	function getAverageCompletionTime(): string {
		// Placeholder - would need to track time on form
		return '2m 34s';
	}

	// Percentage helper used by the status-bar / read-rate visualisations.
	function pct(num: number | undefined, denom: number | undefined): number {
		if (!num || !denom || denom <= 0) return 0;
		return (num / denom) * 100;
	}

	let readRate = $derived(pct(stats?.read_count, stats?.total_submissions));
</script>

<div class="analytics-container">
	{#if loading}
		<div class="loading">Loading analytics...</div>
	{:else}
		<div class="analytics-grid">
			<!-- Key Metrics -->
			<div class="metric-card primary">
				<div class="metric-icon">📊</div>
				<div class="metric-content">
					<div class="metric-value">{stats?.total_submissions || 0}</div>
					<div class="metric-label">Total Submissions</div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon">📈</div>
				<div class="metric-content">
					<div class="metric-value">{stats?.recent_submissions || 0}</div>
					<div class="metric-label">Last 7 Days</div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon">✉️</div>
				<div class="metric-content">
					<div class="metric-value">{stats?.unread_count || 0}</div>
					<div class="metric-label">Unread</div>
				</div>
			</div>

			<div class="metric-card">
				<div class="metric-icon">⭐</div>
				<div class="metric-content">
					<div class="metric-value">{stats?.starred_count || 0}</div>
					<div class="metric-label">Starred</div>
				</div>
			</div>
		</div>

		<!-- Status Breakdown -->
		<div class="chart-card">
			<h3>Status Breakdown</h3>
			<div class="status-bars">
				<div class="status-bar">
					<div class="status-label">
						<span class="status-dot unread"></span>
						Unread ({stats?.unread_count || 0})
					</div>
					<div class="bar-track">
						<div
							class="bar-fill unread"
							style="width: {pct(stats?.unread_count, stats?.total_submissions)}%"
						></div>
					</div>
				</div>

				<div class="status-bar">
					<div class="status-label">
						<span class="status-dot read"></span>
						Read ({stats?.read_count || 0})
					</div>
					<div class="bar-track">
						<div
							class="bar-fill read"
							style="width: {pct(stats?.read_count, stats?.total_submissions)}%"
						></div>
					</div>
				</div>

				<div class="status-bar">
					<div class="status-label">
						<span class="status-dot starred"></span>
						Starred ({stats?.starred_count || 0})
					</div>
					<div class="bar-track">
						<div
							class="bar-fill starred"
							style="width: {pct(stats?.starred_count, stats?.total_submissions)}%"
						></div>
					</div>
				</div>

				<div class="status-bar">
					<div class="status-label">
						<span class="status-dot archived"></span>
						Archived ({stats?.archived_count || 0})
					</div>
					<div class="bar-track">
						<div
							class="bar-fill archived"
							style="width: {pct(stats?.archived_count, stats?.total_submissions)}%"
						></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Submission Trend -->
		{#if submissionTrend.length > 0}
			<div class="chart-card">
				<h3>Submission Trend (Last 30 Days)</h3>
				<div class="trend-chart">
					{#each submissionTrend.slice(-7) as day (day.date)}
						<div class="trend-bar">
							<div
								class="bar"
								style="height: {day.count > 0
									? Math.max(
											(day.count / Math.max(...submissionTrend.map((d) => d.count))) * 100,
											10
										)
									: 5}%"
								title="{day.count} submissions"
							></div>
							<div class="bar-label">{new Date(day.date).getDate()}</div>
						</div>
					{/each}
				</div>
				<div class="trend-summary">
					Last 7 days: {submissionTrend.slice(-7).reduce((sum, d) => sum + d.count, 0)} submissions
				</div>
			</div>
		{/if}

		<!-- Field Completion Rates -->
		{#if fieldAnalytics.length > 0}
			<div class="chart-card">
				<h3>Field Completion Rates</h3>
				<div class="field-completion">
					{#each fieldAnalytics
						.filter((f) => f.completionRate > 0)
						.slice(0, 10) as field (field.label)}
						<div class="completion-row">
							<div class="field-name">{field.label}</div>
							<div class="completion-bar">
								<div class="completion-fill" style="width: {field.completionRate}%"></div>
								<div class="completion-percentage">{field.completionRate.toFixed(0)}%</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Performance Metrics -->
		<div class="performance-grid">
			<div class="performance-card">
				<div class="performance-label">Read Rate</div>
				<div class="performance-value">{readRate.toFixed(1)}%</div>
				<div class="performance-chart">
					<div class="circle-progress" style="--progress: {readRate}"></div>
				</div>
			</div>

			<div class="performance-card">
				<div class="performance-label">Avg. Completion Time</div>
				<div class="performance-value">{getAverageCompletionTime()}</div>
				<div class="performance-note">Estimated</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.analytics-container {
		padding: 0;
	}

	.loading {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.metric-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		transition: all 0.2s;
	}

	.metric-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.2);
	}

	.metric-card.primary {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
		border-color: rgba(99, 102, 241, 0.3);
	}

	.metric-icon {
		font-size: 2.5rem;
		opacity: 0.8;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1;
	}

	.metric-label {
		font-size: 0.8125rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 0.25rem;
	}

	.chart-card {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.chart-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.status-bars {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.status-bar {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.status-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.status-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.status-dot.unread {
		background: #60a5fa;
	}
	.status-dot.read {
		background: #4ade80;
	}
	.status-dot.starred {
		background: #fbbf24;
	}
	.status-dot.archived {
		background: #9ca3af;
	}

	.bar-track {
		height: 8px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.bar-fill {
		height: 100%;
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.bar-fill.unread {
		background: #60a5fa;
	}
	.bar-fill.read {
		background: #4ade80;
	}
	.bar-fill.starred {
		background: #fbbf24;
	}
	.bar-fill.archived {
		background: #9ca3af;
	}

	.trend-chart {
		display: flex;
		align-items: flex-end;
		justify-content: space-around;
		height: 150px;
		gap: 0.5rem;
	}

	.trend-bar {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.trend-bar .bar {
		width: 100%;
		background: linear-gradient(180deg, #6366f1, #8b5cf6);
		border-radius: 4px 4px 0 0;
		transition: height 0.3s ease;
		min-height: 5px;
	}

	.bar-label {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.trend-summary {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		font-size: 0.875rem;
		color: #94a3b8;
		text-align: center;
	}

	.field-completion {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.completion-row {
		display: grid;
		grid-template-columns: 200px 1fr;
		gap: 1rem;
		align-items: center;
	}

	.field-name {
		font-size: 0.875rem;
		color: #e2e8f0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.completion-bar {
		position: relative;
		height: 24px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		overflow: hidden;
	}

	.completion-fill {
		height: 100%;
		background: linear-gradient(90deg, #6366f1, #8b5cf6);
		border-radius: 12px;
		transition: width 0.3s ease;
	}

	.completion-percentage {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		font-size: 0.75rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.performance-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1rem;
	}

	.performance-card {
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1.5rem;
		text-align: center;
	}

	.performance-label {
		font-size: 0.8125rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.performance-value {
		font-size: 2rem;
		font-weight: 700;
		color: #a5b4fc;
		margin-bottom: 0.5rem;
	}

	.performance-note {
		font-size: 0.75rem;
		color: #6b7280;
		font-style: italic;
	}

	@media (max-width: 767.98px) {
		.completion-row {
			grid-template-columns: 1fr;
			gap: 0.5rem;
		}
	}
</style>
