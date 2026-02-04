<script lang="ts">
	import { onMount } from 'svelte';
	import { boardsAPI } from '$lib/api/boards';
	import type { TimeEntry, Board, TimeTrackingStats } from '$lib/boards/types';
	import {
		IconClock,
		IconArrowLeft,
		IconCalendar,
		IconUser,
		IconLayoutKanban,
		IconCurrencyDollar,
		IconFilter,
		IconDownload,
		IconRefresh
	} from '$lib/icons';

	// State
	let timeEntries = $state<TimeEntry[]>([]);
	let boards = $state<Board[]>([]);
	let stats = $state<TimeTrackingStats | null>(null);
	let loading = $state(true);

	// Filters
	let filterBoardId = $state<string | null>(null);
	let filterUserId = $state<string | null>(null);
	let filterDateFrom = $state('');
	let filterDateTo = $state('');
	let filterBillable = $state<boolean | null>(null);

	// Date presets
	let datePreset = $state('this_week');

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const [entriesRes, boardsRes] = await Promise.all([
				boardsAPI.getTimeEntries({
					board_id: filterBoardId || undefined,
					user_id: filterUserId || undefined,
					date_from: filterDateFrom || undefined,
					date_to: filterDateTo || undefined,
					is_billable: filterBillable ?? undefined,
					per_page: 100
				}),
				boardsAPI.getBoards()
			]);

			timeEntries = entriesRes.data;
			boards = boardsRes.data;

			// Calculate stats
			const totalMinutes = timeEntries.reduce((sum, e) => sum + e.minutes, 0);
			const billableMinutes = timeEntries
				.filter((e) => e.is_billable)
				.reduce((sum, e) => sum + e.minutes, 0);
			const totalCost = timeEntries.reduce(
				(sum, e) => sum + (e.minutes / 60) * (e.hourly_rate || 0),
				0
			);

			stats = {
				total_minutes: totalMinutes,
				billable_minutes: billableMinutes,
				non_billable_minutes: totalMinutes - billableMinutes,
				total_cost: totalCost,
				by_user: [],
				by_task: [],
				by_date: []
			};
		} catch (error) {
			console.error('Failed to load time entries:', error);
		} finally {
			loading = false;
		}
	}

	function setDatePreset(preset: string) {
		datePreset = preset;
		const today = new Date();
		const startOfWeek = new Date(today);
		startOfWeek.setDate(today.getDate() - today.getDay());

		switch (preset) {
			case 'today':
				filterDateFrom = today.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'yesterday':
				const yesterday = new Date(today);
				yesterday.setDate(yesterday.getDate() - 1);
				filterDateFrom = yesterday.toISOString().split('T')[0];
				filterDateTo = yesterday.toISOString().split('T')[0];
				break;
			case 'this_week':
				filterDateFrom = startOfWeek.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'last_week':
				const lastWeekStart = new Date(startOfWeek);
				lastWeekStart.setDate(lastWeekStart.getDate() - 7);
				const lastWeekEnd = new Date(startOfWeek);
				lastWeekEnd.setDate(lastWeekEnd.getDate() - 1);
				filterDateFrom = lastWeekStart.toISOString().split('T')[0];
				filterDateTo = lastWeekEnd.toISOString().split('T')[0];
				break;
			case 'this_month':
				const startOfMonth = new Date(today.getFullYear(), today.getMonth(), 1);
				filterDateFrom = startOfMonth.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'last_month':
				const lastMonthStart = new Date(today.getFullYear(), today.getMonth() - 1, 1);
				const lastMonthEnd = new Date(today.getFullYear(), today.getMonth(), 0);
				filterDateFrom = lastMonthStart.toISOString().split('T')[0];
				filterDateTo = lastMonthEnd.toISOString().split('T')[0];
				break;
		}
		loadData();
	}

	function formatMinutes(minutes: number): string {
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		return `${hours}h ${mins}m`;
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function formatTime(dateStr: string): string {
		return new Date(dateStr).toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<svelte:head>
	<title>Time Tracking | Project Boards</title>
</svelte:head>

<div class="bg-gray-50 dark:bg-gray-900">
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
							<IconClock class="w-6 h-6 text-indigo-600 dark:text-indigo-400" />
						</div>
						<div>
							<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Time Tracking</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400">
								Track and analyze time spent on tasks
							</p>
						</div>
					</div>
				</div>

				<div class="flex items-center gap-3">
					<button
						onclick={loadData}
						class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconRefresh class="w-5 h-5" />
					</button>
					<button
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
					>
						<IconDownload class="w-4 h-4" />
						Export
					</button>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Stats Cards -->
		{#if stats}
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
				<div
					class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700"
				>
					<div class="flex items-center gap-3 mb-2">
						<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
							<IconClock class="w-5 h-5 text-blue-600 dark:text-blue-400" />
						</div>
						<span class="text-sm text-gray-500 dark:text-gray-400">Total Time</span>
					</div>
					<div class="text-2xl font-bold text-gray-900 dark:text-white">
						{formatMinutes(stats.total_minutes)}
					</div>
				</div>

				<div
					class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700"
				>
					<div class="flex items-center gap-3 mb-2">
						<div class="p-2 bg-green-100 dark:bg-green-900/30 rounded-lg">
							<IconCurrencyDollar class="w-5 h-5 text-green-600 dark:text-green-400" />
						</div>
						<span class="text-sm text-gray-500 dark:text-gray-400">Billable Time</span>
					</div>
					<div class="text-2xl font-bold text-gray-900 dark:text-white">
						{formatMinutes(stats.billable_minutes)}
					</div>
				</div>

				<div
					class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700"
				>
					<div class="flex items-center gap-3 mb-2">
						<div class="p-2 bg-gray-100 dark:bg-gray-700 rounded-lg">
							<IconClock class="w-5 h-5 text-gray-600 dark:text-gray-400" />
						</div>
						<span class="text-sm text-gray-500 dark:text-gray-400">Non-Billable</span>
					</div>
					<div class="text-2xl font-bold text-gray-900 dark:text-white">
						{formatMinutes(stats.non_billable_minutes)}
					</div>
				</div>

				<div
					class="bg-white dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700"
				>
					<div class="flex items-center gap-3 mb-2">
						<div class="p-2 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
							<IconCurrencyDollar class="w-5 h-5 text-purple-600 dark:text-purple-400" />
						</div>
						<span class="text-sm text-gray-500 dark:text-gray-400">Total Cost</span>
					</div>
					<div class="text-2xl font-bold text-gray-900 dark:text-white">
						${stats.total_cost.toFixed(2)}
					</div>
				</div>
			</div>
		{/if}

		<!-- Filters -->
		<div
			class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4 mb-6"
		>
			<div class="flex flex-wrap items-center gap-4">
				<!-- Date Presets -->
				<div class="flex items-center gap-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
					{#each [{ value: 'today', label: 'Today' }, { value: 'yesterday', label: 'Yesterday' }, { value: 'this_week', label: 'This Week' }, { value: 'last_week', label: 'Last Week' }, { value: 'this_month', label: 'This Month' }, { value: 'last_month', label: 'Last Month' }] as preset}
						<button
							onclick={() => setDatePreset(preset.value)}
							class="px-3 py-1.5 text-sm rounded-md {datePreset === preset.value
								? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
								: 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'}"
						>
							{preset.label}
						</button>
					{/each}
				</div>

				<!-- Board Filter -->
				<select
					bind:value={filterBoardId}
					onchange={() => loadData()}
					class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
				>
					<option value={null}>All Boards</option>
					{#each boards as board}
						<option value={board.id}>{board.title}</option>
					{/each}
				</select>

				<!-- Billable Filter -->
				<select
					bind:value={filterBillable}
					onchange={() => loadData()}
					class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
				>
					<option value={null}>All Entries</option>
					<option value={true}>Billable Only</option>
					<option value={false}>Non-Billable Only</option>
				</select>

				<!-- Custom Date Range -->
				<div class="flex items-center gap-2">
					<input
						id="page-filterdatefrom" name="page-filterdatefrom" type="date"
						bind:value={filterDateFrom}
						onchange={() => {
							datePreset = '';
							loadData();
						}}
						class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
					/>
					<span class="text-gray-500">to</span>
					<input
						id="page-filterdateto" name="page-filterdateto" type="date"
						bind:value={filterDateTo}
						onchange={() => {
							datePreset = '';
							loadData();
						}}
						class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
					/>
				</div>
			</div>
		</div>

		<!-- Time Entries Table -->
		<div
			class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
		>
			{#if loading}
				<div class="flex items-center justify-center py-12">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
				</div>
			{:else if timeEntries.length === 0}
				<div class="text-center py-12">
					<IconClock class="w-12 h-12 text-gray-400 mx-auto mb-4" />
					<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">No time entries</h3>
					<p class="text-gray-500 dark:text-gray-400">
						No time has been logged for the selected filters
					</p>
				</div>
			{:else}
				<table class="w-full">
					<thead class="bg-gray-50 dark:bg-gray-700">
						<tr>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Task</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>User</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Date</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Duration</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Billable</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Cost</th
							>
							<th
								class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
								>Description</th
							>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
						{#each timeEntries as entry}
							<tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
								<td class="px-4 py-3">
									<div class="flex items-center gap-2">
										<IconLayoutKanban class="w-4 h-4 text-gray-400" />
										<span class="text-sm font-medium text-gray-900 dark:text-white"
											>{entry.task?.title || 'Unknown Task'}</span
										>
									</div>
								</td>
								<td class="px-4 py-3">
									<div class="flex items-center gap-2">
										<div
											class="w-6 h-6 rounded-full bg-indigo-500 flex items-center justify-center text-white text-xs"
										>
											{entry.user?.name?.charAt(0).toUpperCase() || 'U'}
										</div>
										<span class="text-sm text-gray-600 dark:text-gray-300"
											>{entry.user?.name || 'Unknown'}</span
										>
									</div>
								</td>
								<td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
									{formatDate(entry.created_at)}
									{#if entry.started_at}
										<span class="text-gray-400">
											{formatTime(entry.started_at)}
											{#if entry.ended_at}
												- {formatTime(entry.ended_at)}
											{/if}
										</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-sm font-medium text-gray-900 dark:text-white">
									{formatMinutes(entry.minutes)}
								</td>
								<td class="px-4 py-3">
									{#if entry.is_billable}
										<span
											class="px-2 py-0.5 text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded"
										>
											Billable
										</span>
									{:else}
										<span
											class="px-2 py-0.5 text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded"
										>
											Non-billable
										</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
									{#if entry.is_billable && entry.hourly_rate}
										${((entry.minutes / 60) * entry.hourly_rate).toFixed(2)}
									{:else}
										-
									{/if}
								</td>
								<td class="px-4 py-3 text-sm text-gray-500 dark:text-gray-400 max-w-xs truncate">
									{entry.description || '-'}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	</div>
</div>
