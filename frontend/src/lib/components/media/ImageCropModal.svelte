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
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import { fade, scale } from 'svelte/transition';

  interface CropArea {
    x: number;
    y: number;
    width: number;
    height: number;
  }

  // Props
  export let src: string;
  export let aspectRatios: Record<string, number | null> = {
    'Free': null,
    '1:1': 1,
    '16:9': 16 / 9,
    '4:3': 4 / 3,
    '3:2': 3 / 2,
    '9:16': 9 / 16,
  };
  export let initialAspectRatio: string = 'Free';
  export let minWidth: number = 50;
  export let minHeight: number = 50;
  export let className: string = '';

  const dispatch = createEventDispatcher<{
    crop: { blob: Blob; cropArea: CropArea; aspectRatio: string };
    cancel: void;
  }>();

  // State
  let canvas: HTMLCanvasElement;
  let image: HTMLImageElement;
  let imageLoaded = false;
  let selectedRatio = initialAspectRatio;
  let rotation = 0;
  let flipH = false;
  let flipV = false;
  let zoom = 1;
  let isDragging = false;
  let isResizing = false;
  let resizeHandle = '';
  let startX = 0;
  let startY = 0;

  // Crop area (in image coordinates)
  let cropArea: CropArea = { x: 0, y: 0, width: 100, height: 100 };

  // Container dimensions
  let containerWidth = 0;
  let containerHeight = 0;
  let scale = 1;

  // Load image
  onMount(() => {
    image = new Image();
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      imageLoaded = true;
      initializeCropArea();
    };
    image.src = src;
  });

  // Initialize crop area based on aspect ratio
  function initializeCropArea() {
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
        height: imgH * (1 - 2 * margin),
      };
    } else {
      // Fixed aspect ratio
      const imgRatio = imgW / imgH;
      if (imgRatio > ratio) {
        // Image is wider - fit height
        const h = imgH * 0.8;
        const w = h * ratio;
        cropArea = {
          x: (imgW - w) / 2,
          y: imgH * 0.1,
          width: w,
          height: h,
        };
      } else {
        // Image is taller - fit width
        const w = imgW * 0.8;
        const h = w / ratio;
        cropArea = {
          x: imgW * 0.1,
          y: (imgH - h) / 2,
          width: w,
          height: h,
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
    startX = point.clientX;
    startY = point.clientY;

    const target = e.target as HTMLElement;
    if (target.classList.contains('resize-handle')) {
      isResizing = true;
      resizeHandle = target.dataset.handle || '';
    } else if (target.classList.contains('crop-overlay')) {
      isDragging = true;
    }
  }

  function handleMouseMove(e: MouseEvent | TouchEvent) {
    if (!isDragging && !isResizing) return;

    const point = 'touches' in e ? e.touches[0] : e;
    const deltaX = (point.clientX - startX) / scale;
    const deltaY = (point.clientY - startY) / scale;

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
    if (ratio !== null) {
      if (resizeHandle.includes('w') || resizeHandle.includes('e')) {
        newArea.height = newArea.width / ratio;
      } else {
        newArea.width = newArea.height * ratio;
      }
    }

    // Keep within bounds
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
            aspectRatio: selectedRatio,
          });
        }
      },
      'image/jpeg',
      0.95
    );
  }

  // Calculate display scale
  $: if (imageLoaded && containerWidth && containerHeight) {
    const scaleX = containerWidth / image.naturalWidth;
    const scaleY = containerHeight / image.naturalHeight;
    scale = Math.min(scaleX, scaleY, 1) * zoom;
  }
</script>

<svelte:window
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:touchmove={handleMouseMove}
  on:touchend={handleMouseUp}
/>

<div class="crop-modal-overlay" transition:fade on:click={() => dispatch('cancel')}>
  <div class="crop-modal {className}" transition:scale on:click|stopPropagation>
    <!-- Header -->
    <div class="modal-header">
      <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Crop & Edit Image</h2>
      <button class="close-btn" on:click={() => dispatch('cancel')}>
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Toolbar -->
    <div class="toolbar">
      <!-- Aspect ratio buttons -->
      <div class="toolbar-group">
        <span class="toolbar-label">Aspect Ratio</span>
        <div class="ratio-buttons">
          {#each Object.keys(aspectRatios) as ratio}
            <button
              class="ratio-btn"
              class:active={selectedRatio === ratio}
              on:click={() => handleRatioChange(ratio)}
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
          <button class="transform-btn" on:click={() => handleRotate(-90)} title="Rotate left">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
            </svg>
          </button>
          <button class="transform-btn" on:click={() => handleRotate(90)} title="Rotate right">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 10h-10a8 8 0 00-8 8v2M21 10l-6 6m6-6l-6-6" />
            </svg>
          </button>
          <button class="transform-btn" class:active={flipH} on:click={handleFlipH} title="Flip horizontal">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12M8 12h12M8 17h12M4 7v10" />
            </svg>
          </button>
          <button class="transform-btn" class:active={flipV} on:click={handleFlipV} title="Flip vertical">
            <svg class="w-5 h-5 rotate-90" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12M8 12h12M8 17h12M4 7v10" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Zoom controls -->
      <div class="toolbar-group">
        <span class="toolbar-label">Zoom</span>
        <div class="zoom-controls">
          <button class="zoom-btn" on:click={() => handleZoom(-0.1)}>-</button>
          <span class="zoom-value">{Math.round(zoom * 100)}%</span>
          <button class="zoom-btn" on:click={() => handleZoom(0.1)}>+</button>
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
          style="transform: scale({scale}); transform-origin: center;"
        >
          <img
            src={src}
            alt="Crop preview"
            class="crop-image"
            style="transform: rotate({rotation}deg) scaleX({flipH ? -1 : 1}) scaleY({flipV ? -1 : 1});"
          />

          <!-- Crop overlay -->
          <div
            class="crop-overlay"
            style="
              left: {cropArea.x}px;
              top: {cropArea.y}px;
              width: {cropArea.width}px;
              height: {cropArea.height}px;
            "
            on:mousedown={handleMouseDown}
            on:touchstart={handleMouseDown}
          >
            <!-- Grid lines -->
            <div class="grid-line grid-line-h1" />
            <div class="grid-line grid-line-h2" />
            <div class="grid-line grid-line-v1" />
            <div class="grid-line grid-line-v2" />

            <!-- Resize handles -->
            <div class="resize-handle resize-nw" data-handle="nw" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-ne" data-handle="ne" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-sw" data-handle="sw" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-se" data-handle="se" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-n" data-handle="n" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-s" data-handle="s" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-w" data-handle="w" on:mousedown={handleMouseDown} />
            <div class="resize-handle resize-e" data-handle="e" on:mousedown={handleMouseDown} />
          </div>

          <!-- Dark overlay outside crop area -->
          <div class="dark-overlay" />
        </div>
      {:else}
        <div class="loading-state">
          <div class="animate-spin w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full" />
          <p class="text-gray-500 mt-2">Loading image...</p>
        </div>
      {/if}
    </div>

    <!-- Hidden canvas for export -->
    <canvas bind:this={canvas} class="hidden" />

    <!-- Footer -->
    <div class="modal-footer">
      <div class="crop-info">
        <span class="text-sm text-gray-500">
          {Math.round(cropArea.width)} x {Math.round(cropArea.height)} px
        </span>
      </div>
      <div class="footer-actions">
        <button class="btn-cancel" on:click={() => dispatch('cancel')}>
          Cancel
        </button>
        <button class="btn-crop" on:click={handleCrop}>
          Apply Crop
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .crop-modal-overlay {
    @apply fixed inset-0 bg-black/70 z-50 flex items-center justify-center p-4;
  }

  .crop-modal {
    @apply bg-white dark:bg-gray-800 rounded-xl shadow-2xl;
    @apply w-full max-w-4xl max-h-[90vh] flex flex-col;
  }

  .modal-header {
    @apply flex items-center justify-between px-6 py-4;
    @apply border-b border-gray-200 dark:border-gray-700;
  }

  .close-btn {
    @apply p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300;
    @apply hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors;
  }

  .toolbar {
    @apply flex flex-wrap items-center gap-6 px-6 py-3;
    @apply bg-gray-50 dark:bg-gray-900/50 border-b border-gray-200 dark:border-gray-700;
  }

  .toolbar-group {
    @apply flex items-center gap-2;
  }

  .toolbar-label {
    @apply text-xs font-medium text-gray-500 dark:text-gray-400;
  }

  .ratio-buttons {
    @apply flex gap-1;
  }

  .ratio-btn {
    @apply px-2 py-1 text-xs rounded;
    @apply bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400;
    @apply hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors;
  }

  .ratio-btn.active {
    @apply bg-blue-500 text-white;
  }

  .transform-buttons {
    @apply flex gap-1;
  }

  .transform-btn {
    @apply p-1.5 rounded text-gray-500 dark:text-gray-400;
    @apply hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors;
  }

  .transform-btn.active {
    @apply bg-blue-100 dark:bg-blue-900/30 text-blue-500;
  }

  .zoom-controls {
    @apply flex items-center gap-2;
  }

  .zoom-btn {
    @apply w-6 h-6 rounded bg-gray-200 dark:bg-gray-700;
    @apply text-gray-600 dark:text-gray-400 font-medium;
    @apply hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors;
  }

  .zoom-value {
    @apply text-xs text-gray-500 w-12 text-center;
  }

  .canvas-container {
    @apply flex-1 min-h-[300px] bg-gray-900 overflow-hidden;
    @apply flex items-center justify-center relative;
  }

  .image-wrapper {
    @apply relative;
  }

  .crop-image {
    @apply block;
  }

  .crop-overlay {
    @apply absolute border-2 border-white cursor-move;
    box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);
  }

  .grid-line {
    @apply absolute bg-white/30;
  }

  .grid-line-h1, .grid-line-h2 {
    @apply left-0 right-0 h-px;
  }

  .grid-line-h1 { top: 33.33%; }
  .grid-line-h2 { top: 66.66%; }

  .grid-line-v1, .grid-line-v2 {
    @apply top-0 bottom-0 w-px;
  }

  .grid-line-v1 { left: 33.33%; }
  .grid-line-v2 { left: 66.66%; }

  .resize-handle {
    @apply absolute w-4 h-4 bg-white rounded-full border-2 border-blue-500;
    @apply shadow-lg;
  }

  .resize-nw { @apply -top-2 -left-2 cursor-nw-resize; }
  .resize-ne { @apply -top-2 -right-2 cursor-ne-resize; }
  .resize-sw { @apply -bottom-2 -left-2 cursor-sw-resize; }
  .resize-se { @apply -bottom-2 -right-2 cursor-se-resize; }
  .resize-n { @apply -top-2 left-1/2 -translate-x-1/2 cursor-n-resize; }
  .resize-s { @apply -bottom-2 left-1/2 -translate-x-1/2 cursor-s-resize; }
  .resize-w { @apply top-1/2 -left-2 -translate-y-1/2 cursor-w-resize; }
  .resize-e { @apply top-1/2 -right-2 -translate-y-1/2 cursor-e-resize; }

  .loading-state {
    @apply flex flex-col items-center justify-center;
  }

  .modal-footer {
    @apply flex items-center justify-between px-6 py-4;
    @apply border-t border-gray-200 dark:border-gray-700;
  }

  .crop-info {
    @apply text-sm text-gray-500;
  }

  .footer-actions {
    @apply flex gap-3;
  }

  .btn-cancel {
    @apply px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300;
    @apply bg-gray-100 dark:bg-gray-700 rounded-lg;
    @apply hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors;
  }

  .btn-crop {
    @apply px-4 py-2 text-sm font-medium text-white;
    @apply bg-blue-500 rounded-lg;
    @apply hover:bg-blue-600 transition-colors;
  }
</style>
