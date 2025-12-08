<!--
  ResponsivePreview Component
  ═══════════════════════════════════════════════════════════════════════════

  Preview image at different responsive breakpoints:
  - Shows all 6 responsive sizes side by side
  - File size comparison for each variant
  - Interactive selection
  - Zoom preview on hover

  @version 2.0.0
-->
<script lang="ts">
  import { fade, scale } from 'svelte/transition';

  interface Variant {
    sizeName: string;
    width: number;
    height: number;
    url: string;
    size: number;
  }

  interface Props {
    variants?: Variant[];
    originalUrl?: string;
    originalSize?: number;
    selectedSize?: string | null;
    showSizes?: boolean;
    interactive?: boolean;
    className?: string;
    onSelect?: (sizeName: string) => void;
  }

  let {
    variants = [],
    originalUrl = '',
    originalSize = 0,
    selectedSize = $bindable(null),
    showSizes = true,
    interactive = true,
    className = '',
    onSelect,
  }: Props = $props();

  // Breakpoint labels
  const breakpointLabels: Record<string, string> = {
    xs: 'Mobile S',
    sm: 'Mobile L',
    md: 'Tablet',
    lg: 'Laptop',
    xl: 'Desktop',
    '2xl': 'Large',
  };

  // Breakpoint widths
  const breakpointWidths: Record<string, number> = {
    xs: 320,
    sm: 640,
    md: 768,
    lg: 1024,
    xl: 1280,
    '2xl': 1920,
  };

  // State
  let hoveredVariant = $state<Variant | null>(null);
  let previewPosition = $state({ x: 0, y: 0 });

  // Derived state
  const sortedVariants = $derived([...variants].sort((a, b) => a.width - b.width));

  // Format bytes
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  // Get savings percentage
  function getSavings(variantSize: number): number {
    if (!originalSize || originalSize === 0) return 0;
    return Math.round((1 - variantSize / originalSize) * 100);
  }

  // Handle variant selection
  function handleSelect(sizeName: string) {
    if (!interactive) return;
    selectedSize = sizeName;
    onSelect?.(sizeName);
  }

  // Handle hover preview
  function handleMouseEnter(e: MouseEvent, variant: Variant) {
    if (!interactive) return;
    hoveredVariant = variant;
    updatePreviewPosition(e);
  }

  function handleMouseMove(e: MouseEvent) {
    if (hoveredVariant) {
      updatePreviewPosition(e);
    }
  }

  function handleMouseLeave() {
    hoveredVariant = null;
  }

  function updatePreviewPosition(e: MouseEvent) {
    const rect = (e.target as HTMLElement).getBoundingClientRect();
    previewPosition = {
      x: rect.right + 10,
      y: rect.top,
    };
  }
</script>

<div class="responsive-preview {className}">
  <!-- Header -->
  <div class="preview-header">
    <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300">
      Responsive Variants
    </h3>
    {#if originalSize > 0}
      <span class="text-xs text-gray-500">
        Original: {formatBytes(originalSize)}
      </span>
    {/if}
  </div>

  <!-- Variants grid -->
  <div class="variants-grid">
    {#each sortedVariants as variant (variant.sizeName)}
      <button
        class="variant-card"
        class:selected={selectedSize === variant.sizeName}
        class:interactive={interactive}
        onclick={() => handleSelect(variant.sizeName)}
        onmouseenter={(e) => handleMouseEnter(e, variant)}
        onmousemove={handleMouseMove}
        onmouseleave={handleMouseLeave}
        transition:scale={{ duration: 200 }}
      >
        <!-- Thumbnail -->
        <div class="variant-thumbnail">
          <img
            src={variant.url}
            alt="{variant.sizeName} preview"
            loading="lazy"
          />
        </div>

        <!-- Info -->
        <div class="variant-info">
          <div class="size-name">
            {breakpointLabels[variant.sizeName] || variant.sizeName}
          </div>
          <div class="size-dimensions">
            {variant.width} x {variant.height}
          </div>
          {#if showSizes}
            <div class="size-details">
              <span class="file-size">{formatBytes(variant.size)}</span>
              {#if originalSize > 0}
                <span class="savings">-{getSavings(variant.size)}%</span>
              {/if}
            </div>
          {/if}
        </div>

        <!-- Breakpoint indicator -->
        <div class="breakpoint-indicator">
          <div
            class="indicator-bar"
            style="width: {(breakpointWidths[variant.sizeName] / 1920) * 100}%"
          ></div>
        </div>
      </button>
    {/each}
  </div>

  <!-- Size comparison chart -->
  {#if showSizes && sortedVariants.length > 0}
    <div class="size-chart">
      <div class="chart-header">
        <span class="text-xs text-gray-500">Size Comparison</span>
      </div>
      <div class="chart-bars">
        {#each sortedVariants as variant}
          <div class="chart-bar-wrapper">
            <div
              class="chart-bar"
              style="height: {(variant.size / originalSize) * 100}%"
            ></div>
            <span class="chart-label">{variant.sizeName}</span>
          </div>
        {/each}
        <div class="chart-bar-wrapper original">
          <div class="chart-bar chart-bar-original" style="height: 100%"></div>
          <span class="chart-label">orig</span>
        </div>
      </div>
    </div>
  {/if}

  <!-- Hover preview tooltip -->
  {#if hoveredVariant}
    <div
      class="hover-preview"
      style="left: {previewPosition.x}px; top: {previewPosition.y}px;"
      transition:fade={{ duration: 150 }}
    >
      <img
        src={hoveredVariant.url}
        alt="Preview"
        class="preview-image"
      />
      <div class="preview-info">
        <span class="font-medium">{hoveredVariant.width}x{hoveredVariant.height}</span>
        <span class="text-gray-500">{formatBytes(hoveredVariant.size)}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .responsive-preview {
    background-color: rgb(249 250 251);
    border-radius: 0.75rem;
    padding: 1rem;
  }

  :global(.dark) .responsive-preview {
    background-color: rgba(31, 41, 55, 0.5);
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .variants-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  @media (min-width: 640px) {
    .variants-grid {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }

  @media (min-width: 768px) {
    .variants-grid {
      grid-template-columns: repeat(6, minmax(0, 1fr));
    }
  }

  .variant-card {
    background-color: #fff;
    border-radius: 0.5rem;
    overflow: hidden;
    border: 2px solid transparent;
    transition: all 0.2s;
  }

  :global(.dark) .variant-card {
    background-color: rgb(31 41 55);
  }

  .variant-card.interactive {
    cursor: pointer;
  }

  .variant-card.interactive:hover {
    border-color: rgb(147 197 253);
    box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -1px rgba(0,0,0,0.06);
  }

  :global(.dark) .variant-card.interactive:hover {
    border-color: rgb(37 99 235);
  }

  .variant-card.selected {
    border-color: rgb(59 130 246);
    box-shadow: 0 0 0 2px rgb(191 219 254);
  }

  :global(.dark) .variant-card.selected {
    box-shadow: 0 0 0 2px rgb(30 64 175);
  }

  .variant-thumbnail {
    aspect-ratio: 1/1;
    background-color: rgb(243 244 246);
    overflow: hidden;
  }

  :global(.dark) .variant-thumbnail {
    background-color: rgb(55 65 81);
  }

  .variant-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .variant-info {
    padding: 0.5rem;
    text-align: center;
  }

  .size-name {
    font-size: 0.75rem;
    font-weight: 500;
    color: rgb(55 65 81);
  }

  :global(.dark) .size-name {
    color: rgb(209 213 219);
  }

  .size-dimensions {
    font-size: 10px;
    color: rgb(107 114 128);
  }

  :global(.dark) .size-dimensions {
    color: rgb(156 163 175);
  }

  .size-details {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .file-size {
    font-size: 10px;
    color: rgb(107 114 128);
  }

  .savings {
    font-size: 10px;
    font-weight: 500;
    color: rgb(34 197 94);
  }

  .breakpoint-indicator {
    height: 0.25rem;
    background-color: rgb(229 231 235);
  }

  :global(.dark) .breakpoint-indicator {
    background-color: rgb(55 65 81);
  }

  .indicator-bar {
    height: 100%;
    background-color: rgb(59 130 246);
  }

  .size-chart {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid rgb(229 231 235);
  }

  :global(.dark) .size-chart {
    border-color: rgb(55 65 81);
  }

  .chart-header {
    margin-bottom: 0.5rem;
  }

  .chart-bars {
    display: flex;
    align-items: flex-end;
    justify-content: center;
    gap: 0.5rem;
    height: 5rem;
  }

  .chart-bar-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .chart-bar {
    width: 1rem;
    background-color: rgb(59 130 246);
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
    transition: all 0.3s;
  }

  .chart-bar-wrapper.original .chart-bar {
    background-color: rgb(156 163 175);
  }

  :global(.dark) .chart-bar-wrapper.original .chart-bar {
    background-color: rgb(107 114 128);
  }

  .chart-label {
    font-size: 10px;
    color: rgb(107 114 128);
  }

  .hover-preview {
    position: fixed;
    z-index: 50;
    background-color: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,0.1), 0 10px 10px -5px rgba(0,0,0,0.04);
    border: 1px solid rgb(229 231 235);
    overflow: hidden;
    pointer-events: none;
  }

  :global(.dark) .hover-preview {
    background-color: rgb(31 41 55);
    border-color: rgb(55 65 81);
  }

  .preview-image {
    max-width: 200px;
    max-height: 150px;
    object-fit: contain;
  }

  .preview-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    background-color: rgb(249 250 251);
  }

  :global(.dark) .preview-info {
    background-color: rgb(17 24 39);
  }
</style>
