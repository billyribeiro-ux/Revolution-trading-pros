<!--
/**
 * Global Component Library - Storyblok-Style Component Browser
 * ===============================================================================
 * Modal/sidebar browser for managing global components (headers, footers, CTAs, etc.)
 * Enables managing repeated elements from one centralized place.
 *
 * Features:
 * - Search and filter by category
 * - Preview thumbnails
 * - Insert into editor
 * - Edit global component inline
 * - Show usage count
 * - Sync/detach functionality
 * - Version history
 *
 * @version 1.0.0 - February 2026
 */
-->

<script lang="ts">
	import { fade, slide, fly } from 'svelte/transition';
	import {
		IconLayoutRows,
		IconClick,
		IconForms,
		IconMenu2,
		IconLayoutKanban,
		IconFlag,
		IconMaximize,
		IconId,
		IconLayoutDashboard,
		IconSearch,
		IconPlus,
		IconEdit,
		IconTrash,
		IconCopy,
		IconHistory,
		IconLink,
		IconRefresh,
		IconCheck,
		IconX,
		IconLoader,
		IconAlertCircle,
		IconFilter
	} from '$lib/icons';

	import { API_BASE_URL } from '$lib/api/config';
	import { getAuthToken } from '$lib/stores/auth.svelte';
	import type { Block } from './types';

	// ==========================================================================
	// Types
	// ==========================================================================

	type ComponentCategory =
		| 'header'
		| 'footer'
		| 'cta'
		| 'form'
		| 'navigation'
		| 'sidebar'
		| 'banner'
		| 'modal'
		| 'card'
		| 'section';

	interface GlobalComponent {
		id: string;
		name: string;
		slug: string;
		description: string | null;
		component_data: Block[];
		category: ComponentCategory;
		tags: string[] | null;
		thumbnail_url: string | null;
		usage_count: number;
		is_global: boolean;
		is_locked: boolean;
		version: number;
		created_at: string;
		updated_at: string;
	}

	interface ComponentVersion {
		id: string;
		component_id: string;
		version: number;
		component_data: Block[];
		change_message: string | null;
		created_by_name: string | null;
		created_at: string;
	}

	interface ComponentUsage {
		id: string;
		component_id: string;
		content_id: string;
		content_title: string;
		content_type: string;
		content_slug: string;
		instance_id: string;
		is_synced: boolean;
		synced_version: number;
		needs_update: boolean;
		created_at: string;
	}

	interface CategoryCount {
		category: string;
		count: number;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		isOpen: boolean;
		contentId?: string;
		oninsert: (blocks: Block[], componentId: string, instanceId: string) => void;
		onclose: () => void;
	}

	let props: Props = $props();
	const isOpen = $derived(props.isOpen);
	const contentId = $derived(props.contentId);
	const oninsert = $derived(props.oninsert);
	const onclose = $derived(props.onclose);

	// ==========================================================================
	// State
	// ==========================================================================

	let components = $state<GlobalComponent[]>([]);
	let categories = $state<CategoryCount[]>([]);
	let selectedCategory = $state<string | null>(null);
	let searchQuery = $state('');
	let sortBy = $state<'name' | 'created_at' | 'updated_at' | 'usage_count'>('updated_at');
	let sortOrder = $state<'asc' | 'desc'>('desc');

	let isLoading = $state(false);
	let error = $state<string | null>(null);

	let selectedComponent = $state<GlobalComponent | null>(null);
	let showCreateModal = $state(false);
	let showVersionHistory = $state(false);
	let showUsagePanel = $state(false);

	let versions = $state<ComponentVersion[]>([]);
	let usages = $state<ComponentUsage[]>([]);

	// Create/Edit form state
	let formName = $state('');
	let formDescription = $state('');
	let formCategory = $state<ComponentCategory>('section');
	let formTags = $state('');
	let formIsGlobal = $state(true);
	let isSubmitting = $state(false);
	let editMode = $state(false);

	// ==========================================================================
	// Category Config
	// ==========================================================================

	const categoryConfig: Record<
		ComponentCategory,
		{ label: string; icon: typeof IconLayoutRows; color: string }
	> = {
		header: { label: 'Headers', icon: IconLayoutRows, color: '#3b82f6' },
		footer: { label: 'Footers', icon: IconLayoutRows, color: '#10b981' },
		cta: { label: 'CTAs', icon: IconClick, color: '#f59e0b' },
		form: { label: 'Forms', icon: IconForms, color: '#8b5cf6' },
		navigation: { label: 'Navigation', icon: IconMenu2, color: '#ec4899' },
		sidebar: { label: 'Sidebars', icon: IconLayoutKanban, color: '#06b6d4' },
		banner: { label: 'Banners', icon: IconFlag, color: '#ef4444' },
		modal: { label: 'Modals', icon: IconMaximize, color: '#84cc16' },
		card: { label: 'Cards', icon: IconId, color: '#f97316' },
		section: { label: 'Sections', icon: IconLayoutDashboard, color: '#64748b' }
	};

	// ==========================================================================
	// Derived
	// ==========================================================================

	let filteredComponents = $derived(() => {
		let result = components;

		if (selectedCategory) {
			result = result.filter((c) => c.category === selectedCategory);
		}

		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(c) =>
					c.name.toLowerCase().includes(query) ||
					c.description?.toLowerCase().includes(query) ||
					c.tags?.some((t) => t.toLowerCase().includes(query))
			);
		}

		return result;
	});

	// ==========================================================================
	// API Helpers
	// ==========================================================================

	function getAuthHeaders(): HeadersInit {
		const token = getAuthToken();
		return {
			'Content-Type': 'application/json',
			...(token && { Authorization: `Bearer ${token}` })
		};
	}

	// ==========================================================================
	// Data Loading
	// ==========================================================================

	async function loadComponents(): Promise<void> {
		isLoading = true;
		error = null;

		try {
			const params = new URLSearchParams({
				sort_by: sortBy,
				sort_order: sortOrder,
				limit: '100'
			});

			if (selectedCategory) {
				params.set('category', selectedCategory);
			}

			if (searchQuery.trim()) {
				params.set('search', searchQuery);
			}

			const response = await fetch(`${API_BASE_URL}/api/cms/global-components?${params}`, {
				headers: getAuthHeaders(),
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error(`Failed to load components: ${response.status}`);
			}

			const data = await response.json();
			components = data.data || [];
			categories = data.categories || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load components';
			console.error('Load components error:', err);
		} finally {
			isLoading = false;
		}
	}

	async function loadVersionHistory(componentId: string): Promise<void> {
		try {
			const response = await fetch(
				`${API_BASE_URL}/api/cms/global-components/${componentId}/versions`,
				{
					headers: getAuthHeaders(),
					credentials: 'include'
				}
			);

			if (!response.ok) {
				throw new Error('Failed to load version history');
			}

			const data = await response.json();
			versions = data.versions || [];
		} catch (err) {
			console.error('Load versions error:', err);
		}
	}

	async function loadUsages(componentId: string): Promise<void> {
		try {
			const response = await fetch(
				`${API_BASE_URL}/api/cms/global-components/${componentId}/usage`,
				{
					headers: getAuthHeaders(),
					credentials: 'include'
				}
			);

			if (!response.ok) {
				throw new Error('Failed to load usage data');
			}

			const data = await response.json();
			usages = data.usages || [];
		} catch (err) {
			console.error('Load usages error:', err);
		}
	}

	// ==========================================================================
	// Actions
	// ==========================================================================

	async function handleInsert(component: GlobalComponent): Promise<void> {
		if (!contentId) {
			// Just insert without tracking if no content context
			const instanceId = `gc_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
			oninsert(component.component_data, component.id, instanceId);
			onclose();
			return;
		}

		try {
			const instanceId = `gc_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;

			// Track usage
			await fetch(`${API_BASE_URL}/api/cms/global-components/usage`, {
				method: 'POST',
				headers: getAuthHeaders(),
				credentials: 'include',
				body: JSON.stringify({
					component_id: component.id,
					content_id: contentId,
					instance_id: instanceId,
					is_synced: true
				})
			});

			oninsert(component.component_data, component.id, instanceId);
			onclose();
		} catch (err) {
			console.error('Insert component error:', err);
			// Still insert even if tracking fails
			const instanceId = `gc_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`;
			oninsert(component.component_data, component.id, instanceId);
			onclose();
		}
	}

	async function handleDuplicate(component: GlobalComponent): Promise<void> {
		try {
			const response = await fetch(
				`${API_BASE_URL}/api/cms/global-components/${component.id}/duplicate`,
				{
					method: 'POST',
					headers: getAuthHeaders(),
					credentials: 'include'
				}
			);

			if (!response.ok) {
				throw new Error('Failed to duplicate component');
			}

			await loadComponents();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate';
		}
	}

	async function handleDelete(component: GlobalComponent): Promise<void> {
		if (!confirm(`Delete "${component.name}"? This cannot be undone.`)) {
			return;
		}

		try {
			const response = await fetch(`${API_BASE_URL}/api/cms/global-components/${component.id}`, {
				method: 'DELETE',
				headers: getAuthHeaders(),
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to delete component');
			}

			await loadComponents();
			selectedComponent = null;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete';
		}
	}

	async function handleRestoreVersion(versionNum: number): Promise<void> {
		if (!selectedComponent) return;

		try {
			const response = await fetch(
				`${API_BASE_URL}/api/cms/global-components/${selectedComponent.id}/versions/${versionNum}/restore`,
				{
					method: 'POST',
					headers: getAuthHeaders(),
					credentials: 'include'
				}
			);

			if (!response.ok) {
				throw new Error('Failed to restore version');
			}

			await loadComponents();
			await loadVersionHistory(selectedComponent.id);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to restore version';
		}
	}

	function openCreateModal(): void {
		formName = '';
		formDescription = '';
		formCategory = 'section';
		formTags = '';
		formIsGlobal = true;
		editMode = false;
		showCreateModal = true;
	}

	function openEditModal(component: GlobalComponent): void {
		formName = component.name;
		formDescription = component.description || '';
		formCategory = component.category;
		formTags = component.tags?.join(', ') || '';
		formIsGlobal = component.is_global;
		editMode = true;
		selectedComponent = component;
		showCreateModal = true;
	}

	async function handleSubmitForm(): Promise<void> {
		if (!formName.trim()) {
			error = 'Name is required';
			return;
		}

		isSubmitting = true;
		error = null;

		try {
			const body = {
				name: formName.trim(),
				description: formDescription.trim() || null,
				category: formCategory,
				tags: formTags
					.split(',')
					.map((t) => t.trim())
					.filter(Boolean),
				is_global: formIsGlobal,
				component_data: editMode ? undefined : []
			};

			const url = editMode
				? `${API_BASE_URL}/api/cms/global-components/${selectedComponent?.id}`
				: `${API_BASE_URL}/api/cms/global-components`;

			const response = await fetch(url, {
				method: editMode ? 'PUT' : 'POST',
				headers: getAuthHeaders(),
				credentials: 'include',
				body: JSON.stringify(body)
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.message || 'Failed to save component');
			}

			showCreateModal = false;
			await loadComponents();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save';
		} finally {
			isSubmitting = false;
		}
	}

	function selectComponent(component: GlobalComponent): void {
		selectedComponent = component;
		showVersionHistory = false;
		showUsagePanel = false;
	}

	function toggleVersionHistory(): void {
		if (!selectedComponent) return;
		showVersionHistory = !showVersionHistory;
		showUsagePanel = false;
		if (showVersionHistory) {
			loadVersionHistory(selectedComponent.id);
		}
	}

	function toggleUsagePanel(): void {
		if (!selectedComponent) return;
		showUsagePanel = !showUsagePanel;
		showVersionHistory = false;
		if (showUsagePanel) {
			loadUsages(selectedComponent.id);
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// ==========================================================================
	// Effects
	// ==========================================================================

	$effect(() => {
		if (isOpen) {
			loadComponents();
		}
	});

	$effect(() => {
		// Reload when filters change
		if (isOpen) {
			loadComponents();
		}
	});
</script>

{#if isOpen}
	<div class="global-component-library" transition:fade={{ duration: 150 }}>
		<div
		class="library-backdrop"
		onclick={onclose}
		onkeydown={(e) => e.key === 'Escape' && onclose()}
		role="button"
		tabindex="-1"
		aria-label="Close library"
	></div>

		<div class="library-modal" transition:fly={{ x: 50, duration: 200 }}>
			<!-- Header -->
			<div class="library-header">
				<div class="header-title">
					<IconLayoutDashboard size={24} />
					<div>
						<h2>Global Components</h2>
						<span class="subtitle">Reusable headers, footers, CTAs & more</span>
					</div>
				</div>
				<div class="header-actions">
					<button type="button" class="btn-create" onclick={openCreateModal}>
						<IconPlus size={18} />
						New Component
					</button>
					<button type="button" class="btn-close" onclick={onclose} title="Close">
						<IconX size={20} />
					</button>
				</div>
			</div>

			<div class="library-body">
				<!-- Sidebar -->
				<div class="library-sidebar">
					<!-- Search -->
					<div class="search-box">
						<IconSearch size={18} />
						<input
							type="text"
							bind:value={searchQuery}
							placeholder="Search components..."
							oninput={() => loadComponents()}
						/>
					</div>

					<!-- Categories -->
					<div class="categories-list">
						<button
							type="button"
							class="category-item"
							class:active={selectedCategory === null}
							onclick={() => {
								selectedCategory = null;
								loadComponents();
							}}
						>
							<span class="category-icon" style="background-color: #94a3b8">
								<IconFilter size={16} />
							</span>
							<span class="category-label">All Components</span>
							<span class="category-count">{components.length}</span>
						</button>

						{#each Object.entries(categoryConfig) as [key, config]}
							{@const count = categories.find((c) => c.category === key)?.count || 0}
							{@const Icon = config.icon}
							<button
								type="button"
								class="category-item"
								class:active={selectedCategory === key}
								onclick={() => {
									selectedCategory = key;
									loadComponents();
								}}
							>
								<span class="category-icon" style="background-color: {config.color}">
									<Icon size={16} />
								</span>
								<span class="category-label">{config.label}</span>
								<span class="category-count">{count}</span>
							</button>
						{/each}
					</div>

					<!-- Sort -->
					<div class="sort-controls">
						<label for="sort-select">Sort by:</label>
						<select
							id="sort-select"
							bind:value={sortBy}
							onchange={() => loadComponents()}
						>
							<option value="updated_at">Recently Updated</option>
							<option value="created_at">Recently Created</option>
							<option value="name">Name</option>
							<option value="usage_count">Most Used</option>
						</select>
					</div>
				</div>

				<!-- Main Content -->
				<div class="library-content">
					{#if isLoading}
						<div class="loading-state">
							<IconLoader size={32} class="spin" />
							<span>Loading components...</span>
						</div>
					{:else if error}
						<div class="error-state">
							<IconAlertCircle size={32} />
							<span>{error}</span>
							<button type="button" onclick={() => loadComponents()}>Try Again</button>
						</div>
					{:else if filteredComponents().length === 0}
						<div class="empty-state">
							<IconLayoutDashboard size={48} />
							<h3>No components found</h3>
							<p>Create your first global component to get started.</p>
							<button type="button" class="btn-create" onclick={openCreateModal}>
								<IconPlus size={18} />
								Create Component
							</button>
						</div>
					{:else}
						<div class="components-grid">
							{#each filteredComponents() as component (component.id)}
								{@const config = categoryConfig[component.category]}
								{@const Icon = config.icon}
								<div
									class="component-card"
									class:selected={selectedComponent?.id === component.id}
									onclick={() => selectComponent(component)}
									onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectComponent(component)}
									role="button"
									tabindex="0"
								>
									<div class="card-thumbnail">
										{#if component.thumbnail_url}
											<img src={component.thumbnail_url} alt={component.name} />
										{:else}
											<div
												class="thumbnail-placeholder"
												style="background-color: {config.color}20"
											>
												<Icon size={32} />
											</div>
										{/if}
										{#if component.is_locked}
											<span class="locked-badge" title="Locked">Locked</span>
										{/if}
									</div>

									<div class="card-body">
										<div class="card-category" style="color: {config.color}">
											<Icon size={14} />
											{config.label}
										</div>
										<h4 class="card-title">{component.name}</h4>
										{#if component.description}
											<p class="card-description">{component.description}</p>
										{/if}
									</div>

									<div class="card-footer">
										<span class="usage-count" title="{component.usage_count} usages">
											<IconLink size={14} />
											{component.usage_count}
										</span>
										<span class="version-badge">v{component.version}</span>
									</div>

									<div class="card-actions">
										<button
											type="button"
											class="action-btn primary"
											onclick={(e) => { e.stopPropagation(); handleInsert(component); }}
											title="Insert into editor"
										>
											<IconPlus size={16} />
										</button>
										<button
											type="button"
											class="action-btn"
											onclick={(e) => { e.stopPropagation(); openEditModal(component); }}
											title="Edit"
										>
											<IconEdit size={16} />
										</button>
										<button
											type="button"
											class="action-btn"
											onclick={(e) => { e.stopPropagation(); handleDuplicate(component); }}
											title="Duplicate"
										>
											<IconCopy size={16} />
										</button>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Detail Panel -->
				{#if selectedComponent}
					<div class="detail-panel" transition:slide={{ axis: 'x', duration: 200 }}>
						<div class="detail-header">
							<h3>{selectedComponent.name}</h3>
							<div class="detail-actions">
								<button
									type="button"
									class="detail-btn"
									class:active={showVersionHistory}
									onclick={toggleVersionHistory}
									title="Version History"
								>
									<IconHistory size={18} />
								</button>
								<button
									type="button"
									class="detail-btn"
									class:active={showUsagePanel}
									onclick={toggleUsagePanel}
									title="Usage ({selectedComponent.usage_count})"
								>
									<IconLink size={18} />
									<span class="badge">{selectedComponent.usage_count}</span>
								</button>
								<button
									type="button"
									class="detail-btn danger"
									onclick={() => handleDelete(selectedComponent!)}
									title="Delete"
									disabled={selectedComponent.is_locked}
								>
									<IconTrash size={18} />
								</button>
							</div>
						</div>

						<div class="detail-content">
							{#if showVersionHistory}
								<div class="version-history" transition:slide>
									<h4>
										<IconHistory size={16} />
										Version History
									</h4>
									{#if versions.length === 0}
										<p class="empty-message">No version history available.</p>
									{:else}
										<div class="versions-list">
											{#each versions as version (version.id)}
												<div
													class="version-item"
													class:current={version.version === selectedComponent.version}
												>
													<div class="version-info">
														<span class="version-number">v{version.version}</span>
														<span class="version-date"
															>{formatDate(version.created_at)}</span
														>
													</div>
													<div class="version-meta">
														{#if version.change_message}
															<p class="change-message">{version.change_message}</p>
														{/if}
														{#if version.created_by_name}
															<span class="author">by {version.created_by_name}</span>
														{/if}
													</div>
													{#if version.version !== selectedComponent.version}
														<button
															type="button"
															class="restore-btn"
															onclick={() => handleRestoreVersion(version.version)}
														>
															<IconRefresh size={14} />
															Restore
														</button>
													{:else}
														<span class="current-badge">Current</span>
													{/if}
												</div>
											{/each}
										</div>
									{/if}
								</div>
							{:else if showUsagePanel}
								<div class="usage-panel" transition:slide>
									<h4>
										<IconLink size={16} />
										Used in {usages.length} places
									</h4>
									{#if usages.length === 0}
										<p class="empty-message">This component is not used anywhere yet.</p>
									{:else}
										<div class="usages-list">
											{#each usages as usage (usage.id)}
												<div class="usage-item">
													<div class="usage-info">
														<span class="content-title">{usage.content_title}</span>
														<span class="content-type">{usage.content_type}</span>
													</div>
													<div class="usage-status">
														{#if usage.is_synced}
															{#if usage.needs_update}
																<span class="status-badge outdated">
																	<IconAlertCircle size={12} />
																	Needs Update
																</span>
															{:else}
																<span class="status-badge synced">
																	<IconCheck size={12} />
																	Synced
																</span>
															{/if}
														{:else}
															<span class="status-badge detached">
																<IconLink size={12} />
																Detached
															</span>
														{/if}
													</div>
												</div>
											{/each}
										</div>
									{/if}
								</div>
							{:else}
								<div class="component-preview">
									<div class="preview-info">
										<p class="description">
											{selectedComponent.description || 'No description provided.'}
										</p>

										<div class="meta-grid">
											<div class="meta-item">
												<span class="meta-label">Category</span>
												<span class="meta-value">
													{categoryConfig[selectedComponent.category].label}
												</span>
											</div>
											<div class="meta-item">
												<span class="meta-label">Version</span>
												<span class="meta-value">v{selectedComponent.version}</span>
											</div>
											<div class="meta-item">
												<span class="meta-label">Created</span>
												<span class="meta-value"
													>{formatDate(selectedComponent.created_at)}</span
												>
											</div>
											<div class="meta-item">
												<span class="meta-label">Updated</span>
												<span class="meta-value"
													>{formatDate(selectedComponent.updated_at)}</span
												>
											</div>
										</div>

										{#if selectedComponent.tags && selectedComponent.tags.length > 0}
											<div class="tags-list">
												{#each selectedComponent.tags as tag}
													<span class="tag">{tag}</span>
												{/each}
											</div>
										{/if}
									</div>

									<button
										type="button"
										class="insert-btn"
										onclick={() => handleInsert(selectedComponent!)}
									>
										<IconPlus size={18} />
										Insert into Editor
									</button>
								</div>
							{/if}
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Create/Edit Modal -->
{#if showCreateModal}
	<div class="modal-overlay" transition:fade={{ duration: 150 }}>
		<div class="modal-dialog" transition:fly={{ y: -20, duration: 200 }}>
			<div class="modal-header">
				<h3>{editMode ? 'Edit Component' : 'Create Component'}</h3>
				<button type="button" class="modal-close" onclick={() => (showCreateModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<form class="modal-body" onsubmit={(e) => { e.preventDefault(); handleSubmitForm(); }}>
				<div class="form-group">
					<label for="comp-name">Name *</label>
					<input
						id="comp-name"
						type="text"
						bind:value={formName}
						placeholder="e.g., Main Header, Footer CTA"
						required
					/>
				</div>

				<div class="form-group">
					<label for="comp-description">Description</label>
					<textarea
						id="comp-description"
						bind:value={formDescription}
						placeholder="Brief description of this component"
						rows="2"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="comp-category">Category *</label>
						<select id="comp-category" bind:value={formCategory}>
							{#each Object.entries(categoryConfig) as [key, config]}
								<option value={key}>{config.label}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="comp-tags">Tags</label>
						<input
							id="comp-tags"
							type="text"
							bind:value={formTags}
							placeholder="tag1, tag2, tag3"
						/>
					</div>
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={formIsGlobal} />
						<span>Make globally available</span>
					</label>
				</div>

				{#if error}
					<div class="form-error">
						<IconAlertCircle size={16} />
						{error}
					</div>
				{/if}

				<div class="modal-actions">
					<button
						type="button"
						class="btn-secondary"
						onclick={() => (showCreateModal = false)}
						disabled={isSubmitting}
					>
						Cancel
					</button>
					<button type="submit" class="btn-primary" disabled={isSubmitting}>
						{#if isSubmitting}
							<IconLoader size={16} class="spin" />
						{/if}
						{editMode ? 'Save Changes' : 'Create Component'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.global-component-library {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		justify-content: flex-end;
	}

	.library-backdrop {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
	}

	.library-modal {
		position: relative;
		width: 100%;
		max-width: 1200px;
		height: 100%;
		background: white;
		display: flex;
		flex-direction: column;
		box-shadow: -4px 0 24px rgba(0, 0, 0, 0.15);
	}

	/* Header */
	.library-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #1f2937;
	}

	.header-title h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.subtitle {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-create {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s;
	}

	.btn-create:hover {
		background: #2563eb;
	}

	.btn-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-close:hover {
		background: #e5e7eb;
		color: #1f2937;
	}

	/* Body Layout */
	.library-body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	/* Sidebar */
	.library-sidebar {
		width: 240px;
		border-right: 1px solid #e5e7eb;
		background: #f9fafb;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		gap: 1rem;
		overflow-y: auto;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.875rem;
	}

	.search-box :global(svg) {
		color: #9ca3af;
	}

	.categories-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.category-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.5rem 0.625rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
	}

	.category-item:hover {
		background: white;
	}

	.category-item.active {
		background: white;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.category-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border-radius: 6px;
		color: white;
	}

	.category-label {
		flex: 1;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.category-count {
		font-size: 0.75rem;
		color: #9ca3af;
		padding: 0.125rem 0.375rem;
		background: #f3f4f6;
		border-radius: 4px;
	}

	.sort-controls {
		margin-top: auto;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	.sort-controls label {
		display: block;
		font-size: 0.6875rem;
		font-weight: 500;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.375rem;
	}

	.sort-controls select {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.8125rem;
		background: white;
		cursor: pointer;
	}

	/* Content */
	.library-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: 1rem;
		color: #6b7280;
		text-align: center;
	}

	.loading-state :global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state h3 {
		margin: 0;
		font-size: 1.125rem;
		color: #374151;
	}

	.empty-state p {
		margin: 0;
	}

	.components-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
		gap: 1rem;
	}

	.component-card {
		position: relative;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.15s;
	}

	.component-card:hover {
		border-color: #3b82f6;
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
	}

	.component-card.selected {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	.card-thumbnail {
		position: relative;
		height: 100px;
		background: #f9fafb;
		overflow: hidden;
	}

	.card-thumbnail img {
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
		color: #6b7280;
	}

	.locked-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.6);
		color: white;
		font-size: 0.625rem;
		font-weight: 500;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.card-body {
		padding: 0.75rem;
	}

	.card-category {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.6875rem;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.card-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.card-description {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	.usage-count {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.version-badge {
		font-size: 0.6875rem;
		font-weight: 500;
		color: #6b7280;
		padding: 0.125rem 0.375rem;
		background: #e5e7eb;
		border-radius: 4px;
	}

	.card-actions {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		display: flex;
		gap: 0.25rem;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.component-card:hover .card-actions {
		opacity: 1;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.action-btn.primary {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.action-btn.primary:hover {
		background: #2563eb;
	}

	/* Detail Panel */
	.detail-panel {
		width: 320px;
		border-left: 1px solid #e5e7eb;
		background: white;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.detail-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.detail-header h3 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.detail-actions {
		display: flex;
		gap: 0.25rem;
	}

	.detail-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.detail-btn:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.detail-btn.active {
		background: #eff6ff;
		color: #3b82f6;
	}

	.detail-btn.danger:hover {
		background: #fef2f2;
		color: #dc2626;
	}

	.detail-btn .badge {
		position: absolute;
		top: 2px;
		right: 2px;
		min-width: 14px;
		height: 14px;
		padding: 0 3px;
		background: #ef4444;
		color: white;
		font-size: 0.625rem;
		font-weight: 600;
		border-radius: 7px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.detail-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.component-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.preview-info {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.description {
		font-size: 0.875rem;
		color: #6b7280;
		line-height: 1.5;
		margin: 0;
	}

	.meta-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.meta-item {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.meta-label {
		font-size: 0.6875rem;
		font-weight: 500;
		color: #9ca3af;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.meta-value {
		font-size: 0.8125rem;
		color: #374151;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.tag {
		padding: 0.25rem 0.5rem;
		background: #f3f4f6;
		border-radius: 4px;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.insert-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s;
		margin-top: auto;
	}

	.insert-btn:hover {
		background: #059669;
	}

	/* Version History */
	.version-history h4,
	.usage-panel h4 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 0.75rem;
	}

	.empty-message {
		font-size: 0.8125rem;
		color: #9ca3af;
		text-align: center;
		padding: 1rem;
	}

	.versions-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.version-item {
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 6px;
		border: 1px solid #e5e7eb;
	}

	.version-item.current {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.version-info {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.25rem;
	}

	.version-number {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #1f2937;
	}

	.version-date {
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	.version-meta {
		margin-bottom: 0.5rem;
	}

	.change-message {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0 0 0.25rem;
	}

	.author {
		font-size: 0.6875rem;
		color: #9ca3af;
	}

	.restore-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-size: 0.6875rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.restore-btn:hover {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.current-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: #3b82f6;
		color: white;
		font-size: 0.6875rem;
		font-weight: 500;
		border-radius: 4px;
	}

	/* Usage Panel */
	.usages-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.usage-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.625rem 0.75rem;
		background: #f9fafb;
		border-radius: 6px;
	}

	.usage-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.content-title {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.content-type {
		font-size: 0.6875rem;
		color: #9ca3af;
		text-transform: capitalize;
	}

	.status-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 500;
	}

	.status-badge.synced {
		background: #d1fae5;
		color: #059669;
	}

	.status-badge.outdated {
		background: #fef3c7;
		color: #d97706;
	}

	.status-badge.detached {
		background: #f3f4f6;
		color: #6b7280;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		z-index: 1100;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.5);
		padding: 1rem;
	}

	.modal-dialog {
		width: 100%;
		max-width: 480px;
		background: white;
		border-radius: 12px;
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.modal-header h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0;
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		cursor: pointer;
	}

	.modal-close:hover {
		background: #f3f4f6;
		color: #1f2937;
	}

	.modal-body {
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.form-group label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		padding: 0.625rem 0.75rem;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.875rem;
		outline: none;
		transition: border-color 0.15s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		border-color: #3b82f6;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.checkbox-label input {
		width: 16px;
		height: 16px;
	}

	.checkbox-label span {
		font-size: 0.875rem;
		color: #374151;
	}

	.form-error {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 6px;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 0.5rem;
	}

	.btn-secondary,
	.btn-primary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary {
		background: white;
		border: 1px solid #e5e7eb;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	.btn-primary {
		background: #3b82f6;
		border: 1px solid #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-primary:disabled,
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-primary :global(.spin) {
		animation: spin 1s linear infinite;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.library-modal {
			max-width: 100%;
		}

		.detail-panel {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.library-sidebar {
			width: 200px;
		}

		.components-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
