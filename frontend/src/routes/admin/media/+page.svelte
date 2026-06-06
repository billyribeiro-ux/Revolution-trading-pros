<script lang="ts">
	import { onMount } from 'svelte';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	/**
	 * Media Library - Apple ICT 7 Grade Enterprise Dashboard
	 *
	 * Principal Engineer Grade implementation featuring:
	 * - AI-powered image analysis and alt text generation
	 * - Smart cropping with subject detection
	 * - Drag-and-drop uploads with XHR progress tracking
	 * - Grid/List view with fluid Svelte 5 transitions
	 * - Full keyboard navigation (arrow keys, Enter, Space, Cmd+A)
	 * - Real-time optimization statistics with tweened animations
	 * - Bulk operations (optimize, download, delete)
	 * - Context menu with all actions
	 * - Responsive preview for multiple breakpoints
	 *
	 * Svelte 5 Runes: $state, $derived, $effect
	 *
	 * @since January 2026
	 */
	import { browser } from '$app/environment';
	import { fly, fade, scale, slide } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import DropZone from '$lib/components/media/DropZone.svelte';
	import OptimizedImage from '$lib/components/media/OptimizedImage.svelte';
	import ImageCropModal from '$lib/components/media/ImageCropModal.svelte';
	import MediaSkeleton from '$lib/components/media/MediaSkeleton.svelte';
	import { mediaApi, type MediaItem, type OptimizationStatistics } from '$lib/api/media';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import ToastContainer from './_components/ToastContainer.svelte';
	import StatsPanel from './_components/StatsPanel.svelte';
	import UploadQueue from './_components/UploadQueue.svelte';
	import Pagination from './_components/Pagination.svelte';
	import ContextMenu, { type ContextAction } from './_components/ContextMenu.svelte';
	import ResponsivePreviewModal from './_components/ResponsivePreviewModal.svelte';
	import PageHeader from './_components/PageHeader.svelte';
	// FIX-2026-04-26-audit (P1-7): media uploads, AI calls, and replace now route through
	// same-origin /api/admin/media/* proxies (see frontend/src/routes/api/admin/media/*).
	// Cross-origin XHRs to API_BASE_URL silently 401'd because the rtp_access_token cookie
	// is scoped to the frontend host; no need for those imports any more.

	// FIX-2026-04-26: Tabler icons replace 43 raw inline <svg> blocks for
	// consistent professional styling per CLAUDE.md icon-system standard.
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconArrowUp from '@tabler/icons-svelte-runes/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte-runes/icons/arrow-down';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconCrop from '@tabler/icons-svelte-runes/icons/crop';
	import IconDevices from '@tabler/icons-svelte-runes/icons/devices';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';

	// State

	// Data
	let items = $state<MediaItem[]>([]);
	let selectedIds = $state(new SvelteSet<string>()); // eslint-disable-line svelte/no-unnecessary-state-wrap
	let focusedId = $state<string | null>(null);

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let showBulkDeleteModal = $state(false);
	let pendingDeleteItem = $state<MediaItem | null>(null);
	let statistics = $state<OptimizationStatistics | null>(null);

	// Pagination
	let currentPage = $state(1);
	let totalPages = $state(1);
	let totalItems = $state(0);
	let perPage = $state(24);

	// Filters
	let searchQuery = $state('');
	let filterType = $state<'all' | 'image' | 'video' | 'document'>('all');
	let filterStatus = $state<'all' | 'optimized' | 'pending' | 'processing'>('all');
	let sortBy = $state<'created_at' | 'filename' | 'size'>('created_at');
	let sortDir = $state<'asc' | 'desc'>('desc');

	// UI State
	let viewMode = $state<'grid' | 'list'>('grid');
	let isLoading = $state(true);
	let isUploading = $state(false);
	let showUploadPanel = $state(false);
	let showDetailsPanel = $state(false);
	let showStatsPanel = $state(true);
	let showCropModal = $state(false);
	let showPreviewModal = $state(false);
	let detailItem = $state<MediaItem | null>(null);
	let contextMenu = $state<{ x: number; y: number; item: MediaItem } | null>(null);

	// Upload tracking
	let uploadQueue = $state<
		Array<{
			id: string;
			file: File;
			progress: number;
			status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
			error?: string;
			result?: MediaItem;
		}>
	>([]);

	// AI features
	let aiEnabled = $state(false);
	let isAnalyzing = $state(false);

	// Animations
	const statsProgress = tweened(0, { duration: 1000, easing: cubicOut });

	// Component-owned async resources
	let isComponentMounted = false;
	let mediaRequestGeneration = 0;
	let statisticsRequestGeneration = 0;
	const uploadRequests = new SvelteMap<string, XMLHttpRequest>();
	const toastTimers = new SvelteMap<string, ReturnType<typeof setTimeout>>();

	// Lifecycle
	onMount(() => {
		if (!browser) return;

		isComponentMounted = true;

		// Load initial data
		void Promise.all([loadMedia(), loadStatistics()]);

		// Setup keyboard and click listeners
		document.addEventListener('keydown', handleKeydown);
		document.addEventListener('click', handleClickOutside);

		// Cleanup on unmount
		return () => {
			isComponentMounted = false;
			mediaRequestGeneration++;
			statisticsRequestGeneration++;
			abortUploads();
			clearToastTimers();
			document.removeEventListener('keydown', handleKeydown);
			document.removeEventListener('click', handleClickOutside);
		};
	});

	// Data Loading

	async function loadMedia() {
		const requestGeneration = ++mediaRequestGeneration;
		if (!isComponentMounted) return;

		isLoading = true;
		try {
			const response = await mediaApi.list({
				page: currentPage,
				per_page: perPage,
				...(searchQuery && { search: searchQuery }),
				...(filterType !== 'all' && { type: filterType }),
				...(filterStatus === 'optimized' && { optimized: true }),
				...(filterStatus === 'pending' && { needs_optimization: true }),
				...(filterStatus === 'processing' && { processing_status: 'processing' }),
				sort_by: sortBy,
				sort_dir: sortDir
			});

			if (!isComponentMounted || requestGeneration !== mediaRequestGeneration) return;

			items = response.data;
			currentPage = response.meta.current_page;
			totalPages = response.meta.last_page;
			totalItems = response.meta.total;
		} catch {
			if (isComponentMounted && requestGeneration === mediaRequestGeneration) {
				showToast('Failed to load media', 'error');
			}
		} finally {
			if (isComponentMounted && requestGeneration === mediaRequestGeneration) {
				isLoading = false;
			}
		}
	}

	async function loadStatistics() {
		const requestGeneration = ++statisticsRequestGeneration;
		if (!isComponentMounted) return;

		try {
			const response = await mediaApi.getStatistics();
			if (!isComponentMounted || requestGeneration !== statisticsRequestGeneration) return;

			statistics = response.data;
			if (statistics) {
				const percent =
					statistics.total_images > 0
						? (statistics.optimized_images / statistics.total_images) * 100
						: 0;
				statsProgress.set(percent);
			}
		} catch (e) {
			if (isComponentMounted && requestGeneration === statisticsRequestGeneration) {
				console.error('Failed to load statistics', e);
			}
		}
	}

	// Upload Handling

	function handleFilesSelected(files: File[]) {
		if (!isComponentMounted || files.length === 0) return;

		showUploadPanel = true;
		isUploading = true;

		files.forEach((file) => {
			const id = crypto.randomUUID();
			uploadQueue = [...uploadQueue, { id, file, progress: 0, status: 'pending' }];
			void uploadFile(id, file);
		});
	}

	async function uploadFile(id: string, file: File) {
		if (!isComponentMounted) return;

		const idx = uploadQueue.findIndex((u) => u.id === id);
		if (idx === -1) return;

		const uploadItem = uploadQueue[idx];
		if (!uploadItem) return;
		// FIX-2026-04-26-audit (P3): in Svelte 5 $state proxies, mutating a property
		// of an item in the array already triggers reactivity. The `uploadQueue = uploadQueue`
		// self-assignment was a Svelte 4 idiom that is now noise.
		uploadItem.status = 'uploading';

		try {
			const formData = new FormData();
			formData.append('file', file);

			const xhr = new XMLHttpRequest();
			uploadRequests.set(id, xhr);

			xhr.upload.onprogress = (e) => {
				if (!isComponentMounted) return;
				if (e.lengthComputable) {
					const progress = Math.round((e.loaded / e.total) * 100);
					const item = uploadQueue.find((upload) => upload.id === id);
					if (item) {
						item.progress = progress;
					}
				}
			};

			const result = await new Promise<MediaItem>((resolve, reject) => {
				xhr.onload = () => {
					if (xhr.status >= 200 && xhr.status < 300) {
						resolve(JSON.parse(xhr.responseText).data);
					} else {
						reject(new Error(xhr.statusText));
					}
				};
				xhr.onerror = () => reject(new Error('Upload failed'));
				xhr.onabort = () => reject(new DOMException('Upload aborted', 'AbortError'));
				// FIX-2026-04-26-audit (P1-7): same-origin proxy. The proxy at
				// frontend/src/routes/api/admin/media/upload/+server.ts forwards the
				// multipart body verbatim with the Bearer header attached server-side.
				xhr.open('POST', '/api/admin/media/upload');
				xhr.send(formData);
			});

			if (!isComponentMounted) return;

			const completedItem = uploadQueue.find((upload) => upload.id === id);
			if (completedItem) {
				completedItem.status = 'complete';
				completedItem.result = result;
			}

			// Add to items list
			items = [result, ...items];
			totalItems++;
			void loadStatistics();
		} catch (e) {
			if ((e as { name?: string }).name === 'AbortError' || !isComponentMounted) return;

			const errorItem = uploadQueue.find((upload) => upload.id === id);
			if (errorItem) {
				errorItem.status = 'error';
				errorItem.error = (e as { message?: string }).message || 'Upload failed';
			}
		} finally {
			uploadRequests.delete(id);
		}

		if (!isComponentMounted) return;

		// Check if all uploads complete
		const allDone = uploadQueue.every((u) => u.status === 'complete' || u.status === 'error');
		if (allDone) {
			isUploading = false;
			const successCount = uploadQueue.filter((u) => u.status === 'complete').length;
			const failCount = uploadQueue.filter((u) => u.status === 'error').length;

			if (successCount > 0 && failCount === 0) {
				showToast(`${successCount} file${successCount > 1 ? 's' : ''} uploaded`, 'success');
			} else if (failCount > 0) {
				showToast(`${failCount} file${failCount > 1 ? 's' : ''} failed to upload`, 'error');
			}
		}
	}

	function clearUploadQueue() {
		abortUploads();
		uploadQueue = [];
		showUploadPanel = false;
		isUploading = false;
	}

	// Item Actions

	function handleItemClick(item: MediaItem, event: MouseEvent) {
		if (event.shiftKey && focusedId) {
			// Range select
			const start = items.findIndex((i) => i.id === focusedId);
			const end = items.findIndex((i) => i.id === item.id);
			const [from, to] = start < end ? [start, end] : [end, start];

			for (let i = from; i <= to; i++) {
				const item = items[i];
				if (item) selectedIds.add(item.id);
			}
		} else if (event.metaKey || event.ctrlKey) {
			// Toggle select
			if (selectedIds.has(item.id)) {
				selectedIds.delete(item.id);
			} else {
				selectedIds.add(item.id);
			}
		} else {
			// Single select and show details
			selectedIds.clear();
			selectedIds.add(item.id);
			focusedId = item.id;
			detailItem = item;
			showDetailsPanel = true;
		}
	}

	function handleItemDoubleClick(item: MediaItem) {
		detailItem = item;
		showPreviewModal = true;
	}

	function handleContextMenu(event: MouseEvent, item: MediaItem) {
		event.preventDefault();
		contextMenu = { x: event.clientX, y: event.clientY, item };
	}

	async function handleOptimize(item: MediaItem) {
		try {
			const response = await mediaApi.optimize(item.id);
			if (!isComponentMounted) return;

			const updatedItem = response.data as MediaItem;
			items = items.map((i) => (i.id === item.id ? updatedItem : i));
			if (detailItem?.id === item.id) detailItem = updatedItem;
			showToast('Image optimized', 'success');
			void loadStatistics();
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Optimization failed', 'error');
			}
		}
	}

	function handleDelete(item: MediaItem) {
		pendingDeleteItem = item;
		showDeleteModal = true;
	}

	async function confirmDelete() {
		if (!pendingDeleteItem) return;
		const item = pendingDeleteItem;
		showDeleteModal = false;
		pendingDeleteItem = null;

		try {
			await mediaApi.delete(item.id);
			if (!isComponentMounted) return;

			items = items.filter((i) => i.id !== item.id);
			selectedIds.delete(item.id);
			totalItems--;

			if (detailItem?.id === item.id) {
				detailItem = null;
				showDetailsPanel = false;
			}

			showToast('File deleted', 'success');
			void loadStatistics();
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Delete failed', 'error');
			}
		}
	}

	async function handleAIAnalyze(item: MediaItem) {
		if (!aiEnabled) return;
		isAnalyzing = true;

		try {
			// FIX-2026-04-26-audit (P1-7): same-origin admin proxy; was hitting an
			// orphan `/api/media/ai/analyze/{id}` route that 404'd in production.
			const response = await fetch(`/api/admin/media/ai/analyze/${item.id}`, {
				method: 'POST'
			});

			if (!response.ok) throw new Error('Analysis failed');
			if (!isComponentMounted) return;

			const result = await response.json();
			if (!isComponentMounted) return;

			// Update item with AI data
			const updatedItem = {
				...item,
				alt: result.altText || item.alt,
				custom_properties: {
					...item.custom_properties,
					ai_tags: result.tags,
					ai_category: result.category,
					focal_point: result.focalPoint
				}
			};

			items = items.map((i) => (i.id === item.id ? updatedItem : i));
			if (detailItem?.id === item.id) detailItem = updatedItem;

			showToast('AI analysis complete', 'success');
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'AI analysis failed', 'error');
			}
		} finally {
			if (isComponentMounted) {
				isAnalyzing = false;
			}
		}
	}

	async function handleGenerateAltText(item: MediaItem) {
		if (!aiEnabled) return;
		isAnalyzing = true;

		try {
			// FIX-2026-04-26-audit (P1-7): same-origin admin proxy.
			const response = await fetch(`/api/admin/media/ai/alt-text/${item.id}`, {
				method: 'POST'
			});

			if (!response.ok) throw new Error('Failed to generate alt text');
			if (!isComponentMounted) return;

			const result = await response.json();
			if (!isComponentMounted) return;

			// Update item
			await mediaApi.update(item.id, { alt_text: result.altText });
			if (!isComponentMounted) return;

			const updatedItem = { ...item, alt_text: result.altText };

			items = items.map((i) => (i.id === item.id ? updatedItem : i));
			if (detailItem?.id === item.id) detailItem = updatedItem;

			showToast('Alt text generated', 'success');
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Failed to generate alt text', 'error');
			}
		} finally {
			if (isComponentMounted) {
				isAnalyzing = false;
			}
		}
	}

	function handleCrop(item: MediaItem) {
		detailItem = item;
		showCropModal = true;
	}

	async function handleCropSave(cropResult: {
		blob: Blob;
		cropArea: { x: number; y: number; width: number; height: number };
		aspectRatio: string;
	}) {
		const cropTarget = detailItem;
		if (!cropTarget) return;

		try {
			const formData = new FormData();
			formData.append('file', cropResult.blob, cropTarget.filename);

			// FIX-2026-04-26-audit (P1-7): same-origin admin proxy; multipart streamed verbatim.
			const response = await fetch(`/api/admin/media/${cropTarget.id}/replace`, {
				method: 'POST',
				body: formData
			});

			if (!response.ok) throw new Error('Failed to save crop');
			if (!isComponentMounted) return;

			const apiResult = await response.json();
			if (!isComponentMounted) return;

			items = items.map((i) => (i.id === cropTarget.id ? apiResult.data : i));
			detailItem = apiResult.data;

			showCropModal = false;
			showToast('Image cropped and saved', 'success');
			void loadStatistics();
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Failed to save crop', 'error');
			}
		}
	}

	// Bulk Actions

	function selectAll() {
		if (selectedIds.size === items.length) {
			selectedIds.clear();
		} else {
			selectedIds = new SvelteSet(items.map((i) => i.id));
		}
	}

	async function bulkOptimize() {
		const ids = Array.from(selectedIds) as string[];
		if (ids.length === 0) return;

		try {
			const response = await mediaApi.bulkOptimize(ids as string[]);
			if (!isComponentMounted) return;

			showToast(`${response.success} images queued for optimization`, 'success');

			items = items.map((i) =>
				selectedIds.has(i.id) && i.file_type === 'image'
					? { ...i, processing_status: 'processing' as const }
					: i
			);

			selectedIds.clear();
			void loadStatistics();
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Bulk optimization failed', 'error');
			}
		}
	}

	function bulkDelete() {
		const ids = Array.from(selectedIds);
		if (ids.length === 0) return;
		showBulkDeleteModal = true;
	}

	async function confirmBulkDelete() {
		const ids = Array.from(selectedIds) as string[];
		showBulkDeleteModal = false;

		try {
			await mediaApi.bulkDelete(ids as string[]);
			if (!isComponentMounted) return;

			items = items.filter((i) => !selectedIds.has(i.id));
			totalItems -= ids.length;
			selectedIds.clear();
			showToast(`${ids.length} items deleted`, 'success');
			void loadStatistics();
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Bulk delete failed', 'error');
			}
		}
	}

	async function bulkDownload() {
		const ids = Array.from(selectedIds);
		if (ids.length === 0) return;

		// Download each file
		for (const id of ids) {
			const item = items.find((i) => i.id === id);
			if (item) {
				const a = document.createElement('a');
				a.href = item.url;
				a.download = item.filename;
				a.click();
			}
		}
	}

	// Keyboard Navigation

	function handleKeydown(event: KeyboardEvent) {
		if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
			return;
		}

		const currentIndex = focusedId ? items.findIndex((i) => i.id === focusedId) : -1;

		switch (event.key) {
			case 'ArrowRight':
				if (currentIndex < items.length - 1) {
					const nextItem = items[currentIndex + 1];
					if (nextItem) {
						focusedId = nextItem.id;
						if (!event.shiftKey) {
							selectedIds.clear();
							selectedIds.add(focusedId);
						}
					}
				}
				event.preventDefault();
				break;

			case 'ArrowLeft':
				if (currentIndex > 0) {
					const prevItem = items[currentIndex - 1];
					if (prevItem) {
						focusedId = prevItem.id;
						if (!event.shiftKey) {
							selectedIds.clear();
							selectedIds.add(focusedId);
						}
					}
				}
				event.preventDefault();
				break;

			case 'ArrowDown':
				const cols = viewMode === 'grid' ? 6 : 1;
				if (currentIndex + cols < items.length) {
					const downItem = items[currentIndex + cols];
					if (downItem) {
						focusedId = downItem.id;
						if (!event.shiftKey) {
							selectedIds.clear();
							selectedIds.add(focusedId);
						}
					}
				}
				event.preventDefault();
				break;

			case 'ArrowUp':
				const colsUp = viewMode === 'grid' ? 6 : 1;
				if (currentIndex - colsUp >= 0) {
					const upItem = items[currentIndex - colsUp];
					if (upItem) {
						focusedId = upItem.id;
						if (!event.shiftKey) {
							selectedIds.clear();
							selectedIds.add(focusedId);
						}
					}
				}
				event.preventDefault();
				break;

			case 'Enter':
				if (focusedId) {
					const item = items.find((i) => i.id === focusedId);
					if (item) {
						detailItem = item;
						showDetailsPanel = true;
					}
				}
				event.preventDefault();
				break;

			case ' ':
				if (focusedId) {
					if (selectedIds.has(focusedId)) {
						selectedIds.delete(focusedId);
					} else {
						selectedIds.add(focusedId);
					}
				}
				event.preventDefault();
				break;

			case 'a':
				if (event.metaKey || event.ctrlKey) {
					selectAll();
					event.preventDefault();
				}
				break;

			case 'Delete':
			case 'Backspace':
				if (selectedIds.size > 0) {
					bulkDelete();
					event.preventDefault();
				}
				break;

			case 'Escape':
				selectedIds.clear();
				contextMenu = null;
				showDetailsPanel = false;
				break;
		}
	}

	function handleClickOutside(_event: MouseEvent) {
		if (contextMenu) {
			contextMenu = null;
		}
	}

	// Context Menu dispatcher (post-R10-C extraction)

	function handleContextAction(action: ContextAction) {
		if (!contextMenu) return;
		const item = contextMenu.item;
		switch (action) {
			case 'view':
				handleItemDoubleClick(item);
				break;
			case 'optimize':
				void handleOptimize(item);
				break;
			case 'crop':
				handleCrop(item);
				break;
			case 'ai-analyze':
				void handleAIAnalyze(item);
				break;
			case 'copy-url':
				void navigator.clipboard.writeText(item.url);
				showToast('URL copied', 'success');
				break;
			case 'delete':
				handleDelete(item);
				break;
		}
	}

	// Toast Notifications

	let toasts = $state<Array<{ id: string; message: string; type: 'success' | 'error' | 'info' }>>(
		[]
	);

	function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
		if (!isComponentMounted) return;

		const id = crypto.randomUUID();
		toasts = [...toasts, { id, message, type }];
		const timer = setTimeout(() => {
			if (!isComponentMounted) return;
			toasts = toasts.filter((t) => t.id !== id);
			toastTimers.delete(id);
		}, 4000);
		toastTimers.set(id, timer);
	}

	function clearToastTimers() {
		for (const timer of toastTimers.values()) {
			clearTimeout(timer);
		}
		toastTimers.clear();
	}

	function abortUploads() {
		for (const xhr of uploadRequests.values()) {
			xhr.abort();
		}
		uploadRequests.clear();
	}

	async function handleAltTextBlur(event: FocusEvent) {
		const target = event.target as HTMLTextAreaElement;
		const current = detailItem;
		if (!current || target.value === current.alt_text) return;

		try {
			await mediaApi.update(current.id, { alt_text: target.value });
			if (!isComponentMounted || detailItem?.id !== current.id) return;

			const updatedItem = { ...current, alt_text: target.value };
			detailItem = updatedItem;
			items = items.map((item) => (item.id === current.id ? updatedItem : item));
			showToast('Alt text saved', 'success');
		} catch (e) {
			if (isComponentMounted) {
				showToast((e as { message?: string }).message || 'Failed to save alt text', 'error');
			}
		}
	}

	// Helpers

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}

	function formatDate(date: string): string {
		return new Date(date).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getFileIcon(type: string): string {
		switch (type) {
			case 'image':
				return '🖼️';
			case 'video':
				return '🎬';
			case 'document':
				return '📄';
			default:
				return '📁';
		}
	}
</script>

<svelte:head>
	<title>Media Library | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class={['admin-media', { 'details-open': showDetailsPanel }]}>
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header (extracted to _components/PageHeader.svelte) -->
		<PageHeader
			{totalItems}
			bind:searchQuery
			bind:viewMode
			bind:showUploadPanel
			onSearchSubmit={() => void loadMedia()}
		/>

		<!-- Statistics Panel (extracted to _components/StatsPanel.svelte) -->
		<StatsPanel {statistics} bind:showStatsPanel statsProgress={$statsProgress} />

		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<!-- Upload Panel -->
		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		{#if showUploadPanel}
			<div class="upload-panel" transition:slide={{ duration: 300 }}>
				<DropZone
					accept="image/*,video/*,application/pdf"
					maxSize={50 * 1024 * 1024}
					multiple={true}
					onfiles={handleFilesSelected}
				/>

				<UploadQueue uploads={uploadQueue} {isUploading} onClearQueue={clearUploadQueue} />
			</div>
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<!-- Toolbar -->
		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<div class="toolbar">
			<div class="toolbar-left">
				<!-- Filters -->
				<select bind:value={filterType} onchange={() => void loadMedia()}>
					<option value="all">All Types</option>
					<option value="image">Images</option>
					<option value="video">Videos</option>
					<option value="document">Documents</option>
				</select>

				<select bind:value={filterStatus} onchange={() => void loadMedia()}>
					<option value="all">All Status</option>
					<option value="optimized">Optimized</option>
					<option value="pending">Needs Optimization</option>
					<option value="processing">Processing</option>
				</select>

				<select bind:value={sortBy} onchange={() => void loadMedia()}>
					<option value="created_at">Date Added</option>
					<option value="filename">Name</option>
					<option value="size">Size</option>
				</select>

				<button
					class="btn-icon"
					onclick={() => {
						sortDir = sortDir === 'asc' ? 'desc' : 'asc';
						void loadMedia();
					}}
				>
					{#if sortDir === 'asc'}
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: arrow-up sort asc -->
						<IconArrowUp size={20} aria-hidden="true" />
					{:else}
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: arrow-down sort desc -->
						<IconArrowDown size={20} aria-hidden="true" />
					{/if}
				</button>
			</div>

			<div class="toolbar-right">
				{#if selectedIds.size > 0}
					<div class="bulk-actions" transition:scale={{ duration: 200 }}>
						<span class="selection-count">{selectedIds.size} selected</span>
						<button class="btn-secondary btn-sm" onclick={selectAll}>
							{selectedIds.size === items.length ? 'Deselect All' : 'Select All'}
						</button>
						<button class="btn-primary btn-sm" onclick={() => void bulkOptimize()}>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (optimize) -->
							<IconBolt size={20} aria-hidden="true" />
							Optimize
						</button>
						<button class="btn-secondary btn-sm" onclick={() => void bulkDownload()}>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: download -->
							<IconDownload size={20} aria-hidden="true" />
							Download
						</button>
						<button class="btn-danger btn-sm" onclick={bulkDelete}>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trash (bulk delete) -->
							<IconTrash size={20} aria-hidden="true" />
							Delete
						</button>
					</div>
				{/if}
			</div>
		</div>

		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<!-- Main Content -->
		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<div class="content-area">
			<main class="media-content">
				{#if isLoading}
					<MediaSkeleton type={viewMode as 'grid' | 'list'} count={perPage} />
				{:else if items.length === 0}
					<div class="empty-state">
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: upload (empty state) -->
						<IconUpload size={48} aria-hidden="true" />
						<h3>No media files</h3>
						<p>Upload files to get started</p>
						<button class="btn-primary" onclick={() => (showUploadPanel = true)}>
							Upload Files
						</button>
					</div>
				{:else}
					<div class={`media-${viewMode}`}>
						{#each items as item (item.id)}
							<div
								class={[
									'media-item',
									{ selected: selectedIds.has(item.id), focused: focusedId === item.id }
								]}
								onclick={(e: MouseEvent) => handleItemClick(item, e)}
								ondblclick={() => handleItemDoubleClick(item)}
								oncontextmenu={(e: MouseEvent) => handleContextMenu(e, item)}
								onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleItemDoubleClick(item)}
								role="button"
								tabindex="0"
								aria-label={item.filename}
								transition:fade={{ duration: 200 }}
							>
								{#if viewMode === 'grid'}
									<div class="item-thumbnail">
										{#if item.file_type === 'image'}
											<OptimizedImage
												src={item.thumbnail_url || item.url}
												alt={item.alt_text || item.filename}
												blurhash={String(item.custom_properties?.['blurhash'] || '')}
												aspectRatio="1"
											/>
										{:else}
											<div class="file-icon">{getFileIcon(item.file_type)}</div>
										{/if}

										<!-- Status badges -->
										<div class="item-badges">
											{#if item.is_optimized}
												<span class="badge badge-green" title="Optimized">
													<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: check (optimized badge) -->
													<IconCheck size={12} aria-hidden="true" />
												</span>
											{/if}
											{#if item.processing_status === 'processing'}
												<span class="badge badge-orange" title="Processing">
													<div class="spinner-mini"></div>
												</span>
											{/if}
										</div>

										<!-- Selection checkbox -->
										<div
											class={[
												'item-checkbox',
												{ visible: selectedIds.size > 0 || selectedIds.has(item.id) }
											]}
										>
											<input
												id="page-checkbox"
												name="page-checkbox"
												type="checkbox"
												checked={selectedIds.has(item.id)}
												onclick={(e: MouseEvent) => {
													e.stopPropagation();
													if (selectedIds.has(item.id)) {
														selectedIds.delete(item.id);
													} else {
														selectedIds.add(item.id);
													}
												}}
											/>
										</div>
									</div>

									<div class="item-info">
										<span class="item-name" title={item.filename}>{item.filename}</span>
										<span class="item-meta">{formatBytes(item.size ?? 0)}</span>
									</div>
								{:else}
									<!-- List view -->
									<div class="item-checkbox">
										<input
											id="page-checkbox"
											name="page-checkbox"
											type="checkbox"
											checked={selectedIds.has(item.id)}
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												if (selectedIds.has(item.id)) {
													selectedIds.delete(item.id);
												} else {
													selectedIds.add(item.id);
												}
											}}
										/>
									</div>

									<div class="item-thumbnail-small">
										{#if item.file_type === 'image'}
											<OptimizedImage
												src={item.thumbnail_url || item.url}
												alt={item.alt_text || item.filename}
												aspectRatio="1"
											/>
										{:else}
											<div class="file-icon-small">{getFileIcon(item.file_type)}</div>
										{/if}
									</div>

									<div class="item-details">
										<span class="item-name">{item.filename}</span>
										<span class="item-path">{item.collection || 'default'}</span>
									</div>

									<div class="item-size">{formatBytes(item.size ?? 0)}</div>

									<div class="item-status">
										{#if item.is_optimized}
											<span class="status-badge green">Optimized</span>
										{:else if item.processing_status === 'processing'}
											<span class="status-badge orange">Processing</span>
										{:else}
											<span class="status-badge gray">Pending</span>
										{/if}
									</div>

									<div class="item-date">{formatDate(item.created_at)}</div>

									<div class="item-actions">
										<button
											class="btn-icon-sm"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												void handleOptimize(item);
											}}
											title="Optimize"
										>
											<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (optimize) -->
											<IconBolt size={16} aria-hidden="true" />
										</button>
										<button
											class="btn-icon-sm"
											onclick={(e: MouseEvent) => {
												e.stopPropagation();
												handleDelete(item);
											}}
											title="Delete"
										>
											<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trash (delete) -->
											<IconTrash size={16} aria-hidden="true" />
										</button>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}

				<!-- Pagination (extracted to _components/Pagination.svelte) -->
				<Pagination bind:currentPage {totalPages} onChange={() => void loadMedia()} />
			</main>

			<!-- ═══════════════════════════════════════════════════════════════════════ -->
			<!-- Details Panel -->
			<!-- ═══════════════════════════════════════════════════════════════════════ -->
			{#if showDetailsPanel && detailItem}
				<aside class="details-panel" transition:fly={{ x: 320, duration: 300 }}>
					<div class="details-header">
						<h2>Details</h2>
						<button
							class="btn-icon"
							onclick={() => (showDetailsPanel = false)}
							aria-label="Close details panel"
						>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close details) -->
							<IconX size={20} aria-hidden="true" />
						</button>
					</div>

					<div class="details-preview">
						{#if detailItem.file_type === 'image'}
							<OptimizedImage
								src={detailItem.url}
								alt={detailItem.alt_text || detailItem.filename}
								blurhash={String(detailItem.custom_properties?.['blurhash'] || '')}
							/>
						{:else}
							<div class="file-preview-icon">{getFileIcon(detailItem.file_type)}</div>
						{/if}
					</div>

					<div class="details-info">
						<div class="detail-row">
							<span class="detail-label">Filename</span>
							<span class="detail-value">{detailItem.filename}</span>
						</div>
						<div class="detail-row">
							<span class="detail-label">Size</span>
							<span class="detail-value">{formatBytes(detailItem.size ?? 0)}</span>
						</div>
						{#if detailItem.dimensions}
							<div class="detail-row">
								<span class="detail-label">Dimensions</span>
								<span class="detail-value"
									>{detailItem.dimensions.width} × {detailItem.dimensions.height}</span
								>
							</div>
						{/if}
						<div class="detail-row">
							<span class="detail-label">Type</span>
							<span class="detail-value">{detailItem.mime_type}</span>
						</div>
						<div class="detail-row">
							<span class="detail-label">Created</span>
							<span class="detail-value">{formatDate(detailItem.created_at)}</span>
						</div>
						<div class="detail-row">
							<span class="detail-label">Status</span>
							<span class="detail-value">
								{#if detailItem.is_optimized}
									<span class="status-badge green">Optimized</span>
								{:else if detailItem.processing_status === 'processing'}
									<span class="status-badge orange">Processing</span>
								{:else}
									<span class="status-badge gray">Pending</span>
								{/if}
							</span>
						</div>
					</div>

					<!-- Alt Text -->
					<div class="details-section">
						<div class="section-header">
							<h3>Alt Text</h3>
							{#if aiEnabled}
								<button
									class="btn-text btn-sm"
									onclick={() => {
										if (detailItem) void handleGenerateAltText(detailItem);
									}}
									disabled={isAnalyzing}
								>
									{#if isAnalyzing}
										<div class="spinner-mini"></div>
									{:else}
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: eye (generate AI) -->
										<IconEye size={20} aria-hidden="true" />
										Generate with AI
									{/if}
								</button>
							{/if}
						</div>
						<textarea
							class="alt-input"
							placeholder="Enter alt text for accessibility..."
							value={detailItem.alt_text || ''}
							onblur={(event) => void handleAltTextBlur(event)}
						></textarea>
					</div>

					<!-- AI Analysis -->
					{#if aiEnabled}
						<div class="details-section">
							<div class="section-header">
								<h3>AI Analysis</h3>
								<button
									class="btn-text btn-sm"
									onclick={() => {
										if (detailItem) void handleAIAnalyze(detailItem);
									}}
									disabled={isAnalyzing}
								>
									{#if isAnalyzing}
										<div class="spinner-mini"></div>
									{:else}
										Analyze
									{/if}
								</button>
							</div>

							{#if detailItem.custom_properties?.['ai_tags'] && Array.isArray(detailItem.custom_properties['ai_tags'])}
								<div class="ai-tags">
									{#each detailItem.custom_properties['ai_tags'] as tag, ti (ti)}
										<span class="tag">{String(tag)}</span>
									{/each}
								</div>
							{:else}
								<p class="ai-hint">Click "Analyze" to get AI-powered tags and insights</p>
							{/if}
						</div>
					{/if}

					<!-- Actions -->
					<div class="details-actions">
						{#if detailItem.file_type === 'image'}
							<button class="btn-secondary" onclick={() => detailItem && handleCrop(detailItem)}>
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: crop -->
								<IconCrop size={20} aria-hidden="true" />
								Crop
							</button>
							<button class="btn-secondary" onclick={() => (showPreviewModal = true)}>
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: devices (responsive preview) -->
								<IconDevices size={20} aria-hidden="true" />
								Responsive Preview
							</button>
						{/if}
						<button
							class="btn-primary"
							onclick={() => {
								if (detailItem) void handleOptimize(detailItem);
							}}
						>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (optimize) -->
							<IconBolt size={20} aria-hidden="true" />
							Optimize
						</button>
						<button class="btn-danger" onclick={() => detailItem && handleDelete(detailItem)}>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trash (delete) -->
							<IconTrash size={20} aria-hidden="true" />
							Delete
						</button>
					</div>
				</aside>
			{/if}
		</div>

		<!-- Context Menu (extracted to _components/ContextMenu.svelte) -->
		<ContextMenu bind:menu={contextMenu} {aiEnabled} onAction={handleContextAction} />

		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<!-- Toast Notifications (extracted to _components/ToastContainer.svelte) -->
		<!-- ═══════════════════════════════════════════════════════════════════════ -->
		<ToastContainer {toasts} />
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Crop Modal -->
{#if showCropModal && detailItem}
	<ImageCropModal
		src={detailItem.url}
		oncrop={(cropResult) => void handleCropSave(cropResult)}
		oncancel={() => (showCropModal = false)}
	/>
{/if}

<!-- Responsive Preview Modal (extracted to _components/ResponsivePreviewModal.svelte) -->
<ResponsivePreviewModal bind:isOpen={showPreviewModal} item={detailItem} />

<!-- Delete Single Item Modal -->
<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete File"
	message={pendingDeleteItem
		? `Delete "${pendingDeleteItem.filename}"? This cannot be undone.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={() => void confirmDelete()}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteItem = null;
	}}
/>

<!-- Bulk Delete Modal -->
<ConfirmationModal
	isOpen={showBulkDeleteModal}
	title="Delete Selected Files"
	message={`Delete ${selectedIds.size} selected file(s)? This cannot be undone.`}
	confirmText="Delete All"
	variant="danger"
	onConfirm={() => void confirmBulkDelete()}
	onCancel={() => (showBulkDeleteModal = false)}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
     Base Layout - Dark Theme (Email Templates Style)
     ═══════════════════════════════════════════════════════════════════════════ */
	.media-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Page-header CSS moved to _components/PageHeader.svelte (R10-C). */

	/* Statistics Panel CSS moved to _components/StatsPanel.svelte (R10-C). */

	/* ═══════════════════════════════════════════════════════════════════════════
     Upload Panel
     ═══════════════════════════════════════════════════════════════════════════ */
	.upload-panel {
		padding: 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	/* Upload-queue/item CSS moved to _components/UploadQueue.svelte (R10-C). */

	/* ═══════════════════════════════════════════════════════════════════════════
     Toolbar
     ═══════════════════════════════════════════════════════════════════════════ */
	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		margin-bottom: 1rem;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.toolbar-left,
	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.toolbar select {
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		font-size: 0.8125rem;
		color: #e2e8f0;
		cursor: pointer;
	}

	.toolbar select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.btn-icon :global(svg) {
		width: 18px;
		height: 18px;
	}

	.bulk-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
	}

	.selection-count {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #a5b4fc;
		padding-right: 0.5rem;
		border-right: 1px solid rgba(99, 102, 241, 0.2);
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger,
	.btn-text {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-sm {
		padding: 0.375rem 0.625rem;
		font-size: 0.75rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
	}

	.btn-primary:hover {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.4);
	}

	.btn-danger {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.btn-text {
		background: transparent;
		color: #a5b4fc;
		padding: 0.25rem 0.5rem;
	}

	.btn-text:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.btn-primary :global(svg),
	.btn-secondary :global(svg),
	.btn-danger :global(svg),
	.btn-text :global(svg) {
		width: 14px;
		height: 14px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
     Content Area
     ═══════════════════════════════════════════════════════════════════════════ */
	.content-area {
		display: flex;
		gap: 1.5rem;
	}

	.media-content {
		flex: 1;
		min-width: 0;
	}

	.admin-media.details-open .media-content {
		margin-right: 340px;
	}

	/* Empty state */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.1);
		color: #94a3b8;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.empty-state :global(svg) {
		width: 64px;
		height: 64px;
		margin-bottom: 1rem;
		opacity: 0.5;
	}

	.empty-state h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	/* Grid View */
	.media-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
		gap: 1rem;
	}

	.media-grid .media-item {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s;
	}

	.media-grid .media-item:hover {
		transform: translateY(-2px);
		border-color: rgba(99, 102, 241, 0.3);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.media-grid .media-item.selected {
		border-color: var(--primary-500);
		box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.3);
	}

	.media-grid .media-item.focused {
		border-color: #8b5cf6;
		box-shadow:
			0 0 0 2px rgba(139, 92, 246, 0.3),
			0 0 0 4px rgba(139, 92, 246, 0.1);
	}

	.item-thumbnail {
		position: relative;
		aspect-ratio: 1;
		background: rgba(15, 23, 42, 0.6);
		overflow: hidden;
	}

	.item-thumbnail :global(img) {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 48px;
	}

	.item-badges {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		gap: 0.25rem;
	}

	.badge {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.badge :global(svg) {
		width: 14px;
		height: 14px;
	}

	.badge-green {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.badge-orange {
		background: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.spinner-mini {
		width: 12px;
		height: 12px;
		border: 2px solid rgba(245, 158, 11, 0.3);
		border-top-color: #f59e0b;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.item-checkbox {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.item-checkbox.visible,
	.media-item:hover .item-checkbox {
		opacity: 1;
	}

	.item-checkbox input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
		accent-color: var(--primary-500);
	}

	.item-info {
		padding: 0.75rem;
	}

	.item-name {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #f1f5f9;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* List View */
	.media-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.media-list .media-item {
		display: grid;
		grid-template-columns: 24px 48px 1fr 100px 100px 120px 80px;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.media-list .media-item:hover {
		background: rgba(30, 41, 59, 0.6);
		border-color: rgba(99, 102, 241, 0.2);
	}

	.media-list .media-item.selected {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
	}

	.media-list .item-checkbox {
		position: static;
		opacity: 1;
	}

	.item-thumbnail-small {
		width: 48px;
		height: 48px;
		border-radius: 6px;
		overflow: hidden;
		background: rgba(15, 23, 42, 0.6);
	}

	.item-thumbnail-small :global(img) {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-icon-small {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 24px;
	}

	.item-details {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.item-path {
		font-size: 0.75rem;
		color: #64748b;
	}

	.item-size,
	.item-date {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.item-status {
		display: flex;
		align-items: center;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.6875rem;
		font-weight: 500;
	}

	.status-badge.green {
		background: rgba(34, 197, 94, 0.15);
		color: #22c55e;
	}

	.status-badge.orange {
		background: rgba(245, 158, 11, 0.15);
		color: #f59e0b;
	}

	.status-badge.gray {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.item-actions {
		display: flex;
		gap: 0.25rem;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.media-list .media-item:hover .item-actions {
		opacity: 1;
	}

	.btn-icon-sm {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.btn-icon-sm:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
	}

	/* FIX-2026-04-26: :global() so styles reach Tabler's internal <svg> */
	.btn-icon-sm :global(svg) {
		width: 16px;
		height: 16px;
	}

	/* Pagination CSS moved to _components/Pagination.svelte (R10-C). */

	/* ═══════════════════════════════════════════════════════════════════════════
     Details Panel
     ═══════════════════════════════════════════════════════════════════════════ */
	.details-panel {
		position: fixed;
		right: 0;
		top: 0;
		bottom: 0;
		width: 320px;
		background: rgba(15, 23, 42, 0.98);
		border-left: 1px solid rgba(148, 163, 184, 0.1);
		display: flex;
		flex-direction: column;
		z-index: 50;
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
	}

	.details-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.details-header h2 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.details-preview {
		aspect-ratio: 16/9;
		background: rgba(30, 41, 59, 0.6);
		overflow: hidden;
	}

	.details-preview :global(img) {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.file-preview-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 64px;
	}

	.details-info {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0;
	}

	.detail-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.detail-value {
		font-size: 0.8125rem;
		color: #f1f5f9;
		font-weight: 500;
	}

	.details-section {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.75rem;
	}

	.section-header h3 {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.alt-input {
		width: 100%;
		min-height: 80px;
		padding: 0.75rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: #f1f5f9;
		resize: vertical;
		outline: none;
	}

	.alt-input::placeholder {
		color: #64748b;
	}

	.alt-input:focus {
		border-color: rgba(99, 102, 241, 0.5);
	}

	.ai-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.tag {
		display: inline-flex;
		padding: 0.25rem 0.625rem;
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
		border-radius: 9999px;
		font-size: 0.75rem;
	}

	.ai-hint {
		font-size: 0.75rem;
		color: #64748b;
		margin: 0;
	}

	.details-actions {
		padding: 1rem 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-top: auto;
	}

	.details-actions button {
		width: 100%;
		justify-content: center;
	}

	/* Context-menu CSS moved to _components/ContextMenu.svelte (R10-C). */

	/* ═══════════════════════════════════════════════════════════════════════════
     Toast Notifications
     ═══════════════════════════════════════════════════════════════════════════ */
	/* Toast CSS moved to _components/ToastContainer.svelte (R10-C). */

	/* Modal CSS moved to _components/ResponsivePreviewModal.svelte (R10-C). */

	/* ═══════════════════════════════════════════════════════════════════════════
     Loading State
     ═══════════════════════════════════════════════════════════════════════════ */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loader {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* File Type Badges - Indigo/Purple Scheme */
	.type-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
     Animations
     ═══════════════════════════════════════════════════════════════════════════ */
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
     Responsive
     ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1023.98px) {
		.admin-media.details-open .media-content {
			margin-right: 0;
		}

		.details-panel {
			width: 100%;
			max-width: 400px;
		}
	}

	@media (max-width: 767.98px) {
		.media-page {
			padding: 1rem;
		}

		/* Page-header responsive overrides moved to _components/PageHeader.svelte (R10-C). */

		.toolbar {
			flex-direction: column;
			align-items: stretch;
		}

		.toolbar-left,
		.toolbar-right {
			justify-content: center;
		}

		.media-grid {
			grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		}

		.media-list .media-item {
			grid-template-columns: 24px 40px 1fr auto;
		}

		.media-list .item-status,
		.media-list .item-date,
		.media-list .item-size {
			display: none;
		}
	}
</style>
