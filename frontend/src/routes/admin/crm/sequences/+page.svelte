<script lang="ts">
	import { onMount } from 'svelte';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconMailForward from '@tabler/icons-svelte/icons/mail-forward';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte/icons/player-pause';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import { crmAPI } from '$lib/api/crm';
	import type { EmailSequence, SequenceFilters, SequenceStatus } from '$lib/crm/types';

	let sequences = $state<EmailSequence[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<SequenceStatus | 'all'>('all');

	let stats = $state({
		total: 0,
		active: 0,
		totalSubscribers: 0,
		totalSent: 0
	});

	const statusOptions = [
		{ value: 'all', label: 'All Sequences' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'active', label: 'Active' },
		{ value: 'paused', label: 'Paused' },
		{ value: 'completed', label: 'Completed' }
	];

	async function loadSequences() {
		isLoading = true;
		error = '';

		try {
			const filters: SequenceFilters = {
				search: searchQuery || undefined,
				status: selectedStatus !== 'all' ? selectedStatus : undefined
			};

			const response = await crmAPI.getSequences(filters);
			sequences = response.data || [];

			stats = {
				total: sequences.length,
				active: sequences.filter(s => s.status === 'active').length,
				totalSubscribers: sequences.reduce((sum, s) => sum + s.subscribers_count, 0),
				totalSent: sequences.reduce((sum, s) => sum + s.total_sent, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load sequences';
		} finally {
			isLoading = false;
		}
	}

	async function duplicateSequence(id: string) {
		try {
			await crmAPI.duplicateSequence(id);
			await loadSequences();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate sequence';
		}
	}

	async function deleteSequence(id: string) {
		if (!confirm('Are you sure you want to delete this sequence?')) return;

		try {
			await crmAPI.deleteSequence(id);
			await loadSequences();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete sequence';
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatRate(sent: number, opened: number): string {
		if (sent === 0) return '0%';
		return ((opened / sent) * 100).toFixed(1) + '%';
	}

	function getStatusColor(status: SequenceStatus): string {
		const colors: Record<SequenceStatus, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			active: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-amber-500/20 text-amber-400',
			completed: 'bg-blue-500/20 text-blue-400'
		};
		return colors[status];
	}

	let filteredSequences = $derived(
		sequences.filter(seq => {
			const matchesSearch = !searchQuery ||
				seq.title.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || seq.status === selectedStatus;
			return matchesSearch && matchesStatus;
		})
	);

	onMount(() => {
		loadSequences();
	});
</script>

<svelte:head>
	<title>Email Sequences - FluentCRM Pro</title>
</svelte:head>

<div class="sequences-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Email Sequences</h1>
			<p class="page-description">Create automated drip campaigns for your contacts</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadSequences()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/sequences/new" class="btn-primary">
				<IconPlus size={18} />
				New Sequence
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconMail size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Sequences</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconPlayerPlay size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.active)}</span>
				<span class="stat-label">Active</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalSubscribers)}</span>
				<span class="stat-label">Total Subscribers</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconMailForward size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalSent)}</span>
				<span class="stat-label">Emails Sent</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search sequences..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={selectedStatus}>
			{#each statusOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</div>

	<!-- Sequences Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading sequences...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadSequences()}>Try Again</button>
		</div>
	{:else if filteredSequences.length === 0}
		<div class="empty-state">
			<IconMail size={48} />
			<h3>No sequences found</h3>
			<p>Create your first email sequence to start automating your campaigns</p>
			<a href="/admin/crm/sequences/new" class="btn-primary">
				<IconPlus size={18} />
				Create Sequence
			</a>
		</div>
	{:else}
		<div class="table-container">
			<table class="data-table">
				<thead>
					<tr>
						<th>Sequence</th>
						<th>Status</th>
						<th>Emails</th>
						<th>Subscribers</th>
						<th>Open Rate</th>
						<th>Click Rate</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredSequences as sequence}
						<tr>
							<td>
								<div class="sequence-cell">
									<div class="sequence-icon">
										<IconMail size={20} />
									</div>
									<div class="sequence-info">
										<span class="sequence-title">{sequence.title}</span>
										<span class="sequence-meta">
											<IconClock size={12} />
											{sequence.emails_count} emails
										</span>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(sequence.status)}">
									{sequence.status}
								</span>
							</td>
							<td>{sequence.emails_count}</td>
							<td>{formatNumber(sequence.subscribers_count)}</td>
							<td>{formatRate(sequence.total_sent, sequence.total_opened)}</td>
							<td>{formatRate(sequence.total_sent, sequence.total_clicked)}</td>
							<td>
								<div class="action-buttons">
									<a href="/admin/crm/sequences/{sequence.id}" class="btn-icon" title="View">
										<IconEye size={16} />
									</a>
									<a href="/admin/crm/sequences/{sequence.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button class="btn-icon" title="Duplicate" onclick={() => duplicateSequence(sequence.id)}>
										<IconCopy size={16} />
									</button>
									<button class="btn-icon danger" title="Delete" onclick={() => deleteSequence(sequence.id)}>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.sequences-page {
		max-width: 1600px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #FFD11A;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1200px) {
		.stats-grid { grid-template-columns: repeat(2, 1fr); }
	}

	@media (max-width: 640px) {
		.stats-grid { grid-template-columns: 1fr; }
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.purple { background: rgba(230, 184, 0, 0.15); color: #FFD11A; }
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.table-container {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(230, 184, 0, 0.05);
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.sequence-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.sequence-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.sequence-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.sequence-title {
		font-weight: 600;
		color: #f1f5f9;
	}

	.sequence-meta {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.status-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: #FFD11A;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #E6B800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>
