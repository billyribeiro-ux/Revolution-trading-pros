<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconSearch,
		IconArrowForward,
		IconTrash,
		IconEyeOff,
		IconAlertTriangle
	} from '@tabler/icons-svelte';
	import CreateRedirectModal from '$lib/components/seo/CreateRedirectModal.svelte';

	let logs: any[] = $state([]);
	let stats: any = $state(null);
	let loading = $state(false);
	let searchQuery = $state('');
	let selectedIds: number[] = $state([]);
	let showCreateRedirect = $state(false);
	let selected404: any = $state(null);

	const sortOptions = [
		{ value: 'hits', label: 'Most Hits' },
		{ value: 'latest', label: 'Latest' },
		{ value: 'oldest', label: 'Oldest' }
	];
	let sortBy = $state('hits');

	onMount(() => {
		loadLogs();
		loadStats();
	});

	async function loadLogs() {
		loading = true;
		try {
			const response = await fetch(`/api/seo/404-logs?sort=${sortBy}`);
			const data = await response.json();
			logs = data.data || [];
		} catch (error) {
			console.error('Failed to load 404 logs:', error);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const response = await fetch('/api/seo/404-logs/stats');
			stats = await response.json();
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	function createRedirect(log: any) {
		selected404 = log;
		showCreateRedirect = true;
	}

	async function ignore(id: number) {
		try {
			await fetch(`/api/seo/404-logs/${id}/ignore`, { method: 'POST' });
			loadLogs();
			loadStats();
		} catch (error) {
			console.error('Failed to ignore log:', error);
		}
	}

	async function deleteLogs(ids: number[]) {
		if (!confirm(`Delete ${ids.length} log(s)?`)) return;

		try {
			await fetch('/api/seo/404-logs/bulk-delete', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ ids })
			});
			selectedIds = [];
			loadLogs();
			loadStats();
		} catch (error) {
			console.error('Failed to delete logs:', error);
		}
	}

	function toggleSelection(id: number) {
		if (selectedIds.includes(id)) {
			selectedIds = selectedIds.filter((i) => i !== id);
		} else {
			selectedIds = [...selectedIds, id];
		}
	}

	function handleRedirectCreated() {
		showCreateRedirect = false;
		selected404 = null;
		loadLogs();
		loadStats();
	}

	$effect(() => {
		if (sortBy) {
			loadLogs();
		}
	});

	let filteredLogs = $derived(
		logs.filter(
			(log) =>
				log.url.toLowerCase().includes(searchQuery.toLowerCase()) ||
				log.referer?.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
</script>

<svelte:head>
	<title>404 Monitor | SEO</title>
</svelte:head>

<div class="monitor-page">
	<header class="page-header">
		<div>
			<h1>404 Error Monitor</h1>
			<p>Track 404 errors and convert them to redirects</p>
		</div>
	</header>

	{#if stats}
		<div class="stats-grid">
			<div class="stat-card warning">
				<IconAlertTriangle size={32} />
				<div class="stat-value">{stats.total}</div>
				<div class="stat-label">Total 404s</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.unresolved}</div>
				<div class="stat-label">Unresolved</div>
			</div>
			<div class="stat-card success">
				<div class="stat-value">{stats.resolved}</div>
				<div class="stat-label">Resolved</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.total_hits?.toLocaleString() || 0}</div>
				<div class="stat-label">Total Hits</div>
			</div>
		</div>
	{/if}

	<div class="controls-bar">
		<div class="search-box">
			<IconSearch size={20} />
			<input type="text" bind:value={searchQuery} placeholder="Search URLs..." />
		</div>

		<select class="sort-select" bind:value={sortBy}>
			{#each sortOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>

		{#if selectedIds.length > 0}
			<button class="btn-danger" onclick={() => deleteLogs(selectedIds)}>
				<IconTrash size={18} />
				Delete ({selectedIds.length})
			</button>
		{/if}
	</div>

	<div class="logs-table">
		{#if loading}
			<div class="loading">Loading 404 logs...</div>
		{:else if filteredLogs.length === 0}
			<div class="empty-state">
				<h3>No 404 errors found</h3>
				<p>{searchQuery ? 'Try a different search' : 'Great! No 404 errors detected'}</p>
			</div>
		{:else}
			<table>
				<thead>
					<tr>
						<th style="width: 40px"><input type="checkbox" /></th>
						<th>URL</th>
						<th>Hits</th>
						<th>Last Hit</th>
						<th>Referer</th>
						<th>Status</th>
						<th style="width: 160px">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredLogs as log}
						<tr>
							<td>
								<input
									type="checkbox"
									checked={selectedIds.includes(log.id)}
									onchange={() => toggleSelection(log.id)}
								/>
							</td>
							<td>
								<div class="url-cell">
									{log.url}
								</div>
							</td>
							<td>
								<span class="hits-badge" class:high={log.hits > 10}>
									{log.hits}
								</span>
							</td>
							<td class="date-cell">
								{new Date(log.last_hit_at).toLocaleDateString()}
							</td>
							<td class="referer-cell">
								{log.referer || '—'}
							</td>
							<td>
								{#if log.is_resolved}
									<span class="status-badge resolved">Resolved</span>
								{:else if log.is_ignored}
									<span class="status-badge ignored">Ignored</span>
								{:else}
									<span class="status-badge pending">Pending</span>
								{/if}
							</td>
							<td>
								<div class="actions">
									{#if !log.is_resolved && !log.is_ignored}
										<button
											class="action-btn primary"
											onclick={() => createRedirect(log)}
											title="Create Redirect"
										>
											<IconArrowForward size={18} />
										</button>
										<button class="action-btn" onclick={() => ignore(log.id)} title="Ignore">
											<IconEyeOff size={18} />
										</button>
									{/if}
									<button
										class="action-btn danger"
										onclick={() => deleteLogs([log.id])}
										title="Delete"
									>
										<IconTrash size={18} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	</div>
</div>

{#if showCreateRedirect && selected404}
	<CreateRedirectModal
		url404={selected404.url}
		oncreated={handleRedirectCreated}
		oncancel={() => {
			showCreateRedirect = false;
			selected404 = null;
		}}
	/>
{/if}

<style>
	.monitor-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.stat-card.warning {
		background: #fef3c7;
		border-color: #fde047;
	}

	.stat-card.success {
		background: #dcfce7;
		border-color: #86efac;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0.5rem 0;
	}

	.stat-label {
		color: #666;
		font-size: 0.9rem;
	}

	.controls-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		align-items: center;
	}

	.search-box {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.95rem;
	}

	.sort-select {
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		cursor: pointer;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		background: #ef4444;
		color: white;
	}

	.btn-danger:hover {
		background: #dc2626;
	}

	.logs-table {
		background: white;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th {
		text-align: left;
		padding: 1rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
		font-weight: 600;
		color: #1a1a1a;
		font-size: 0.9rem;
	}

	td {
		padding: 1rem;
		border-bottom: 1px solid #f0f0f0;
		font-size: 0.95rem;
	}

	tbody tr:hover {
		background: #f8f9fa;
	}

	.url-cell {
		color: #dc2626;
		font-family: monospace;
		font-size: 0.9rem;
	}

	.hits-badge {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: #f0f0f0;
		color: #666;
		border-radius: 12px;
		font-size: 0.85rem;
		font-weight: 600;
	}

	.hits-badge.high {
		background: #fee2e2;
		color: #dc2626;
	}

	.date-cell {
		color: #666;
		font-size: 0.9rem;
	}

	.referer-cell {
		color: #999;
		font-size: 0.85rem;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.status-badge {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		font-size: 0.85rem;
		font-weight: 600;
	}

	.status-badge.pending {
		background: #fef3c7;
		color: #d97706;
	}

	.status-badge.resolved {
		background: #dcfce7;
		color: #16a34a;
	}

	.status-badge.ignored {
		background: #f0f0f0;
		color: #999;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		padding: 0.5rem;
		background: none;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f8f9fa;
	}

	.action-btn.primary {
		color: #3b82f6;
		border-color: #dbeafe;
	}

	.action-btn.primary:hover {
		background: #dbeafe;
	}

	.action-btn.danger {
		color: #ef4444;
		border-color: #fee2e2;
	}

	.action-btn.danger:hover {
		background: #fee2e2;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-state h3 {
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: #666;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: #999;
	}
</style>
