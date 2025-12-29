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
			}
		} catch (error) {
			console.error('Failed to load boards:', error);
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

	$effect(() => {
		if (selectedBoardId && period) {
			loadReport();
		}
	});

	function formatMinutes(minutes: number): string {
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		if (hours > 0) return `${hours}h ${mins}m`;
		return `${mins}m`;
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

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<a
						href="/admin/boards"
						class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconArrowLeft class="w-5 h-5" />
					</a>
					<div class="flex items-center gap-3">
						<div class="p-2 bg-indigo-100 dark:bg-indigo-900/30 rounded-lg">
							<IconChartBar class="w-6 h-6 text-indigo-600 dark:text-indigo-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Board Reports</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">Analytics and insights for your boards</p>
						</div>
					</div>
				</div>

				<div class="flex items-center gap-3">
					<button
						onclick={() => exportReport('csv')}
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
					>
						<IconDownload class="w-4 h-4" />
						Export CSV
					</button>
					<button
						onclick={() => exportReport('pdf')}
						class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg flex items-center gap-2"
					>
						<IconDownload class="w-4 h-4" />
						Export PDF
					</button>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Filters -->
		<div class="flex items-center gap-4 mb-8">
			<select
				bind:value={selectedBoardId}
				class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
			>
				{#each boards as board}
					<option value={board.id}>{board.title}</option>
				{/each}
			</select>

			<div class="flex items-center gap-1 bg-gray-100 dark:bg-gray-800 rounded-lg p-1">
				{#each [
					{ value: 'week', label: 'Week' },
					{ value: 'month', label: 'Month' },
					{ value: 'quarter', label: 'Quarter' },
					{ value: 'year', label: 'Year' }
				] as p}
					<button
						onclick={() => period = p.value as ReportPeriod}
						class="px-4 py-2 text-sm rounded-md {period === p.value ? 'bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm' : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'}"
					>
						{p.label}
					</button>
				{/each}
			</div>

			<button
				onclick={loadReport}
				class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
			>
				<IconRefresh class="w-5 h-5" />
			</button>
		</div>

		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
			</div>
		{:else if report}
			<!-- Overview Stats -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<div class="flex items-center justify-between mb-4">
						<span class="text-sm text-gray-500 dark:text-gray-400">Total Tasks</span>
						<IconChecks class="w-5 h-5 text-blue-500" />
					</div>
					<div class="text-3xl font-bold text-gray-900 dark:text-white">{report.total_tasks}</div>
				</div>

				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<div class="flex items-center justify-between mb-4">
						<span class="text-sm text-gray-500 dark:text-gray-400">Completed</span>
						<IconChecks class="w-5 h-5 text-green-500" />
					</div>
					<div class="text-3xl font-bold text-green-600">{report.completed_tasks}</div>
					<div class="text-sm text-gray-500 dark:text-gray-400">{report.completion_rate.toFixed(1)}% completion rate</div>
				</div>

				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<div class="flex items-center justify-between mb-4">
						<span class="text-sm text-gray-500 dark:text-gray-400">Overdue</span>
						<IconCalendar class="w-5 h-5 text-red-500" />
					</div>
					<div class="text-3xl font-bold text-red-600">{report.overdue_tasks}</div>
				</div>

				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<div class="flex items-center justify-between mb-4">
						<span class="text-sm text-gray-500 dark:text-gray-400">Avg. Completion Time</span>
						<IconClock class="w-5 h-5 text-purple-500" />
					</div>
					<div class="text-3xl font-bold text-gray-900 dark:text-white">{formatMinutes(report.avg_completion_time)}</div>
				</div>
			</div>

			<!-- Velocity -->
			<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700 mb-8">
				<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Velocity</h3>
				<div class="flex items-center gap-8">
					<div>
						<div class="text-sm text-gray-500 dark:text-gray-400 mb-1">This Period</div>
						<div class="text-2xl font-bold text-gray-900 dark:text-white">{report.velocity.current_period} tasks</div>
					</div>
					<div>
						<div class="text-sm text-gray-500 dark:text-gray-400 mb-1">Previous Period</div>
						<div class="text-2xl font-bold text-gray-600 dark:text-gray-400">{report.velocity.previous_period} tasks</div>
					</div>
					<div class="flex items-center gap-2">
						{#if report.velocity.trend === 'up'}
							<IconTrendingUp class="w-6 h-6 text-green-500" />
							<span class="text-green-600 font-medium">+{report.velocity.change_percentage.toFixed(1)}%</span>
						{:else if report.velocity.trend === 'down'}
							<IconTrendingDown class="w-6 h-6 text-red-500" />
							<span class="text-red-600 font-medium">{report.velocity.change_percentage.toFixed(1)}%</span>
						{:else}
							<span class="text-gray-500">No change</span>
						{/if}
					</div>
				</div>
			</div>

			<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
				<!-- Tasks by Stage -->
				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Tasks by Stage</h3>
					<div class="space-y-4">
						{#each report.tasks_by_stage as stage}
							<div>
								<div class="flex items-center justify-between mb-1">
									<span class="text-sm text-gray-700 dark:text-gray-300">{stage.stage_title}</span>
									<span class="text-sm text-gray-500">{stage.task_count} ({stage.percentage.toFixed(1)}%)</span>
								</div>
								<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
									<div
										class="bg-indigo-600 h-2 rounded-full transition-all"
										style="width: {stage.percentage}%"
									></div>
								</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- Tasks by Priority -->
				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Tasks by Priority</h3>
					<div class="space-y-4">
						{#each report.tasks_by_priority as priority}
							{@const colors = { urgent: 'bg-red-500', high: 'bg-orange-500', medium: 'bg-yellow-500', low: 'bg-blue-500', none: 'bg-gray-400' }}
							<div>
								<div class="flex items-center justify-between mb-1">
									<span class="text-sm text-gray-700 dark:text-gray-300 capitalize">{priority.priority}</span>
									<span class="text-sm text-gray-500">{priority.task_count} ({priority.percentage.toFixed(1)}%)</span>
								</div>
								<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
									<div
										class="{colors[priority.priority as keyof typeof colors]} h-2 rounded-full transition-all"
										style="width: {priority.percentage}%"
									></div>
								</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- Team Performance -->
				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Team Performance</h3>
					<div class="space-y-4">
						{#each report.tasks_by_assignee as assignee}
							<div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
								<div class="flex items-center gap-3">
									<div class="w-8 h-8 rounded-full bg-indigo-500 flex items-center justify-center text-white text-sm">
										{assignee.user_name.charAt(0).toUpperCase()}
									</div>
									<div>
										<div class="text-sm font-medium text-gray-900 dark:text-white">{assignee.user_name}</div>
										<div class="text-xs text-gray-500">{assignee.assigned_count} assigned, {assignee.completed_count} completed</div>
									</div>
								</div>
								<div class="text-right">
									<div class="text-sm font-medium text-gray-900 dark:text-white">{assignee.completion_rate.toFixed(0)}%</div>
									<div class="text-xs text-gray-500">completion</div>
								</div>
							</div>
						{/each}
					</div>
				</div>

				<!-- Labels Distribution -->
				<div class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
					<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Labels Distribution</h3>
					<div class="flex flex-wrap gap-3">
						{#each report.tasks_by_label as label}
							<div
								class="px-3 py-2 rounded-lg text-white text-sm"
								style="background-color: {label.label_color}"
							>
								{label.label_title}
								<span class="ml-2 opacity-75">{label.task_count}</span>
							</div>
						{/each}
					</div>
				</div>
			</div>
		{:else}
			<div class="text-center py-12 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700">
				<IconChartBar class="w-12 h-12 text-gray-400 mx-auto mb-4" />
				<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">No report available</h3>
				<p class="text-gray-500 dark:text-gray-400">Select a board to view its report</p>
			</div>
		{/if}
	</div>
</div>
