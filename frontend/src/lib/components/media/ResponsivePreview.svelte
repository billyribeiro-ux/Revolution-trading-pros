<!--
  ResponsivePreview Component
  ═══════════════════════════════════════════════════════════════════════════

  Preview image at different responsive breakpoints:
  - Shows all 6 responsive sizes side by side
  - File size comparison for each variant
  - Interactive selection
  - Zoom preview on hover

  @version 1.0.0
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fade, scale } from 'svelte/transition';

  interface Variant {
    sizeName: string;
    width: number;
    height: number;
    url: string;
    size: number;
  }

  // Props
  export let variants: Variant[] = [];
  export let originalUrl: string = '';
  export let originalSize: number = 0;
  export let selectedSize: string | null = null;
  export let showSizes: boolean = true;
  export let interactive: boolean = true;
  export let className: string = '';

  const dispatch = createEventDispatcher<{
    select: string;
  }>();

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
  let hoveredVariant: Variant | null = null;
  let previewPosition = { x: 0, y: 0 };

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
    dispatch('select', sizeName);
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

  // Sort variants by width
  $: sortedVariants = [...variants].sort((a, b) => a.width - b.width);
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
        on:click={() => handleSelect(variant.sizeName)}
        on:mouseenter={(e) => handleMouseEnter(e, variant)}
        on:mousemove={handleMouseMove}
        on:mouseleave={handleMouseLeave}
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
          />
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
            />
            <span class="chart-label">{variant.sizeName}</span>
          </div>
        {/each}
        <div class="chart-bar-wrapper original">
          <div class="chart-bar chart-bar-original" style="height: 100%" />
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
    @apply bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4;
  }

  .preview-header {
    @apply flex items-center justify-between mb-4;
  }

  .variants-grid {
    @apply grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-3;
  }

  .variant-card {
    @apply bg-white dark:bg-gray-800 rounded-lg overflow-hidden;
    @apply border-2 border-transparent;
    @apply transition-all duration-200;
  }

  .variant-card.interactive {
    @apply cursor-pointer hover:border-blue-300 dark:hover:border-blue-600;
    @apply hover:shadow-md;
  }

  .variant-card.selected {
    @apply border-blue-500 ring-2 ring-blue-200 dark:ring-blue-800;
  }

  .variant-thumbnail {
    @apply aspect-square bg-gray-100 dark:bg-gray-700 overflow-hidden;
  }

  .variant-thumbnail img {
    @apply w-full h-full object-cover;
  }

  .variant-info {
    @apply p-2 text-center;
  }

  .size-name {
    @apply text-xs font-medium text-gray-700 dark:text-gray-300;
  }

  .size-dimensions {
    @apply text-[10px] text-gray-500 dark:text-gray-400;
  }

  .size-details {
    @apply flex items-center justify-center gap-2 mt-1;
  }

  .file-size {
    @apply text-[10px] text-gray-500;
  }

  .savings {
    @apply text-[10px] font-medium text-green-500;
  }

  .breakpoint-indicator {
    @apply h-1 bg-gray-200 dark:bg-gray-700;
  }

  .indicator-bar {
    @apply h-full bg-blue-500;
  }

  .size-chart {
    @apply mt-4 pt-4 border-t border-gray-200 dark:border-gray-700;
  }

  .chart-header {
    @apply mb-2;
  }

  .chart-bars {
    @apply flex items-end justify-center gap-2 h-20;
  }

  .chart-bar-wrapper {
    @apply flex flex-col items-center gap-1;
  }

  .chart-bar {
    @apply w-4 bg-blue-500 rounded-t transition-all duration-300;
  }

  .chart-bar-wrapper.original .chart-bar {
    @apply bg-gray-400 dark:bg-gray-500;
  }

  .chart-label {
    @apply text-[10px] text-gray-500;
  }

  .hover-preview {
    @apply fixed z-50 bg-white dark:bg-gray-800 rounded-lg shadow-xl;
    @apply border border-gray-200 dark:border-gray-700 overflow-hidden;
    @apply pointer-events-none;
  }

  .preview-image {
    @apply max-w-[200px] max-h-[150px] object-contain;
  }

  .preview-info {
    @apply flex items-center justify-between gap-2 px-2 py-1;
    @apply text-xs bg-gray-50 dark:bg-gray-900;
  }
</style>
