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
      'Free': null,
      '1:1': 1,
      '16:9': 16 / 9,
      '4:3': 4 / 3,
      '3:2': 3 / 2,
      '9:16': 9 / 16,
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
  let hasLoadedImage = false;
  
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

</script>

<svelte:window
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
  ontouchmove={handleMouseMove}
  ontouchend={handleMouseUp}
/>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_click_events_have_key_events -->
<div 
  class="crop-modal-overlay" 
  transition:fade 
  onclick={() => dispatch('cancel')}
  onkeydown={(e) => e.key === 'Escape' && dispatch('cancel')}
  role="dialog"
  aria-modal="true"
  aria-labelledby="crop-modal-title"
  tabindex="-1"
>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="crop-modal {className}" transition:scale onclick={(e) => e.stopPropagation()} role="document">
    <!-- Header -->
    <div class="modal-header">
      <h2 id="crop-modal-title" class="text-lg font-semibold text-gray-900 dark:text-white">Crop & Edit Image</h2>
      <button class="close-btn" onclick={() => dispatch('cancel')} aria-label="Close modal">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
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
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
            </svg>
          </button>
          <button class="transform-btn" onclick={() => handleRotate(90)} title="Rotate right">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 10h-10a8 8 0 00-8 8v2M21 10l-6 6m6-6l-6-6" />
            </svg>
          </button>
          <button class="transform-btn" class:active={flipH} onclick={handleFlipH} title="Flip horizontal">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12M8 12h12M8 17h12M4 7v10" />
            </svg>
          </button>
          <button class="transform-btn" class:active={flipV} onclick={handleFlipV} title="Flip vertical">
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
            src={src}
            alt="Crop preview"
            class="crop-image"
            style="transform: rotate({rotation}deg) scaleX({flipH ? -1 : 1}) scaleY({flipV ? -1 : 1});"
          />

          <!-- Crop overlay -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="crop-overlay"
            style="
              left: {cropArea.x}px;
              top: {cropArea.y}px;
              width: {cropArea.width}px;
              height: {cropArea.height}px;
            "
            onmousedown={handleMouseDown}
            ontouchstart={handleMouseDown}
            role="application"
            aria-label="Crop area - drag to move or resize"
          >
            <!-- Grid lines -->
            <div class="grid-line grid-line-h1"></div>
            <div class="grid-line grid-line-h2"></div>
            <div class="grid-line grid-line-v1"></div>
            <div class="grid-line grid-line-v2"></div>

            <!-- Resize handles - svelte-ignore directives for interactive drag handles -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-nw" data-handle="nw" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-ne" data-handle="ne" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-sw" data-handle="sw" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-se" data-handle="se" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-n" data-handle="n" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-s" data-handle="s" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-w" data-handle="w" onmousedown={handleMouseDown}></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="resize-handle resize-e" data-handle="e" onmousedown={handleMouseDown}></div>
          </div>

          <!-- Dark overlay outside crop area -->
          <div class="dark-overlay"></div>
        </div>
      {:else}
        <div class="loading-state">
          <div class="animate-spin w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full"></div>
          <p class="text-gray-500 mt-2">Loading image...</p>
        </div>
      {/if}
    </div>

    <!-- Hidden canvas for export -->
    <canvas bind:this={canvas} class="hidden"></canvas>

    <!-- Footer -->
    <div class="modal-footer">
      <div class="crop-info">
        <span class="text-sm text-gray-500">
          {Math.round(cropArea.width)} x {Math.round(cropArea.height)} px
        </span>
      </div>
      <div class="footer-actions">
        <button class="btn-cancel" onclick={() => dispatch('cancel')}>
          Cancel
        </button>
        <button class="btn-crop" onclick={handleCrop}>
          Apply Crop
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .crop-modal-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.7);
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
  }

  .crop-modal {
    background-color: white;
    border-radius: 0.75rem;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
    width: 100%;
    max-width: 56rem;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
  }

  :global(.dark) .crop-modal {
    background-color: #1f2937;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }

  :global(.dark) .modal-header {
    border-color: #374151;
  }

  .close-btn {
    padding: 0.5rem;
    color: #9ca3af;
    border-radius: 0.5rem;
    transition: all 150ms ease;
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .close-btn:hover {
    color: #4b5563;
    background-color: #f3f4f6;
  }

  :global(.dark) .close-btn:hover {
    color: #d1d5db;
    background-color: #374151;
  }

  .toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1.5rem;
    padding: 0.75rem 1.5rem;
    background-color: #f9fafb;
    border-bottom: 1px solid #e5e7eb;
  }

  :global(.dark) .toolbar {
    background-color: rgba(17, 24, 39, 0.5);
    border-color: #374151;
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .toolbar-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
  }

  :global(.dark) .toolbar-label {
    color: #9ca3af;
  }

  .ratio-buttons {
    display: flex;
    gap: 0.25rem;
  }

  .ratio-btn {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border-radius: 0.25rem;
    background-color: #e5e7eb;
    color: #4b5563;
    border: none;
    cursor: pointer;
    transition: all 150ms ease;
  }

  :global(.dark) .ratio-btn {
    background-color: #374151;
    color: #9ca3af;
  }

  .ratio-btn:hover {
    background-color: #d1d5db;
  }

  :global(.dark) .ratio-btn:hover {
    background-color: #4b5563;
  }

  .ratio-btn.active {
    background-color: #3b82f6;
    color: white;
  }

  .transform-buttons {
    display: flex;
    gap: 0.25rem;
  }

  .transform-btn {
    padding: 0.375rem;
    border-radius: 0.25rem;
    color: #6b7280;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 150ms ease;
  }

  :global(.dark) .transform-btn {
    color: #9ca3af;
  }

  .transform-btn:hover {
    background-color: #e5e7eb;
  }

  :global(.dark) .transform-btn:hover {
    background-color: #374151;
  }

  .transform-btn.active {
    background-color: #dbeafe;
    color: #3b82f6;
  }

  :global(.dark) .transform-btn.active {
    background-color: rgba(30, 58, 138, 0.3);
  }

  .zoom-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .zoom-btn {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 0.25rem;
    background-color: #e5e7eb;
    color: #4b5563;
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: all 150ms ease;
  }

  :global(.dark) .zoom-btn {
    background-color: #374151;
    color: #9ca3af;
  }

  .zoom-btn:hover {
    background-color: #d1d5db;
  }

  :global(.dark) .zoom-btn:hover {
    background-color: #4b5563;
  }

  .zoom-value {
    font-size: 0.75rem;
    color: #6b7280;
    width: 3rem;
    text-align: center;
  }

  .canvas-container {
    flex: 1;
    min-height: 300px;
    background-color: #111827;
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
    position: absolute;
    border: 2px solid white;
    cursor: move;
    box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);
  }

  .grid-line {
    position: absolute;
    background-color: rgba(255, 255, 255, 0.3);
  }

  .grid-line-h1, .grid-line-h2 {
    left: 0;
    right: 0;
    height: 1px;
  }

  .grid-line-h1 { top: 33.33%; }
  .grid-line-h2 { top: 66.66%; }

  .grid-line-v1, .grid-line-v2 {
    top: 0;
    bottom: 0;
    width: 1px;
  }

  .grid-line-v1 { left: 33.33%; }
  .grid-line-v2 { left: 66.66%; }

  .resize-handle {
    position: absolute;
    width: 1rem;
    height: 1rem;
    background-color: white;
    border-radius: 9999px;
    border: 2px solid #3b82f6;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
  }

  .resize-nw { top: -0.5rem; left: -0.5rem; cursor: nw-resize; }
  .resize-ne { top: -0.5rem; right: -0.5rem; cursor: ne-resize; }
  .resize-sw { bottom: -0.5rem; left: -0.5rem; cursor: sw-resize; }
  .resize-se { bottom: -0.5rem; right: -0.5rem; cursor: se-resize; }
  .resize-n { top: -0.5rem; left: 50%; transform: translateX(-50%); cursor: n-resize; }
  .resize-s { bottom: -0.5rem; left: 50%; transform: translateX(-50%); cursor: s-resize; }
  .resize-w { top: 50%; left: -0.5rem; transform: translateY(-50%); cursor: w-resize; }
  .resize-e { top: 50%; right: -0.5rem; transform: translateY(-50%); cursor: e-resize; }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-top: 1px solid #e5e7eb;
  }

  :global(.dark) .modal-footer {
    border-color: #374151;
  }

  .crop-info {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .footer-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn-cancel {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    background-color: #f3f4f6;
    border-radius: 0.5rem;
    border: none;
    cursor: pointer;
    transition: all 150ms ease;
  }

  :global(.dark) .btn-cancel {
    color: #d1d5db;
    background-color: #374151;
  }

  .btn-cancel:hover {
    background-color: #e5e7eb;
  }

  :global(.dark) .btn-cancel:hover {
    background-color: #4b5563;
  }

  .btn-crop {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: white;
    background-color: #3b82f6;
    border-radius: 0.5rem;
    border: none;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .btn-crop:hover {
    background-color: #2563eb;
  }
</style>
