<!--
  ImageCropModal Component
  ═══════════════════════════════════════════════════════════════════════════

  Image cropping and editing modal with:
  - Crop to preset aspect ratios
  - Free-form cropping
  - Rotate and flip
  - Focal point selection
  - Zoom controls
  - Real-time preview

  @version 1.0.0
-->
<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { untrack } from 'svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════
	interface CropArea {
		x: number;
		y: number;
		width: number;
		height: number;
	}

	interface CropResult {
		blob: Blob;
		cropArea: CropArea;
		aspectRatio: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Props - Svelte 5 $props() rune
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		src: string;
		aspectRatios?: Record<string, number | null>;
		initialAspectRatio?: string;
		minWidth?: number;
		minHeight?: number;
		className?: string;
		oncrop?: (result: CropResult) => void;
		oncancel?: () => void;
	}

	let {
		src,
		aspectRatios = {
			Free: null,
			'1:1': 1,
			'16:9': 16 / 9,
			'4:3': 4 / 3,
			'3:2': 3 / 2,
			'9:16': 9 / 16
		},
		initialAspectRatio = 'Free',
		minWidth = 50,
		minHeight = 50,
		className = '',
		oncrop,
		oncancel
	}: Props = $props();

	// Helper to dispatch events (for backward compatibility with on:event usage)
	function dispatch(event: 'crop' | 'cancel', detail?: CropResult) {
		if (event === 'crop' && detail) {
			oncrop?.(detail);
		} else if (event === 'cancel') {
			oncancel?.();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State - Svelte 5 $state() rune
	// ═══════════════════════════════════════════════════════════════════════════
	let canvas: HTMLCanvasElement | undefined = $state();
	let image: HTMLImageElement | undefined = $state();
	let imageLoaded = $state(false);
	// Intentionally capture initial value only - selectedRatio is user-controlled after mount
	// svelte-ignore state_referenced_locally
	let selectedRatio = $state(initialAspectRatio || 'Free');
	let rotation = $state(0);
	let flipH = $state(false);
	let flipV = $state(false);
	let zoom = $state(1);
	let isDragging = $state(false);
	let isResizing = $state(false);
	let resizeHandle = $state('');
	let startX = $state(0);
	let startY = $state(0);

	// Crop area (in image coordinates)
	let cropArea: CropArea = $state({ x: 0, y: 0, width: 100, height: 100 });

	// Container dimensions
	let containerWidth = $state(0);
	let containerHeight = $state(0);
	let imageScale = $state(1);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect() rune
	// ═══════════════════════════════════════════════════════════════════════════

	// Load image once on mount - use untrack to prevent re-running
	let hasLoadedImage = $state(false);

	$effect(() => {
		// Only load once
		if (hasLoadedImage) return;
		hasLoadedImage = true;

		const img = new Image();
		img.crossOrigin = 'anonymous';
		img.onload = () => {
			image = img;
			imageLoaded = true;
			untrack(() => initializeCropArea());
		};
		img.src = src;
	});

	// Calculate display scale - derived from dependencies
	$effect(() => {
		// Read dependencies
		const loaded = imageLoaded;
		const cw = containerWidth;
		const ch = containerHeight;
		const img = image;
		const z = zoom;

		if (loaded && cw && ch && img) {
			const scaleX = cw / img.naturalWidth;
			const scaleY = ch / img.naturalHeight;
			// Use untrack to prevent infinite loop when setting imageScale
			untrack(() => {
				imageScale = Math.min(scaleX, scaleY, 1) * z;
			});
		}
	});

	// Initialize crop area based on aspect ratio
	function initializeCropArea() {
		if (!image) return;
		const ratio = aspectRatios[selectedRatio];
		const imgW = image.naturalWidth;
		const imgH = image.naturalHeight;

		if (ratio === null) {
			// Free aspect ratio - use center 80%
			const margin = 0.1;
			cropArea = {
				x: imgW * margin,
				y: imgH * margin,
				width: imgW * (1 - 2 * margin),
				height: imgH * (1 - 2 * margin)
			};
		} else {
			// Fixed aspect ratio
			const imgRatio = imgW / imgH;
			if (ratio !== null && ratio !== undefined && imgRatio > ratio) {
				// Image is wider - fit height
				const h = imgH * 0.8;
				const w = ratio !== null && ratio !== undefined ? h * ratio : h;
				cropArea = {
					x: (imgW - w) / 2,
					y: imgH * 0.1,
					width: w,
					height: h
				};
			} else {
				// Image is taller - fit width
				const w = imgW * 0.8;
				const h = ratio !== null && ratio !== undefined ? w / ratio : w;
				cropArea = {
					x: imgW * 0.1,
					y: (imgH - h) / 2,
					width: w,
					height: h
				};
			}
		}
	}

	// Handle aspect ratio change
	function handleRatioChange(ratio: string) {
		selectedRatio = ratio;
		if (imageLoaded) {
			initializeCropArea();
		}
	}

	// Handle rotation
	function handleRotate(degrees: number) {
		rotation = (rotation + degrees) % 360;
	}

	// Handle flip
	function handleFlipH() {
		flipH = !flipH;
	}

	function handleFlipV() {
		flipV = !flipV;
	}

	// Handle zoom
	function handleZoom(delta: number) {
		zoom = Math.max(0.5, Math.min(3, zoom + delta));
	}

	// Mouse/touch handlers
	function handleMouseDown(e: MouseEvent | TouchEvent) {
		e.preventDefault();
		const point = 'touches' in e ? e.touches[0] : e;
		if (!point) return;
		startX = point.clientX;
		startY = point.clientY;

		const target = e.target as HTMLElement;
		if (target.classList.contains('resize-handle')) {
			isResizing = true;
			resizeHandle = target.dataset['handle'] || '';
		} else if (target.classList.contains('crop-overlay')) {
			isDragging = true;
		}
	}

	function handleMouseMove(e: MouseEvent | TouchEvent) {
		if (!isDragging && !isResizing) return;

		const point = 'touches' in e ? e.touches[0] : e;
		if (!point) return;
		const deltaX = (point.clientX - startX) / imageScale;
		const deltaY = (point.clientY - startY) / imageScale;

		if (isDragging) {
			moveCropArea(deltaX, deltaY);
		} else if (isResizing) {
			resizeCropArea(deltaX, deltaY);
		}

		startX = point.clientX;
		startY = point.clientY;
	}

	function handleMouseUp() {
		isDragging = false;
		isResizing = false;
		resizeHandle = '';
	}

	// Move crop area
	function moveCropArea(dx: number, dy: number) {
		if (!image) return;
		const maxX = image.naturalWidth - cropArea.width;
		const maxY = image.naturalHeight - cropArea.height;
		cropArea.x = Math.max(0, Math.min(maxX, cropArea.x + dx));
		cropArea.y = Math.max(0, Math.min(maxY, cropArea.y + dy));
		cropArea = cropArea; // Trigger reactivity
	}

	// Resize crop area
	function resizeCropArea(dx: number, dy: number) {
		const ratio = aspectRatios[selectedRatio];
		let newArea = { ...cropArea };

		switch (resizeHandle) {
			case 'nw':
				newArea.x += dx;
				newArea.y += dy;
				newArea.width -= dx;
				newArea.height -= dy;
				break;
			case 'ne':
				newArea.y += dy;
				newArea.width += dx;
				newArea.height -= dy;
				break;
			case 'sw':
				newArea.x += dx;
				newArea.width -= dx;
				newArea.height += dy;
				break;
			case 'se':
				newArea.width += dx;
				newArea.height += dy;
				break;
			case 'n':
				newArea.y += dy;
				newArea.height -= dy;
				break;
			case 's':
				newArea.height += dy;
				break;
			case 'w':
				newArea.x += dx;
				newArea.width -= dx;
				break;
			case 'e':
				newArea.width += dx;
				break;
		}

		// Enforce minimum size
		newArea.width = Math.max(minWidth, newArea.width);
		newArea.height = Math.max(minHeight, newArea.height);

		// Enforce aspect ratio if set
		if (ratio !== null && ratio !== undefined) {
			if (resizeHandle.includes('w') || resizeHandle.includes('e')) {
				newArea.height = newArea.width / ratio;
			} else {
				newArea.width = newArea.height * ratio;
			}
		}

		// Keep within bounds
		if (!image) return;
		newArea.x = Math.max(0, Math.min(image.naturalWidth - newArea.width, newArea.x));
		newArea.y = Math.max(0, Math.min(image.naturalHeight - newArea.height, newArea.y));

		cropArea = newArea;
	}

	// Generate cropped image
	async function handleCrop() {
		if (!canvas || !image) return;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		canvas.width = cropArea.width;
		canvas.height = cropArea.height;

		// Apply transformations
		ctx.save();

		if (rotation !== 0 || flipH || flipV) {
			ctx.translate(canvas.width / 2, canvas.height / 2);
			ctx.rotate((rotation * Math.PI) / 180);
			ctx.scale(flipH ? -1 : 1, flipV ? -1 : 1);
			ctx.translate(-canvas.width / 2, -canvas.height / 2);
		}

		// Draw cropped portion
		ctx.drawImage(
			image,
			cropArea.x,
			cropArea.y,
			cropArea.width,
			cropArea.height,
			0,
			0,
			canvas.width,
			canvas.height
		);

		ctx.restore();

		// Convert to blob
		canvas.toBlob(
			(blob) => {
				if (blob) {
					dispatch('crop', {
						blob,
						cropArea,
						aspectRatio: selectedRatio
					});
				}
			},
			'image/jpeg',
			0.95
		);
	}
</script>

<svelte:window
	onmousemove={handleMouseMove}
	onmouseup={handleMouseUp}
	ontouchmove={handleMouseMove}
	ontouchend={handleMouseUp}
/>

<div
	class="crop-modal-overlay"
	transition:fade
	onclick={() => dispatch('cancel')}
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape') dispatch('cancel');
		if (e.key === 'Enter' || e.key === ' ') dispatch('cancel');
	}}
	role="button"
	tabindex="0"
	aria-label="Close modal"
>
	<div
		class="crop-modal {className}"
		transition:scale
		onclick={(e: MouseEvent) => e.stopPropagation()}
		onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="crop-modal-title"
		tabindex="-1"
	>
		<!-- Header -->
		<div class="modal-header">
			<h2 id="crop-modal-title" class="modal-title">Crop & Edit Image</h2>
			<button class="close-btn" onclick={() => dispatch('cancel')} aria-label="Close modal">
				<svg
					class="icon-md"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					aria-hidden="true"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		</div>

		<!-- Toolbar -->
		<div class="toolbar">
			<!-- Aspect ratio buttons -->
			<div class="toolbar-group">
				<span class="toolbar-label">Aspect Ratio</span>
				<div class="ratio-buttons">
					{#each Object.keys(aspectRatios) as ratio (ratio)}
						<button
							class="ratio-btn"
							data-active={selectedRatio === ratio || undefined}
							onclick={() => handleRatioChange(ratio)}
						>
							{ratio}
						</button>
					{/each}
				</div>
			</div>

			<!-- Transform buttons -->
			<div class="toolbar-group">
				<span class="toolbar-label">Transform</span>
				<div class="transform-buttons">
					<button class="transform-btn" onclick={() => handleRotate(-90)} title="Rotate left">
						<svg class="icon-md" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"
							/>
						</svg>
					</button>
					<button class="transform-btn" onclick={() => handleRotate(90)} title="Rotate right">
						<svg class="icon-md" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M21 10h-10a8 8 0 00-8 8v2M21 10l-6 6m6-6l-6-6"
							/>
						</svg>
					</button>
					<button
						class="transform-btn"
						data-active={flipH || undefined}
						onclick={handleFlipH}
						title="Flip horizontal"
					>
						<svg class="icon-md" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M8 7h12M8 12h12M8 17h12M4 7v10"
							/>
						</svg>
					</button>
					<button
						class="transform-btn"
						data-active={flipV || undefined}
						onclick={handleFlipV}
						title="Flip vertical"
					>
						<svg class="icon-md icon-rotate" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M8 7h12M8 12h12M8 17h12M4 7v10"
							/>
						</svg>
					</button>
				</div>
			</div>

			<!-- Zoom controls -->
			<div class="toolbar-group">
				<span class="toolbar-label">Zoom</span>
				<div class="zoom-controls">
					<button class="zoom-btn" onclick={() => handleZoom(-0.1)}>-</button>
					<span class="zoom-value">{Math.round(zoom * 100)}%</span>
					<button class="zoom-btn" onclick={() => handleZoom(0.1)}>+</button>
				</div>
			</div>
		</div>

		<!-- Canvas area -->
		<div
			class="canvas-container"
			bind:clientWidth={containerWidth}
			bind:clientHeight={containerHeight}
		>
			{#if imageLoaded}
				<div
					class="image-wrapper"
					style="transform: scale({imageScale}); transform-origin: center;"
				>
					<img
						{src}
						alt="Crop preview"
						class="crop-image"
						style="transform: rotate({rotation}deg) scaleX({flipH ? -1 : 1}) scaleY({flipV
							? -1
							: 1});"
					/>

					<!-- Crop overlay -->
					<button
						type="button"
						class="crop-overlay"
						style="
              left: {cropArea.x}px;
              top: {cropArea.y}px;
              width: {cropArea.width}px;
              height: {cropArea.height}px;
            "
						onmousedown={handleMouseDown}
						ontouchstart={handleMouseDown}
						aria-label="Crop area - drag to move or resize"
					>
						<!-- Grid lines -->
						<div class="grid-line grid-line-h1"></div>
						<div class="grid-line grid-line-h2"></div>
						<div class="grid-line grid-line-v1"></div>
						<div class="grid-line grid-line-v2"></div>

						<!-- Resize handles - svelte-ignore directives for interactive drag handles -->
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div
							class="resize-handle resize-nw"
							data-handle="nw"
							onmousedown={handleMouseDown}
						></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div
							class="resize-handle resize-ne"
							data-handle="ne"
							onmousedown={handleMouseDown}
						></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div
							class="resize-handle resize-sw"
							data-handle="sw"
							onmousedown={handleMouseDown}
						></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div
							class="resize-handle resize-se"
							data-handle="se"
							onmousedown={handleMouseDown}
						></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div class="resize-handle resize-n" data-handle="n" onmousedown={handleMouseDown}></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div class="resize-handle resize-s" data-handle="s" onmousedown={handleMouseDown}></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div class="resize-handle resize-w" data-handle="w" onmousedown={handleMouseDown}></div>
						<!-- svelte-ignore a11y_no_static_element_interactions -->
						<div class="resize-handle resize-e" data-handle="e" onmousedown={handleMouseDown}></div>
					</button>

					<!-- Dark overlay outside crop area -->
					<div class="dark-overlay"></div>
				</div>
			{:else}
				<div class="loading-state">
					<div class="loading-spinner"></div>
					<p class="loading-text">Loading image...</p>
				</div>
			{/if}
		</div>

		<!-- Hidden canvas for export -->
		<canvas bind:this={canvas} class="sr-only"></canvas>

		<!-- Footer -->
		<div class="modal-footer">
			<div class="crop-info">
				<span>
					{Math.round(cropArea.width)} x {Math.round(cropArea.height)} px
				</span>
			</div>
			<div class="footer-actions">
				<button class="btn-cancel" onclick={() => dispatch('cancel')}> Cancel </button>
				<button class="btn-crop" onclick={handleCrop}> Apply Crop </button>
			</div>
		</div>
	</div>
</div>

<style>
	/* ─── Modal overlay & shell ─── */
	.crop-modal-overlay {
		position: fixed;
		inset: 0;
		background-color: oklch(0 0 0 / 80%);
		z-index: 50;
		display: flex;
		align-items: flex-end;
		justify-content: center;
		padding: 0;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}

	.crop-modal {
		position: fixed;
		inset: 0;
		background-color: oklch(1 0 0);
		border-radius: 0;
		box-shadow: 0 25px 50px oklch(0 0 0 / 25%);
		inline-size: 100%;
		max-block-size: 100dvh;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		padding-block-start: env(safe-area-inset-top, 0);
		padding-block-end: env(safe-area-inset-bottom, 0);
	}

	.crop-modal::before {
		content: '';
		position: absolute;
		inset-block-start: 8px;
		inset-inline-start: 50%;
		transform: translateX(-50%);
		inline-size: 36px;
		block-size: 4px;
		background: oklch(0 0 0 / 20%);
		border-radius: 2px;
		z-index: 11;
	}

	.modal-header {
		position: sticky;
		inset-block-start: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4);
		padding-block-start: calc(1.25rem + env(safe-area-inset-top, 0));
		border-block-end: 1px solid oklch(0.9 0.005 265);
		background: oklch(1 0 0 / 95%);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.modal-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.icon-md {
		inline-size: 1.25rem;
		block-size: 1.25rem;
	}
	.icon-rotate {
		transform: rotate(90deg);
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-inline-size: 44px;
		min-block-size: 44px;
		inline-size: 44px;
		block-size: 44px;
		padding: 0;
		color: oklch(0.65 0.01 265);
		border-radius: var(--radius-xl);
		transition: all 150ms var(--ease-default);
		background: transparent;
		border: none;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;

		&:hover {
			color: oklch(0.4 0.01 265);
			background-color: oklch(0.96 0.002 265);
		}
		&:focus-visible {
			outline: 2px solid oklch(0.6 0.2 260);
			outline-offset: 2px;
		}
	}

	/* ─── Toolbar ─── */
	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-6);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.98 0.002 265);
		border-block-end: 1px solid oklch(0.9 0.005 265);
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.toolbar-label {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.5 0.01 265);
	}

	.ratio-buttons {
		display: flex;
		gap: 0.25rem;
	}

	.ratio-btn {
		padding-inline: var(--space-2);
		padding-block: 0.25rem;
		font-size: var(--text-xs);
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
		color: oklch(0.4 0.01 265);
		border: none;
		cursor: pointer;
		transition: all 150ms var(--ease-default);

		&:hover {
			background-color: oklch(0.85 0.005 265);
		}
		&[data-active] {
			background-color: oklch(0.6 0.2 260);
			color: oklch(1 0 0);
		}
	}

	.transform-buttons {
		display: flex;
		gap: 0.25rem;
	}

	.transform-btn {
		padding: 0.375rem;
		border-radius: var(--radius-sm);
		color: oklch(0.5 0.01 265);
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 150ms var(--ease-default);

		&:hover {
			background-color: oklch(0.9 0.005 265);
		}
		&[data-active] {
			background-color: oklch(0.92 0.06 260);
			color: oklch(0.6 0.2 260);
		}
	}

	.zoom-controls {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.zoom-btn {
		inline-size: 1.5rem;
		block-size: 1.5rem;
		border-radius: var(--radius-sm);
		background-color: oklch(0.9 0.005 265);
		color: oklch(0.4 0.01 265);
		font-weight: var(--weight-medium);
		border: none;
		cursor: pointer;
		transition: all 150ms var(--ease-default);

		&:hover {
			background-color: oklch(0.85 0.005 265);
		}
	}

	.zoom-value {
		font-size: var(--text-xs);
		color: oklch(0.5 0.01 265);
		inline-size: 3rem;
		text-align: center;
	}

	/* ─── Canvas area ─── */
	.canvas-container {
		flex: 1;
		min-block-size: 300px;
		background-color: oklch(0.1 0.01 265);
		overflow: hidden;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.image-wrapper {
		position: relative;
	}
	.crop-image {
		display: block;
	}

	.crop-overlay {
		all: unset;
		position: absolute;
		border: 2px solid oklch(1 0 0);
		cursor: move;
		box-shadow: 0 0 0 9999px oklch(0 0 0 / 50%);
		display: block;
		box-sizing: border-box;
	}

	.grid-line {
		position: absolute;
		background-color: oklch(1 0 0 / 30%);
	}

	.grid-line-h1,
	.grid-line-h2 {
		inset-inline-start: 0;
		inset-inline-end: 0;
		block-size: 1px;
	}

	.grid-line-h1 {
		inset-block-start: 33.33%;
	}
	.grid-line-h2 {
		inset-block-start: 66.66%;
	}

	.grid-line-v1,
	.grid-line-v2 {
		inset-block-start: 0;
		inset-block-end: 0;
		inline-size: 1px;
	}

	.grid-line-v1 {
		inset-inline-start: 33.33%;
	}
	.grid-line-v2 {
		inset-inline-start: 66.66%;
	}

	.resize-handle {
		position: absolute;
		inline-size: 1rem;
		block-size: 1rem;
		background-color: oklch(1 0 0);
		border-radius: 9999px;
		border: 2px solid oklch(0.6 0.2 260);
		box-shadow: 0 10px 15px oklch(0 0 0 / 10%);
	}

	.resize-nw {
		inset-block-start: -0.5rem;
		inset-inline-start: -0.5rem;
		cursor: nw-resize;
	}
	.resize-ne {
		inset-block-start: -0.5rem;
		inset-inline-end: -0.5rem;
		cursor: ne-resize;
	}
	.resize-sw {
		inset-block-end: -0.5rem;
		inset-inline-start: -0.5rem;
		cursor: sw-resize;
	}
	.resize-se {
		inset-block-end: -0.5rem;
		inset-inline-end: -0.5rem;
		cursor: se-resize;
	}
	.resize-n {
		inset-block-start: -0.5rem;
		inset-inline-start: 50%;
		transform: translateX(-50%);
		cursor: n-resize;
	}
	.resize-s {
		inset-block-end: -0.5rem;
		inset-inline-start: 50%;
		transform: translateX(-50%);
		cursor: s-resize;
	}
	.resize-w {
		inset-block-start: 50%;
		inset-inline-start: -0.5rem;
		transform: translateY(-50%);
		cursor: w-resize;
	}
	.resize-e {
		inset-block-start: 50%;
		inset-inline-end: -0.5rem;
		transform: translateY(-50%);
		cursor: e-resize;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.loading-spinner {
		inline-size: 2rem;
		block-size: 2rem;
		border: 2px solid oklch(0.6 0.2 260);
		border-block-start-color: transparent;
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	.loading-text {
		color: oklch(0.55 0.01 265);
		margin-block-start: var(--space-2);
	}

	.sr-only {
		position: absolute;
		inline-size: 1px;
		block-size: 1px;
		overflow: hidden;
		clip: rect(0 0 0 0);
		white-space: nowrap;
		border: 0;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* ─── Footer ─── */
	.modal-footer {
		position: sticky;
		inset-block-end: 0;
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-4);
		padding-block-end: calc(1rem + env(safe-area-inset-bottom, 0));
		border-block-start: 1px solid oklch(0.9 0.005 265);
		background: oklch(1 0 0 / 95%);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.crop-info {
		font-size: var(--text-sm);
		color: oklch(0.5 0.01 265);
		text-align: center;
	}

	.footer-actions {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		inline-size: 100%;
	}

	.btn-cancel,
	.btn-crop {
		min-block-size: 44px;
		padding-block: var(--space-3);
		padding-inline: var(--space-6);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		border-radius: var(--radius-xl);
		border: none;
		cursor: pointer;
		transition: all 150ms var(--ease-default);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-cancel {
		color: oklch(0.3 0.01 265);
		background-color: oklch(0.96 0.002 265);

		&:hover {
			background-color: oklch(0.9 0.005 265);
		}
		&:focus-visible {
			outline: 2px solid oklch(0.5 0.01 265);
			outline-offset: 2px;
		}
	}

	.btn-crop {
		color: oklch(1 0 0);
		background-color: oklch(0.6 0.2 260);

		&:hover {
			background-color: oklch(0.5 0.22 260);
		}
		&:focus-visible {
			outline: 2px solid oklch(0.6 0.2 260);
			outline-offset: 2px;
		}
	}

	/* ─── Responsive ─── */
	@media (min-width: 640px) {
		.crop-modal-overlay {
			align-items: center;
			padding: var(--space-6);
		}

		.crop-modal {
			position: relative;
			inset: auto;
			max-inline-size: 48rem;
			max-block-size: 85vh;
			border-radius: var(--radius-xl);
			padding-block-start: 0;
			padding-block-end: 0;
		}

		.crop-modal::before {
			display: none;
		}

		.modal-header {
			padding: var(--space-4) var(--space-6);
			padding-block-start: var(--space-4);
			border-start-start-radius: var(--radius-xl);
			border-start-end-radius: var(--radius-xl);
		}

		.modal-footer {
			flex-direction: row;
			align-items: center;
			justify-content: space-between;
			padding: var(--space-4) var(--space-6);
			padding-block-end: var(--space-4);
			border-end-start-radius: var(--radius-xl);
			border-end-end-radius: var(--radius-xl);
		}

		.crop-info {
			text-align: start;
		}
		.footer-actions {
			flex-direction: row;
			inline-size: auto;
		}
	}

	@media (min-width: 768px) {
		.crop-modal {
			max-inline-size: 56rem;
		}
	}

	@media (max-height: 500px) and (orientation: landscape) {
		.crop-modal {
			max-block-size: 100dvh;
		}
		.modal-header {
			padding: var(--space-2) var(--space-4);
		}
		.toolbar {
			padding: var(--space-2) var(--space-4);
		}
		.modal-footer {
			padding: var(--space-2) var(--space-4);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.close-btn,
		.btn-cancel,
		.btn-crop {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.crop-modal {
			border: 2px solid currentColor;
		}
		.btn-cancel,
		.btn-crop {
			border: 2px solid currentColor;
		}
	}
</style>
