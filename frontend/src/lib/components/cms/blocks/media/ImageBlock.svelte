<!--
/**
 * Image Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Production-ready responsive image block with lightbox support
 * Apple Principal Engineer ICT 7 Standard
 */
-->

<script lang="ts">
	import {
		IconPhoto,
		IconUpload,
		IconLink,
		IconX,
		IconLoader2,
		IconAlertCircle,
		IconRefresh,
		IconMaximize,
		IconAdjustments
	} from '$lib/icons';
	import { sanitizeURL, sanitizeText } from '$lib/utils/sanitization';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// ==========================================================================
	// State Manager
	// ==========================================================================

	const stateManager = getBlockStateManager();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let isUploading = $state(false);
	let uploadError = $state<string | null>(null);
	let showObjectFitControls = $state(false);
	let urlInputValue = $state('');
	let fileInputRef = $state<HTMLInputElement | null>(null);

	// ==========================================================================
	// Derived Values
	// ==========================================================================

	const imageUrl = $derived(props.block.content.mediaUrl || '');
	const imageAlt = $derived(props.block.content.mediaAlt || '');
	const caption = $derived(props.block.content.mediaCaption || '');
	const objectFit = $derived(
		(props.block.settings?.objectFit as 'cover' | 'contain' | 'fill') || 'cover'
	);
	const hasImage = $derived(!!imageUrl);
	const sanitizedURL = $derived(imageUrl ? sanitizeURL(imageUrl) : '');

	// Generate srcset for responsive images
	const srcset = $derived(() => {
		if (!sanitizedURL) return '';
		// If it's a blob URL or data URL, don't generate srcset
		if (sanitizedURL.startsWith('blob:') || sanitizedURL.startsWith('data:')) {
			return '';
		}
		// Generate responsive srcset widths
		const widths = [400, 800, 1200, 1600];
		return widths
			.map((w) => {
				// If the URL already has query params, append; otherwise add
				const separator = sanitizedURL.includes('?') ? '&' : '?';
				return `${sanitizedURL}${separator}w=${w} ${w}w`;
			})
			.join(', ');
	});

	// Unique IDs for ARIA
	const captionId = $derived(`image-caption-${props.blockId}`);
	const descriptionId = $derived(`image-desc-${props.blockId}`);

	// ==========================================================================
	// Constants
	// ==========================================================================

	const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB
	const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/gif', 'image/webp', 'image/svg+xml'];
	const OBJECT_FIT_OPTIONS: Array<{ value: 'cover' | 'contain' | 'fill'; label: string }> = [
		{ value: 'cover', label: 'Cover' },
		{ value: 'contain', label: 'Contain' },
		{ value: 'fill', label: 'Fill' }
	];

	// ==========================================================================
	// Content Updates
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({
			content: { ...props.block.content, ...updates }
		});
	}

	function updateSettings(updates: Partial<Block['settings']>): void {
		props.onUpdate({
			settings: { ...props.block.settings, ...updates }
		});
	}

	// ==========================================================================
	// File Upload Handlers
	// ==========================================================================

	function handleFileSelect(e: Event): void {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			processFile(file);
		}
		// Reset input so same file can be selected again
		input.value = '';
	}

	function handleDrop(e: DragEvent): void {
		e.preventDefault();
		const file = e.dataTransfer?.files?.[0];
		if (file) {
			processFile(file);
		}
	}

	function handleDragOver(e: DragEvent): void {
		e.preventDefault();
	}

	async function processFile(file: File): Promise<void> {
		uploadError = null;

		// Validate file type
		if (!file.type.startsWith('image/') || !ALLOWED_TYPES.includes(file.type)) {
			const error = 'Invalid file type. Please upload an image (JPEG, PNG, GIF, WebP, or SVG).';
			uploadError = error;
			props.onError?.(new Error(error));
			return;
		}

		// Validate file size
		if (file.size > MAX_FILE_SIZE) {
			const error = 'File too large. Maximum size is 10MB.';
			uploadError = error;
			props.onError?.(new Error(error));
			return;
		}

		isUploading = true;

		try {
			// In production, this would upload to a CDN/storage service
			// For now, create a local object URL for preview
			const objectUrl = URL.createObjectURL(file);

			// Simulate upload delay for realistic UX
			await new Promise((resolve) => setTimeout(resolve, 500));

			updateContent({
				mediaUrl: objectUrl,
				mediaAlt: sanitizeText(file.name.replace(/\.[^/.]+$/, ''))
			});

			uploadError = null;
		} catch (err) {
			const error = err instanceof Error ? err.message : 'Failed to upload image';
			uploadError = error;
			props.onError?.(new Error(error));
		} finally {
			isUploading = false;
		}
	}

	// ==========================================================================
	// URL Input Handlers
	// ==========================================================================

	function handleURLSubmit(): void {
		const trimmedUrl = urlInputValue.trim();
		if (!trimmedUrl) return;

		const sanitized = sanitizeURL(trimmedUrl);
		if (sanitized) {
			updateContent({ mediaUrl: sanitized });
			urlInputValue = '';
			uploadError = null;
		} else {
			uploadError = 'Invalid URL. Please enter a valid image URL.';
		}
	}

	function handleURLKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleURLSubmit();
		}
	}

	// ==========================================================================
	// Edit Mode Handlers
	// ==========================================================================

	function handleAltInput(e: Event): void {
		const target = e.target as HTMLElement;
		const text = sanitizeText(target.textContent || '', 500);
		updateContent({ mediaAlt: text });
	}

	function handleCaptionInput(e: Event): void {
		const target = e.target as HTMLElement;
		const text = sanitizeText(target.textContent || '', 1000);
		updateContent({ mediaCaption: text });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, sanitizeText(text, 1000));
	}

	function handleObjectFitChange(fit: 'cover' | 'contain' | 'fill'): void {
		updateSettings({ objectFit: fit });
		showObjectFitControls = false;
	}

	// ==========================================================================
	// Image Actions
	// ==========================================================================

	function handleImageClick(): void {
		if (!props.isEditing && hasImage) {
			stateManager.openLightbox(props.blockId, 0, 1);
		}
	}

	function handleImageKeyDown(e: KeyboardEvent): void {
		if ((e.key === 'Enter' || e.key === ' ') && !props.isEditing && hasImage) {
			e.preventDefault();
			stateManager.openLightbox(props.blockId, 0, 1);
		}
	}

	function removeImage(): void {
		updateContent({
			mediaUrl: '',
			mediaAlt: '',
			mediaCaption: ''
		});
		uploadError = null;
	}

	function retryUpload(): void {
		uploadError = null;
		fileInputRef?.click();
	}

	function triggerFileInput(): void {
		fileInputRef?.click();
	}
</script>

<div
	class="image-block"
	class:image-block--selected={props.isSelected}
	class:image-block--editing={props.isEditing}
	role="figure"
	aria-label={imageAlt || 'Image block'}
>
	{#if hasImage && sanitizedURL}
		<!-- Image Display -->
		<div class="image-block__container">
			{#if props.isEditing}
				<!-- Edit Mode Overlay -->
				<div class="image-block__edit-overlay">
					<button
						type="button"
						class="image-block__action-btn image-block__action-btn--remove"
						onclick={removeImage}
						aria-label="Remove image"
					>
						<IconX size={18} aria-hidden="true" />
					</button>
					<button
						type="button"
						class="image-block__action-btn image-block__action-btn--fit"
						onclick={() => (showObjectFitControls = !showObjectFitControls)}
						aria-label="Object fit settings"
						aria-expanded={showObjectFitControls}
					>
						<IconAdjustments size={18} aria-hidden="true" />
					</button>
					<button
						type="button"
						class="image-block__action-btn image-block__action-btn--preview"
						onclick={handleImageClick}
						aria-label="Preview in lightbox"
					>
						<IconMaximize size={18} aria-hidden="true" />
					</button>
				</div>

				<!-- Object Fit Controls -->
				{#if showObjectFitControls}
					<div class="image-block__fit-controls" role="radiogroup" aria-label="Image fit options">
						{#each OBJECT_FIT_OPTIONS as option (option.value)}
							<button
								type="button"
								class="image-block__fit-btn"
								class:image-block__fit-btn--active={objectFit === option.value}
								onclick={() => handleObjectFitChange(option.value)}
								role="radio"
								aria-checked={objectFit === option.value}
							>
								{option.label}
							</button>
						{/each}
					</div>
				{/if}
			{/if}

			<!-- Responsive Image -->
			<img
				src={sanitizedURL}
				srcset={srcset()}
				sizes="(max-width: 480px) 100vw, (max-width: 768px) 90vw, (max-width: 1200px) 80vw, 1200px"
				alt={imageAlt}
				loading="lazy"
				decoding="async"
				class="image-block__image"
				style="object-fit: {objectFit}"
				role={props.isEditing ? undefined : 'button'}
				tabindex={props.isEditing ? -1 : 0}
				aria-label={props.isEditing ? undefined : `View ${imageAlt || 'image'} in lightbox`}
				aria-describedby={caption ? captionId : undefined}
				onclick={props.isEditing ? undefined : handleImageClick}
				onkeydown={props.isEditing ? undefined : handleImageKeyDown}
			/>
		</div>

		<!-- Alt Text Editor (Edit Mode) -->
		{#if props.isEditing}
			<div class="image-block__alt-editor">
				<label class="image-block__alt-label" for={descriptionId}>
					Alt text (accessibility)
				</label>
				<p
					id={descriptionId}
					contenteditable="true"
					class="image-block__alt-input"
					class:image-block__alt-input--empty={!imageAlt}
					oninput={handleAltInput}
					onpaste={handlePaste}
					data-placeholder="Describe this image for screen readers..."
					role="textbox"
					aria-label="Image alt text"
					aria-multiline="false"
				>
					{imageAlt}
				</p>
			</div>
		{/if}

		<!-- Caption -->
		{#if caption || props.isEditing}
			<figcaption
				id={captionId}
				contenteditable={props.isEditing}
				class="image-block__caption"
				class:image-block__caption--empty={!caption && props.isEditing}
				oninput={props.isEditing ? handleCaptionInput : undefined}
				onpaste={props.isEditing ? handlePaste : undefined}
				data-placeholder="Add a caption..."
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Image caption' : undefined}
				aria-multiline="false"
			>
				{caption}
			</figcaption>
		{/if}
	{:else if props.isEditing}
		<!-- Upload Placeholder -->
		<div
			class="image-block__placeholder"
			class:image-block__placeholder--error={!!uploadError}
			class:image-block__placeholder--uploading={isUploading}
			ondrop={handleDrop}
			ondragover={handleDragOver}
			role="region"
			aria-label="Image upload area"
		>
			{#if isUploading}
				<!-- Loading State -->
				<div class="image-block__loading">
					<IconLoader2 size={48} class="image-block__spinner" aria-hidden="true" />
					<span class="image-block__loading-text">Uploading image...</span>
				</div>
			{:else if uploadError}
				<!-- Error State -->
				<div class="image-block__error">
					<IconAlertCircle size={48} aria-hidden="true" />
					<span class="image-block__error-text">{uploadError}</span>
					<button
						type="button"
						class="image-block__retry-btn"
						onclick={retryUpload}
						aria-label="Retry upload"
					>
						<IconRefresh size={18} aria-hidden="true" />
						<span>Try Again</span>
					</button>
				</div>
			{:else}
				<!-- Default Upload UI -->
				<div class="image-block__upload-content">
					<IconPhoto size={56} aria-hidden="true" />
					<span class="image-block__upload-title">Add an image</span>
					<span class="image-block__upload-hint">Drag and drop or click to upload</span>

					<div class="image-block__upload-actions">
						<button
							type="button"
							class="image-block__upload-btn image-block__upload-btn--file"
							onclick={triggerFileInput}
						>
							<IconUpload size={18} aria-hidden="true" />
							<span>Upload File</span>
						</button>

						<span class="image-block__upload-divider">or</span>

						<div class="image-block__url-input-wrapper">
							<IconLink size={18} aria-hidden="true" />
							<input
								type="url"
								class="image-block__url-input"
								placeholder="Paste image URL..."
								bind:value={urlInputValue}
								onkeydown={handleURLKeyDown}
								aria-label="Image URL"
							/>
							<button
								type="button"
								class="image-block__url-submit"
								onclick={handleURLSubmit}
								disabled={!urlInputValue.trim()}
								aria-label="Add image from URL"
							>
								Add
							</button>
						</div>
					</div>

					<span class="image-block__upload-formats">
						Supported: JPEG, PNG, GIF, WebP, SVG (max 10MB)
					</span>
				</div>
			{/if}

			<!-- Hidden file input -->
			<input
				bind:this={fileInputRef}
				type="file"
				accept="image/*"
				onchange={handleFileSelect}
				class="image-block__file-input"
				aria-hidden="true"
				tabindex="-1"
			/>
		</div>
	{:else}
		<!-- Empty State (View Mode) -->
		<div class="image-block__empty" role="status">
			<IconPhoto size={48} aria-hidden="true" />
			<span>No image available</span>
		</div>
	{/if}
</div>

<style>
	/* ==========================================================================
	 * Block Container - BEM Naming Convention
	 * ========================================================================== */

	.image-block {
		width: 100%;
		position: relative;
	}

	.image-block--editing {
		cursor: default;
	}

	/* ==========================================================================
	 * Image Container
	 * ========================================================================== */

	.image-block__container {
		position: relative;
		overflow: hidden;
		border-radius: 8px;
		background: #f1f5f9;
	}

	.image-block__image {
		display: block;
		width: 100%;
		height: auto;
		max-height: 70vh;
		transition: transform 0.2s ease;
	}

	.image-block:not(.image-block--editing) .image-block__image {
		cursor: zoom-in;
	}

	.image-block:not(.image-block--editing) .image-block__image:hover {
		transform: scale(1.01);
	}

	.image-block:not(.image-block--editing) .image-block__image:focus {
		outline: 3px solid #3b82f6;
		outline-offset: 2px;
	}

	/* ==========================================================================
	 * Edit Mode Overlay
	 * ========================================================================== */

	.image-block__edit-overlay {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		display: flex;
		gap: 0.5rem;
		z-index: 10;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.image-block--editing .image-block__container:hover .image-block__edit-overlay {
		opacity: 1;
	}

	.image-block__action-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		border: none;
		border-radius: 8px;
		color: white;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.image-block__action-btn:hover {
		background: rgba(0, 0, 0, 0.85);
		transform: scale(1.05);
	}

	.image-block__action-btn:focus-visible {
		outline: 2px solid white;
		outline-offset: 2px;
	}

	.image-block__action-btn--remove:hover {
		background: #dc2626;
	}

	/* ==========================================================================
	 * Object Fit Controls
	 * ========================================================================== */

	.image-block__fit-controls {
		position: absolute;
		top: 3.5rem;
		right: 0.75rem;
		display: flex;
		gap: 0.25rem;
		padding: 0.375rem;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		border-radius: 8px;
		z-index: 10;
	}

	.image-block__fit-btn {
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.image-block__fit-btn:hover {
		color: white;
		background: rgba(255, 255, 255, 0.1);
	}

	.image-block__fit-btn--active {
		color: white;
		background: #3b82f6;
	}

	.image-block__fit-btn--active:hover {
		background: #2563eb;
	}

	/* ==========================================================================
	 * Alt Text Editor
	 * ========================================================================== */

	.image-block__alt-editor {
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
	}

	.image-block__alt-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		margin-bottom: 0.375rem;
	}

	.image-block__alt-input {
		margin: 0;
		padding: 0.5rem;
		font-size: 0.875rem;
		line-height: 1.5;
		color: #1e293b;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		outline: none;
		min-height: 2.5rem;
		transition: border-color 0.15s ease;
	}

	.image-block__alt-input:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.image-block__alt-input--empty:empty::before {
		content: attr(data-placeholder);
		color: #94a3b8;
	}

	/* ==========================================================================
	 * Caption
	 * ========================================================================== */

	.image-block__caption {
		margin: 0.75rem 0 0;
		padding: 0;
		font-size: 0.875rem;
		font-style: italic;
		line-height: 1.6;
		color: #64748b;
		text-align: center;
		outline: none;
		transition: color 0.15s ease;
	}

	.image-block--editing .image-block__caption {
		padding: 0.5rem;
		border-radius: 6px;
		cursor: text;
	}

	.image-block--editing .image-block__caption:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	.image-block--editing .image-block__caption:focus {
		background: rgba(59, 130, 246, 0.1);
		color: #374151;
	}

	.image-block__caption--empty:empty::before {
		content: attr(data-placeholder);
		color: #94a3b8;
		font-style: normal;
	}

	/* ==========================================================================
	 * Upload Placeholder
	 * ========================================================================== */

	.image-block__placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 280px;
		padding: 2rem;
		background: #f8fafc;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		transition: all 0.2s ease;
	}

	.image-block__placeholder:hover {
		border-color: #3b82f6;
		background: #f1f5f9;
	}

	.image-block__placeholder--uploading {
		border-color: #3b82f6;
		background: linear-gradient(135deg, #eff6ff 0%, #f8fafc 100%);
	}

	.image-block__placeholder--error {
		border-color: #ef4444;
		background: #fef2f2;
	}

	/* ==========================================================================
	 * Upload Content
	 * ========================================================================== */

	.image-block__upload-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		color: #64748b;
		text-align: center;
	}

	.image-block__upload-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1e293b;
	}

	.image-block__upload-hint {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.image-block__upload-actions {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		margin-top: 1rem;
		width: 100%;
		max-width: 400px;
	}

	.image-block__upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.image-block__upload-btn:hover {
		background: #2563eb;
		transform: translateY(-1px);
	}

	.image-block__upload-btn:active {
		transform: translateY(0);
	}

	.image-block__upload-btn:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.image-block__upload-divider {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.image-block__url-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		transition: border-color 0.15s ease;
	}

	.image-block__url-input-wrapper:focus-within {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.image-block__url-input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		color: #1e293b;
		outline: none;
	}

	.image-block__url-input::placeholder {
		color: #94a3b8;
	}

	.image-block__url-submit {
		padding: 0.375rem 0.75rem;
		background: #1e293b;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.image-block__url-submit:hover:not(:disabled) {
		background: #334155;
	}

	.image-block__url-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.image-block__upload-formats {
		font-size: 0.75rem;
		color: #94a3b8;
		margin-top: 0.5rem;
	}

	.image-block__file-input {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ==========================================================================
	 * Loading State
	 * ========================================================================== */

	.image-block__loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		color: #3b82f6;
	}

	.image-block__loading-text {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #64748b;
	}

	:global(.image-block__spinner) {
		animation: image-block-spin 1s linear infinite;
	}

	@keyframes image-block-spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* ==========================================================================
	 * Error State
	 * ========================================================================== */

	.image-block__error {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		color: #ef4444;
	}

	.image-block__error-text {
		font-size: 0.9375rem;
		color: #dc2626;
		text-align: center;
	}

	.image-block__retry-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: white;
		border: 1px solid #fca5a5;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.image-block__retry-btn:hover {
		background: #fef2f2;
		border-color: #ef4444;
	}

	.image-block__retry-btn:focus-visible {
		outline: 2px solid #ef4444;
		outline-offset: 2px;
	}

	/* ==========================================================================
	 * Empty State (View Mode)
	 * ========================================================================== */

	.image-block__empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		color: #94a3b8;
	}

	/* ==========================================================================
	 * Mobile Responsive
	 * ========================================================================== */

	@media (max-width: 640px) {
		.image-block__image {
			max-height: 50vh;
		}

		.image-block__placeholder {
			min-height: 220px;
			padding: 1.5rem;
		}

		.image-block__upload-content {
			gap: 0.5rem;
		}

		.image-block__upload-title {
			font-size: 1rem;
		}

		.image-block__upload-actions {
			gap: 0.5rem;
		}

		.image-block__edit-overlay {
			opacity: 1;
		}

		.image-block__action-btn {
			width: 32px;
			height: 32px;
		}

		.image-block__fit-controls {
			flex-wrap: wrap;
			justify-content: center;
		}
	}

	/* ==========================================================================
	 * Dark Mode
	 * ========================================================================== */

	:global(.dark) .image-block__container {
		background: #1e293b;
	}

	:global(.dark) .image-block__alt-editor {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .image-block__alt-label {
		color: #94a3b8;
	}

	:global(.dark) .image-block__alt-input {
		background: #0f172a;
		border-color: #334155;
		color: #e2e8f0;
	}

	:global(.dark) .image-block__alt-input:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	:global(.dark) .image-block__alt-input--empty:empty::before {
		color: #64748b;
	}

	:global(.dark) .image-block__caption {
		color: #94a3b8;
	}

	:global(.dark) .image-block--editing .image-block__caption:hover {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .image-block--editing .image-block__caption:focus {
		background: rgba(59, 130, 246, 0.15);
		color: #e2e8f0;
	}

	:global(.dark) .image-block__caption--empty:empty::before {
		color: #64748b;
	}

	:global(.dark) .image-block__placeholder {
		background: #1e293b;
		border-color: #475569;
	}

	:global(.dark) .image-block__placeholder:hover {
		border-color: #3b82f6;
		background: #0f172a;
	}

	:global(.dark) .image-block__placeholder--uploading {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
	}

	:global(.dark) .image-block__placeholder--error {
		background: rgba(239, 68, 68, 0.1);
		border-color: #dc2626;
	}

	:global(.dark) .image-block__upload-title {
		color: #f1f5f9;
	}

	:global(.dark) .image-block__upload-hint,
	:global(.dark) .image-block__upload-divider,
	:global(.dark) .image-block__upload-formats {
		color: #64748b;
	}

	:global(.dark) .image-block__url-input-wrapper {
		background: #0f172a;
		border-color: #475569;
	}

	:global(.dark) .image-block__url-input-wrapper:focus-within {
		border-color: #3b82f6;
	}

	:global(.dark) .image-block__url-input {
		color: #e2e8f0;
	}

	:global(.dark) .image-block__url-input::placeholder {
		color: #64748b;
	}

	:global(.dark) .image-block__url-submit {
		background: #f1f5f9;
		color: #0f172a;
	}

	:global(.dark) .image-block__url-submit:hover:not(:disabled) {
		background: white;
	}

	:global(.dark) .image-block__loading-text {
		color: #94a3b8;
	}

	:global(.dark) .image-block__error-text {
		color: #f87171;
	}

	:global(.dark) .image-block__retry-btn {
		background: #1e293b;
		border-color: #7f1d1d;
		color: #f87171;
	}

	:global(.dark) .image-block__retry-btn:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: #dc2626;
	}

	:global(.dark) .image-block__empty {
		background: #1e293b;
		border-color: #334155;
		color: #64748b;
	}
</style>
