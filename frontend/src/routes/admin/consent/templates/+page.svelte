<script lang="ts">
	/**
	 * Template Selector Admin Page
	 *
	 * Full-featured admin interface for:
	 * - Browsing 20 pre-built templates
	 * - Live preview of templates
	 * - Creating custom templates
	 * - Editing existing templates
	 * - Setting active template
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { BANNER_TEMPLATES, getTemplateCategories, DEFAULT_TEMPLATE_ID } from '$lib/consent/templates/registry';
	import {
		activeTemplate,
		allTemplates,
		initializeTemplateStore,
		setActiveTemplate,
		saveAsCustomTemplate,
		deleteCustomTemplate,
		enterPreviewMode,
		exitPreviewMode,
		applyPreview,
		isPreviewMode,
		getActiveTemplateConfig,
		exportTemplateConfig,
		importTemplateConfig,
	} from '$lib/consent/templates/store';
	import TemplatePreviewCard from '$lib/consent/templates/TemplatePreviewCard.svelte';
	import TemplateEditor from '$lib/consent/templates/TemplateEditor.svelte';
	import BannerRenderer from '$lib/consent/templates/BannerRenderer.svelte';
	import type { BannerTemplate } from '$lib/consent/templates/types';

	// State
	let selectedCategory = $state('all');
	let searchQuery = $state('');
	let showEditor = $state(false);
	let editingTemplate = $state<BannerTemplate | null>(null);
	let isCreatingNew = $state(false);
	let showImportModal = $state(false);
	let importJson = $state('');
	let notification = $state('');
	const importPlaceholder = '{"activeConfig": {...}, "customTemplates": [...]}';

	// Get categories
	let categories = $derived(['all', ...getTemplateCategories()]);

	// Filtered templates
	let filteredTemplates = $derived($allTemplates.filter((t) => {
		const matchesCategory = selectedCategory === 'all' || t.category === selectedCategory;
		const matchesSearch =
			!searchQuery ||
			t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
			t.description.toLowerCase().includes(searchQuery.toLowerCase());
		return matchesCategory && matchesSearch;
	}));

	// Active template ID
	let activeTemplateId = $derived(getActiveTemplateConfig().templateId);

	onMount(() => {
		if (browser) {
			initializeTemplateStore();
		}
	});

	// Handlers
	function handleSelectTemplate(template: BannerTemplate) {
		setActiveTemplate(template.id);
		showNotification(`"${template.name}" is now active`);
	}

	function handlePreviewTemplate(template: BannerTemplate) {
		enterPreviewMode(template);
	}

	function handleEditTemplate(template: BannerTemplate) {
		editingTemplate = template;
		isCreatingNew = false;
		showEditor = true;
	}

	function handleDeleteTemplate(template: BannerTemplate) {
		if (confirm(`Are you sure you want to delete "${template.name}"?`)) {
			deleteCustomTemplate(template.id);
			showNotification(`"${template.name}" deleted`);
		}
	}

	function handleCreateNew() {
		// Create a new template based on default
		const baseTemplate = BANNER_TEMPLATES.find((t) => t.id === DEFAULT_TEMPLATE_ID)!;
		editingTemplate = {
			...JSON.parse(JSON.stringify(baseTemplate)),
			id: `custom-${Date.now().toString(36)}`,
			name: 'My Custom Template',
			description: 'A custom template created from scratch',
			category: 'custom',
			isEditable: true,
		};
		isCreatingNew = true;
		showEditor = true;
	}

	function handleSaveTemplate(template: BannerTemplate) {
		// Svelte 5: Callback props receive the value directly (no CustomEvent wrapper)

		if (isCreatingNew) {
			const newId = saveAsCustomTemplate(template.name);
			showNotification(`"${template.name}" created successfully`);
		} else {
			// Update existing custom template or save customization
			setActiveTemplate(template.id);
			showNotification(`"${template.name}" saved`);
		}

		showEditor = false;
		editingTemplate = null;
	}

	function handleCancelEdit() {
		showEditor = false;
		editingTemplate = null;
	}

	function handleEditorPreview(template: BannerTemplate) {
		// Svelte 5: Callback props receive the value directly
		enterPreviewMode(template);
	}

	function handleExport() {
		const data = exportTemplateConfig();
		const blob = new Blob([data], { type: 'application/json' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'consent-templates-export.json';
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
		showNotification('Configuration exported');
	}

	function handleImport() {
		showImportModal = true;
	}

	async function confirmImport() {
		if (await importTemplateConfig(importJson)) {
			showNotification('Configuration imported successfully');
			showImportModal = false;
			importJson = '';
		} else {
			showNotification('Failed to import configuration');
		}
	}

	function showNotification(message: string) {
		notification = message;
		setTimeout(() => {
			notification = '';
		}, 3000);
	}

	function handleApplyPreview() {
		applyPreview();
		showNotification('Template applied');
	}
</script>

<svelte:head>
	<title>Banner Templates | Admin</title>
</svelte:head>

<div class="templates-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-content">
			<h1>Banner Templates</h1>
			<p>Choose from 20 pre-built templates or create your own</p>
		</div>
		<div class="header-actions">
			<button class="btn btn-secondary" onclick={handleExport}>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
					<polyline points="17 8 12 3 7 8"/>
					<line x1="12" y1="3" x2="12" y2="15"/>
				</svg>
				Export
			</button>
			<button class="btn btn-secondary" onclick={handleImport}>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
					<polyline points="7 10 12 15 17 10"/>
					<line x1="12" y1="15" x2="12" y2="3"/>
				</svg>
				Import
			</button>
			<button class="btn btn-primary" onclick={handleCreateNew}>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M5 12h14"/>
					<path d="M12 5v14"/>
				</svg>
				Create New
			</button>
		</div>
	</header>

	<!-- Filters -->
	<div class="filters">
		<div class="search-box">
			<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="11" cy="11" r="8"/>
				<path d="m21 21-4.3-4.3"/>
			</svg>
			<input
				type="text"
				placeholder="Search templates..."
				bind:value={searchQuery}
			/>
		</div>
		<div class="category-tabs">
			{#each categories as category}
				<button
					class="category-tab"
					class:active={selectedCategory === category}
					onclick={() => (selectedCategory = category)}
				>
					{category === 'all' ? 'All Templates' : category}
					{#if category === 'all'}
						<span class="count">{$allTemplates.length}</span>
					{:else}
						<span class="count">{$allTemplates.filter((t) => t.category === category).length}</span>
					{/if}
				</button>
			{/each}
		</div>
	</div>

	<!-- Template Grid -->
	{#if !showEditor}
		<div class="templates-grid">
			{#each filteredTemplates as template (template.id)}
				<TemplatePreviewCard
					{template}
					isActive={template.id === activeTemplateId}
					onSelect={() => handleSelectTemplate(template)}
					onPreview={() => handlePreviewTemplate(template)}
					onEdit={() => handleEditTemplate(template)}
					onDelete={() => handleDeleteTemplate(template)}
				/>
			{/each}
		</div>

		{#if filteredTemplates.length === 0}
			<div class="empty-state">
				<p>No templates found matching your criteria.</p>
				<button class="btn btn-secondary" onclick={() => { searchQuery = ''; selectedCategory = 'all'; }}>
					Clear Filters
				</button>
			</div>
		{/if}
	{/if}

	<!-- Editor -->
	{#if showEditor && editingTemplate}
		<TemplateEditor
			template={editingTemplate}
			isNew={isCreatingNew}
			onSave={handleSaveTemplate}
			onCancel={handleCancelEdit}
			onPreview={handleEditorPreview}
		/>
	{/if}

	<!-- Preview Mode Banner -->
	{#if $isPreviewMode}
		<div class="preview-controls">
			<span>Preview Mode Active</span>
			<button class="btn btn-secondary btn-sm" onclick={exitPreviewMode}>
				Cancel
			</button>
			<button class="btn btn-primary btn-sm" onclick={handleApplyPreview}>
				Apply This Template
			</button>
		</div>
	{/if}

	<!-- Banner Preview -->
	<BannerRenderer forceShow={$isPreviewMode} />

	<!-- Import Modal -->
	{#if showImportModal}
		<div
			class="modal-backdrop"
			role="button"
			tabindex="0"
			onclick={() => (showImportModal = false)}
			onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showImportModal = false)}
		>
			<div class="modal" role="dialog" aria-modal="true" tabindex="-1" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()}>
				<h3>Import Configuration</h3>
				<p>Paste your exported JSON configuration:</p>
				<textarea
					class="import-textarea"
					bind:value={importJson}
					placeholder={importPlaceholder}
					rows="10"
				></textarea>
				<div class="modal-actions">
					<button class="btn btn-secondary" onclick={() => (showImportModal = false)}>
						Cancel
					</button>
					<button class="btn btn-primary" onclick={confirmImport}>
						Import
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Notification -->
	{#if notification}
		<div class="notification">
			<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="m9 12 2 2 4-4"/>
				<circle cx="12" cy="12" r="10"/>
			</svg>
			{notification}
		</div>
	{/if}
</div>

<style>
	.templates-page {
		min-height: 100vh;
		background: #0a101c;
		color: #e2e8f0;
		padding: 2rem;
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
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
	}

	.page-header p {
		margin: 0.25rem 0 0;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #0ea5e9, #06b6d4);
		color: white;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.btn-sm {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	.btn:hover {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	.filters {
		margin-bottom: 2rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		margin-bottom: 1rem;
	}

	.search-box svg {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.category-tabs {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.category-tab {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid transparent;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
		text-transform: capitalize;
	}

	.category-tab:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}

	.category-tab.active {
		background: rgba(14, 165, 233, 0.15);
		border-color: rgba(14, 165, 233, 0.3);
		color: #38bdf8;
	}

	.category-tab .count {
		padding: 0.125rem 0.375rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		font-size: 0.6875rem;
	}

	.templates-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(255, 255, 255, 0.02);
		border-radius: 12px;
	}

	.empty-state p {
		margin: 0 0 1rem;
		color: #64748b;
	}

	.preview-controls {
		position: fixed;
		bottom: 6rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1.5rem;
		background: rgba(15, 23, 42, 0.95);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 99998;
		backdrop-filter: blur(10px);
	}

	.preview-controls span {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100000;
		padding: 1rem;
	}

	.modal {
		background: #1e293b;
		border-radius: 12px;
		padding: 1.5rem;
		width: 100%;
		max-width: 500px;
		max-height: 80vh;
		overflow-y: auto;
	}

	.modal h3 {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
	}

	.modal p {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.import-textarea {
		width: 100%;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #f1f5f9;
		font-family: monospace;
		font-size: 0.8125rem;
		resize: vertical;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.notification {
		position: fixed;
		bottom: 2rem;
		right: 2rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: #22c55e;
		color: white;
		border-radius: 10px;
		font-size: 0.875rem;
		font-weight: 500;
		box-shadow: 0 10px 30px rgba(34, 197, 94, 0.3);
		animation: slideIn 0.3s ease;
		z-index: 100001;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@media (max-width: 768px) {
		.templates-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.templates-grid {
			grid-template-columns: 1fr;
		}

		.preview-controls {
			left: 1rem;
			right: 1rem;
			transform: none;
			flex-wrap: wrap;
			justify-content: center;
		}
	}
</style>
