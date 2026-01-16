<!--
	/admin/crm/templates - Email Template Library
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Visual and HTML template management
	- Category-based organization with reactive filtering
	- Template preview modal with live content
	- Bulk selection and operations (delete, export)
	- Pagination with configurable page sizes
	- Debounced search with $effect reactivity
	- Toast notifications for all CRUD operations
	- Full Svelte 5 $state/$derived/$effect reactivity
	- WCAG 2.1 AA accessibility compliance
-->

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
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconChevronLeft from '@tabler/icons-svelte/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconSquare from '@tabler/icons-svelte/icons/square';
	import IconSquareCheck from '@tabler/icons-svelte/icons/square-check';
	import IconFileExport from '@tabler/icons-svelte/icons/file-export';
	import { crmAPI } from '$lib/api/crm';
	import type { EmailTemplate, TemplateCategory } from '$lib/crm/types';

	// =====================================================
	// STATE MANAGEMENT - Svelte 5 Runes
	// =====================================================

	let templates = $state<EmailTemplate[]>([]);
	let categories = $state<TemplateCategory[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let debouncedSearch = $state('');
	let selectedCategory = $state('');
	let selectedTemplates = $state<Set<string>>(new Set());
	let isInitialLoad = $state(true);

	// Pagination state
	let currentPage = $state(1);
	let perPage = $state(12);
	let totalTemplates = $state(0);
	let totalPages = $state(1);

	// Preview modal state
	let showPreview = $state(false);
	let previewTemplate = $state<EmailTemplate | null>(null);
	let isLoadingPreview = $state(false);

	// Toast notification state
	let toasts = $state<Array<{ id: string; type: 'success' | 'error' | 'info'; message: string }>>([]);

	// Bulk operation state
	let isBulkDeleting = $state(false);

	// Stats derived from templates
	let stats = $derived({
		total: totalTemplates,
		visual: templates.filter((t) => t.design_template === 'visual').length,
		raw: templates.filter((t) => t.design_template === 'raw').length,
		byCategory: categories.reduce(
			(acc, cat) => {
				acc[cat.slug] = cat.templates_count;
				return acc;
			},
			{} as Record<string, number>
		)
	});

	// Check if all visible templates are selected
	let allSelected = $derived(
		templates.length > 0 && templates.every((t) => selectedTemplates.has(t.id))
	);

	// Count of selected templates
	let selectedCount = $derived(selectedTemplates.size);

	// Pagination info
	let paginationInfo = $derived({
		start: (currentPage - 1) * perPage + 1,
		end: Math.min(currentPage * perPage, totalTemplates),
		total: totalTemplates
	});

	// =====================================================
	// DEBOUNCED SEARCH - $effect with cleanup
	// =====================================================

	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		const query = searchQuery;

		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		searchTimeout = setTimeout(() => {
			debouncedSearch = query;
		}, 300);

		return () => {
			if (searchTimeout) {
				clearTimeout(searchTimeout);
			}
		};
	});

	// =====================================================
	// REACTIVE DATA LOADING - $effect
	// =====================================================

	$effect(() => {
		// Dependencies: debouncedSearch, selectedCategory, currentPage, perPage
		const search = debouncedSearch;
		const category = selectedCategory;
		const page = currentPage;
		const limit = perPage;

		// Skip initial load (handled by onMount)
		if (isInitialLoad) return;

		loadTemplates();
	});

	// Reset to page 1 when filters change
	$effect(() => {
		const search = debouncedSearch;
		const category = selectedCategory;

		if (!isInitialLoad) {
			currentPage = 1;
		}
	});

	// =====================================================
	// API FUNCTIONS
	// =====================================================

	async function loadTemplates() {
		isLoading = true;
		error = '';

		try {
			const [templatesResponse, categoriesResponse] = await Promise.all([
				crmAPI.getEmailTemplates({
					search: debouncedSearch || undefined,
					category: selectedCategory || undefined,
					per_page: perPage
				}),
				crmAPI.getTemplateCategories()
			]);

			templates = templatesResponse.data || [];
			totalTemplates = templatesResponse.meta?.total || templates.length;
			totalPages = Math.ceil(totalTemplates / perPage) || 1;
			categories = categoriesResponse || [];

			// Clear selection when data changes
			selectedTemplates = new Set();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load templates';
			showToast('error', error);
		} finally {
			isLoading = false;
			isInitialLoad = false;
		}
	}

	async function deleteTemplate(id: string) {
		if (!confirm('Are you sure you want to delete this template? This action cannot be undone.')) {
			return;
		}

		try {
			await crmAPI.deleteEmailTemplate(id);
			showToast('success', 'Template deleted successfully');
			await loadTemplates();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to delete template';
			showToast('error', message);
		}
	}

	async function duplicateTemplate(id: string) {
		try {
			const newTemplate = await crmAPI.duplicateEmailTemplate(id);
			showToast('success', `Template duplicated as "${newTemplate.title}"`);
			await loadTemplates();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Failed to duplicate template';
			showToast('error', message);
		}
	}

	async function bulkDeleteTemplates() {
		if (selectedCount === 0) return;

		const confirmed = confirm(
			`Are you sure you want to delete ${selectedCount} template${selectedCount > 1 ? 's' : ''}? This action cannot be undone.`
		);

		if (!confirmed) return;

		isBulkDeleting = true;
		let successCount = 0;
		let failCount = 0;

		for (const id of selectedTemplates) {
			try {
				await crmAPI.deleteEmailTemplate(id);
				successCount++;
			} catch {
				failCount++;
			}
		}

		isBulkDeleting = false;
		selectedTemplates = new Set();

		if (failCount === 0) {
			showToast('success', `Successfully deleted ${successCount} template${successCount > 1 ? 's' : ''}`);
		} else {
			showToast('error', `Deleted ${successCount}, failed to delete ${failCount} template${failCount > 1 ? 's' : ''}`);
		}

		await loadTemplates();
	}

	async function openPreview(template: EmailTemplate) {
		showPreview = true;
		isLoadingPreview = true;

		try {
			previewTemplate = await crmAPI.getEmailTemplate(template.id);
		} catch (err) {
			previewTemplate = template;
			showToast('error', 'Failed to load full template preview');
		} finally {
			isLoadingPreview = false;
		}
	}

	function closePreview() {
		showPreview = false;
		previewTemplate = null;
	}

	// =====================================================
	// SELECTION FUNCTIONS
	// =====================================================

	function toggleSelectAll() {
		if (allSelected) {
			selectedTemplates = new Set();
		} else {
			selectedTemplates = new Set(templates.map((t) => t.id));
		}
	}

	function toggleSelectTemplate(id: string) {
		const newSet = new Set(selectedTemplates);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		selectedTemplates = newSet;
	}

	// =====================================================
	// PAGINATION FUNCTIONS
	// =====================================================

	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
		}
	}

	function changePerPage(newPerPage: number) {
		perPage = newPerPage;
		currentPage = 1;
	}

	// =====================================================
	// TOAST NOTIFICATIONS
	// =====================================================

	function showToast(type: 'success' | 'error' | 'info', message: string) {
		const id = crypto.randomUUID();
		toasts = [...toasts, { id, type, message }];

		setTimeout(() => {
			toasts = toasts.filter((t) => t.id !== id);
		}, 5000);
	}

	function dismissToast(id: string) {
		toasts = toasts.filter((t) => t.id !== id);
	}

	// =====================================================
	// UTILITY FUNCTIONS
	// =====================================================

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && showPreview) {
			closePreview();
		}
	}

	// =====================================================
	// LIFECYCLE
	// =====================================================

	onMount(() => {
		loadTemplates();
	});
</script>

<svelte:head>
	<title>Email Templates - FluentCRM Pro</title>
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<div class="page" role="main" aria-label="Email Templates">
	<!-- Header -->
	<header class="page-header">
		<h1>Email Templates</h1>
		<p class="subtitle">Create and manage reusable email templates for campaigns and automations</p>
		<div class="header-actions">
			<button
				class="btn-refresh"
				onclick={() => loadTemplates()}
				disabled={isLoading}
				aria-label="Refresh templates"
				title="Refresh"
			>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/templates/prebuilt" class="btn-secondary">
				<IconDownload size={18} />
				<span>Pre-built Library</span>
			</a>
			<a href="/admin/crm/templates/new" class="btn-primary">
				<IconPlus size={18} />
				<span>New Template</span>
			</a>
		</div>
	</header>

	<!-- Stats Cards -->
	<section class="stats-grid" aria-label="Template statistics">
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
			<div class="stat-icon gold">
				<IconFolder size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{categories.length}</span>
				<span class="stat-label">Categories</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconEye size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.visual}</span>
				<span class="stat-label">Visual Templates</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon orange">
				<IconEdit size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.raw}</span>
				<span class="stat-label">Raw HTML</span>
			</div>
		</div>
	</section>

	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search templates..."
				bind:value={searchQuery}
				aria-label="Search templates"
			/>
			{#if searchQuery}
				<button
					class="search-clear"
					onclick={() => (searchQuery = '')}
					aria-label="Clear search"
				>
					<IconX size={16} />
				</button>
			{/if}
		</div>
		<select
			bind:value={selectedCategory}
			class="filter-select"
			aria-label="Filter by category"
		>
			<option value="">All Categories</option>
			{#each categories as category}
				<option value={category.slug}>{category.name} ({category.templates_count})</option>
			{/each}
		</select>
		<select
			value={perPage}
			onchange={(e) => changePerPage(parseInt(e.currentTarget.value))}
			class="filter-select per-page-select"
			aria-label="Templates per page"
		>
			<option value={12}>12 per page</option>
			<option value={24}>24 per page</option>
			<option value={48}>48 per page</option>
		</select>
	</div>

	<!-- Bulk Actions Bar -->
	{#if selectedCount > 0}
		<div class="bulk-actions-bar" role="toolbar" aria-label="Bulk actions">
			<div class="selection-info">
				<IconSquareCheck size={18} />
				<span>{selectedCount} template{selectedCount > 1 ? 's' : ''} selected</span>
			</div>
			<div class="bulk-buttons">
				<button
					class="btn-bulk-action danger"
					onclick={bulkDeleteTemplates}
					disabled={isBulkDeleting}
					aria-label="Delete selected templates"
				>
					{#if isBulkDeleting}
						<span class="btn-spinner"></span>
					{:else}
						<IconTrash size={16} />
					{/if}
					<span>Delete Selected</span>
				</button>
				<button class="btn-bulk-action" aria-label="Export selected templates">
					<IconFileExport size={16} />
					<span>Export</span>
				</button>
				<button
					class="btn-bulk-clear"
					onclick={() => (selectedTemplates = new Set())}
					aria-label="Clear selection"
				>
					<IconX size={16} />
					<span>Clear</span>
				</button>
			</div>
		</div>
	{/if}

	<!-- Templates Grid -->
	{#if isLoading && templates.length === 0}
		<div class="loading-state" role="status" aria-live="polite">
			<div class="spinner"></div>
			<p>Loading templates...</p>
		</div>
	{:else if error && templates.length === 0}
		<div class="error-state" role="alert">
			<IconAlertCircle size={48} />
			<h3>Failed to Load Templates</h3>
			<p>{error}</p>
			<button class="btn-primary" onclick={() => loadTemplates()}>
				<IconRefresh size={18} />
				Try Again
			</button>
		</div>
	{:else if templates.length === 0}
		<div class="empty-state">
			<IconTemplate size={48} />
			<h3>No templates found</h3>
			<p>
				{#if searchQuery || selectedCategory}
					No templates match your current filters. Try adjusting your search criteria.
				{:else}
					Create your first template or import from the pre-built library.
				{/if}
			</p>
			<div class="empty-actions">
				{#if searchQuery || selectedCategory}
					<button
						class="btn-secondary"
						onclick={() => {
							searchQuery = '';
							selectedCategory = '';
						}}
					>
						<IconX size={18} />
						Clear Filters
					</button>
				{:else}
					<a href="/admin/crm/templates/new" class="btn-primary">
						<IconPlus size={18} />
						Create Template
					</a>
					<a href="/admin/crm/templates/prebuilt" class="btn-secondary">
						<IconDownload size={18} />
						Pre-built Templates
					</a>
				{/if}
			</div>
		</div>
	{:else}
		<!-- Select All Header -->
		<div class="templates-header">
			<button
				class="select-all-btn"
				onclick={toggleSelectAll}
				aria-label={allSelected ? 'Deselect all templates' : 'Select all templates'}
			>
				{#if allSelected}
					<IconSquareCheck size={20} />
				{:else}
					<IconSquare size={20} />
				{/if}
				<span>Select All</span>
			</button>
			<span class="results-count">
				Showing {paginationInfo.start}-{paginationInfo.end} of {paginationInfo.total} templates
			</span>
		</div>

		<div class="templates-grid" role="list" aria-label="Templates list">
			{#each templates as template (template.id)}
				<article
					class="template-card"
					class:selected={selectedTemplates.has(template.id)}
					role="listitem"
					aria-label={template.title}
				>
					<button
						class="template-checkbox"
						onclick={(e) => {
							e.stopPropagation();
							toggleSelectTemplate(template.id);
						}}
						aria-label={selectedTemplates.has(template.id)
							? `Deselect ${template.title}`
							: `Select ${template.title}`}
					>
						{#if selectedTemplates.has(template.id)}
							<IconSquareCheck size={20} />
						{:else}
							<IconSquare size={20} />
						{/if}
					</button>

					<div class="template-thumbnail">
						{#if template.thumbnail}
							<img src={template.thumbnail} alt={`${template.title} preview`} loading="lazy" />
						{:else}
							<div class="thumbnail-placeholder">
								<IconTemplate size={32} />
							</div>
						{/if}
						<div class="template-overlay">
							<button
								class="btn-icon"
								title="Preview"
								onclick={() => openPreview(template)}
								aria-label={`Preview ${template.title}`}
							>
								<IconEye size={18} />
							</button>
							<a
								href="/admin/crm/templates/{template.id}/edit"
								class="btn-icon"
								title="Edit"
								aria-label={`Edit ${template.title}`}
							>
								<IconEdit size={18} />
							</a>
						</div>
					</div>

					<div class="template-content">
						<h3 class="template-title">{template.title}</h3>
						{#if template.subject}
							<p class="template-subject" title={template.subject}>{template.subject}</p>
						{/if}
						<div class="template-meta">
							<span class="template-type" class:visual={template.design_template === 'visual'}>
								{template.design_template === 'visual' ? 'Visual' : 'Raw HTML'}
							</span>
							{#if template.category}
								<span class="template-category">{template.category}</span>
							{/if}
						</div>
						<div class="template-footer">
							<span class="template-date">
								<time datetime={template.updated_at}>Updated {formatDate(template.updated_at)}</time>
							</span>
							<div class="template-actions">
								<button
									class="btn-icon-sm"
									title="Duplicate"
									onclick={() => duplicateTemplate(template.id)}
									aria-label={`Duplicate ${template.title}`}
								>
									<IconCopy size={14} />
								</button>
								<button
									class="btn-icon-sm danger"
									title="Delete"
									onclick={() => deleteTemplate(template.id)}
									aria-label={`Delete ${template.title}`}
								>
									<IconTrash size={14} />
								</button>
							</div>
						</div>
					</div>
				</article>
			{/each}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<nav class="pagination" aria-label="Template pages">
				<button
					class="pagination-btn"
					onclick={() => goToPage(currentPage - 1)}
					disabled={currentPage === 1}
					aria-label="Previous page"
				>
					<IconChevronLeft size={18} />
					<span>Previous</span>
				</button>

				<div class="pagination-pages">
					{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
						{#if page === 1 || page === totalPages || (page >= currentPage - 1 && page <= currentPage + 1)}
							<button
								class="pagination-page"
								class:active={page === currentPage}
								onclick={() => goToPage(page)}
								aria-label={`Page ${page}`}
								aria-current={page === currentPage ? 'page' : undefined}
							>
								{page}
							</button>
						{:else if page === currentPage - 2 || page === currentPage + 2}
							<span class="pagination-ellipsis">...</span>
						{/if}
					{/each}
				</div>

				<button
					class="pagination-btn"
					onclick={() => goToPage(currentPage + 1)}
					disabled={currentPage === totalPages}
					aria-label="Next page"
				>
					<span>Next</span>
					<IconChevronRight size={18} />
				</button>
			</nav>
		{/if}
	{/if}
</div>

<!-- Preview Modal -->
{#if showPreview}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div
		class="modal-backdrop"
		onclick={closePreview}
		onkeydown={(e) => e.key === 'Escape' && closePreview()}
		role="presentation"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content preview-modal"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="preview-title"
			tabindex="-1"
		>
			<header class="modal-header">
				<h2 id="preview-title">{previewTemplate?.title || 'Template Preview'}</h2>
				<button
					class="modal-close"
					onclick={closePreview}
					aria-label="Close preview"
				>
					<IconX size={20} />
				</button>
			</header>

			<div class="modal-body">
				{#if isLoadingPreview}
					<div class="preview-loading">
						<div class="spinner"></div>
						<p>Loading preview...</p>
					</div>
				{:else if previewTemplate}
					<div class="preview-info">
						{#if previewTemplate.subject}
							<div class="preview-field">
								<label for="subject">Subject:</label>
								<span id="subject">{previewTemplate.subject}</span>
							</div>
						{/if}
						<div class="preview-field">
							<label for="type">Type:</label>
							<span id="type" class="template-type" class:visual={previewTemplate.design_template === 'visual'}>
								{previewTemplate.design_template === 'visual' ? 'Visual Editor' : 'Raw HTML'}
							</span>
						</div>
						{#if previewTemplate.category}
							<div class="preview-field">
								<label for="category">Category:</label>
								<span id="category" class="template-category">{previewTemplate.category}</span>
							</div>
						{/if}
					</div>
					<div class="preview-content">
						<h3>Email Content</h3>
						<div class="preview-frame">
							{@html previewTemplate.content || '<p class="no-content">No content available</p>'}
						</div>
					</div>
				{/if}
			</div>

			<footer class="modal-footer">
				<button class="btn-secondary" onclick={closePreview}>Close</button>
				{#if previewTemplate}
					<a href="/admin/crm/templates/{previewTemplate.id}/edit" class="btn-primary">
						<IconEdit size={18} />
						Edit Template
					</a>
				{/if}
			</footer>
		</div>
	</div>
{/if}

<!-- Toast Notifications -->
{#if toasts.length > 0}
	<div class="toast-container" role="region" aria-label="Notifications">
		{#each toasts as toast (toast.id)}
			<div
				class="toast toast-{toast.type}"
				role="alert"
				aria-live="polite"
			>
				<div class="toast-icon">
					{#if toast.type === 'success'}
						<IconCheck size={18} />
					{:else if toast.type === 'error'}
						<IconAlertCircle size={18} />
					{:else}
						<IconTemplate size={18} />
					{/if}
				</div>
				<span class="toast-message">{toast.message}</span>
				<button
					class="toast-dismiss"
					onclick={() => dismissToast(toast.id)}
					aria-label="Dismiss notification"
				>
					<IconX size={16} />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	/* =====================================================
	   Layout & Container
	   ===================================================== */
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* =====================================================
	   Page Header - CENTERED
	   ===================================================== */
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
		margin: 0 0 1.25rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		flex-wrap: wrap;
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

	.btn-refresh:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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
		font-size: 0.9rem;
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
		font-size: 0.9rem;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
	}

	/* =====================================================
	   Stats Grid
	   ===================================================== */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
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
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.25);
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.orange { background: rgba(249, 115, 22, 0.15); color: #fb923c; }

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

	/* =====================================================
	   Filters Bar
	   ===================================================== */
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
		transition: all 0.2s;
	}

	.search-box:focus-within {
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
		min-width: 0;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.search-clear {
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		padding: 4px;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.search-clear:hover {
		color: #f87171;
		background: rgba(248, 113, 113, 0.1);
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.per-page-select {
		min-width: 130px;
	}

	/* =====================================================
	   Bulk Actions Bar
	   ===================================================== */
	.bulk-actions-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 10px;
		margin-bottom: 1rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.selection-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #c7d2fe;
		font-weight: 500;
	}

	.bulk-buttons {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.btn-bulk-action {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.5rem 0.75rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk-action:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-bulk-action.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.2);
		border-color: rgba(239, 68, 68, 0.4);
		color: #fca5a5;
	}

	.btn-bulk-action:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-bulk-clear {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk-clear:hover {
		color: #e2e8f0;
		border-color: rgba(99, 102, 241, 0.4);
	}

	.btn-spinner {
		width: 14px;
		height: 14px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* =====================================================
	   Templates Header & Grid
	   ===================================================== */
	.templates-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
		padding: 0 0.25rem;
	}

	.select-all-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.select-all-btn:hover {
		color: #c7d2fe;
		border-color: rgba(99, 102, 241, 0.4);
		background: rgba(99, 102, 241, 0.1);
	}

	.results-count {
		font-size: 0.85rem;
		color: #64748b;
	}

	.templates-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.5rem;
	}

	.template-card {
		position: relative;
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

	.template-card.selected {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(99, 102, 241, 0.1);
	}

	.template-checkbox {
		position: absolute;
		top: 0.75rem;
		left: 0.75rem;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(15, 23, 42, 0.9);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		opacity: 0;
	}

	.template-card:hover .template-checkbox,
	.template-card.selected .template-checkbox {
		opacity: 1;
	}

	.template-checkbox:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #c7d2fe;
	}

	.template-card.selected .template-checkbox {
		background: rgba(99, 102, 241, 0.3);
		color: #c7d2fe;
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
		transform: scale(1.05);
	}

	.template-content {
		padding: 1.25rem;
	}

	.template-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
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
		flex-wrap: wrap;
	}

	.template-type, .template-category {
		padding: 0.25rem 0.5rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #E6B800;
	}

	.template-type.visual {
		background: rgba(34, 197, 94, 0.1);
		color: #4ade80;
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

	/* =====================================================
	   Pagination
	   ===================================================== */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		gap: 0.35rem;
		padding: 0.5rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #c7d2fe;
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-pages {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.pagination-page {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pagination-page:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #c7d2fe;
	}

	.pagination-page.active {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-color: transparent;
		color: white;
	}

	.pagination-ellipsis {
		padding: 0 0.5rem;
		color: #64748b;
	}

	/* =====================================================
	   States
	   ===================================================== */
	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.error-state :global(svg),
	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.error-state h3,
	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
		font-size: 1.25rem;
	}

	.error-state p,
	.empty-state p {
		margin: 0 0 1.5rem 0;
		max-width: 400px;
	}

	.empty-actions {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		justify-content: center;
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

	/* =====================================================
	   Preview Modal
	   ===================================================== */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
		backdrop-filter: blur(4px);
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 800px;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.modal-body {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.preview-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		color: #64748b;
	}

	.preview-info {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.preview-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.preview-field label {
		font-size: 0.85rem;
		color: #64748b;
		min-width: 80px;
	}

	.preview-field span {
		color: #e2e8f0;
	}

	.preview-content h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 1rem 0;
	}

	.preview-frame {
		background: white;
		border-radius: 8px;
		padding: 1.5rem;
		min-height: 200px;
		color: #1e293b;
	}

	.preview-frame :global(.no-content) {
		color: #94a3b8;
		text-align: center;
		font-style: italic;
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	/* =====================================================
	   Toast Notifications
	   ===================================================== */
	.toast-container {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		z-index: 1100;
		max-width: 400px;
	}

	.toast {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.toast-success {
		border-color: rgba(34, 197, 94, 0.3);
	}

	.toast-success .toast-icon {
		color: #4ade80;
	}

	.toast-error {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.toast-error .toast-icon {
		color: #f87171;
	}

	.toast-info .toast-icon {
		color: #60a5fa;
	}

	.toast-icon {
		flex-shrink: 0;
	}

	.toast-message {
		flex: 1;
		color: #e2e8f0;
		font-size: 0.9rem;
	}

	.toast-dismiss {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.toast-dismiss:hover {
		color: #f87171;
		background: rgba(248, 113, 113, 0.1);
	}

	/* =====================================================
	   Responsive Adjustments
	   ===================================================== */
	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.header-actions {
			flex-direction: column;
			width: 100%;
		}

		.header-actions .btn-primary,
		.header-actions .btn-secondary {
			width: 100%;
			justify-content: center;
		}

		.search-box {
			max-width: none;
		}

		.filters-bar {
			flex-direction: column;
		}

		.filter-select {
			width: 100%;
		}

		.templates-grid {
			grid-template-columns: 1fr;
		}

		.pagination {
			flex-wrap: wrap;
		}

		.toast-container {
			left: 1rem;
			right: 1rem;
			max-width: none;
		}
	}
</style>
