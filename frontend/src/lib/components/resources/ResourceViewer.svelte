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
	import { logger } from '$lib/utils/logger';
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
			logger.error('Failed to load version history:', e);
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
			logger.error('Download failed:', e);
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
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="rv-backdrop" onclick={handleClose} aria-label="Close viewer"></div>

	<div class="rv-modal">
		<!-- Header -->
		<header class="rv-header">
			<div class="rv-header-left">
				<div
					class="rv-type-icon"
					data-type={isVideo ? 'video' : isPdf ? 'pdf' : isImage ? 'image' : 'default'}
				>
					{#if isVideo}
						<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
							/>
						</svg>
					{:else if isPdf}
						<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
							/>
						</svg>
					{:else if isImage}
						<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
							/>
						</svg>
					{:else}
						<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
							/>
						</svg>
					{/if}
				</div>

				<div>
					<h2 class="rv-title">{resource.title}</h2>
					<div class="rv-meta-row">
						<span>{resource.formatted_date}</span>
						{#if resource.formatted_size}
							<span class="rv-sep">|</span>
							<span>{resource.formatted_size}</span>
						{/if}
						{#if resource.version && resource.version > 1}
							<span class="rv-sep">|</span>
							<span class="rv-version">v{resource.version}</span>
						{/if}
					</div>
				</div>
			</div>

			<div class="rv-header-actions">
				<span class="rv-access-badge" data-premium={isPremium || undefined}>
					{isPremium ? 'Premium' : 'Free'}
				</span>

				<button class="rv-download-btn" onclick={handleDownload} disabled={downloading}>
					{#if downloading}
						<svg class="rv-icon-sm rv-spin" fill="none" viewBox="0 0 24 24">
							<circle
								class="rv-spinner-track"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="rv-spinner-fill"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Downloading...
					{:else}
						<svg class="rv-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
							/>
						</svg>
						Download
					{/if}
				</button>

				<button class="rv-close-btn" onclick={handleClose} aria-label="Close">
					<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
		</header>

		<!-- Content -->
		<div class="rv-body">
			<div class="rv-preview">
				{#if isVideo}
					<div class="rv-center-content">
						<div class="rv-video-container">
							<iframe
								src={resource.embed_url}
								class="rv-iframe"
								allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
								allowfullscreen
								title={resource.title}
							></iframe>
						</div>
					</div>
				{:else if isPdf}
					<div class="rv-pdf-container">
						<iframe src="{resource.file_url}#view=FitH" class="rv-pdf-iframe" title={resource.title}
						></iframe>
					</div>
				{:else if isImage}
					<div
						class="rv-image-container"
						role="group"
						aria-label="Image viewer with zoom controls"
						use:panZoomAction
					>
						<img
							src={resource.file_url}
							alt={resource.title}
							width="1600"
							height="1200"
							loading="eager"
							fetchpriority="high"
							decoding="sync"
							class="max-h-full max-w-full object-contain transition-transform duration-200"
							style="transform: scale({imageZoom}) translate({imagePosition.x /
								imageZoom}px, {imagePosition.y / imageZoom}px); cursor: {imageZoom > 1
								? dragging
									? 'grabbing'
									: 'grab'
								: 'default'}"
							draggable="false"
						/>

						<div class="rv-zoom-controls">
							<button
								class="rv-zoom-btn"
								onclick={zoomOut}
								disabled={imageZoom <= 0.5}
								aria-label="Zoom out"
							>
								<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M20 12H4"
									/>
								</svg>
							</button>
							<span class="rv-zoom-label">{Math.round(imageZoom * 100)}%</span>
							<button
								class="rv-zoom-btn"
								onclick={zoomIn}
								disabled={imageZoom >= 3}
								aria-label="Zoom in"
							>
								<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 4v16m8-8H4"
									/>
								</svg>
							</button>
							<button class="rv-zoom-btn" onclick={resetZoom} aria-label="Reset zoom">
								<svg class="rv-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
									/>
								</svg>
							</button>
						</div>
					</div>
				{:else}
					<div class="rv-no-preview">
						<div class="rv-no-preview-icon">
							<svg class="rv-icon-xxl" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
								/>
							</svg>
						</div>
						<h3 class="rv-no-preview-title">Preview not available</h3>
						<p class="rv-no-preview-desc">
							This file type cannot be previewed in the browser. Click the download button to save
							it to your device.
						</p>
						<div class="rv-file-info">
							<span>File type: <strong>{resource.mime_type || 'Unknown'}</strong></span>
							{#if resource.formatted_size}
								<span>Size: <strong>{resource.formatted_size}</strong></span>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Sidebar -->
			<aside class="rv-sidebar">
				{#if resource.description}
					<div class="rv-sidebar-section">
						<h3 class="rv-sidebar-heading">Description</h3>
						<p class="rv-sidebar-text">{resource.description}</p>
					</div>
				{/if}

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

				<div class="rv-sidebar-section">
					<h3 class="rv-sidebar-heading">Statistics</h3>
					<div class="rv-stat-list">
						<div class="rv-stat-row">
							<span class="rv-stat-label">Views</span>
							<span class="rv-stat-value">{resource.views_count}</span>
						</div>
						<div class="rv-stat-row">
							<span class="rv-stat-label">Downloads</span>
							<span class="rv-stat-value">{resource.downloads_count}</span>
						</div>
						{#if resource.difficulty_level}
							<div class="rv-stat-row">
								<span class="rv-stat-label">Difficulty</span>
								<span class="rv-stat-value rv-capitalize">{resource.difficulty_level}</span>
							</div>
						{/if}
					</div>
				</div>

				{#if showVersionHistory}
					<div class="rv-sidebar-section">
						<h3 class="rv-sidebar-heading">Version History</h3>
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
										class="rv-ver-btn"
										data-active={version.id === resource.id || undefined}
										onclick={() => selectVersion(version)}
									>
										<div class="rv-ver-row">
											<span class="rv-ver-name">Version {version.version}</span>
											{#if version.is_latest_version}
												<span class="rv-latest-badge">Latest</span>
											{/if}
										</div>
										<span class="rv-ver-date">{version.created_at}</span>
									</button>
								{/each}
							</div>
						{:else}
							<p class="rv-sidebar-text">No version history available.</p>
						{/if}
					</div>
				{/if}
			</aside>
		</div>
	</div>
{/if}

<style>
	.rv-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		background-color: oklch(0 0 0 / 80%);
		backdrop-filter: blur(4px);
	}

	.rv-modal {
		position: fixed;
		inset: 1rem;
		z-index: 50;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		border-radius: var(--radius-xl);
		background-color: oklch(1 0 0);
		box-shadow: 0 25px 50px oklch(0 0 0 / 25%);

		@media (min-width: 1024px) {
			inset: 2rem;
		}
	}

	/* ─── Header ─── */
	.rv-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		border-block-end: 1px solid oklch(0.9 0.005 265);
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
	}

	.rv-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.rv-type-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);

		&[data-type='video'] {
			background-color: oklch(0.92 0.06 260);
			color: oklch(0.5 0.2 260);
		}
		&[data-type='pdf'] {
			background-color: oklch(0.92 0.06 25);
			color: oklch(0.5 0.2 25);
		}
		&[data-type='image'] {
			background-color: oklch(0.92 0.06 160);
			color: oklch(0.5 0.18 160);
		}
		&[data-type='default'] {
			background-color: oklch(0.95 0.002 265);
			color: oklch(0.45 0.01 265);
		}
	}

	.rv-icon {
		inline-size: 1.25rem;
		block-size: 1.25rem;
	}
	.rv-icon-sm {
		inline-size: 1rem;
		block-size: 1rem;
	}
	.rv-icon-xxl {
		inline-size: 3rem;
		block-size: 3rem;
		color: oklch(0.65 0.01 265);
	}

	.rv-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.rv-meta-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}

	.rv-sep {
		color: oklch(0.82 0.005 265);
	}
	.rv-version {
		color: oklch(0.5 0.2 260);
	}

	.rv-header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.rv-access-badge {
		border-radius: var(--radius-md);
		padding-inline: var(--space-2);
		padding-block: 0.25rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		background-color: oklch(0.92 0.06 160);
		color: oklch(0.35 0.15 160);

		&[data-premium] {
			background-color: oklch(0.92 0.08 80);
			color: oklch(0.45 0.15 80);
		}
	}

	.rv-download-btn {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		border-radius: var(--radius-lg);
		background-color: oklch(0.55 0.2 260);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		border: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.48 0.2 260);
		}
		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}

	.rv-close-btn {
		border-radius: var(--radius-lg);
		padding: var(--space-2);
		color: oklch(0.55 0.01 265);
		background: none;
		border: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.95 0.002 265);
		}
	}

	/* ─── Body / Preview ─── */
	.rv-body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.rv-preview {
		flex: 1;
		overflow: auto;
		background-color: oklch(0.96 0.002 265);
	}

	.rv-center-content {
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 100%;
		block-size: 100%;
		padding: var(--space-4);
	}

	.rv-video-container {
		aspect-ratio: 16 / 9;
		inline-size: 100%;
		max-inline-size: 80rem;
		overflow: hidden;
		border-radius: var(--radius-lg);
		background-color: oklch(0 0 0);
		box-shadow: 0 10px 25px oklch(0 0 0 / 15%);
	}

	.rv-iframe {
		inline-size: 100%;
		block-size: 100%;
		border: none;
	}

	.rv-pdf-container {
		inline-size: 100%;
		block-size: 100%;
		padding: var(--space-4);
	}

	.rv-pdf-iframe {
		inline-size: 100%;
		block-size: 100%;
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(1 0 0);
	}

	.rv-image-container {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 100%;
		block-size: 100%;
		overflow: hidden;
		padding: var(--space-4);
	}

	.rv-preview-img {
		max-block-size: 100%;
		max-inline-size: 100%;
		object-fit: contain;
		transition: transform 200ms var(--ease-default);
	}

	/* ─── Zoom controls ─── */
	.rv-zoom-controls {
		position: absolute;
		inset-block-end: var(--space-4);
		inset-inline-start: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: var(--space-2);
		border-radius: var(--radius-lg);
		background-color: oklch(0 0 0 / 70%);
		padding: var(--space-2);
		backdrop-filter: blur(4px);
	}

	.rv-zoom-btn {
		border-radius: var(--radius-sm);
		padding: 0.25rem;
		color: oklch(1 0 0);
		background: none;
		border: none;
		cursor: pointer;

		&:hover {
			background-color: oklch(1 0 0 / 20%);
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}

	.rv-zoom-label {
		min-inline-size: 3rem;
		text-align: center;
		font-size: var(--text-sm);
		color: oklch(1 0 0);
	}

	/* ─── No-preview fallback ─── */
	.rv-no-preview {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		inline-size: 100%;
		block-size: 100%;
		padding: var(--space-8);
		text-align: center;
	}

	.rv-no-preview-icon {
		margin-block-end: var(--space-6);
		display: flex;
		align-items: center;
		justify-content: center;
		inline-size: 6rem;
		block-size: 6rem;
		border-radius: 9999px;
		background-color: oklch(0.9 0.005 265);
	}

	.rv-no-preview-title {
		margin-block-end: var(--space-2);
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.rv-no-preview-desc {
		margin-block-end: var(--space-6);
		max-inline-size: 28rem;
		color: oklch(0.45 0.01 265);
	}

	.rv-file-info {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
	}

	/* ─── Sidebar ─── */
	.rv-sidebar {
		display: none;
		inline-size: 20rem;
		flex-shrink: 0;
		overflow-y: auto;
		border-inline-start: 1px solid oklch(0.9 0.005 265);
		background-color: oklch(1 0 0);
		padding: var(--space-4);

		@media (min-width: 1024px) {
			display: block;
		}
	}

	.rv-sidebar-section {
		margin-block-end: var(--space-6);
	}

	.rv-sidebar-heading {
		margin-block-end: var(--space-2);
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.rv-sidebar-text {
		font-size: var(--text-sm);
		color: oklch(0.45 0.01 265);
	}

	.rv-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.rv-tag {
		border-radius: var(--radius-sm);
		background-color: oklch(0.95 0.002 265);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.01 265);
	}

	.rv-stat-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		font-size: var(--text-sm);
	}
	.rv-stat-row {
		display: flex;
		justify-content: space-between;
	}
	.rv-stat-label {
		color: oklch(0.55 0.01 265);
	}
	.rv-stat-value {
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}
	.rv-capitalize {
		text-transform: capitalize;
	}

	/* ─── Version history ─── */
	.rv-ver-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.rv-ver-btn {
		inline-size: 100%;
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.9 0.005 265);
		padding: var(--space-3);
		text-align: start;
		background: none;
		cursor: pointer;
		transition: background-color 200ms var(--ease-default);

		&:hover {
			background-color: oklch(0.97 0.002 265);
		}

		&[data-active] {
			border-color: oklch(0.6 0.2 260);
			background-color: oklch(0.96 0.03 260);
		}
	}

	.rv-ver-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.rv-ver-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}
	.rv-ver-date {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.rv-latest-badge {
		border-radius: var(--radius-sm);
		background-color: oklch(0.92 0.06 160);
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		color: oklch(0.4 0.15 160);
	}

	/* ─── Skeleton ─── */
	.rv-ver-skel {
		border-radius: var(--radius-lg);
		background-color: oklch(0.95 0.002 265);
		padding: var(--space-3);
		animation: pulse 2s ease-in-out infinite;
	}

	.rv-skel-line {
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
	}
	.rv-skel-w16 {
		block-size: 1rem;
		inline-size: 4rem;
		margin-block-end: 0.25rem;
	}
	.rv-skel-w24 {
		block-size: 0.75rem;
		inline-size: 6rem;
	}

	.rv-spin {
		animation: spin 1s linear infinite;
	}
	.rv-spinner-track {
		opacity: 0.25;
	}
	.rv-spinner-fill {
		opacity: 0.75;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
