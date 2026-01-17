<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconPlus,
		IconSearch,
		IconTrash,
		IconEdit,
		IconToggleLeft,
		IconToggleRight,
		IconDownload,
		IconUpload,
		IconChartLine
	} from '$lib/icons';
	import RedirectEditor from '$lib/components/seo/RedirectEditor.svelte';

	let redirects: any[] = $state([]);
	let stats: any = $state(null);
	let loading = $state(false);
	let searchQuery = $state('');
	let showEditor = $state(false);
	let editingRedirect: any = $state(null);
	let selectedIds: number[] = $state([]);

	const filterTypes = ['all', '301', '302', '307', '308', '410'];
	let activeFilter = $state('all');

	onMount(() => {
		loadRedirects();
		loadStats();
	});

	async function loadRedirects() {
		loading = true;
		try {
			const response = await fetch('/api/seo/redirects');
			const data = await response.json();
			redirects = data.data || [];
		} catch (error) {
			console.error('Failed to load redirects:', error);
		} finally {
			loading = false;
		}
	}

	async function loadStats() {
		try {
			const response = await fetch('/api/seo/redirects/stats');
			stats = await response.json();
		} catch (error) {
			console.error('Failed to load stats:', error);
		}
	}

	function createRedirect() {
		editingRedirect = null;
		showEditor = true;
	}

	function editRedirect(redirect: any) {
		editingRedirect = redirect;
		showEditor = true;
	}

	async function deleteRedirect(id: number) {
		if (!confirm('Are you sure you want to delete this redirect?')) return;

		try {
			await fetch(`/api/seo/redirects/${id}`, { method: 'DELETE' });
			loadRedirects();
			loadStats();
		} catch (error) {
			console.error('Failed to delete redirect:', error);
		}
	}

	async function toggleRedirect(redirect: any) {
		try {
			await fetch(`/api/seo/redirects/${redirect.id}/toggle`, { method: 'POST' });
			redirect.is_active = !redirect.is_active;
			redirects = [...redirects];
		} catch (error) {
			console.error('Failed to toggle redirect:', error);
		}
	}

	function toggleSelection(id: number) {
		if (selectedIds.includes(id)) {
			selectedIds = selectedIds.filter((i) => i !== id);
		} else {
			selectedIds = [...selectedIds, id];
		}
	}

	async function bulkDelete() {
		if (!confirm(`Delete ${selectedIds.length} redirects?`)) return;

		try {
			await fetch('/api/seo/redirects/bulk-delete', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ ids: selectedIds })
			});
			selectedIds = [];
			loadRedirects();
			loadStats();
		} catch (error) {
			console.error('Failed to delete redirects:', error);
		}
	}

	function handleSaved() {
		showEditor = false;
		loadRedirects();
		loadStats();
	}

	let filteredRedirects = $derived(
		redirects.filter((redirect) => {
			const matchesSearch =
				redirect.source_url.toLowerCase().includes(searchQuery.toLowerCase()) ||
				redirect.destination_url.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesFilter = activeFilter === 'all' || redirect.redirect_type === activeFilter;

			return matchesSearch && matchesFilter;
		})
	);
</script>

<svelte:head>
	<title>Redirect Manager | SEO</title>
</svelte:head>

<div class="redirects-page">
	<header class="page-header">
		<div>
			<h1>Redirect Manager</h1>
			<p>Manage 301/302 redirects with regex support</p>
		</div>
		<button class="btn-primary" onclick={createRedirect}>
			<IconPlus size={18} />
			Add Redirect
		</button>
	</header>

	{#if stats}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-value">{stats.total}</div>
				<div class="stat-label">Total Redirects</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.active}</div>
				<div class="stat-label">Active</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.inactive}</div>
				<div class="stat-label">Inactive</div>
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
			<label for="search-redirects" class="sr-only">Search redirects</label>
			<input
				type="text"
				id="search-redirects"
				name="search"
				bind:value={searchQuery}
				placeholder="Search redirects..."
			/>
		</div>

		<div class="filter-tabs">
			{#each filterTypes as type}
				<button
					class="filter-tab"
					class:active={activeFilter === type}
					onclick={() => (activeFilter = type)}
				>
					{type === 'all' ? 'All' : type}
				</button>
			{/each}
		</div>

		{#if selectedIds.length > 0}
			<button class="btn-danger" onclick={bulkDelete}>
				<IconTrash size={18} />
				Delete ({selectedIds.length})
			</button>
		{/if}
	</div>

	<div class="redirects-table">
		{#if loading}
			<div class="loading">Loading redirects...</div>
		{:else if filteredRedirects.length === 0}
			<div class="empty-state">
				<h3>No redirects found</h3>
				<p>Create your first redirect to get started</p>
				<button class="btn-primary" onclick={createRedirect}>
					<IconPlus size={18} />
					Add Redirect
				</button>
			</div>
		{:else}
			<table>
				<thead>
					<tr>
						<th style="width: 40px"><input type="checkbox" /></th>
						<th>Source URL</th>
						<th>Destination URL</th>
						<th>Type</th>
						<th>Hits</th>
						<th>Status</th>
						<th style="width: 120px">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredRedirects as redirect}
						<tr>
							<td>
								<input
									type="checkbox"
									checked={selectedIds.includes(redirect.id)}
									onchange={() => toggleSelection(redirect.id)}
								/>
							</td>
							<td>
								<div class="url-cell">
									{redirect.source_url}
									{#if redirect.is_regex}
										<span class="regex-badge">REGEX</span>
									{/if}
								</div>
							</td>
							<td class="destination-url">{redirect.destination_url}</td>
							<td>
								<span class="type-badge type-{redirect.redirect_type}">
									{redirect.redirect_type}
								</span>
							</td>
							<td>{redirect.hits?.toLocaleString() || 0}</td>
							<td>
								<button
									class="status-toggle"
									class:active={redirect.is_active}
									onclick={() => toggleRedirect(redirect)}
								>
									{#if redirect.is_active}
										<IconToggleRight size={24} />
										Active
									{:else}
										<IconToggleLeft size={24} />
										Inactive
									{/if}
								</button>
							</td>
							<td>
								<div class="actions">
									<button class="action-btn" onclick={() => editRedirect(redirect)} title="Edit">
										<IconEdit size={18} />
									</button>
									<button
										class="action-btn danger"
										onclick={() => deleteRedirect(redirect.id)}
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

{#if showEditor}
	<RedirectEditor
		redirect={editingRedirect}
		onsaved={handleSaved}
		oncancel={() => (showEditor = false)}
	/>
{/if}

<style>
	.redirects-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
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

	.btn-primary,
	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		color: #0d1117;
	}

	.btn-primary:hover {
		background: linear-gradient(135deg, #ffd11a 0%, #e6b800 100%);
	}

	.btn-danger {
		background: #ef4444;
		color: white;
	}

	.btn-danger:hover {
		background: #dc2626;
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
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
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

	.filter-tabs {
		display: flex;
		gap: 0.5rem;
	}

	.filter-tab {
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		color: #666;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-tab:hover {
		background: #f8f9fa;
	}

	.filter-tab.active {
		background: #e6b800;
		color: #0d1117;
		border-color: #e6b800;
	}

	.redirects-table {
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
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.regex-badge {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.destination-url {
		color: #666;
		max-width: 300px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.type-badge {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		font-size: 0.85rem;
		font-weight: 600;
	}

	.type-badge.type-301 {
		background: #dcfce7;
		color: #16a34a;
	}

	.type-badge.type-302 {
		background: #fef3c7;
		color: #d97706;
	}

	.type-badge.type-307 {
		background: #dbeafe;
		color: #3b82f6;
	}

	.type-badge.type-308 {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}

	.type-badge.type-410 {
		background: #fee2e2;
		color: #dc2626;
	}

	.status-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: none;
		border: none;
		color: #999;
		cursor: pointer;
		font-size: 0.9rem;
	}

	.status-toggle.active {
		color: #16a34a;
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
		margin-bottom: 1.5rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: #999;
	}
</style>
