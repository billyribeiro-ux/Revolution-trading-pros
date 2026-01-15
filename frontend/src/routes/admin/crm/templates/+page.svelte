<script lang="ts">
	import { onMount } from 'svelte';
	import IconTemplate from '@tabler/icons-svelte/icons/template';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconFolder from '@tabler/icons-svelte/icons/folder';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import { crmAPI } from '$lib/api/crm';
	import type { EmailTemplate, TemplateCategory } from '$lib/crm/types';

	let templates = $state<EmailTemplate[]>([]);
	let categories = $state<TemplateCategory[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let selectedCategory = $state('');

	let stats = $state({
		total: 0,
		byCategory: {} as Record<string, number>
	});

	async function loadTemplates() {
		isLoading = true;
		error = '';

		try {
			const [templatesResponse, categoriesResponse] = await Promise.all([
				crmAPI.getEmailTemplates({
					search: searchQuery || undefined,
					category: selectedCategory || undefined
				}),
				crmAPI.getTemplateCategories()
			]);

			templates = templatesResponse.data || [];
			categories = categoriesResponse || [];

			stats = {
				total: templates.length,
				byCategory: categories.reduce(
					(acc, cat) => {
						acc[cat.slug] = cat.templates_count;
						return acc;
					},
					{} as Record<string, number>
				)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load templates';
		} finally {
			isLoading = false;
		}
	}

	async function deleteTemplate(id: string) {
		if (!confirm('Are you sure you want to delete this template?')) return;

		try {
			await crmAPI.deleteEmailTemplate(id);
			await loadTemplates();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete template';
		}
	}

	async function duplicateTemplate(id: string) {
		try {
			await crmAPI.duplicateEmailTemplate(id);
			await loadTemplates();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate template';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString();
	}

	let filteredTemplates = $derived(
		templates.filter((template) => {
			if (selectedCategory && template.category !== selectedCategory) return false;
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				return template.title.toLowerCase().includes(query);
			}
			return true;
		})
	);

	onMount(() => {
		loadTemplates();
	});
</script>

<svelte:head>
	<title>Email Templates - FluentCRM Pro</title>
</svelte:head>

<div class="templates-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Email Templates</h1>
			<p class="page-description">Create and manage reusable email templates</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadTemplates()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/templates/prebuilt" class="btn-secondary">
				<IconDownload size={18} />
				Pre-built Templates
			</a>
			<a href="/admin/crm/templates/new" class="btn-primary">
				<IconPlus size={18} />
				New Template
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconTemplate size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.total}</span>
				<span class="stat-label">Total Templates</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconFolder size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{categories.length}</span>
				<span class="stat-label">Categories</span>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search templates..." bind:value={searchQuery} />
		</div>
		<select bind:value={selectedCategory} class="filter-select">
			<option value="">All Categories</option>
			{#each categories as category}
				<option value={category.slug}>{category.name} ({category.templates_count})</option>
			{/each}
		</select>
	</div>

	<!-- Templates Grid -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading templates...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadTemplates()}>Try Again</button>
		</div>
	{:else if filteredTemplates.length === 0}
		<div class="empty-state">
			<IconTemplate size={48} />
			<h3>No templates found</h3>
			<p>Create your first template or import from pre-built library</p>
			<div class="empty-actions">
				<a href="/admin/crm/templates/new" class="btn-primary">
					<IconPlus size={18} />
					Create Template
				</a>
				<a href="/admin/crm/templates/prebuilt" class="btn-secondary">
					<IconDownload size={18} />
					Pre-built Templates
				</a>
			</div>
		</div>
	{:else}
		<div class="templates-grid">
			{#each filteredTemplates as template}
				<div class="template-card">
					<div class="template-thumbnail">
						{#if template.thumbnail}
							<img src={template.thumbnail} alt={template.title} />
						{:else}
							<div class="thumbnail-placeholder">
								<IconTemplate size={32} />
							</div>
						{/if}
						<div class="template-overlay">
							<a href="/admin/crm/templates/{template.id}/preview" class="btn-icon" title="Preview">
								<IconEye size={18} />
							</a>
							<a href="/admin/crm/templates/{template.id}/edit" class="btn-icon" title="Edit">
								<IconEdit size={18} />
							</a>
						</div>
					</div>
					<div class="template-content">
						<h3 class="template-title">{template.title}</h3>
						{#if template.subject}
							<p class="template-subject">{template.subject}</p>
						{/if}
						<div class="template-meta">
							<span class="template-type">{template.design_template === 'visual' ? 'Visual' : 'Raw HTML'}</span>
							{#if template.category}
								<span class="template-category">{template.category}</span>
							{/if}
						</div>
						<div class="template-footer">
							<span class="template-date">Updated {formatDate(template.updated_at)}</span>
							<div class="template-actions">
								<button class="btn-icon-sm" title="Duplicate" onclick={() => duplicateTemplate(template.id)}>
									<IconCopy size={14} />
								</button>
								<button class="btn-icon-sm danger" title="Delete" onclick={() => deleteTemplate(template.id)}>
									<IconTrash size={14} />
								</button>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.templates-page {
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

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
		max-width: 500px;
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

	.templates-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.template-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
		transition: all 0.2s;
	}

	.template-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
		transform: translateY(-2px);
	}

	.template-thumbnail {
		position: relative;
		height: 160px;
		background: rgba(30, 41, 59, 0.5);
		overflow: hidden;
	}

	.template-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #475569;
	}

	.template-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.template-card:hover .template-overlay {
		opacity: 1;
	}

	.btn-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 10px;
		color: white;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.template-content {
		padding: 1.25rem;
	}

	.template-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.template-subject {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0 0 0.75rem 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.template-meta {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.template-type, .template-category {
		padding: 0.25rem 0.5rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #E6B800;
	}

	.template-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.template-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.template-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-icon-sm {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon-sm:hover {
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
	}

	.btn-icon-sm.danger:hover {
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

	.empty-actions {
		display: flex;
		gap: 1rem;
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
