<!--
/**
 * ImageUploader - Drag & Drop Image Upload Component
 * =============================================================================
 * Enterprise-grade image uploader with R2 CDN integration, image processing,
 * blurhash placeholders, progress tracking, and Asset Manager integration
 *
 * Features:
 * - Drag and drop zone
 * - Click to select files
 * - Paste from clipboard
 * - Image preview before upload
 * - Blurhash placeholder during upload
 * - Progress bar with percentage
 * - Error handling with retry
 * - Multiple file support
 * - Choose from Asset Manager library
 * - Recent assets quick access
 *
 * @version 2.0.0 - February 2026
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { cubicOut, elasticOut } from 'svelte/easing';
	import {
		uploadImage,
		UploadController,
		formatFileSize,
		isValidImageType,
		type UploadResult
	} from './upload/uploader';
	import {
		processImage,
		canProcessImage,
		type ProcessedImage,
		type ProcessOptions
	} from './upload/image-processor';
	import AssetManager from './AssetManager.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		/** Callback when upload completes successfully */
		onUpload: (result: UploadResult) => void;
		/** Accepted file types (default: image/*) */
		accept?: string;
		/** Maximum file size in bytes (default: 10MB) */
		maxSize?: number;
		/** Allow multiple file selection */
		multiple?: boolean;
		/** Collection/folder for uploaded images */
		collection?: string;
		/** Show compact mode */
		compact?: boolean;
		/** Process images before upload */
		processBeforeUpload?: boolean;
		/** Image processing options */
		processOptions?: ProcessOptions;
		/** Show "Choose from library" button */
		showLibrary?: boolean;
		/** Show recent assets quick access */
		showRecent?: boolean;
	}

	let props: Props = $props();
	const onUpload = $derived(props.onUpload);
	const accept = $derived(props.accept ?? 'image/*');
	const maxSize = $derived(props.maxSize ?? 10 * 1024 * 1024); // 10MB
	const multiple = $derived(props.multiple ?? false);
	const collection = $derived(props.collection ?? 'blog-images');
	const compact = $derived(props.compact ?? false);
	const processBeforeUpload = $derived(props.processBeforeUpload ?? true);
	const processOptions = $derived(props.processOptions ?? {});
	const showLibrary = $derived(props.showLibrary ?? true);
	const showRecent = $derived(props.showRecent ?? true);

	// ==========================================================================
	// State
	// ==========================================================================

	interface UploadItem {
		id: string;
		file: File;
		status: 'pending' | 'processing' | 'uploading' | 'complete' | 'error';
		progress: number;
		error?: string;
		previewUrl?: string;
		blurhash?: string;
		processedImage?: ProcessedImage;
		result?: UploadResult;
		controller?: UploadController;
	}

	let isDragging = $state(false);
	let dragCounter = $state(0);
	let uploadQueue = $state<UploadItem[]>([]);
	let fileInput = $state<HTMLInputElement | null>(null);
	// Bound via bind:this in template for future DOM operations
	let dropZone = $state<HTMLDivElement | null>(null);

	// Asset Manager state
	let assetManagerOpen = $state(false);
	let recentAssets = $state<RecentAsset[]>([]);
	let showRecentPanel = $state(false);
	// Tracks loading state for async operation (for future loading indicator)
	let _isLoadingRecent = $state(false);

	interface RecentAsset {
		id: string;
		filename: string;
		cdn_url: string;
		thumbnail_url: string | null;
		mime_type: string;
		width: number | null;
		height: number | null;
		blurhash: string | null;
		created_at: string | null;
	}

	// Derived state for progress tracking
	const _isUploading = $derived(
		uploadQueue.some((item) => item.status === 'processing' || item.status === 'uploading')
	);
	const _hasErrors = $derived(uploadQueue.some((item) => item.status === 'error'));
	const _completedCount = $derived(uploadQueue.filter((item) => item.status === 'complete').length);
	const _totalProgress = $derived.by(() => {
		if (uploadQueue.length === 0) return 0;
		const sum = uploadQueue.reduce((acc, item) => acc + item.progress, 0);
		return Math.round(sum / uploadQueue.length);
	});

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		// Add paste event listener
		document.addEventListener('paste', handlePaste);

		// Load recent assets if enabled
		if (showRecent) {
			loadRecentAssets();
		}
	});

	onDestroy(() => {
		// Clean up
		document.removeEventListener('paste', handlePaste);

		// Cancel any pending uploads
		uploadQueue.forEach((item) => {
			item.controller?.abort();
			if (item.previewUrl) {
				URL.revokeObjectURL(item.previewUrl);
			}
		});
	});

	// ==========================================================================
	// Asset Manager Integration
	// ==========================================================================

	async function loadRecentAssets() {
		if (!showRecent) return;

		_isLoadingRecent = true;
		try {
			const response = await fetch('/api/cms/assets/recent?limit=8', {
				credentials: 'include'
			});
			if (response.ok) {
				const data = await response.json();
				recentAssets = data.filter((a: RecentAsset) => a.mime_type.startsWith('image/'));
			}
		} catch (error) {
			logger.error('Failed to load recent assets:', error);
		} finally {
			_isLoadingRecent = false;
		}
	}

	function openAssetManager() {
		assetManagerOpen = true;
	}

	function handleAssetSelect(asset: any) {
		// Convert asset to UploadResult format
		const result: UploadResult = {
			id: asset.id,
			url: asset.cdn_url,
			filename: asset.filename,
			mimeType: asset.mime_type || 'image/jpeg',
			size: asset.file_size,
			width: asset.width,
			height: asset.height,
			blurhash: asset.blurhash,
			createdAt: asset.created_at || new Date().toISOString()
		};

		onUpload(result);

		// Track usage
		trackAssetUsage(asset.id);
	}

	function handleRecentAssetClick(asset: RecentAsset) {
		const result: UploadResult = {
			id: parseInt(asset.id) || 0,
			url: asset.cdn_url,
			filename: asset.filename,
			mimeType: asset.mime_type || 'image/jpeg',
			size: 0,
			width: asset.width || 0,
			height: asset.height || 0,
			blurhash: asset.blurhash || undefined,
			createdAt: asset.created_at || new Date().toISOString()
		};

		onUpload(result);
		trackAssetUsage(asset.id);
		showRecentPanel = false;
	}

	async function trackAssetUsage(assetId: string) {
		try {
			await fetch(`/api/cms/assets/${assetId}/track-usage`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					content_type: 'blog_post',
					field_name: 'image_block'
				})
			});
		} catch (_error) {
			// Silent fail - usage tracking is not critical
		}
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleDragEnter(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		dragCounter++;

		if (e.dataTransfer?.types.includes('Files')) {
			isDragging = true;
		}
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		dragCounter--;

		if (dragCounter === 0) {
			isDragging = false;
		}
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragging = false;
		dragCounter = 0;

		const files = Array.from(e.dataTransfer?.files || []);
		processFiles(files);
	}

	function handleFileInput(e: Event) {
		const input = e.target as HTMLInputElement;
		const files = Array.from(input.files || []);
		processFiles(files);
		input.value = ''; // Reset for re-selection
	}

	function handlePaste(e: ClipboardEvent) {
		const items = e.clipboardData?.items;
		if (!items) return;

		const imageFiles: File[] = [];

		for (const item of items) {
			if (item.type.startsWith('image/')) {
				const file = item.getAsFile();
				if (file) {
					imageFiles.push(file);
				}
			}
		}

		if (imageFiles.length > 0) {
			e.preventDefault();
			processFiles(imageFiles);
		}
	}

	function handleClick() {
		fileInput?.click();
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleClick();
		}
	}

	// ==========================================================================
	// File Processing
	// ==========================================================================

	function processFiles(files: File[]) {
		const validFiles = files.filter((file) => {
			// Check type
			if (!isValidImageType(file)) {
				logger.warn(`Invalid file type: ${file.type}`);
				return false;
			}

			// Check size
			if (file.size > maxSize) {
				logger.warn(`File too large: ${formatFileSize(file.size)}`);
				return false;
			}

			return true;
		});

		// Limit to first file if not multiple
		const filesToProcess = multiple ? validFiles : validFiles.slice(0, 1);

		// Create upload items
		const newItems: UploadItem[] = filesToProcess.map((file) => ({
			id: crypto.randomUUID(),
			file,
			status: 'pending' as const,
			progress: 0,
			previewUrl: URL.createObjectURL(file),
			controller: new UploadController()
		}));

		if (multiple) {
			uploadQueue = [...uploadQueue, ...newItems];
		} else {
			// Clear previous uploads in single mode
			uploadQueue.forEach((item) => {
				if (item.previewUrl) URL.revokeObjectURL(item.previewUrl);
				item.controller?.abort();
			});
			uploadQueue = newItems;
		}

		// Start processing/uploading
		newItems.forEach((item) => startUpload(item.id));
	}

	async function startUpload(itemId: string) {
		const idx = uploadQueue.findIndex((item) => item.id === itemId);
		if (idx === -1) return;

		const item = uploadQueue[idx];

		try {
			// Step 1: Process image (if enabled)
			if (processBeforeUpload && canProcessImage(item.file)) {
				uploadQueue[idx].status = 'processing';
				uploadQueue = uploadQueue;

				const processed = await processImage(item.file, {
					maxWidth: 2000,
					quality: 0.8,
					convertToWebP: true,
					generateBlurhash: true,
					...processOptions
				});

				uploadQueue[idx].processedImage = processed;
				uploadQueue[idx].blurhash = processed.blurhash;

				// Update preview to processed image
				if (item.previewUrl) {
					URL.revokeObjectURL(item.previewUrl);
				}
				uploadQueue[idx].previewUrl = processed.dataUrl;
				uploadQueue = uploadQueue;
			}

			// Step 2: Upload
			uploadQueue[idx].status = 'uploading';
			uploadQueue = uploadQueue;

			const fileToUpload = uploadQueue[idx].processedImage?.file || item.file;

			const result = await uploadImage(fileToUpload, {
				collection,
				onProgress: (progress) => {
					const currentIdx = uploadQueue.findIndex((i) => i.id === itemId);
					if (currentIdx !== -1) {
						uploadQueue[currentIdx].progress = progress;
						uploadQueue = uploadQueue;
					}
				},
				signal: item.controller?.signal
			});

			// Add blurhash to result if we generated one
			if (uploadQueue[idx].blurhash && !result.blurhash) {
				result.blurhash = uploadQueue[idx].blurhash;
			}

			uploadQueue[idx].status = 'complete';
			uploadQueue[idx].progress = 100;
			uploadQueue[idx].result = result;
			uploadQueue = uploadQueue;

			// Notify parent
			onUpload(result);
		} catch (error) {
			const currentIdx = uploadQueue.findIndex((i) => i.id === itemId);
			if (currentIdx !== -1) {
				uploadQueue[currentIdx].status = 'error';
				uploadQueue[currentIdx].error = error instanceof Error ? error.message : 'Upload failed';
				uploadQueue = uploadQueue;
			}
		}
	}

	function retryUpload(itemId: string) {
		const idx = uploadQueue.findIndex((item) => item.id === itemId);
		if (idx === -1) return;

		// Reset state
		uploadQueue[idx].status = 'pending';
		uploadQueue[idx].progress = 0;
		uploadQueue[idx].error = undefined;
		uploadQueue[idx].controller = new UploadController();
		uploadQueue = uploadQueue;

		startUpload(itemId);
	}

	function cancelUpload(itemId: string) {
		const idx = uploadQueue.findIndex((item) => item.id === itemId);
		if (idx === -1) return;

		// Abort in-progress upload
		uploadQueue[idx].controller?.abort();

		// Clean up
		if (uploadQueue[idx].previewUrl) {
			URL.revokeObjectURL(uploadQueue[idx].previewUrl!);
		}

		// Remove from queue
		uploadQueue = uploadQueue.filter((item) => item.id !== itemId);
	}

	function removeItem(itemId: string) {
		const idx = uploadQueue.findIndex((item) => item.id === itemId);
		if (idx === -1) return;

		if (uploadQueue[idx].previewUrl) {
			URL.revokeObjectURL(uploadQueue[idx].previewUrl!);
		}

		uploadQueue = uploadQueue.filter((item) => item.id !== itemId);
	}

	// ==========================================================================
	// Blurhash Rendering
	// ==========================================================================

	function renderBlurhashToDataUrl(hash: string, width: number = 32, height: number = 32): string {
		// Simplified blurhash decoder - returns a gradient approximation
		// In production, use the blurhash library for proper decoding
		const canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;
		const ctx = canvas.getContext('2d');

		if (ctx && hash.length >= 5) {
			// Decode base color from hash (simplified)
			const base83Chars =
				'0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~';

			const idx1 = base83Chars.indexOf(hash[1]);
			const idx2 = base83Chars.indexOf(hash[2]);
			const idx3 = base83Chars.indexOf(hash[3]);
			const idx4 = base83Chars.indexOf(hash[4]);

			const dcValue = (idx1 << 18) + (idx2 << 12) + (idx3 << 6) + idx4;
			const r = (dcValue >> 16) & 255;
			const g = (dcValue >> 8) & 255;
			const b = dcValue & 255;

			// Create gradient
			const gradient = ctx.createRadialGradient(
				width / 2,
				height / 2,
				0,
				width / 2,
				height / 2,
				Math.max(width, height)
			);
			gradient.addColorStop(0, `rgb(${r}, ${g}, ${b})`);
			gradient.addColorStop(
				1,
				`rgb(${Math.max(0, r - 30)}, ${Math.max(0, g - 30)}, ${Math.max(0, b - 30)})`
			);

			ctx.fillStyle = gradient;
			ctx.fillRect(0, 0, width, height);
		}

		return canvas.toDataURL();
	}
</script>

<div class="image-uploader" class:compact class:dragging={isDragging}>
	<!-- Library & Recent Actions -->
	{#if (showLibrary || showRecent) && !compact}
		<div class="library-actions" transition:fade={{ duration: 150 }}>
			{#if showLibrary}
				<button type="button" class="library-btn" onclick={openAssetManager}>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="3" y="3" width="7" height="7" />
						<rect x="14" y="3" width="7" height="7" />
						<rect x="14" y="14" width="7" height="7" />
						<rect x="3" y="14" width="7" height="7" />
					</svg>
					Choose from library
				</button>
			{/if}
			{#if showRecent && recentAssets.length > 0}
				<button
					type="button"
					class="recent-btn"
					class:active={showRecentPanel}
					onclick={() => (showRecentPanel = !showRecentPanel)}
				>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<polyline points="12 6 12 12 16 14" />
					</svg>
					Recent
					<svg
						class="chevron"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						style="transform: rotate({showRecentPanel ? 180 : 0}deg)"
					>
						<polyline points="6 9 12 15 18 9" />
					</svg>
				</button>
			{/if}
		</div>
	{/if}

	<!-- Recent Assets Panel -->
	{#if showRecentPanel && recentAssets.length > 0}
		<div class="recent-panel" transition:slide={{ duration: 200 }}>
			<div class="recent-grid">
				{#each recentAssets as asset (asset.id)}
					<button
						type="button"
						class="recent-asset"
						onclick={() => handleRecentAssetClick(asset)}
						title={asset.filename}
					>
						<img src={asset.thumbnail_url || asset.cdn_url} alt={asset.filename} loading="lazy" />
					</button>
				{/each}
			</div>
			<button type="button" class="view-all-btn" onclick={openAssetManager}>
				View all assets
			</button>
		</div>
	{/if}

	<!-- Drop Zone -->
	<div
		bind:this={dropZone}
		class="drop-zone"
		class:active={isDragging}
		class:has-items={uploadQueue.length > 0}
		role="button"
		tabindex="0"
		ondragenter={handleDragEnter}
		ondragleave={handleDragLeave}
		ondragover={handleDragOver}
		ondrop={handleDrop}
		onclick={handleClick}
		onkeydown={handleKeyDown}
		aria-label="Drop image here or click to select"
	>
		<input
			bind:this={fileInput}
			type="file"
			{accept}
			{multiple}
			onchange={handleFileInput}
			class="file-input"
		/>

		{#if isDragging}
			<div class="drop-indicator" in:scale={{ duration: 200, easing: elasticOut }}>
				<svg
					class="drop-icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
					<polyline points="17 8 12 3 7 8" />
					<line x1="12" y1="3" x2="12" y2="15" />
				</svg>
				<span class="drop-text">Drop image here</span>
			</div>
		{:else if uploadQueue.length === 0}
			<div class="empty-state">
				<svg
					class="upload-icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
				>
					<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
					<circle cx="8.5" cy="8.5" r="1.5" />
					<polyline points="21 15 16 10 5 21" />
				</svg>
				<div class="empty-text">
					<span class="primary-text">
						{compact ? 'Add image' : 'Drag & drop image here'}
					</span>
					{#if !compact}
						<span class="secondary-text">or click to browse</span>
						<span class="hint-text">Paste from clipboard also works</span>
					{/if}
				</div>
				{#if !compact}
					<div class="file-hints">
						<span class="hint">JPG, PNG, WebP, GIF</span>
						<span class="hint">Max {formatFileSize(maxSize)}</span>
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Upload Queue -->
	{#if uploadQueue.length > 0}
		<div class="upload-queue" transition:fade={{ duration: 200 }}>
			{#each uploadQueue as item (item.id)}
				<div
					class="queue-item"
					class:processing={item.status === 'processing'}
					class:uploading={item.status === 'uploading'}
					class:complete={item.status === 'complete'}
					class:error={item.status === 'error'}
					transition:fly={{ y: 20, duration: 300, easing: cubicOut }}
				>
					<!-- Preview -->
					<div class="item-preview">
						{#if item.status === 'uploading' && item.blurhash}
							<!-- Show blurhash placeholder during upload -->
							<img
								src={renderBlurhashToDataUrl(item.blurhash, 64, 64)}
								alt="Loading placeholder"
								class="blurhash-preview"
							/>
							<div class="preview-overlay">
								<div class="spinner"></div>
							</div>
						{:else if item.previewUrl}
							<img src={item.previewUrl} alt={item.file.name} />
						{:else}
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
								<circle cx="8.5" cy="8.5" r="1.5" />
								<polyline points="21 15 16 10 5 21" />
							</svg>
						{/if}
					</div>

					<!-- Info -->
					<div class="item-info">
						<div class="item-name" title={item.file.name}>
							{item.file.name}
						</div>
						<div class="item-meta">
							<span class="item-size"
								>{formatFileSize(item.processedImage?.size || item.file.size)}</span
							>
							{#if item.processedImage && item.processedImage.compressionRatio < 0.9}
								<span class="compression-badge">
									-{Math.round((1 - item.processedImage.compressionRatio) * 100)}%
								</span>
							{/if}
						</div>

						<!-- Progress Bar -->
						{#if item.status === 'processing' || item.status === 'uploading'}
							<div class="progress-container">
								<div class="progress-bar">
									<div
										class="progress-fill"
										class:indeterminate={item.status === 'processing'}
										style="width: {item.status === 'processing' ? '100%' : `${item.progress}%`}"
									></div>
								</div>
								<span class="progress-text">
									{#if item.status === 'processing'}
										Processing...
									{:else}
										{item.progress}%
									{/if}
								</span>
							</div>
						{/if}

						<!-- Error Message -->
						{#if item.error}
							<div class="error-message">{item.error}</div>
						{/if}
					</div>

					<!-- Status Icon -->
					<div class="item-status">
						{#if item.status === 'complete'}
							<svg
								class="status-icon success"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
								<polyline points="22 4 12 14.01 9 11.01" />
							</svg>
						{:else if item.status === 'error'}
							<button
								type="button"
								class="retry-btn"
								onclick={() => retryUpload(item.id)}
								title="Retry upload"
							>
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<polyline points="23 4 23 10 17 10" />
									<path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
								</svg>
							</button>
						{:else if item.status === 'processing' || item.status === 'uploading'}
							<div class="spinner small"></div>
						{/if}
					</div>

					<!-- Remove Button -->
					<button
						type="button"
						class="remove-btn"
						onclick={() =>
							item.status === 'uploading' ? cancelUpload(item.id) : removeItem(item.id)}
						title={item.status === 'uploading' ? 'Cancel upload' : 'Remove'}
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18" />
							<line x1="6" y1="6" x2="18" y2="18" />
						</svg>
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Asset Manager Modal -->
<AssetManager
	isOpen={assetManagerOpen}
	onClose={() => (assetManagerOpen = false)}
	onSelect={handleAssetSelect}
	acceptTypes={['image']}
/>

<style>
	.image-uploader {
		width: 100%;
	}

	.drop-zone {
		border: 2px dashed rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		padding: 2rem;
		text-align: center;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		background: rgba(255, 255, 255, 0.02);
		min-height: 160px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.drop-zone:hover,
	.drop-zone:focus {
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.05);
		outline: none;
	}

	.drop-zone.active {
		border-color: #3b82f6;
		border-style: solid;
		background: rgba(59, 130, 246, 0.1);
		transform: scale(1.01);
	}

	.drop-zone.has-items {
		min-height: 80px;
		padding: 1rem;
	}

	.compact .drop-zone {
		min-height: 100px;
		padding: 1rem;
	}

	.file-input {
		display: none;
	}

	/* Drop Indicator */
	.drop-indicator {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		color: #3b82f6;
	}

	.drop-icon {
		width: 48px;
		height: 48px;
		animation: bounce 0.5s infinite alternate;
	}

	.drop-text {
		font-size: 1.125rem;
		font-weight: 500;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.upload-icon {
		width: 48px;
		height: 48px;
		color: #64748b;
	}

	.compact .upload-icon {
		width: 32px;
		height: 32px;
	}

	.empty-text {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
	}

	.primary-text {
		font-size: 1rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	.secondary-text {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.hint-text {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	.file-hints {
		display: flex;
		gap: 1rem;
		margin-top: 0.5rem;
	}

	.hint {
		font-size: 0.75rem;
		color: #64748b;
		padding: 0.25rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 999px;
	}

	/* Upload Queue */
	.upload-queue {
		margin-top: 1rem;
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
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.queue-item:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.queue-item.complete {
		border-color: rgba(34, 197, 94, 0.3);
		background: rgba(34, 197, 94, 0.05);
	}

	.queue-item.error {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.05);
	}

	/* Item Preview */
	.item-preview {
		width: 48px;
		height: 48px;
		border-radius: 6px;
		overflow: hidden;
		background: rgba(255, 255, 255, 0.05);
		flex-shrink: 0;
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.item-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.item-preview svg {
		width: 24px;
		height: 24px;
		color: #64748b;
	}

	.blurhash-preview {
		filter: blur(8px);
		transform: scale(1.1);
	}

	.preview-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Item Info */
	.item-info {
		flex: 1;
		min-width: 0;
	}

	.item-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: #e2e8f0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.125rem;
	}

	.item-size {
		font-size: 0.75rem;
		color: #64748b;
	}

	.compression-badge {
		font-size: 0.625rem;
		color: #22c55e;
		background: rgba(34, 197, 94, 0.1);
		padding: 0.125rem 0.375rem;
		border-radius: 999px;
		font-weight: 500;
	}

	/* Progress */
	.progress-container {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.progress-bar {
		flex: 1;
		height: 4px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #06b6d4);
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.progress-fill.indeterminate {
		animation: indeterminate 1.5s infinite;
		width: 30% !important;
	}

	.progress-text {
		font-size: 0.75rem;
		color: #94a3b8;
		min-width: 4rem;
		text-align: right;
	}

	/* Error */
	.error-message {
		font-size: 0.75rem;
		color: #ef4444;
		margin-top: 0.25rem;
	}

	/* Status */
	.item-status {
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.status-icon {
		width: 20px;
		height: 20px;
	}

	.status-icon.success {
		color: #22c55e;
	}

	.retry-btn {
		width: 24px;
		height: 24px;
		border: none;
		background: none;
		color: #f59e0b;
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.retry-btn:hover {
		color: #fbbf24;
		transform: rotate(-30deg);
	}

	.retry-btn svg {
		width: 18px;
		height: 18px;
	}

	/* Remove Button */
	.remove-btn {
		width: 24px;
		height: 24px;
		border: none;
		background: none;
		color: #64748b;
		cursor: pointer;
		padding: 0;
		opacity: 0;
		transition: all 0.2s;
	}

	.queue-item:hover .remove-btn {
		opacity: 1;
	}

	.remove-btn:hover {
		color: #ef4444;
	}

	.remove-btn svg {
		width: 16px;
		height: 16px;
	}

	/* Spinner */
	.spinner {
		width: 24px;
		height: 24px;
		border: 2px solid rgba(59, 130, 246, 0.2);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner.small {
		width: 16px;
		height: 16px;
		border-width: 2px;
	}

	/* Animations */
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes bounce {
		to {
			transform: translateY(-8px);
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

	/* Responsive */
	@media (max-width: 640px) {
		.drop-zone {
			padding: 1.5rem;
			min-height: 120px;
		}

		.file-hints {
			flex-direction: column;
			gap: 0.5rem;
		}
	}

	/* ========================================================================== */
	/* Library & Recent Assets */
	/* ========================================================================== */

	.library-actions {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.library-btn,
	.recent-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.2);
		border-radius: 8px;
		color: #3b82f6;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.library-btn:hover,
	.recent-btn:hover,
	.recent-btn.active {
		background: rgba(59, 130, 246, 0.15);
		border-color: rgba(59, 130, 246, 0.3);
	}

	.library-btn svg,
	.recent-btn svg {
		width: 16px;
		height: 16px;
		flex-shrink: 0;
	}

	.recent-btn .chevron {
		width: 14px;
		height: 14px;
		margin-left: auto;
		transition: transform 0.2s;
	}

	.recent-panel {
		margin-bottom: 0.75rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
	}

	.recent-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
		gap: 0.5rem;
	}

	.recent-asset {
		aspect-ratio: 1;
		border: 2px solid transparent;
		border-radius: 6px;
		overflow: hidden;
		cursor: pointer;
		background: #1e293b;
		padding: 0;
		transition: all 0.2s;
	}

	.recent-asset:hover {
		border-color: #3b82f6;
		transform: scale(1.05);
	}

	.recent-asset img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.view-all-btn {
		width: 100%;
		margin-top: 0.75rem;
		padding: 0.5rem;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.view-all-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #f1f5f9;
	}

	@media (max-width: 640px) {
		.library-actions {
			flex-direction: column;
		}

		.library-btn,
		.recent-btn {
			justify-content: center;
		}

		.recent-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}
</style>
