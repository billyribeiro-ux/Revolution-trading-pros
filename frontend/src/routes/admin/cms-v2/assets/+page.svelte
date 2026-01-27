<!--
	CMS v2 Asset Management - Apple ICT 7+ Principal Engineer Grade
	═══════════════════════════════════════════════════════════════════════════════

	Enterprise Digital Asset Management (DAM) with:
	- Hierarchical folder organization
	- Drag-and-drop upload with progress
	- Grid/List view modes
	- Asset search and filtering
	- Metadata editing panel
	- Bulk operations (move, delete)
	- Integration with CMS v2 API

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { fly, fade, scale, slide } from 'svelte/transition';
	import { cubicOut, backOut } from 'svelte/easing';
	import { cmsApi, type CmsAsset, type CmsAssetSummary, type CmsAssetFolder, type AssetListQuery } from '$lib/api/cms-v2';
	import DropZone from '$lib/components/media/DropZone.svelte';
	import {
		IconFolder,
		IconFolderPlus,
		IconFile,
		IconPhoto,
		IconVideo,
		IconFileText,
		IconChevronRight,
		IconChevronDown,
		IconUpload,
		IconSearch,
		IconX,
		IconTrash,
		IconEdit,
		IconCopy,
		IconDownload,
		IconLayoutGrid,
		IconList,
		IconDotsVertical,
		IconCheck,
		IconRefresh,
		IconPlus,
		IconArrowsLeftRight
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// Svelte Actions (WCAG 2.1 AA Compliant)
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * focusOnMount - Accessible alternative to autofocus attribute
	 * Programmatically focuses element after mount, which is the WCAG-compliant way
	 */
	function focusOnMount(node: HTMLElement) {
		// Use requestAnimationFrame to ensure DOM is ready
		requestAnimationFrame(() => {
			node.focus();
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════

	type FolderNode = CmsAssetFolder & { children: FolderNode[] };

	interface UploadItem {
		id: string;
		file: File;
		progress: number;
		status: 'pending' | 'uploading' | 'complete' | 'error';
		error?: string;
		result?: CmsAsset;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	// Data
	let folders = $state<CmsAssetFolder[]>([]);
	let assets = $state<CmsAssetSummary[]>([]);
	let selectedAsset = $state<CmsAssetSummary | null>(null);
	let selectedIds = $state(new Set<string>());

	// Folder state
	let currentFolderId = $state<string | null>(null);
	let expandedFolders = $state(new Set<string>());
	let editingFolder = $state<CmsAssetFolder | null>(null);
	let newFolderName = $state('');
	let isCreatingFolder = $state(false);

	// Pagination
	let currentPage = $state(1);
	let totalPages = $state(1);
	let totalAssets = $state(0);
	let perPage = $state(24);

	// Filters
	let searchQuery = $state('');
	let filterMimeType = $state<string | null>(null);

	// UI State
	let viewMode = $state<'grid' | 'list'>('grid');
	let isLoading = $state(true);
	let isUploading = $state(false);
	let showUploadPanel = $state(false);
	let showDetailsPanel = $state(true);
	let showContextMenu = $state<{ x: number; y: number; assetId: string } | null>(null);
	let showFolderContextMenu = $state<{ x: number; y: number; folderId: string } | null>(null);
	let showCreateFolderModal = $state(false);
	let showMoveFolderModal = $state(false);
	let moveTargetFolderId = $state<string | null>(null);

	// Upload tracking
	let uploadQueue = $state<UploadItem[]>([]);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════

	let folderTree = $derived(buildFolderTree(folders));

	let currentFolder = $derived(
		currentFolderId ? folders.find((f) => f.id === currentFolderId) : null
	);

	let breadcrumbs = $derived(getBreadcrumbs(currentFolderId));

	let hasSelection = $derived(selectedIds.size > 0);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser) return;

		loadFolders();
		loadAssets();

		document.addEventListener('click', handleClickOutside);
		document.addEventListener('keydown', handleKeydown);

		return () => {
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('keydown', handleKeydown);
		};
	});

	// Reload assets when folder or filters change
	$effect(() => {
		if (!browser) return;
		// Dependencies: currentFolderId, searchQuery, filterMimeType, currentPage
		const _deps = [currentFolderId, searchQuery, filterMimeType, currentPage];
		loadAssets();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadFolders() {
		try {
			const response = await cmsApi.listAssetFolders();
			folders = response;
		} catch (e) {
			console.error('Failed to load folders:', e);
		}
	}

	async function loadAssets() {
		isLoading = true;
		try {
			const query: AssetListQuery = {
				page: currentPage,
				per_page: perPage,
				folder_id: currentFolderId ?? undefined,
				search: searchQuery || undefined,
				mime_type_filter: filterMimeType ?? undefined
			};

			const response = await cmsApi.listAssets(query);
			assets = response.data;
			totalAssets = response.pagination.total;
			totalPages = response.pagination.total_pages;
		} catch (e) {
			console.error('Failed to load assets:', e);
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Folder Operations
	// ═══════════════════════════════════════════════════════════════════════════

	function buildFolderTree(folderList: CmsAssetFolder[]): FolderNode[] {
		const tree: FolderNode[] = [];
		const map = new Map<string, FolderNode>();

		// Create map with children arrays
		folderList.forEach((folder) => {
			map.set(folder.id, { ...folder, children: [] });
		});

		// Build tree
		folderList.forEach((folder) => {
			const node = map.get(folder.id)!;
			if (folder.parent_id) {
				const parent = map.get(folder.parent_id);
				if (parent) {
					parent.children.push(node);
				}
			} else {
				tree.push(node);
			}
		});

		return tree;
	}

	function getBreadcrumbs(folderId: string | null): CmsAssetFolder[] {
		if (!folderId) return [];

		const crumbs: CmsAssetFolder[] = [];
		let current = folders.find((f) => f.id === folderId);

		while (current) {
			crumbs.unshift(current);
			current = current.parent_id ? folders.find((f) => f.id === current!.parent_id) : undefined;
		}

		return crumbs;
	}

	function toggleFolder(folderId: string) {
		if (expandedFolders.has(folderId)) {
			expandedFolders.delete(folderId);
		} else {
			expandedFolders.add(folderId);
		}
		expandedFolders = expandedFolders;
	}

	function selectFolder(folderId: string | null) {
		currentFolderId = folderId;
		currentPage = 1;
		selectedIds.clear();
		selectedIds = selectedIds;
		selectedAsset = null;
	}

	async function createFolder() {
		if (!newFolderName.trim()) return;

		try {
			await cmsApi.createAssetFolder({
				name: newFolderName.trim(),
				parentId: currentFolderId ?? undefined
			});
			newFolderName = '';
			showCreateFolderModal = false;
			await loadFolders();
		} catch (e) {
			console.error('Failed to create folder:', e);
		}
	}

	async function renameFolder(folder: CmsAssetFolder, newName: string) {
		if (!newName.trim() || newName === folder.name) {
			editingFolder = null;
			return;
		}

		try {
			await cmsApi.updateAssetFolder(folder.id, { name: newName.trim() });
			editingFolder = null;
			await loadFolders();
		} catch (e) {
			console.error('Failed to rename folder:', e);
		}
	}

	async function deleteFolder(folderId: string) {
		if (!confirm('Delete this folder and all its contents? This cannot be undone.')) return;

		try {
			await cmsApi.deleteAssetFolder(folderId);
			if (currentFolderId === folderId) {
				currentFolderId = null;
			}
			await loadFolders();
			await loadAssets();
		} catch (e) {
			console.error('Failed to delete folder:', e);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Asset Operations
	// ═══════════════════════════════════════════════════════════════════════════

	function selectAsset(asset: CmsAssetSummary, event?: MouseEvent) {
		if (event?.shiftKey && selectedIds.size > 0) {
			// Range select
			const assetIndices = assets.map((a) => a.id);
			const lastSelectedIndex = assetIndices.indexOf([...selectedIds].pop()!);
			const currentIndex = assetIndices.indexOf(asset.id);
			const [start, end] = [
				Math.min(lastSelectedIndex, currentIndex),
				Math.max(lastSelectedIndex, currentIndex)
			];

			for (let i = start; i <= end; i++) {
				selectedIds.add(assets[i].id);
			}
			selectedIds = selectedIds;
		} else if (event?.ctrlKey || event?.metaKey) {
			// Toggle select
			if (selectedIds.has(asset.id)) {
				selectedIds.delete(asset.id);
			} else {
				selectedIds.add(asset.id);
			}
			selectedIds = selectedIds;
		} else {
			// Single select
			selectedIds.clear();
			selectedIds.add(asset.id);
			selectedIds = selectedIds;
		}

		selectedAsset = asset;
	}

	function selectAll() {
		if (selectedIds.size === assets.length) {
			selectedIds.clear();
		} else {
			assets.forEach((a) => selectedIds.add(a.id));
		}
		selectedIds = selectedIds;
	}

	async function deleteAsset(assetId: string) {
		if (!confirm('Delete this asset? This cannot be undone.')) return;

		try {
			await cmsApi.deleteAsset(assetId);
			selectedIds.delete(assetId);
			selectedIds = selectedIds;
			if (selectedAsset?.id === assetId) {
				selectedAsset = null;
			}
			await loadAssets();
		} catch (e) {
			console.error('Failed to delete asset:', e);
		}
	}

	async function deleteSelected() {
		if (!confirm(`Delete ${selectedIds.size} assets? This cannot be undone.`)) return;

		try {
			await Promise.all([...selectedIds].map((id) => cmsApi.deleteAsset(id)));
			selectedIds.clear();
			selectedIds = selectedIds;
			selectedAsset = null;
			await loadAssets();
		} catch (e) {
			console.error('Failed to delete assets:', e);
		}
	}

	async function moveSelected(targetFolderId: string | null) {
		try {
			await Promise.all(
				[...selectedIds].map((id) =>
					cmsApi.updateAsset(id, { folderId: targetFolderId ?? undefined })
				)
			);
			selectedIds.clear();
			selectedIds = selectedIds;
			showMoveFolderModal = false;
			await loadAssets();
		} catch (e) {
			console.error('Failed to move assets:', e);
		}
	}

	async function updateAssetMetadata() {
		if (!selectedAsset) return;

		try {
			await cmsApi.updateAsset(selectedAsset.id, {
				altText: selectedAsset.alt_text ?? undefined,
				title: selectedAsset.title ?? undefined
			});
			await loadAssets();
		} catch (e) {
			console.error('Failed to update asset:', e);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Upload Handling
	// ═══════════════════════════════════════════════════════════════════════════

	function handleFilesSelected(files: File[]) {
		if (files.length === 0) return;

		showUploadPanel = true;
		isUploading = true;

		files.forEach((file) => {
			const id = crypto.randomUUID();
			uploadQueue = [...uploadQueue, { id, file, progress: 0, status: 'pending' }];
			uploadFile(id, file);
		});
	}

	async function uploadFile(id: string, file: File) {
		const idx = uploadQueue.findIndex((u) => u.id === id);
		if (idx === -1) return;

		const uploadItem = uploadQueue[idx];
		if (!uploadItem) return;

		uploadItem.status = 'uploading';
		uploadQueue = uploadQueue;

		try {
			const formData = new FormData();
			formData.append('file', file);
			if (currentFolderId) {
				formData.append('folder_id', currentFolderId);
			}

			// Using XMLHttpRequest for progress tracking
			const xhr = new XMLHttpRequest();

			xhr.upload.onprogress = (e) => {
				if (e.lengthComputable) {
					const progress = Math.round((e.loaded / e.total) * 100);
					const item = uploadQueue.find((u) => u.id === id);
					if (item) {
						item.progress = progress;
						uploadQueue = uploadQueue;
					}
				}
			};

			const result = await new Promise<CmsAsset>((resolve, reject) => {
				xhr.onload = () => {
					if (xhr.status >= 200 && xhr.status < 300) {
						resolve(JSON.parse(xhr.responseText));
					} else {
						reject(new Error(xhr.statusText || 'Upload failed'));
					}
				};
				xhr.onerror = () => reject(new Error('Network error'));

				xhr.open('POST', '/api/admin/cms-v2/assets');
				xhr.withCredentials = true;
				xhr.send(formData);
			});

			const item = uploadQueue.find((u) => u.id === id);
			if (item) {
				item.status = 'complete';
				item.result = result;
				uploadQueue = uploadQueue;
			}
		} catch (e: any) {
			const item = uploadQueue.find((u) => u.id === id);
			if (item) {
				item.status = 'error';
				item.error = e.message || 'Upload failed';
				uploadQueue = uploadQueue;
			}
		}

		// Check if all uploads complete
		const allDone = uploadQueue.every((u) => u.status === 'complete' || u.status === 'error');
		if (allDone) {
			isUploading = false;
			await loadAssets();
		}
	}

	function clearUploadQueue() {
		uploadQueue = [];
		showUploadPanel = false;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Event Handlers
	// ═══════════════════════════════════════════════════════════════════════════

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.context-menu')) {
			showContextMenu = null;
			showFolderContextMenu = null;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			showContextMenu = null;
			showFolderContextMenu = null;
			showCreateFolderModal = false;
			showMoveFolderModal = false;
		}

		if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
			e.preventDefault();
			selectAll();
		}

		if (e.key === 'Delete' && hasSelection) {
			deleteSelected();
		}
	}

	function handleAssetContextMenu(e: MouseEvent, asset: CmsAssetSummary) {
		e.preventDefault();
		if (!selectedIds.has(asset.id)) {
			selectAsset(asset);
		}
		showContextMenu = { x: e.clientX, y: e.clientY, assetId: asset.id };
	}

	function handleFolderContextMenu(e: MouseEvent, folderId: string) {
		e.preventDefault();
		e.stopPropagation();
		showFolderContextMenu = { x: e.clientX, y: e.clientY, folderId };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	function getAssetIcon(mimeType: string) {
		if (mimeType.startsWith('image/')) return IconPhoto;
		if (mimeType.startsWith('video/')) return IconVideo;
		if (mimeType.startsWith('text/') || mimeType.includes('pdf')) return IconFileText;
		return IconFile;
	}

	function formatFileSize(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<svelte:head>
	<title>Asset Management | CMS v2</title>
</svelte:head>

<div class="asset-manager">
	<!-- Sidebar - Folder Tree -->
	<aside class="folder-sidebar" in:fly={{ x: -20, duration: 400 }}>
		<div class="sidebar-header">
			<h2 class="sidebar-title">Folders</h2>
			<button
				class="btn-icon"
				onclick={() => (showCreateFolderModal = true)}
				title="Create folder"
			>
				<IconFolderPlus size={18} />
			</button>
		</div>

		<!-- All Files -->
		<button
			class="folder-item"
			class:active={currentFolderId === null}
			onclick={() => selectFolder(null)}
		>
			<IconFolder size={20} />
			<span class="folder-name">All Files</span>
			<span class="folder-count">{totalAssets}</span>
		</button>

		<!-- Folder Tree -->
		<div class="folder-list">
			{#each folderTree as folder}
				{@const isExpanded = expandedFolders.has(folder.id)}
				{@const isSelected = currentFolderId === folder.id}
				{@const hasChildren = folder.children.length > 0}

				<div class="folder-group">
					<div class="folder-item-wrapper">
						{#if hasChildren}
							<button
								class="expand-btn"
								onclick={() => toggleFolder(folder.id)}
								aria-label={isExpanded ? 'Collapse' : 'Expand'}
							>
								{#if isExpanded}
									<IconChevronDown size={16} />
								{:else}
									<IconChevronRight size={16} />
								{/if}
							</button>
						{:else}
							<span class="expand-spacer"></span>
						{/if}

						<button
							class="folder-item"
							class:active={isSelected}
							onclick={() => selectFolder(folder.id)}
							oncontextmenu={(e) => handleFolderContextMenu(e, folder.id)}
						>
							<IconFolder size={20} />
							{#if editingFolder?.id === folder.id}
								<input
									class="folder-rename-input"
									type="text"
									value={folder.name}
									onblur={(e) => renameFolder(folder, e.currentTarget.value)}
									onkeydown={(e) => {
										if (e.key === 'Enter') renameFolder(folder, e.currentTarget.value);
										if (e.key === 'Escape') (editingFolder = null);
									}}
									use:focusOnMount
								/>
							{:else}
								<span class="folder-name">{folder.name}</span>
							{/if}
							<span class="folder-count">{folder.asset_count}</span>
						</button>
					</div>

					{#if hasChildren && isExpanded}
						{#each folder.children as child}
							{@const childExpanded = expandedFolders.has(child.id)}
							{@const childSelected = currentFolderId === child.id}
							{@const childHasChildren = child.children.length > 0}

							<div class="folder-item-wrapper child">
								{#if childHasChildren}
									<button
										class="expand-btn"
										onclick={() => toggleFolder(child.id)}
									>
										{#if childExpanded}
											<IconChevronDown size={16} />
										{:else}
											<IconChevronRight size={16} />
										{/if}
									</button>
								{:else}
									<span class="expand-spacer"></span>
								{/if}

								<button
									class="folder-item"
									class:active={childSelected}
									onclick={() => selectFolder(child.id)}
									oncontextmenu={(e) => handleFolderContextMenu(e, child.id)}
								>
									<IconFolder size={18} />
									<span class="folder-name">{child.name}</span>
									<span class="folder-count">{child.asset_count}</span>
								</button>
							</div>
						{/each}
					{/if}
				</div>
			{/each}
		</div>
	</aside>

	<!-- Main Content -->
	<main class="asset-content">
		<!-- Toolbar -->
		<header class="toolbar" in:fly={{ y: -10, duration: 300 }}>
			<!-- Breadcrumbs -->
			<nav class="breadcrumbs">
				<button class="breadcrumb" onclick={() => selectFolder(null)}>
					<IconFolder size={16} />
					All Files
				</button>
				{#each breadcrumbs as crumb}
					<IconChevronRight size={14} class="breadcrumb-sep" />
					<button class="breadcrumb" onclick={() => selectFolder(crumb.id)}>
						{crumb.name}
					</button>
				{/each}
			</nav>

			<!-- Actions -->
			<div class="toolbar-actions">
				<!-- Search -->
				<div class="search-box">
					<IconSearch size={16} />
					<input
						type="text"
						placeholder="Search assets..."
						bind:value={searchQuery}
						class="search-input"
					/>
					{#if searchQuery}
						<button class="btn-clear" onclick={() => (searchQuery = '')}>
							<IconX size={14} />
						</button>
					{/if}
				</div>

				<!-- View Toggle -->
				<div class="view-toggle">
					<button
						class="toggle-btn"
						class:active={viewMode === 'grid'}
						onclick={() => (viewMode = 'grid')}
						title="Grid view"
					>
						<IconLayoutGrid size={18} />
					</button>
					<button
						class="toggle-btn"
						class:active={viewMode === 'list'}
						onclick={() => (viewMode = 'list')}
						title="List view"
					>
						<IconList size={18} />
					</button>
				</div>

				<!-- Bulk Actions -->
				{#if hasSelection}
					<div class="bulk-actions" in:scale={{ duration: 200 }}>
						<span class="selection-count">{selectedIds.size} selected</span>
						<button
							class="btn-action"
							onclick={() => (showMoveFolderModal = true)}
							title="Move"
						>
							<IconArrowsLeftRight size={16} />
						</button>
						<button class="btn-action danger" onclick={deleteSelected} title="Delete">
							<IconTrash size={16} />
						</button>
					</div>
				{/if}

				<!-- Upload Button -->
				<button
					class="btn-upload"
					onclick={() => (showUploadPanel = true)}
				>
					<IconUpload size={18} />
					<span>Upload</span>
				</button>

				<!-- Refresh -->
				<button
					class="btn-icon"
					onclick={() => { loadFolders(); loadAssets(); }}
					title="Refresh"
				>
					<IconRefresh size={18} />
				</button>
			</div>
		</header>

		<!-- Upload Panel -->
		{#if showUploadPanel}
			<div class="upload-panel" in:slide={{ duration: 300 }}>
				<div class="upload-header">
					<h3>Upload Files</h3>
					<button class="btn-close" onclick={clearUploadQueue}>
						<IconX size={18} />
					</button>
				</div>

				<DropZone
					accept="image/*,video/*,application/pdf,.doc,.docx,.xls,.xlsx"
					multiple={true}
					maxSize={100 * 1024 * 1024}
					onfiles={handleFilesSelected}
				/>

				{#if uploadQueue.length > 0}
					<div class="upload-queue">
						{#each uploadQueue as item (item.id)}
							<div class="upload-item" class:error={item.status === 'error'}>
								<div class="upload-info">
									<span class="upload-name">{item.file.name}</span>
									<span class="upload-size">{formatFileSize(item.file.size)}</span>
								</div>
								<div class="upload-progress-bar">
									<div
										class="upload-progress-fill"
										class:complete={item.status === 'complete'}
										class:error={item.status === 'error'}
										style="width: {item.progress}%"
									></div>
								</div>
								{#if item.status === 'complete'}
									<IconCheck size={16} class="text-green-500" />
								{:else if item.status === 'error'}
									<span class="text-red-500 text-xs">{item.error}</span>
								{:else}
									<span class="text-gray-400 text-xs">{item.progress}%</span>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/if}

		<!-- Asset Grid/List -->
		<div class="asset-area">
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading assets...</p>
				</div>
			{:else if assets.length === 0}
				<div class="empty-state" in:scale={{ duration: 400 }}>
					<IconPhoto size={64} />
					<h3>No assets found</h3>
					<p>
						{#if searchQuery}
							No assets match your search. Try different keywords.
						{:else if currentFolderId}
							This folder is empty. Upload some files to get started.
						{:else}
							Your asset library is empty. Start by uploading files.
						{/if}
					</p>
					<button class="btn-primary" onclick={() => (showUploadPanel = true)}>
						<IconUpload size={18} />
						Upload Files
					</button>
				</div>
			{:else}
				<div class="asset-{viewMode}">
					{#each assets as asset (asset.id)}
						{@const AssetIcon = getAssetIcon(asset.mime_type)}
						{@const isSelected = selectedIds.has(asset.id)}

						<button
							class="asset-item"
							class:selected={isSelected}
							onclick={(e) => selectAsset(asset, e)}
							ondblclick={() => {
								selectedAsset = asset;
								showDetailsPanel = true;
							}}
							oncontextmenu={(e) => handleAssetContextMenu(e, asset)}
							in:scale={{ duration: 300, delay: 50 }}
						>
							{#if viewMode === 'grid'}
								<div class="asset-preview">
									{#if asset.mime_type.startsWith('image/')}
										<img
											src={asset.url}
											alt={asset.alt_text || asset.filename}
											loading="lazy"
										/>
									{:else}
										<AssetIcon size={48} />
									{/if}
								</div>
								<div class="asset-info">
									<span class="asset-name">{asset.filename}</span>
									<span class="asset-meta">{formatFileSize(asset.file_size)}</span>
								</div>
							{:else}
								<div class="asset-icon">
									{#if asset.mime_type.startsWith('image/')}
										<img
											src={asset.url}
											alt={asset.alt_text || asset.filename}
											loading="lazy"
										/>
									{:else}
										<AssetIcon size={24} />
									{/if}
								</div>
								<div class="asset-details">
									<span class="asset-name">{asset.filename}</span>
									<span class="asset-type">{asset.mime_type}</span>
								</div>
								<span class="asset-size">{formatFileSize(asset.file_size)}</span>
								<span class="asset-date">{formatDate(asset.created_at)}</span>
							{/if}

							{#if isSelected}
								<div class="selection-indicator">
									<IconCheck size={16} />
								</div>
							{/if}
						</button>
					{/each}
				</div>

				<!-- Pagination -->
				{#if totalPages > 1}
					<div class="pagination">
						<button
							class="page-btn"
							disabled={currentPage === 1}
							onclick={() => (currentPage = Math.max(1, currentPage - 1))}
						>
							Previous
						</button>
						<span class="page-info">
							Page {currentPage} of {totalPages}
						</span>
						<button
							class="page-btn"
							disabled={currentPage === totalPages}
							onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
						>
							Next
						</button>
					</div>
				{/if}
			{/if}
		</div>
	</main>

	<!-- Details Panel -->
	{#if showDetailsPanel && selectedAsset}
		<aside class="details-panel" in:fly={{ x: 20, duration: 300 }}>
			<div class="details-header">
				<h3>Asset Details</h3>
				<button class="btn-close" onclick={() => (showDetailsPanel = false)}>
					<IconX size={18} />
				</button>
			</div>

			<div class="details-preview">
				{#if selectedAsset.mime_type.startsWith('image/')}
					<img src={selectedAsset.url} alt={selectedAsset.alt_text || selectedAsset.filename} />
				{:else}
					{@const DetailIcon = getAssetIcon(selectedAsset.mime_type)}
					<DetailIcon size={64} />
				{/if}
			</div>

			<dl class="details-info">
				<div class="detail-row">
					<dt>Filename</dt>
					<dd>{selectedAsset.filename}</dd>
				</div>
				<div class="detail-row">
					<dt>Type</dt>
					<dd>{selectedAsset.mime_type}</dd>
				</div>
				<div class="detail-row">
					<dt>Size</dt>
					<dd>{formatFileSize(selectedAsset.file_size)}</dd>
				</div>
				{#if selectedAsset.width && selectedAsset.height}
					<div class="detail-row">
						<dt>Dimensions</dt>
						<dd>{selectedAsset.width} x {selectedAsset.height}</dd>
					</div>
				{/if}
				<div class="detail-row">
					<dt>Uploaded</dt>
					<dd>{formatDate(selectedAsset.created_at)}</dd>
				</div>
			</dl>

			<div class="details-metadata">
				<h4>Metadata</h4>
				<div class="metadata-field">
					<label for="alt-text">Alt Text</label>
					<input
						id="alt-text"
						type="text"
						bind:value={selectedAsset.alt_text}
						placeholder="Describe the image..."
					/>
				</div>
				<div class="metadata-field">
					<label for="title">Title</label>
					<input
						id="title"
						type="text"
						bind:value={selectedAsset.title}
						placeholder="Asset title..."
					/>
				</div>
				<button class="btn-save" onclick={updateAssetMetadata}>
					Save Metadata
				</button>
			</div>

			<div class="details-actions">
				<button class="btn-action" onclick={() => window.open(selectedAsset?.url, '_blank')}>
					<IconDownload size={16} />
					Download
				</button>
				<button
					class="btn-action"
					onclick={() => navigator.clipboard.writeText(selectedAsset?.url || '')}
				>
					<IconCopy size={16} />
					Copy URL
				</button>
				<button class="btn-action danger" onclick={() => deleteAsset(selectedAsset!.id)}>
					<IconTrash size={16} />
					Delete
				</button>
			</div>
		</aside>
	{/if}
</div>

<!-- Context Menu -->
{#if showContextMenu}
	<div
		class="context-menu"
		style="left: {showContextMenu.x}px; top: {showContextMenu.y}px"
		in:scale={{ duration: 150, start: 0.95 }}
	>
		<button
			class="context-item"
			onclick={() => {
				selectedAsset = assets.find((a) => a.id === showContextMenu?.assetId) || null;
				showDetailsPanel = true;
				showContextMenu = null;
			}}
		>
			<IconEdit size={16} />
			Edit Details
		</button>
		<button
			class="context-item"
			onclick={() => {
				showMoveFolderModal = true;
				showContextMenu = null;
			}}
		>
			<IconArrowsLeftRight size={16} />
			Move to Folder
		</button>
		<button
			class="context-item"
			onclick={() => {
				navigator.clipboard.writeText(
					assets.find((a) => a.id === showContextMenu?.assetId)?.url || ''
				);
				showContextMenu = null;
			}}
		>
			<IconCopy size={16} />
			Copy URL
		</button>
		<button
			class="context-item"
			onclick={() => {
				window.open(assets.find((a) => a.id === showContextMenu?.assetId)?.url, '_blank');
				showContextMenu = null;
			}}
		>
			<IconDownload size={16} />
			Download
		</button>
		<hr class="context-divider" />
		<button
			class="context-item danger"
			onclick={() => {
				deleteAsset(showContextMenu?.assetId || '');
				showContextMenu = null;
			}}
		>
			<IconTrash size={16} />
			Delete
		</button>
	</div>
{/if}

<!-- Folder Context Menu -->
{#if showFolderContextMenu}
	<div
		class="context-menu"
		style="left: {showFolderContextMenu.x}px; top: {showFolderContextMenu.y}px"
		in:scale={{ duration: 150, start: 0.95 }}
	>
		<button
			class="context-item"
			onclick={() => {
				editingFolder = folders.find((f) => f.id === showFolderContextMenu?.folderId) || null;
				showFolderContextMenu = null;
			}}
		>
			<IconEdit size={16} />
			Rename
		</button>
		<hr class="context-divider" />
		<button
			class="context-item danger"
			onclick={() => {
				deleteFolder(showFolderContextMenu?.folderId || '');
				showFolderContextMenu = null;
			}}
		>
			<IconTrash size={16} />
			Delete
		</button>
	</div>
{/if}

<!-- Create Folder Modal -->
{#if showCreateFolderModal}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={() => (showCreateFolderModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateFolderModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="create-folder-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			in:scale={{ duration: 200, start: 0.95 }}
		>
			<div class="modal-header">
				<h3 id="create-folder-title">Create Folder</h3>
				<button class="btn-close" onclick={() => (showCreateFolderModal = false)}>
					<IconX size={18} />
				</button>
			</div>
			<div class="modal-body">
				<div class="form-field">
					<label for="folder-name">Folder Name</label>
					<input
						id="folder-name"
						type="text"
						bind:value={newFolderName}
						placeholder="Enter folder name..."
						use:focusOnMount
						onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && createFolder()}
					/>
				</div>
				{#if currentFolder}
					<p class="folder-location">
						Creating in: <strong>{currentFolder.name}</strong>
					</p>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateFolderModal = false)}>
					Cancel
				</button>
				<button class="btn-primary" onclick={createFolder} disabled={!newFolderName.trim()}>
					Create Folder
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Move to Folder Modal -->
{#if showMoveFolderModal}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={() => (showMoveFolderModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showMoveFolderModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="move-folder-title"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			in:scale={{ duration: 200, start: 0.95 }}
		>
			<div class="modal-header">
				<h3 id="move-folder-title">Move {selectedIds.size} {selectedIds.size === 1 ? 'Asset' : 'Assets'}</h3>
				<button class="btn-close" onclick={() => (showMoveFolderModal = false)}>
					<IconX size={18} />
				</button>
			</div>
			<div class="modal-body">
				<div class="move-folder-list">
					<button
						class="move-folder-item"
						class:selected={moveTargetFolderId === null}
						onclick={() => (moveTargetFolderId = null)}
					>
						<IconFolder size={20} />
						<span>Root (No Folder)</span>
					</button>
					{#each folders as folder}
						<button
							class="move-folder-item"
							class:selected={moveTargetFolderId === folder.id}
							onclick={() => (moveTargetFolderId = folder.id)}
						>
							<IconFolder size={20} />
							<span>{folder.name}</span>
						</button>
					{/each}
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showMoveFolderModal = false)}>
					Cancel
				</button>
				<button class="btn-primary" onclick={() => moveSelected(moveTargetFolderId)}>
					Move Here
				</button>
			</div>
		</div>
	</div>
{/if}

<style lang="postcss">
	/* ═══════════════════════════════════════════════════════════════════════════
	   Asset Manager Layout
	   ═══════════════════════════════════════════════════════════════════════════ */

	.asset-manager {
		display: grid;
		grid-template-columns: 260px 1fr;
		gap: 0;
		height: calc(100vh - 140px);
		background: var(--admin-bg);
	}

	.asset-manager:has(.details-panel) {
		grid-template-columns: 260px 1fr 340px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Folder Sidebar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.folder-sidebar {
		background: rgba(30, 41, 59, 0.5);
		border-right: 1px solid rgba(51, 65, 85, 0.5);
		padding: 1rem;
		overflow-y: auto;
	}

	.sidebar-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.sidebar-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.folder-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		margin-top: 0.5rem;
	}

	.folder-group {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.folder-item-wrapper {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.folder-item-wrapper.child {
		padding-left: 1.5rem;
	}

	.expand-btn {
		padding: 0.25rem;
		color: #64748b;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		cursor: pointer;
		transition: color 0.15s;
	}

	.expand-btn:hover {
		color: #e6b800;
	}

	.expand-spacer {
		width: 1.25rem;
	}

	.folder-item {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.folder-item:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.folder-item.active {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}

	.folder-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.folder-count {
		font-size: 0.75rem;
		padding: 0.125rem 0.375rem;
		background: rgba(51, 65, 85, 0.5);
		border-radius: 0.25rem;
		color: #64748b;
	}

	.folder-item.active .folder-count {
		background: rgba(230, 184, 0, 0.2);
		color: #e6b800;
	}

	.folder-rename-input {
		flex: 1;
		padding: 0.25rem 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid #e6b800;
		border-radius: 0.25rem;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Main Content
	   ═══════════════════════════════════════════════════════════════════════════ */

	.asset-content {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem 1.5rem;
		background: rgba(30, 41, 59, 0.3);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.breadcrumbs {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.breadcrumb:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.breadcrumb:last-child {
		color: #e6b800;
	}

	.breadcrumbs :global(.breadcrumb-sep) {
		color: #475569;
	}

	.toolbar-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
	}

	.search-input {
		width: 200px;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		outline: none;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.btn-clear {
		padding: 0.125rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: color 0.15s;
	}

	.btn-clear:hover {
		color: #ef4444;
	}

	.view-toggle {
		display: flex;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.toggle-btn {
		padding: 0.5rem 0.625rem;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toggle-btn:hover {
		color: #f1f5f9;
	}

	.toggle-btn.active {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 0.5rem;
	}

	.selection-count {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e6b800;
	}

	.btn-action {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.375rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-action:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-action.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.btn-upload {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-upload:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-icon:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
		border-color: rgba(230, 184, 0, 0.3);
	}

	/* Upload Panel */
	.upload-panel {
		padding: 1rem 1.5rem;
		background: rgba(30, 41, 59, 0.5);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.upload-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.upload-header h3 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.btn-close {
		padding: 0.375rem;
		background: transparent;
		border: none;
		color: #64748b;
		border-radius: 0.375rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-close:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.upload-queue {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.upload-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}

	.upload-item.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.upload-info {
		flex: 1;
		min-width: 0;
	}

	.upload-name {
		display: block;
		font-size: 0.875rem;
		color: #f1f5f9;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.upload-size {
		font-size: 0.75rem;
		color: #64748b;
	}

	.upload-progress-bar {
		flex: 1;
		height: 4px;
		background: rgba(51, 65, 85, 0.5);
		border-radius: 2px;
		overflow: hidden;
	}

	.upload-progress-fill {
		height: 100%;
		background: #e6b800;
		transition: width 0.2s;
	}

	.upload-progress-fill.complete {
		background: #10b981;
	}

	.upload-progress-fill.error {
		background: #ef4444;
	}

	/* Asset Area */
	.asset-area {
		flex: 1;
		padding: 1.5rem;
		overflow-y: auto;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		margin: 1rem 0 0.5rem;
		font-size: 1.25rem;
		color: #f1f5f9;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
		max-width: 320px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Asset Grid */
	.asset-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
		gap: 1rem;
	}

	.asset-grid .asset-item {
		display: flex;
		flex-direction: column;
		padding: 0;
		background: rgba(30, 41, 59, 0.5);
		border: 2px solid transparent;
		border-radius: 0.75rem;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
	}

	.asset-grid .asset-item:hover {
		background: rgba(51, 65, 85, 0.5);
		border-color: rgba(51, 65, 85, 0.5);
	}

	.asset-grid .asset-item.selected {
		border-color: #e6b800;
		background: rgba(230, 184, 0, 0.1);
	}

	.asset-preview {
		width: 100%;
		aspect-ratio: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.2);
		color: #64748b;
	}

	.asset-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.asset-grid .asset-info {
		padding: 0.75rem;
	}

	.asset-name {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #f1f5f9;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.asset-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.selection-indicator {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e6b800;
		border-radius: 50%;
		color: #0f172a;
	}

	/* Asset List */
	.asset-list {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.asset-list .asset-item {
		display: grid;
		grid-template-columns: 48px 1fr 100px 140px;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid transparent;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.15s;
		position: relative;
	}

	.asset-list .asset-item:hover {
		background: rgba(51, 65, 85, 0.5);
	}

	.asset-list .asset-item.selected {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
	}

	.asset-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.375rem;
		color: #64748b;
		overflow: hidden;
	}

	.asset-icon img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.asset-details {
		min-width: 0;
	}

	.asset-type {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
	}

	.asset-size,
	.asset-date {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.asset-list .selection-indicator {
		position: absolute;
		left: -2px;
		top: 50%;
		transform: translateY(-50%);
		width: 4px;
		height: 60%;
		border-radius: 0 2px 2px 0;
		background: #e6b800;
	}

	.asset-list .selection-indicator :global(svg) {
		display: none;
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		margin-top: 1.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.page-btn {
		padding: 0.5rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Details Panel
	   ═══════════════════════════════════════════════════════════════════════════ */

	.details-panel {
		background: rgba(30, 41, 59, 0.5);
		border-left: 1px solid rgba(51, 65, 85, 0.5);
		display: flex;
		flex-direction: column;
		overflow-y: auto;
	}

	.details-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.details-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.details-preview {
		width: 100%;
		aspect-ratio: 16/10;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
		color: #64748b;
	}

	.details-preview img {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
	}

	.details-info {
		padding: 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		padding: 0.5rem 0;
	}

	.detail-row dt {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.detail-row dd {
		font-size: 0.8125rem;
		color: #f1f5f9;
	}

	.details-metadata {
		padding: 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.details-metadata h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.metadata-field {
		margin-bottom: 0.75rem;
	}

	.metadata-field label {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		margin-bottom: 0.375rem;
	}

	.metadata-field input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		transition: border-color 0.15s;
	}

	.metadata-field input:focus {
		outline: none;
		border-color: #e6b800;
	}

	.btn-save {
		width: 100%;
		margin-top: 0.75rem;
		padding: 0.5rem;
		background: rgba(230, 184, 0, 0.15);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 0.375rem;
		color: #e6b800;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-save:hover {
		background: rgba(230, 184, 0, 0.25);
	}

	.details-actions {
		padding: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.details-actions .btn-action {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.875rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.875rem;
		justify-content: flex-start;
		width: 100%;
	}

	.details-actions .btn-action:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.details-actions .btn-action.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Context Menu
	   ═══════════════════════════════════════════════════════════════════════════ */

	.context-menu {
		position: fixed;
		z-index: 1000;
		min-width: 180px;
		padding: 0.5rem;
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
	}

	.context-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.context-item:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.context-item.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.context-divider {
		margin: 0.375rem 0;
		border: none;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Modal
	   ═══════════════════════════════════════════════════════════════════════════ */

	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.modal {
		width: 100%;
		max-width: 440px;
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-header h3 {
		font-size: 1.0625rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.modal-body {
		padding: 1.25rem;
	}

	.form-field {
		margin-bottom: 1rem;
	}

	.form-field label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.375rem;
	}

	.form-field input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: border-color 0.15s;
	}

	.form-field input:focus {
		outline: none;
		border-color: #e6b800;
	}

	.folder-location {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.folder-location strong {
		color: #e6b800;
	}

	.move-folder-list {
		max-height: 300px;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.move-folder-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.875rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
		width: 100%;
	}

	.move-folder-item:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.move-folder-item.selected {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.btn-secondary {
		padding: 0.5rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Utilities
	   ═══════════════════════════════════════════════════════════════════════════ */

	.text-green-500 {
		color: #10b981;
	}

	.text-red-500 {
		color: #ef4444;
	}

	.text-gray-400 {
		color: #9ca3af;
	}

	.text-xs {
		font-size: 0.75rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Responsive
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1200px) {
		.asset-manager {
			grid-template-columns: 220px 1fr;
		}

		.asset-manager:has(.details-panel) {
			grid-template-columns: 220px 1fr 300px;
		}
	}

	@media (max-width: 1024px) {
		.asset-manager {
			grid-template-columns: 1fr;
		}

		.asset-manager:has(.details-panel) {
			grid-template-columns: 1fr;
		}

		.folder-sidebar {
			display: none;
		}

		.details-panel {
			position: fixed;
			inset: 0;
			z-index: 100;
			border-left: none;
		}
	}

	@media (max-width: 768px) {
		.toolbar {
			flex-direction: column;
			align-items: stretch;
			gap: 0.75rem;
		}

		.toolbar-actions {
			flex-wrap: wrap;
		}

		.search-input {
			width: 100%;
		}

		.asset-grid {
			grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		}

		.asset-list .asset-item {
			grid-template-columns: 40px 1fr;
		}

		.asset-size,
		.asset-date {
			display: none;
		}
	}
</style>
