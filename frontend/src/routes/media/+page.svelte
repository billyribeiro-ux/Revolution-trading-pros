<!--
	Media Library Dashboard
	═══════════════════════════════════════════════════════════════════════════
	
	Complete media management interface with uploads, folders, AI metadata,
	optimization, and usage tracking.
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import {
		mediaStore,
		getCurrentFiles,
		getSelectedCount,
		getCurrentViewMode,
		getIsMediaLoading
	} from '$lib/stores/media.svelte';
	import UploadDropzone from '$lib/components/media/UploadDropzone.svelte';
	import FolderTree from '$lib/components/media/FolderTree.svelte';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { Icon, IconFileText, IconList, IconPhoto, IconSearch, IconSparkles, IconTable, IconTrash, IconVideo } from '$lib/icons';

	let showUploadModal = $state(false);
	// TODO: Implement CreateFolderModal component
	// let showCreateFolderModal = false;
	let searchQuery = $state('');
	let selectedFileType = $state('all');

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showDeleteErrorModal = $state(false);

	// Local derived from getters
	const viewMode = $derived(getCurrentViewMode());
	const isLoading = $derived(getIsMediaLoading());
	const currentFiles = $derived(getCurrentFiles());
	const selectedCount = $derived(getSelectedCount());

	onMount(() => {
		mediaStore.initialize();
	});

	function handleFolderSelect(folderId: string | null) {
		mediaStore.setCurrentFolder(folderId);
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / 1024 / 1024).toFixed(1) + ' MB';
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getFileIcon(fileType: string) {
		switch (fileType) {
			case 'image':
				return IconPhoto;
			case 'video':
				return IconVideo;
			default:
				return IconFileText;
		}
	}

	function handleBulkDelete() {
		showDeleteModal = true;
	}

	async function confirmBulkDelete() {
		showDeleteModal = false;

		try {
			await mediaStore.bulkDelete(false);
		} catch (_error) {
			showDeleteErrorModal = true;
		}
	}
</script>

<div class="media-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-title">
			<Icon icon={IconPhoto} size={32} />
			<div>
				<h1 class="page-heading">Media Library</h1>
				<p class="page-subheading">Manage your files, images, and videos</p>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-primary" onclick={() => (showUploadModal = !showUploadModal)}>
				<Icon icon={IconPhoto} size={18} />
				Upload Files
			</button>
		</div>
	</div>

	<!-- Main Content -->
	<div class="dashboard-content">
		<!-- Sidebar -->
		<aside class="sidebar">
			<FolderTree
				currentFolderId={mediaStore.currentFolder}
				onFolderSelect={handleFolderSelect}
				onCreateFolder={() => {
					// TODO: Implement CreateFolderModal
					logger.warn('Create folder modal not yet implemented');
				}}
			/>
		</aside>

		<!-- Main Area -->
		<main class="main-area">
			<!-- Upload Dropzone -->
			{#if showUploadModal}
				<div class="upload-section">
					<UploadDropzone folderId={mediaStore.currentFolder} />
				</div>
			{/if}

			<!-- Toolbar -->
			<div class="toolbar">
				<div class="toolbar-left">
					<!-- Search -->
					<div class="search-box">
						<Icon icon={IconSearch} size={20} class="search-icon" />
						<input
							id="page-searchquery"
							name="page-searchquery"
							type="text"
							bind:value={searchQuery}
							oninput={(e: Event) =>
								mediaStore.setSearchQuery((e.currentTarget as HTMLInputElement).value)}
							placeholder="Search files..."
							class="search-input"
						/>
					</div>

					<!-- Filter -->
					<select
						bind:value={selectedFileType}
						onchange={(e: Event) =>
							mediaStore.setFilterType(
								(e.currentTarget as HTMLSelectElement).value === 'all'
									? null
									: (e.currentTarget as HTMLSelectElement).value
							)}
						class="filter-select"
					>
						<option value="all">All Types</option>
						<option value="image">Images</option>
						<option value="video">Videos</option>
						<option value="document">Documents</option>
					</select>
				</div>

				<div class="toolbar-right">
					<!-- View Mode -->
					<div class="view-toggle">
						<button
							class="view-btn"
							class:active={viewMode === 'grid'}
							onclick={() => mediaStore.setViewMode('grid')}
							aria-label="Grid view"
						>
							<Icon icon={IconTable} size={20} />
						</button>
						<button
							class="view-btn"
							class:active={viewMode === 'list'}
							onclick={() => mediaStore.setViewMode('list')}
							aria-label="List view"
						>
							<Icon icon={IconList} size={20} />
						</button>
					</div>

					<!-- Bulk Actions -->
					{#if selectedCount > 0}
						<div class="bulk-actions">
							<span class="selected-count">{selectedCount} selected</span>
							<button class="bulk-btn" onclick={handleBulkDelete}>
								<Icon icon={IconTrash} size={18} />
								Delete
							</button>
							<button class="bulk-btn" onclick={() => mediaStore.deselectAll()}> Clear </button>
						</div>
					{/if}
				</div>
			</div>

			<!-- Files Grid/List -->
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p class="loading-text">Loading files...</p>
				</div>
			{:else if currentFiles.length > 0}
				{#if viewMode === 'grid'}
					<div class="files-grid">
						{#each currentFiles as file}
							<div
								class="file-card"
								class:selected={mediaStore.selectedFiles.has(file.id)}
								onclick={() => mediaStore.toggleFileSelection(file.id)}
								role="button"
								tabindex="0"
								onkeydown={(e: KeyboardEvent) =>
									e.key === 'Enter' && mediaStore.toggleFileSelection(file.id)}
							>
								{#if file.file_type === 'image'}
									<div class="file-thumbnail">
										<img src={file.thumbnail_url || file.url} alt={file.alt_text || file.title} />
									</div>
								{:else}
									{@const FileIcon = getFileIcon(file.file_type)}
									<div class="file-icon-wrapper">
										<Icon icon={FileIcon} size={48} />
									</div>
								{/if}

								<div class="file-info">
									<div class="file-name">{file.title || file.filename}</div>
									<div class="file-meta">
										{formatFileSize(file.file_size)} • {formatDate(file.created_at)}
									</div>
								</div>

								{#if file.has_webp}
									<div class="file-badge webp">WebP</div>
								{/if}
								{#if file.ai_metadata}
									<div class="file-badge ai">
										<Icon icon={IconSparkles} size={12} />
										AI
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{:else}
					<div class="files-list">
						<table class="files-table">
							<thead>
								<tr>
										<th class="col-checkbox"></th>
									<th>Name</th>
									<th>Type</th>
									<th>Size</th>
									<th>Date</th>
									<th>Tags</th>
								</tr>
							</thead>
							<tbody>
								{#each currentFiles as file}
									<tr
										class:selected={mediaStore.selectedFiles.has(file.id)}
										onclick={() => mediaStore.toggleFileSelection(file.id)}
									>
										<td>
											<input
												id="page-checkbox"
												name="page-checkbox"
												type="checkbox"
												checked={mediaStore.selectedFiles.has(file.id)}
												onchange={() => mediaStore.toggleFileSelection(file.id)}
											/>
										</td>
										<td class="file-name-cell">
											{#if file.file_type === 'image'}
												<img
													src={file.thumbnail_url || file.url}
													alt={file.alt_text}
													class="file-thumb-small"
												/>
											{/if}
											<span>{file.title || file.filename}</span>
										</td>
										<td>{file.file_type}</td>
										<td>{formatFileSize(file.file_size)}</td>
										<td>{formatDate(file.created_at)}</td>
										<td>
											<div class="tags">
												{#each (file.tags || []).slice(0, 3) as tag}
													<span class="tag">{tag}</span>
												{/each}
											</div>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{/if}

				<!-- Pagination -->
				{#if mediaStore.pagination.total_pages > 1}
					<div class="pagination">
						<button
							class="page-btn"
							disabled={mediaStore.pagination.page === 1}
							onclick={() => mediaStore.loadFiles(mediaStore.pagination.page - 1)}
						>
							Previous
						</button>
						<span class="page-info">
							Page {mediaStore.pagination.page} of {mediaStore.pagination.total_pages}
						</span>
						<button
							class="page-btn"
							disabled={mediaStore.pagination.page === mediaStore.pagination.total_pages}
							onclick={() => mediaStore.loadFiles(mediaStore.pagination.page + 1)}
						>
							Next
						</button>
					</div>
				{/if}
			{:else}
				<div class="empty-state">
					<Icon icon={IconPhoto} size={64} />
					<h3 class="empty-title">No files yet</h3>
					<p class="empty-subtitle">Upload your first file to get started</p>
					<button class="btn-primary empty-action" onclick={() => (showUploadModal = true)}>
						<Icon icon={IconPhoto} size={18} />
						Upload Files
					</button>
				</div>
			{/if}
		</main>
	</div>
</div>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Files"
	message={`Delete ${selectedCount} file(s)?`}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => {
		showDeleteModal = false;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteErrorModal}
	title="Delete Failed"
	message="Failed to delete files. Some files may be in use."
	confirmText="OK"
	variant="warning"
	onConfirm={() => {
		showDeleteErrorModal = false;
	}}
	onCancel={() => {
		showDeleteErrorModal = false;
	}}
/>

<style>
	.media-dashboard {
		background: linear-gradient(to bottom right, oklch(0.13 0.02 260), oklch(0.2 0.02 250), oklch(0.13 0.02 260));
		padding: var(--space-6);
	}

	.dashboard-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-8);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		color: oklch(0.8 0.18 90);
	}

	.page-heading {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.page-subheading {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
	}

	.header-actions { display: flex; align-items: center; gap: var(--space-3); }

	.btn-primary {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.8 0.18 90);
		color: oklch(0.15 0.02 90);
		font-weight: var(--weight-semibold);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.85 0.16 90); }
	}

	.dashboard-content {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);
		@media (min-width: 1024px) { grid-template-columns: 300px 1fr; }
	}

	.sidebar {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.main-area {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.upload-section { margin-block-end: var(--space-6); }

	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		flex-wrap: wrap;
	}

	.toolbar-left { display: flex; align-items: center; gap: var(--space-3); flex: 1; }
	.toolbar-right { display: flex; align-items: center; gap: var(--space-3); }

	.search-box {
		position: relative;
		flex: 1;
		max-inline-size: 28rem;
	}

	.search-box :global(.search-icon) {
		position: absolute;
		inset-inline-start: var(--space-3);
		inset-block-start: 50%;
		transform: translateY(-50%);
		color: oklch(0.65 0.01 250);
	}

	.search-input {
		inline-size: 100%;
		padding-inline-start: 2.5rem;
		padding-inline-end: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.25 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.38 0.01 250);
		&:focus { outline: none; box-shadow: 0 0 0 2px oklch(0.8 0.18 90); }
	}

	.filter-select {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.25 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.38 0.01 250);
		&:focus { outline: none; box-shadow: 0 0 0 2px oklch(0.8 0.18 90); }
	}

	.view-toggle {
		display: flex;
		gap: 0.25rem;
		background-color: oklch(0.25 0.01 250);
		border-radius: var(--radius-lg);
		padding: 0.25rem;
		border: 1px solid oklch(0.38 0.01 250);
	}

	.view-btn {
		padding: var(--space-2);
		color: oklch(0.65 0.01 250);
		border-radius: var(--radius-md);
		border: none;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);
		&:hover { color: oklch(1 0 0); background-color: oklch(0.38 0.01 250); }
		&.active { background-color: oklch(0.8 0.18 90); color: oklch(0.15 0.02 90); }
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.25 0.01 250);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.38 0.01 250);
	}

	.selected-count { font-size: var(--text-sm); color: oklch(0.65 0.01 250); }

	.bulk-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding-inline: var(--space-3);
		padding-block: var(--space-1);
		font-size: var(--text-sm);
		color: oklch(1 0 0);
		background-color: oklch(0.38 0.01 250);
		border-radius: var(--radius-sm);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.45 0.01 250); }
	}

	.files-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-4);
		@media (min-width: 768px) { grid-template-columns: repeat(3, 1fr); }
		@media (min-width: 1024px) { grid-template-columns: repeat(4, 1fr); }
		@media (min-width: 1280px) { grid-template-columns: repeat(5, 1fr); }
	}

	.file-card {
		position: relative;
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		overflow: hidden;
		border: 2px solid oklch(0.38 0.01 250 / 50%);
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);
		&:hover { border-color: oklch(0.8 0.18 90 / 50%); }
		&.selected { border-color: oklch(0.8 0.18 90); background-color: oklch(0.8 0.18 90 / 10%); }
	}

	.file-thumbnail {
		aspect-ratio: 1;
		background-color: oklch(0.15 0.01 250);
		overflow: hidden;
	}

	.file-thumbnail img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
	}

	.file-icon-wrapper {
		aspect-ratio: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: oklch(0.15 0.01 250);
		color: oklch(0.65 0.01 250);
	}

	.file-info { padding: var(--space-3); }

	.file-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-meta {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
	}

	.file-badge {
		position: absolute;
		inset-block-start: var(--space-2);
		inset-inline-end: var(--space-2);
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		border-radius: var(--radius-sm);

		&.webp { background-color: oklch(0.6 0.18 160 / 20%); color: oklch(0.7 0.18 160); }
		&.ai {
			background-color: oklch(0.55 0.2 300 / 20%);
			color: oklch(0.7 0.18 300);
			display: flex;
			align-items: center;
			gap: 0.25rem;
		}
	}

	.files-list {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		overflow: hidden;
	}

	.files-table { inline-size: 100%; border-collapse: collapse; }

	.files-table thead { background-color: oklch(0.15 0.01 250 / 50%); }

	.files-table th {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		text-align: start;
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.65 0.01 250);
	}

	.col-checkbox { inline-size: 3rem; }

	.files-table tbody tr {
		border-block-start: 1px solid oklch(0.38 0.01 250 / 50%);
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.38 0.01 250 / 30%); }
		&.selected { background-color: oklch(0.8 0.18 90 / 10%); }
	}

	.files-table td {
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		font-size: var(--text-sm);
		color: oklch(0.75 0.01 250);
	}

	.file-name-cell {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.file-thumb-small {
		inline-size: 2.5rem;
		block-size: 2.5rem;
		object-fit: cover;
		border-radius: var(--radius-sm);
	}

	.tags { display: flex; gap: 0.25rem; flex-wrap: wrap; }

	.tag {
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		background-color: oklch(0.38 0.01 250);
		color: oklch(0.75 0.01 250);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		margin-block-start: var(--space-6);
	}

	.page-btn {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.25 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.38 0.01 250);
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover { background-color: oklch(0.38 0.01 250); }
		&:disabled { opacity: 0.5; cursor: not-allowed; }
	}

	.page-info { color: oklch(0.65 0.01 250); }

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
		color: oklch(0.45 0.01 250);
	}

	.loading-text { color: oklch(0.65 0.01 250); margin-block-start: var(--space-4); }

	.empty-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-4);
	}

	.empty-subtitle {
		color: oklch(0.55 0.01 250);
		margin-block-start: var(--space-2);
	}

	.empty-action { margin-block-start: var(--space-4); }

	.spinner {
		inline-size: 3rem;
		block-size: 3rem;
		border: 4px solid oklch(0.38 0.01 250);
		border-block-start-color: oklch(0.8 0.18 90);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin { to { transform: rotate(360deg); } }
</style>
