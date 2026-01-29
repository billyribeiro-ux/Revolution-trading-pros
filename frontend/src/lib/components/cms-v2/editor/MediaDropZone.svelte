<!--
/**
 * Media Drop Zone - Inline Media Insert System
 * ═══════════════════════════════════════════════════════════════════════════
 * Advanced drag & drop media insertion for CMS editor
 *
 * Features:
 * - Drag & drop anywhere in editor with visual drop indicator
 * - Paste image from clipboard with auto-upload
 * - URL paste detection for auto-embedding (YouTube, Vimeo, TradingView, Twitter)
 * - Upload progress indicator with cancellation support
 * - Responsive drop zone with between-block indicators
 *
 * @version 1.0.0
 */
-->

<script lang="ts">
	import { onMount, onDestroy, type Snippet } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';

	import {
		detectEmbed,
		isEmbedUrl,
		scanTextForEmbeds,
		type EmbedConfig
	} from '$lib/utils/media-embed-detector';

	// ==========================================================================
	// Types
	// ==========================================================================

	interface DropIndicatorPosition {
		top: number;
		visible: boolean;
		insertIndex: number;
	}

	interface UploadState {
		id: string;
		file: File;
		progress: number;
		status: 'pending' | 'uploading' | 'complete' | 'error';
		error?: string;
		abortController?: AbortController;
	}

	type DragEventHandler = (e: DragEvent) => void;
	type ClipboardEventHandler = (e: ClipboardEvent) => void;

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/**
		 * Callback when files are dropped
		 * @param files - Array of dropped files
		 * @param insertIndex - Index position to insert the media
		 */
		onFileDrop: (files: File[], insertIndex: number) => Promise<void>;

		/**
		 * Callback when an embeddable URL is pasted
		 * @param embedConfig - Configuration for the detected embed
		 * @param insertIndex - Index position to insert the embed
		 */
		onUrlPaste: (embedConfig: EmbedConfig, insertIndex: number) => void;

		/**
		 * Callback for upload progress updates
		 * @param progress - Upload progress percentage (0-100)
		 */
		onUploadProgress: (progress: number) => void;

		/**
		 * Content to render inside the drop zone
		 */
		children: Snippet;

		/**
		 * Array of block element references for calculating drop positions
		 */
		blockElements?: HTMLElement[];

		/**
		 * Current focused/selected block index
		 */
		focusedBlockIndex?: number;

		/**
		 * Whether the editor is in read-only mode
		 */
		readOnly?: boolean;

		/**
		 * Accepted file types (MIME types or extensions)
		 */
		acceptedTypes?: string[];

		/**
		 * Maximum file size in bytes (default: 50MB)
		 */
		maxFileSize?: number;

		/**
		 * Maximum number of files per drop
		 */
		maxFiles?: number;

		/**
		 * Custom class for the container
		 */
		class?: string;
	}

	let {
		onFileDrop,
		onUrlPaste,
		onUploadProgress,
		children,
		blockElements = [],
		focusedBlockIndex = 0,
		readOnly = false,
		acceptedTypes = ['image/*', 'video/*', 'audio/*', '.pdf', '.doc', '.docx'],
		maxFileSize = 50 * 1024 * 1024, // 50MB
		maxFiles = 10,
		class: className = ''
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let containerRef: HTMLDivElement;
	let isDragging = $state(false);
	let isDraggingOver = $state(false);
	let dragCounter = $state(0);
	let dropIndicator = $state<DropIndicatorPosition>({
		top: 0,
		visible: false,
		insertIndex: 0
	});
	let uploads = $state<UploadState[]>([]);
	let errorMessage = $state<string | null>(null);
	let showDropZone = $state(false);

	// ==========================================================================
	// Computed
	// ==========================================================================

	let hasActiveUploads = $derived(uploads.some((u) => u.status === 'uploading'));
	let totalProgress = $derived(() => {
		const uploading = uploads.filter((u) => u.status === 'uploading');
		if (uploading.length === 0) return 0;
		return uploading.reduce((sum, u) => sum + u.progress, 0) / uploading.length;
	});

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		if (!browser) return;

		// Add global drag listeners for better UX
		document.addEventListener('dragenter', handleDocumentDragEnter);
		document.addEventListener('dragleave', handleDocumentDragLeave);
		document.addEventListener('drop', handleDocumentDrop);
		document.addEventListener('paste', handlePaste as EventListener);
	});

	onDestroy(() => {
		if (!browser) return;

		document.removeEventListener('dragenter', handleDocumentDragEnter);
		document.removeEventListener('dragleave', handleDocumentDragLeave);
		document.removeEventListener('drop', handleDocumentDrop);
		document.removeEventListener('paste', handlePaste as EventListener);

		// Cancel any pending uploads
		uploads.forEach((upload) => {
			upload.abortController?.abort();
		});
	});

	// ==========================================================================
	// Drag Event Handlers
	// ==========================================================================

	function handleDocumentDragEnter(e: DragEvent) {
		if (readOnly) return;
		if (!hasFiles(e)) return;

		dragCounter++;
		if (dragCounter === 1) {
			isDragging = true;
			showDropZone = true;
		}
	}

	function handleDocumentDragLeave(e: DragEvent) {
		if (readOnly) return;

		dragCounter--;
		if (dragCounter === 0) {
			isDragging = false;
			showDropZone = false;
			dropIndicator.visible = false;
		}
	}

	function handleDocumentDrop(e: DragEvent) {
		dragCounter = 0;
		isDragging = false;
		showDropZone = false;
		dropIndicator.visible = false;
	}

	const handleDragEnter: DragEventHandler = (e) => {
		if (readOnly) return;

		e.preventDefault();
		e.stopPropagation();

		if (!hasFiles(e)) return;

		isDraggingOver = true;
		dropIndicator.visible = true;
		updateDropIndicator(e);
	};

	const handleDragOver: DragEventHandler = (e) => {
		if (readOnly) return;

		e.preventDefault();
		e.stopPropagation();

		if (!hasFiles(e)) return;

		// Update the visual indicator position
		updateDropIndicator(e);

		// Set drop effect
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'copy';
		}
	};

	const handleDragLeave: DragEventHandler = (e) => {
		if (readOnly) return;

		e.preventDefault();
		e.stopPropagation();

		// Only hide if we're leaving the container entirely
		const rect = containerRef?.getBoundingClientRect();
		if (rect) {
			const { clientX, clientY } = e;
			if (
				clientX < rect.left ||
				clientX > rect.right ||
				clientY < rect.top ||
				clientY > rect.bottom
			) {
				isDraggingOver = false;
				dropIndicator.visible = false;
			}
		}
	};

	const handleDrop: DragEventHandler = async (e) => {
		if (readOnly) return;

		e.preventDefault();
		e.stopPropagation();

		isDragging = false;
		isDraggingOver = false;
		showDropZone = false;

		const insertIndex = dropIndicator.insertIndex;
		dropIndicator.visible = false;

		// Handle dropped files
		const files = getFilesFromEvent(e);
		if (files.length > 0) {
			await processFiles(files, insertIndex);
			return;
		}

		// Check for dropped URLs
		const text = e.dataTransfer?.getData('text/plain');
		if (text && isEmbedUrl(text)) {
			const result = detectEmbed(text);
			if (result.isEmbed && result.config) {
				onUrlPaste(result.config, insertIndex);
			}
		}
	};

	// ==========================================================================
	// Paste Handler
	// ==========================================================================

	const handlePaste: ClipboardEventHandler = async (e) => {
		if (readOnly) return;

		// Don't intercept paste in input fields
		const target = e.target as HTMLElement;
		if (
			target.tagName === 'INPUT' ||
			target.tagName === 'TEXTAREA' ||
			target.getAttribute('contenteditable') === 'true'
		) {
			// Still check for image paste in contenteditable
			if (target.getAttribute('contenteditable') !== 'true') {
				return;
			}
		}

		const clipboardData = e.clipboardData;
		if (!clipboardData) return;

		// Check for pasted images
		const imageFiles = Array.from(clipboardData.files).filter((file) =>
			file.type.startsWith('image/')
		);

		if (imageFiles.length > 0) {
			e.preventDefault();
			await processFiles(imageFiles, focusedBlockIndex);
			return;
		}

		// Check for pasted URLs
		const text = clipboardData.getData('text/plain');
		if (text) {
			const embeds = scanTextForEmbeds(text);
			if (embeds.length > 0) {
				e.preventDefault();
				// Insert the first detected embed
				onUrlPaste(embeds[0], focusedBlockIndex);

				// If multiple embeds, insert the rest sequentially
				for (let i = 1; i < embeds.length; i++) {
					onUrlPaste(embeds[i], focusedBlockIndex + i);
				}
			}
		}
	};

	// ==========================================================================
	// Drop Indicator Positioning
	// ==========================================================================

	function updateDropIndicator(e: DragEvent) {
		if (!containerRef) return;

		const containerRect = containerRef.getBoundingClientRect();
		const mouseY = e.clientY;

		// Find the closest block boundary
		let closestIndex = 0;
		let closestDistance = Infinity;
		let indicatorTop = containerRect.top;

		if (blockElements.length === 0) {
			// No blocks yet - show indicator at top
			dropIndicator = {
				top: 0,
				visible: true,
				insertIndex: 0
			};
			return;
		}

		// Check position relative to each block
		for (let i = 0; i <= blockElements.length; i++) {
			let boundaryY: number;

			if (i === 0) {
				// Top of first block
				const firstBlock = blockElements[0];
				if (firstBlock) {
					boundaryY = firstBlock.getBoundingClientRect().top;
				} else {
					boundaryY = containerRect.top;
				}
			} else if (i === blockElements.length) {
				// Bottom of last block
				const lastBlock = blockElements[blockElements.length - 1];
				if (lastBlock) {
					boundaryY = lastBlock.getBoundingClientRect().bottom;
				} else {
					boundaryY = containerRect.bottom;
				}
			} else {
				// Between blocks - use midpoint
				const prevBlock = blockElements[i - 1];
				const nextBlock = blockElements[i];
				if (prevBlock && nextBlock) {
					const prevBottom = prevBlock.getBoundingClientRect().bottom;
					const nextTop = nextBlock.getBoundingClientRect().top;
					boundaryY = (prevBottom + nextTop) / 2;
				} else {
					continue;
				}
			}

			const distance = Math.abs(mouseY - boundaryY);
			if (distance < closestDistance) {
				closestDistance = distance;
				closestIndex = i;
				indicatorTop = boundaryY - containerRect.top;
			}
		}

		dropIndicator = {
			top: indicatorTop,
			visible: true,
			insertIndex: closestIndex
		};
	}

	// ==========================================================================
	// File Processing
	// ==========================================================================

	function hasFiles(e: DragEvent): boolean {
		if (!e.dataTransfer) return false;

		// Check if any of the items are files
		if (e.dataTransfer.types) {
			return e.dataTransfer.types.includes('Files');
		}

		return e.dataTransfer.files?.length > 0;
	}

	function getFilesFromEvent(e: DragEvent): File[] {
		const items = e.dataTransfer?.items;
		const files: File[] = [];

		if (items) {
			for (let i = 0; i < items.length; i++) {
				const item = items[i];
				if (item.kind === 'file') {
					const file = item.getAsFile();
					if (file) files.push(file);
				}
			}
		} else if (e.dataTransfer?.files) {
			files.push(...Array.from(e.dataTransfer.files));
		}

		return files;
	}

	function validateFile(file: File): string | null {
		// Check file size
		if (file.size > maxFileSize) {
			const maxMB = Math.round(maxFileSize / (1024 * 1024));
			return `File "${file.name}" is too large. Maximum size is ${maxMB}MB.`;
		}

		// Check file type
		const isValidType = acceptedTypes.some((type) => {
			if (type.startsWith('.')) {
				return file.name.toLowerCase().endsWith(type.toLowerCase());
			}
			if (type.endsWith('/*')) {
				const baseType = type.replace('/*', '');
				return file.type.startsWith(baseType);
			}
			return file.type === type;
		});

		if (!isValidType) {
			return `File type "${file.type || 'unknown'}" is not supported.`;
		}

		return null;
	}

	async function processFiles(files: File[], insertIndex: number) {
		errorMessage = null;

		// Validate file count
		if (files.length > maxFiles) {
			errorMessage = `Too many files. Maximum is ${maxFiles} files at once.`;
			return;
		}

		// Validate each file
		const validFiles: File[] = [];
		const errors: string[] = [];

		for (const file of files) {
			const error = validateFile(file);
			if (error) {
				errors.push(error);
			} else {
				validFiles.push(file);
			}
		}

		if (errors.length > 0) {
			errorMessage = errors.join('\n');
		}

		if (validFiles.length === 0) return;

		// Create upload states
		const newUploads: UploadState[] = validFiles.map((file) => ({
			id: `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
			file,
			progress: 0,
			status: 'pending' as const,
			abortController: new AbortController()
		}));

		uploads = [...uploads, ...newUploads];

		// Start uploads
		for (const upload of newUploads) {
			upload.status = 'uploading';
			simulateUploadProgress(upload);
		}

		// Call the file drop handler
		try {
			await onFileDrop(validFiles, insertIndex);

			// Mark uploads as complete
			uploads = uploads.map((u) =>
				newUploads.find((nu) => nu.id === u.id)
					? { ...u, status: 'complete' as const, progress: 100 }
					: u
			);
		} catch (error) {
			// Mark uploads as errored
			uploads = uploads.map((u) =>
				newUploads.find((nu) => nu.id === u.id)
					? { ...u, status: 'error' as const, error: (error as Error).message }
					: u
			);
		}

		// Clean up completed uploads after delay
		setTimeout(() => {
			uploads = uploads.filter(
				(u) => u.status !== 'complete' && !newUploads.find((nu) => nu.id === u.id)
			);
		}, 2000);
	}

	function simulateUploadProgress(upload: UploadState) {
		let progress = 0;
		const interval = setInterval(() => {
			if (upload.status !== 'uploading') {
				clearInterval(interval);
				return;
			}

			progress += Math.random() * 15;
			progress = Math.min(progress, 95); // Cap at 95% until actual completion

			const uploadIndex = uploads.findIndex((u) => u.id === upload.id);
			if (uploadIndex !== -1) {
				uploads[uploadIndex].progress = progress;
				onUploadProgress(totalProgress());
			}
		}, 200);
	}

	function cancelUpload(uploadId: string) {
		const upload = uploads.find((u) => u.id === uploadId);
		if (upload) {
			upload.abortController?.abort();
			uploads = uploads.filter((u) => u.id !== uploadId);
		}
	}

	function dismissError() {
		errorMessage = null;
	}

	// ==========================================================================
	// Utility Functions
	// ==========================================================================

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	}

	function getFileIcon(file: File): string {
		if (file.type.startsWith('image/')) return 'photo';
		if (file.type.startsWith('video/')) return 'video';
		if (file.type.startsWith('audio/')) return 'volume';
		if (file.type.includes('pdf')) return 'file-type-pdf';
		if (file.type.includes('word') || file.name.endsWith('.doc')) return 'file-type-doc';
		return 'file';
	}
</script>

<div
	bind:this={containerRef}
	class="media-drop-zone {className}"
	class:is-dragging={isDragging}
	class:is-dragging-over={isDraggingOver}
	class:has-uploads={hasActiveUploads}
	ondragenter={handleDragEnter}
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
	role="region"
	aria-label="Media drop zone"
>
	<!-- Content Slot -->
	<div class="drop-zone-content">
		{@render children()}
	</div>

	<!-- Drop Indicator Line -->
	{#if dropIndicator.visible && isDraggingOver}
		<div
			class="drop-indicator"
			style:top="{dropIndicator.top}px"
			transition:fade={{ duration: 150 }}
		>
			<div class="drop-indicator-line"></div>
			<div class="drop-indicator-label">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M12 5v14" />
					<path d="m19 12-7 7-7-7" />
				</svg>
				<span>Drop here to insert</span>
			</div>
		</div>
	{/if}

	<!-- Full-Screen Drop Overlay -->
	{#if showDropZone && isDragging}
		<div class="drop-overlay" transition:fade={{ duration: 200 }}>
			<div
				class="drop-overlay-content"
				transition:scale={{ duration: 200, start: 0.95, easing: cubicOut }}
			>
				<div class="drop-icon">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="48"
						height="48"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
						<path d="M12 12v9" />
						<path d="m8 17 4 4 4-4" />
					</svg>
				</div>
				<h3>Drop files to upload</h3>
				<p>Images, videos, audio, and documents</p>
				<div class="accepted-types">
					{#each ['Images', 'Videos', 'Audio', 'PDFs'] as type}
						<span class="type-badge">{type}</span>
					{/each}
				</div>
			</div>
		</div>
	{/if}

	<!-- Upload Progress Panel -->
	{#if uploads.length > 0}
		<div class="upload-panel" transition:fly={{ y: 20, duration: 200 }}>
			<div class="upload-panel-header">
				<h4>
					{#if hasActiveUploads}
						Uploading {uploads.filter((u) => u.status === 'uploading').length} file(s)...
					{:else}
						Upload complete
					{/if}
				</h4>
				{#if hasActiveUploads}
					<div class="overall-progress">
						<div class="progress-bar">
							<div class="progress-fill" style:width="{totalProgress()}%"></div>
						</div>
						<span class="progress-text">{Math.round(totalProgress())}%</span>
					</div>
				{/if}
			</div>

			<div class="upload-list">
				{#each uploads as upload (upload.id)}
					<div
						class="upload-item"
						class:uploading={upload.status === 'uploading'}
						class:complete={upload.status === 'complete'}
						class:error={upload.status === 'error'}
						transition:fly={{ x: -20, duration: 200 }}
					>
						<div class="upload-icon">
							{#if upload.status === 'uploading'}
								<div class="spinner"></div>
							{:else if upload.status === 'complete'}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M20 6 9 17l-5-5" />
								</svg>
							{:else if upload.status === 'error'}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<circle cx="12" cy="12" r="10" />
									<line x1="15" y1="9" x2="9" y2="15" />
									<line x1="9" y1="9" x2="15" y2="15" />
								</svg>
							{/if}
						</div>

						<div class="upload-info">
							<span class="upload-name">{upload.file.name}</span>
							<span class="upload-size">{formatFileSize(upload.file.size)}</span>
							{#if upload.status === 'uploading'}
								<div class="upload-progress-bar">
									<div class="upload-progress-fill" style:width="{upload.progress}%"></div>
								</div>
							{/if}
							{#if upload.error}
								<span class="upload-error">{upload.error}</span>
							{/if}
						</div>

						{#if upload.status === 'uploading'}
							<button
								type="button"
								class="upload-cancel"
								onclick={() => cancelUpload(upload.id)}
								title="Cancel upload"
							>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<line x1="18" y1="6" x2="6" y2="18" />
									<line x1="6" y1="6" x2="18" y2="18" />
								</svg>
							</button>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Error Message -->
	{#if errorMessage}
		<div class="error-toast" transition:fly={{ y: 20, duration: 200 }}>
			<div class="error-icon">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
					<line x1="12" y1="9" x2="12" y2="13" />
					<line x1="12" y1="17" x2="12.01" y2="17" />
				</svg>
			</div>
			<p class="error-message">{errorMessage}</p>
			<button type="button" class="error-dismiss" onclick={dismissError}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<line x1="18" y1="6" x2="6" y2="18" />
					<line x1="6" y1="6" x2="18" y2="18" />
				</svg>
			</button>
		</div>
	{/if}
</div>

<style>
	/* Container */
	.media-drop-zone {
		position: relative;
		min-height: 200px;
		transition: all 0.2s ease;
	}

	.media-drop-zone.is-dragging {
		/* Subtle indication that dragging is happening */
	}

	.media-drop-zone.is-dragging-over {
		/* Border highlight when dragging over */
	}

	/* Content */
	.drop-zone-content {
		position: relative;
		z-index: 1;
	}

	/* Drop Indicator */
	.drop-indicator {
		position: absolute;
		left: 0;
		right: 0;
		z-index: 100;
		pointer-events: none;
		transform: translateY(-50%);
	}

	.drop-indicator-line {
		height: 3px;
		background: linear-gradient(90deg, transparent, #3b82f6, transparent);
		border-radius: 2px;
		box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
	}

	.drop-indicator-label {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 0.25rem;
		padding: 0.375rem 0.75rem;
		background: #3b82f6;
		color: white;
		font-size: 0.75rem;
		font-weight: 500;
		border-radius: 4px;
		width: fit-content;
		margin-left: auto;
		margin-right: auto;
		box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
	}

	.drop-indicator-label svg {
		width: 14px;
		height: 14px;
	}

	/* Drop Overlay */
	.drop-overlay {
		position: absolute;
		inset: 0;
		background: rgba(59, 130, 246, 0.08);
		backdrop-filter: blur(2px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		border: 3px dashed #3b82f6;
		border-radius: 12px;
	}

	.drop-overlay-content {
		text-align: center;
		padding: 2rem;
	}

	.drop-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		margin: 0 auto 1rem;
		background: #dbeafe;
		border-radius: 20px;
		color: #3b82f6;
	}

	.drop-overlay-content h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.drop-overlay-content p {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem;
	}

	.accepted-types {
		display: flex;
		gap: 0.5rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.type-badge {
		padding: 0.25rem 0.625rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		font-size: 0.75rem;
		color: #6b7280;
	}

	/* Upload Panel */
	.upload-panel {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		width: 320px;
		background: white;
		border-radius: 12px;
		box-shadow:
			0 10px 40px rgba(0, 0, 0, 0.15),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		z-index: 1000;
		overflow: hidden;
	}

	.upload-panel-header {
		padding: 1rem;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.upload-panel-header h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.overall-progress {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.progress-bar {
		flex: 1;
		height: 6px;
		background: #e5e7eb;
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: #3b82f6;
		border-radius: 3px;
		transition: width 0.2s ease;
	}

	.progress-text {
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		min-width: 2.5rem;
		text-align: right;
	}

	/* Upload List */
	.upload-list {
		max-height: 240px;
		overflow-y: auto;
	}

	.upload-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		border-bottom: 1px solid #f3f4f6;
	}

	.upload-item:last-child {
		border-bottom: none;
	}

	.upload-item.complete .upload-icon {
		color: #10b981;
	}

	.upload-item.error .upload-icon {
		color: #ef4444;
	}

	.upload-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #f3f4f6;
		border-radius: 8px;
		flex-shrink: 0;
		color: #6b7280;
	}

	.upload-info {
		flex: 1;
		min-width: 0;
	}

	.upload-name {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #1a1a1a;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.upload-size {
		display: block;
		font-size: 0.75rem;
		color: #9ca3af;
		margin-top: 0.125rem;
	}

	.upload-progress-bar {
		height: 4px;
		background: #e5e7eb;
		border-radius: 2px;
		margin-top: 0.5rem;
		overflow: hidden;
	}

	.upload-progress-fill {
		height: 100%;
		background: #3b82f6;
		border-radius: 2px;
		transition: width 0.15s ease;
	}

	.upload-error {
		display: block;
		font-size: 0.75rem;
		color: #ef4444;
		margin-top: 0.25rem;
	}

	.upload-cancel {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		color: #9ca3af;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.upload-cancel:hover {
		background: #fee2e2;
		color: #ef4444;
	}

	/* Spinner */
	.spinner {
		width: 18px;
		height: 18px;
		border: 2px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Error Toast */
	.error-toast {
		position: fixed;
		bottom: 1.5rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: white;
		border: 1px solid #fecaca;
		border-radius: 10px;
		box-shadow:
			0 10px 25px rgba(239, 68, 68, 0.1),
			0 0 0 1px rgba(239, 68, 68, 0.05);
		z-index: 1001;
		max-width: 400px;
	}

	.error-icon {
		color: #ef4444;
		flex-shrink: 0;
	}

	.error-message {
		flex: 1;
		font-size: 0.875rem;
		color: #1a1a1a;
		margin: 0;
		white-space: pre-line;
	}

	.error-dismiss {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #9ca3af;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.error-dismiss:hover {
		background: #f3f4f6;
		color: #6b7280;
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.drop-overlay {
			background: rgba(59, 130, 246, 0.12);
		}

		.drop-icon {
			background: rgba(59, 130, 246, 0.2);
		}

		.drop-overlay-content h3 {
			color: #f9fafb;
		}

		.drop-overlay-content p {
			color: #9ca3af;
		}

		.type-badge {
			background: #374151;
			border-color: #4b5563;
			color: #d1d5db;
		}

		.upload-panel {
			background: #1f2937;
		}

		.upload-panel-header {
			background: #111827;
			border-color: #374151;
		}

		.upload-panel-header h4 {
			color: #f9fafb;
		}

		.upload-item {
			border-color: #374151;
		}

		.upload-icon {
			background: #374151;
		}

		.upload-name {
			color: #f9fafb;
		}

		.progress-bar,
		.upload-progress-bar {
			background: #374151;
		}

		.error-toast {
			background: #1f2937;
			border-color: #7f1d1d;
		}

		.error-message {
			color: #f9fafb;
		}
	}

	/* Responsive */
	@media (max-width: 640px) {
		.upload-panel {
			left: 1rem;
			right: 1rem;
			width: auto;
			bottom: 1rem;
		}

		.error-toast {
			left: 1rem;
			right: 1rem;
			transform: none;
			max-width: none;
		}
	}
</style>
