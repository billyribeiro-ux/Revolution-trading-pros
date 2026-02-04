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
		} catch (error) {
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
			<IconPhoto size={32} class="text-yellow-400" />
			<div>
				<h1 class="text-3xl font-bold text-white">Media Library</h1>
				<p class="text-gray-400 mt-1">Manage your files, images, and videos</p>
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
							id="page-searchquery" name="page-searchquery" type="text"
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
					<p class="text-gray-400 mt-4">Loading files...</p>
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
										<FileIcon size={48} class="text-gray-400" />
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
									<th class="w-12"></th>
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
												id="page-checkbox" name="page-checkbox" type="checkbox"
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
					<IconPhoto size={64} class="text-gray-600" />
					<h3 class="text-xl font-semibold text-gray-400 mt-4">No files yet</h3>
					<p class="text-gray-500 mt-2">Upload your first file to get started</p>
					<button class="btn-primary mt-4" onclick={() => (showUploadModal = true)}>
						<IconPhoto size={18} />
						Upload Files
					</button>
				</div>
			{/if}
		</main>
	</div>
</div>

<style lang="postcss">
	@reference "../../app.css";
	.media-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
	}

	.dashboard-header {
		@apply flex items-center justify-between mb-8;
	}

	.header-title {
		@apply flex items-center gap-4;
	}

	.header-actions {
		@apply flex items-center gap-3;
	}

	.btn-primary {
		@apply flex items-center gap-2 px-6 py-3 bg-yellow-500 text-gray-900 font-semibold rounded-lg;
		@apply hover:bg-yellow-400 transition-colors;
	}

	.dashboard-content {
		@apply grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-6;
	}

	.sidebar {
		@apply space-y-4;
	}

	.main-area {
		@apply space-y-6;
	}

	.upload-section {
		@apply mb-6;
	}

	.toolbar {
		@apply flex items-center justify-between gap-4 flex-wrap;
	}

	.toolbar-left {
		@apply flex items-center gap-3 flex-1;
	}

	.toolbar-right {
		@apply flex items-center gap-3;
	}

	.search-box {
		@apply relative flex-1 max-w-md;
	}

	.search-box :global(.search-icon) {
		@apply absolute left-3 top-1/2 -translate-y-1/2 text-gray-400;
	}

	.search-input {
		@apply w-full pl-10 pr-4 py-2 bg-gray-800 text-white rounded-lg border border-gray-700;
		@apply focus:outline-none focus:ring-2 focus:ring-yellow-500;
	}

	.filter-select {
		@apply px-4 py-2 bg-gray-800 text-white rounded-lg border border-gray-700;
		@apply focus:outline-none focus:ring-2 focus:ring-yellow-500;
	}

	.view-toggle {
		@apply flex gap-1 bg-gray-800 rounded-lg p-1 border border-gray-700;
	}

	.view-btn {
		@apply p-2 text-gray-400 rounded-md transition-colors;
		@apply hover:text-white hover:bg-gray-700;
	}

	.view-btn.active {
		@apply bg-yellow-500 text-gray-900;
	}

	.bulk-actions {
		@apply flex items-center gap-2 px-4 py-2 bg-gray-800 rounded-lg border border-gray-700;
	}

	.selected-count {
		@apply text-sm text-gray-400;
	}

	.bulk-btn {
		@apply flex items-center gap-1 px-3 py-1 text-sm text-white bg-gray-700 rounded;
		@apply hover:bg-gray-600 transition-colors;
	}

	.files-grid {
		@apply grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4;
	}

	.file-card {
		@apply relative bg-gray-800/50 rounded-xl overflow-hidden border-2 border-gray-700/50;
		@apply cursor-pointer transition-all hover:border-yellow-500/50;
	}

	.file-card.selected {
		@apply border-yellow-500 bg-yellow-500/10;
	}

	.file-thumbnail {
		@apply aspect-square bg-gray-900 overflow-hidden;
	}

	.file-thumbnail img {
		@apply w-full h-full object-cover;
	}

	.file-icon-wrapper {
		@apply aspect-square flex items-center justify-center bg-gray-900;
	}

	.file-info {
		@apply p-3;
	}

	.file-name {
		@apply text-sm font-medium text-white truncate;
	}

	.file-meta {
		@apply text-xs text-gray-400 mt-1;
	}

	.file-badge {
		@apply absolute top-2 right-2 px-2 py-1 text-xs font-semibold rounded;
	}

	.file-badge.webp {
		@apply bg-green-500/20 text-green-400;
	}

	.file-badge.ai {
		@apply bg-purple-500/20 text-purple-400 flex items-center gap-1;
	}

	.files-list {
		@apply bg-gray-800/50 rounded-xl border border-gray-700/50 overflow-hidden;
	}

	.files-table {
		@apply w-full;
	}

	.files-table thead {
		@apply bg-gray-900/50;
	}

	.files-table th {
		@apply px-4 py-3 text-left text-sm font-semibold text-gray-400;
	}

	.files-table tbody tr {
		@apply border-t border-gray-700/50 hover:bg-gray-700/30 transition-colors;
	}

	.files-table tbody tr.selected {
		@apply bg-yellow-500/10;
	}

	.files-table td {
		@apply px-4 py-3 text-sm text-gray-300;
	}

	.file-name-cell {
		@apply flex items-center gap-3;
	}

	.file-thumb-small {
		@apply w-10 h-10 object-cover rounded;
	}

	.tags {
		@apply flex gap-1 flex-wrap;
	}

	.tag {
		@apply px-2 py-0.5 bg-gray-700 text-gray-300 rounded text-xs;
	}

	.pagination {
		@apply flex items-center justify-center gap-4 mt-6;
	}

	.page-btn {
		@apply px-4 py-2 bg-gray-800 text-white rounded-lg border border-gray-700;
		@apply hover:bg-gray-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed;
	}

	.page-info {
		@apply text-gray-400;
	}

	.loading-state,
	.empty-state {
		@apply flex flex-col items-center justify-center py-20 text-center;
	}

	.spinner {
		@apply w-12 h-12 border-4 border-gray-700 border-t-yellow-400 rounded-full animate-spin;
	}
</style>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Files"
	message={`Delete ${selectedCount} file(s)?`}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmBulkDelete}
	onCancel={() => { showDeleteModal = false; }}
/>

<ConfirmationModal
	isOpen={showDeleteErrorModal}
	title="Delete Failed"
	message="Failed to delete files. Some files may be in use."
	confirmText="OK"
	variant="warning"
	onConfirm={() => { showDeleteErrorModal = false; }}
	onCancel={() => { showDeleteErrorModal = false; }}
/>
