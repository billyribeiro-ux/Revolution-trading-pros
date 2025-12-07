<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconShare,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconPlayerPlay,
		IconPlayerPause,
		IconCopy,
		IconRefresh,
		IconUsers,
		IconChartBar,
		IconDownload,
		IconUpload,
		IconBolt
	} from '@tabler/icons-svelte';
	import { crmAPI } from '$lib/api/crm';
	import type { AutomationFunnel, FunnelFilters, FunnelStatus, TriggerType } from '$lib/crm/types';

	let funnels = $state<AutomationFunnel[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedStatus = $state<FunnelStatus | 'all'>('all');
	let selectedTrigger = $state<TriggerType | 'all'>('all');

	let stats = $state({
		total: 0,
		active: 0,
		totalSubscribers: 0,
		completed: 0
	});

	const statusOptions = [
		{ value: 'all', label: 'All Automations' },
		{ value: 'draft', label: 'Draft' },
		{ value: 'active', label: 'Active' },
		{ value: 'paused', label: 'Paused' }
	];

	const triggerLabels: Record<string, string> = {
		contact_created: 'Contact Created',
		tag_applied: 'Tag Applied',
		tag_removed: 'Tag Removed',
		list_applied: 'List Applied',
		list_removed: 'List Removed',
		form_submitted: 'Form Submitted',
		order_completed: 'Order Completed',
		subscription_started: 'Subscription Started',
		user_login: 'User Login',
		custom_event: 'Custom Event'
	};

	async function loadFunnels() {
		isLoading = true;
		error = '';

		try {
			const filters: FunnelFilters = {
				search: searchQuery || undefined,
				status: selectedStatus !== 'all' ? selectedStatus : undefined,
				trigger_type: selectedTrigger !== 'all' ? selectedTrigger : undefined
			};

			const response = await crmAPI.getAutomationFunnels(filters);
			funnels = response.data || [];

			stats = {
				total: funnels.length,
				active: funnels.filter(f => f.status === 'active').length,
				totalSubscribers: funnels.reduce((sum, f) => sum + f.subscribers_count, 0),
				completed: funnels.reduce((sum, f) => sum + f.completed_count, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load automations';
		} finally {
			isLoading = false;
		}
	}

	async function duplicateFunnel(id: string) {
		try {
			await crmAPI.duplicateAutomationFunnel(id);
			await loadFunnels();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate automation';
		}
	}

	async function deleteFunnel(id: string) {
		if (!confirm('Are you sure you want to delete this automation?')) return;

		try {
			await crmAPI.deleteAutomationFunnel(id);
			await loadFunnels();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete automation';
		}
	}

	async function exportFunnel(id: string) {
		try {
			const data = await crmAPI.exportAutomationFunnel(id);
			const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `automation-${id}.json`;
			a.click();
			URL.revokeObjectURL(url);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to export automation';
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function getStatusColor(status: FunnelStatus): string {
		const colors: Record<FunnelStatus, string> = {
			draft: 'bg-slate-500/20 text-slate-400',
			active: 'bg-emerald-500/20 text-emerald-400',
			paused: 'bg-amber-500/20 text-amber-400'
		};
		return colors[status];
	}

	let filteredFunnels = $derived(
		funnels.filter(funnel => {
			const matchesSearch = !searchQuery ||
				funnel.title.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || funnel.status === selectedStatus;
			const matchesTrigger = selectedTrigger === 'all' || funnel.trigger_type === selectedTrigger;
			return matchesSearch && matchesStatus && matchesTrigger;
		})
	);

	onMount(() => {
		loadFunnels();
	});
</script>

<svelte:head>
	<title>Automation Funnels - FluentCRM Pro</title>
</svelte:head>

<div class="automations-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Automation Funnels</h1>
			<p class="page-description">Create powerful marketing automations triggered by events</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadFunnels()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/automations/new" class="btn-primary">
				<IconPlus size={18} />
				New Automation
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconShare size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Automations</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconBolt size={24} />
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
				<span class="stat-label">Total Contacts</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconChartBar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.completed)}</span>
				<span class="stat-label">Completed</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search automations..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={selectedStatus}>
			{#each statusOptions as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</div>

	<!-- Automations Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading automations...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadFunnels()}>Try Again</button>
		</div>
	{:else if filteredFunnels.length === 0}
		<div class="empty-state">
			<IconShare size={48} />
			<h3>No automations found</h3>
			<p>Create your first automation to engage contacts automatically</p>
			<a href="/admin/crm/automations/new" class="btn-primary">
				<IconPlus size={18} />
				Create Automation
			</a>
		</div>
	{:else}
		<div class="table-container">
			<table class="data-table">
				<thead>
					<tr>
						<th>Automation</th>
						<th>Status</th>
						<th>Trigger</th>
						<th>Contacts</th>
						<th>Completed</th>
						<th>Completion Rate</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredFunnels as funnel}
						<tr>
							<td>
								<div class="funnel-cell">
									<div class="funnel-icon">
										<IconShare size={20} />
									</div>
									<div class="funnel-info">
										<span class="funnel-title">{funnel.title}</span>
										<span class="funnel-meta">
											{funnel.actions?.length || 0} actions
										</span>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {getStatusColor(funnel.status)}">
									{funnel.status}
								</span>
							</td>
							<td>
								<span class="trigger-badge">
									{triggerLabels[funnel.trigger_type] || funnel.trigger_type}
								</span>
							</td>
							<td>{formatNumber(funnel.subscribers_count)}</td>
							<td>{formatNumber(funnel.completed_count)}</td>
							<td>
								{funnel.subscribers_count > 0
									? ((funnel.completed_count / funnel.subscribers_count) * 100).toFixed(1) + '%'
									: '0%'}
							</td>
							<td>
								<div class="action-buttons">
									<a href="/admin/crm/automations/{funnel.id}" class="btn-icon" title="View">
										<IconEye size={16} />
									</a>
									<a href="/admin/crm/automations/{funnel.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button class="btn-icon" title="Duplicate" onclick={() => duplicateFunnel(funnel.id)}>
										<IconCopy size={16} />
									</button>
									<button class="btn-icon" title="Export" onclick={() => exportFunnel(funnel.id)}>
										<IconDownload size={16} />
									</button>
									<button class="btn-icon danger" title="Delete" onclick={() => deleteFunnel(funnel.id)}>
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
	.automations-page {
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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
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
		border: 1px solid rgba(99, 102, 241, 0.1);
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
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
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
		border: 1px solid rgba(99, 102, 241, 0.1);
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
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.table-container {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
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
		background: rgba(99, 102, 241, 0.05);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.data-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: #e2e8f0;
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.funnel-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.funnel-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #ec4899, #be185d);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.funnel-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.funnel-title {
		font-weight: 600;
		color: #f1f5f9;
	}

	.funnel-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.trigger-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #818cf8;
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
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
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
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>
