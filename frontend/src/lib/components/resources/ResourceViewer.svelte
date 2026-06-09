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
	<button type="button" class="viewer-backdrop" aria-label="Close viewer" onclick={handleClose}
	></button>

	<!-- Modal -->
	<div {@attach loadVersionsAttachment()} class="resource-viewer-modal resource-viewer-open">
		<!-- Header -->
		<header class="viewer-header">
			<div class="viewer-heading">
				<!-- Resource type icon -->
				<div
					class={[
						'resource-type-icon',
						isVideo && 'resource-type-video',
						isPdf && 'resource-type-pdf',
						isImage && 'resource-type-image',
						!isVideo && !isPdf && !isImage && 'resource-type-file'
					]}
				>
					{#if isVideo}
						<Icon name="IconVideo" size={20} />
					{:else if isPdf}
						<Icon name="IconPdf" size={20} />
					{:else if isImage}
						<Icon name="IconPhoto" size={20} />
					{:else}
						<Icon name="IconFileText" size={20} />
					{/if}
				</div>

				<!-- Title and meta -->
				<div>
					<h2 class="viewer-title">{resource.title}</h2>
					<div class="resource-meta">
						<span>{resource.formatted_date}</span>
						{#if resource.formatted_size}
							<span class="resource-meta-separator">|</span>
							<span>{resource.formatted_size}</span>
						{/if}
						{#if resource.version && resource.version > 1}
							<span class="resource-meta-separator">|</span>
							<span class="resource-version">v{resource.version}</span>
						{/if}
					</div>
				</div>
			</div>

			<!-- Actions -->
			<div class="viewer-actions">
				<!-- Access level badge -->
				<span class={['access-badge', isPremium ? 'access-badge-premium' : 'access-badge-free']}>
					{isPremium ? 'Premium' : 'Free'}
				</span>

				<!-- Download button -->
				<button class="download-button" onclick={handleDownload} disabled={downloading}>
					{#if downloading}
						<Icon name="IconLoader2" size={16} class="spinner-icon" />
						Downloading...
					{:else}
						<Icon name="IconDownload" size={16} />
						Download
					{/if}
				</button>

				<!-- Close button -->
				<button class="close-button" onclick={handleClose} aria-label="Close">
					<Icon name="IconX" size={20} />
				</button>
			</div>
		</header>

		<!-- Content -->
		<div class="viewer-content">
			<!-- Main preview area -->
			<div class="preview-area">
				{#if isVideo}
					<!-- Video player -->
					<div class="video-preview">
						<div class="video-frame">
							<iframe
								src={resource.embed_url}
								class="media-frame"
								allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
								allowfullscreen
								title={resource.title}
							></iframe>
						</div>
					</div>
				{:else if isPdf}
					<!-- PDF viewer -->
					<div class="pdf-preview">
						<iframe src={`${resource.file_url}#view=FitH`} class="pdf-frame" title={resource.title}
						></iframe>
					</div>
				{:else if isImage}
					<!-- Image viewer with zoom - using Svelte action for a11y compliance -->
					<div
						class="image-preview"
						role="group"
						aria-label="Image viewer with zoom controls"
						{@attach panZoomAttachment()}
					>
						<!-- TODO(cls): fullscreen image viewer; intrinsic dims of arbitrary user uploads unknown -->
						<img
							src={resource.file_url}
							alt={resource.title}
							class="preview-image"
							style:transform={`scale(${imageZoom}) translate(${imagePosition.x / imageZoom}px, ${imagePosition.y / imageZoom}px)`}
							style:cursor={imageZoom > 1 ? (dragging ? 'grabbing' : 'grab') : 'default'}
							draggable="false"
							loading="lazy"
						/>

						<!-- Zoom controls -->
						<div class="zoom-controls">
							<button
								class="zoom-button"
								onclick={zoomOut}
								disabled={imageZoom <= 0.5}
								aria-label="Zoom out"
							>
								<Icon name="IconMinus" size={20} />
							</button>
							<span class="zoom-value">{Math.round(imageZoom * 100)}%</span>
							<button
								class="zoom-button"
								onclick={zoomIn}
								disabled={imageZoom >= 3}
								aria-label="Zoom in"
							>
								<Icon name="IconPlus" size={20} />
							</button>
							<button class="zoom-button" onclick={resetZoom} aria-label="Reset zoom">
								<Icon name="IconRefresh" size={20} />
							</button>
						</div>
					</div>
				{:else}
					<!-- Non-previewable content -->
					<div class="empty-preview">
						<div class="empty-preview-icon">
							<Icon name="IconFileText" size={48} />
						</div>
						<h3 class="empty-preview-title">Preview not available</h3>
						<p class="empty-preview-copy">
							This file type cannot be previewed in the browser. Click the download button to save
							it to your device.
						</p>
						<div class="empty-preview-meta">
							<span>File type: <strong>{resource.mime_type || 'Unknown'}</strong></span>
							{#if resource.formatted_size}
								<span>Size: <strong>{resource.formatted_size}</strong></span>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Sidebar (description & version history) -->
			<aside class="viewer-sidebar">
				<!-- Description -->
				{#if resource.description}
					<div class="sidebar-section">
						<h3 class="sidebar-heading">Description</h3>
						<p class="sidebar-copy">{resource.description}</p>
					</div>
				{/if}

				<!-- Tags -->
				{#if resource.tags && resource.tags.length > 0}
					<div class="sidebar-section">
						<h3 class="sidebar-heading">Tags</h3>
						<div class="tag-list">
							{#each resource.tags as tag (tag)}
								<span class="resource-tag">{tag}</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Stats -->
				<div class="sidebar-section">
					<h3 class="sidebar-heading">Statistics</h3>
					<div class="stat-list">
						<div class="stat-row">
							<span class="stat-label">Views</span>
							<span class="stat-value">{resource.views_count}</span>
						</div>
						<div class="stat-row">
							<span class="stat-label">Downloads</span>
							<span class="stat-value">{resource.downloads_count}</span>
						</div>
						{#if resource.difficulty_level}
							<div class="stat-row">
								<span class="stat-label">Difficulty</span>
								<span class="stat-value stat-value-capitalize">{resource.difficulty_level}</span>
							</div>
						{/if}
					</div>
				</div>

				<!-- Version history -->
				{#if showVersionHistory}
					<div>
						<h3 class="sidebar-heading">Version History</h3>
						{#if loadingVersions}
							<div class="version-list">
								{#each [1, 2, 3] as _, i (i)}
									<div class="version-skeleton">
										<div class="version-skeleton-title"></div>
										<div class="version-skeleton-date"></div>
									</div>
								{/each}
							</div>
						{:else if versions.length > 0}
							<div class="version-list">
								{#each versions as version (version.id)}
									<button
										class={[
											'version-button',
											version.id === resource.id && 'version-button-current'
										]}
										onclick={() => selectVersion(version)}
									>
										<div class="version-row">
											<span class="version-title">Version {version.version}</span>
											{#if version.is_latest_version}
												<span class="latest-badge">Latest</span>
											{/if}
										</div>
										<span class="version-date">{version.created_at}</span>
									</button>
								{/each}
							</div>
						{:else}
							<p class="empty-version-copy">No version history available.</p>
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

	.viewer-backdrop {
		position: fixed;
		z-index: 50;
		inset: 0;
		cursor: default;
		border: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
	}

	.resource-viewer-modal {
		position: fixed;
		z-index: 50;
		inset: 1rem;
		display: flex;
		overflow: hidden;
		flex-direction: column;
		border-radius: 0.75rem;
		background: white;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.35),
			0 0 0 1px rgba(15, 23, 42, 0.08);
	}

	.viewer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.viewer-heading,
	.viewer-actions,
	.resource-meta,
	.download-button,
	.video-preview,
	.image-preview,
	.empty-preview,
	.empty-preview-icon,
	.tag-list,
	.stat-row,
	.version-row {
		display: flex;
	}

	.viewer-heading {
		align-items: center;
		gap: 0.75rem;
		min-width: 0;
	}

	.resource-type-icon {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
		background: #dbeafe;
	}

	.resource-type-video {
		color: #2563eb;
	}

	.resource-type-pdf {
		color: #dc2626;
	}

	.resource-type-image {
		color: #16a34a;
	}

	.resource-type-file {
		color: #4b5563;
	}

	.viewer-title {
		overflow: hidden;
		margin: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.35;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.resource-meta {
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.resource-meta-separator {
		color: #d1d5db;
	}

	.resource-version {
		color: #2563eb;
	}

	.viewer-actions {
		align-items: center;
		gap: 0.5rem;
	}

	.access-badge {
		border-radius: 0.375rem;
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.access-badge-premium {
		background: #fef3c7;
		color: #92400e;
	}

	.access-badge-free {
		background: #d1fae5;
		color: #065f46;
	}

	.download-button {
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #2563eb;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		transition: background 150ms ease;
	}

	.download-button:hover {
		background: #1d4ed8;
	}

	.download-button:disabled {
		opacity: 0.5;
	}

	:global(.spinner-icon) {
		animation: resource-viewer-spin 1s linear infinite;
	}

	.close-button {
		padding: 0.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #6b7280;
		transition:
			background 150ms ease,
			color 150ms ease;
	}

	.close-button:hover {
		background: #f3f4f6;
		color: #374151;
	}

	.viewer-content {
		display: flex;
		min-height: 0;
		flex: 1 1 auto;
		overflow: hidden;
	}

	.preview-area {
		flex: 1 1 auto;
		overflow: auto;
		background: #f3f4f6;
	}

	.video-preview,
	.image-preview,
	.empty-preview {
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
	}

	.video-preview,
	.pdf-preview,
	.image-preview {
		padding: 1rem;
	}

	.video-frame {
		overflow: hidden;
		width: 100%;
		max-width: 64rem;
		aspect-ratio: 16 / 9;
		border-radius: 0.5rem;
		background: black;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.35);
	}

	.media-frame,
	.pdf-frame {
		width: 100%;
		height: 100%;
	}

	.pdf-preview {
		width: 100%;
		height: 100%;
	}

	.pdf-frame {
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background: white;
	}

	.image-preview {
		position: relative;
		overflow: hidden;
	}

	.preview-image {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		transition: transform 200ms ease;
	}

	.zoom-controls {
		position: absolute;
		bottom: 1rem;
		left: 50%;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		border-radius: 0.5rem;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		transform: translateX(-50%);
	}

	.zoom-button {
		padding: 0.25rem;
		border: 0;
		border-radius: 0.25rem;
		background: transparent;
		color: white;
		transition: background 150ms ease;
	}

	.zoom-button:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.zoom-button:disabled {
		opacity: 0.45;
	}

	.zoom-value {
		min-width: 3rem;
		color: white;
		font-size: 0.875rem;
		text-align: center;
	}

	.empty-preview {
		flex-direction: column;
		padding: 2rem;
		text-align: center;
	}

	.empty-preview-icon {
		width: 6rem;
		height: 6rem;
		align-items: center;
		justify-content: center;
		margin-bottom: 1.5rem;
		border-radius: 999px;
		background: #e5e7eb;
		color: #9ca3af;
	}

	.empty-preview-title {
		margin: 0 0 0.5rem;
		color: #111827;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.empty-preview-copy {
		width: min(100%, 28rem);
		margin: 0 0 1.5rem;
		color: #4b5563;
	}

	.empty-preview-meta {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		color: #6b7280;
		font-size: 0.875rem;
	}

	.viewer-sidebar {
		display: none;
		width: 20rem;
		flex: 0 0 auto;
		overflow-y: auto;
		padding: 1rem;
		border-left: 1px solid #e5e7eb;
		background: white;
	}

	.sidebar-section {
		margin-bottom: 1.5rem;
	}

	.sidebar-heading {
		margin: 0 0 0.5rem;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.sidebar-copy,
	.empty-version-copy {
		margin: 0;
		color: #4b5563;
		font-size: 0.875rem;
	}

	.tag-list {
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.resource-tag {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		background: #f3f4f6;
		color: #4b5563;
		font-size: 0.75rem;
	}

	.stat-list,
	.version-list {
		display: grid;
		gap: 0.5rem;
		font-size: 0.875rem;
	}

	.stat-row,
	.version-row {
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.stat-label,
	.version-date {
		color: #6b7280;
	}

	.stat-value,
	.version-title {
		color: #111827;
		font-weight: 500;
	}

	.stat-value-capitalize {
		text-transform: capitalize;
	}

	.version-skeleton {
		padding: 0.75rem;
		border-radius: 0.5rem;
		background: #f3f4f6;
		animation: resource-viewer-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.version-skeleton-title,
	.version-skeleton-date {
		border-radius: 0.25rem;
		background: #e5e7eb;
	}

	.version-skeleton-title {
		width: 4rem;
		height: 1rem;
		margin-bottom: 0.25rem;
	}

	.version-skeleton-date {
		width: 6rem;
		height: 0.75rem;
	}

	.version-button {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		background: transparent;
		text-align: left;
		transition:
			background 150ms ease,
			border-color 150ms ease;
	}

	.version-button:hover {
		background: #f9fafb;
	}

	.version-button-current {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.latest-badge {
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		background: #dcfce7;
		color: #15803d;
		font-size: 0.75rem;
	}

	.version-date {
		font-size: 0.75rem;
	}

	@keyframes resource-viewer-spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes resource-viewer-pulse {
		50% {
			opacity: 0.5;
		}
	}

	@media (min-width: 1024px) {
		.resource-viewer-modal {
			inset: 2rem;
		}

		.viewer-sidebar {
			display: block;
		}
	}

	:global(.dark) .resource-viewer-modal {
		background: #111827;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.55),
			0 0 0 1px rgba(255, 255, 255, 0.08);
	}

	:global(.dark) .viewer-header,
	:global(.dark) .viewer-sidebar,
	:global(.dark) .pdf-frame {
		border-color: #374151;
	}

	:global(.dark) .viewer-title,
	:global(.dark) .empty-preview-title,
	:global(.dark) .sidebar-heading,
	:global(.dark) .stat-value,
	:global(.dark) .version-title {
		color: white;
	}

	:global(.dark) .resource-meta,
	:global(.dark) .sidebar-copy,
	:global(.dark) .empty-version-copy,
	:global(.dark) .empty-preview-copy {
		color: #9ca3af;
	}

	:global(.dark) .resource-meta-separator {
		color: #4b5563;
	}

	:global(.dark) .resource-version,
	:global(.dark) .resource-type-video {
		color: #60a5fa;
	}

	:global(.dark) .resource-type-icon {
		background: rgba(30, 58, 138, 0.3);
	}

	:global(.dark) .resource-type-pdf {
		color: #f87171;
	}

	:global(.dark) .resource-type-image {
		color: #4ade80;
	}

	:global(.dark) .resource-type-file,
	:global(.dark) .close-button,
	:global(.dark) .stat-label,
	:global(.dark) .version-date,
	:global(.dark) .empty-preview-meta {
		color: #9ca3af;
	}

	:global(.dark) .access-badge-premium {
		background: rgba(120, 53, 15, 0.3);
		color: #fcd34d;
	}

	:global(.dark) .access-badge-free {
		background: rgba(6, 78, 59, 0.3);
		color: #6ee7b7;
	}

	:global(.dark) .close-button:hover,
	:global(.dark) .resource-tag,
	:global(.dark) .version-skeleton {
		background: #1f2937;
	}

	:global(.dark) .preview-area {
		background: #1f2937;
	}

	:global(.dark) .pdf-frame,
	:global(.dark) .viewer-sidebar {
		background: #111827;
	}

	:global(.dark) .empty-preview-icon,
	:global(.dark) .version-skeleton-title,
	:global(.dark) .version-skeleton-date {
		background: #374151;
	}

	:global(.dark) .resource-tag {
		color: #9ca3af;
	}

	:global(.dark) .version-button {
		border-color: #374151;
		color: #f9fafb;
	}

	:global(.dark) .version-button:hover {
		background: #1f2937;
	}

	:global(.dark) .version-button-current {
		border-color: #60a5fa;
		background: rgba(30, 58, 138, 0.2);
	}

	:global(.dark) .latest-badge {
		background: rgba(20, 83, 45, 0.3);
		color: #86efac;
	}
</style>
