<!--
	CMS v2 Content List
	═══════════════════════════════════════════════════════════════════════════════

	Content management page with:
	- Filterable content list
	- Status workflow indicators
	- Quick actions
	- Search and sorting

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { fly, scale } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import {
		cmsApi,
		type CmsContentSummary,
		type ContentListParams
	} from '$lib/api/cms-v2';
	import type { CmsContentType, CmsContentStatus } from '$lib/page-builder/types';
	import {
		IconPlus,
		IconSearch,
		IconFilter,
		IconSortAscending,
		IconSortDescending,
		IconEdit,
		IconTrash,
		IconEye,
		IconCopy,
		IconDotsVertical,
		IconFileText,
		IconArticle,
		IconCategory,
		IconRefresh,
		IconX,
		IconCheck,
		IconClock,
		IconArchive
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	let content = $state<CmsContentSummary[]>([]);
	let isLoading = $state(true);
	let selectedIds = $state(new Set<string>());

	// Pagination
	let currentPage = $state(1);
	let totalPages = $state(1);
	let totalItems = $state(0);
	let perPage = $state(20);

	// Filters
	let searchQuery = $state('');
	let filterType = $state<CmsContentType | ''>('');
	let filterStatus = $state<CmsContentStatus | ''>('');
	let sortBy = $state<'created_at' | 'updated_at' | 'title'>('updated_at');
	let sortOrder = $state<'ASC' | 'DESC'>('DESC');

	// UI State
	let showFilters = $state(false);
	let showContextMenu = $state<{ x: number; y: number; contentId: string } | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived
	// ═══════════════════════════════════════════════════════════════════════════

	let hasSelection = $derived(selectedIds.size > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;
		loadContent();

		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	});

	// Reload on filter changes
	$effect(() => {
		if (!browser) return;
		const _deps = [searchQuery, filterType, filterStatus, sortBy, sortOrder, currentPage];
		loadContent();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadContent() {
		isLoading = true;
		try {
			const params: ContentListParams = {
				search: searchQuery || undefined,
				contentType: filterType || undefined,
				status: filterStatus || undefined,
				sortBy,
				sortOrder,
				limit: perPage,
				offset: (currentPage - 1) * perPage
			};

			const response = await cmsApi.listContent(params);
			content = response.data;
			totalItems = response.pagination.total;
			totalPages = response.pagination.total_pages;
		} catch (e) {
			console.error('Failed to load content:', e);
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════

	function selectContent(id: string, event?: MouseEvent) {
		if (event?.ctrlKey || event?.metaKey) {
			if (selectedIds.has(id)) {
				selectedIds.delete(id);
			} else {
				selectedIds.add(id);
			}
			selectedIds = selectedIds;
		} else {
			selectedIds.clear();
			selectedIds.add(id);
			selectedIds = selectedIds;
		}
	}

	function selectAll() {
		if (selectedIds.size === content.length) {
			selectedIds.clear();
		} else {
			content.forEach((c) => selectedIds.add(c.id));
		}
		selectedIds = selectedIds;
	}

	async function deleteContent(id: string) {
		if (!confirm('Delete this content? This cannot be undone.')) return;

		try {
			await cmsApi.deleteContent(id);
			selectedIds.delete(id);
			selectedIds = selectedIds;
			await loadContent();
		} catch (e) {
			console.error('Failed to delete content:', e);
		}
	}

	async function deleteSelected() {
		if (!confirm(`Delete ${selectedIds.size} items? This cannot be undone.`)) return;

		try {
			await Promise.all([...selectedIds].map((id) => cmsApi.deleteContent(id)));
			selectedIds.clear();
			selectedIds = selectedIds;
			await loadContent();
		} catch (e) {
			console.error('Failed to delete content:', e);
		}
	}

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.context-menu')) {
			showContextMenu = null;
		}
	}

	function handleContextMenu(e: MouseEvent, id: string) {
		e.preventDefault();
		showContextMenu = { x: e.clientX, y: e.clientY, contentId: id };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	function getTypeIcon(type: CmsContentType) {
		switch (type) {
			case 'page':
				return IconFileText;
			case 'blog_post':
				return IconArticle;
			default:
				return IconFileText;
		}
	}

	function getStatusColor(status: CmsContentStatus): string {
		switch (status) {
			case 'published':
				return 'green';
			case 'draft':
				return 'gray';
			case 'in_review':
				return 'yellow';
			case 'scheduled':
				return 'blue';
			case 'archived':
				return 'red';
			default:
				return 'gray';
		}
	}

	function getStatusIcon(status: CmsContentStatus) {
		switch (status) {
			case 'published':
				return IconCheck;
			case 'draft':
				return IconEdit;
			case 'in_review':
				return IconClock;
			case 'scheduled':
				return IconClock;
			case 'archived':
				return IconArchive;
			default:
				return IconFileText;
		}
	}

	function formatDate(dateString: string | null): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	const contentTypes: { value: CmsContentType; label: string }[] = [
		{ value: 'page', label: 'Page' },
		{ value: 'blog_post', label: 'Blog Post' },
		{ value: 'alert_service', label: 'Alert Service' },
		{ value: 'trading_room', label: 'Trading Room' },
		{ value: 'indicator', label: 'Indicator' },
		{ value: 'course', label: 'Course' },
		{ value: 'lesson', label: 'Lesson' },
		{ value: 'testimonial', label: 'Testimonial' },
		{ value: 'faq', label: 'FAQ' }
	];

	const statusOptions: { value: CmsContentStatus; label: string }[] = [
		{ value: 'draft', label: 'Draft' },
		{ value: 'in_review', label: 'In Review' },
		{ value: 'approved', label: 'Approved' },
		{ value: 'scheduled', label: 'Scheduled' },
		{ value: 'published', label: 'Published' },
		{ value: 'archived', label: 'Archived' }
	];
</script>

<svelte:head>
	<title>Content | CMS v2</title>
</svelte:head>

<div class="content-page">
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 300 }}>
		<div class="header-content">
			<h1 class="page-title">Content</h1>
			<p class="page-subtitle">{totalItems} items</p>
		</div>
		<div class="header-actions">
			<a href="/admin/cms-v2/content/create" class="btn-create">
				<IconPlus size={18} />
				New Content
			</a>
		</div>
	</header>

	<!-- Toolbar -->
	<div class="toolbar" in:fly={{ y: -10, duration: 300, delay: 50 }}>
		<!-- Search -->
		<div class="search-box">
			<IconSearch size={16} />
			<input
				type="text"
				placeholder="Search content..."
				bind:value={searchQuery}
				class="search-input"
			/>
			{#if searchQuery}
				<button class="btn-clear" onclick={() => (searchQuery = '')}>
					<IconX size={14} />
				</button>
			{/if}
		</div>

		<!-- Filters Toggle -->
		<button
			class="btn-filter"
			class:active={showFilters}
			onclick={() => (showFilters = !showFilters)}
		>
			<IconFilter size={16} />
			Filters
			{#if filterType || filterStatus}
				<span class="filter-badge">
					{(filterType ? 1 : 0) + (filterStatus ? 1 : 0)}
				</span>
			{/if}
		</button>

		<!-- Sort -->
		<div class="sort-group">
			<select bind:value={sortBy} class="sort-select">
				<option value="updated_at">Last Updated</option>
				<option value="created_at">Created</option>
				<option value="title">Title</option>
			</select>
			<button
				class="btn-sort-order"
				onclick={() => (sortOrder = sortOrder === 'ASC' ? 'DESC' : 'ASC')}
				title={sortOrder === 'ASC' ? 'Ascending' : 'Descending'}
			>
				{#if sortOrder === 'ASC'}
					<IconSortAscending size={16} />
				{:else}
					<IconSortDescending size={16} />
				{/if}
			</button>
		</div>

		<!-- Bulk Actions -->
		{#if hasSelection}
			<div class="bulk-actions" in:scale={{ duration: 200 }}>
				<span class="selection-count">{selectedIds.size} selected</span>
				<button class="btn-action danger" onclick={deleteSelected} title="Delete">
					<IconTrash size={16} />
				</button>
			</div>
		{/if}

		<!-- Refresh -->
		<button class="btn-icon" onclick={loadContent} title="Refresh">
			<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
		</button>
	</div>

	<!-- Filters Panel -->
	{#if showFilters}
		<div class="filters-panel" in:fly={{ y: -10, duration: 200 }}>
			<div class="filter-group">
				<label for="type-filter">Type</label>
				<select id="type-filter" bind:value={filterType}>
					<option value="">All Types</option>
					{#each contentTypes as type}
						<option value={type.value}>{type.label}</option>
					{/each}
				</select>
			</div>

			<div class="filter-group">
				<label for="status-filter">Status</label>
				<select id="status-filter" bind:value={filterStatus}>
					<option value="">All Statuses</option>
					{#each statusOptions as status}
						<option value={status.value}>{status.label}</option>
					{/each}
				</select>
			</div>

			<button
				class="btn-clear-filters"
				onclick={() => {
					filterType = '';
					filterStatus = '';
				}}
			>
				Clear Filters
			</button>
		</div>
	{/if}

	<!-- Content List -->
	<div class="content-list">
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading content...</p>
			</div>
		{:else if content.length === 0}
			<div class="empty-state" in:scale={{ duration: 400 }}>
				<IconFileText size={64} />
				<h3>No content found</h3>
				<p>
					{#if searchQuery || filterType || filterStatus}
						No content matches your filters. Try adjusting your search.
					{:else}
						Get started by creating your first content item.
					{/if}
				</p>
				<a href="/admin/cms-v2/content/create" class="btn-primary">
					<IconPlus size={18} />
					Create Content
				</a>
			</div>
		{:else}
			<!-- Table Header -->
			<div class="table-header">
				<div class="cell-checkbox">
					<input
						type="checkbox"
						checked={selectedIds.size === content.length && content.length > 0}
						onchange={selectAll}
					/>
				</div>
				<div class="cell-title">Title</div>
				<div class="cell-type">Type</div>
				<div class="cell-status">Status</div>
				<div class="cell-updated">Updated</div>
				<div class="cell-actions"></div>
			</div>

			<!-- Table Body -->
			{#each content as item, i (item.id)}
				{@const TypeIcon = getTypeIcon(item.content_type)}
				{@const StatusIcon = getStatusIcon(item.status)}
				{@const isSelected = selectedIds.has(item.id)}

				<div
					class="table-row"
					class:selected={isSelected}
					role="row"
					tabindex="0"
					onclick={(e: MouseEvent) => selectContent(item.id, e)}
					onkeydown={(e: KeyboardEvent) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							selectContent(item.id);
						}
					}}
					oncontextmenu={(e: MouseEvent) => handleContextMenu(e, item.id)}
					in:fly={{ x: -10, duration: 200, delay: i * 20 }}
				>
					<div class="cell-checkbox">
						<input
							type="checkbox"
							checked={isSelected}
							onclick={(e) => e.stopPropagation()}
							onchange={() => selectContent(item.id)}
						/>
					</div>

					<div class="cell-title">
						<a href="/admin/cms-v2/content/{item.id}" class="content-link">
							<TypeIcon size={18} class="type-icon" />
							<div class="title-info">
								<span class="title-text">{item.title}</span>
								<span class="slug-text">/{item.slug}</span>
							</div>
						</a>
					</div>

					<div class="cell-type">
						<span class="type-badge">{item.content_type.replace('_', ' ')}</span>
					</div>

					<div class="cell-status">
						<span class="status-badge status-{getStatusColor(item.status)}">
							<StatusIcon size={12} />
							{item.status.replace('_', ' ')}
						</span>
					</div>

					<div class="cell-updated">
						{formatDate(item.updated_at)}
					</div>

					<div class="cell-actions">
						<a
							href="/admin/cms-v2/content/{item.id}"
							class="btn-action"
							title="Edit"
							onclick={(e) => e.stopPropagation()}
						>
							<IconEdit size={16} />
						</a>
						<button
							class="btn-action"
							title="More"
							onclick={(e) => {
								e.stopPropagation();
								handleContextMenu(e, item.id);
							}}
						>
							<IconDotsVertical size={16} />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</div>

	<!-- Pagination -->
	{#if totalPages > 1}
		<div class="pagination">
			<button
				class="page-btn"
				disabled={currentPage === 1}
				onclick={() => (currentPage = Math.max(1, currentPage - 1))}
			>
				Previous
			</button>
			<span class="page-info">
				Page {currentPage} of {totalPages}
			</span>
			<button
				class="page-btn"
				disabled={currentPage === totalPages}
				onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
			>
				Next
			</button>
		</div>
	{/if}
</div>

<!-- Context Menu -->
{#if showContextMenu}
	<div
		class="context-menu"
		style="left: {showContextMenu.x}px; top: {showContextMenu.y}px"
		in:scale={{ duration: 150, start: 0.95 }}
	>
		<a
			href="/admin/cms-v2/content/{showContextMenu.contentId}"
			class="context-item"
			onclick={() => (showContextMenu = null)}
		>
			<IconEdit size={16} />
			Edit
		</a>
		<button
			class="context-item"
			onclick={() => {
				window.open(`/preview/${showContextMenu?.contentId}`, '_blank');
				showContextMenu = null;
			}}
		>
			<IconEye size={16} />
			Preview
		</button>
		<button
			class="context-item"
			onclick={() => {
				const item = content.find((c) => c.id === showContextMenu?.contentId);
				if (item) goto(`/admin/cms-v2/content/create?duplicate=${item.id}`);
				showContextMenu = null;
			}}
		>
			<IconCopy size={16} />
			Duplicate
		</button>
		<hr class="context-divider" />
		<button
			class="context-item danger"
			onclick={() => {
				deleteContent(showContextMenu?.contentId || '');
				showContextMenu = null;
			}}
		>
			<IconTrash size={16} />
			Delete
		</button>
	</div>
{/if}

<style>
	.content-page {
		padding: 1.5rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-subtitle {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0;
	}

	.btn-create {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-create:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		margin-bottom: 1rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		flex: 1;
		max-width: 320px;
	}

	.search-input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		outline: none;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.btn-clear {
		padding: 0.125rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.15s;
	}

	.btn-clear:hover {
		color: #ef4444;
	}

	.btn-filter {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-filter:hover,
	.btn-filter.active {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.filter-badge {
		padding: 0.125rem 0.375rem;
		background: var(--primary-500);
		color: #0f172a;
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		font-weight: 700;
	}

	.sort-group {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.sort-select {
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem 0 0 0.5rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.btn-sort-order {
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-left: none;
		border-radius: 0 0.5rem 0.5rem 0;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-sort-order:hover {
		color: #f1f5f9;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 0.5rem;
	}

	.selection-count {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--primary-500);
	}

	.btn-action {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.375rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.15s;
		text-decoration: none;
	}

	.btn-action:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-action.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
		margin-left: auto;
	}

	.btn-icon:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-icon :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Filters Panel */
	.filters-panel {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		margin-bottom: 1rem;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.filter-group label {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.filter-group select {
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.btn-clear-filters {
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
		margin-left: auto;
	}

	.btn-clear-filters:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	/* Content List */
	.content-list {
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.table-header {
		display: grid;
		grid-template-columns: 40px 1fr 120px 120px 100px 80px;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		color: #64748b;
	}

	.table-row {
		display: grid;
		grid-template-columns: 40px 1fr 120px 120px 100px 80px;
		gap: 1rem;
		padding: 0.75rem 1rem;
		align-items: center;
		border-bottom: 1px solid rgba(51, 65, 85, 0.3);
		transition: background 0.15s;
		cursor: pointer;
	}

	.table-row:hover {
		background: rgba(51, 65, 85, 0.3);
	}

	.table-row.selected {
		background: rgba(230, 184, 0, 0.1);
	}

	.table-row:last-child {
		border-bottom: none;
	}

	.cell-checkbox {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.cell-checkbox input[type='checkbox'] {
		width: 16px;
		height: 16px;
		cursor: pointer;
		accent-color: var(--primary-500);
	}

	.content-link {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
		color: inherit;
	}

	.content-link :global(.type-icon) {
		color: var(--primary-500);
		flex-shrink: 0;
	}

	.title-info {
		min-width: 0;
	}

	.title-text {
		display: block;
		font-weight: 500;
		color: #f1f5f9;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.slug-text {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.type-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: rgba(51, 65, 85, 0.5);
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: #94a3b8;
		text-transform: capitalize;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.status-green {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.status-gray {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.status-yellow {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.status-blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.status-red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.cell-updated {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.cell-actions {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		justify-content: flex-end;
	}

	/* Loading / Empty */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem;
		font-size: 1.25rem;
		color: #f1f5f9;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
		max-width: 320px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.page-btn {
		padding: 0.5rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	/* Context Menu */
	.context-menu {
		position: fixed;
		z-index: 1000;
		min-width: 160px;
		padding: 0.5rem;
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
	}

	.context-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
		text-decoration: none;
	}

	.context-item:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.context-item.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.context-divider {
		margin: 0.375rem 0;
		border: none;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.table-header,
		.table-row {
			grid-template-columns: 40px 1fr 100px 80px;
		}

		.cell-type,
		.cell-updated {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.content-page {
			padding: 1rem;
		}

		.toolbar {
			flex-wrap: wrap;
		}

		.search-box {
			flex: 1 1 100%;
			max-width: none;
		}

		.filters-panel {
			flex-wrap: wrap;
		}
	}
</style>
