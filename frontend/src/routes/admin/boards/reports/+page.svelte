<script lang="ts">
	import { onMount } from 'svelte';
	import { boardsAPI } from '$lib/api/boards';
	import type { Board, BoardReport, ReportPeriod } from '$lib/boards/types';
	import {
		IconChartBar,
		IconArrowLeft,
		IconDownload,
		IconRefresh,
		IconTrendingUp,
		IconTrendingDown,
		IconChecks,
		IconClock,
		IconCalendar
	} from '$lib/icons';

	// State
	let boards = $state<Board[]>([]);
	let selectedBoardId = $state<string | null>(null);
	let report = $state<BoardReport | null>(null);
	let loading = $state(true);
	let period = $state<ReportPeriod>('month');

	const reportPeriods: { value: ReportPeriod; label: string }[] = [
		{ value: 'week', label: 'Week' },
		{ value: 'month', label: 'Month' },
		{ value: 'quarter', label: 'Quarter' },
		{ value: 'year', label: 'Year' }
	];

	onMount(async () => {
		await loadBoards();
	});

	async function loadBoards() {
		try {
			const res = await boardsAPI.getBoards();
			boards = res.data;
			if (boards.length > 0 && !selectedBoardId) {
				selectedBoardId = boards[0]?.id ?? null;
				await loadReport();
			} else {
				loading = false;
			}
		} catch (error) {
			console.error('Failed to load boards:', error);
			loading = false;
		}
	}

	async function loadReport() {
		if (!selectedBoardId) return;
		loading = true;
		try {
			report = await boardsAPI.getBoardReport(selectedBoardId, period);
		} catch (error) {
			console.error('Failed to load report:', error);
		} finally {
			loading = false;
		}
	}

	function formatMinutes(minutes: number): string {
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		if (hours > 0) return `${hours}h ${mins}m`;
		return `${mins}m`;
	}

	function handleBoardChange(event: Event) {
		selectedBoardId = (event.currentTarget as HTMLSelectElement).value || null;
		void loadReport();
	}

	function handlePeriodChange(value: ReportPeriod) {
		period = value;
		void loadReport();
	}

	function getPriorityTone(priority: string): 'urgent' | 'high' | 'medium' | 'low' | 'none' {
		if (
			priority === 'urgent' ||
			priority === 'high' ||
			priority === 'medium' ||
			priority === 'low'
		) {
			return priority;
		}
		return 'none';
	}

	async function exportReport(format: 'pdf' | 'csv') {
		if (!selectedBoardId) return;
		try {
			const blob = await boardsAPI.exportReport(selectedBoardId, period, format);
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `board-report-${period}.${format}`;
			a.click();
			URL.revokeObjectURL(url);
		} catch (error) {
			console.error('Failed to export report:', error);
		}
	}
</script>

<svelte:head>
	<title>Board Reports | Project Boards</title>
</svelte:head>

<div class="reports-page">
	<header class="reports-header">
		<div class="reports-container reports-header__inner">
			<div class="reports-header__title-row">
				<a href="/admin/boards" class="icon-button" aria-label="Back to boards">
					<IconArrowLeft size={20} aria-hidden="true" />
				</a>
				<div class="reports-header__title-group">
					<div class="reports-header__icon">
						<IconChartBar size={24} aria-hidden="true" />
					</div>
					<div>
						<h1>Board Reports</h1>
						<p>Analytics and insights for your boards</p>
					</div>
				</div>
			</div>

			<div class="reports-header__actions">
				<button type="button" onclick={() => exportReport('csv')} class="secondary-action">
					<IconDownload size={18} aria-hidden="true" />
					<span>Export CSV</span>
				</button>
				<button type="button" onclick={() => exportReport('pdf')} class="primary-action">
					<IconDownload size={18} aria-hidden="true" />
					<span>Export PDF</span>
				</button>
			</div>
		</div>
	</header>

	<main class="reports-container reports-main">
		<!-- Filters -->
		<section class="filters-panel" aria-label="Board report filters">
			<label class="filter-field" for="report-board-filter">
				<span>Board</span>
				<select id="report-board-filter" value={selectedBoardId} onchange={handleBoardChange}>
					{#each boards as board (board.id)}
						<option value={board.id}>{board.title}</option>
					{/each}
				</select>
			</label>

			<div class="period-group" aria-label="Report period">
				{#each reportPeriods as item (item.value)}
					<button
						type="button"
						onclick={() => handlePeriodChange(item.value)}
						class={{ 'period-button': true, 'is-active': period === item.value }}
					>
						{item.label}
					</button>
				{/each}
			</div>

			<button type="button" onclick={loadReport} class="icon-button" aria-label="Refresh report">
				<IconRefresh size={20} aria-hidden="true" />
			</button>
		</section>

		{#if loading}
			<div class="loading-state" aria-live="polite" aria-label="Loading report">
				<div class="loading-spinner"></div>
			</div>
		{:else if report}
			<!-- Overview Stats -->
			<section class="overview-grid" aria-label="Report overview">
				<article class="overview-card">
					<div class="overview-card__header">
						<span>Total Tasks</span>
						<IconChecks size={20} aria-hidden="true" />
					</div>
					<div class="overview-card__value">{report.total_tasks}</div>
				</article>

				<article class="overview-card">
					<div class="overview-card__header">
						<span>Completed</span>
						<IconChecks size={20} aria-hidden="true" />
					</div>
					<div class="overview-card__value" data-tone="success">{report.completed_tasks}</div>
					<div class="overview-card__note">
						{report.completion_rate.toFixed(1)}% completion rate
					</div>
				</article>

				<article class="overview-card">
					<div class="overview-card__header">
						<span>Overdue</span>
						<IconCalendar size={20} aria-hidden="true" />
					</div>
					<div class="overview-card__value" data-tone="danger">{report.overdue_tasks}</div>
				</article>

				<article class="overview-card">
					<div class="overview-card__header">
						<span>Avg. Completion Time</span>
						<IconClock size={20} aria-hidden="true" />
					</div>
					<div class="overview-card__value">{formatMinutes(report.avg_completion_time)}</div>
				</article>
			</section>

			<!-- Velocity -->
			<section class="panel velocity-panel" aria-label="Velocity">
				<h2>Velocity</h2>
				<div class="velocity-grid">
					<div class="velocity-metric">
						<span>This Period</span>
						<strong>{report.velocity.current_period} tasks</strong>
					</div>
					<div class="velocity-metric">
						<span>Previous Period</span>
						<strong>{report.velocity.previous_period} tasks</strong>
					</div>
					<div class="velocity-trend" data-trend={report.velocity.trend}>
						{#if report.velocity.trend === 'up'}
							<IconTrendingUp size={24} aria-hidden="true" />
							<strong>+{report.velocity.change_percentage.toFixed(1)}%</strong>
						{:else if report.velocity.trend === 'down'}
							<IconTrendingDown size={24} aria-hidden="true" />
							<strong>{report.velocity.change_percentage.toFixed(1)}%</strong>
						{:else}
							<span>No change</span>
						{/if}
					</div>
				</div>
			</section>

			<div class="report-sections">
				<!-- Tasks by Stage -->
				<section class="panel" aria-label="Tasks by stage">
					<h2>Tasks by Stage</h2>
					<div class="metric-list">
						{#each report.tasks_by_stage as stage (stage.stage_id)}
							<div class="metric-row">
								<div class="metric-row__header">
									<span>{stage.stage_title}</span>
									<span>{stage.task_count} ({stage.percentage.toFixed(1)}%)</span>
								</div>
								<div class="progress-track">
									<div class="progress-bar" style:width={`${stage.percentage}%`}></div>
								</div>
							</div>
						{/each}
					</div>
				</section>

				<!-- Tasks by Priority -->
				<section class="panel" aria-label="Tasks by priority">
					<h2>Tasks by Priority</h2>
					<div class="metric-list">
						{#each report.tasks_by_priority as priority (priority.priority)}
							<div class="metric-row">
								<div class="metric-row__header">
									<span class="priority-name">{priority.priority}</span>
									<span>{priority.task_count} ({priority.percentage.toFixed(1)}%)</span>
								</div>
								<div class="progress-track">
									<div
										class="progress-bar"
										data-priority={getPriorityTone(priority.priority)}
										style:width={`${priority.percentage}%`}
									></div>
								</div>
							</div>
						{/each}
					</div>
				</section>

				<!-- Team Performance -->
				<section class="panel" aria-label="Team performance">
					<h2>Team Performance</h2>
					<div class="assignee-list">
						{#each report.tasks_by_assignee as assignee (assignee.user_id)}
							<article class="assignee-card">
								<div class="assignee-card__person">
									<div class="assignee-avatar" aria-hidden="true">
										{assignee.user_name.charAt(0).toUpperCase()}
									</div>
									<div>
										<h3>{assignee.user_name}</h3>
										<p>{assignee.assigned_count} assigned, {assignee.completed_count} completed</p>
									</div>
								</div>
								<div class="assignee-card__rate">
									<strong>{assignee.completion_rate.toFixed(0)}%</strong>
									<span>completion</span>
								</div>
							</article>
						{/each}
					</div>
				</section>

				<!-- Labels Distribution -->
				<section class="panel" aria-label="Labels distribution">
					<h2>Labels Distribution</h2>
					<div class="label-list">
						{#each report.tasks_by_label as label (label.label_id)}
							<div class="label-chip" style:background-color={label.label_color}>
								{label.label_title}
								<span>{label.task_count}</span>
							</div>
						{/each}
					</div>
				</section>
			</div>
		{:else}
			<div class="empty-state">
				<div class="empty-state__icon">
					<IconChartBar size={48} aria-hidden="true" />
				</div>
				<h3>No report available</h3>
				<p>Select a board to view its report</p>
			</div>
		{/if}
	</main>
</div>

<style>
	.reports-page {
		min-height: 100vh;
		background: #f9fafb;
		color: #111827;
	}

	:global(.dark) .reports-page {
		background: #111827;
		color: white;
	}

	.reports-container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding-inline: 1rem;
	}

	.reports-header {
		border-bottom: 1px solid #e5e7eb;
		background: white;
	}

	:global(.dark) .reports-header {
		border-bottom-color: #374151;
		background: #1f2937;
	}

	.reports-header__inner,
	.reports-header__title-row,
	.reports-header__title-group,
	.reports-header__actions,
	.secondary-action,
	.primary-action,
	.filters-panel,
	.overview-card__header,
	.velocity-trend,
	.metric-row__header,
	.assignee-card,
	.assignee-card__person {
		display: flex;
		align-items: center;
	}

	.reports-header__inner {
		justify-content: space-between;
		gap: 1.5rem;
		padding-block: 1.5rem;
	}

	.reports-header__title-row {
		gap: 1rem;
		min-width: 0;
	}

	.reports-header__title-group {
		gap: 0.75rem;
		min-width: 0;
	}

	.reports-header__icon,
	.icon-button,
	.empty-state__icon {
		display: inline-grid;
		place-items: center;
	}

	.reports-header__icon {
		width: 2.75rem;
		height: 2.75rem;
		flex: 0 0 auto;
		border-radius: 0.75rem;
		background: #e0e7ff;
		color: #4f46e5;
	}

	:global(.dark) .reports-header__icon {
		background: rgb(79 70 229 / 0.28);
		color: #a5b4fc;
	}

	.reports-header h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.reports-header p {
		margin: 0.2rem 0 0;
		color: #6b7280;
		font-size: 0.875rem;
	}

	:global(.dark) .reports-header p {
		color: #9ca3af;
	}

	.reports-header__actions {
		gap: 0.75rem;
	}

	.icon-button,
	.secondary-action,
	.primary-action,
	.period-button {
		border: 0;
		font: inherit;
		transition:
			background 180ms ease,
			color 180ms ease,
			border-color 180ms ease,
			box-shadow 180ms ease;
	}

	.icon-button {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.625rem;
		background: transparent;
		color: #6b7280;
		text-decoration: none;
	}

	.icon-button:hover {
		background: #f3f4f6;
		color: #374151;
	}

	:global(.dark) .icon-button {
		color: #9ca3af;
	}

	:global(.dark) .icon-button:hover {
		background: #374151;
		color: #e5e7eb;
	}

	.secondary-action,
	.primary-action {
		gap: 0.5rem;
		min-height: 2.5rem;
		border-radius: 0.625rem;
		padding: 0.5rem 0.875rem;
	}

	.secondary-action {
		background: transparent;
		color: #374151;
	}

	.secondary-action:hover {
		background: #f3f4f6;
	}

	.primary-action {
		background: #4f46e5;
		color: white;
	}

	.primary-action:hover {
		background: #4338ca;
	}

	:global(.dark) .secondary-action {
		color: #d1d5db;
	}

	:global(.dark) .secondary-action:hover {
		background: #374151;
	}

	.reports-main {
		padding-block: 2rem;
	}

	.filters-panel {
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.filter-field {
		display: grid;
		gap: 0.375rem;
		min-width: 16rem;
		color: #4b5563;
		font-size: 0.75rem;
		font-weight: 600;
	}

	:global(.dark) .filter-field {
		color: #d1d5db;
	}

	.filter-field select {
		min-height: 2.5rem;
		border: 1px solid #d1d5db;
		border-radius: 0.625rem;
		background: white;
		color: #111827;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 400;
		padding: 0.5rem 0.875rem;
	}

	.filter-field select:focus {
		border-color: #4f46e5;
		box-shadow: 0 0 0 3px rgb(79 70 229 / 0.14);
		outline: none;
	}

	:global(.dark) .filter-field select {
		border-color: #4b5563;
		background: #1f2937;
		color: white;
	}

	.period-group {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		border-radius: 0.625rem;
		background: #f3f4f6;
		padding: 0.25rem;
	}

	:global(.dark) .period-group {
		background: #1f2937;
	}

	.period-button {
		min-height: 2.5rem;
		border-radius: 0.5rem;
		background: transparent;
		color: #4b5563;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.period-button:hover,
	.period-button.is-active {
		background: white;
		color: #111827;
		box-shadow: 0 1px 3px rgb(17 24 39 / 0.12);
	}

	:global(.dark) .period-button {
		color: #9ca3af;
	}

	:global(.dark) .period-button:hover,
	:global(.dark) .period-button.is-active {
		background: #374151;
		color: white;
	}

	.loading-state,
	.empty-state {
		display: grid;
		min-height: 14rem;
		place-items: center;
		padding: 3rem 1rem;
		text-align: center;
	}

	.loading-spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgb(79 70 229 / 0.22);
		border-bottom-color: #4f46e5;
		border-radius: 999px;
		animation: reports-spin 760ms linear infinite;
	}

	.overview-grid {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.overview-card,
	.panel,
	.empty-state {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: white;
	}

	:global(.dark) .overview-card,
	:global(.dark) .panel,
	:global(.dark) .empty-state {
		border-color: #374151;
		background: #1f2937;
	}

	.overview-card {
		padding: 1.5rem;
	}

	.overview-card__header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.overview-card__header :global(svg) {
		color: #3b82f6;
	}

	.overview-card:nth-child(2) .overview-card__header :global(svg) {
		color: #22c55e;
	}

	.overview-card:nth-child(3) .overview-card__header :global(svg) {
		color: #ef4444;
	}

	.overview-card:nth-child(4) .overview-card__header :global(svg) {
		color: #8b5cf6;
	}

	:global(.dark) .overview-card__header {
		color: #9ca3af;
	}

	.overview-card__value {
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.15;
	}

	.overview-card__value[data-tone='success'] {
		color: #16a34a;
	}

	.overview-card__value[data-tone='danger'] {
		color: #dc2626;
	}

	.overview-card__note {
		margin-top: 0.25rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	:global(.dark) .overview-card__note {
		color: #9ca3af;
	}

	.panel {
		padding: 1.5rem;
	}

	.panel h2 {
		margin: 0 0 1rem;
		font-size: 1.125rem;
		font-weight: 700;
	}

	.velocity-panel {
		margin-bottom: 2rem;
	}

	.velocity-grid {
		display: flex;
		align-items: center;
		gap: 2rem;
		flex-wrap: wrap;
	}

	.velocity-metric {
		display: grid;
		gap: 0.25rem;
	}

	.velocity-metric span {
		color: #6b7280;
		font-size: 0.875rem;
	}

	.velocity-metric strong {
		font-size: 1.5rem;
	}

	:global(.dark) .velocity-metric span {
		color: #9ca3af;
	}

	.velocity-trend {
		gap: 0.5rem;
		color: #6b7280;
	}

	.velocity-trend[data-trend='up'] {
		color: #16a34a;
	}

	.velocity-trend[data-trend='down'] {
		color: #dc2626;
	}

	.report-sections {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 2rem;
	}

	.metric-list,
	.assignee-list {
		display: grid;
		gap: 1rem;
	}

	.metric-row__header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.375rem;
		color: #374151;
		font-size: 0.875rem;
	}

	.metric-row__header span:last-child {
		color: #6b7280;
	}

	:global(.dark) .metric-row__header {
		color: #d1d5db;
	}

	:global(.dark) .metric-row__header span:last-child {
		color: #9ca3af;
	}

	.priority-name {
		text-transform: capitalize;
	}

	.progress-track {
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: #e5e7eb;
	}

	:global(.dark) .progress-track {
		background: #374151;
	}

	.progress-bar {
		height: 100%;
		border-radius: inherit;
		background: #4f46e5;
		transition: width 320ms ease;
	}

	.progress-bar[data-priority='urgent'] {
		background: #ef4444;
	}

	.progress-bar[data-priority='high'] {
		background: #f97316;
	}

	.progress-bar[data-priority='medium'] {
		background: #eab308;
	}

	.progress-bar[data-priority='low'] {
		background: #3b82f6;
	}

	.progress-bar[data-priority='none'] {
		background: #9ca3af;
	}

	.assignee-card {
		justify-content: space-between;
		gap: 1rem;
		border-radius: 0.625rem;
		background: #f9fafb;
		padding: 0.75rem;
	}

	:global(.dark) .assignee-card {
		background: #374151;
	}

	.assignee-card__person {
		gap: 0.75rem;
		min-width: 0;
	}

	.assignee-avatar {
		display: inline-grid;
		width: 2rem;
		height: 2rem;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 999px;
		background: #6366f1;
		color: white;
		font-size: 0.875rem;
		font-weight: 700;
	}

	.assignee-card h3,
	.assignee-card p {
		margin: 0;
	}

	.assignee-card h3 {
		font-size: 0.875rem;
		font-weight: 700;
	}

	.assignee-card p,
	.assignee-card__rate span {
		color: #6b7280;
		font-size: 0.75rem;
	}

	.assignee-card__rate {
		display: grid;
		justify-items: end;
		gap: 0.125rem;
		text-align: right;
	}

	.assignee-card__rate strong {
		font-size: 0.875rem;
	}

	:global(.dark) .assignee-card p,
	:global(.dark) .assignee-card__rate span {
		color: #9ca3af;
	}

	.label-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.label-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.625rem;
		color: white;
		padding: 0.5rem 0.75rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.label-chip span {
		opacity: 0.75;
	}

	.empty-state {
		gap: 0.5rem;
		color: #6b7280;
	}

	.empty-state__icon {
		width: 4rem;
		height: 4rem;
		margin-bottom: 0.25rem;
		color: #9ca3af;
	}

	.empty-state h3 {
		margin: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.empty-state p {
		margin: 0;
	}

	:global(.dark) .empty-state,
	:global(.dark) .empty-state__icon {
		color: #9ca3af;
	}

	:global(.dark) .empty-state h3 {
		color: white;
	}

	@media (min-width: 640px) {
		.reports-container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.overview-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.reports-container {
			padding-inline: 2rem;
		}

		.overview-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.report-sections {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (max-width: 760px) {
		.reports-header__inner,
		.reports-header__actions,
		.filters-panel,
		.velocity-grid {
			align-items: stretch;
			flex-direction: column;
		}

		.reports-header__inner {
			align-items: flex-start;
		}

		.reports-header__actions,
		.secondary-action,
		.primary-action,
		.filter-field {
			width: 100%;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.icon-button,
		.secondary-action,
		.primary-action,
		.period-button,
		.progress-bar {
			transition: none;
		}

		.loading-spinner {
			animation: none;
		}
	}

	@keyframes reports-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
