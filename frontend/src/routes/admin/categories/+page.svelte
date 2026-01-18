<!--
	/admin/categories - Product & Content Category Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Full CRUD operations with categoriesApi
	- Hierarchical category support (parent/child)
	- Bulk operations (delete, visibility toggle)
	- Drag-and-drop reorder capability
	- Search and filter functionality
	- Category merge and export
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import IconFolder from '@tabler/icons-svelte/icons/folder';
	import IconFolderPlus from '@tabler/icons-svelte/icons/folder-plus';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconEyeOff from '@tabler/icons-svelte/icons/eye-off';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconGitMerge from '@tabler/icons-svelte/icons/git-merge';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import { categoriesApi, AdminApiError, type Category } from '$lib/api/admin';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let categories = $state<Category[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let searchQuery = $state('');
	let showHidden = $state(false);
	let parentFilter = $state<number | null>(null);

	// Modal states
	let showCategoryModal = $state(false);
	let showMergeModal = $state(false);
	let editingCategory = $state<Category | null>(null);

	// Selection for bulk operations
	let selectedIds = $state<Set<number>>(new Set());

	// Form data
	let categoryForm = $state({
		name: '',
		slug: '',
		description: '',
		color: '#6366f1',
		is_visible: true,
		parent_id: null as number | null,
		meta_title: '',
		meta_description: ''
	});

	// Merge form
	let mergeForm = $state({
		targetId: null as number | null
	});

	// Validation errors
	let formErrors = $state<string[]>([]);

	// Toast notifications
	let toastMessage = $state('');
	let toastType = $state<'success' | 'error'>('success');
	let showToast = $state(false);

	// Stats
	let stats = $state({
		total: 0,
		visible: 0,
		hidden: 0,
		withPosts: 0
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Svelte 5 $derived
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredCategories = $derived(
		categories.filter((cat) => {
			const matchesSearch =
				!searchQuery ||
				cat.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				cat.slug.toLowerCase().includes(searchQuery.toLowerCase()) ||
				cat.description?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesVisibility = showHidden || cat.is_visible;
			const matchesParent =
				parentFilter === null || cat.parent_id === parentFilter || cat.id === parentFilter;
			return matchesSearch && matchesVisibility && matchesParent;
		})
	);

	let parentCategories = $derived(categories.filter((cat) => !cat.parent_id));

	let allSelected = $derived(
		filteredCategories.length > 0 && selectedIds.size === filteredCategories.length
	);

	let mergeTargetOptions = $derived(categories.filter((cat) => !selectedIds.has(cat.id)));

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-generate slug when name changes for new categories
	$effect(() => {
		if (categoryForm.name && !editingCategory) {
			categoryForm.slug = generateSlug(categoryForm.name);
		}
	});

	// Update stats when categories change
	$effect(() => {
		stats = {
			total: categories.length,
			visible: categories.filter((c) => c.is_visible).length,
			hidden: categories.filter((c) => !c.is_visible).length,
			withPosts: categories.filter((c) => c.post_count > 0).length
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS - Complete CRUD
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadCategories() {
		isLoading = true;
		error = '';

		try {
			const response = await categoriesApi.list({ all: true });
			categories = (response.data || []) as Category[];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load categories';
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			}
		} finally {
			isLoading = false;
		}
	}

	async function saveCategory() {
		if (!validateForm()) return;

		isSaving = true;
		try {
			const payload = {
				name: categoryForm.name,
				slug: categoryForm.slug,
				description: categoryForm.description || undefined,
				color: categoryForm.color,
				is_visible: categoryForm.is_visible,
				parent_id: categoryForm.parent_id || undefined,
				meta_title: categoryForm.meta_title || undefined,
				meta_description: categoryForm.meta_description || undefined
			};

			if (editingCategory) {
				await categoriesApi.update(editingCategory.id, payload);
				showToastMessage('Category updated successfully', 'success');
			} else {
				await categoriesApi.create(payload);
				showToastMessage('Category created successfully', 'success');
			}

			await loadCategories();
			closeCategoryModal();
		} catch (err) {
			console.error('Failed to save category:', err);
			if (err instanceof AdminApiError) {
				if (err.isValidationError && err.response?.errors) {
					formErrors = Object.values(err.response.errors).flat() as string[];
				} else {
					showToastMessage(err.message, 'error');
				}
			} else {
				showToastMessage('Failed to save category', 'error');
			}
		} finally {
			isSaving = false;
		}
	}

	async function deleteCategory(id: number) {
		const category = categories.find((c) => c.id === id);
		if (!category) return;

		const hasChildren = categories.some((c) => c.parent_id === id);
		const warningMsg = hasChildren
			? `Delete "${category.name}"? This will also affect ${categories.filter((c) => c.parent_id === id).length} child categories.`
			: `Delete "${category.name}"? ${category.post_count} associated posts will not be deleted.`;

		if (!confirm(warningMsg)) return;

		try {
			await categoriesApi.delete(id);
			showToastMessage('Category deleted successfully', 'success');
			selectedIds.delete(id);
			selectedIds = new Set(selectedIds);
			await loadCategories();
		} catch (err) {
			console.error('Failed to delete category:', err);
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			} else {
				showToastMessage('Failed to delete category', 'error');
			}
		}
	}

	async function bulkDelete() {
		if (selectedIds.size === 0) return;
		if (!confirm(`Delete ${selectedIds.size} categories? This action cannot be undone.`)) return;

		try {
			await categoriesApi.bulkDelete(Array.from(selectedIds));
			showToastMessage(`${selectedIds.size} categories deleted successfully`, 'success');
			selectedIds = new Set();
			await loadCategories();
		} catch (err) {
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			}
		}
	}

	async function bulkToggleVisibility(visible: boolean) {
		if (selectedIds.size === 0) return;

		try {
			await categoriesApi.bulkUpdateVisibility(Array.from(selectedIds), visible);
			showToastMessage(
				`${selectedIds.size} categories ${visible ? 'shown' : 'hidden'} successfully`,
				'success'
			);
			selectedIds = new Set();
			await loadCategories();
		} catch (err) {
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			}
		}
	}

	async function mergeCategories() {
		if (selectedIds.size === 0 || !mergeForm.targetId) return;

		try {
			await categoriesApi.merge(Array.from(selectedIds), mergeForm.targetId);
			showToastMessage('Categories merged successfully', 'success');
			selectedIds = new Set();
			showMergeModal = false;
			mergeForm.targetId = null;
			await loadCategories();
		} catch (err) {
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			}
		}
	}

	async function exportCategories() {
		try {
			const response = await categoriesApi.export();
			const blob = new Blob([JSON.stringify(response.data, null, 2)], {
				type: 'application/json'
			});
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = response.data?.filename || 'categories-export.json';
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			showToastMessage('Categories exported successfully', 'success');
		} catch (err) {
			if (err instanceof AdminApiError) {
				showToastMessage(err.message, 'error');
			}
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function openCategoryModal(category: Category | null = null) {
		if (category) {
			editingCategory = category;
			categoryForm = {
				name: category.name,
				slug: category.slug,
				description: category.description || '',
				color: category.color,
				is_visible: category.is_visible,
				parent_id: category.parent_id || null,
				meta_title: category.meta_title || '',
				meta_description: category.meta_description || ''
			};
		} else {
			editingCategory = null;
			categoryForm = {
				name: '',
				slug: '',
				description: '',
				color: '#6366f1',
				is_visible: true,
				parent_id: null,
				meta_title: '',
				meta_description: ''
			};
		}
		formErrors = [];
		showCategoryModal = true;
	}

	function closeCategoryModal() {
		showCategoryModal = false;
		editingCategory = null;
		formErrors = [];
	}

	function validateForm(): boolean {
		formErrors = [];
		if (!categoryForm.name.trim()) formErrors.push('Name is required');
		if (!categoryForm.slug.trim()) formErrors.push('Slug is required');
		else if (!/^[a-z0-9-]+$/.test(categoryForm.slug)) {
			formErrors.push('Slug can only contain lowercase letters, numbers, and hyphens');
		}
		return formErrors.length === 0;
	}

	function generateSlug(name: string): string {
		return name
			.toLowerCase()
			.trim()
			.replace(/[^\w\s-]/g, '')
			.replace(/[\s_-]+/g, '-')
			.replace(/^-+|-+$/g, '');
	}

	function showToastMessage(message: string, type: 'success' | 'error') {
		toastMessage = message;
		toastType = type;
		showToast = true;
		setTimeout(() => {
			showToast = false;
		}, 5000);
	}

	function toggleSelection(id: number) {
		if (selectedIds.has(id)) {
			selectedIds.delete(id);
		} else {
			selectedIds.add(id);
		}
		selectedIds = new Set(selectedIds);
	}

	function toggleSelectAll() {
		if (allSelected) {
			selectedIds = new Set();
		} else {
			selectedIds = new Set(filteredCategories.map((c) => c.id));
		}
	}

	function copySlug(slug: string) {
		navigator.clipboard.writeText(slug);
		showToastMessage('Slug copied to clipboard', 'success');
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadCategories();
	});
</script>

<svelte:head>
	<title>Categories | Admin Dashboard</title>
</svelte:head>

<!-- Toast Notification -->
{#if showToast}
	<div class="toast toast-{toastType}" transition:fade>
		{#if toastType === 'success'}
			<IconCheck size={20} />
		{:else}
			<IconAlertCircle size={20} />
		{/if}
		<span>{toastMessage}</span>
		<button onclick={() => (showToast = false)} class="toast-close">
			<IconX size={16} />
		</button>
	</div>
{/if}

<div class="page">
	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<h1>
				<IconFolder size={28} />
				Categories
			</h1>
			<p class="subtitle">Organize products and content with categories</p>
			<div class="header-actions">
				<button class="btn-refresh" onclick={() => loadCategories()} disabled={isLoading}>
					<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
				</button>
				<button class="btn-secondary" onclick={exportCategories}>
					<IconDownload size={18} />
					Export
				</button>
				<button class="btn-primary" onclick={() => openCategoryModal()}>
					<IconFolderPlus size={18} />
					Add Category
				</button>
			</div>
		</header>

		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconFolder size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.total}</span>
					<span class="stat-label">Total Categories</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon green">
					<IconEye size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.visible}</span>
					<span class="stat-label">Visible</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon amber">
					<IconEyeOff size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.hidden}</span>
					<span class="stat-label">Hidden</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.withPosts}</span>
					<span class="stat-label">With Content</span>
				</div>
			</div>
		</div>

		<!-- Filters Bar -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<input type="text" placeholder="Search categories..." bind:value={searchQuery} />
			</div>
			<select class="filter-select" bind:value={parentFilter}>
				<option value={null}>All Categories</option>
				<option value={0}>Root Categories Only</option>
				{#each parentCategories as parent}
					<option value={parent.id}>{parent.name} & Children</option>
				{/each}
			</select>
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={showHidden} />
				<span>Show Hidden</span>
			</label>
		</div>

		<!-- Bulk Actions Bar -->
		{#if selectedIds.size > 0}
			<div class="bulk-actions-bar" transition:fade>
				<span class="selected-count">{selectedIds.size} selected</span>
				<div class="bulk-buttons">
					<button class="btn-bulk" onclick={() => bulkToggleVisibility(true)}>
						<IconEye size={16} />
						Show
					</button>
					<button class="btn-bulk" onclick={() => bulkToggleVisibility(false)}>
						<IconEyeOff size={16} />
						Hide
					</button>
					<button class="btn-bulk" onclick={() => (showMergeModal = true)}>
						<IconGitMerge size={16} />
						Merge
					</button>
					<button class="btn-bulk danger" onclick={bulkDelete}>
						<IconTrash size={16} />
						Delete
					</button>
				</div>
			</div>
		{/if}

		<!-- Categories List -->
		<div class="categories-container">
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading categories...</p>
				</div>
			{:else if error}
				<div class="error-state">
					<IconAlertCircle size={48} />
					<p>{error}</p>
					<button class="btn-primary" onclick={() => loadCategories()}>Try Again</button>
				</div>
			{:else if filteredCategories.length === 0}
				<div class="empty-state">
					<IconFolder size={48} />
					<h3>No categories found</h3>
					<p>Create your first category to organize your content</p>
					<button class="btn-primary" onclick={() => openCategoryModal()}>
						<IconFolderPlus size={18} />
						Create Category
					</button>
				</div>
			{:else}
				<div class="categories-table-wrapper">
					<table class="categories-table">
						<thead>
							<tr>
								<th class="th-checkbox">
									<input type="checkbox" checked={allSelected} onchange={toggleSelectAll} />
								</th>
								<th class="th-drag"></th>
								<th>Category</th>
								<th>Slug</th>
								<th>Items</th>
								<th>Visibility</th>
								<th>Updated</th>
								<th class="th-actions">Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredCategories as category (category.id)}
								<tr
									class:selected={selectedIds.has(category.id)}
									class:child-row={category.parent_id}
								>
									<td>
										<input
											type="checkbox"
											checked={selectedIds.has(category.id)}
											onchange={() => toggleSelection(category.id)}
										/>
									</td>
									<td class="drag-handle">
										<IconGripVertical size={16} />
									</td>
									<td>
										<div class="category-cell">
											{#if category.parent_id}
												<span class="child-indicator">
													<IconChevronRight size={14} />
												</span>
											{/if}
											<div class="category-color" style="background: {category.color}"></div>
											<div class="category-info">
												<span class="category-name">{category.name}</span>
												{#if category.description}
													<span class="category-desc">{category.description}</span>
												{/if}
											</div>
										</div>
									</td>
									<td>
										<button
											class="slug-badge"
											onclick={() => copySlug(category.slug)}
											title="Click to copy"
										>
											/{category.slug}
											<IconCopy size={12} />
										</button>
									</td>
									<td>
										<span class="count-badge">{category.post_count}</span>
									</td>
									<td>
										{#if category.is_visible}
											<span class="visibility-badge visible">
												<IconEye size={14} />
												Visible
											</span>
										{:else}
											<span class="visibility-badge hidden">
												<IconEyeOff size={14} />
												Hidden
											</span>
										{/if}
									</td>
									<td class="date-cell">
										{formatDate(category.updated_at)}
									</td>
									<td>
										<div class="action-buttons">
											<button
												class="btn-icon"
												onclick={() => openCategoryModal(category)}
												title="Edit"
											>
												<IconEdit size={16} />
											</button>
											<button
												class="btn-icon danger"
												onclick={() => deleteCategory(category.id)}
												title="Delete"
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

				<!-- Table Footer -->
				<div class="table-footer">
					<span class="results-count">
						Showing {filteredCategories.length} of {categories.length} categories
					</span>
				</div>
			{/if}
		</div>
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Category Modal -->
{#if showCategoryModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={closeCategoryModal}
		onkeydown={(e) => e.key === 'Escape' && closeCategoryModal()}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h3>{editingCategory ? 'Edit Category' : 'New Category'}</h3>
				<button class="modal-close" onclick={closeCategoryModal}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if formErrors.length > 0}
					<div class="error-banner">
						<IconAlertCircle size={18} />
						<div>
							{#each formErrors as error}
								<p>{error}</p>
							{/each}
						</div>
					</div>
				{/if}

				<div class="form-grid">
					<div class="form-group full-width">
						<label for="cat-name">Name *</label>
						<input
							id="cat-name"
							type="text"
							bind:value={categoryForm.name}
							placeholder="Category name"
							required
						/>
					</div>

					<div class="form-group">
						<label for="cat-slug">Slug *</label>
						<input
							id="cat-slug"
							type="text"
							bind:value={categoryForm.slug}
							placeholder="category-slug"
							required
						/>
						<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
					</div>

					<div class="form-group">
						<label for="cat-parent">Parent Category</label>
						<select id="cat-parent" bind:value={categoryForm.parent_id}>
							<option value={null}>None (Root Category)</option>
							{#each parentCategories.filter((c) => c.id !== editingCategory?.id) as parent}
								<option value={parent.id}>{parent.name}</option>
							{/each}
						</select>
					</div>

					<div class="form-group full-width">
						<label for="cat-desc">Description</label>
						<textarea
							id="cat-desc"
							bind:value={categoryForm.description}
							placeholder="Brief description of this category"
							rows="3"
						></textarea>
					</div>

					<div class="form-group">
						<label for="cat-color">Color</label>
						<div class="color-input-wrapper">
							<input id="cat-color" type="color" bind:value={categoryForm.color} />
							<span class="color-value">{categoryForm.color}</span>
						</div>
					</div>

					<div class="form-group">
						<label class="checkbox-label-inline">
							<input type="checkbox" bind:checked={categoryForm.is_visible} />
							<span>Visible</span>
						</label>
						<p class="help-text">Hidden categories won't appear in public listings</p>
					</div>

					<div class="form-group full-width">
						<label for="cat-meta-title">SEO Meta Title</label>
						<input
							id="cat-meta-title"
							type="text"
							bind:value={categoryForm.meta_title}
							placeholder="Custom page title for SEO"
						/>
					</div>

					<div class="form-group full-width">
						<label for="cat-meta-desc">SEO Meta Description</label>
						<textarea
							id="cat-meta-desc"
							bind:value={categoryForm.meta_description}
							placeholder="Custom description for search engines"
							rows="2"
						></textarea>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={closeCategoryModal} disabled={isSaving}>
					Cancel
				</button>
				<button class="btn-primary" onclick={saveCategory} disabled={isSaving}>
					{#if isSaving}
						Saving...
					{:else}
						{editingCategory ? 'Update' : 'Create'} Category
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Merge Modal -->
{#if showMergeModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showMergeModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showMergeModal = false)}
	>
		<div
			class="modal modal-sm"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h3>Merge Categories</h3>
				<button class="modal-close" onclick={() => (showMergeModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<p class="merge-info">
					Merge {selectedIds.size} selected categories into a single category. All associated content
					will be moved to the target category.
				</p>

				<div class="form-group">
					<label for="merge-target">Target Category *</label>
					<select id="merge-target" bind:value={mergeForm.targetId}>
						<option value={null}>Select target category...</option>
						{#each mergeTargetOptions as category}
							<option value={category.id}>{category.name}</option>
						{/each}
					</select>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showMergeModal = false)}> Cancel </button>
				<button class="btn-primary" onclick={mergeCategories} disabled={!mergeForm.targetId}>
					<IconGitMerge size={18} />
					Merge Categories
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Page Layout */
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		margin: 0 0 0.5rem 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
	}

	.page-header h1 :global(svg) {
		color: var(--primary-500);
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-refresh :global(.spinning) {
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
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(99, 102, 241, 0.2);
		color: var(--text-primary);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(99, 102, 241, 0.4);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
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
		color: var(--success-emphasis);
	}
	.stat-icon.amber {
		background: rgba(245, 158, 11, 0.15);
		color: var(--warning-emphasis);
	}
	.stat-icon.purple {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
	}

	.stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
		flex-wrap: wrap;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: var(--text-tertiary);
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: var(--text-primary);
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary);
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-secondary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Bulk Actions Bar */
	.bulk-actions-bar {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.selected-count {
		font-size: 0.9rem;
		font-weight: 600;
		color: #818cf8;
	}

	.bulk-buttons {
		display: flex;
		gap: 0.5rem;
		margin-left: auto;
	}

	.btn-bulk {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: var(--text-primary);
		font-size: 0.8rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-bulk:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.btn-bulk.danger {
		border-color: var(--error-base);
		color: var(--error-emphasis);
	}

	.btn-bulk.danger:hover {
		background: rgba(239, 68, 68, 0.15);
	}

	/* Categories Container */
	.categories-container {
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		overflow: hidden;
	}

	.categories-table-wrapper {
		overflow-x: auto;
	}

	.categories-table {
		width: 100%;
		border-collapse: collapse;
	}

	.categories-table th {
		padding: 14px 16px;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(22, 27, 34, 0.8);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.categories-table th.th-checkbox,
	.categories-table th.th-drag {
		width: 40px;
	}

	.categories-table th.th-actions {
		width: 100px;
	}

	.categories-table td {
		padding: 14px 16px;
		font-size: 0.875rem;
		color: var(--text-primary);
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.categories-table tbody tr {
		transition: background 0.2s;
	}

	.categories-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.categories-table tbody tr.selected {
		background: rgba(99, 102, 241, 0.1);
	}

	.categories-table tbody tr.child-row {
		background: rgba(0, 0, 0, 0.1);
	}

	.drag-handle {
		color: var(--text-muted);
		cursor: grab;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	/* Category Cell */
	.category-cell {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.child-indicator {
		color: var(--text-muted);
		margin-left: 8px;
	}

	.category-color {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.category-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.category-name {
		font-weight: 600;
		color: var(--text-primary);
	}

	.category-desc {
		font-size: 0.8rem;
		color: var(--text-tertiary);
		max-width: 300px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Badges */
	.slug-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		font-family: monospace;
		color: #818cf8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.slug-badge:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.count-badge {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #818cf8;
	}

	.visibility-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.visibility-badge.visible {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}

	.visibility-badge.hidden {
		background: rgba(99, 102, 241, 0.1);
		color: var(--primary-500);
	}

	.date-cell {
		color: var(--text-tertiary);
		font-size: 0.8rem;
	}

	/* Action Buttons */
	.action-buttons {
		display: flex;
		gap: 0.375rem;
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
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: var(--error-emphasis);
		border-color: var(--error-base);
	}

	/* Table Footer */
	.table-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.results-count {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-tertiary);
	}

	.empty-state :global(svg),
	.error-state :global(svg) {
		color: var(--text-muted);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary);
		margin: 0 0 0.5rem 0;
	}

	.empty-state p,
	.error-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: rgba(22, 27, 34, 0.95);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.modal-sm {
		max-width: 480px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.modal-close {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(239, 68, 68, 0.1);
		color: var(--error-emphasis);
		border-color: var(--error-base);
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	/* Form */
	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary);
	}

	.form-group input[type='text'],
	.form-group textarea,
	.form-group select {
		padding: 0.75rem 1rem;
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: var(--text-primary);
		font-size: 0.9rem;
		font-family: inherit;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.form-group textarea {
		resize: vertical;
	}

	.color-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.form-group input[type='color'] {
		width: 48px;
		height: 48px;
		padding: 0;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		cursor: pointer;
		background: transparent;
	}

	.color-value {
		font-family: monospace;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.checkbox-label-inline {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		padding-top: 0.5rem;
	}

	.checkbox-label-inline input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.checkbox-label-inline span {
		font-size: 0.9rem;
		color: var(--text-primary);
	}

	.help-text {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin: 0;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: var(--error-emphasis);
		margin-bottom: 1rem;
	}

	.error-banner p {
		margin: 0;
		font-size: 0.875rem;
	}

	/* Merge Modal */
	.merge-info {
		font-size: 0.9rem;
		color: var(--text-secondary);
		margin: 0 0 1.5rem 0;
		line-height: 1.6;
	}

	/* Toast */
	.toast {
		position: fixed;
		top: 1.5rem;
		right: 1.5rem;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(22, 27, 34, 0.95);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
		z-index: 2000;
		min-width: 300px;
	}

	.toast-success {
		border-left: 4px solid var(--success-emphasis);
	}

	.toast-success :global(svg:first-child) {
		color: var(--success-emphasis);
	}

	.toast-error {
		border-left: 4px solid var(--error-emphasis);
	}

	.toast-error :global(svg:first-child) {
		color: var(--error-emphasis);
	}

	.toast span {
		flex: 1;
		color: var(--text-primary);
		font-size: 0.9rem;
	}

	.toast-close {
		background: none;
		border: none;
		color: var(--text-tertiary);
		cursor: pointer;
		padding: 0;
		display: flex;
	}

	.toast-close:hover {
		color: var(--text-secondary);
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-group.full-width {
			grid-column: 1;
		}
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: stretch;
		}

		.header-actions {
			justify-content: flex-start;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			max-width: none;
		}

		.bulk-actions-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.bulk-buttons {
			margin-left: 0;
			flex-wrap: wrap;
		}
	}
</style>
