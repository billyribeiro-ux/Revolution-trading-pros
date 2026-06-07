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
	import Icon from '$lib/components/Icon.svelte';
	import type { Attachment } from 'svelte/attachments';

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
		if (!open) return;
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

	function panZoomAttachment(): Attachment<HTMLElement> {
		return (node) => {
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

			return () => {
				node.removeEventListener('mousedown', onMouseDown);
				node.removeEventListener('mousemove', onMouseMove);
				node.removeEventListener('mouseup', onMouseUp);
				node.removeEventListener('mouseleave', onMouseUp);
			};
		};
	}

	function loadVersionsAttachment(): Attachment<HTMLElement> {
		return () => {
			if (!showVersionHistory || versions.length > 0) return;

			let cancelled = false;
			loadingVersions = true;

			void (async () => {
				try {
					const response = await fetch(`/api/room-resources/${resource.id}/versions`);
					const data = await response.json();
					if (!cancelled && data.success) {
						versions = data.data;
					}
				} catch (e) {
					console.error('Failed to load version history:', e);
				} finally {
					if (!cancelled) {
						loadingVersions = false;
					}
				}
			})();

			return () => {
				cancelled = true;
			};
		};
	}

	// Version selection
	function selectVersion(version: RoomResource) {
		onVersionSelect?.(version);
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!--
		Backdrop as <button> so screen readers / keyboard users can dismiss it
		(replaces a <div onclick> which needed two svelte-ignore directives).
		The button is purely visual; the actual focus management lives on the
		modal below.
	-->
	<button
		type="button"
		class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm border-0 cursor-default"
		aria-label="Close viewer"
		onclick={handleClose}
	></button>

	<!-- Modal -->
	<div
		{@attach loadVersionsAttachment()}
		class="fixed inset-4 z-50 flex flex-col overflow-hidden rounded-xl bg-white shadow-2xl dark:bg-gray-900 lg:inset-8"
	>
		<!-- Header -->
		<header
			class="flex items-center justify-between border-b border-gray-200 px-4 py-3 dark:border-gray-700"
		>
			<div class="flex items-center gap-3">
				<!-- Resource type icon -->
				<div
					class="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-100 dark:bg-blue-900/30"
				>
					{#if isVideo}
						<Icon name="IconVideo" size={20} class="text-blue-600 dark:text-blue-400" />
					{:else if isPdf}
						<Icon name="IconPdf" size={20} class="text-red-600 dark:text-red-400" />
					{:else if isImage}
						<Icon name="IconPhoto" size={20} class="text-green-600 dark:text-green-400" />
					{:else}
						<Icon name="IconFileText" size={20} class="text-gray-600 dark:text-gray-400" />
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
				<span
					class={[
						'rounded-md px-2 py-1 text-xs font-medium',
						isPremium
							? 'bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-300'
							: 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-300'
					]}
				>
					{isPremium ? 'Premium' : 'Free'}
				</span>

				<!-- Download button -->
				<button
					class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:opacity-50"
					onclick={handleDownload}
					disabled={downloading}
				>
					{#if downloading}
						<Icon name="IconLoader2" size={16} class="animate-spin" />
						Downloading...
					{:else}
						<Icon name="IconDownload" size={16} />
						Download
					{/if}
				</button>

				<!-- Close button -->
				<button
					class="rounded-lg p-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
					onclick={handleClose}
					aria-label="Close"
				>
					<Icon name="IconX" size={20} />
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
						<div
							class="aspect-video w-full max-w-5xl overflow-hidden rounded-lg bg-black shadow-lg"
						>
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
							src={`${resource.file_url}#view=FitH`}
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
						{@attach panZoomAttachment()}
					>
						<!-- TODO(cls): fullscreen image viewer; intrinsic dims of arbitrary user uploads unknown -->
						<img
							src={resource.file_url}
							alt={resource.title}
							class="max-h-full max-w-full object-contain transition-transform duration-200"
							style:transform={`scale(${imageZoom}) translate(${imagePosition.x / imageZoom}px, ${imagePosition.y / imageZoom}px)`}
							style:cursor={imageZoom > 1 ? (dragging ? 'grabbing' : 'grab') : 'default'}
							draggable="false"
							loading="lazy"
						/>

						<!-- Zoom controls -->
						<div
							class="absolute bottom-4 left-1/2 flex -translate-x-1/2 items-center gap-2 rounded-lg bg-black/70 p-2 backdrop-blur-sm"
						>
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={zoomOut}
								disabled={imageZoom <= 0.5}
								aria-label="Zoom out"
							>
								<Icon name="IconMinus" size={20} />
							</button>
							<span class="min-w-[3rem] text-center text-sm text-white"
								>{Math.round(imageZoom * 100)}%</span
							>
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={zoomIn}
								disabled={imageZoom >= 3}
								aria-label="Zoom in"
							>
								<Icon name="IconPlus" size={20} />
							</button>
							<button
								class="rounded p-1 text-white hover:bg-white/20"
								onclick={resetZoom}
								aria-label="Reset zoom"
							>
								<Icon name="IconRefresh" size={20} />
							</button>
						</div>
					</div>
				{:else}
					<!-- Non-previewable content -->
					<div class="flex h-full w-full flex-col items-center justify-center p-8 text-center">
						<div
							class="mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-gray-200 dark:bg-gray-700"
						>
							<Icon name="IconFileText" size={48} class="text-gray-400" />
						</div>
						<h3 class="mb-2 text-xl font-semibold text-gray-900 dark:text-white">
							Preview not available
						</h3>
						<p class="mb-6 max-w-md text-gray-600 dark:text-gray-400">
							This file type cannot be previewed in the browser. Click the download button to save
							it to your device.
						</p>
						<div class="flex flex-col items-center gap-2">
							<span class="text-sm text-gray-500"
								>File type: <strong>{resource.mime_type || 'Unknown'}</strong></span
							>
							{#if resource.formatted_size}
								<span class="text-sm text-gray-500"
									>Size: <strong>{resource.formatted_size}</strong></span
								>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Sidebar (description & version history) -->
			<aside
				class="hidden w-80 flex-shrink-0 overflow-y-auto border-l border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-900 lg:block"
			>
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
							{#each resource.tags as tag (tag)}
								<span
									class="rounded bg-gray-100 px-2 py-0.5 text-xs text-gray-600 dark:bg-gray-800 dark:text-gray-400"
								>
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
							<span class="font-medium text-gray-900 dark:text-white"
								>{resource.downloads_count}</span
							>
						</div>
						{#if resource.difficulty_level}
							<div class="flex justify-between">
								<span class="text-gray-500 dark:text-gray-400">Difficulty</span>
								<span class="font-medium capitalize text-gray-900 dark:text-white"
									>{resource.difficulty_level}</span
								>
							</div>
						{/if}
					</div>
				</div>

				<!-- Version history -->
				{#if showVersionHistory}
					<div>
						<h3 class="mb-2 text-sm font-semibold text-gray-900 dark:text-white">
							Version History
						</h3>
						{#if loadingVersions}
							<div class="space-y-2">
								{#each [1, 2, 3] as _, i (i)}
									<div class="animate-pulse rounded-lg bg-gray-100 p-3 dark:bg-gray-800">
										<div class="mb-1 h-4 w-16 rounded bg-gray-200 dark:bg-gray-700"></div>
										<div class="h-3 w-24 rounded bg-gray-200 dark:bg-gray-700"></div>
									</div>
								{/each}
							</div>
						{:else if versions.length > 0}
							<div class="space-y-2">
								{#each versions as version (version.id)}
									<button
										class={[
											'w-full rounded-lg border p-3 text-left transition-colors',
											version.id === resource.id
												? 'border-blue-500 bg-blue-50 dark:border-blue-400 dark:bg-blue-900/20'
												: 'border-gray-200 hover:bg-gray-50 dark:border-gray-700 dark:hover:bg-gray-800'
										]}
										onclick={() => selectVersion(version)}
									>
										<div class="flex items-center justify-between">
											<span class="text-sm font-medium text-gray-900 dark:text-white">
												Version {version.version}
											</span>
											{#if version.is_latest_version}
												<span
													class="rounded bg-green-100 px-1.5 py-0.5 text-xs text-green-700 dark:bg-green-900/30 dark:text-green-300"
												>
													Latest
												</span>
											{/if}
										</div>
										<span class="text-xs text-gray-500 dark:text-gray-400"
											>{version.created_at}</span
										>
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
