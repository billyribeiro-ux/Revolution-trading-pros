<!--
/**
 * Blog Categories & Tags Management - Google L4+ Enterprise Implementation
 * Enterprise features: TypeScript types, API integration, bulk operations,
 * search/filter, validation, loading states, error handling
 */
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconTag,
		IconFolder,
		IconSearch,
		IconRefresh,
		IconCheck,
		IconX,
		IconAlertCircle,
		IconChartBar,
		IconEye,
		IconEyeOff,
		IconCopy
	} from '@tabler/icons-svelte';
	import { categoriesApi, tagsApi, AdminApiError, type Category, type Tag } from '$lib/api/admin';

	// State
	let categories: Category[] = [];
	let tags: Tag[] = [];
	let filteredCategories: Category[] = [];
	let filteredTags: Tag[] = [];
	let loading = false;
	let saving = false;
	let showCategoryModal = false;
	let showTagModal = false;
	let editingCategory: Category | null = null;
	let editingTag: Tag | null = null;

	// Search & Filter
	let categorySearch = '';
	let tagSearch = '';
	let showHidden = false;

	// Selection for bulk operations
	let selectedCategories = new Set<number>();
	let selectedTags = new Set<number>();

	// Forms
	let categoryForm = {
		name: '',
		slug: '',
		description: '',
		color: '#3b82f6',
		is_visible: true
	};

	let tagForm = {
		name: '',
		slug: '',
		color: '#10b981',
		is_visible: true
	};

	// Validation errors
	let categoryErrors: string[] = [];
	let tagErrors: string[] = [];

	// Toast notifications
	let toastMessage = '';
	let toastType: 'success' | 'error' = 'success';
	let showToast = false;

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		try {
			await Promise.all([loadCategories(), loadTags()]);
		} catch (error) {
			showToastMessage('Failed to load data', 'error');
		} finally {
			loading = false;
		}
	}

	async function loadCategories() {
		try {
			const response = await categoriesApi.list({ all: true });
			categories = (response.data || []) as Category[];
			applyFilters();
		} catch (error) {
			console.error('Failed to load categories:', error);
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	async function loadTags() {
		try {
			const response = await tagsApi.list({ all: true });
			tags = (response.data || []) as Tag[];
			applyFilters();
		} catch (error) {
			console.error('Failed to load tags:', error);
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	function applyFilters() {
		filteredCategories = categories.filter((cat) => {
			const matchesSearch =
				cat.name.toLowerCase().includes(categorySearch.toLowerCase()) ||
				cat.description?.toLowerCase().includes(categorySearch.toLowerCase());
			const matchesVisibility = showHidden || cat.is_visible;
			return matchesSearch && matchesVisibility;
		});

		filteredTags = tags.filter((tag) => {
			const matchesSearch = tag.name.toLowerCase().includes(tagSearch.toLowerCase());
			const matchesVisibility = showHidden || tag.is_visible;
			return matchesSearch && matchesVisibility;
		});
	}

	$: {
		categorySearch;
		tagSearch;
		showHidden;
		applyFilters();
	}

	function openCategoryModal(category: Category | null = null) {
		if (category) {
			editingCategory = category;
			categoryForm = {
				name: category.name,
				slug: category.slug,
				description: category.description || '',
				color: category.color,
				is_visible: category.is_visible
			};
		} else {
			editingCategory = null;
			categoryForm = { name: '', slug: '', description: '', color: '#3b82f6', is_visible: true };
		}
		categoryErrors = [];
		showCategoryModal = true;
	}

	function openTagModal(tag: Tag | null = null) {
		if (tag) {
			editingTag = tag;
			tagForm = {
				name: tag.name,
				slug: tag.slug,
				color: tag.color,
				is_visible: tag.is_visible
			};
		} else {
			editingTag = null;
			tagForm = { name: '', slug: '', color: '#10b981', is_visible: true };
		}
		tagErrors = [];
		showTagModal = true;
	}

	function validateCategory(): boolean {
		categoryErrors = [];
		if (!categoryForm.name.trim()) categoryErrors.push('Name is required');
		if (!categoryForm.slug.trim()) categoryErrors.push('Slug is required');
		else if (!/^[a-z0-9-]+$/.test(categoryForm.slug)) {
			categoryErrors.push('Slug can only contain lowercase letters, numbers, and hyphens');
		}
		return categoryErrors.length === 0;
	}

	async function saveCategory() {
		if (!validateCategory()) return;

		saving = true;
		try {
			if (editingCategory) {
				await categoriesApi.update(editingCategory.id, categoryForm);
				showToastMessage('Category updated successfully', 'success');
			} else {
				await categoriesApi.create(categoryForm);
				showToastMessage('Category created successfully', 'success');
			}
			await loadCategories();
			showCategoryModal = false;
		} catch (error) {
			console.error('Failed to save category:', error);
			if (error instanceof AdminApiError) {
				if (error.isValidationError && error.response?.errors) {
					categoryErrors = Object.values(error.response.errors).flat() as string[];
				} else {
					showToastMessage(error.message, 'error');
				}
			} else {
				showToastMessage('Failed to save category', 'error');
			}
		} finally {
			saving = false;
		}
	}

	function validateTag(): boolean {
		tagErrors = [];
		if (!tagForm.name.trim()) tagErrors.push('Name is required');
		if (!tagForm.slug.trim()) tagErrors.push('Slug is required');
		else if (!/^[a-z0-9-]+$/.test(tagForm.slug)) {
			tagErrors.push('Slug can only contain lowercase letters, numbers, and hyphens');
		}
		return tagErrors.length === 0;
	}

	async function saveTag() {
		if (!validateTag()) return;

		saving = true;
		try {
			if (editingTag) {
				await tagsApi.update(editingTag.id, tagForm);
				showToastMessage('Tag updated successfully', 'success');
			} else {
				await tagsApi.create(tagForm);
				showToastMessage('Tag created successfully', 'success');
			}
			await loadTags();
			showTagModal = false;
		} catch (error) {
			console.error('Failed to save tag:', error);
			if (error instanceof AdminApiError) {
				if (error.isValidationError && error.response?.errors) {
					tagErrors = Object.values(error.response.errors).flat() as string[];
				} else {
					showToastMessage(error.message, 'error');
				}
			} else {
				showToastMessage('Failed to save tag', 'error');
			}
		} finally {
			saving = false;
		}
	}

	async function deleteCategory(id: number) {
		const category = categories.find((c) => c.id === id);
		if (!category) return;

		if (!confirm(`Delete "${category.name}"? ${category.post_count} posts will not be deleted.`))
			return;

		try {
			await categoriesApi.delete(id);
			showToastMessage('Category deleted successfully', 'success');
			await loadCategories();
		} catch (error) {
			console.error('Failed to delete category:', error);
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			} else {
				showToastMessage('Failed to delete category', 'error');
			}
		}
	}

	async function deleteTag(id: number) {
		const tag = tags.find((t) => t.id === id);
		if (!tag) return;

		if (!confirm(`Delete "${tag.name}"? ${tag.post_count} posts will not be deleted.`)) return;

		try {
			await tagsApi.delete(id);
			showToastMessage('Tag deleted successfully', 'success');
			await loadTags();
		} catch (error) {
			console.error('Failed to delete tag:', error);
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			} else {
				showToastMessage('Failed to delete tag', 'error');
			}
		}
	}

	// Bulk operations
	async function bulkDeleteCategories() {
		if (selectedCategories.size === 0) return;
		if (!confirm(`Delete ${selectedCategories.size} categories?`)) return;

		try {
			await categoriesApi.bulkDelete(Array.from(selectedCategories));
			showToastMessage('Categories deleted successfully', 'success');
			selectedCategories.clear();
			selectedCategories = selectedCategories;
			await loadCategories();
		} catch (error) {
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	async function bulkDeleteTags() {
		if (selectedTags.size === 0) return;
		if (!confirm(`Delete ${selectedTags.size} tags?`)) return;

		try {
			await tagsApi.bulkDelete(Array.from(selectedTags));
			showToastMessage('Tags deleted successfully', 'success');
			selectedTags.clear();
			selectedTags = selectedTags;
			await loadTags();
		} catch (error) {
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	// Utility functions
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

	// Auto-generate slug when name changes
	$: if (categoryForm.name && !editingCategory) {
		categoryForm.slug = generateSlug(categoryForm.name);
	}

	$: if (tagForm.name && !editingTag) {
		tagForm.slug = generateSlug(tagForm.name);
	}
</script>

<svelte:head>
	<title>Categories & Tags | Blog</title>
</svelte:head>

<div class="categories-page">
	<!-- Toast Notification -->
	{#if showToast}
		<div class="toast toast-{toastType}" transition:fade>
			{#if toastType === 'success'}
				<IconCheck size={20} />
			{:else}
				<IconAlertCircle size={20} />
			{/if}
			<span>{toastMessage}</span>
			<button on:click={() => (showToast = false)}>
				<IconX size={16} />
			</button>
		</div>
	{/if}

	<header class="page-header">
		<div class="header-content">
			<div>
				<h1>Categories & Tags</h1>
				<p>Organize your blog posts with categories and tags</p>
			</div>
			<button class="btn-secondary" on:click={loadData} disabled={loading}>
				<IconRefresh size={18} class={loading ? 'spinning' : ''} />
				Refresh
			</button>
		</div>
	</header>

	<!-- Search & Filter Toolbar -->
	<div class="toolbar">
		<div class="search-group">
			<IconSearch size={18} />
			<input
				type="text"
				class="search-input"
				placeholder="Search categories and tags..."
				bind:value={categorySearch}
			/>
		</div>
		<label class="checkbox-label">
			<input type="checkbox" bind:checked={showHidden} />
			<span>Show Hidden</span>
		</label>
	</div>

	<div class="content-grid">
		<!-- Categories Section -->
		<div class="section">
			<div class="section-header">
				<div class="section-title">
					<IconFolder size={24} />
					<h2>Categories</h2>
					<span class="count-badge">{filteredCategories.length}</span>
				</div>
				<div class="section-actions">
					{#if selectedCategories.size > 0}
						<button class="btn-danger-ghost" on:click={bulkDeleteCategories}>
							<IconTrash size={16} />
							Delete ({selectedCategories.size})
						</button>
					{/if}
					<button class="btn-primary" on:click={() => openCategoryModal()}>
						<IconPlus size={18} />
						Add Category
					</button>
				</div>
			</div>

			<div class="items-list">
				{#if loading}
					{#each Array(3) as _}
						<div class="skeleton-card"></div>
					{/each}
				{:else if filteredCategories.length === 0}
					<div class="empty-state">
						<IconFolder size={48} />
						<p>No categories found</p>
						<button class="btn-primary" on:click={() => openCategoryModal()}>
							Create your first category
						</button>
					</div>
				{:else}
					{#each filteredCategories as category (category.id)}
						<div class="item-card" transition:fade>
							<label class="checkbox-wrapper">
								<input
									type="checkbox"
									checked={selectedCategories.has(category.id)}
									on:change={(e) => {
										if (e.currentTarget.checked) {
											selectedCategories.add(category.id);
										} else {
											selectedCategories.delete(category.id);
										}
										selectedCategories = selectedCategories;
									}}
								/>
							</label>
							<div class="item-color" style="background: {category.color}"></div>
							<div class="item-info">
								<div class="item-header">
									<h3>{category.name}</h3>
									{#if !category.is_visible}
										<span class="badge badge-gray">
											<IconEyeOff size={12} />
											Hidden
										</span>
									{/if}
								</div>
								{#if category.description}
									<p class="item-description">{category.description}</p>
								{/if}
								<div class="item-meta">
									<span class="item-count">
										<IconChartBar size={14} />
										{category.post_count} posts
									</span>
									<span class="item-slug">/{category.slug}</span>
								</div>
							</div>
							<div class="item-actions">
								<button
									class="action-btn"
									on:click={() => openCategoryModal(category)}
									title="Edit"
								>
									<IconEdit size={18} />
								</button>
								<button
									class="action-btn"
									on:click={() => navigator.clipboard.writeText(category.slug)}
									title="Copy slug"
								>
									<IconCopy size={18} />
								</button>
								<button
									class="action-btn danger"
									on:click={() => deleteCategory(category.id)}
									title="Delete"
								>
									<IconTrash size={18} />
								</button>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>

		<!-- Tags Section -->
		<div class="section">
			<div class="section-header">
				<div class="section-title">
					<IconTag size={24} />
					<h2>Tags</h2>
					<span class="count-badge">{filteredTags.length}</span>
				</div>
				<div class="section-actions">
					{#if selectedTags.size > 0}
						<button class="btn-danger-ghost" on:click={bulkDeleteTags}>
							<IconTrash size={16} />
							Delete ({selectedTags.size})
						</button>
					{/if}
					<button class="btn-primary" on:click={() => openTagModal()}>
						<IconPlus size={18} />
						Add Tag
					</button>
				</div>
			</div>

			<div class="items-list">
				{#if loading}
					{#each Array(3) as _}
						<div class="skeleton-card"></div>
					{/each}
				{:else if filteredTags.length === 0}
					<div class="empty-state">
						<IconTag size={48} />
						<p>No tags found</p>
						<button class="btn-primary" on:click={() => openTagModal()}>
							Create your first tag
						</button>
					</div>
				{:else}
					{#each filteredTags as tag (tag.id)}
						<div class="item-card" transition:fade>
							<label class="checkbox-wrapper">
								<input
									type="checkbox"
									checked={selectedTags.has(tag.id)}
									on:change={(e) => {
										if (e.currentTarget.checked) {
											selectedTags.add(tag.id);
										} else {
											selectedTags.delete(tag.id);
										}
										selectedTags = selectedTags;
									}}
								/>
							</label>
							<div class="item-color" style="background: {tag.color}"></div>
							<div class="item-info">
								<div class="item-header">
									<h3>{tag.name}</h3>
									{#if !tag.is_visible}
										<span class="badge badge-gray">
											<IconEyeOff size={12} />
											Hidden
										</span>
									{/if}
								</div>
								<div class="item-meta">
									<span class="item-count">
										<IconChartBar size={14} />
										{tag.post_count} posts
									</span>
									<span class="item-slug">/{tag.slug}</span>
								</div>
							</div>
							<div class="item-actions">
								<button class="action-btn" on:click={() => openTagModal(tag)} title="Edit">
									<IconEdit size={18} />
								</button>
								<button
									class="action-btn"
									on:click={() => navigator.clipboard.writeText(tag.slug)}
									title="Copy slug"
								>
									<IconCopy size={18} />
								</button>
								<button class="action-btn danger" on:click={() => deleteTag(tag.id)} title="Delete">
									<IconTrash size={18} />
								</button>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	</div>
</div>

<!-- Category Modal -->
{#if showCategoryModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		on:click={() => (showCategoryModal = false)}
		on:keydown={(e) => e.key === 'Escape' && (showCategoryModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			on:click|stopPropagation
			on:keydown|stopPropagation
		>
			<div class="modal-header">
				<h3>{editingCategory ? 'Edit' : 'Add'} Category</h3>
				<button class="close-btn" on:click={() => (showCategoryModal = false)}>×</button>
			</div>

			<div class="modal-body">
				{#if categoryErrors.length > 0}
					<div class="error-banner">
						<IconAlertCircle size={18} />
						<div>
							{#each categoryErrors as error}
								<p>{error}</p>
							{/each}
						</div>
					</div>
				{/if}

				<div class="form-group">
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
					<label for="cat-desc">Description</label>
					<textarea
						id="cat-desc"
						bind:value={categoryForm.description}
						placeholder="Brief description"
						rows="3"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="cat-color">Color</label>
						<input id="cat-color" type="color" bind:value={categoryForm.color} />
					</div>

					<div class="form-group">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={categoryForm.is_visible} />
							<span>Visible</span>
						</label>
						<p class="help-text">Hidden categories won't appear in public listings</p>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" on:click={() => (showCategoryModal = false)} disabled={saving}
					>Cancel</button
				>
				<button class="btn-primary" on:click={saveCategory} disabled={saving}>
					{#if saving}
						Saving...
					{:else}
						Save
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Tag Modal -->
{#if showTagModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		on:click={() => (showTagModal = false)}
		on:keydown={(e) => e.key === 'Escape' && (showTagModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			on:click|stopPropagation
			on:keydown|stopPropagation
		>
			<div class="modal-header">
				<h3>{editingTag ? 'Edit' : 'Add'} Tag</h3>
				<button class="close-btn" on:click={() => (showTagModal = false)}>×</button>
			</div>

			<div class="modal-body">
				{#if tagErrors.length > 0}
					<div class="error-banner">
						<IconAlertCircle size={18} />
						<div>
							{#each tagErrors as error}
								<p>{error}</p>
							{/each}
						</div>
					</div>
				{/if}

				<div class="form-group">
					<label for="tag-name">Name *</label>
					<input
						id="tag-name"
						type="text"
						bind:value={tagForm.name}
						placeholder="Tag name"
						required
					/>
				</div>

				<div class="form-group">
					<label for="tag-slug">Slug *</label>
					<input
						id="tag-slug"
						type="text"
						bind:value={tagForm.slug}
						placeholder="tag-slug"
						required
					/>
					<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="tag-color">Color</label>
						<input id="tag-color" type="color" bind:value={tagForm.color} />
					</div>

					<div class="form-group">
						<label class="checkbox-label">
							<input type="checkbox" bind:checked={tagForm.is_visible} />
							<span>Visible</span>
						</label>
						<p class="help-text">Hidden tags won't appear in public listings</p>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" on:click={() => (showTagModal = false)} disabled={saving}
					>Cancel</button
				>
				<button class="btn-primary" on:click={saveTag} disabled={saving}>
					{#if saving}
						Saving...
					{:else}
						Save
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.categories-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
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

	.content-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
		gap: 2rem;
	}

	.section {
		background: white;
		border-radius: 12px;
		border: 1px solid #e5e5e5;
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.section-header h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.items-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.item-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.item-card:hover {
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.item-color {
		width: 40px;
		height: 40px;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.item-info {
		flex: 1;
	}

	.item-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.25rem;
	}

	.item-info p {
		font-size: 0.85rem;
		color: #666;
		margin: 0 0 0.25rem;
	}

	.item-count {
		font-size: 0.8rem;
		color: #999;
	}

	.item-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
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
		padding: 3rem 1rem;
		color: #999;
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: white;
		border-radius: 12px;
		width: 100%;
		max-width: 500px;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 2rem;
		color: #666;
		cursor: pointer;
		line-height: 1;
	}

	.modal-body {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-weight: 500;
		color: #1a1a1a;
		font-size: 0.95rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
	}

	.form-group input[type='color'] {
		height: 50px;
		cursor: pointer;
	}

	.form-group textarea {
		resize: vertical;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid #e5e5e5;
	}

	.btn-secondary {
		padding: 0.625rem 1.25rem;
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-secondary:hover {
		background: #f8f9fa;
	}

	/* Toast Notification */
	.toast {
		position: fixed;
		top: 2rem;
		right: 2rem;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: white;
		border-radius: 8px;
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
		z-index: 2000;
		min-width: 300px;
	}

	.toast-success {
		border-left: 4px solid #10b981;
		color: #10b981;
	}

	.toast-error {
		border-left: 4px solid #ef4444;
		color: #ef4444;
	}

	.toast span {
		flex: 1;
		color: #1a1a1a;
	}

	.toast button {
		background: none;
		border: none;
		color: #999;
		cursor: pointer;
		padding: 0;
		display: flex;
	}

	/* Header Content */
	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-bottom: 2rem;
		padding: 1rem;
		background: white;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.search-group {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #f8f9fa;
		border-radius: 6px;
	}

	.search-input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		font-size: 0.95rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		user-select: none;
	}

	/* Section Title */
	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.section-actions {
		display: flex;
		gap: 0.5rem;
	}

	.count-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		height: 24px;
		padding: 0 0.5rem;
		background: #f0f0f0;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
	}

	.btn-danger-ghost {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #fee2e2;
		color: #ef4444;
		border: 1px solid #fecaca;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.btn-danger-ghost:hover {
		background: #fecaca;
	}

	/* Checkbox Wrapper */
	.checkbox-wrapper {
		display: flex;
		align-items: center;
		cursor: pointer;
	}

	.checkbox-wrapper input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	/* Item Header */
	.item-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.item-description {
		font-size: 0.85rem;
		color: #666;
		margin: 0.25rem 0;
	}

	.item-meta {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: 0.8rem;
		color: #999;
	}

	.item-slug {
		font-family: monospace;
		background: #f8f9fa;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
	}

	/* Badge */
	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.badge-gray {
		background: #f3f4f6;
		color: #6b7280;
	}

	/* Loading States */
	.skeleton-card {
		height: 80px;
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: loading 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes loading {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.spinning {
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

	/* Error Banner */
	.error-banner {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: #fee2e2;
		border: 1px solid #fecaca;
		border-radius: 6px;
		color: #ef4444;
		margin-bottom: 1rem;
	}

	.error-banner p {
		margin: 0;
		font-size: 0.9rem;
	}

	/* Form Row */
	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.help-text {
		font-size: 0.8rem;
		color: #999;
		margin-top: 0.25rem;
	}

	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
