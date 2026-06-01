<!--
/**
 * Blog Categories & Tags Management — Google L4+ Enterprise Implementation
 * Enterprise features: TypeScript types, API integration, bulk operations,
 * search/filter, validation, loading states, error handling.
 *
 * R28-C extraction (2026-05-20): shell-only — markup is delegated to seven
 * leaf components in `_components/`:
 *   - CategoriesPageHeader.svelte
 *   - CategoriesToolbar.svelte
 *   - CategorySection.svelte
 *   - TagSection.svelte
 *   - CategoryFormModal.svelte
 *   - TagFormModal.svelte
 *   - ToastNotification.svelte
 *   - types.ts (hoisted form shapes)
 * Confirmation dialogs continue to use the shared `ConfirmationModal`.
 */
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import { categoriesApi, tagsApi, AdminApiError, type Category, type Tag } from '$lib/api/admin';
	import { logger } from '$lib/utils/logger';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	import CategoriesPageHeader from './_components/CategoriesPageHeader.svelte';
	import CategoriesToolbar from './_components/CategoriesToolbar.svelte';
	import CategorySection from './_components/CategorySection.svelte';
	import TagSection from './_components/TagSection.svelte';
	import CategoryFormModal from './_components/CategoryFormModal.svelte';
	import TagFormModal from './_components/TagFormModal.svelte';
	import ToastNotification from './_components/ToastNotification.svelte';
	import type { CategoryFormData, TagFormData, ToastType } from './_components/types';

	// State
	let categories = $state<Category[]>([]);
	let tags = $state<Tag[]>([]);
	const filteredCategories = $derived(
		categories.filter((cat) => {
			const matchesSearch =
				cat.name.toLowerCase().includes(categorySearch.toLowerCase()) ||
				cat.description?.toLowerCase().includes(categorySearch.toLowerCase());
			const matchesVisibility = showHidden || cat.is_visible;
			return matchesSearch && matchesVisibility;
		})
	);
	const filteredTags = $derived(
		tags.filter((tag) => {
			const matchesSearch = tag.name.toLowerCase().includes(categorySearch.toLowerCase());
			const matchesVisibility = showHidden || tag.is_visible;
			return matchesSearch && matchesVisibility;
		})
	);
	let loading = $state(false);
	let saving = $state(false);
	let showCategoryModal = $state(false);
	let showTagModal = $state(false);
	let editingCategory = $state<Category | null>(null);
	let editingTag = $state<Tag | null>(null);

	// Search & Filter (unified search for both categories and tags)
	let categorySearch = $state('');
	let showHidden = $state(false);

	// Selection for bulk operations
	let selectedCategories = new SvelteSet<number>();
	let selectedTags = new SvelteSet<number>();

	// Forms
	let categoryForm = $state<CategoryFormData>({
		name: '',
		slug: '',
		description: '',
		color: '#3b82f6',
		is_visible: true
	});

	let tagForm = $state<TagFormData>({
		name: '',
		slug: '',
		color: '#10b981',
		is_visible: true
	});

	// Validation errors
	let categoryErrors = $state<string[]>([]);
	let tagErrors = $state<string[]>([]);

	// Toast notifications
	let toastMessage = $state('');
	let toastType = $state<ToastType>('success');
	let showToast = $state(false);

	// Delete confirmation modal state
	let showDeleteCategoryModal = $state(false);
	let showDeleteTagModal = $state(false);
	let showBulkDeleteCategoriesModal = $state(false);
	let showBulkDeleteTagsModal = $state(false);
	let pendingDeleteCategory = $state<Category | null>(null);
	let pendingDeleteTag = $state<Tag | null>(null);

	// FIX-2026-04-26 (P0-4): once a user manually edits the slug, stop
	// auto-generating on every name keystroke. Tracked per-form.
	let categorySlugEdited = $state(false);
	let tagSlugEdited = $state(false);

	// Auto-generate slug when name changes (only for new items, not when editing)
	$effect(() => {
		if (categoryForm.name && !editingCategory && !categorySlugEdited) {
			categoryForm.slug = generateSlug(categoryForm.name);
		}
	});
	$effect(() => {
		if (tagForm.name && !editingTag && !tagSlugEdited) {
			tagForm.slug = generateSlug(tagForm.name);
		}
	});

	onMount(() => {
		loadData();
	});

	async function loadData() {
		loading = true;
		try {
			await Promise.all([loadCategories(), loadTags()]);
		} catch (_error) {
			showToastMessage('Failed to load data', 'error');
		} finally {
			loading = false;
		}
	}

	async function loadCategories() {
		try {
			const response = await categoriesApi.list({ all: true });
			categories = (response.data || []) as Category[];
		} catch (error) {
			logger.error('Failed to load categories', { error });
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	async function loadTags() {
		try {
			const response = await tagsApi.list({ all: true });
			tags = (response.data || []) as Tag[];
		} catch (error) {
			logger.error('Failed to load tags', { error });
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
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
			categorySlugEdited = true;
		} else {
			editingCategory = null;
			categoryForm = { name: '', slug: '', description: '', color: '#3b82f6', is_visible: true };
			categorySlugEdited = false;
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
			tagSlugEdited = true;
		} else {
			editingTag = null;
			tagForm = { name: '', slug: '', color: '#10b981', is_visible: true };
			tagSlugEdited = false;
		}
		tagErrors = [];
		showTagModal = true;
	}

	function validateCategory(): boolean {
		categoryErrors = [];
		if (!categoryForm.name.trim()) categoryErrors.push('Name is required');
		if (!categoryForm.slug.trim()) categoryErrors.push('Slug is required');
		else if (!/^[a-z0-9-]+$/.test(categoryForm.slug)) {
			// FIX-2026-04-26 (P2-3): include the offending characters so legacy
			// data (uppercase, underscores) can be diagnosed.
			const bad = categoryForm.slug.replace(/[a-z0-9-]/g, '');
			const detail = bad ? ` (invalid: "${[...new Set(bad.split(''))].join('')}")` : '';
			categoryErrors.push(`Slug can only contain lowercase letters, numbers, and hyphens${detail}`);
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
			logger.error('Failed to save category', { error });
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
			// FIX-2026-04-26 (P2-3): include offending characters in the error.
			const bad = tagForm.slug.replace(/[a-z0-9-]/g, '');
			const detail = bad ? ` (invalid: "${[...new Set(bad.split(''))].join('')}")` : '';
			tagErrors.push(`Slug can only contain lowercase letters, numbers, and hyphens${detail}`);
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
			logger.error('Failed to save tag', { error });
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

	function deleteCategory(id: number) {
		const category = categories.find((c) => c.id === id);
		if (!category) return;
		pendingDeleteCategory = category;
		showDeleteCategoryModal = true;
	}

	async function confirmDeleteCategory() {
		if (!pendingDeleteCategory) return;
		showDeleteCategoryModal = false;
		const id = pendingDeleteCategory.id;
		pendingDeleteCategory = null;
		try {
			await categoriesApi.delete(id);
			showToastMessage('Category deleted successfully', 'success');
			await loadCategories();
		} catch (error) {
			logger.error('Failed to delete category', { error });
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			} else {
				showToastMessage('Failed to delete category', 'error');
			}
		}
	}

	function deleteTag(id: number) {
		const tag = tags.find((t) => t.id === id);
		if (!tag) return;
		pendingDeleteTag = tag;
		showDeleteTagModal = true;
	}

	async function confirmDeleteTag() {
		if (!pendingDeleteTag) return;
		showDeleteTagModal = false;
		const id = pendingDeleteTag.id;
		pendingDeleteTag = null;
		try {
			await tagsApi.delete(id);
			showToastMessage('Tag deleted successfully', 'success');
			await loadTags();
		} catch (error) {
			logger.error('Failed to delete tag', { error });
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			} else {
				showToastMessage('Failed to delete tag', 'error');
			}
		}
	}

	// Bulk operations
	function bulkDeleteCategories() {
		if (selectedCategories.size === 0) return;
		showBulkDeleteCategoriesModal = true;
	}

	async function confirmBulkDeleteCategories() {
		showBulkDeleteCategoriesModal = false;
		try {
			await categoriesApi.bulkDelete(Array.from(selectedCategories));
			showToastMessage('Categories deleted successfully', 'success');
			// FIX-2026-04-26 (P2-2): Set mutations are reactive in Svelte 5 — drop
			// the legacy self-assignment hack and reassign with a fresh Set.
			selectedCategories.clear();
			await loadCategories();
		} catch (error) {
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	function bulkDeleteTags() {
		if (selectedTags.size === 0) return;
		showBulkDeleteTagsModal = true;
	}

	async function confirmBulkDeleteTags() {
		showBulkDeleteTagsModal = false;
		try {
			await tagsApi.bulkDelete(Array.from(selectedTags));
			showToastMessage('Tags deleted successfully', 'success');
			// FIX-2026-04-26 (P2-2): drop legacy self-assignment hack.
			selectedTags.clear();
			await loadTags();
		} catch (error) {
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

	// Selection toggles invoked from the row children. Set mutations
	// are fully reactive in Svelte 5 — no reassignment needed.
	function toggleCategorySelect(id: number, checked: boolean) {
		if (checked) {
			selectedCategories.add(id);
		} else {
			selectedCategories.delete(id);
		}
	}

	function toggleTagSelect(id: number, checked: boolean) {
		if (checked) {
			selectedTags.add(id);
		} else {
			selectedTags.delete(id);
		}
	}

	function copyCategorySlug(slug: string) {
		navigator.clipboard.writeText(slug);
	}

	function copyTagSlug(slug: string) {
		navigator.clipboard.writeText(slug);
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

	function showToastMessage(message: string, type: ToastType) {
		toastMessage = message;
		toastType = type;
		showToast = true;
		setTimeout(() => {
			showToast = false;
		}, 5000);
	}

	function handleCategorySlugInput() {
		categorySlugEdited = true;
	}
	function handleTagSlugInput() {
		tagSlugEdited = true;
	}
</script>

<svelte:head>
	<title>Categories & Tags | Blog</title>
</svelte:head>

<div class="categories-page">
	{#if showToast}
		<ToastNotification
			message={toastMessage}
			type={toastType}
			onClose={() => (showToast = false)}
		/>
	{/if}

	<CategoriesPageHeader {loading} onRefresh={loadData} />

	<CategoriesToolbar bind:search={categorySearch} bind:showHidden />

	<div class="content-grid">
		<CategorySection
			categories={filteredCategories}
			{loading}
			selected={selectedCategories}
			onAdd={() => openCategoryModal()}
			onEdit={openCategoryModal}
			onDelete={deleteCategory}
			onCopySlug={copyCategorySlug}
			onBulkDelete={bulkDeleteCategories}
			onToggleSelect={toggleCategorySelect}
		/>

		<TagSection
			tags={filteredTags}
			{loading}
			selected={selectedTags}
			onAdd={() => openTagModal()}
			onEdit={openTagModal}
			onDelete={deleteTag}
			onCopySlug={copyTagSlug}
			onBulkDelete={bulkDeleteTags}
			onToggleSelect={toggleTagSelect}
		/>
	</div>
</div>

<CategoryFormModal
	open={showCategoryModal}
	isEdit={!!editingCategory}
	bind:form={categoryForm}
	errors={categoryErrors}
	{saving}
	onSlugInput={handleCategorySlugInput}
	onSave={saveCategory}
	onClose={() => (showCategoryModal = false)}
/>

<TagFormModal
	open={showTagModal}
	isEdit={!!editingTag}
	bind:form={tagForm}
	errors={tagErrors}
	{saving}
	onSlugInput={handleTagSlugInput}
	onSave={saveTag}
	onClose={() => (showTagModal = false)}
/>

<ConfirmationModal
	isOpen={showDeleteCategoryModal}
	title="Delete Category"
	message={pendingDeleteCategory
		? `Delete "${pendingDeleteCategory.name}"? ${pendingDeleteCategory.post_count} posts will not be deleted.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteCategory}
	onCancel={() => {
		showDeleteCategoryModal = false;
		pendingDeleteCategory = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteTagModal}
	title="Delete Tag"
	message={pendingDeleteTag
		? `Delete "${pendingDeleteTag.name}"? ${pendingDeleteTag.post_count} posts will not be deleted.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTag}
	onCancel={() => {
		showDeleteTagModal = false;
		pendingDeleteTag = null;
	}}
/>

<ConfirmationModal
	isOpen={showBulkDeleteCategoriesModal}
	title="Delete Categories"
	message={`Delete ${selectedCategories.size} selected categories?`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={confirmBulkDeleteCategories}
	onCancel={() => (showBulkDeleteCategoriesModal = false)}
/>

<ConfirmationModal
	isOpen={showBulkDeleteTagsModal}
	title="Delete Tags"
	message={`Delete ${selectedTags.size} selected tags?`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={confirmBulkDeleteTags}
	onCancel={() => (showBulkDeleteTagsModal = false)}
/>

<style>
	.categories-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
		background: var(--admin-bg, #0f172a);
		color: #f1f5f9;
	}

	.content-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
		gap: 2rem;
	}

	@media (max-width: 1023.98px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
