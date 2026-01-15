<script lang="ts">
	import { onMount } from 'svelte';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconPalette from '@tabler/icons-svelte/icons/palette';
	import { crmAPI } from '$lib/api/crm';
	import type { ContactTag } from '$lib/crm/types';

	let tags = $state<ContactTag[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');

	let stats = $state({
		total: 0,
		totalContacts: 0
	});

	async function loadTags() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getContactTags({
				search: searchQuery || undefined
			});
			tags = response.data || [];

			stats = {
				total: tags.length,
				totalContacts: tags.reduce((sum, t) => sum + t.contacts_count, 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load tags';
		} finally {
			isLoading = false;
		}
	}

	async function deleteTag(id: string) {
		if (!confirm('Are you sure you want to delete this tag? It will be removed from all contacts.')) return;

		try {
			await crmAPI.deleteContactTag(id);
			await loadTags();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete tag';
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString();
	}

	let filteredTags = $derived(
		tags.filter(tag => {
			return !searchQuery || tag.title.toLowerCase().includes(searchQuery.toLowerCase());
		})
	);

	onMount(() => {
		loadTags();
	});
</script>

<svelte:head>
	<title>Contact Tags - FluentCRM Pro</title>
</svelte:head>

<div class="tags-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Contact Tags</h1>
			<p class="page-description">Label and categorize your contacts with tags</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadTags()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/tags/new" class="btn-primary">
				<IconPlus size={18} />
				New Tag
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconTag size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Tags</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalContacts)}</span>
				<span class="stat-label">Tagged Contacts</span>
			</div>
		</div>
	</div>

	<!-- Search -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search tags..." bind:value={searchQuery} />
		</div>
	</div>

	<!-- Tags Grid -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading tags...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadTags()}>Try Again</button>
		</div>
	{:else if filteredTags.length === 0}
		<div class="empty-state">
			<IconTag size={48} />
			<h3>No tags found</h3>
			<p>Create your first tag to label your contacts</p>
			<a href="/admin/crm/tags/new" class="btn-primary">
				<IconPlus size={18} />
				Create Tag
			</a>
		</div>
	{:else}
		<div class="tags-grid">
			{#each filteredTags as tag}
				<div class="tag-card">
					<div class="tag-header">
						<div class="tag-color" style="background-color: {tag.color || '#E6B800'}"></div>
						<div class="tag-info">
							<h3 class="tag-title">{tag.title}</h3>
							{#if tag.description}
								<p class="tag-description">{tag.description}</p>
							{/if}
						</div>
					</div>
					<div class="tag-stats">
						<div class="tag-stat">
							<IconUsers size={16} />
							<span>{formatNumber(tag.contacts_count)} contacts</span>
						</div>
						<div class="tag-stat">
							<span class="tag-date">Created {formatDate(tag.created_at)}</span>
						</div>
					</div>
					<div class="tag-actions">
						<a href="/admin/crm/tags/{tag.id}" class="btn-icon" title="View Contacts">
							<IconUsers size={16} />
						</a>
						<a href="/admin/crm/tags/{tag.id}/edit" class="btn-icon" title="Edit">
							<IconEdit size={16} />
						</a>
						<button class="btn-icon danger" title="Delete" onclick={() => deleteTag(tag.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.tags-page {
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
		color: #E6B800;
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
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
		max-width: 600px;
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
	.stat-icon.purple { background: rgba(230, 184, 0, 0.15); color: #E6B800; }

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

	.tags-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
	}

	.tag-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.tag-header {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.tag-color {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		margin-top: 0.25rem;
		flex-shrink: 0;
	}

	.tag-info {
		flex: 1;
	}

	.tag-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.tag-description {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0;
		line-height: 1.4;
	}

	.tag-stats {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.tag-stat {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.tag-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.tag-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
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
		color: #E6B800;
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
