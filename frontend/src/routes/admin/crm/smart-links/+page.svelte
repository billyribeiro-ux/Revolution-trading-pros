<!--
	/admin/crm/smart-links - Action Link Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Smart link CRUD with automation triggers
	- Click tracking (total/unique)
	- Copy to clipboard functionality
	- Active/inactive filtering with API integration
	- Short URL display
	- Full Svelte 5 $state/$derived/$effect reactivity
	- Debounced search with server-side filtering
	- Toggle link status via API
-->

<script lang="ts">
	import { untrack } from 'svelte';
	import {
		IconLink,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconCopy,
		IconRefresh,
		IconClick,
		IconChartBar,
		IconExternalLink,
		IconToggleLeft,
		IconToggleRight,
		IconCheck
	} from '$lib/icons';
	import { crmAPI } from '$lib/api/crm';
	import type { SmartLink, SmartLinkFilters } from '$lib/crm/types';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// Reactive state using Svelte 5 runes
	let smartLinks = $state<SmartLink[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let filterActive = $state<boolean | 'all'>('all');
	let copySuccess = $state<string | null>(null);
	let togglingId = $state<string | null>(null);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<string | null>(null);

	// Debounced search query for API calls
	let debouncedSearch = $state('');
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	let stats = $state({
		total: 0,
		active: 0,
		totalClicks: 0,
		uniqueClicks: 0
	});

	// Derived statistics from loaded data
	let computedStats = $derived({
		total: smartLinks.length,
		active: smartLinks.filter((l) => l.is_active).length,
		totalClicks: smartLinks.reduce((sum, l) => sum + (l.click_count || 0), 0),
		uniqueClicks: smartLinks.reduce((sum, l) => sum + (l.unique_clicks || 0), 0)
	});

	// Filtered links for display (client-side filtering for immediate feedback)
	let filteredLinks = $derived(
		smartLinks.filter((link) => {
			const matchesSearch =
				!searchQuery ||
				link.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				link.short.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesActive = filterActive === 'all' || link.is_active === filterActive;
			return matchesSearch && matchesActive;
		})
	);

	// Load smart links from API
	async function loadSmartLinks() {
		isLoading = true;
		error = '';

		try {
			const filters: SmartLinkFilters = {
				search: debouncedSearch || undefined,
				is_active: filterActive !== 'all' ? filterActive : undefined
			};

			const response = await crmAPI.getSmartLinks(filters);
			smartLinks = response.data || [];

			// Update stats from computed values
			stats = computedStats;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load smart links';
			smartLinks = [];
		} finally {
			isLoading = false;
		}
	}

	// Toggle smart link active status via API
	async function toggleLinkStatus(link: SmartLink) {
		togglingId = link.id;
		error = '';

		try {
			const updated = await crmAPI.updateSmartLink(link.id, {
				is_active: !link.is_active
			});

			// Update local state optimistically
			smartLinks = smartLinks.map((l) =>
				l.id === link.id ? { ...l, is_active: updated.is_active } : l
			);

			// Update stats
			stats = computedStats;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update smart link status';
		} finally {
			togglingId = null;
		}
	}

	// Delete smart link with confirmation
	function deleteSmartLink(id: string) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteSmartLink() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;

		try {
			await crmAPI.deleteSmartLink(id);
			// Remove from local state immediately
			smartLinks = smartLinks.filter((l) => l.id !== id);
			stats = computedStats;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete smart link';
		}
	}

	// Copy URL to clipboard with feedback
	async function copyToClipboard(text: string, linkId: string) {
		try {
			await navigator.clipboard.writeText(text);
			copySuccess = linkId;
			setTimeout(() => {
				copySuccess = null;
			}, 2000);
		} catch (err) {
			error = 'Failed to copy to clipboard';
		}
	}

	// Format numbers with locale
	function formatNumber(num: number): string {
		return (num || 0).toLocaleString();
	}

	// Debounce search input - triggers API reload
	$effect(() => {
		const query = searchQuery;

		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		searchTimeout = setTimeout(() => {
			untrack(() => {
				debouncedSearch = query;
			});
		}, 300);

		return () => {
			if (searchTimeout) {
				clearTimeout(searchTimeout);
			}
		};
	});

	// Reload data when debounced search or filter changes (also handles initial load)
	$effect(() => {
		// Track dependencies - this effect runs on mount and when these change
		debouncedSearch;
		filterActive;

		// Load data (untrack to prevent infinite loops)
		untrack(() => {
			loadSmartLinks();
		});

		// Cleanup function (runs on destroy)
		return () => {
			if (searchTimeout) {
				clearTimeout(searchTimeout);
			}
		};
	});
</script>

<svelte:head>
	<title>Smart Links - FluentCRM Pro</title>
</svelte:head>

<div class="admin-crm-smart-links">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<h1>Smart Links</h1>
			<p class="subtitle">Create action links that trigger automations when clicked</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={() => loadSmartLinks()} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<a href="/admin/crm/smart-links/new" class="btn-primary">
					<IconPlus size={18} />
					New Smart Link
				</a>
			</div>
		</header>
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
			<div class="stat-icon gold">
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
			<input
				type="text"
				id="search-smart-links"
				name="search"
				placeholder="Search smart links..."
				bind:value={searchQuery}
			/>
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
								<button
									class="status-toggle {link.is_active ? 'active' : 'inactive'}"
									onclick={() => toggleLinkStatus(link)}
									disabled={togglingId === link.id}
									title={link.is_active ? 'Click to deactivate' : 'Click to activate'}
								>
									{#if togglingId === link.id}
										<span class="toggle-loading"></span>
									{:else if link.is_active}
										<IconToggleRight size={18} />
									{:else}
										<IconToggleLeft size={18} />
									{/if}
									<span>{link.is_active ? 'Active' : 'Inactive'}</span>
								</button>
							</td>
							<td>
								<div class="url-cell">
									<code class="short-url">{link.short}</code>
									<button
										class="btn-copy {copySuccess === link.id ? 'copied' : ''}"
										onclick={() => copyToClipboard(link.short_url || link.short, link.id)}
										title={copySuccess === link.id ? 'Copied!' : 'Copy URL'}
									>
										{#if copySuccess === link.id}
											<IconCheck size={14} />
										{:else}
											<IconCopy size={14} />
										{/if}
									</button>
								</div>
							</td>
							<td>
								<span class="target-url" title={link.target_url}>
									{link.target_url
										? link.target_url.length > 40
											? link.target_url.slice(0, 40) + '...'
											: link.target_url
										: '-'}
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
									<a
										href="/admin/crm/smart-links/{link.id}"
										class="btn-icon"
										title="View Analytics"
									>
										<IconChartBar size={16} />
									</a>
									<a href="/admin/crm/smart-links/{link.id}/edit" class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</a>
									<button
										class="btn-icon danger"
										title="Delete"
										onclick={() => deleteSmartLink(link.id)}
									>
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

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Smart Link"
	message="Are you sure you want to delete this smart link? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSmartLink}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<style>
	.admin-crm-smart-links {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	.btn-secondary {
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

	/* Unused - keeping for future use
	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #E6B800;
	}*/

	.btn-secondary :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
		border: none;
		border-radius: 8px;
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
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
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

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.stat-icon.gold {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}
	.stat-icon.amber {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

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
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		font-size: 0.8rem;
		color: var(--primary-500);
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
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
	}

	.btn-copy.copied {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.target-url {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.click-stats {
		font-family: monospace;
		font-size: 0.85rem;
	}

	.status-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.35rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.status-toggle.active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-toggle.active:hover:not(:disabled) {
		background: rgba(34, 197, 94, 0.3);
	}

	.status-toggle.inactive {
		background: rgba(148, 163, 184, 0.2);
		color: #94a3b8;
	}

	.status-toggle.inactive:hover:not(:disabled) {
		background: rgba(148, 163, 184, 0.3);
	}

	.status-toggle:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.toggle-loading {
		width: 14px;
		height: 14px;
		border: 2px solid currentColor;
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
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
		color: var(--primary-500);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.loading-state,
	.error-state,
	.empty-state {
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
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>
