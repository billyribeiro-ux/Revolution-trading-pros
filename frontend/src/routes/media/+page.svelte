<!--
	Media Library Dashboard
	═══════════════════════════════════════════════════════════════════════════
	
	Complete media management interface with uploads, folders, AI metadata,
	optimization, and usage tracking.
-->

<script lang="ts">
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
	import {
		IconPhoto,
		IconVideo,
		IconFileText,
		IconSearch,
		IconTable,
		IconList,
		IconTrash,
		IconSparkles
	} from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

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

<svelte:head>
	<title>Media Library | Revolution Trading Pros</title>
</svelte:head>

<div class="media-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-title">
			<IconPhoto size={32} class="header-icon" />
			<div>
				<h1 class="media-title">Media Library</h1>
				<p class="media-subtitle">Manage your files, images, and videos</p>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-primary" onclick={() => (showUploadModal = !showUploadModal)}>
				<IconPhoto size={18} />
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
					console.warn('Create folder modal not yet implemented');
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
						<IconSearch size={20} class="search-icon" />
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
							<IconTable size={20} />
						</button>
						<button
							class="view-btn"
							class:active={viewMode === 'list'}
							onclick={() => mediaStore.setViewMode('list')}
							aria-label="List view"
						>
							<IconList size={20} />
						</button>
					</div>

					<!-- Bulk Actions -->
					{#if selectedCount > 0}
						<div class="bulk-actions">
							<span class="selected-count">{selectedCount} selected</span>
							<button class="bulk-btn" onclick={handleBulkDelete}>
								<IconTrash size={18} />
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
					<p class="state-text state-text--spaced">Loading files...</p>
				</div>
			{:else if currentFiles.length > 0}
				{#if viewMode === 'grid'}
					<div class="files-grid">
						{#each currentFiles as file (file.id)}
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
										<img
											src={file.thumbnail_url || file.url}
											alt={file.alt_text || file.title}
											width="200"
											height="200"
											loading="lazy"
										/>
									</div>
								{:else}
									{@const FileIcon = getFileIcon(file.file_type)}
									<div class="file-icon-wrapper">
										<FileIcon size={48} class="file-icon" />
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
										<IconSparkles size={12} />
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
									<th class="checkbox-column"></th>
									<th>Name</th>
									<th>Type</th>
									<th>Size</th>
									<th>Date</th>
									<th>Tags</th>
								</tr>
							</thead>
							<tbody>
								{#each currentFiles as file (file.id)}
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
													width="40"
													height="40"
													loading="lazy"
												/>
											{/if}
											<span>{file.title || file.filename}</span>
										</td>
										<td>{file.file_type}</td>
										<td>{formatFileSize(file.file_size)}</td>
										<td>{formatDate(file.created_at)}</td>
										<td>
											<div class="tags">
												{#each (file.tags || []).slice(0, 3) as tag (tag)}
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
					<IconPhoto size={64} class="empty-icon" />
					<h3 class="empty-title">No files yet</h3>
					<p class="empty-copy">Upload your first file to get started</p>
					<button class="btn-primary empty-action" onclick={() => (showUploadModal = true)}>
						<IconPhoto size={18} />
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
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
		min-height: 100%;
	}

	.dashboard-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 2rem;
		gap: 1rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-title :global(.header-icon) {
		color: #facc15;
	}

	.media-title {
		margin: 0;
		color: #fff;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.media-subtitle {
		margin: 0.25rem 0 0;
		color: #9ca3af;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: #eab308;
		color: #111827;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 150ms ease;
	}

	.btn-primary:hover {
		background: #facc15;
	}

	.dashboard-content {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.sidebar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.main-area {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.upload-section {
		margin-bottom: 1.5rem;
	}

	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.toolbar-left {
		display: flex;
		flex: 1;
		align-items: center;
		gap: 0.75rem;
		min-width: min(100%, 20rem);
	}

	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.search-box {
		position: relative;
		flex: 1;
		max-width: 28rem;
	}

	.search-box :global(.search-icon) {
		position: absolute;
		top: 50%;
		left: 0.75rem;
		color: #9ca3af;
		transform: translateY(-50%);
	}

	.search-input {
		width: 100%;
		padding: 0.5rem 1rem 0.5rem 2.5rem;
		border: 1px solid #374151;
		border-radius: 0.5rem;
		background: #1f2937;
		color: #fff;
	}

	.search-input:focus,
	.filter-select:focus {
		outline: none;
		box-shadow: 0 0 0 2px #eab308;
	}

	.filter-select {
		padding: 0.5rem 1rem;
		border: 1px solid #374151;
		border-radius: 0.5rem;
		background: #1f2937;
		color: #fff;
	}

	.view-toggle {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		border: 1px solid #374151;
		border-radius: 0.5rem;
		background: #1f2937;
	}

	.view-btn {
		padding: 0.5rem;
		border: 0;
		border-radius: 0.375rem;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		transition:
			background-color 150ms ease,
			color 150ms ease;
	}

	.view-btn:hover {
		background: #374151;
		color: #fff;
	}

	.view-btn.active {
		background: #eab308;
		color: #111827;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 1px solid #374151;
		border-radius: 0.5rem;
		background: #1f2937;
	}

	.selected-count {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	.bulk-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.75rem;
		border: 0;
		border-radius: 0.25rem;
		background: #374151;
		color: #fff;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background-color 150ms ease;
	}

	.bulk-btn:hover {
		background: #4b5563;
	}

	.files-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.file-card {
		position: relative;
		overflow: hidden;
		border: 2px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
		cursor: pointer;
		transition:
			background-color 150ms ease,
			border-color 150ms ease;
	}

	.file-card:hover {
		border-color: rgba(234, 179, 8, 0.5);
	}

	.file-card.selected {
		border-color: #eab308;
		background: rgba(234, 179, 8, 0.1);
	}

	.file-thumbnail {
		aspect-ratio: 1;
		overflow: hidden;
		background: #111827;
	}

	.file-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-icon-wrapper {
		display: flex;
		aspect-ratio: 1;
		align-items: center;
		justify-content: center;
		background: #111827;
	}

	.file-icon-wrapper :global(.file-icon) {
		color: #9ca3af;
	}

	.file-info {
		padding: 0.75rem;
	}

	.file-name {
		overflow: hidden;
		color: #fff;
		font-size: 0.875rem;
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-meta {
		margin-top: 0.25rem;
		color: #9ca3af;
		font-size: 0.75rem;
	}

	.file-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.file-badge.webp {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.file-badge.ai {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}

	.files-list {
		overflow: hidden;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.files-table {
		width: 100%;
		border-collapse: collapse;
	}

	.files-table thead {
		background: rgba(17, 24, 39, 0.5);
	}

	.files-table th {
		padding: 0.75rem 1rem;
		color: #9ca3af;
		font-size: 0.875rem;
		font-weight: 600;
		text-align: left;
	}

	.files-table tbody tr {
		border-top: 1px solid rgba(55, 65, 81, 0.5);
		transition: background-color 150ms ease;
	}

	.files-table tbody tr:hover {
		background: rgba(55, 65, 81, 0.3);
	}

	.files-table tbody tr.selected {
		background: rgba(234, 179, 8, 0.1);
	}

	.files-table td {
		padding: 0.75rem 1rem;
		color: #d1d5db;
		font-size: 0.875rem;
	}

	.checkbox-column {
		width: 3rem;
	}

	.file-name-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.file-thumb-small {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.25rem;
		object-fit: cover;
	}

	.tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.tag {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		background: #374151;
		color: #d1d5db;
		font-size: 0.75rem;
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.page-btn {
		padding: 0.5rem 1rem;
		border: 1px solid #374151;
		border-radius: 0.5rem;
		background: #1f2937;
		color: #fff;
		cursor: pointer;
		transition: background-color 150ms ease;
	}

	.page-btn:hover:not(:disabled) {
		background: #374151;
	}

	.page-btn:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.page-info {
		color: #9ca3af;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
	}

	.state-text {
		color: #9ca3af;
	}

	.state-text--spaced {
		margin-top: 1rem;
	}

	.empty-state :global(.empty-icon) {
		color: #4b5563;
	}

	.empty-title {
		margin: 1rem 0 0;
		color: #9ca3af;
		font-size: 1.25rem;
		font-weight: 600;
		line-height: 1.75rem;
	}

	.empty-copy {
		margin: 0.5rem 0 0;
		color: #6b7280;
	}

	.empty-action {
		margin-top: 1rem;
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		border: 4px solid #374151;
		border-top-color: #facc15;
		border-radius: 50%;
		animation: media-spin 1s linear infinite;
	}

	@keyframes media-spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 768px) {
		.files-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.dashboard-content {
			grid-template-columns: 300px 1fr;
		}

		.files-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}
	}

	@media (min-width: 1280px) {
		.files-grid {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}
	}
</style>
