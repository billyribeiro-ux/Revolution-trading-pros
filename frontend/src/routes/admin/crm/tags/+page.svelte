<!--
	/admin/crm/tags - Contact Tag Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Tag CRUD with color support
	- Contact count per tag
	- Search filtering
	- Inline tag creation modal
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconTag,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconRefresh,
		IconUsers,
		IconPalette,
		IconX,
		IconCheck
	} from '$lib/icons';
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

	// Modal state for create/edit
	let showModal = $state(false);
	let editingTag = $state<ContactTag | null>(null);
	let isSaving = $state(false);
	let formError = $state('');
	let formData = $state({
		title: '',
		description: '',
		color: '#6366f1'
	});

	// Predefined color palette
	const colorPalette = [
		'#6366f1', '#8b5cf6', '#a855f7', '#d946ef',
		'#ec4899', '#f43f5e', '#ef4444', '#f97316',
		'#f59e0b', '#eab308', '#84cc16', '#22c55e',
		'#10b981', '#14b8a6', '#06b6d4', '#0ea5e9',
		'#3b82f6', '#6366f1', '#64748b', '#475569'
	];

	function openCreateModal() {
		editingTag = null;
		formData = { title: '', description: '', color: '#6366f1' };
		formError = '';
		showModal = true;
	}

	function openEditModal(tag: ContactTag) {
		editingTag = tag;
		formData = {
			title: tag.title,
			description: tag.description || '',
			color: tag.color || '#6366f1'
		};
		formError = '';
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingTag = null;
		formError = '';
	}

	async function saveTag() {
		if (!formData.title.trim()) {
			formError = 'Tag title is required';
			return;
		}

		isSaving = true;
		formError = '';

		try {
			if (editingTag) {
				await crmAPI.updateContactTag(editingTag.id, {
					title: formData.title.trim(),
					description: formData.description.trim() || undefined,
					color: formData.color
				});
			} else {
				await crmAPI.createContactTag({
					title: formData.title.trim(),
					description: formData.description.trim() || undefined,
					color: formData.color
				});
			}
			closeModal();
			await loadTags();
		} catch (err) {
			formError = err instanceof Error ? err.message : 'Failed to save tag';
		} finally {
			isSaving = false;
		}
	}

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

<div class="page">
	<!-- Header -->
	<div class="page-header">
		<h1>Contact Tags</h1>
		<p class="subtitle">Label and categorize your contacts with tags</p>
		<div class="header-actions">
			<button class="btn-secondary" onclick={() => loadTags()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-primary" onclick={openCreateModal}>
				<IconPlus size={18} />
				New Tag
			</button>
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
			<div class="stat-icon gold">
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
			<button class="btn-primary" onclick={openCreateModal}>
				<IconPlus size={18} />
				Create Tag
			</button>
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
						<button class="btn-icon" title="Edit" onclick={() => openEditModal(tag)}>
							<IconEdit size={16} />
						</button>
						<button class="btn-icon danger" title="Delete" onclick={() => deleteTag(tag.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Create/Edit Modal -->
{#if showModal}
	<div class="modal-overlay" onclick={closeModal}>
		<div class="modal-content" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2>{editingTag ? 'Edit Tag' : 'Create Tag'}</h2>
				<button class="modal-close" onclick={closeModal}>
					<IconX size={20} />
				</button>
			</div>
			<form class="modal-body" onsubmit={(e) => { e.preventDefault(); saveTag(); }}>
				{#if formError}
					<div class="form-error">
						{formError}
					</div>
				{/if}
				<div class="form-group">
					<label for="tag-title">Title <span class="required">*</span></label>
					<input
						type="text"
						id="tag-title"
						bind:value={formData.title}
						placeholder="Enter tag title"
						disabled={isSaving}
					/>
				</div>
				<div class="form-group">
					<label for="tag-description">Description</label>
					<textarea
						id="tag-description"
						bind:value={formData.description}
						placeholder="Optional description"
						rows="3"
						disabled={isSaving}
					></textarea>
				</div>
				<div class="form-group">
					<label>Color</label>
					<div class="color-picker">
						<div class="color-preview" style="background-color: {formData.color}"></div>
						<div class="color-palette">
							{#each colorPalette as color}
								<button
									type="button"
									class="color-swatch"
									class:selected={formData.color === color}
									style="background-color: {color}"
									onclick={() => formData.color = color}
									disabled={isSaving}
								>
									{#if formData.color === color}
										<IconCheck size={12} />
									{/if}
								</button>
							{/each}
						</div>
					</div>
				</div>
				<div class="modal-footer">
					<button type="button" class="btn-secondary" onclick={closeModal} disabled={isSaving}>
						Cancel
					</button>
					<button type="submit" class="btn-primary" disabled={isSaving}>
						{#if isSaving}
							<span class="btn-spinner"></span>
							Saving...
						{:else}
							<IconCheck size={18} />
							{editingTag ? 'Update Tag' : 'Create Tag'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.page {
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

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #E6B800;
	}

	.btn-secondary :global(.spinning) {
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
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
		max-width: 600px;
		margin-left: auto;
		margin-right: auto;
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
	.stat-icon.gold { background: rgba(230, 184, 0, 0.15); color: #E6B800; }

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

	/* Modal Styles */
	.modal-overlay {
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
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 480px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
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
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.modal-body {
		padding: 1.5rem;
	}

	.form-error {
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #f87171;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.form-group .required {
		color: #f87171;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		transition: all 0.2s;
		outline: none;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		border-color: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
	}

	.form-group input:disabled,
	.form-group textarea:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.color-picker {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.color-preview {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		border: 2px solid rgba(255, 255, 255, 0.2);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.color-palette {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.color-swatch {
		width: 28px;
		height: 28px;
		border-radius: 6px;
		border: 2px solid transparent;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.color-swatch:hover {
		transform: scale(1.1);
	}

	.color-swatch.selected {
		border-color: white;
		box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.5);
	}

	.color-swatch:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.modal-footer {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		margin-top: 0.5rem;
	}

	.btn-spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
</style>
