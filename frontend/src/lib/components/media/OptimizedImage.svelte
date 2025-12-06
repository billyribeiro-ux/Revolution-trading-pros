<!--
  OptimizedImage Component
  ═══════════════════════════════════════════════════════════════════════════

  High-performance image component with:
  - BlurHash placeholder rendering
  - LQIP (Low Quality Image Placeholder) fallback
  - Progressive loading with smooth fade-in
  - Responsive srcset support
  - WebP/AVIF format negotiation
  - Lazy loading with Intersection Observer

  @version 1.0.0
-->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { decode } from 'blurhash';

  // Props
  export let src: string;
  export let alt: string = '';
  export let blurhash: string | null = null;
  export let lqip: string | null = null;
  export let width: number | null = null;
  export let height: number | null = null;
  export let srcset: Record<string, string> | null = null;
  export let sizes: string = '100vw';
  export let loading: 'lazy' | 'eager' = 'lazy';
  export let className: string = '';
  export let aspectRatio: string | null = null;
  export let objectFit: 'cover' | 'contain' | 'fill' | 'none' = 'cover';
  export let priority: boolean = false;
  export let onLoad: (() => void) | null = null;
  export let onError: ((error: Error) => void) | null = null;

  // State
  let loaded = false;
  let error = false;
  let canvas: HTMLCanvasElement;
  let imgElement: HTMLImageElement;
  let containerRef: HTMLDivElement;
  let isInView = false;
  let observer: IntersectionObserver | null = null;

  // Decode BlurHash to canvas
  function renderBlurhash(node: HTMLCanvasElement) {
    if (!blurhash || !node) return;

    try {
      const pixels = decode(blurhash, 32, 32);
      const ctx = node.getContext('2d');
      if (!ctx) return;

      const imageData = ctx.createImageData(32, 32);
      imageData.data.set(pixels);
      ctx.putImageData(imageData, 0, 0);
    } catch (err) {
      console.warn('BlurHash decode failed:', err);
    }
  }

  // Build srcset string
  function buildSrcset(): string | undefined {
    if (!srcset) return undefined;

    return Object.entries(srcset)
      .map(([size, url]) => {
        const width = size.replace(/[^0-9]/g, '');
        return `${url} ${width}w`;
      })
      .join(', ');
  }

  // Handle image load
  function handleLoad() {
    loaded = true;
    onLoad?.();
  }

  // Handle image error
  function handleError(e: Event) {
    error = true;
    onError?.(new Error('Image failed to load'));
  }

  // Setup Intersection Observer for lazy loading
  onMount(() => {
    if (priority || loading === 'eager') {
      isInView = true;
      return;
    }

    observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            isInView = true;
            observer?.disconnect();
          }
        });
      },
      {
        rootMargin: '50px',
        threshold: 0.01,
      }
    );

    if (containerRef) {
      observer.observe(containerRef);
    }
  });

  onDestroy(() => {
    observer?.disconnect();
  });

  // Computed aspect ratio style
  $: aspectRatioStyle = aspectRatio
    ? `aspect-ratio: ${aspectRatio};`
    : width && height
      ? `aspect-ratio: ${width}/${height};`
      : '';
</script>

<div
  bind:this={containerRef}
  class="optimized-image-container relative overflow-hidden {className}"
  style="{aspectRatioStyle}"
>
  <!-- BlurHash placeholder canvas -->
  {#if blurhash && !loaded}
    <canvas
      bind:this={canvas}
      use:renderBlurhash
      width="32"
      height="32"
      class="absolute inset-0 w-full h-full blur-xl scale-110 transition-opacity duration-500"
      class:opacity-0={loaded}
      aria-hidden="true"
    />
  {/if}

  <!-- LQIP fallback -->
  {#if lqip && !blurhash && !loaded}
    <img
      src={lqip}
      alt=""
      class="absolute inset-0 w-full h-full object-{objectFit} blur-lg scale-105 transition-opacity duration-500"
      class:opacity-0={loaded}
      aria-hidden="true"
    />
  {/if}

  <!-- Main image with picture element for format negotiation -->
  {#if isInView || priority}
    <picture>
      <!-- AVIF source (best compression) -->
      {#if srcset?.avif}
        <source
          srcset={srcset.avif}
          type="image/avif"
        />
      {/if}

      <!-- WebP source (good compression, wide support) -->
      {#if srcset?.webp}
        <source
          srcset={srcset.webp}
          type="image/webp"
        />
      {/if}

      <!-- Responsive srcset -->
      {#if srcset && !srcset.avif && !srcset.webp}
        <source
          srcset={buildSrcset()}
          {sizes}
        />
      {/if}

      <!-- Fallback image -->
      <img
        bind:this={imgElement}
        {src}
        {alt}
        {width}
        {height}
        loading={priority ? 'eager' : loading}
        decoding={priority ? 'sync' : 'async'}
        fetchpriority={priority ? 'high' : 'auto'}
        onload={handleLoad}
        onerror={handleError}
        class="w-full h-full object-{objectFit} transition-opacity duration-500"
        class:opacity-0={!loaded}
        style="color: transparent;"
      />
    </picture>
  {/if}

  <!-- Error state -->
  {#if error}
    <div class="absolute inset-0 flex items-center justify-center bg-gray-100 dark:bg-gray-800">
      <div class="text-center text-gray-500 dark:text-gray-400">
        <svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <p class="text-sm">Failed to load image</p>
      </div>
    </div>
  {/if}

  <!-- Loading skeleton (if no placeholder available) -->
  {#if !blurhash && !lqip && !loaded && !error && isInView}
    <div class="absolute inset-0 bg-gray-200 dark:bg-gray-700 animate-pulse" />
  {/if}
</div>

<style>
  .optimized-image-container {
    background-color: transparent;
  }

  /* Smooth blur-to-sharp transition */
  canvas {
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }
</style>
