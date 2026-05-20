<!--
/**
 * Admin /blog/categories — Categories & Tags management.
 *
 * R27-E extraction (2026-05-20): broke the 1515-LOC page into seven
 * focused components under `_components/`:
 *   - BlogTaxonomyPageHeader.svelte  — title + Refresh button
 *   - BlogTaxonomyToolbar.svelte     — search + show-hidden ($bindable)
 *   - BlogToastNotification.svelte   — success/error toast
 *   - BlogTaxonomyList.svelte        — GENERIC list panel; renders
 *                                       categories OR tags, eliminating
 *                                       the ~95% duplication between
 *                                       the two original "<!-- ... Section -->"
 *                                       blocks (~150 LOC saved).
 *   - BlogCategoryModal.svelte       — create/edit Category (has description)
 *   - BlogTagModal.svelte            — create/edit Tag (no description)
 *   - types.ts                       — CategoryFormData / TagFormData /
 *                                       ToastType + re-export of canonical
 *                                       Category / Tag entity types.
 *
 * The page now owns ONLY: state, load/save/delete API orchestration,
 * filter $effect (one for both lists), validation, slug-auto-generator
 * $effects, and the four ConfirmationModal wires for the destructive
 * actions.
 */
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { categoriesApi, tagsApi, AdminApiError } from '$lib/api/admin';
	import { logger } from '$lib/utils/logger';
	import { IconFolder, IconTag } from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	import BlogCategoryModal from './_components/BlogCategoryModal.svelte';
	import BlogTagModal from './_components/BlogTagModal.svelte';
	import BlogTaxonomyList from './_components/BlogTaxonomyList.svelte';
	import BlogTaxonomyPageHeader from './_components/BlogTaxonomyPageHeader.svelte';
	import BlogTaxonomyToolbar from './_components/BlogTaxonomyToolbar.svelte';
	import BlogToastNotification from './_components/BlogToastNotification.svelte';
	import type {
		Category,
		CategoryFormData,
		Tag,
		TagFormData,
		ToastType
	} from './_components/types';

	// State
	let categories = $state<Category[]>([]);
	let tags = $state<Tag[]>([]);
	let filteredCategories = $state<Category[]>([]);
	let filteredTags = $state<Tag[]>([]);
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
	let selectedCategories = $state(new Set<number>());
	let selectedTags = $state(new Set<number>());

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
	// clobbering it on every name keystroke. Tracked per-form.
	let categorySlugEdited = $state(false);
	let tagSlugEdited = $state(false);

	$effect(() => {
		if (browser) {
			loadData();
		}
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
			applyFilters();
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
			applyFilters();
		} catch (error) {
			logger.error('Failed to load tags', { error });
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
			// Use categorySearch for unified search across both categories and tags
			const matchesSearch = tag.name.toLowerCase().includes(categorySearch.toLowerCase());
			const matchesVisibility = showHidden || tag.is_visible;
			return matchesSearch && matchesVisibility;
		});
	}

	// Effect to apply filters when search/filter changes
	$effect(() => {
		// Track dependencies — accessing these state values triggers re-run on change
		categorySearch;
		showHidden;
		applyFilters();
	});

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
			// FIX-2026-04-26 (P0-4): editing — slug is user-curated already.
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
			// FIX-2026-04-26 (P0-4): editing — slug is user-curated.
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
			// FIX-2026-04-26 (P2-3): include the offending characters.
			const bad = categoryForm.slug.replace(/[a-z0-9-]/g, '');
			const detail = bad ? ` (invalid: "${[...new Set(bad.split(''))].join('')}")` : '';
			categoryErrors.push(
				`Slug can only contain lowercase letters, numbers, and hyphens${detail}`
			);
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
			// FIX-2026-04-26 (P2-3): include offending characters.
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

	function requestDeleteCategory(id: number) {
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

	function requestDeleteTag(id: number) {
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

	function bulkDeleteCategories() {
		if (selectedCategories.size === 0) return;
		showBulkDeleteCategoriesModal = true;
	}

	async function confirmBulkDeleteCategories() {
		showBulkDeleteCategoriesModal = false;
		try {
			await categoriesApi.bulkDelete(Array.from(selectedCategories));
			showToastMessage('Categories deleted successfully', 'success');
			// FIX-2026-04-26 (P2-2): Set mutations are reactive in Svelte 5.
			selectedCategories = new Set();
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
			selectedTags = new Set();
			await loadTags();
		} catch (error) {
			if (error instanceof AdminApiError) {
				showToastMessage(error.message, 'error');
			}
		}
	}

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

	// Auto-generate slug when name changes for categories
	$effect(() => {
		if (categoryForm.name && !editingCategory && !categorySlugEdited) {
			categoryForm.slug = generateSlug(categoryForm.name);
		}
	});

	// Auto-generate slug when name changes for tags
	$effect(() => {
		if (tagForm.name && !editingTag && !tagSlugEdited) {
			tagForm.slug = generateSlug(tagForm.name);
		}
	});
</script>

<svelte:head>
	<title>Categories & Tags | Blog</title>
</svelte:head>

<div class="categories-page">
	{#if showToast}
		<BlogToastNotification
			message={toastMessage}
			type={toastType}
			onClose={() => (showToast = false)}
		/>
	{/if}

	<BlogTaxonomyPageHeader {loading} onRefresh={loadData} />

	<BlogTaxonomyToolbar bind:search={categorySearch} bind:showHidden />

	<div class="content-grid">
		<BlogTaxonomyList
			kind="category"
			label="Categories"
			items={filteredCategories}
			{loading}
			bind:selectedIds={selectedCategories}
			onCreate={() => openCategoryModal()}
			onEdit={openCategoryModal}
			onDelete={requestDeleteCategory}
			onBulkDelete={bulkDeleteCategories}
		>
			{#snippet headerIcon()}
				<IconFolder size={24} />
			{/snippet}
			{#snippet emptyIcon()}
				<IconFolder size={48} />
			{/snippet}
		</BlogTaxonomyList>

		<BlogTaxonomyList
			kind="tag"
			label="Tags"
			items={filteredTags}
			{loading}
			bind:selectedIds={selectedTags}
			onCreate={() => openTagModal()}
			onEdit={openTagModal}
			onDelete={requestDeleteTag}
			onBulkDelete={bulkDeleteTags}
		>
			{#snippet headerIcon()}
				<IconTag size={24} />
			{/snippet}
			{#snippet emptyIcon()}
				<IconTag size={48} />
			{/snippet}
		</BlogTaxonomyList>
	</div>
</div>

<BlogCategoryModal
	isOpen={showCategoryModal}
	isEditing={editingCategory !== null}
	bind:formData={categoryForm}
	errors={categoryErrors}
	{saving}
	onClose={() => (showCategoryModal = false)}
	onSave={saveCategory}
	onSlugManuallyEdited={() => (categorySlugEdited = true)}
/>

<BlogTagModal
	isOpen={showTagModal}
	isEditing={editingTag !== null}
	bind:formData={tagForm}
	errors={tagErrors}
	{saving}
	onClose={() => (showTagModal = false)}
	onSave={saveTag}
	onSlugManuallyEdited={() => (tagSlugEdited = true)}
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
	/* Page-level chrome only — section/modal/toast styles live in _components/. */

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

	:global(.spinning) {
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

	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
