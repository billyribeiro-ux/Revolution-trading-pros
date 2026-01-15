<!--
	/admin/crm/smart-links - Action Link Management
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Smart link CRUD with automation triggers
	- Click tracking (total/unique)
	- Copy to clipboard functionality
	- Active/inactive filtering
	- Short URL display
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconLink,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconCopy,
		IconRefresh,
		IconClick,
		IconChartBar,
		IconExternalLink
	} from '$lib/icons';
	import { crmAPI } from '$lib/api/crm';
	import type { SmartLink, SmartLinkFilters } from '$lib/crm/types';

	let smartLinks = $state<SmartLink[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let filterActive = $state<boolean | 'all'>('all');

	let stats = $state({
		total: 0,
		active: 0,
		totalClicks: 0,
		uniqueClicks: 0
	});

	async function loadSmartLinks() {
		isLoading = true;
		error = '';

		try {
			const filters: SmartLinkFilters = {
				search: searchQuery || undefined,
				is_active: filterActive !== 'all' ? filterActive : undefined
			};

			const response = await crmAPI.getSmartLinks(filters);
			smartLinks = response.data || [];

			stats = {
				total: smartLinks.length,
				active: smartLinks.filter(l => l.is_active).length,
				totalClicks: smartLinks.reduce((sum, l) => sum + l.click_count, 0),
				uniqueClicks: smartLinks.reduce((sum, l) => sum + l.unique_clicks, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load smart links';
		} finally {
			isLoading = false;
		}
	}

	async function deleteSmartLink(id: string) {
		if (!confirm('Are you sure you want to delete this smart link?')) return;

		try {
			await crmAPI.deleteSmartLink(id);
			await loadSmartLinks();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete smart link';
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	let filteredLinks = $derived(
		smartLinks.filter(link => {
			const matchesSearch = !searchQuery ||
				link.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				link.short.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesActive = filterActive === 'all' || link.is_active === filterActive;
			return matchesSearch && matchesActive;
		})
	);

	onMount(() => {
		loadSmartLinks();
	});
</script>

<svelte:head>
	<title>Smart Links - FluentCRM Pro</title>
</svelte:head>

<div class="smart-links-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Smart Links</h1>
			<p class="page-description">Create action links that trigger automations when clicked</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadSmartLinks()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/smart-links/new" class="btn-primary">
				<IconPlus size={18} />
				New Smart Link
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconLink size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Links</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconExternalLink size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.active)}</span>
				<span class="stat-label">Active Links</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconClick size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalClicks)}</span>
				<span class="stat-label">Total Clicks</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconChartBar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.uniqueClicks)}</span>
				<span class="stat-label">Unique Clicks</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search smart links..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={filterActive}>
			<option value="all">All Links</option>
			<option value={true}>Active</option>
			<option value={false}>Inactive</option>
		</select>
	</div>

	<!-- Smart Links Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading smart links...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadSmartLinks()}>Try Again</button>
		</div>
	{:else if filteredLinks.length === 0}
		<div class="empty-state">
			<IconLink size={48} />
			<h3>No smart links found</h3>
			<p>Create your first smart link to trigger actions when contacts click</p>
			<a href="/admin/crm/smart-links/new" class="btn-primary">
				<IconPlus size={18} />
				Create Smart Link
			</a>
		</div>
	{:else}
		<div class="table-container">
			<table class="data-table">
				<thead>
					<tr>
						<th>Smart Link</th>
						<th>Status</th>
						<th>Short URL</th>
						<th>Target URL</th>
						<th>Actions</th>
						<th>Clicks</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					{#each filteredLinks as link}
						<tr>
							<td>
								<div class="link-cell">
									<div class="link-icon">
										<IconLink size={20} />
									</div>
									<div class="link-info">
										<span class="link-title">{link.title}</span>
										<span class="link-meta">{link.actions?.length || 0} actions configured</span>
									</div>
								</div>
							</td>
							<td>
								<span class="status-badge {link.is_active ? 'active' : 'inactive'}">
									{link.is_active ? 'Active' : 'Inactive'}
								</span>
							</td>
							<td>
								<div class="url-cell">
									<code class="short-url">{link.short}</code>
									<button class="btn-copy" onclick={() => copyToClipboard(link.short_url || link.short)} title="Copy">
										<IconCopy size={14} />
									</button>
								</div>
							</td>
							<td>
								<span class="target-url" title={link.target_url}>
									{link.target_url ? (link.target_url.length > 40 ? link.target_url.slice(0, 40) + '...' : link.target_url) : '-'}
								</span>
							</td>
							<td>
								{link.actions?.length || 0}
							</td>
							<td>
								<span class="click-stats">
									{formatNumber(link.click_count)} / {formatNumber(link.unique_clicks)}
								</span>
							</td>
							<td>
								<div class="action-buttons">
									<a href="/admin/crm/smart-links/{link.id}" class="btn-icon" title="View Analytics">
										<IconChartBar size={16} />
									</a>
									<a href="/admin/crm/smart-links/{link.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button class="btn-icon danger" title="Delete" onclick={() => deleteSmartLink(link.id)}>
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
	.smart-links-page {
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

	.link-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.link-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.link-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.link-title {
		font-weight: 600;
		color: #f1f5f9;
	}

	.link-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.url-cell {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.short-url {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.8rem;
		color: #818cf8;
	}

	.btn-copy {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.btn-copy:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.target-url {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.click-stats {
		font-family: monospace;
		font-size: 0.85rem;
	}

	.status-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.status-badge.active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-badge.inactive {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
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
