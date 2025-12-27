<script lang="ts">
  /**
   * Image Quality Comparison Component
   * ═══════════════════════════════════════════════════════════════════════════
   *
   * Apple HIG-inspired slider comparison tool for:
   * - Before/After image comparison
   * - Quality assessment
   * - Compression artifact detection
   * - Side-by-side and overlay modes
   *
   * @version 2.0.0 - Svelte 5 Runes
   */
  import { onMount } from 'svelte';
  import { spring } from 'svelte/motion';

  // Props - Svelte 5 syntax
  interface Props {
    originalSrc: string;
    optimizedSrc: string;
    originalSize?: number;
    optimizedSize?: number;
    width?: number;
    height?: number;
  }

  let {
    originalSrc,
    optimizedSrc,
    originalSize = 0,
    optimizedSize = 0,
    width: _width = 800,
    height: _height = 600
  }: Props = $props();

  // State - Svelte 5 runes
  let container = $state<HTMLDivElement | null>(null);
  let isDragging = $state(false);
  let mode = $state<'slider' | 'side-by-side' | 'toggle'>('slider');
  let showOriginal = $state(false);
  let zoomLevel = $state(1);
  let panX = $state(0);
  let panY = $state(0);

  // Animated slider position (0-100)
  const sliderPosition = spring(50, { stiffness: 0.3, damping: 0.8 });

  // Computed values - Svelte 5 derived
  let savings = $derived(originalSize > 0 ? ((originalSize - optimizedSize) / originalSize * 100).toFixed(1) : '0');
  let compressionRatio = $derived(optimizedSize > 0 ? (originalSize / optimizedSize).toFixed(1) : '1');

  // ═══════════════════════════════════════════════════════════════════════════
  // Event Handlers
  // ═══════════════════════════════════════════════════════════════════════════

  function handleMouseDown(event: MouseEvent) {
    if (mode !== 'slider') return;
    isDragging = true;
    updateSliderPosition(event);
  }

  function handleMouseMove(event: MouseEvent) {
    if (!isDragging || mode !== 'slider') return;
    updateSliderPosition(event);
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function handleTouchStart(event: TouchEvent) {
    if (mode !== 'slider') return;
    isDragging = true;
    updateSliderPositionFromTouch(event);
  }

  function handleTouchMove(event: TouchEvent) {
    if (!isDragging || mode !== 'slider') return;
    event.preventDefault();
    updateSliderPositionFromTouch(event);
  }

  function updateSliderPosition(event: MouseEvent) {
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const percent = Math.max(0, Math.min(100, (x / rect.width) * 100));
    sliderPosition.set(percent);
  }

  function updateSliderPositionFromTouch(event: TouchEvent) {
    if (!container || !event.touches[0]) return;
    const rect = container.getBoundingClientRect();
    const x = event.touches[0].clientX - rect.left;
    const percent = Math.max(0, Math.min(100, (x / rect.width) * 100));
    sliderPosition.set(percent);
  }

  function handleWheel(event: WheelEvent) {
    if (event.ctrlKey || event.metaKey) {
      event.preventDefault();
      const delta = event.deltaY > 0 ? -0.1 : 0.1;
      zoomLevel = Math.max(1, Math.min(5, zoomLevel + delta));
    }
  }

  function resetZoom() {
    zoomLevel = 1;
    panX = 0;
    panY = 0;
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // Helpers
  // ═══════════════════════════════════════════════════════════════════════════

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1024 / 1024).toFixed(2) + ' MB';
  }

  onMount(() => {
    window.addEventListener('mouseup', handleMouseUp);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('touchend', handleMouseUp);

    return () => {
      window.removeEventListener('mouseup', handleMouseUp);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('touchend', handleMouseUp);
    };
  });
</script>

<div
  class="comparison-container"
  bind:this={container}
  onwheel={handleWheel}
  aria-label="Image comparison tool"
>
  <!-- Mode Selector -->
  <div class="mode-selector">
    <button
      class:active={mode === 'slider'}
      onclick={() => mode = 'slider'}
      title="Slider comparison"
      aria-label="Slider comparison"
    >
      <svg viewBox="0 0 20 20" fill="currentColor">
        <path d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"/>
        <path d="M10 3a1 1 0 011 1v12a1 1 0 11-2 0V4a1 1 0 011-1z"/>
      </svg>
    </button>
    <button
      class:active={mode === 'side-by-side'}
      onclick={() => mode = 'side-by-side'}
      title="Side by side"
      aria-label="Side by side comparison"
    >
      <svg viewBox="0 0 20 20" fill="currentColor">
        <path d="M2 4a1 1 0 011-1h6a1 1 0 011 1v12a1 1 0 01-1 1H3a1 1 0 01-1-1V4zM12 4a1 1 0 011-1h4a1 1 0 011 1v12a1 1 0 01-1 1h-4a1 1 0 01-1-1V4z"/>
      </svg>
    </button>
    <button
      class:active={mode === 'toggle'}
      onclick={() => mode = 'toggle'}
      title="Toggle view"
      aria-label="Toggle view"
    >
      <svg viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
      </svg>
    </button>
  </div>

  <!-- Zoom Controls -->
  <div class="zoom-controls">
    <button onclick={() => zoomLevel = Math.max(1, zoomLevel - 0.5)} disabled={zoomLevel <= 1} aria-label="Zoom out">
      <svg viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"/>
      </svg>
    </button>
    <span>{Math.round(zoomLevel * 100)}%</span>
    <button onclick={() => zoomLevel = Math.min(5, zoomLevel + 0.5)} disabled={zoomLevel >= 5} aria-label="Zoom in">
      <svg viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd"/>
      </svg>
    </button>
    {#if zoomLevel > 1}
      <button onclick={resetZoom} class="reset-btn">Reset</button>
    {/if}
  </div>

  <!-- Comparison View -->
  <div
    class="comparison-view"
    class:slider-mode={mode === 'slider'}
    class:side-by-side-mode={mode === 'side-by-side'}
    class:toggle-mode={mode === 'toggle'}
    style="--zoom: {zoomLevel}; --pan-x: {panX}px; --pan-y: {panY}px;"
  >
    {#if mode === 'slider'}
      <!-- Slider Mode -->
      <div class="slider-wrapper" onmousedown={handleMouseDown} ontouchstart={handleTouchStart} ontouchmove={handleTouchMove} role="slider" aria-label="Image comparison slider" aria-valuenow={$sliderPosition} aria-valuemin="0" aria-valuemax="100" tabindex="0">
        <!-- Optimized (Background) -->
        <div class="image-layer optimized">
          <img src={optimizedSrc} alt="Optimized version" draggable="false" />
          <span class="image-label">Optimized</span>
        </div>

        <!-- Original (Clipped) -->
        <div class="image-layer original" style="clip-path: inset(0 {100 - $sliderPosition}% 0 0);">
          <img src={originalSrc} alt="Original version" draggable="false" />
          <span class="image-label">Original</span>
        </div>

        <!-- Slider Handle -->
        <div class="slider-handle" style="left: {$sliderPosition}%;">
          <div class="handle-line"></div>
          <div class="handle-grip">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M8 12h8M12 8v8"/>
            </svg>
          </div>
        </div>
      </div>

    {:else if mode === 'side-by-side'}
      <!-- Side by Side Mode -->
      <div class="side-by-side-wrapper">
        <div class="side-panel">
          <img src={originalSrc} alt="Original version" draggable="false" />
          <span class="image-label">Original</span>
          <span class="size-badge">{formatBytes(originalSize)}</span>
        </div>
        <div class="side-panel">
          <img src={optimizedSrc} alt="Optimized version" draggable="false" />
          <span class="image-label">Optimized</span>
          <span class="size-badge">{formatBytes(optimizedSize)}</span>
        </div>
      </div>

    {:else}
      <!-- Toggle Mode -->
      <div class="toggle-wrapper" onclick={() => showOriginal = !showOriginal} onkeydown={(e: KeyboardEvent) => e.key === ' ' && (showOriginal = !showOriginal, e.preventDefault())} role="button" tabindex="0">
        <div class="image-layer" class:visible={!showOriginal}>
          <img src={optimizedSrc} alt="Optimized version" draggable="false" />
          <span class="image-label">Optimized</span>
        </div>
        <div class="image-layer" class:visible={showOriginal}>
          <img src={originalSrc} alt="Original version" draggable="false" />
          <span class="image-label">Original</span>
        </div>
        <div class="toggle-hint">
          <span>Click or press Space to toggle</span>
        </div>
      </div>
    {/if}
  </div>

  <!-- Stats Bar -->
  <div class="stats-bar">
    <div class="stat">
      <span class="stat-label">Original</span>
      <span class="stat-value">{formatBytes(originalSize)}</span>
    </div>
    <div class="stat">
      <span class="stat-label">Optimized</span>
      <span class="stat-value">{formatBytes(optimizedSize)}</span>
    </div>
    <div class="stat highlight">
      <span class="stat-label">Savings</span>
      <span class="stat-value green">{savings}%</span>
    </div>
    <div class="stat">
      <span class="stat-label">Compression</span>
      <span class="stat-value">{compressionRatio}x</span>
    </div>
  </div>

  <!-- Instructions -->
  <div class="instructions">
    {#if mode === 'slider'}
      <span>Drag the slider to compare</span>
    {:else if mode === 'side-by-side'}
      <span>Compare images side by side</span>
    {:else}
      <span>Click to toggle between versions</span>
    {/if}
    <span class="hint">Ctrl+Scroll to zoom</span>
  </div>
</div>

<style>
  .comparison-container {
    position: relative;
    width: 100%;
    max-width: 100%;
    background: #1d1d1f;
    border-radius: 16px;
    overflow: hidden;
    outline: none;
  }

  .comparison-container:focus {
    box-shadow: 0 0 0 4px rgba(0, 113, 227, 0.3);
  }

  /* Mode Selector */
  .mode-selector {
    position: absolute;
    top: 16px;
    left: 16px;
    display: flex;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 10px;
    padding: 4px;
    z-index: 20;
  }

  .mode-selector button {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    transition: all 0.2s;
  }

  .mode-selector button:hover {
    color: white;
    background: rgba(255, 255, 255, 0.1);
  }

  .mode-selector button.active {
    background: rgba(255, 255, 255, 0.2);
    color: white;
  }

  .mode-selector button svg {
    width: 18px;
    height: 18px;
  }

  /* Zoom Controls */
  .zoom-controls {
    position: absolute;
    top: 16px;
    right: 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 10px;
    padding: 4px 12px;
    z-index: 20;
  }

  .zoom-controls button {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.7);
    cursor: pointer;
    transition: all 0.2s;
  }

  .zoom-controls button:hover:not(:disabled) {
    color: white;
    background: rgba(255, 255, 255, 0.1);
  }

  .zoom-controls button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .zoom-controls button svg {
    width: 16px;
    height: 16px;
  }

  .zoom-controls span {
    font-size: 12px;
    font-weight: 500;
    color: white;
    min-width: 40px;
    text-align: center;
  }

  .zoom-controls .reset-btn {
    width: auto;
    padding: 0 10px;
    font-size: 11px;
    font-weight: 500;
  }

  /* Comparison View */
  .comparison-view {
    position: relative;
    width: 100%;
    aspect-ratio: 4/3;
    overflow: hidden;
  }

  /* Slider Mode */
  .slider-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    cursor: ew-resize;
    user-select: none;
  }

  .image-layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    transform: scale(var(--zoom, 1)) translate(var(--pan-x, 0), var(--pan-y, 0));
    transform-origin: center;
  }

  .image-layer img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    pointer-events: none;
  }

  .image-label {
    position: absolute;
    bottom: 16px;
    left: 16px;
    padding: 6px 12px;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    color: white;
  }

  .original .image-label {
    background: rgba(239, 68, 68, 0.8);
  }

  .optimized .image-label {
    background: rgba(16, 185, 129, 0.8);
  }

  /* Slider Handle */
  .slider-handle {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 4px;
    margin-left: -2px;
    z-index: 10;
  }

  .handle-line {
    position: absolute;
    inset: 0;
    background: white;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
  }

  .handle-grip {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 44px;
    height: 44px;
    background: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  }

  .handle-grip svg {
    width: 20px;
    height: 20px;
    color: #1d1d1f;
  }

  /* Side by Side Mode */
  .side-by-side-wrapper {
    display: flex;
    width: 100%;
    height: 100%;
    gap: 2px;
  }

  .side-panel {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #111;
    overflow: hidden;
  }

  .side-panel img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }

  .size-badge {
    position: absolute;
    top: 16px;
    right: 16px;
    padding: 6px 12px;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    color: white;
  }

  /* Toggle Mode */
  .toggle-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  .toggle-wrapper .image-layer {
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .toggle-wrapper .image-layer.visible {
    opacity: 1;
  }

  .toggle-hint {
    position: absolute;
    bottom: 16px;
    left: 50%;
    transform: translateX(-50%);
    padding: 8px 16px;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 8px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.8);
  }

  /* Stats Bar */
  .stats-bar {
    display: flex;
    justify-content: center;
    gap: 32px;
    padding: 16px 24px;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .stat-label {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stat-value {
    font-size: 16px;
    font-weight: 600;
    color: white;
  }

  .stat-value.green {
    color: #34d399;
  }

  .stat.highlight {
    padding: 8px 20px;
    background: rgba(16, 185, 129, 0.2);
    border-radius: 8px;
  }

  /* Instructions */
  .instructions {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 24px;
    padding: 12px;
    background: #111;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.5);
  }

  .instructions .hint {
    padding: 4px 10px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .mode-selector,
    .zoom-controls {
      padding: 2px;
    }

    .mode-selector button,
    .zoom-controls button {
      width: 32px;
      height: 32px;
    }

    .stats-bar {
      gap: 16px;
      padding: 12px 16px;
      flex-wrap: wrap;
    }

    .stat-value {
      font-size: 14px;
    }

    .side-by-side-wrapper {
      flex-direction: column;
    }
  }
</style>
