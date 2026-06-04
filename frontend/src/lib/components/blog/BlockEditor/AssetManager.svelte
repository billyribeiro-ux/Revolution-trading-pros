<!--
/**
 * AssetManager - Digital Asset Manager
 * Enterprise-grade DAM with:
 * - Grid/List view modes
 * - Search by filename, tags, metadata
 * - Filter by type (image, video, audio, document)
 * - Drag-drop upload with progress
 * - Folder navigation
 * - Asset details sidebar (preview, metadata, usage)
 * - Image editing integration (crop, resize)
 * - Bulk operations (select, delete, move, tag)
 *
 */
-->

<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import { fade, fly, slide, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { flip } from 'svelte/animate';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import Icon from '$lib/components/Icon.svelte';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { logger } from '$lib/utils/logger';
	import DamHeader from './asset-manager/DamHeader.svelte';
	import DamTabs from './asset-manager/DamTabs.svelte';
	import DamFooter from './asset-manager/DamFooter.svelte';
	import DragOverlay from './asset-manager/DragOverlay.svelte';
	import LoadingState from './asset-manager/LoadingState.svelte';
	import EmptyState from './asset-manager/EmptyState.svelte';
	import DropZone from './asset-manager/DropZone.svelte';
	import FoldersSidebar from './asset-manager/FoldersSidebar.svelte';

	// Types

	interface Asset {
		id: string;
		folder_id: string | null;
		filename: string;
		original_filename?: string;
		mime_type: string;
		file_size: number;
		cdn_url: string;
		width: number | null;
		height: number | null;
		blurhash: string | null;
		thumbnail_url: string | null;
		title: string | null;
		alt_text: string | null;
		caption: string | null;
		description: string | null;
		credits: string | null;
		seo_title: string | null;
		seo_description: string | null;
		tags: string[] | null;
		usage_count: number;
		created_at: string;
		updated_at?: string;
	}

	interface Folder {
		id: string;
		name: string;
		slug: string;
		parent_id: string | null;
		path: string;
		depth: number;
		color: string | null;
		icon: string | null;
		asset_count: number;
	}

	interface AssetUsage {
		id: string;
		content_type: string;
		content_id: string;
		content_title: string | null;
		content_slug: string | null;
		field_name: string;
	}

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSelect: (asset: Asset) => void;
		allowMultiple?: boolean;
		acceptTypes?: ('image' | 'video' | 'audio' | 'document')[];
		initialFolder?: string | null;
	}

	let props: Props = $props();
	const isOpen = $derived(props.isOpen);
	const onClose = $derived(props.onClose);
	const onSelect = $derived(props.onSelect);
	const allowMultiple = $derived(props.allowMultiple ?? false);
	const acceptTypes: ('image' | 'video' | 'audio' | 'document')[] = $derived(
		props.acceptTypes ?? ['image', 'video', 'audio', 'document']
	);
	const initialFolder = $derived(props.initialFolder ?? null);

	// State

	// View state
	let viewMode = $state<'grid' | 'list'>('grid');
	let activeTab = $state<'library' | 'upload'>('library');
	let sidebarOpen = $state(false);

	// Navigation
	let currentFolderId: string | null = $derived(initialFolder);
	let folders = $state<Folder[]>([]);
	let folderStack = $state<Folder[]>([]);

	// Assets
	let assets = $state<Asset[]>([]);
	let selectedAssets = new SvelteSet<string>();
	let selectedAsset = $state<Asset | null>(null);
	let assetUsage = $state<AssetUsage[]>([]);

	// Loading states
	let isLoading = $state(false);
	let isLoadingMore = $state(false);
	let isLoadingUsage = $state(false);

	// Confirmation modal state (replaces native confirm())
	let showDeleteAssetModal = $state(false);
	let pendingDeleteAssetId = $state<string | null>(null);
	let showBulkDeleteAssetsModal = $state(false);

	// Pagination
	let currentPage = $state(1);
	let totalAssets = $state(0);
	let hasMore = $state(false);

	// Filters
	let searchQuery = $state('');
	let typeFilter = $state<string>('all');
	let sortBy = $state<'created_at' | 'filename' | 'file_size'>('created_at');
	let sortOrder = $state<'asc' | 'desc'>('desc');
	let tagFilter = $state<string[]>([]);

	// Upload state
	let isDragging = $state(false);
	let dragCounter = $state(0);
	let uploadQueue = $state<UploadItem[]>([]);
	let isUploading = $state(false);

	interface UploadItem {
		id: string;
		file: File;
		status: 'pending' | 'uploading' | 'complete' | 'error';
		progress: number;
		error?: string;
		previewUrl?: string;
		asset?: Asset;
	}

	// Metadata editing
	let editingMetadata = $state(false);
	let metadataForm = $state({
		title: '',
		alt_text: '',
		caption: '',
		description: '',
		credits: '',
		seo_title: '',
		seo_description: '',
		tags: ''
	});

	// Debounce timer
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;
	let uploadCompleteTimeout: ReturnType<typeof setTimeout> | null = null;
	let isComponentMounted = false;
	let assetsRequestGeneration = 0;
	let foldersRequestGeneration = 0;
	let tagsRequestGeneration = 0;
	let usageRequestGeneration = 0;
	let assetsAbortController: AbortController | null = null;
	let foldersAbortController: AbortController | null = null;
	let tagsAbortController: AbortController | null = null;
	let usageAbortController: AbortController | null = null;
	const uploadAbortControllers = new SvelteMap<string, AbortController>();

	function clearSearchTimeout() {
		if (searchTimeout === null) return;
		clearTimeout(searchTimeout);
		searchTimeout = null;
	}

	function clearUploadCompleteTimeout() {
		if (uploadCompleteTimeout === null) return;
		clearTimeout(uploadCompleteTimeout);
		uploadCompleteTimeout = null;
	}

	function releaseUploadPreview(item: UploadItem) {
		if (!item.previewUrl) return;
		URL.revokeObjectURL(item.previewUrl);
	}

	function releaseUploadQueuePreviews(items = uploadQueue) {
		items.forEach(releaseUploadPreview);
	}

	function clearUploadQueue() {
		clearUploadCompleteTimeout();
		abortUploadRequests();
		releaseUploadQueuePreviews();
		uploadQueue = [];
	}

	function abortLibraryRequests() {
		assetsRequestGeneration += 1;
		foldersRequestGeneration += 1;
		tagsRequestGeneration += 1;
		usageRequestGeneration += 1;
		assetsAbortController?.abort();
		foldersAbortController?.abort();
		tagsAbortController?.abort();
		usageAbortController?.abort();
		assetsAbortController = null;
		foldersAbortController = null;
		tagsAbortController = null;
		usageAbortController = null;
		isLoading = false;
		isLoadingMore = false;
		isLoadingUsage = false;
	}

	function abortUploadRequests() {
		for (const controller of uploadAbortControllers.values()) {
			controller.abort();
		}
		uploadAbortControllers.clear();
		isUploading = false;
	}

	function updateUploadItem(itemId: string, update: (item: UploadItem) => void) {
		const index = uploadQueue.findIndex((item) => item.id === itemId);
		if (index === -1) return false;
		update(uploadQueue[index]);
		uploadQueue = [...uploadQueue];
		return true;
	}

	// API Functions

	async function fetchAssets(reset = false) {
		assetsAbortController?.abort();
		const controller = new AbortController();
		assetsAbortController = controller;
		const requestGeneration = ++assetsRequestGeneration;

		if (reset) {
			currentPage = 1;
			assets = [];
		}

		isLoading = reset;
		isLoadingMore = !reset;

		try {
			const params = new URLSearchParams({
				page: currentPage.toString(),
				per_page: '48',
				sort_by: sortBy,
				sort_order: sortOrder
			});

			if (currentFolderId) params.append('folder_id', currentFolderId);
			if (searchQuery) params.append('search', searchQuery);
			if (typeFilter !== 'all') params.append('type', typeFilter);
			if (tagFilter.length > 0) params.append('tags', tagFilter.join(','));

			const response = await fetch(`/api/cms/assets?${params}`, {
				credentials: 'include',
				signal: controller.signal
			});

			if (!response.ok) throw new Error('Failed to fetch assets');

			const data = await response.json();
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== assetsRequestGeneration
			) {
				return;
			}

			if (reset) {
				assets = data.data;
			} else {
				assets = [...assets, ...data.data];
			}

			totalAssets = data.meta.total;
			// totalPages available in data.meta.total_pages if needed for pagination UI
			hasMore = data.meta.has_more;
		} catch (error) {
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== assetsRequestGeneration
			) {
				return;
			}
			logger.error('[AssetManager] Fetch assets failed', { error });
		} finally {
			if (assetsAbortController === controller) {
				assetsAbortController = null;
			}
			if (isComponentMounted && requestGeneration === assetsRequestGeneration) {
				isLoading = false;
				isLoadingMore = false;
			}
		}
	}

	async function fetchFolders() {
		foldersAbortController?.abort();
		const controller = new AbortController();
		foldersAbortController = controller;
		const requestGeneration = ++foldersRequestGeneration;

		try {
			const response = await fetch('/api/cms/assets/folders', {
				credentials: 'include',
				signal: controller.signal
			});
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== foldersRequestGeneration
			) {
				return;
			}
			if (response.ok) {
				folders = await response.json();
			}
		} catch (error) {
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== foldersRequestGeneration
			) {
				return;
			}
			logger.error('[AssetManager] Fetch folders failed', { error });
		} finally {
			if (foldersAbortController === controller) {
				foldersAbortController = null;
			}
		}
	}

	async function fetchTags() {
		tagsAbortController?.abort();
		const controller = new AbortController();
		tagsAbortController = controller;
		const requestGeneration = ++tagsRequestGeneration;

		try {
			const response = await fetch('/api/cms/assets/tags', {
				credentials: 'include',
				signal: controller.signal
			});
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== tagsRequestGeneration
			) {
				return;
			}
			if (response.ok) {
				// Tags loaded for future tag filtering UI
				await response.json();
			}
		} catch (error) {
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== tagsRequestGeneration
			) {
				return;
			}
			logger.error('[AssetManager] Fetch tags failed', { error });
		} finally {
			if (tagsAbortController === controller) {
				tagsAbortController = null;
			}
		}
	}

	async function loadAssetManager() {
		await Promise.all([fetchAssets(true), fetchFolders(), fetchTags()]);
	}

	async function fetchAssetUsage(assetId: string) {
		usageAbortController?.abort();
		const controller = new AbortController();
		usageAbortController = controller;
		const requestGeneration = ++usageRequestGeneration;
		isLoadingUsage = true;
		try {
			const response = await fetch(`/api/cms/assets/${assetId}/usage`, {
				credentials: 'include',
				signal: controller.signal
			});
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== usageRequestGeneration ||
				selectedAsset?.id !== assetId
			) {
				return;
			}
			if (response.ok) {
				assetUsage = await response.json();
			}
		} catch (error) {
			if (
				!isComponentMounted ||
				controller.signal.aborted ||
				requestGeneration !== usageRequestGeneration
			) {
				return;
			}
			logger.error('[AssetManager] Fetch usage failed', { error });
		} finally {
			if (usageAbortController === controller) {
				usageAbortController = null;
			}
			if (isComponentMounted && requestGeneration === usageRequestGeneration) {
				isLoadingUsage = false;
			}
		}
	}

	// Fetch recent assets - available for future use
	// async function fetchRecentAssets() {
	// 	try {
	// 		const response = await fetch('/api/cms/assets/recent?limit=12', {
	// 			credentials: 'include'
	// 		});
	// 		if (response.ok) {
	// 			return await response.json();
	// 		}
	// 	} catch (error) {
	// 		console.error('Failed to fetch recent:', error);
	// 	}
	// 	return [];
	// }

	async function updateAssetMetadata(assetId: string) {
		try {
			const response = await fetch(`/api/cms/assets/${assetId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					title: metadataForm.title || null,
					alt_text: metadataForm.alt_text || null,
					caption: metadataForm.caption || null,
					description: metadataForm.description || null,
					credits: metadataForm.credits || null,
					seo_title: metadataForm.seo_title || null,
					seo_description: metadataForm.seo_description || null,
					tags: metadataForm.tags
						? metadataForm.tags
								.split(',')
								.map((t) => t.trim())
								.filter(Boolean)
						: null
				})
			});

			if (response.ok) {
				const updated = await response.json();
				if (!isComponentMounted) return;
				// Update in list
				const idx = assets.findIndex((a) => a.id === assetId);
				if (idx !== -1) {
					assets[idx] = updated;
					assets = [...assets];
				}
				if (selectedAsset?.id === assetId) {
					selectedAsset = updated;
				}
				editingMetadata = false;
			}
		} catch (error) {
			logger.error('[AssetManager] Update metadata failed', { error });
		}
	}

	function deleteAsset(assetId: string) {
		pendingDeleteAssetId = assetId;
		showDeleteAssetModal = true;
	}

	async function confirmDeleteAsset() {
		const assetId = pendingDeleteAssetId;
		if (!assetId) return;
		showDeleteAssetModal = false;
		pendingDeleteAssetId = null;

		try {
			const response = await fetch(`/api/cms/assets/${assetId}`, {
				method: 'DELETE',
				credentials: 'include'
			});

			if (response.ok) {
				if (!isComponentMounted) return;
				assets = assets.filter((a) => a.id !== assetId);
				selectedAssets.delete(assetId);
				if (selectedAsset?.id === assetId) {
					selectedAsset = null;
					sidebarOpen = false;
				}
			}
		} catch (error) {
			logger.error('[AssetManager] Delete asset failed', { error });
		}
	}

	function cancelDeleteAsset() {
		showDeleteAssetModal = false;
		pendingDeleteAssetId = null;
	}

	function bulkDelete() {
		if (selectedAssets.size === 0) return;
		showBulkDeleteAssetsModal = true;
	}

	async function confirmBulkDeleteAssets() {
		showBulkDeleteAssetsModal = false;
		try {
			const response = await fetch('/api/cms/assets/bulk/delete', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					asset_ids: Array.from(selectedAssets)
				})
			});

			if (response.ok) {
				if (!isComponentMounted) return;
				assets = assets.filter((a) => !selectedAssets.has(a.id));
				selectedAssets.clear();
				selectedAsset = null;
				sidebarOpen = false;
			}
		} catch (error) {
			logger.error('[AssetManager] Bulk delete failed', { error });
		}
	}

	function cancelBulkDeleteAssets() {
		showBulkDeleteAssetsModal = false;
	}

	// Move to folder - available for future use
	// async function moveToFolder(targetFolderId: string | null) {
	// 	if (selectedAssets.size === 0) return;
	//
	// 	try {
	// 		const response = await fetch('/api/cms/assets/bulk/move', {
	// 			method: 'POST',
	// 			headers: { 'Content-Type': 'application/json' },
	// 			credentials: 'include',
	// 			body: JSON.stringify({
	// 				asset_ids: Array.from(selectedAssets),
	// 				target_folder_id: targetFolderId
	// 			})
	// 		});
	//
	// 		if (response.ok) {
	// 			await fetchAssets(true);
	// 			selectedAssets.clear();
	// 		}
	// 	} catch (error) {
	// 		console.error('Failed to move assets:', error);
	// 	}
	// }

	// Upload Functions

	async function uploadFile(item: UploadItem) {
		const controller = new AbortController();
		uploadAbortControllers.set(item.id, controller);

		if (
			!updateUploadItem(item.id, (queuedItem) => {
				queuedItem.status = 'uploading';
				queuedItem.error = undefined;
			})
		) {
			uploadAbortControllers.delete(item.id);
			return;
		}

		try {
			// Get presigned URL
			const presignResponse = await fetch('/api/admin/media/presigned-upload', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					filename: item.file.name,
					content_type: item.file.type,
					size: item.file.size,
					collection: currentFolderId ? `folder-${currentFolderId}` : 'uploads'
				}),
				signal: controller.signal
			});

			if (!presignResponse.ok) throw new Error('Failed to get upload URL');
			const { data: presignData } = await presignResponse.json();
			if (!isComponentMounted || controller.signal.aborted) return;

			// Upload to R2
			const uploadResponse = await fetch(presignData.upload_url, {
				method: 'PUT',
				body: item.file,
				headers: {
					'Content-Type': item.file.type
				},
				signal: controller.signal
			});

			if (!uploadResponse.ok) throw new Error('Upload failed');
			if (!isComponentMounted || controller.signal.aborted) return;

			// Get image dimensions if applicable
			let width: number | undefined;
			let height: number | undefined;

			if (item.file.type.startsWith('image/')) {
				const dimensions = await getImageDimensions(item.file);
				width = dimensions.width;
				height = dimensions.height;
			}
			if (!isComponentMounted || controller.signal.aborted) return;

			// Confirm upload
			const confirmResponse = await fetch('/api/cms/assets/upload', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					file_key: presignData.file_key,
					original_filename: item.file.name,
					mime_type: item.file.type,
					file_size: item.file.size,
					folder_id: currentFolderId,
					width,
					height
				}),
				signal: controller.signal
			});

			if (!confirmResponse.ok) throw new Error('Failed to confirm upload');

			const asset = await confirmResponse.json();
			if (!isComponentMounted || controller.signal.aborted) return;

			if (
				!updateUploadItem(item.id, (queuedItem) => {
					queuedItem.status = 'complete';
					queuedItem.progress = 100;
					queuedItem.asset = asset;
				})
			) {
				return;
			}

			// Add to assets list
			assets = [asset, ...assets];
			totalAssets += 1;
		} catch (error) {
			if (!isComponentMounted || controller.signal.aborted) return;
			updateUploadItem(item.id, (queuedItem) => {
				queuedItem.status = 'error';
				queuedItem.error = error instanceof Error ? error.message : 'Upload failed';
			});
		} finally {
			if (uploadAbortControllers.get(item.id) === controller) {
				uploadAbortControllers.delete(item.id);
			}
		}
	}

	async function startUpload() {
		if (uploadQueue.length === 0 || isUploading) return;

		isUploading = true;

		const pendingItems = uploadQueue.filter((item) => item.status === 'pending');
		for (const item of pendingItems) {
			await uploadFile(item);
			if (!isComponentMounted) return;
		}

		isUploading = false;

		// Switch to library if all complete
		const allComplete = uploadQueue.every((item) => item.status === 'complete');
		if (allComplete) {
			clearUploadCompleteTimeout();
			uploadCompleteTimeout = setTimeout(() => {
				if (!isComponentMounted) return;
				activeTab = 'library';
				clearUploadQueue();
			}, 1000);
		}
	}

	function getImageDimensions(file: File): Promise<{ width: number; height: number }> {
		return new Promise((resolve) => {
			const img = new Image();
			const objectUrl = URL.createObjectURL(file);
			img.onload = () => {
				resolve({ width: img.naturalWidth, height: img.naturalHeight });
				URL.revokeObjectURL(objectUrl);
			};
			img.onerror = () => {
				URL.revokeObjectURL(objectUrl);
				resolve({ width: 0, height: 0 });
			};
			img.src = objectUrl;
		});
	}

	// Event Handlers

	function handleDragEnter(e: DragEvent) {
		e.preventDefault();
		dragCounter++;
		if (e.dataTransfer?.types.includes('Files')) {
			isDragging = true;
		}
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		dragCounter--;
		if (dragCounter === 0) {
			isDragging = false;
		}
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
		dragCounter = 0;

		const files = Array.from(e.dataTransfer?.files || []);
		addFilesToQueue(files);
		activeTab = 'upload';
	}

	function handleFileInput(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files) {
			addFilesToQueue(Array.from(input.files));
			activeTab = 'upload';
		}
		input.value = '';
	}

	function addFilesToQueue(files: File[]) {
		const validFiles = files.filter((file) => {
			const type = file.type.split('/')[0];
			if (type === 'image' && acceptTypes.includes('image')) return true;
			if (type === 'video' && acceptTypes.includes('video')) return true;
			if (type === 'audio' && acceptTypes.includes('audio')) return true;
			if (file.type.startsWith('application/') && acceptTypes.includes('document')) return true;
			return false;
		});

		const newItems: UploadItem[] = validFiles.map((file) => ({
			id: crypto.randomUUID(),
			file,
			status: 'pending',
			progress: 0,
			previewUrl: file.type.startsWith('image/') ? URL.createObjectURL(file) : undefined
		}));

		uploadQueue = [...uploadQueue, ...newItems];
	}

	function removeFromQueue(id: string) {
		const item = uploadQueue.find((u) => u.id === id);
		if (item) releaseUploadPreview(item);
		uploadAbortControllers.get(id)?.abort();
		uploadAbortControllers.delete(id);
		uploadQueue = uploadQueue.filter((u) => u.id !== id);
		if (uploadQueue.length === 0) clearUploadCompleteTimeout();
	}

	function toggleAssetSelection(asset: Asset) {
		if (allowMultiple) {
			if (selectedAssets.has(asset.id)) {
				selectedAssets.delete(asset.id);
			} else {
				selectedAssets.add(asset.id);
			}
		} else {
			selectedAssets.clear();
			selectedAssets.add(asset.id);
		}
	}

	function openAssetDetails(asset: Asset) {
		selectedAsset = asset;
		assetUsage = [];
		sidebarOpen = true;
		void fetchAssetUsage(asset.id);

		// Populate metadata form
		metadataForm = {
			title: asset.title || '',
			alt_text: asset.alt_text || '',
			caption: asset.caption || '',
			description: asset.description || '',
			credits: asset.credits || '',
			seo_title: asset.seo_title || '',
			seo_description: asset.seo_description || '',
			tags: asset.tags?.join(', ') || ''
		};
	}

	function handleInsert() {
		const selected = assets.filter((a) => selectedAssets.has(a.id));
		if (selected.length > 0) {
			if (allowMultiple) {
				selected.forEach((a) => onSelect(a));
			} else {
				onSelect(selected[0]);
			}
			onClose();
		}
	}

	function handleDoubleClick(asset: Asset) {
		onSelect(asset);
		onClose();
	}

	function navigateToFolder(folder: Folder | null) {
		if (folder) {
			currentFolderId = folder.id;
			folderStack = [...folderStack, folder];
		} else {
			currentFolderId = null;
			folderStack = [];
		}
		void fetchAssets(true);
	}

	// Navigate up - available for future use
	// function navigateUp() {
	// 	if (folderStack.length > 1) {
	// 		folderStack = folderStack.slice(0, -1);
	// 		currentFolderId = folderStack[folderStack.length - 1]?.id || null;
	// 	} else {
	// 		folderStack = [];
	// 		currentFolderId = null;
	// 	}
	// 	fetchAssets(true);
	// }

	function handleSearch() {
		clearSearchTimeout();
		searchTimeout = setTimeout(() => {
			if (!isComponentMounted || !isOpen) return;
			void fetchAssets(true);
		}, 300);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (sidebarOpen) {
				sidebarOpen = false;
			} else {
				onClose();
			}
		}
	}

	// Helpers

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
		return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
	}

	function getAssetType(mimeType: string): string {
		if (mimeType.startsWith('image/')) return 'image';
		if (mimeType.startsWith('video/')) return 'video';
		if (mimeType.startsWith('audio/')) return 'audio';
		return 'document';
	}

	// Get type icon - available for future use
	// function getTypeIcon(type: string): string {
	// 	switch (type) {
	// 		case 'image': return 'image';
	// 		case 'video': return 'video';
	// 		case 'audio': return 'audio';
	// 		case 'document': return 'file-text';
	// 		default: return 'file';
	// 	}
	// }

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	const mountAssetManager: Attachment<HTMLDivElement> = () => {
		void loadAssetManager();

		return () => {
			clearSearchTimeout();
			abortLibraryRequests();
			isDragging = false;
			dragCounter = 0;
		};
	};

	onMount(() => {
		isComponentMounted = true;
	});

	// Cleanup
	onDestroy(() => {
		isComponentMounted = false;
		clearSearchTimeout();
		abortLibraryRequests();
		abortUploadRequests();
		clearUploadCompleteTimeout();
		releaseUploadQueuePreviews();
	});

	// Get child folders for current location
	const childFolders = $derived(folders.filter((f) => f.parent_id === currentFolderId));
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div
		class="dam-overlay"
		{@attach mountAssetManager}
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="button"
		tabindex="-1"
		aria-label="Close Asset Manager"
		transition:fade={{ duration: 200 }}
	>
		<div
			class="dam-modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Enter' && e.stopPropagation()}
			ondragenter={handleDragEnter}
			ondragleave={handleDragLeave}
			ondragover={handleDragOver}
			ondrop={handleDrop}
			role="dialog"
			aria-modal="true"
			aria-label="Asset Manager"
			tabindex="-1"
			transition:fly={{ y: 20, duration: 300, easing: cubicOut }}
		>
			<!-- Header -->
			<DamHeader {totalAssets} {acceptTypes} {onClose} onFileInput={handleFileInput} />

			<!-- Tabs -->
			<DamTabs bind:activeTab uploadQueueCount={uploadQueue.length} />

			<!-- Main Content -->
			<div class="dam-body">
				{#if activeTab === 'library'}
					<!-- Toolbar -->
					<div class="toolbar">
						<div class="toolbar-left">
							<!-- Search -->
							<div class="search-box">
								<Icon name="IconSearch" size={16} />
								<input
									type="text"
									placeholder="Search assets..."
									bind:value={searchQuery}
									oninput={handleSearch}
								/>
								{#if searchQuery}
									<button
										class="clear-search"
										onclick={() => {
											searchQuery = '';
											fetchAssets(true);
										}}
										aria-label="Clear search"
									>
										<Icon name="IconX" size={14} />
									</button>
								{/if}
							</div>

							<!-- Type Filter -->
							<select
								class="filter-select"
								bind:value={typeFilter}
								onchange={() => fetchAssets(true)}
							>
								<option value="all">All Types</option>
								<option value="image">Images</option>
								<option value="video">Videos</option>
								<option value="audio">Audio</option>
								<option value="document">Documents</option>
							</select>

							<!-- Sort -->
							<select class="filter-select" bind:value={sortBy} onchange={() => fetchAssets(true)}>
								<option value="created_at">Date Added</option>
								<option value="filename">Name</option>
								<option value="file_size">Size</option>
							</select>

							<button
								class="sort-btn"
								onclick={() => {
									sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
									fetchAssets(true);
								}}
								title={sortOrder === 'asc' ? 'Ascending' : 'Descending'}
								style="transform: {sortOrder === 'asc' ? 'rotate(180deg)' : 'rotate(0)'}"
							>
								<Icon name="IconArrowDown" size={16} />
							</button>
						</div>

						<div class="toolbar-right">
							<!-- Bulk Actions -->
							{#if selectedAssets.size > 0}
								<div class="bulk-actions" transition:fade={{ duration: 150 }}>
									<span class="selected-count">{selectedAssets.size} selected</span>
									<button class="bulk-btn danger" onclick={bulkDelete} title="Delete selected">
										<Icon name="IconTrash" size={14} />
									</button>
									<button
										class="bulk-btn"
										onclick={() => selectedAssets.clear()}
										title="Clear selection"
									>
										<Icon name="IconX" size={14} />
									</button>
								</div>
							{/if}

							<!-- View Toggle -->
							<div class="view-toggle">
								<button
									class:active={viewMode === 'grid'}
									onclick={() => (viewMode = 'grid')}
									title="Grid view"
								>
									<Icon name="IconLayoutGrid" size={16} />
								</button>
								<button
									class:active={viewMode === 'list'}
									onclick={() => (viewMode = 'list')}
									title="List view"
								>
									<Icon name="IconList" size={16} />
								</button>
							</div>
						</div>
					</div>

					<!-- Library Layout -->
					<div class="library-layout" class:sidebar-open={sidebarOpen}>
						<!-- Sidebar - Folders -->
						<FoldersSidebar {folders} {currentFolderId} onNavigate={navigateToFolder} />

						<!-- Main Content -->
						<main class="asset-content">
							<!-- Breadcrumb -->
							{#if folderStack.length > 0}
								<div class="breadcrumb" transition:slide={{ duration: 200 }}>
									<button onclick={() => navigateToFolder(null)}>All Assets</button>
									{#each folderStack as folder, i (folder.id)}
										<Icon name="IconChevronRight" size={12} />
										<button
											onclick={() => {
												folderStack = folderStack.slice(0, i + 1);
												currentFolderId = folder.id;
												fetchAssets(true);
											}}
										>
											{folder.name}
										</button>
									{/each}
								</div>
							{/if}

							<!-- Loading -->
							{#if isLoading}
								<LoadingState />
							{:else if assets.length === 0}
								<!-- Empty State -->
								<EmptyState {acceptTypes} onFileInput={handleFileInput} />
							{:else}
								<!-- Child Folders -->
								{#if childFolders.length > 0 && !searchQuery}
									<div class="folder-grid">
										{#each childFolders as folder (folder.id)}
											<button
												class="folder-card"
												ondblclick={() => navigateToFolder(folder)}
												onclick={() => {}}
											>
												<Icon name="IconFolder" size={24} color={folder.color || '#64748b'} />
												<span class="folder-name">{folder.name}</span>
												<span class="folder-asset-count">{folder.asset_count} items</span>
											</button>
										{/each}
									</div>
								{/if}

								<!-- Asset Grid/List -->
								{#if viewMode === 'grid'}
									<div class="asset-grid">
										{#each assets as asset (asset.id)}
											<div
												class="asset-card"
												class:selected={selectedAssets.has(asset.id)}
												role="button"
												tabindex="0"
												onclick={() => toggleAssetSelection(asset)}
												ondblclick={() => handleDoubleClick(asset)}
												onkeydown={(e) => e.key === 'Enter' && toggleAssetSelection(asset)}
												animate:flip={{ duration: 200 }}
											>
												<div class="asset-preview">
													{#if getAssetType(asset.mime_type) === 'image'}
														<img
															src={asset.thumbnail_url || asset.cdn_url}
															alt={asset.alt_text || asset.filename}
															loading="lazy"
															width="200"
															height="150"
														/>
													{:else if getAssetType(asset.mime_type) === 'video'}
														<div class="type-preview video">
															<Icon name="IconPlayerPlay" size={32} stroke={1.5} />
														</div>
													{:else if getAssetType(asset.mime_type) === 'audio'}
														<div class="type-preview audio">
															<Icon name="IconVolume" size={32} stroke={1.5} />
														</div>
													{:else}
														<div class="type-preview document">
															<Icon name="IconFileText" size={32} stroke={1.5} />
														</div>
													{/if}

													<!-- Selection Indicator -->
													{#if selectedAssets.has(asset.id)}
														<div class="selection-indicator" transition:scale={{ duration: 150 }}>
															<Icon name="IconCheck" size={16} stroke={3} />
														</div>
													{/if}

													<!-- Quick Actions -->
													<div class="quick-actions">
														<button
															class="action-btn"
															onclick={(e) => {
																e.stopPropagation();
																openAssetDetails(asset);
															}}
															title="Details"
														>
															<Icon name="IconInfoCircle" size={14} />
														</button>
													</div>
												</div>
												<div class="asset-info">
													<span class="asset-name" title={asset.filename}>{asset.filename}</span>
													<span class="asset-meta">{formatFileSize(asset.file_size)}</span>
												</div>
											</div>
										{/each}
									</div>
								{:else}
									<!-- List View -->
									<div class="asset-list">
										<table>
											<thead>
												<tr>
													<th class="th-preview"></th>
													<th class="th-name">Name</th>
													<th class="th-type">Type</th>
													<th class="th-size">Size</th>
													<th class="th-date">Date</th>
													<th class="th-actions"></th>
												</tr>
											</thead>
											<tbody>
												{#each assets as asset (asset.id)}
													<tr
														class:selected={selectedAssets.has(asset.id)}
														onclick={() => toggleAssetSelection(asset)}
														ondblclick={() => handleDoubleClick(asset)}
													>
														<td class="td-preview">
															{#if getAssetType(asset.mime_type) === 'image'}
																<img
																	src={asset.thumbnail_url || asset.cdn_url}
																	alt=""
																	width="40"
																	height="40"
																	loading="lazy"
																/>
															{:else}
																<div class="type-icon {getAssetType(asset.mime_type)}">
																	{#if getAssetType(asset.mime_type) === 'video'}
																		<Icon name="IconPlayerPlay" size={20} />
																	{:else if getAssetType(asset.mime_type) === 'audio'}
																		<Icon name="IconVolume" size={20} />
																	{:else}
																		<Icon name="IconFileText" size={20} />
																	{/if}
																</div>
															{/if}
														</td>
														<td class="td-name">
															<span title={asset.filename}>{asset.filename}</span>
														</td>
														<td class="td-type">{getAssetType(asset.mime_type)}</td>
														<td class="td-size">{formatFileSize(asset.file_size)}</td>
														<td class="td-date">{formatDate(asset.created_at)}</td>
														<td class="td-actions">
															<button
																onclick={(e) => {
																	e.stopPropagation();
																	openAssetDetails(asset);
																}}
																title="Details"
															>
																<Icon name="IconInfoCircle" size={14} />
															</button>
														</td>
													</tr>
												{/each}
											</tbody>
										</table>
									</div>
								{/if}

								<!-- Load More -->
								{#if hasMore}
									<div class="load-more">
										<button
											onclick={() => {
												currentPage++;
												fetchAssets(false);
											}}
											disabled={isLoadingMore}
										>
											{isLoadingMore ? 'Loading...' : 'Load More'}
										</button>
									</div>
								{/if}
							{/if}
						</main>

						<!-- Details Sidebar -->
						{#if sidebarOpen && selectedAsset}
							<aside class="details-sidebar" transition:fly={{ x: 300, duration: 200 }}>
								<div class="sidebar-header">
									<h4>Details</h4>
									<button
										class="close-sidebar"
										onclick={() => (sidebarOpen = false)}
										aria-label="Close sidebar"
									>
										<Icon name="IconX" size={16} />
									</button>
								</div>

								<!-- Preview -->
								<div class="detail-preview">
									{#if getAssetType(selectedAsset.mime_type) === 'image'}
										<img
											src={selectedAsset.cdn_url}
											alt={selectedAsset.alt_text || selectedAsset.filename}
											width="400"
											height="250"
										/>
									{:else if getAssetType(selectedAsset.mime_type) === 'video'}
										<!-- svelte-ignore a11y_media_has_caption -->
										<video src={selectedAsset.cdn_url} controls></video>
									{:else}
										<div class="type-preview-large {getAssetType(selectedAsset.mime_type)}">
											{#if getAssetType(selectedAsset.mime_type) === 'audio'}
												<Icon name="IconVolume" size={48} stroke={1.5} />
											{:else}
												<Icon name="IconFileText" size={48} stroke={1.5} />
											{/if}
										</div>
									{/if}
								</div>

								<!-- File Info -->
								<div class="detail-section">
									<h5>File Information</h5>
									<dl class="info-list">
										<dt>Filename</dt>
										<dd>{selectedAsset.filename}</dd>
										<dt>Type</dt>
										<dd>{selectedAsset.mime_type}</dd>
										<dt>Size</dt>
										<dd>{formatFileSize(selectedAsset.file_size)}</dd>
										{#if selectedAsset.width && selectedAsset.height}
											<dt>Dimensions</dt>
											<dd>{selectedAsset.width} x {selectedAsset.height}</dd>
										{/if}
										<dt>Uploaded</dt>
										<dd>{formatDate(selectedAsset.created_at)}</dd>
									</dl>
								</div>

								<!-- Metadata -->
								<div class="detail-section">
									<div class="section-header">
										<h5>Metadata</h5>
										<button class="edit-btn" onclick={() => (editingMetadata = !editingMetadata)}>
											{editingMetadata ? 'Cancel' : 'Edit'}
										</button>
									</div>

									{#if editingMetadata}
										<div class="metadata-form" transition:slide={{ duration: 200 }}>
											<div class="field">
												<label for="meta-title">Title</label>
												<input id="meta-title" type="text" bind:value={metadataForm.title} />
											</div>
											<div class="field">
												<label for="meta-alt">Alt Text</label>
												<input
													id="meta-alt"
													type="text"
													bind:value={metadataForm.alt_text}
													placeholder="Describe the image"
												/>
											</div>
											<div class="field">
												<label for="meta-caption">Caption</label>
												<textarea id="meta-caption" bind:value={metadataForm.caption}></textarea>
											</div>
											<div class="field">
												<label for="meta-credits">Credits</label>
												<input
													id="meta-credits"
													type="text"
													bind:value={metadataForm.credits}
													placeholder="Photo credit or attribution"
												/>
											</div>
											<div class="field">
												<label for="meta-seo-title">SEO Title</label>
												<input
													id="meta-seo-title"
													type="text"
													bind:value={metadataForm.seo_title}
												/>
											</div>
											<div class="field">
												<label for="meta-seo-description">SEO Description</label>
												<input
													id="meta-seo-description"
													type="text"
													bind:value={metadataForm.seo_description}
												/>
											</div>
											<div class="field">
												<label for="meta-tags">Tags (comma separated)</label>
												<input id="meta-tags" type="text" bind:value={metadataForm.tags} />
											</div>
											<button
												class="save-btn"
												onclick={() => selectedAsset && updateAssetMetadata(selectedAsset.id)}
											>
												Save Changes
											</button>
										</div>
									{:else}
										<dl class="info-list">
											<dt>Title</dt>
											<dd>{selectedAsset.title || '-'}</dd>
											<dt>Alt Text</dt>
											<dd>{selectedAsset.alt_text || '-'}</dd>
											<dt>Caption</dt>
											<dd>{selectedAsset.caption || '-'}</dd>
											<dt>Credits</dt>
											<dd>{selectedAsset.credits || '-'}</dd>
											{#if selectedAsset.tags && selectedAsset.tags.length > 0}
												<dt>Tags</dt>
												<dd>
													<div class="tags">
														{#each selectedAsset.tags as tag (tag)}
															<span class="tag">{tag}</span>
														{/each}
													</div>
												</dd>
											{/if}
										</dl>
									{/if}
								</div>

								<!-- Usage -->
								<div class="detail-section">
									<h5>Used In ({assetUsage.length})</h5>
									{#if isLoadingUsage}
										<div class="loading-small">Loading...</div>
									{:else if assetUsage.length === 0}
										<p class="no-usage">Not used in any content</p>
									{:else}
										<ul class="usage-list">
											{#each assetUsage as usage (usage.id)}
												<li>
													<span class="usage-type">{usage.content_type}</span>
													<span class="usage-title">{usage.content_title || usage.content_id}</span>
												</li>
											{/each}
										</ul>
									{/if}
								</div>

								<!-- Actions -->
								<div class="detail-actions">
									<a href={selectedAsset.cdn_url} target="_blank" class="action-link">
										<Icon name="IconExternalLink" size={14} />
										Open Original
									</a>
									<button
										class="action-link danger"
										onclick={() => selectedAsset && deleteAsset(selectedAsset.id)}
									>
										<Icon name="IconTrash" size={14} />
										Delete Asset
									</button>
								</div>
							</aside>
						{/if}
					</div>
				{:else}
					<!-- Upload Tab -->
					<div class="upload-area">
						<DropZone {isDragging} {acceptTypes} onFileInput={handleFileInput} />

						{#if uploadQueue.length > 0}
							<div class="upload-queue" transition:slide={{ duration: 200 }}>
								<div class="queue-header">
									<h4>Upload Queue ({uploadQueue.length})</h4>
									{#if !isUploading}
										<button class="clear-queue" onclick={clearUploadQueue}>Clear All</button>
									{/if}
								</div>
								<ul class="queue-list">
									{#each uploadQueue as item (item.id)}
										<li
											class="queue-item"
											class:complete={item.status === 'complete'}
											class:error={item.status === 'error'}
										>
											<div class="item-preview">
												{#if item.previewUrl}
													<img src={item.previewUrl} alt="" width="40" height="40" />
												{:else}
													<div class="file-icon">
														<Icon name="IconFile" size={20} />
													</div>
												{/if}
											</div>
											<div class="item-info">
												<span class="item-name">{item.file.name}</span>
												<span class="item-size">{formatFileSize(item.file.size)}</span>
												{#if item.status === 'uploading'}
													<div class="progress-bar">
														<div class="progress-fill indeterminate"></div>
													</div>
												{:else if item.status === 'error'}
													<span class="error-text">{item.error}</span>
												{/if}
											</div>
											<div class="item-status">
												{#if item.status === 'complete'}
													<Icon name="IconCheck" size={18} color="#22c55e" />
												{:else if item.status === 'error'}
													<Icon name="IconCircleX" size={18} color="#ef4444" />
												{:else if item.status === 'uploading'}
													<div class="spinner small"></div>
												{:else}
													<button
														class="remove-item"
														onclick={() => removeFromQueue(item.id)}
														aria-label="Remove from queue"
													>
														<Icon name="IconX" size={14} />
													</button>
												{/if}
											</div>
										</li>
									{/each}
								</ul>
								<button
									class="start-upload-btn"
									onclick={startUpload}
									disabled={isUploading || uploadQueue.every((i) => i.status !== 'pending')}
								>
									{#if isUploading}
										Uploading...
									{:else}
										Start Upload
									{/if}
								</button>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<DamFooter selectedCount={selectedAssets.size} {onClose} onInsert={handleInsert} />

			<!-- Drag Overlay -->
			{#if isDragging}
				<DragOverlay />
			{/if}
		</div>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteAssetModal}
	title="Delete asset?"
	message="Are you sure you want to delete this asset? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteAsset}
	onCancel={cancelDeleteAsset}
/>

<ConfirmationModal
	isOpen={showBulkDeleteAssetsModal}
	title="Delete assets?"
	message={`Delete ${selectedAssets.size} asset(s)? This action cannot be undone.`}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmBulkDeleteAssets}
	onCancel={cancelBulkDeleteAssets}
/>

<style>
	/* ========================================================================== */
	/* Modal Container */
	/* ========================================================================== */

	.dam-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.dam-modal {
		width: 100%;
		max-width: 1400px;
		height: 90vh;
		max-height: 900px;
		background: #0f172a;
		border-radius: 16px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		border: 1px solid rgba(255, 255, 255, 0.1);
		position: relative;
	}

	/* Header CSS extracted with DamHeader (R6-C #1). */

	/* Tabs CSS extracted with DamTabs (R6-C #2). */

	/* ========================================================================== */
	/* Body */
	/* ========================================================================== */

	.dam-body {
		flex: 1;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	/* ========================================================================== */
	/* Toolbar */
	/* ========================================================================== */

	.toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		gap: 1rem;
		flex-wrap: wrap;
	}

	.toolbar-left,
	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		min-width: 240px;
	}

	.search-box :global(svg) {
		color: #64748b;
		flex-shrink: 0;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		color: #f1f5f9;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.clear-search {
		background: none;
		border: none;
		padding: 0;
		color: #64748b;
		cursor: pointer;
		display: flex;
	}

	.filter-select {
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		font-size: 0.875rem;
		color: #f1f5f9;
		cursor: pointer;
	}

	.filter-select option {
		background: #1e293b;
		color: #f1f5f9;
	}

	.sort-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.sort-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.sort-btn :global(svg) {
		transition: transform 0.2s;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding-right: 0.75rem;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		margin-right: 0.25rem;
	}

	.selected-count {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.bulk-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.05);
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.bulk-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.bulk-btn.danger:hover {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.view-toggle {
		display: flex;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		overflow: hidden;
	}

	.view-toggle button {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.view-toggle button:not(:last-child) {
		border-right: 1px solid rgba(255, 255, 255, 0.1);
	}

	.view-toggle button.active {
		background: #3b82f6;
		color: white;
	}

	/* ========================================================================== */
	/* Library Layout */
	/* ========================================================================== */

	.library-layout {
		flex: 1;
		display: flex;
		overflow: hidden;
	}

	.library-layout.sidebar-open .asset-content {
		margin-right: 320px;
	}

	/* Folders Sidebar CSS extracted with FoldersSidebar (R6-C #8).
	   Base .sidebar-header + .sidebar-header h4 retained — still used by details-sidebar. */

	.sidebar-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.sidebar-header h4 {
		margin: 0;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Asset Content */
	.asset-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		transition: margin-right 0.2s;
	}

	/* Breadcrumb */
	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		margin-bottom: 1rem;
		font-size: 0.8125rem;
	}

	.breadcrumb button {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0;
		transition: color 0.2s;
	}

	.breadcrumb button:hover {
		color: #3b82f6;
	}

	.breadcrumb button:last-child {
		color: #f1f5f9;
	}

	.breadcrumb :global(svg) {
		color: #475569;
	}

	/* Loading & Empty States CSS extracted with LoadingState / EmptyState (R6-C #5+#6). */

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(59, 130, 246, 0.2);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner.small {
		width: 18px;
		height: 18px;
		border-width: 2px;
	}

	/* Folder Grid */
	.folder-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.folder-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.folder-card:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.15);
	}

	.folder-name {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e2e8f0;
		text-align: center;
	}

	.folder-asset-count {
		font-size: 0.6875rem;
		color: #64748b;
	}

	/* Asset Grid */
	.asset-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
		gap: 1rem;
	}

	.asset-card {
		background: rgba(255, 255, 255, 0.03);
		border: 2px solid transparent;
		border-radius: 10px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s;
	}

	.asset-card:hover {
		border-color: rgba(255, 255, 255, 0.15);
	}

	.asset-card.selected {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	.asset-preview {
		position: relative;
		aspect-ratio: 4/3;
		background: #1e293b;
		overflow: hidden;
	}

	.asset-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.type-preview {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.type-preview.video {
		background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%);
	}
	.type-preview.audio {
		background: linear-gradient(135deg, #14532d 0%, #166534 100%);
	}
	.type-preview.document {
		background: linear-gradient(135deg, #1e3a5f 0%, #0369a1 100%);
	}

	.selection-indicator {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		width: 24px;
		height: 24px;
		background: #3b82f6;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.quick-actions {
		position: absolute;
		bottom: 0.5rem;
		right: 0.5rem;
		display: flex;
		gap: 0.25rem;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.asset-card:hover .quick-actions {
		opacity: 1;
	}

	.action-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		border: none;
		border-radius: 6px;
		color: white;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(0, 0, 0, 0.8);
	}

	.asset-info {
		padding: 0.75rem;
	}

	.asset-name {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e2e8f0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.asset-meta {
		font-size: 0.6875rem;
		color: #64748b;
	}

	/* Asset List */
	.asset-list {
		overflow-x: auto;
	}

	.asset-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.asset-list th,
	.asset-list td {
		padding: 0.75rem;
		text-align: left;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.asset-list th {
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.asset-list tr {
		cursor: pointer;
		transition: background 0.2s;
	}

	.asset-list tbody tr:hover {
		background: rgba(255, 255, 255, 0.03);
	}

	.asset-list tr.selected {
		background: rgba(59, 130, 246, 0.1);
	}

	.td-preview {
		width: 50px;
	}

	.td-preview img {
		width: 40px;
		height: 40px;
		object-fit: cover;
		border-radius: 4px;
	}

	.type-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		color: #64748b;
	}

	.type-icon.video {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}
	.type-icon.audio {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}
	.type-icon.document {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.td-name span {
		display: block;
		max-width: 300px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		color: #e2e8f0;
	}

	.td-type {
		color: #94a3b8;
		text-transform: capitalize;
	}
	.td-size {
		color: #64748b;
	}
	.td-date {
		color: #64748b;
	}

	.td-actions button {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 4px;
	}

	.td-actions button:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #94a3b8;
	}

	/* Load More */
	.load-more {
		display: flex;
		justify-content: center;
		padding: 2rem;
	}

	.load-more button {
		padding: 0.625rem 1.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.load-more button:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.1);
	}

	.load-more button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ========================================================================== */
	/* Details Sidebar */
	/* ========================================================================== */

	.details-sidebar {
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 320px;
		background: #0f172a;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		overflow-y: auto;
		z-index: 10;
	}

	.details-sidebar .sidebar-header {
		position: sticky;
		top: 0;
		background: #0f172a;
		z-index: 1;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.details-sidebar .sidebar-header h4 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.close-sidebar {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.close-sidebar:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #94a3b8;
	}

	.detail-preview {
		aspect-ratio: 16/10;
		background: #1e293b;
		overflow: hidden;
	}

	.detail-preview img,
	.detail-preview video {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.type-preview-large {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.detail-section {
		padding: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.detail-section h5 {
		margin: 0 0 0.75rem;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.section-header h5 {
		margin-bottom: 0;
	}

	.edit-btn {
		background: none;
		border: none;
		color: #3b82f6;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.info-list {
		margin: 0;
	}

	.info-list dt {
		font-size: 0.6875rem;
		color: #64748b;
		margin-top: 0.5rem;
	}

	.info-list dt:first-child {
		margin-top: 0;
	}

	.info-list dd {
		margin: 0.125rem 0 0 0;
		font-size: 0.8125rem;
		color: #e2e8f0;
		word-break: break-word;
	}

	.tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		margin-top: 0.25rem;
	}

	.tag {
		padding: 0.125rem 0.5rem;
		background: rgba(59, 130, 246, 0.15);
		color: #3b82f6;
		border-radius: 999px;
		font-size: 0.6875rem;
	}

	/* Metadata Form */
	.metadata-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field label {
		display: block;
		font-size: 0.6875rem;
		color: #64748b;
		margin-bottom: 0.25rem;
	}

	.field input,
	.field textarea {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.8125rem;
	}

	.field input:focus,
	.field textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.field textarea {
		min-height: 60px;
		resize: vertical;
	}

	.save-btn {
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		margin-top: 0.5rem;
	}

	/* Usage */
	.loading-small {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.no-usage {
		font-size: 0.8125rem;
		color: #64748b;
		font-style: italic;
	}

	.usage-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.usage-list li {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.usage-type {
		font-size: 0.6875rem;
		padding: 0.125rem 0.375rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		color: #94a3b8;
		text-transform: capitalize;
	}

	.usage-title {
		font-size: 0.8125rem;
		color: #e2e8f0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	/* Detail Actions */
	.detail-actions {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.action-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: none;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.8125rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-link:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #f1f5f9;
	}

	.action-link.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	/* ========================================================================== */
	/* Upload Tab */
	/* ========================================================================== */

	.upload-area {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	/* Drop-zone CSS extracted with DropZone (R6-C #7). */

	/* Upload Queue */
	.upload-queue {
		margin-top: 2rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
	}

	.queue-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.queue-header h4 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.clear-queue {
		background: none;
		border: none;
		color: #64748b;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.queue-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
	}

	.queue-item.complete {
		border: 1px solid rgba(34, 197, 94, 0.3);
	}

	.queue-item.error {
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.queue-item .item-preview {
		width: 40px;
		height: 40px;
		border-radius: 4px;
		overflow: hidden;
		background: #1e293b;
		flex-shrink: 0;
	}

	.queue-item .item-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-icon {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.queue-item .item-info {
		flex: 1;
		min-width: 0;
	}

	.item-name {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #e2e8f0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-size {
		font-size: 0.6875rem;
		color: #64748b;
	}

	.error-text {
		display: block;
		font-size: 0.6875rem;
		color: #ef4444;
		margin-top: 0.25rem;
	}

	.item-status {
		flex-shrink: 0;
	}

	.remove-item {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
	}

	.progress-bar {
		height: 3px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		margin-top: 0.5rem;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #06b6d4);
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.progress-fill.indeterminate {
		width: 30%;
		animation: indeterminate 1.5s infinite;
	}

	.start-upload-btn {
		width: 100%;
		padding: 0.75rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		margin-top: 1rem;
		transition: all 0.2s;
	}

	.start-upload-btn:hover:not(:disabled) {
		background: #2563eb;
	}

	.start-upload-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Footer CSS extracted with DamFooter (R6-C #3). */

	/* Drag-overlay CSS extracted with DragOverlay (R6-C #4). */

	/* ========================================================================== */
	/* Animations */
	/* ========================================================================== */

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes indeterminate {
		0% {
			transform: translateX(-100%);
		}
		100% {
			transform: translateX(400%);
		}
	}

	/* ========================================================================== */
	/* Responsive */
	/* ========================================================================== */

	@media (max-width: 1023.98px) {
		/* .folders-sidebar display:none moved into FoldersSidebar (R6-C #8). */

		.library-layout.sidebar-open .asset-content {
			margin-right: 0;
		}

		.details-sidebar {
			width: 100%;
		}
	}

	@media (max-width: 767.98px) {
		.dam-modal {
			height: 100vh;
			max-height: none;
			border-radius: 0;
		}

		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.toolbar-left,
		.toolbar-right {
			width: 100%;
		}

		.search-box {
			min-width: auto;
			flex: 1;
		}

		.asset-grid {
			grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		}
	}
</style>
