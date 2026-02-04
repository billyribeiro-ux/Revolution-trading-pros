<!--
  ResourceViewer.svelte
  Apple Principal Engineer ICT 7 Grade - February 2026

  Modal viewer for resources with:
  - Video playback with embedded player
  - PDF preview with embedded viewer
  - Image preview with zoom
  - Download functionality with secure URLs
  - Version history display
  - Access control indication
-->
<script lang="ts">
	import type { RoomResource } from '$lib/api/room-resources';
	import { trackDownload } from '$lib/api/room-resources';

	interface Props {
		resource: RoomResource;
		open?: boolean;
		showVersionHistory?: boolean;
		onClose?: () => void;
		onDownload?: (resource: RoomResource) => void;
		onVersionSelect?: (resource: RoomResource) => void;
	}

	let {
		resource,
		open = $bindable(false),
		showVersionHistory = false,
		onClose,
		onDownload,
		onVersionSelect
	}: Props = $props();

	let downloading = $state(false);
	let versions = $state<RoomResource[]>([]);
	let loadingVersions = $state(false);
	let imageZoom = $state(1);
	let imagePosition = $state({ x: 0, y: 0 });
	let dragging = $state(false);
	let startPos = $state({ x: 0, y: 0 });

	let isVideo = $derived(resource.resource_type === 'video');
	let isPdf = $derived(resource.resource_type === 'pdf');
	let isImage = $derived(resource.resource_type === 'image');
	let isPremium = $derived(resource.access_level !== 'free');

	// Load version history if requested
	async function loadVersionHistory() {
		if (!showVersionHistory || versions.length > 0) return;

		loadingVersions = true;
		try {
			const response = await fetch(`/api/room-resources/${resource.id}/versions`);
			const data = await response.json();
			if (data.success) {
				versions = data.data;
			}
		} catch (e) {
			console.error('Failed to load version history:', e);
		} finally {
			loadingVersions = false;
		}
	}

	$effect(() => {
		if (open && showVersionHistory) {
			loadVersionHistory();
		}
	});

	// Handle download
	async function handleDownload() {
		downloading = true;
		try {
			// Track the download
			await trackDownload(resource.id);

			// For premium resources, get secure download URL
			if (isPremium) {
				const response = await fetch(`/api/room-resources/${resource.id}/secure-download`, {
					method: 'POST',
					credentials: 'include'
				});
				const data = await response.json();
				if (data.success && data.file_url) {
					window.open(data.file_url, '_blank');
				}
			} else {
				// Direct download for free resources
				window.open(resource.file_url, '_blank');
			}

			onDownload?.(resource);
		} catch (e) {
			console.error('Download failed:', e);
		} finally {
			downloading = false;
		}
	}

	// Close modal
	function handleClose() {
		open = false;
		onClose?.();
	}

	// Handle keyboard events
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			handleClose();
		}
	}

	// Image zoom controls
	function zoomIn() {
		imageZoom = Math.min(imageZoom + 0.25, 3);
	}

	function zoomOut() {
		imageZoom = Math.max(imageZoom - 0.25, 0.5);
	}

	function resetZoom() {
		imageZoom = 1;
		imagePosition = { x: 0, y: 0 };
	}

	// Svelte action for image pan/zoom - official Svelte 5 pattern per docs
	function panZoomAction(node: HTMLElement) {
		function onMouseDown(event: MouseEvent) {
			if (imageZoom > 1) {
				dragging = true;
				startPos = { x: event.clientX - imagePosition.x, y: event.clientY - imagePosition.y };
				event.preventDefault();
			}
		}

		function onMouseMove(event: MouseEvent) {
			if (dragging) {
				imagePosition = {
					x: event.clientX - startPos.x,
					y: event.clientY - startPos.y
				};
			}
		}

		function onMouseUp() {
			dragging = false;
		}

		node.addEventListener('mousedown', onMouseDown);
		node.addEventListener('mousemove', onMouseMove);
		node.addEventListener('mouseup', onMouseUp);
		node.addEventListener('mouseleave', onMouseUp);

		return {
			destroy() {
				node.removeEventListener('mousedown', onMouseDown);
				node.removeEventListener('mousemove', onMouseMove);
				node.removeEventListener('mouseup', onMouseUp);
				node.removeEventListener('mouseleave', onMouseUp);
			}
		};
	}

	// Version selection
	function selectVersion(version: RoomResource) {
		onVersionSelect?.(version);
	}

	// Lifecycle - keyboard event handling
	$effect(() => {
		if (!open) return;
		document.addEventListener('keydown', handleKeydown);
		return () => {
			document.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

{#if open}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm"
		onclick={handleClose}
		onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleClose()}
		role="button"
		tabindex="0"
		aria-label="Close viewer"
	></div>

	<!-- Modal -->
	<div class="fixed inset-4 z-50 flex flex-col overflow-hidden rounded-xl bg-white shadow-2xl dark:bg-gray-900 lg:inset-8">
		<!-- Header -->
		<header class="flex items-center justify-between border-b border-gray-200 px-4 py-3 dark:border-gray-700">
			<div class="flex items-center gap-3">
				<!-- Resource type icon -->
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30">
					{#if isVideo}
						<svg class="h-5 w-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
						</svg>
					{:else if isPdf}
						<svg class="h-5 w-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
						</svg>
					{:else if isImage}
						<svg class="h-5 w-5 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
						</svg>
					{:else}
						<svg class="h-5 w-5 text-gray-600 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
						</svg>
					{/if}
				</div>

				<!-- Title and meta -->
				<div>
					<h2 class="text-lg font-semibold text-gray-900 dark:text-white">{resource.title}</h2>
					<div class="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
						<span>{resource.formatted_date}</span>
						{#if resource.formatted_size}
							<span class="text-gray-300 dark:text-gray-600">|</span>
							<span>{resource.formatted_size}</span>
						{/if}
						{#if resource.version && resource.version > 1}
							<span class="text-gray-300 dark:text-gray-600">|</span>
							<span class="text-blue-600 dark:text-blue-400">v{resource.version}</span>
						{/if}
					</div>
				</div>
			</div>

			<!-- Actions -->
			<div class="flex items-center gap-2">
				<!-- Access level badge -->
				<span class="rounded-md px-2 py-1 text-xs font-medium {isPremium ? 'bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-300' : 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-300'}">
					{isPremium ? 'Premium' : 'Free'}
				</span>

				<!-- Download button -->
				<button
					class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:opacity-50"
					onclick={handleDownload}
					disabled={downloading}
				>
					{#if downloading}
						<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Downloading...
					{:else}
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
						</svg>
						Download
					{/if}
				</button>

				<!-- Close button -->
				<button
					class="rounded-lg p-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
					onclick={handleClose}
					aria-label="Close"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		</header>

		<!-- Content -->
		<div class="flex flex-1 overflow-hidden">
			<!-- Main preview area -->
			<div class="flex-1 overflow-auto bg-gray-100 dark:bg-gray-800">
				{#if isVideo}
					<!-- Video player -->
					<div class="flex h-full w-full items-center justify-center p-4">
						<div class="aspect-video w-full max-w-5xl overflow-hidden rounded-lg bg-black shadow-lg">
							<iframe
								src={resource.embed_url}
								class="h-full w-full"
								allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
								allowfullscreen
								title={resource.title}
							></iframe>
						</div>
					</div>
				{:else if isPdf}
					<!-- PDF viewer -->
					<div class="h-full w-full p-4">
						<iframe
							src="{resource.file_url}#view=FitH"
							class="h-full w-full rounded-lg border border-gray-200 bg-white dark:border-gray-700"
							title={resource.title}
						></iframe>
					</div>
				{:else if isImage}
					<!-- Image viewer with zoom - using Svelte action for a11y compliance -->
					<div
						class="relative flex h-full w-full items-center justify-center overflow-hidden p-4"
						role="group"
						aria-label="Image viewer with zoom controls"
						use:panZoomAction
					>
						<img
							src={resource.file_url}
							alt={resource.title}
							class="max-h-full max-w-full object-contain transition-transform duration-200"
							style="transform: scale({imageZoom}) translate({imagePosition.x / imageZoom}px, {imagePosition.y / imageZoom}px); cursor: {imageZoom > 1 ? (dragging ? 'grabbing' : 'grab') : 'default'}"
							draggable="false"
						/>

						<!-- Zoom controls -->
						<div class="absolute bottom-4 left-1/2 flex -translate-x-1/2 items-center gap-2 rounded-lg bg-black/70 p-2 backdrop-blur-sm">
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={zoomOut}
								disabled={imageZoom <= 0.5}
								aria-label="Zoom out"
							>
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
								</svg>
							</button>
							<span class="min-w-[3rem] text-center text-sm text-white">{Math.round(imageZoom * 100)}%</span>
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={zoomIn}
								disabled={imageZoom >= 3}
								aria-label="Zoom in"
							>
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
								</svg>
							</button>
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={resetZoom}
								aria-label="Reset zoom"
							>
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
								</svg>
							</button>
						</div>
					</div>
				{:else}
					<!-- Non-previewable content -->
					<div class="flex h-full w-full flex-col items-center justify-center p-8 text-center">
						<div class="mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-gray-200 dark:bg-gray-700">
							<svg class="h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
							</svg>
						</div>
						<h3 class="mb-2 text-xl font-semibold text-gray-900 dark:text-white">Preview not available</h3>
						<p class="mb-6 max-w-md text-gray-600 dark:text-gray-400">
							This file type cannot be previewed in the browser. Click the download button to save it to your device.
						</p>
						<div class="flex flex-col items-center gap-2">
							<span class="text-sm text-gray-500">File type: <strong>{resource.mime_type || 'Unknown'}</strong></span>
							{#if resource.formatted_size}
								<span class="text-sm text-gray-500">Size: <strong>{resource.formatted_size}</strong></span>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Sidebar (description & version history) -->
			<aside class="hidden w-80 flex-shrink-0 overflow-y-auto border-l border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900 lg:block">
				<!-- Description -->
				{#if resource.description}
					<div class="mb-6">
						<h3 class="mb-2 text-sm font-semibold text-gray-900 dark:text-white">Description</h3>
						<p class="text-sm text-gray-600 dark:text-gray-400">{resource.description}</p>
					</div>
				{/if}

				<!-- Tags -->
				{#if resource.tags && resource.tags.length > 0}
					<div class="mb-6">
						<h3 class="mb-2 text-sm font-semibold text-gray-900 dark:text-white">Tags</h3>
						<div class="flex flex-wrap gap-1">
							{#each resource.tags as tag}
								<span class="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600 dark:bg-gray-800 dark:text-gray-400">
									{tag}
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Stats -->
				<div class="mb-6">
					<h3 class="mb-2 text-sm font-semibold text-gray-900 dark:text-white">Statistics</h3>
					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-gray-500 dark:text-gray-400">Views</span>
							<span class="font-medium text-gray-900 dark:text-white">{resource.views_count}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-500 dark:text-gray-400">Downloads</span>
							<span class="font-medium text-gray-900 dark:text-white">{resource.downloads_count}</span>
						</div>
						{#if resource.difficulty_level}
							<div class="flex justify-between">
								<span class="text-gray-500 dark:text-gray-400">Difficulty</span>
								<span class="font-medium capitalize text-gray-900 dark:text-white">{resource.difficulty_level}</span>
							</div>
						{/if}
					</div>
				</div>

				<!-- Version history -->
				{#if showVersionHistory}
					<div>
						<h3 class="mb-2 text-sm font-semibold text-gray-900 dark:text-white">Version History</h3>
						{#if loadingVersions}
							<div class="space-y-2">
								{#each [1, 2, 3] as _}
									<div class="animate-pulse rounded-lg bg-gray-100 p-3 dark:bg-gray-800">
										<div class="mb-1 h-4 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
										<div class="h-3 w-24 rounded bg-gray-200 dark:bg-gray-700"></div>
									</div>
								{/each}
							</div>
						{:else if versions.length > 0}
							<div class="space-y-2">
								{#each versions as version}
									<button
										class="w-full rounded-lg border p-3 text-left transition-colors {version.id === resource.id ? 'border-blue-500 bg-blue-50 dark:border-blue-400 dark:bg-blue-900/20' : 'border-gray-200 hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800'}"
										onclick={() => selectVersion(version)}
									>
										<div class="flex items-center justify-between">
											<span class="text-sm font-medium text-gray-900 dark:text-white">
												Version {version.version}
											</span>
											{#if version.is_latest_version}
												<span class="rounded bg-green-100 px-1.5 py-0.5 text-xs text-green-700 dark:bg-green-900/30 dark:text-green-300">
													Latest
												</span>
											{/if}
										</div>
										<span class="text-xs text-gray-500 dark:text-gray-400">{version.created_at}</span>
									</button>
								{/each}
							</div>
						{:else}
							<p class="text-sm text-gray-500 dark:text-gray-400">No version history available.</p>
						{/if}
					</div>
				{/if}
			</aside>
		</div>
	</div>
{/if}

<style>
	/* Prevent body scroll when modal is open */
	:global(body:has(.resource-viewer-open)) {
		overflow: hidden;
	}
</style>
