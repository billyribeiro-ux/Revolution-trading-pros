<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteDate } from 'svelte/reactivity';
	import { boardsAPI } from '#lib/api/boards.js';
	import type { TimeEntry, Board, TimeTrackingStats } from '#lib/boards/types.js';
	import {
		IconClock,
		IconArrowLeft,
		IconLayoutKanban,
		IconCurrencyDollar,
		IconDownload,
		IconRefresh
	} from '#lib/icons/index.js';

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

	const datePresets = [
		{ value: 'today', label: 'Today' },
		{ value: 'yesterday', label: 'Yesterday' },
		{ value: 'this_week', label: 'This Week' },
		{ value: 'last_week', label: 'Last Week' },
		{ value: 'this_month', label: 'This Month' },
		{ value: 'last_month', label: 'Last Month' }
	];

	onMount(async () => {
		// FIX-2026-04-26 (P2-2): Boards list is invariant for the lifetime of this view.
		// Was previously re-fetched on every filter change inside loadData(); now we load
		// it once on mount and re-use the result. loadData() only re-fetches entries.
		try {
			const boardsRes = await boardsAPI.getBoards();
			boards = boardsRes.data;
		} catch (error) {
			console.error('Failed to load boards:', error);
		}
		await loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const entriesRes = await boardsAPI.getTimeEntries({
				board_id: filterBoardId || undefined,
				user_id: filterUserId || undefined,
				date_from: filterDateFrom || undefined,
				date_to: filterDateTo || undefined,
				is_billable: filterBillable ?? undefined,
				per_page: 100
			});

			timeEntries = entriesRes.data;

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
		const today = new SvelteDate();
		const startOfWeek = new SvelteDate(today);
		startOfWeek.setDate(today.getDate() - today.getDay());

		switch (preset) {
			case 'today':
				filterDateFrom = today.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'yesterday':
				const yesterday = new SvelteDate(today);
				yesterday.setDate(yesterday.getDate() - 1);
				filterDateFrom = yesterday.toISOString().split('T')[0];
				filterDateTo = yesterday.toISOString().split('T')[0];
				break;
			case 'this_week':
				filterDateFrom = startOfWeek.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'last_week':
				const lastWeekStart = new SvelteDate(startOfWeek);
				lastWeekStart.setDate(lastWeekStart.getDate() - 7);
				const lastWeekEnd = new SvelteDate(startOfWeek);
				lastWeekEnd.setDate(lastWeekEnd.getDate() - 1);
				filterDateFrom = lastWeekStart.toISOString().split('T')[0];
				filterDateTo = lastWeekEnd.toISOString().split('T')[0];
				break;
			case 'this_month':
				const startOfMonth = new SvelteDate(today.getFullYear(), today.getMonth(), 1);
				filterDateFrom = startOfMonth.toISOString().split('T')[0];
				filterDateTo = today.toISOString().split('T')[0];
				break;
			case 'last_month':
				const lastMonthStart = new SvelteDate(today.getFullYear(), today.getMonth() - 1, 1);
				const lastMonthEnd = new SvelteDate(today.getFullYear(), today.getMonth(), 0);
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

<div class="time-page">
	<header class="time-header">
		<div class="time-container time-header__inner">
			<div class="time-header__title-row">
				<a href="/admin/boards" class="icon-button" aria-label="Back to boards">
					<IconArrowLeft size={20} aria-hidden="true" />
				</a>
				<div class="time-header__title-group">
					<div class="time-header__icon">
						<IconClock size={24} aria-hidden="true" />
					</div>
					<div>
						<h1>Time Tracking</h1>
						<p>Track and analyze time spent on tasks</p>
					</div>
				</div>
			</div>

			<div class="time-header__actions">
				<button
					type="button"
					onclick={loadData}
					class="icon-button"
					aria-label="Refresh time entries"
				>
					<IconRefresh size={20} aria-hidden="true" />
				</button>
				<button type="button" class="secondary-action">
					<IconDownload size={18} aria-hidden="true" />
					<span>Export</span>
				</button>
			</div>
		</div>
	</header>

	<main class="time-container time-main">
		<!-- Stats Cards -->
		{#if stats}
			<section class="stats-grid" aria-label="Time tracking summary">
				<article class="stat-card">
					<div class="stat-card__label-row">
						<div class="stat-card__icon stat-card__icon--blue">
							<IconClock size={20} aria-hidden="true" />
						</div>
						<span>Total Time</span>
					</div>
					<div class="stat-card__value">{formatMinutes(stats.total_minutes)}</div>
				</article>

				<article class="stat-card">
					<div class="stat-card__label-row">
						<div class="stat-card__icon stat-card__icon--green">
							<IconCurrencyDollar size={20} aria-hidden="true" />
						</div>
						<span>Billable Time</span>
					</div>
					<div class="stat-card__value">{formatMinutes(stats.billable_minutes)}</div>
				</article>

				<article class="stat-card">
					<div class="stat-card__label-row">
						<div class="stat-card__icon stat-card__icon--neutral">
							<IconClock size={20} aria-hidden="true" />
						</div>
						<span>Non-Billable</span>
					</div>
					<div class="stat-card__value">{formatMinutes(stats.non_billable_minutes)}</div>
				</article>

				<article class="stat-card">
					<div class="stat-card__label-row">
						<div class="stat-card__icon stat-card__icon--violet">
							<IconCurrencyDollar size={20} aria-hidden="true" />
						</div>
						<span>Total Cost</span>
					</div>
					<div class="stat-card__value">${stats.total_cost.toFixed(2)}</div>
				</article>
			</section>
		{/if}

		<!-- Filters -->
		<section class="filters-panel" aria-label="Time tracking filters">
			<div class="preset-group" aria-label="Date presets">
				{#each datePresets as preset (preset.value)}
					<button
						type="button"
						onclick={() => setDatePreset(preset.value)}
						class={{ 'preset-button': true, 'is-active': datePreset === preset.value }}
					>
						{preset.label}
					</button>
				{/each}
			</div>

			<div class="filter-controls">
				<label class="filter-field" for="time-board-filter">
					<span>Board</span>
					<select id="time-board-filter" bind:value={filterBoardId} onchange={() => loadData()}>
						<option value={null}>All Boards</option>
						{#each boards as board (board.id)}
							<option value={board.id}>{board.title}</option>
						{/each}
					</select>
				</label>

				<label class="filter-field" for="time-billable-filter">
					<span>Billable</span>
					<select id="time-billable-filter" bind:value={filterBillable} onchange={() => loadData()}>
						<option value={null}>All Entries</option>
						<option value={true}>Billable Only</option>
						<option value={false}>Non-Billable Only</option>
					</select>
				</label>

				<div class="date-range">
					<label class="filter-field" for="page-filterdatefrom">
						<span>From</span>
						<input autocomplete="off"
							id="page-filterdatefrom"
							name="page-filterdatefrom"
							type="date"
							bind:value={filterDateFrom}
							onchange={() => {
								datePreset = '';
								loadData();
							}}
						/>
					</label>
					<span class="date-range__separator" aria-hidden="true">to</span>
					<label class="filter-field" for="page-filterdateto">
						<span>To</span>
						<input autocomplete="off"
							id="page-filterdateto"
							name="page-filterdateto"
							type="date"
							bind:value={filterDateTo}
							onchange={() => {
								datePreset = '';
								loadData();
							}}
						/>
					</label>
				</div>
			</div>
		</section>

		<!-- Time Entries Table -->
		<section class="entries-panel" aria-label="Time entries">
			{#if loading}
				<div class="table-state" aria-live="polite" aria-label="Loading time entries">
					<div class="loading-spinner"></div>
				</div>
			{:else if timeEntries.length === 0}
				<div class="empty-state">
					<div class="empty-state__icon">
						<IconClock size={48} aria-hidden="true" />
					</div>
					<h3>No time entries</h3>
					<p>No time has been logged for the selected filters</p>
				</div>
			{:else}
				<div class="table-scroll">
					<table class="entries-table">
						<thead>
							<tr>
								<th scope="col">Task</th>
								<th scope="col">User</th>
								<th scope="col">Date</th>
								<th scope="col">Duration</th>
								<th scope="col">Billable</th>
								<th scope="col">Cost</th>
								<th scope="col">Description</th>
							</tr>
						</thead>
						<tbody>
							{#each timeEntries as entry (entry.id)}
								<tr>
									<td>
										<div class="cell-with-icon">
											<IconLayoutKanban size={16} aria-hidden="true" />
											<span class="task-title">{entry.task?.title || 'Unknown Task'}</span>
										</div>
									</td>
									<td>
										<div class="cell-with-icon">
											<div class="user-avatar" aria-hidden="true">
												{entry.user?.name?.charAt(0).toUpperCase() || 'U'}
											</div>
											<span>{entry.user?.name || 'Unknown'}</span>
										</div>
									</td>
									<td>
										<div class="date-cell">
											<span>{formatDate(entry.created_at)}</span>
											{#if entry.started_at}
												<span class="date-cell__time">
													{formatTime(entry.started_at)}
													{#if entry.ended_at}
														- {formatTime(entry.ended_at)}
													{/if}
												</span>
											{/if}
										</div>
									</td>
									<td>
										<strong class="entry-duration">{formatMinutes(entry.minutes)}</strong>
									</td>
									<td>
										{#if entry.is_billable}
											<span class="status-pill status-pill--billable">Billable</span>
										{:else}
											<span class="status-pill">Non-billable</span>
										{/if}
									</td>
									<td>
										{#if entry.is_billable && entry.hourly_rate}
											${((entry.minutes / 60) * entry.hourly_rate).toFixed(2)}
										{:else}
											-
										{/if}
									</td>
									<td>
										<span class="description-cell">{entry.description || '-'}</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		</section>
	</main>
</div>

<style>
	.time-page {
		min-height: 100vh;
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
	}

	:global(.dark) .time-page {
		background: var(--bg-base, #0d1117);
		color: white;
	}

	.time-container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding-inline: 1rem;
	}

	.time-header {
		border-bottom: 1px solid var(--border-default, #30363d);
		background: white;
	}

	:global(.dark) .time-header {
		border-bottom-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.time-header__inner,
	.time-header__title-row,
	.time-header__title-group,
	.time-header__actions,
	.stat-card__label-row,
	.secondary-action,
	.cell-with-icon,
	.date-range {
		display: flex;
		align-items: center;
	}

	.time-header__inner {
		justify-content: space-between;
		gap: 1.5rem;
		padding-block: 1.5rem;
	}

	.time-header__title-row {
		gap: 1rem;
		min-width: 0;
	}

	.time-header__title-group {
		gap: 0.75rem;
		min-width: 0;
	}

	.time-header__icon,
	.stat-card__icon,
	.empty-state__icon,
	.icon-button {
		display: inline-grid;
		place-items: center;
	}

	.time-header__icon {
		width: 2.75rem;
		height: 2.75rem;
		flex: 0 0 auto;
		border-radius: 0.75rem;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--primary-500, #e6b800);
	}

	:global(.dark) .time-header__icon {
		background: rgba(230, 184, 0, 0.28);
		color: var(--primary-400, #ffd11a);
	}

	.time-header h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.time-header p {
		margin: 0.2rem 0 0;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	:global(.dark) .time-header p {
		color: var(--text-secondary, #8b949e);
	}

	.time-header__actions {
		gap: 0.75rem;
	}

	.icon-button,
	.secondary-action,
	.preset-button {
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
		color: var(--text-tertiary, #6e7681);
		text-decoration: none;
	}

	.icon-button:hover {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}

	:global(.dark) .icon-button {
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .icon-button:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.secondary-action {
		gap: 0.5rem;
		min-height: 2.5rem;
		border-radius: 0.625rem;
		background: transparent;
		color: var(--text-primary, #f0f6fc);
		padding: 0.5rem 0.875rem;
	}

	.secondary-action:hover {
		background: var(--bg-surface, #1c2128);
	}

	:global(.dark) .secondary-action {
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .secondary-action:hover {
		background: var(--bg-hover, #252b33);
	}

	.time-main {
		padding-block: 2rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(1, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card,
	.filters-panel,
	.entries-panel {
		border: 1px solid var(--border-default, #30363d);
		background: white;
	}

	:global(.dark) .stat-card,
	:global(.dark) .filters-panel,
	:global(.dark) .entries-panel {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.stat-card {
		border-radius: 0.75rem;
		padding: 1.5rem;
	}

	.stat-card__label-row {
		gap: 0.75rem;
		margin-bottom: 0.5rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	:global(.dark) .stat-card__label-row {
		color: var(--text-secondary, #8b949e);
	}

	.stat-card__icon {
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 0.625rem;
	}

	.stat-card__icon--blue {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
	}

	.stat-card__icon--green {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.stat-card__icon--neutral {
		background: var(--bg-surface, #1c2128);
		color: var(--text-tertiary, #6e7681);
	}

	.stat-card__icon--violet {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--primary-500, #e6b800);
	}

	:global(.dark) .stat-card__icon--blue {
		background: rgba(88, 166, 255, 0.24);
		color: var(--info-emphasis, #58a6ff);
	}

	:global(.dark) .stat-card__icon--green {
		background: rgb(22 163 74 / 0.24);
		color: var(--success-emphasis, #3fb950);
	}

	:global(.dark) .stat-card__icon--neutral {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .stat-card__icon--violet {
		background: rgba(230, 184, 0, 0.24);
		color: var(--primary-400, #ffd11a);
	}

	.stat-card__value {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.filters-panel {
		display: grid;
		gap: 1rem;
		margin-bottom: 1.5rem;
		border-radius: 0.75rem;
		padding: 1rem;
	}

	.preset-group {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		border-radius: 0.625rem;
		background: var(--bg-surface, #1c2128);
		padding: 0.25rem;
	}

	:global(.dark) .preset-group {
		background: var(--bg-hover, #252b33);
	}

	.preset-button {
		min-height: 2.125rem;
		border-radius: 0.5rem;
		background: transparent;
		color: var(--text-tertiary, #6e7681);
		padding: 0.375rem 0.75rem;
		font-size: 0.875rem;
	}

	.preset-button:hover,
	.preset-button.is-active {
		background: white;
		color: var(--text-primary, #f0f6fc);
		box-shadow: 0 1px 3px rgb(17 24 39 / 0.12);
	}

	:global(.dark) .preset-button {
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .preset-button:hover,
	:global(.dark) .preset-button.is-active {
		background: var(--bg-active, #2d333b);
		color: white;
	}

	.filter-controls {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.filter-field {
		display: grid;
		gap: 0.375rem;
		min-width: 10rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		font-weight: 600;
	}

	:global(.dark) .filter-field {
		color: var(--text-secondary, #8b949e);
	}

	.filter-field select,
	.filter-field input {
		min-height: 2.375rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.625rem;
		background: white;
		color: var(--text-primary, #f0f6fc);
		font: inherit;
		font-size: 0.875rem;
		font-weight: 400;
		padding: 0.375rem 0.75rem;
	}

	.filter-field select:focus,
	.filter-field input:focus {
		border-color: var(--primary-500, #e6b800);
		box-shadow: 0 0 0 3px rgb(79 70 229 / 0.14);
		outline: none;
	}

	:global(.dark) .filter-field select,
	:global(.dark) .filter-field input {
		border-color: var(--border-default, #30363d);
		background: var(--bg-hover, #252b33);
		color: white;
	}

	.date-range {
		gap: 0.5rem;
	}

	.date-range__separator {
		margin-top: 1.35rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	.entries-panel {
		overflow: hidden;
		border-radius: 0.75rem;
	}

	.table-state,
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
		border: 2px solid rgba(230, 184, 0, 0.22);
		border-bottom-color: var(--primary-500, #e6b800);
		border-radius: 999px;
		animation: time-spin 760ms linear infinite;
	}

	.empty-state {
		gap: 0.5rem;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state__icon {
		width: 4rem;
		height: 4rem;
		margin-bottom: 0.25rem;
		color: var(--text-secondary, #8b949e);
	}

	.empty-state h3 {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.125rem;
		font-weight: 600;
	}

	.empty-state p {
		margin: 0;
	}

	:global(.dark) .empty-state,
	:global(.dark) .empty-state__icon {
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .empty-state h3 {
		color: white;
	}

	.table-scroll {
		overflow-x: auto;
	}

	.entries-table {
		width: 100%;
		min-width: 58rem;
		border-collapse: collapse;
	}

	.entries-table thead {
		background: var(--bg-elevated, #161b22);
	}

	:global(.dark) .entries-table thead {
		background: var(--bg-hover, #252b33);
	}

	.entries-table th,
	.entries-table td {
		padding: 0.75rem 1rem;
		text-align: left;
		vertical-align: middle;
	}

	.entries-table th {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
	}

	:global(.dark) .entries-table th {
		color: var(--text-secondary, #8b949e);
	}

	.entries-table tbody tr {
		border-top: 1px solid var(--border-default, #30363d);
		transition: background 160ms ease;
	}

	.entries-table tbody tr:hover {
		background: var(--bg-elevated, #161b22);
	}

	:global(.dark) .entries-table tbody tr {
		border-top-color: var(--border-default, #30363d);
	}

	:global(.dark) .entries-table tbody tr:hover {
		background: rgb(55 65 81 / 0.5);
	}

	.entries-table td {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	:global(.dark) .entries-table td {
		color: var(--text-secondary, #8b949e);
	}

	.cell-with-icon {
		gap: 0.5rem;
		min-width: 0;
	}

	.cell-with-icon > :global(svg) {
		flex: 0 0 auto;
		color: var(--text-secondary, #8b949e);
	}

	.task-title,
	.entry-duration {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
	}

	:global(.dark) .task-title,
	:global(.dark) .entry-duration {
		color: white;
	}

	.user-avatar {
		display: inline-grid;
		width: 1.5rem;
		height: 1.5rem;
		flex: 0 0 auto;
		place-items: center;
		border-radius: 999px;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		font-size: 0.75rem;
		font-weight: 700;
	}

	.date-cell {
		display: grid;
		gap: 0.125rem;
	}

	.date-cell__time {
		color: var(--text-secondary, #8b949e);
		font-size: 0.8125rem;
	}

	.status-pill {
		display: inline-flex;
		border-radius: 0.375rem;
		background: var(--bg-surface, #1c2128);
		color: var(--text-tertiary, #6e7681);
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.status-pill--billable {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	:global(.dark) .status-pill {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	:global(.dark) .status-pill--billable {
		background: rgb(22 163 74 / 0.24);
		color: var(--success-emphasis, #3fb950);
	}

	.description-cell {
		display: inline-block;
		max-width: 20rem;
		overflow: hidden;
		color: var(--text-tertiary, #6e7681);
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	:global(.dark) .description-cell {
		color: var(--text-secondary, #8b949e);
	}

	@media (min-width: 640px) {
		.time-container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.time-container {
			padding-inline: 2rem;
		}
	}

	@media (max-width: 760px) {
		.time-header__inner,
		.time-header__actions,
		.filter-controls,
		.date-range {
			align-items: stretch;
			flex-direction: column;
		}

		.time-header__inner {
			align-items: flex-start;
		}

		.time-header__actions,
		.secondary-action,
		.filter-field {
			width: 100%;
		}

		.date-range__separator {
			display: none;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.icon-button,
		.secondary-action,
		.preset-button,
		.entries-table tbody tr {
			transition: none;
		}

		.loading-spinner {
			animation: none;
		}
	}

	@keyframes time-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
