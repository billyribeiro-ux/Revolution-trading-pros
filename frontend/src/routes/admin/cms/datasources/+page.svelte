<!--
	/admin/cms/datasources - Storyblok-Style Datasources Management
	Apple Principal Engineer ICT 7 Grade - February 2026

	Features:
	- Full CRUD for datasources (reusable option lists)
	- Entry management with drag-to-reorder
	- CSV import/export functionality
	- Search and filter within entries
	- Dimension support for translations
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { fade, slide } from 'svelte/transition';
	import IconDatabase from '@tabler/icons-svelte/icons/database';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconWorld from '@tabler/icons-svelte/icons/world';
	import { API_BASE_URL } from '$lib/api/config';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	// Types
	interface Datasource {
		id: string;
		name: string;
		slug: string;
		description: string | null;
		icon: string | null;
		color: string | null;
		entry_count: number;
		is_system: boolean;
		is_locked: boolean;
		created_at: string;
		updated_at: string;
	}

	interface DatasourceEntry {
		id: string;
		name: string;
		value: string;
		dimension: string;
		sort_order: number;
		metadata: Record<string, unknown> | null;
	}

	interface PaginationMeta {
		total: number;
		limit: number;
		offset: number;
		has_more: boolean;
	}

	// State
	let datasources = $state<Datasource[]>([]);
	let entries = $state<DatasourceEntry[]>([]);
	let dimensions = $state<string[]>(['default']);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let searchQuery = $state('');
	let entrySearchQuery = $state('');

	// Selected datasource for entry management
	let selectedDatasource = $state<Datasource | null>(null);
	let selectedDimension = $state('default');

	// Pagination
	let pagination = $state<PaginationMeta>({ total: 0, limit: 50, offset: 0, has_more: false });
	let entriesPagination = $state<PaginationMeta>({
		total: 0,
		limit: 100,
		offset: 0,
		has_more: false
	});

	// Modal states
	let showDatasourceModal = $state(false);
	let showEntryModal = $state(false);
	let showImportModal = $state(false);
	let editingDatasource = $state<Datasource | null>(null);
	let editingEntry = $state<DatasourceEntry | null>(null);

	// Form data
	let datasourceForm = $state({
		name: '',
		slug: '',
		description: '',
		icon: '',
		color: '#6366f1'
	});

	let entryForm = $state({
		name: '',
		value: '',
		dimension: 'default'
	});

	// Import state
	let importFile = $state<File | null>(null);

	// Drag state
	let draggedEntryId = $state<string | null>(null);

	// Toast notifications
	let toastMessage = $state('');
	let toastType = $state<'success' | 'error'>('success');
	let showToast = $state(false);

	// Derived
	let filteredDatasources = $derived(
		datasources.filter(
			(ds) =>
				!searchQuery ||
				ds.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				ds.slug.toLowerCase().includes(searchQuery.toLowerCase()) ||
				ds.description?.toLowerCase().includes(searchQuery.toLowerCase())
		)
	);

	let filteredEntries = $derived(
		entries.filter(
			(entry) =>
				!entrySearchQuery ||
				entry.name.toLowerCase().includes(entrySearchQuery.toLowerCase()) ||
				entry.value.toLowerCase().includes(entrySearchQuery.toLowerCase())
		)
	);

	// API Functions
	async function fetchDatasources() {
		isLoading = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch(
				`${API_BASE_URL}/cms/datasources?limit=${pagination.limit}&offset=${pagination.offset}`,
				{
					headers: {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json'
					}
				}
			);

			if (!response.ok) throw new Error('Failed to fetch datasources');

			const data = await response.json();
			datasources = data.data || [];
			pagination = data.meta || pagination;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load datasources';
			showToastMessage(error, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function fetchEntries() {
		if (!selectedDatasource) return;

		isLoading = true;
		try {
			const token = getAuthToken();
			const url = `${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/entries?dimension=${selectedDimension}&limit=${entriesPagination.limit}&offset=${entriesPagination.offset}`;

			if (entrySearchQuery) {
				// Add search parameter
			}

			const response = await fetch(url, {
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				}
			});

			if (!response.ok) throw new Error('Failed to fetch entries');

			const data = await response.json();
			entries = data.data || [];
			entriesPagination = data.meta || entriesPagination;
			dimensions = data.dimensions || ['default'];
		} catch (err) {
			showToastMessage('Failed to load entries', 'error');
		} finally {
			isLoading = false;
		}
	}

	async function saveDatasource() {
		isSaving = true;
		try {
			const token = getAuthToken();
			const method = editingDatasource ? 'PUT' : 'POST';
			const url = editingDatasource
				? `${API_BASE_URL}/cms/datasources/${editingDatasource.id}`
				: `${API_BASE_URL}/cms/datasources`;

			const response = await fetch(url, {
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					name: datasourceForm.name,
					slug: datasourceForm.slug || undefined,
					description: datasourceForm.description || undefined,
					icon: datasourceForm.icon || undefined,
					color: datasourceForm.color || undefined
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				throw new Error(errorData.message || 'Failed to save datasource');
			}

			showToastMessage(
				editingDatasource ? 'Datasource updated successfully' : 'Datasource created successfully',
				'success'
			);
			closeDatasourceModal();
			await fetchDatasources();
		} catch (err) {
			showToastMessage(err instanceof Error ? err.message : 'Failed to save datasource', 'error');
		} finally {
			isSaving = false;
		}
	}

	async function deleteDatasource(id: string) {
		const ds = datasources.find((d) => d.id === id);
		if (!ds) return;

		if (ds.is_system) {
			showToastMessage('Cannot delete system datasources', 'error');
			return;
		}

		if (!confirm(`Delete datasource "${ds.name}"? This will also delete all ${ds.entry_count} entries.`))
			return;

		try {
			const token = getAuthToken();
			const response = await fetch(`${API_BASE_URL}/cms/datasources/${id}`, {
				method: 'DELETE',
				headers: {
					Authorization: `Bearer ${token}`
				}
			});

			if (!response.ok) throw new Error('Failed to delete datasource');

			showToastMessage('Datasource deleted successfully', 'success');
			await fetchDatasources();
		} catch (err) {
			showToastMessage('Failed to delete datasource', 'error');
		}
	}

	async function duplicateDatasource(id: string) {
		try {
			const token = getAuthToken();
			const response = await fetch(`${API_BASE_URL}/cms/datasources/${id}/duplicate`, {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${token}`
				}
			});

			if (!response.ok) throw new Error('Failed to duplicate datasource');

			showToastMessage('Datasource duplicated successfully', 'success');
			await fetchDatasources();
		} catch (err) {
			showToastMessage('Failed to duplicate datasource', 'error');
		}
	}

	async function saveEntry() {
		if (!selectedDatasource) return;

		isSaving = true;
		try {
			const token = getAuthToken();
			const method = editingEntry ? 'PUT' : 'POST';
			const url = editingEntry
				? `${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/entries/${editingEntry.id}`
				: `${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/entries`;

			const response = await fetch(url, {
				method,
				headers: {
					Authorization: `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					name: entryForm.name,
					value: entryForm.value,
					dimension: entryForm.dimension || 'default'
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				throw new Error(errorData.message || 'Failed to save entry');
			}

			showToastMessage(
				editingEntry ? 'Entry updated successfully' : 'Entry created successfully',
				'success'
			);
			closeEntryModal();
			await fetchEntries();
		} catch (err) {
			showToastMessage(err instanceof Error ? err.message : 'Failed to save entry', 'error');
		} finally {
			isSaving = false;
		}
	}

	async function deleteEntry(entryId: string) {
		if (!selectedDatasource) return;

		const entry = entries.find((e) => e.id === entryId);
		if (!confirm(`Delete entry "${entry?.name}"?`)) return;

		try {
			const token = getAuthToken();
			const response = await fetch(
				`${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/entries/${entryId}`,
				{
					method: 'DELETE',
					headers: {
						Authorization: `Bearer ${token}`
					}
				}
			);

			if (!response.ok) throw new Error('Failed to delete entry');

			showToastMessage('Entry deleted successfully', 'success');
			await fetchEntries();
		} catch (err) {
			showToastMessage('Failed to delete entry', 'error');
		}
	}

	async function reorderEntries(newOrder: string[]) {
		if (!selectedDatasource) return;

		try {
			const token = getAuthToken();
			const response = await fetch(
				`${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/entries/reorder`,
				{
					method: 'PUT',
					headers: {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ entry_ids: newOrder })
				}
			);

			if (!response.ok) throw new Error('Failed to reorder entries');

			// Update local state
			const orderedEntries = newOrder
				.map((id) => entries.find((e) => e.id === id))
				.filter((e): e is DatasourceEntry => e !== undefined);
			entries = orderedEntries;
		} catch (err) {
			showToastMessage('Failed to reorder entries', 'error');
			await fetchEntries(); // Refresh to get correct order
		}
	}

	async function exportCsv() {
		if (!selectedDatasource) return;

		try {
			const token = getAuthToken();
			const response = await fetch(
				`${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/export`,
				{
					headers: {
						Authorization: `Bearer ${token}`
					}
				}
			);

			if (!response.ok) throw new Error('Failed to export');

			const blob = await response.blob();
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `${selectedDatasource.slug}-entries.csv`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);

			showToastMessage('CSV exported successfully', 'success');
		} catch (err) {
			showToastMessage('Failed to export CSV', 'error');
		}
	}

	async function importCsv() {
		if (!selectedDatasource || !importFile) return;

		isSaving = true;
		try {
			const token = getAuthToken();
			const formData = new FormData();
			formData.append('file', importFile);

			const response = await fetch(
				`${API_BASE_URL}/cms/datasources/${selectedDatasource.id}/import`,
				{
					method: 'POST',
					headers: {
						Authorization: `Bearer ${token}`
					},
					body: formData
				}
			);

			if (!response.ok) throw new Error('Failed to import');

			const result = await response.json();
			showToastMessage(
				`Import completed: ${result.created} created, ${result.skipped} skipped`,
				'success'
			);
			showImportModal = false;
			importFile = null;
			await fetchEntries();
		} catch (err) {
			showToastMessage('Failed to import CSV', 'error');
		} finally {
			isSaving = false;
		}
	}

	// Helper Functions
	function openDatasourceModal(datasource: Datasource | null = null) {
		if (datasource) {
			editingDatasource = datasource;
			datasourceForm = {
				name: datasource.name,
				slug: datasource.slug,
				description: datasource.description || '',
				icon: datasource.icon || '',
				color: datasource.color || '#6366f1'
			};
		} else {
			editingDatasource = null;
			datasourceForm = {
				name: '',
				slug: '',
				description: '',
				icon: '',
				color: '#6366f1'
			};
		}
		showDatasourceModal = true;
	}

	function closeDatasourceModal() {
		showDatasourceModal = false;
		editingDatasource = null;
	}

	function openEntryModal(entry: DatasourceEntry | null = null) {
		if (entry) {
			editingEntry = entry;
			entryForm = {
				name: entry.name,
				value: entry.value,
				dimension: entry.dimension
			};
		} else {
			editingEntry = null;
			entryForm = {
				name: '',
				value: '',
				dimension: selectedDimension
			};
		}
		showEntryModal = true;
	}

	function closeEntryModal() {
		showEntryModal = false;
		editingEntry = null;
	}

	function selectDatasource(datasource: Datasource) {
		selectedDatasource = datasource;
		selectedDimension = 'default';
		entriesPagination.offset = 0;
		fetchEntries();
	}

	function backToList() {
		selectedDatasource = null;
		entries = [];
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

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			importFile = input.files[0];
		}
	}

	// Drag and drop handlers
	function handleDragStart(event: DragEvent, entryId: string) {
		draggedEntryId = entryId;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDrop(event: DragEvent, targetEntryId: string) {
		event.preventDefault();
		if (!draggedEntryId || draggedEntryId === targetEntryId) {
			draggedEntryId = null;
			return;
		}

		// Reorder entries
		const currentOrder = entries.map((e) => e.id);
		const draggedIndex = currentOrder.indexOf(draggedEntryId);
		const targetIndex = currentOrder.indexOf(targetEntryId);

		currentOrder.splice(draggedIndex, 1);
		currentOrder.splice(targetIndex, 0, draggedEntryId);

		reorderEntries(currentOrder);
		draggedEntryId = null;
	}

	// Auto-generate slug when name changes
	$effect(() => {
		if (datasourceForm.name && !editingDatasource) {
			datasourceForm.slug = generateSlug(datasourceForm.name);
		}
	});

	// Fetch entries when dimension changes
	$effect(() => {
		if (selectedDatasource && selectedDimension) {
			entriesPagination.offset = 0;
			fetchEntries();
		}
	});

	// Lifecycle
	$effect(() => {
		if (browser) {
			fetchDatasources();
		}
	});
</script>

<svelte:head>
	<title>Datasources | CMS Admin</title>
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

<div class="admin-datasources">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		{#if !selectedDatasource}
			<!-- Datasources List View -->
			<header class="page-header">
				<h1>
					<IconDatabase size={28} />
					Datasources
				</h1>
				<p class="subtitle">Reusable option lists for dropdowns and form fields</p>
				<div class="header-actions">
					<button class="btn-refresh" onclick={() => fetchDatasources()} disabled={isLoading}>
						<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
					</button>
					<button class="btn-primary" onclick={() => openDatasourceModal()}>
						<IconPlus size={18} />
						New Datasource
					</button>
				</div>
			</header>

			<!-- Search -->
			<div class="filters-bar">
				<div class="search-box">
					<IconSearch size={18} />
					<input
						type="text"
						id="search-datasources"
						name="search"
						placeholder="Search datasources..."
						bind:value={searchQuery}
					/>
				</div>
			</div>

			<!-- Datasources Grid -->
			<div class="datasources-grid">
				{#if isLoading}
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading datasources...</p>
					</div>
				{:else if error}
					<div class="error-state">
						<IconAlertCircle size={48} />
						<p>{error}</p>
						<button class="btn-primary" onclick={() => fetchDatasources()}>Try Again</button>
					</div>
				{:else if filteredDatasources.length === 0}
					<div class="empty-state">
						<IconDatabase size={48} />
						<h3>No datasources found</h3>
						<p>Create your first datasource to start managing option lists</p>
						<button class="btn-primary" onclick={() => openDatasourceModal()}>
							<IconPlus size={18} />
							Create Datasource
						</button>
					</div>
				{:else}
					{#each filteredDatasources as datasource (datasource.id)}
						<div
							class="datasource-card"
							class:system={datasource.is_system}
							transition:fade
						>
							<div class="card-header">
								<div
									class="card-icon"
									style="background-color: {datasource.color || '#6366f1'}20"
								>
									{#if datasource.is_system}
										<IconWorld size={24} style="color: {datasource.color || '#6366f1'}" />
									{:else}
										<IconDatabase size={24} style="color: {datasource.color || '#6366f1'}" />
									{/if}
								</div>
								<div class="card-badges">
									{#if datasource.is_system}
										<span class="badge badge-system">System</span>
									{/if}
									{#if datasource.is_locked}
										<span class="badge badge-locked"><IconLock size={12} /> Locked</span>
									{/if}
								</div>
							</div>

							<div class="card-body">
								<h3 class="card-title">{datasource.name}</h3>
								<p class="card-slug">/{datasource.slug}</p>
								{#if datasource.description}
									<p class="card-description">{datasource.description}</p>
								{/if}
								<div class="card-stats">
									<span class="stat">
										<strong>{datasource.entry_count}</strong> entries
									</span>
									<span class="stat-sep">|</span>
									<span class="stat">Updated {formatDate(datasource.updated_at)}</span>
								</div>
							</div>

							<div class="card-actions">
								<button
									class="btn-card-action primary"
									onclick={() => selectDatasource(datasource)}
								>
									<IconChevronRight size={16} />
									Manage Entries
								</button>
								<div class="action-buttons">
									<button
										class="btn-icon"
										onclick={() => openDatasourceModal(datasource)}
										title="Edit"
										disabled={datasource.is_system}
									>
										<IconEdit size={16} />
									</button>
									<button
										class="btn-icon"
										onclick={() => duplicateDatasource(datasource.id)}
										title="Duplicate"
									>
										<IconCopy size={16} />
									</button>
									<button
										class="btn-icon danger"
										onclick={() => deleteDatasource(datasource.id)}
										title="Delete"
										disabled={datasource.is_system}
									>
										<IconTrash size={16} />
									</button>
								</div>
							</div>
						</div>
					{/each}
				{/if}
			</div>

			<!-- Pagination -->
			{#if pagination.total > pagination.limit}
				<div class="pagination">
					<span class="pagination-info">
						Showing {pagination.offset + 1} - {Math.min(pagination.offset + pagination.limit, pagination.total)} of {pagination.total}
					</span>
					<div class="pagination-buttons">
						<button
							class="btn-secondary"
							disabled={pagination.offset === 0}
							onclick={() => {
								pagination.offset = Math.max(0, pagination.offset - pagination.limit);
								fetchDatasources();
							}}
						>
							Previous
						</button>
						<button
							class="btn-secondary"
							disabled={!pagination.has_more}
							onclick={() => {
								pagination.offset += pagination.limit;
								fetchDatasources();
							}}
						>
							Next
						</button>
					</div>
				</div>
			{/if}
		{:else}
			<!-- Entries View -->
			<header class="page-header">
				<button class="btn-back" onclick={backToList}>
					<IconArrowLeft size={18} />
					Back to Datasources
				</button>
				<h1>
					<div
						class="header-icon"
						style="background-color: {selectedDatasource.color || '#6366f1'}20"
					>
						<IconDatabase size={24} style="color: {selectedDatasource.color || '#6366f1'}" />
					</div>
					{selectedDatasource.name}
				</h1>
				<p class="subtitle">/{selectedDatasource.slug} - {selectedDatasource.entry_count} entries</p>
				<div class="header-actions">
					<button class="btn-secondary" onclick={exportCsv}>
						<IconDownload size={18} />
						Export CSV
					</button>
					<button class="btn-secondary" onclick={() => (showImportModal = true)}>
						<IconUpload size={18} />
						Import CSV
					</button>
					<button class="btn-primary" onclick={() => openEntryModal()}>
						<IconPlus size={18} />
						Add Entry
					</button>
				</div>
			</header>

			<!-- Filters -->
			<div class="filters-bar">
				<div class="search-box">
					<IconSearch size={18} />
					<input
						type="text"
						id="search-entries"
						name="search"
						placeholder="Search entries..."
						bind:value={entrySearchQuery}
					/>
				</div>
				{#if dimensions.length > 1}
					<select class="dimension-select" bind:value={selectedDimension}>
						{#each dimensions as dim}
							<option value={dim}>{dim === 'default' ? 'Default' : dim}</option>
						{/each}
					</select>
				{/if}
			</div>

			<!-- Entries Table -->
			<div class="entries-container">
				{#if isLoading}
					<div class="loading-state">
						<div class="spinner"></div>
						<p>Loading entries...</p>
					</div>
				{:else if filteredEntries.length === 0}
					<div class="empty-state">
						<IconDatabase size={48} />
						<h3>No entries yet</h3>
						<p>Add your first entry to this datasource</p>
						<button class="btn-primary" onclick={() => openEntryModal()}>
							<IconPlus size={18} />
							Add Entry
						</button>
					</div>
				{:else}
					<div class="entries-table-wrapper">
						<table class="entries-table">
							<thead>
								<tr>
									<th class="th-drag"></th>
									<th>Name (Label)</th>
									<th>Value</th>
									<th>Dimension</th>
									<th class="th-actions">Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each filteredEntries as entry, index (entry.id)}
									<tr
										draggable="true"
										ondragstart={(e) => handleDragStart(e, entry.id)}
										ondragover={handleDragOver}
										ondrop={(e) => handleDrop(e, entry.id)}
										class:dragging={draggedEntryId === entry.id}
									>
										<td class="drag-handle">
											<IconGripVertical size={16} />
										</td>
										<td>
											<span class="entry-name">{entry.name}</span>
										</td>
										<td>
											<code class="entry-value">{entry.value}</code>
										</td>
										<td>
											<span class="dimension-badge">{entry.dimension}</span>
										</td>
										<td>
											<div class="action-buttons">
												<button
													class="btn-icon"
													onclick={() => openEntryModal(entry)}
													title="Edit"
												>
													<IconEdit size={16} />
												</button>
												<button
													class="btn-icon danger"
													onclick={() => deleteEntry(entry.id)}
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

					<div class="table-footer">
						<span class="results-count">
							Showing {filteredEntries.length} of {entriesPagination.total} entries
						</span>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<!-- Datasource Modal -->
{#if showDatasourceModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={closeDatasourceModal}
		onkeydown={(e) => e.key === 'Escape' && closeDatasourceModal()}
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
				<h3>{editingDatasource ? 'Edit Datasource' : 'New Datasource'}</h3>
				<button class="modal-close" onclick={closeDatasourceModal}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-grid">
					<div class="form-group full-width">
						<label for="ds-name">Name *</label>
						<input
							id="ds-name"
							name="ds-name"
							type="text"
							bind:value={datasourceForm.name}
							placeholder="e.g., US States"
							required
						/>
					</div>

					<div class="form-group">
						<label for="ds-slug">Slug *</label>
						<input
							id="ds-slug"
							name="ds-slug"
							type="text"
							bind:value={datasourceForm.slug}
							placeholder="us-states"
							required
						/>
						<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
					</div>

					<div class="form-group">
						<label for="ds-color">Color</label>
						<div class="color-input-wrapper">
							<input id="ds-color" name="ds-color" type="color" bind:value={datasourceForm.color} />
							<span class="color-value">{datasourceForm.color}</span>
						</div>
					</div>

					<div class="form-group full-width">
						<label for="ds-description">Description</label>
						<textarea
							id="ds-description"
							bind:value={datasourceForm.description}
							placeholder="Brief description of this datasource"
							rows="3"
						></textarea>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={closeDatasourceModal} disabled={isSaving}>
					Cancel
				</button>
				<button class="btn-primary" onclick={saveDatasource} disabled={isSaving}>
					{#if isSaving}
						Saving...
					{:else}
						{editingDatasource ? 'Update' : 'Create'} Datasource
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Entry Modal -->
{#if showEntryModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={closeEntryModal}
		onkeydown={(e) => e.key === 'Escape' && closeEntryModal()}
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
				<h3>{editingEntry ? 'Edit Entry' : 'Add Entry'}</h3>
				<button class="modal-close" onclick={closeEntryModal}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-group">
					<label for="entry-name">Name (Label) *</label>
					<input
						id="entry-name"
						name="entry-name"
						type="text"
						bind:value={entryForm.name}
						placeholder="e.g., California"
						required
					/>
					<p class="help-text">The display text shown to users</p>
				</div>

				<div class="form-group">
					<label for="entry-value">Value *</label>
					<input
						id="entry-value"
						name="entry-value"
						type="text"
						bind:value={entryForm.value}
						placeholder="e.g., CA"
						required
					/>
					<p class="help-text">The actual value stored when selected</p>
				</div>

				<div class="form-group">
					<label for="entry-dimension">Dimension</label>
					<select id="entry-dimension" bind:value={entryForm.dimension}>
						<option value="default">Default</option>
						{#each dimensions.filter((d) => d !== 'default') as dim}
							<option value={dim}>{dim}</option>
						{/each}
					</select>
					<p class="help-text">Use dimensions for translations (e.g., en, de, fr)</p>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={closeEntryModal} disabled={isSaving}>
					Cancel
				</button>
				<button class="btn-primary" onclick={saveEntry} disabled={isSaving}>
					{#if isSaving}
						Saving...
					{:else}
						{editingEntry ? 'Update' : 'Add'} Entry
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Import Modal -->
{#if showImportModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showImportModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showImportModal = false)}
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
				<h3>Import CSV</h3>
				<button class="modal-close" onclick={() => (showImportModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<p class="import-info">
					Upload a CSV file with columns: <code>name</code>, <code>value</code>, <code>dimension</code> (optional).
					Existing entries with the same value will be updated.
				</p>

				<div class="form-group">
					<label for="import-file">CSV File</label>
					<input
						id="import-file"
						name="import-file"
						type="file"
						accept=".csv"
						onchange={handleFileSelect}
					/>
				</div>

				{#if importFile}
					<p class="file-info">Selected: {importFile.name}</p>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showImportModal = false)} disabled={isSaving}>
					Cancel
				</button>
				<button class="btn-primary" onclick={importCsv} disabled={isSaving || !importFile}>
					{#if isSaving}
						Importing...
					{:else}
						<IconUpload size={18} />
						Import
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Page Layout */
	.admin-datasources {
		position: relative;
		min-height: 100vh;
	}

	.admin-page-container {
		position: relative;
		z-index: 1;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Background Effects */
	.bg-effects {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		pointer-events: none;
		z-index: 0;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(100px);
		opacity: 0.3;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
		top: -200px;
		right: -200px;
	}

	.bg-blob-2 {
		width: 400px;
		height: 400px;
		background: linear-gradient(135deg, #3b82f6 0%, #06b6d4 100%);
		bottom: -100px;
		left: -100px;
	}

	.bg-blob-3 {
		width: 300px;
		height: 300px;
		background: linear-gradient(135deg, #10b981 0%, #3b82f6 100%);
		top: 40%;
		left: 30%;
	}

	/* Header */
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

	.header-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
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
		flex-wrap: wrap;
	}

	.btn-back {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: var(--text-secondary);
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 1rem;
	}

	.btn-back:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
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

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
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

	.dimension-select {
		padding: 0.75rem 1rem;
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Datasources Grid */
	.datasources-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.datasource-card {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.datasource-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.2);
	}

	.datasource-card.system {
		border-color: rgba(59, 130, 246, 0.3);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.card-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.card-badges {
		display: flex;
		gap: 0.375rem;
	}

	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge-system {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.badge-locked {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.card-body {
		margin-bottom: 1rem;
	}

	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 0.25rem 0;
	}

	.card-slug {
		font-size: 0.75rem;
		font-family: monospace;
		color: #818cf8;
		margin: 0 0 0.5rem 0;
	}

	.card-description {
		font-size: 0.8rem;
		color: var(--text-tertiary);
		margin: 0 0 0.75rem 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-stats {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: var(--text-muted);
	}

	.card-stats strong {
		color: var(--text-secondary);
	}

	.stat-sep {
		color: var(--text-muted);
	}

	.card-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-card-action {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #818cf8;
		font-size: 0.8rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-card-action:hover {
		background: rgba(99, 102, 241, 0.2);
	}

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

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Entries Table */
	.entries-container {
		background: rgba(22, 27, 34, 0.4);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		overflow: hidden;
	}

	.entries-table-wrapper {
		overflow-x: auto;
	}

	.entries-table {
		width: 100%;
		border-collapse: collapse;
	}

	.entries-table th {
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

	.entries-table th.th-drag {
		width: 40px;
	}

	.entries-table th.th-actions {
		width: 100px;
	}

	.entries-table td {
		padding: 14px 16px;
		font-size: 0.875rem;
		color: var(--text-primary);
		border-bottom: 1px solid rgba(99, 102, 241, 0.05);
	}

	.entries-table tbody tr {
		transition: background 0.2s;
	}

	.entries-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.entries-table tbody tr.dragging {
		opacity: 0.5;
		background: rgba(99, 102, 241, 0.1);
	}

	.drag-handle {
		color: var(--text-muted);
		cursor: grab;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	.entry-name {
		font-weight: 500;
	}

	.entry-value {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #818cf8;
	}

	.dimension-badge {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #a78bfa;
	}

	.table-footer {
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.results-count {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1.5rem;
	}

	.pagination-info {
		font-size: 0.875rem;
		color: var(--text-tertiary);
	}

	.pagination-buttons {
		display: flex;
		gap: 0.5rem;
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
		border-radius: 12px;
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
	.form-group input[type='file'],
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

	.help-text {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin: 0;
	}

	.import-info {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	.import-info code {
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.8rem;
		color: #818cf8;
	}

	.file-info {
		font-size: 0.8rem;
		color: var(--text-secondary);
		margin-top: 0.5rem;
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
	@media (max-width: 768px) {
		.datasources-grid {
			grid-template-columns: 1fr;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-group.full-width {
			grid-column: 1;
		}

		.header-actions {
			flex-direction: column;
			width: 100%;
		}

		.header-actions button {
			width: 100%;
			justify-content: center;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			max-width: none;
		}
	}
</style>
