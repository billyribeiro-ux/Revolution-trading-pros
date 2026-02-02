<!--
/**
 * Gallery Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Multi-image gallery with lightbox, keyboard navigation, and responsive grid
 *
 * Features:
 * - Layout options: grid, masonry, carousel
 * - Configurable grid columns (2-4)
 * - Lightbox with prev/next navigation
 * - Keyboard navigation (ArrowLeft, ArrowRight, Escape)
 * - Image counter display (e.g., "2 / 5")
 * - Add/remove images in edit mode
 * - Caption editing for each image
 * - Drag-and-drop reordering in edit mode
 * - Gap control between images
 * - Full ARIA accessibility
 * - Dark mode support
 * - Mobile responsive (1-2 columns max on small screens)
 */
-->

<script lang="ts">
	import {
		IconPhoto,
		IconPlus,
		IconTrash,
		IconX,
		IconChevronLeft,
		IconChevronRight,
		IconLayoutGrid,
		IconColumns
	} from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import {
		getBlockStateManager,
		type BlockId,
		type LightboxState
	} from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	// ============================================================================
	// Types
	// ============================================================================

	interface GalleryImage {
		id: string;
		url: string;
		alt: string;
		caption?: string;
	}

	type GalleryLayout = 'grid' | 'masonry' | 'carousel';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// ============================================================================
	// Props and State
	// ============================================================================

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	// Local state for URL input when adding new images
	let newImageUrl = $state('');
	let newImageAlt = $state('');
	let newImageCaption = $state('');
	let showAddForm = $state(false);

	// Drag and drop state
	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);

	// Carousel state
	let carouselIndex = $state(0);

	// ============================================================================
	// Derived Values
	// ============================================================================

	let images = $derived<GalleryImage[]>(
		props.block.content.galleryImages?.map((img, idx) => ({
			id: img.id || `img_${idx}_${Date.now()}`,
			url: img.url || '',
			alt: img.alt || '',
			caption: img.caption || ''
		})) || []
	);

	let layout = $derived<GalleryLayout>(
		(props.block.settings.galleryLayout as GalleryLayout) || 'grid'
	);

	let columns = $derived(
		Math.min(4, Math.max(2, props.block.settings.galleryColumns || 3))
	);

	let gap = $derived(props.block.settings.gap || '1rem');

	let lightboxState = $derived<LightboxState>(stateManager.getLightboxState());

	let isLightboxOpen = $derived(
		lightboxState.open && lightboxState.blockId === props.blockId
	);

	let currentIndex = $derived(lightboxState.index);

	let currentImage = $derived<GalleryImage | null>(
		isLightboxOpen && images[currentIndex] ? images[currentIndex] : null
	);

	let imageCounter = $derived(
		isLightboxOpen ? `${currentIndex + 1} / ${images.length}` : ''
	);

	// Grid style computed from columns and gap
	let gridStyle = $derived(
		`--gallery-columns: ${columns}; --gallery-gap: ${gap};`
	);

	// ============================================================================
	// Content Update Helpers
	// ============================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<Block['settings']>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	// ============================================================================
	// Image Management
	// ============================================================================

	function addImage(): void {
		if (!newImageUrl.trim()) return;

		const newImage: GalleryImage = {
			id: `img_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`,
			url: newImageUrl.trim(),
			alt: newImageAlt.trim() || 'Gallery image',
			caption: newImageCaption.trim()
		};

		const updatedImages = [
			...images,
			{ url: newImage.url, alt: newImage.alt, id: newImage.id, caption: newImage.caption }
		];

		updateContent({ galleryImages: updatedImages });
		newImageUrl = '';
		newImageAlt = '';
		newImageCaption = '';
		showAddForm = false;
	}

	function removeImage(index: number): void {
		if (index < 0 || index >= images.length) return;

		const updatedImages = images
			.filter((_, i) => i !== index)
			.map((img) => ({ url: img.url, alt: img.alt, id: img.id, caption: img.caption }));

		updateContent({ galleryImages: updatedImages });

		// Close lightbox if we removed the last image or current image
		if (updatedImages.length === 0 || (isLightboxOpen && currentIndex >= updatedImages.length)) {
			stateManager.closeLightbox();
		}
	}

	function updateImageAlt(index: number, alt: string): void {
		if (index < 0 || index >= images.length) return;

		const updatedImages = images.map((img, i) =>
			i === index
				? { url: img.url, alt, id: img.id, caption: img.caption }
				: { url: img.url, alt: img.alt, id: img.id, caption: img.caption }
		);

		updateContent({ galleryImages: updatedImages });
	}

	function updateImageCaption(index: number, caption: string): void {
		if (index < 0 || index >= images.length) return;

		const updatedImages = images.map((img, i) =>
			i === index
				? { url: img.url, alt: img.alt, id: img.id, caption }
				: { url: img.url, alt: img.alt, id: img.id, caption: img.caption }
		);

		updateContent({ galleryImages: updatedImages });
	}

	function updateImageUrl(index: number, url: string): void {
		if (index < 0 || index >= images.length) return;

		const updatedImages = images.map((img, i) =>
			i === index
				? { url, alt: img.alt, id: img.id, caption: img.caption }
				: { url: img.url, alt: img.alt, id: img.id, caption: img.caption }
		);

		updateContent({ galleryImages: updatedImages });
	}

	// ============================================================================
	// Drag and Drop Reordering
	// ============================================================================

	function handleDragStart(event: DragEvent, index: number): void {
		draggedIndex = index;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			event.dataTransfer.setData('text/plain', String(index));
		}
	}

	function handleDragOver(event: DragEvent, index: number): void {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
		dragOverIndex = index;
	}

	function handleDragLeave(): void {
		dragOverIndex = null;
	}

	function handleDrop(event: DragEvent, dropIndex: number): void {
		event.preventDefault();

		if (draggedIndex === null || draggedIndex === dropIndex) {
			draggedIndex = null;
			dragOverIndex = null;
			return;
		}

		const reorderedImages = [...images];
		const [movedImage] = reorderedImages.splice(draggedIndex, 1);
		reorderedImages.splice(dropIndex, 0, movedImage);

		const updatedImages = reorderedImages.map((img) => ({
			url: img.url,
			alt: img.alt,
			id: img.id,
			caption: img.caption
		}));

		updateContent({ galleryImages: updatedImages });
		draggedIndex = null;
		dragOverIndex = null;
	}

	function handleDragEnd(): void {
		draggedIndex = null;
		dragOverIndex = null;
	}

	// ============================================================================
	// Layout Management
	// ============================================================================

	function updateLayout(newLayout: GalleryLayout): void {
		updateSettings({ galleryLayout: newLayout });
	}

	// Carousel Navigation
	function carouselNext(): void {
		if (images.length > 0) {
			carouselIndex = (carouselIndex + 1) % images.length;
		}
	}

	function carouselPrev(): void {
		if (images.length > 0) {
			carouselIndex = (carouselIndex - 1 + images.length) % images.length;
		}
	}

	// ============================================================================
	// Lightbox Controls
	// ============================================================================

	function openLightbox(index: number): void {
		if (props.isEditing) return;
		if (images.length === 0) return;

		stateManager.openLightbox(props.blockId, index, images.length);
	}

	function closeLightbox(): void {
		stateManager.closeLightbox();
	}

	function navigateNext(): void {
		if (!isLightboxOpen) return;
		stateManager.navigateLightbox('next');
	}

	function navigatePrev(): void {
		if (!isLightboxOpen) return;
		stateManager.navigateLightbox('prev');
	}

	// ============================================================================
	// Keyboard Navigation
	// ============================================================================

	function handleKeyDown(event: KeyboardEvent): void {
		if (!isLightboxOpen) return;

		switch (event.key) {
			case 'Escape':
				event.preventDefault();
				closeLightbox();
				break;
			case 'ArrowLeft':
				event.preventDefault();
				navigatePrev();
				break;
			case 'ArrowRight':
				event.preventDefault();
				navigateNext();
				break;
		}
	}

	// ============================================================================
	// Effects
	// ============================================================================

	$effect(() => {
		if (isLightboxOpen) {
			if (typeof window !== 'undefined') {
				window.addEventListener('keydown', handleKeyDown);
			}
		}

		return () => {
			if (typeof window !== 'undefined') {
				window.removeEventListener('keydown', handleKeyDown);
			}
		};
	});

	// ============================================================================
	// Event Handlers
	// ============================================================================

	function handleImageClick(index: number): void {
		if (!props.isEditing) {
			openLightbox(index);
		}
	}

	function handleImageKeyDown(event: KeyboardEvent, index: number): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleImageClick(index);
		}
	}

	function handleOverlayClick(event: MouseEvent): void {
		if (event.target === event.currentTarget) {
			closeLightbox();
		}
	}

	function handleAddFormKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			addImage();
		} else if (event.key === 'Escape') {
			event.preventDefault();
			showAddForm = false;
			newImageUrl = '';
			newImageAlt = '';
			newImageCaption = '';
		}
	}

	function handleColumnsChange(value: number): void {
		updateSettings({ galleryColumns: value });
	}

	function handleGapChange(value: string): void {
		updateSettings({ gap: value });
	}
</script>

<!-- ============================================================================
     Template
     ============================================================================ -->

<div
	class="gallery-block"
	class:editing={props.isEditing}
	class:selected={props.isSelected}
	class:layout-grid={layout === 'grid'}
	class:layout-masonry={layout === 'masonry'}
	class:layout-carousel={layout === 'carousel'}
	role="group"
	aria-label="Image gallery"
>
	{#if images.length > 0}
		<!-- Grid and Masonry Layout -->
		{#if layout === 'grid' || layout === 'masonry'}
			<div
				class="gallery-grid"
				class:masonry={layout === 'masonry'}
				style={gridStyle}
			>
				{#each images as image, index (image.id)}
					<div
						class="gallery-item"
						class:dragging={draggedIndex === index}
						class:drag-over={dragOverIndex === index}
						draggable={props.isEditing}
						ondragstart={(e) => handleDragStart(e, index)}
						ondragover={(e) => handleDragOver(e, index)}
						ondragleave={handleDragLeave}
						ondrop={(e) => handleDrop(e, index)}
						ondragend={handleDragEnd}
					>
						{#if props.isEditing}
							<div class="gallery-item-edit">
								<img
									src={sanitizeURL(image.url)}
									alt={image.alt}
									class="gallery-image"
									loading="lazy"
								/>
								<div class="gallery-item-overlay">
									<div class="drag-handle" aria-label="Drag to reorder">
										<IconColumns size={16} />
									</div>
									<button
										type="button"
										class="gallery-item-remove"
										onclick={() => removeImage(index)}
										aria-label="Remove image"
									>
										<IconTrash size={16} />
									</button>
								</div>
								<div class="gallery-item-meta">
									<input
										type="text"
										class="gallery-alt-input"
										placeholder="Alt text (accessibility)"
										value={image.alt}
										oninput={(e) => updateImageAlt(index, (e.target as HTMLInputElement).value)}
									/>
									<input
										type="text"
										class="gallery-caption-input"
										placeholder="Caption (visible)"
										value={image.caption}
										oninput={(e) => updateImageCaption(index, (e.target as HTMLInputElement).value)}
									/>
									<input
										type="url"
										class="gallery-url-input"
										placeholder="Image URL"
										value={image.url}
										oninput={(e) => updateImageUrl(index, (e.target as HTMLInputElement).value)}
									/>
								</div>
							</div>
						{:else}
							<button
								type="button"
								class="gallery-item-button"
								onclick={() => handleImageClick(index)}
								onkeydown={(e) => handleImageKeyDown(e, index)}
								role="button"
								aria-label="View {image.alt || 'image'} in lightbox"
								tabindex="0"
							>
								<img
									src={sanitizeURL(image.url)}
									alt={image.alt}
									class="gallery-image"
									loading="lazy"
								/>
								<div class="gallery-item-hover">
									<IconPhoto size={24} />
								</div>
								{#if image.caption}
									<div class="gallery-item-caption">
										{image.caption}
									</div>
								{/if}
							</button>
						{/if}
					</div>
				{/each}
			</div>
		{:else if layout === 'carousel'}
			<!-- Carousel Layout -->
			<div class="gallery-carousel">
				<div class="carousel-viewport">
					{#if images[carouselIndex]}
						<button
							type="button"
							class="carousel-image-button"
							onclick={() => handleImageClick(carouselIndex)}
							onkeydown={(e) => handleImageKeyDown(e, carouselIndex)}
							aria-label="View {images[carouselIndex].alt || 'image'} in lightbox"
						>
							<img
								src={sanitizeURL(images[carouselIndex].url)}
								alt={images[carouselIndex].alt}
								class="carousel-image"
							/>
							{#if images[carouselIndex].caption}
								<div class="carousel-caption">
									{images[carouselIndex].caption}
								</div>
							{/if}
						</button>
					{/if}
				</div>
				{#if images.length > 1}
					<button
						type="button"
						class="carousel-nav carousel-prev"
						onclick={carouselPrev}
						aria-label="Previous image"
					>
						<IconChevronLeft size={24} />
					</button>
					<button
						type="button"
						class="carousel-nav carousel-next"
						onclick={carouselNext}
						aria-label="Next image"
					>
						<IconChevronRight size={24} />
					</button>
					<div class="carousel-dots">
						{#each images as _, dotIndex}
							<button
								type="button"
								class="carousel-dot"
								class:active={dotIndex === carouselIndex}
								onclick={() => (carouselIndex = dotIndex)}
								aria-label="Go to image {dotIndex + 1}"
							></button>
						{/each}
					</div>
				{/if}
				<div class="carousel-counter" aria-live="polite">
					{carouselIndex + 1} / {images.length}
				</div>
			</div>
		{/if}
	{:else}
		<div class="gallery-empty">
			<IconLayoutGrid size={48} aria-hidden="true" />
			<p>No images in gallery</p>
			{#if props.isEditing}
				<button type="button" class="gallery-add-btn-empty" onclick={() => (showAddForm = true)}>
					<IconPlus size={16} />
					Add Images
				</button>
			{/if}
		</div>
	{/if}

	<!-- Edit Mode Controls -->
	{#if props.isEditing && props.isSelected}
		<div class="gallery-controls">
			<!-- Add Image Form -->
			{#if showAddForm}
				<div class="gallery-add-form" onkeydown={handleAddFormKeyDown}>
					<div class="add-form-header">
						<span>Add New Image</span>
						<button
							type="button"
							class="add-form-close"
							onclick={() => {
								showAddForm = false;
								newImageUrl = '';
								newImageAlt = '';
								newImageCaption = '';
							}}
							aria-label="Close add form"
						>
							<IconX size={16} />
						</button>
					</div>
					<input
						type="url"
						class="add-form-input"
						placeholder="Image URL (https://...)"
						bind:value={newImageUrl}
					/>
					<input
						type="text"
						class="add-form-input"
						placeholder="Alt text for accessibility"
						bind:value={newImageAlt}
					/>
					<input
						type="text"
						class="add-form-input"
						placeholder="Caption (optional, visible below image)"
						bind:value={newImageCaption}
					/>
					<button
						type="button"
						class="add-form-submit"
						onclick={addImage}
						disabled={!newImageUrl.trim()}
					>
						<IconPlus size={16} />
						Add Image
					</button>
				</div>
			{:else if images.length > 0}
				<button type="button" class="gallery-add-btn" onclick={() => (showAddForm = true)}>
					<IconPlus size={16} />
					Add Image
				</button>
			{/if}

			<!-- Gallery Settings -->
			<div class="gallery-settings">
				<div class="setting-group">
					<label class="setting-label" for="layout-{props.blockId}">Layout</label>
					<select
						id="layout-{props.blockId}"
						class="setting-select"
						value={layout}
						onchange={(e) => updateLayout((e.target as HTMLSelectElement).value as GalleryLayout)}
					>
						<option value="grid">Grid</option>
						<option value="masonry">Masonry</option>
						<option value="carousel">Carousel</option>
					</select>
				</div>
				{#if layout !== 'carousel'}
					<div class="setting-group">
						<label class="setting-label" for="columns-{props.blockId}">Columns</label>
						<select
							id="columns-{props.blockId}"
							class="setting-select"
							value={columns}
							onchange={(e) => handleColumnsChange(parseInt((e.target as HTMLSelectElement).value))}
						>
							{#each [2, 3, 4] as col}
								<option value={col}>{col}</option>
							{/each}
						</select>
					</div>
					<div class="setting-group">
						<label class="setting-label" for="gap-{props.blockId}">Gap</label>
						<select
							id="gap-{props.blockId}"
							class="setting-select"
							value={gap}
							onchange={(e) => handleGapChange((e.target as HTMLSelectElement).value)}
						>
							<option value="0.25rem">XS</option>
							<option value="0.5rem">SM</option>
							<option value="1rem">MD</option>
							<option value="1.5rem">LG</option>
							<option value="2rem">XL</option>
						</select>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<!-- Lightbox Overlay -->
{#if isLightboxOpen && currentImage}
	<div
		class="lightbox-overlay"
		onclick={handleOverlayClick}
		onkeydown={() => {}}
		role="dialog"
		aria-modal="true"
		aria-label="Image lightbox"
	>
		<div class="lightbox-content">
			<!-- Close Button -->
			<button
				type="button"
				class="lightbox-close"
				onclick={closeLightbox}
				aria-label="Close lightbox"
			>
				<IconX size={24} />
			</button>

			<!-- Navigation: Previous -->
			{#if images.length > 1}
				<button
					type="button"
					class="lightbox-nav lightbox-prev"
					onclick={navigatePrev}
					aria-label="Previous image"
				>
					<IconChevronLeft size={32} />
				</button>
			{/if}

			<!-- Image Container -->
			<div class="lightbox-image-container">
				<img
					src={sanitizeURL(currentImage.url)}
					alt={currentImage.alt}
					class="lightbox-image"
				/>
			</div>

			<!-- Navigation: Next -->
			{#if images.length > 1}
				<button
					type="button"
					class="lightbox-nav lightbox-next"
					onclick={navigateNext}
					aria-label="Next image"
				>
					<IconChevronRight size={32} />
				</button>
			{/if}

			<!-- Image Counter -->
			<div class="lightbox-counter" aria-live="polite">
				{imageCounter}
			</div>

			<!-- Image Caption -->
			{#if currentImage.caption || currentImage.alt}
				<div class="lightbox-caption">
					{currentImage.caption || currentImage.alt}
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- ============================================================================
     Styles
     ============================================================================ -->

<style>
	/* ========================================================================
	   Gallery Block Container
	   ======================================================================== */

	.gallery-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		background: #ffffff;
	}

	.gallery-block.editing {
		border-style: dashed;
	}

	.gallery-block.selected {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	/* ========================================================================
	   Gallery Grid
	   ======================================================================== */

	.gallery-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: var(--gallery-gap, 1rem);
		padding: 1rem;
	}

	/* Override for explicit column count when set */
	.layout-grid .gallery-grid {
		grid-template-columns: repeat(var(--gallery-columns, 3), 1fr);
	}

	/* Masonry Layout */
	.gallery-grid.masonry {
		display: block;
		column-count: var(--gallery-columns, 3);
		column-gap: var(--gallery-gap, 1rem);
	}

	.gallery-grid.masonry .gallery-item {
		break-inside: avoid;
		margin-bottom: var(--gallery-gap, 1rem);
	}

	/* ========================================================================
	   Gallery Item
	   ======================================================================== */

	.gallery-item {
		position: relative;
		aspect-ratio: 1;
		overflow: hidden;
		border-radius: 8px;
		background: #f1f5f9;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.gallery-grid.masonry .gallery-item {
		aspect-ratio: auto;
	}

	/* Drag and Drop States */
	.gallery-item.dragging {
		opacity: 0.5;
		transform: scale(0.95);
	}

	.gallery-item.drag-over {
		box-shadow: 0 0 0 3px #3b82f6;
		transform: scale(1.02);
	}

	.gallery-item-button {
		display: block;
		width: 100%;
		height: 100%;
		padding: 0;
		border: none;
		background: transparent;
		cursor: pointer;
		position: relative;
	}

	.gallery-item-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.gallery-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	.gallery-item-button:hover .gallery-image {
		transform: scale(1.05);
	}

	.gallery-item-hover {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		color: white;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.gallery-item-button:hover .gallery-item-hover,
	.gallery-item-button:focus-visible .gallery-item-hover {
		opacity: 1;
	}

	/* ========================================================================
	   Edit Mode Item
	   ======================================================================== */

	.gallery-item-edit {
		width: 100%;
		height: 100%;
		position: relative;
	}

	.gallery-item-overlay {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		gap: 0.25rem;
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.gallery-item-edit:hover .gallery-item-overlay {
		opacity: 1;
	}

	.gallery-item-remove {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: rgba(220, 38, 38, 0.9);
		border: none;
		border-radius: 6px;
		color: white;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.gallery-item-remove:hover {
		background: #dc2626;
	}

	.gallery-item-meta {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.gallery-alt-input,
	.gallery-caption-input,
	.gallery-url-input {
		width: 100%;
		padding: 0.375rem 0.5rem;
		font-size: 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 4px;
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.gallery-alt-input::placeholder,
	.gallery-caption-input::placeholder,
	.gallery-url-input::placeholder {
		color: rgba(255, 255, 255, 0.5);
	}

	.gallery-alt-input:focus,
	.gallery-caption-input:focus,
	.gallery-url-input:focus {
		outline: none;
		border-color: #3b82f6;
	}

	/* Drag Handle */
	.drag-handle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 6px;
		color: white;
		cursor: grab;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	/* Caption Display */
	.gallery-item-caption {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0.5rem 0.75rem;
		background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
		color: white;
		font-size: 0.8125rem;
		line-height: 1.4;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.gallery-item-button:hover .gallery-item-caption,
	.gallery-item-button:focus-visible .gallery-item-caption {
		opacity: 1;
	}

	/* ========================================================================
	   Empty State
	   ======================================================================== */

	.gallery-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 3rem 2rem;
		color: #94a3b8;
		text-align: center;
	}

	.gallery-empty p {
		margin: 0;
		font-size: 0.9375rem;
	}

	.gallery-add-btn-empty {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.gallery-add-btn-empty:hover {
		background: #2563eb;
	}

	/* ========================================================================
	   Gallery Controls
	   ======================================================================== */

	.gallery-controls {
		padding: 1rem;
		background: #f8fafc;
		border-top: 1px solid #e5e7eb;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.gallery-add-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #e0f2fe;
		border: 1px dashed #3b82f6;
		border-radius: 6px;
		color: #3b82f6;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
		align-self: flex-start;
	}

	.gallery-add-btn:hover {
		background: #bae6fd;
	}

	/* ========================================================================
	   Add Image Form
	   ======================================================================== */

	.gallery-add-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
	}

	.add-form-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-weight: 600;
		font-size: 0.875rem;
		color: #374151;
	}

	.add-form-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
	}

	.add-form-close:hover {
		background: #f3f4f6;
		color: #374151;
	}

	.add-form-input {
		width: 100%;
		padding: 0.625rem 0.75rem;
		font-size: 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
	}

	.add-form-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.add-form-submit {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.add-form-submit:hover:not(:disabled) {
		background: #2563eb;
	}

	.add-form-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ========================================================================
	   Gallery Settings
	   ======================================================================== */

	.gallery-settings {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.setting-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.setting-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.setting-select {
		padding: 0.375rem 0.625rem;
		font-size: 0.8125rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		cursor: pointer;
	}

	.setting-select:focus {
		outline: none;
		border-color: #3b82f6;
	}

	/* ========================================================================
	   Carousel Layout
	   ======================================================================== */

	.gallery-carousel {
		position: relative;
		padding: 1rem;
	}

	.carousel-viewport {
		position: relative;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		border-radius: 8px;
		background: #f1f5f9;
	}

	.carousel-image-button {
		display: block;
		width: 100%;
		height: 100%;
		padding: 0;
		border: none;
		background: transparent;
		cursor: pointer;
		position: relative;
	}

	.carousel-image-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.carousel-image {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.carousel-caption {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0.75rem 1rem;
		background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
		color: white;
		font-size: 0.875rem;
		text-align: center;
	}

	.carousel-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(255, 255, 255, 0.9);
		border: none;
		border-radius: 50%;
		color: #374151;
		cursor: pointer;
		transition: background 0.15s ease, color 0.15s ease;
		z-index: 10;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}

	.carousel-nav:hover {
		background: white;
		color: #111827;
	}

	.carousel-prev {
		left: 1.5rem;
	}

	.carousel-next {
		right: 1.5rem;
	}

	.carousel-dots {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.carousel-dot {
		width: 8px;
		height: 8px;
		padding: 0;
		background: #d1d5db;
		border: none;
		border-radius: 50%;
		cursor: pointer;
		transition: background 0.15s ease, transform 0.15s ease;
	}

	.carousel-dot:hover {
		background: #9ca3af;
	}

	.carousel-dot.active {
		background: #3b82f6;
		transform: scale(1.25);
	}

	.carousel-counter {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 20px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	/* ========================================================================
	   Lightbox
	   ======================================================================== */

	.lightbox-overlay {
		position: fixed;
		inset: 0;
		z-index: 9999;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.9);
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.lightbox-content {
		position: relative;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 4rem 5rem;
	}

	.lightbox-close {
		position: absolute;
		top: 1rem;
		right: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s ease;
		z-index: 10;
	}

	.lightbox-close:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.lightbox-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s ease;
		z-index: 10;
	}

	.lightbox-nav:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.lightbox-prev {
		left: 1rem;
	}

	.lightbox-next {
		right: 1rem;
	}

	.lightbox-image-container {
		max-width: 90%;
		max-height: 85vh;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lightbox-image {
		max-width: 100%;
		max-height: 85vh;
		object-fit: contain;
		border-radius: 4px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.lightbox-counter {
		position: absolute;
		top: 1rem;
		left: 50%;
		transform: translateX(-50%);
		padding: 0.5rem 1rem;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 20px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.lightbox-caption {
		position: absolute;
		bottom: 2rem;
		left: 50%;
		transform: translateX(-50%);
		max-width: 80%;
		padding: 0.75rem 1.25rem;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 8px;
		color: white;
		font-size: 0.9375rem;
		text-align: center;
		line-height: 1.5;
	}

	/* ========================================================================
	   Dark Mode
	   ======================================================================== */

	:global(.dark) .gallery-block {
		background: #111827;
		border-color: #374151;
	}

	:global(.dark) .gallery-block.selected {
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
	}

	:global(.dark) .gallery-item {
		background: #1e293b;
	}

	:global(.dark) .gallery-empty {
		color: #64748b;
	}

	:global(.dark) .gallery-add-btn-empty {
		background: #2563eb;
	}

	:global(.dark) .gallery-add-btn-empty:hover {
		background: #1d4ed8;
	}

	:global(.dark) .gallery-controls {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .gallery-add-btn {
		background: rgba(59, 130, 246, 0.1);
		border-color: #3b82f6;
	}

	:global(.dark) .gallery-add-btn:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	:global(.dark) .gallery-add-form {
		background: #0f172a;
		border-color: #374151;
	}

	:global(.dark) .add-form-header {
		color: #e2e8f0;
	}

	:global(.dark) .add-form-close {
		color: #94a3b8;
	}

	:global(.dark) .add-form-close:hover {
		background: #1e293b;
		color: #e2e8f0;
	}

	:global(.dark) .add-form-input {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .add-form-input:focus {
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
	}

	:global(.dark) .setting-label {
		color: #e2e8f0;
	}

	:global(.dark) .setting-select {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .setting-select:focus {
		border-color: #60a5fa;
	}

	:global(.dark) .gallery-alt-input,
	:global(.dark) .gallery-caption-input,
	:global(.dark) .gallery-url-input {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.15);
	}

	/* Carousel Dark Mode */
	:global(.dark) .carousel-viewport {
		background: #1e293b;
	}

	:global(.dark) .carousel-nav {
		background: rgba(30, 41, 59, 0.9);
		color: #e2e8f0;
	}

	:global(.dark) .carousel-nav:hover {
		background: #334155;
		color: white;
	}

	:global(.dark) .carousel-dot {
		background: #475569;
	}

	:global(.dark) .carousel-dot:hover {
		background: #64748b;
	}

	:global(.dark) .carousel-dot.active {
		background: #60a5fa;
	}

	/* ========================================================================
	   Mobile Responsive
	   ======================================================================== */

	@media (max-width: 768px) {
		.gallery-grid {
			grid-template-columns: repeat(min(var(--gallery-columns, 3), 2), 1fr);
		}

		.lightbox-content {
			padding: 3rem 1rem;
		}

		.lightbox-nav {
			width: 40px;
			height: 40px;
		}

		.lightbox-prev {
			left: 0.5rem;
		}

		.lightbox-next {
			right: 0.5rem;
		}

		.lightbox-close {
			top: 0.5rem;
			right: 0.5rem;
			width: 40px;
			height: 40px;
		}

		.lightbox-counter {
			font-size: 0.8125rem;
			padding: 0.375rem 0.75rem;
		}

		.lightbox-caption {
			bottom: 1rem;
			font-size: 0.8125rem;
			padding: 0.5rem 1rem;
		}

		/* Carousel mobile */
		.carousel-nav {
			width: 36px;
			height: 36px;
		}

		.carousel-prev {
			left: 0.75rem;
		}

		.carousel-next {
			right: 0.75rem;
		}

		/* Masonry mobile */
		.gallery-grid.masonry {
			column-count: 2;
		}
	}

	@media (max-width: 480px) {
		.gallery-grid {
			grid-template-columns: 1fr;
		}

		.gallery-grid.masonry {
			column-count: 1;
		}

		.gallery-settings {
			flex-direction: column;
			gap: 0.75rem;
		}

		.setting-group {
			justify-content: space-between;
		}

		.lightbox-image-container {
			max-width: 100%;
		}

		/* Carousel very small screens */
		.carousel-dots {
			gap: 0.375rem;
		}

		.carousel-dot {
			width: 6px;
			height: 6px;
		}
	}
</style>
