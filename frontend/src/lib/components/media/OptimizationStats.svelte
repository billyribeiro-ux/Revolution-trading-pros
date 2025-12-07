<!--
  OptimizationStats Component
  ═══════════════════════════════════════════════════════════════════════════

  Display image optimization statistics with visual feedback:
  - Original vs optimized size comparison
  - Percentage savings
  - Format conversions
  - Processing time

  @version 2.0.0
-->
<script lang="ts">
  import { fade, slide } from 'svelte/transition';
  import { tweened } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';

  interface Stats {
    originalSize: number;
    optimizedSize: number;
    savingsPercent: number;
    originalFormat?: string;
    optimizedFormat?: string;
    processingTime?: number;
    variantsCount?: number;
  }

  // Props
  let {
    stats,
    showDetails = true,
    compact = false,
    animated = true,
    className = ''
  }: {
    stats: Stats;
    showDetails?: boolean;
    compact?: boolean;
    animated?: boolean;
    className?: string;
  } = $props();

  // Animated savings
  const animatedSavings = tweened(0, {
    duration: 1000,
    easing: cubicOut,
  });

  $effect(() => {
    const duration = animated ? 1000 : 0;
    animatedSavings.set(stats.savingsPercent, { duration });
  });

  // Format bytes
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  // Get savings color
  function getSavingsColor(percent: number): string {
    if (percent >= 70) return 'text-green-500';
    if (percent >= 50) return 'text-green-400';
    if (percent >= 30) return 'text-yellow-500';
    return 'text-orange-500';
  }

  // Get ring color
  function getRingColor(percent: number): string {
    if (percent >= 70) return 'stroke-green-500';
    if (percent >= 50) return 'stroke-green-400';
    if (percent >= 30) return 'stroke-yellow-500';
    return 'stroke-orange-500';
  }
</script>

{#if compact}
  <!-- Compact inline version -->
  <div class="stats-compact {className}" transition:fade>
    <span class="text-gray-400 line-through">{formatBytes(stats.originalSize)}</span>
    <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
    </svg>
    <span class="font-medium text-gray-700 dark:text-gray-300">{formatBytes(stats.optimizedSize)}</span>
    <span class="savings-badge {getSavingsColor(stats.savingsPercent)}">
      -{Math.round($animatedSavings)}%
    </span>
  </div>
{:else}
  <!-- Full card version -->
  <div class="stats-card {className}" transition:slide>
    <!-- Success header -->
    <div class="stats-header">
      <div class="success-icon">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <div>
        <h3 class="stats-title">Optimized Successfully</h3>
        {#if stats.processingTime}
          <p class="stats-subtitle">Processed in {stats.processingTime}ms</p>
        {/if}
      </div>
    </div>

    <!-- Main stats -->
    <div class="stats-main">
      <!-- Circular progress -->
      <div class="savings-ring">
        <svg viewBox="0 0 100 100" class="w-24 h-24">
          <!-- Background ring -->
          <circle
            cx="50"
            cy="50"
            r="40"
            fill="none"
            stroke="currentColor"
            stroke-width="8"
            class="text-gray-200 dark:text-gray-700"
          />
          <!-- Progress ring -->
          <circle
            cx="50"
            cy="50"
            r="40"
            fill="none"
            stroke-width="8"
            stroke-linecap="round"
            class="{getRingColor(stats.savingsPercent)} transition-all duration-1000"
            stroke-dasharray="{$animatedSavings * 2.51} 251"
            transform="rotate(-90 50 50)"
          />
        </svg>
        <div class="savings-value">
          <span class="text-2xl font-bold {getSavingsColor(stats.savingsPercent)}">
            {Math.round($animatedSavings)}%
          </span>
          <span class="text-xs text-gray-500">smaller</span>
        </div>
      </div>

      <!-- Size comparison -->
      <div class="size-comparison">
        <div class="size-item">
          <span class="size-label">Original</span>
          <span class="size-value text-gray-500 line-through">{formatBytes(stats.originalSize)}</span>
          {#if stats.originalFormat}
            <span class="format-badge">{stats.originalFormat.toUpperCase()}</span>
          {/if}
        </div>
        <div class="arrow-down">
          <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
          </svg>
        </div>
        <div class="size-item">
          <span class="size-label">Optimized</span>
          <span class="size-value text-green-600 dark:text-green-400 font-semibold">{formatBytes(stats.optimizedSize)}</span>
          {#if stats.optimizedFormat}
            <span class="format-badge format-badge-new">{stats.optimizedFormat.toUpperCase()}</span>
          {/if}
        </div>
      </div>
    </div>

    <!-- Additional details -->
    {#if showDetails}
      <div class="stats-details">
        <div class="detail-item">
          <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          <span>Lossless quality</span>
        </div>
        {#if stats.variantsCount && stats.variantsCount > 0}
          <div class="detail-item">
            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z" />
            </svg>
            <span>{stats.variantsCount} responsive sizes</span>
          </div>
        {/if}
        <div class="detail-item">
          <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
          <span>CDN-ready</span>
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  @reference "tailwindcss";

  .stats-compact {
    @apply inline-flex items-center gap-2 text-sm;
  }

  .savings-badge {
    @apply px-2 py-0.5 rounded-full text-xs font-semibold;
    @apply bg-green-100 dark:bg-green-900/30;
  }

  .stats-card {
    @apply bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800;
    @apply rounded-xl overflow-hidden;
  }

  .stats-header {
    @apply flex items-center gap-3 p-4 bg-green-100/50 dark:bg-green-900/30;
    @apply border-b border-green-200 dark:border-green-800;
  }

  .success-icon {
    @apply w-10 h-10 rounded-full bg-green-500 text-white;
    @apply flex items-center justify-center;
  }

  .stats-title {
    @apply text-lg font-semibold text-green-800 dark:text-green-200;
  }

  .stats-subtitle {
    @apply text-sm text-green-600 dark:text-green-400;
  }

  .stats-main {
    @apply flex items-center justify-around p-6 gap-6;
  }

  .savings-ring {
    @apply relative flex items-center justify-center;
  }

  .savings-value {
    @apply absolute inset-0 flex flex-col items-center justify-center;
  }

  .size-comparison {
    @apply flex flex-col items-center gap-2;
  }

  .size-item {
    @apply flex items-center gap-2;
  }

  .size-label {
    @apply text-xs text-gray-500 dark:text-gray-400 w-16;
  }

  .size-value {
    @apply text-sm;
  }

  .format-badge {
    @apply px-1.5 py-0.5 text-[10px] font-medium rounded;
    @apply bg-gray-200 text-gray-600 dark:bg-gray-700 dark:text-gray-400;
  }

  .format-badge-new {
    @apply bg-green-200 text-green-700 dark:bg-green-800 dark:text-green-300;
  }

  .arrow-down {
    @apply py-1;
  }

  .stats-details {
    @apply flex items-center justify-center gap-6 px-4 py-3;
    @apply border-t border-green-200 dark:border-green-800;
    @apply text-xs text-gray-600 dark:text-gray-400;
  }

  .detail-item {
    @apply flex items-center gap-1.5;
  }
</style>
