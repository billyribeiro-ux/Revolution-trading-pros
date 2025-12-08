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
  .stats-compact {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    line-height: 1.25rem;
  }

  .savings-badge {
    padding: 0.125rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    background-color: rgb(220 252 231);
  }

  :global(.dark) .savings-badge {
    background-color: rgba(20, 83, 45, 0.3);
  }

  .stats-card {
    background-color: rgb(240 253 244);
    border: 1px solid rgb(187 247 208);
    border-radius: 0.75rem;
    overflow: hidden;
  }

  :global(.dark) .stats-card {
    background-color: rgba(20, 83, 45, 0.2);
    border-color: rgb(22 101 52);
  }

  .stats-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background-color: rgba(220, 252, 231, 0.5);
    border-bottom: 1px solid rgb(187 247 208);
  }

  :global(.dark) .stats-header {
    background-color: rgba(20, 83, 45, 0.3);
    border-color: rgb(22 101 52);
  }

  .success-icon {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 9999px;
    background-color: rgb(34 197 94);
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stats-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: rgb(22 101 52);
  }

  :global(.dark) .stats-title {
    color: rgb(187 247 208);
  }

  .stats-subtitle {
    font-size: 0.875rem;
    color: rgb(22 163 74);
  }

  :global(.dark) .stats-subtitle {
    color: rgb(74 222 128);
  }

  .stats-main {
    display: flex;
    align-items: center;
    justify-content: space-around;
    padding: 1.5rem;
    gap: 1.5rem;
  }

  .savings-ring {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .savings-value {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .size-comparison {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .size-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .size-label {
    font-size: 0.75rem;
    color: rgb(107 114 128);
    width: 4rem;
  }

  :global(.dark) .size-label {
    color: rgb(156 163 175);
  }

  .size-value {
    font-size: 0.875rem;
  }

  .format-badge {
    padding: 0.125rem 0.375rem;
    font-size: 10px;
    font-weight: 500;
    border-radius: 0.25rem;
    background-color: rgb(229 231 235);
    color: rgb(75 85 99);
  }

  :global(.dark) .format-badge {
    background-color: rgb(55 65 81);
    color: rgb(156 163 175);
  }

  .format-badge-new {
    background-color: rgb(187 247 208);
    color: rgb(21 128 61);
  }

  :global(.dark) .format-badge-new {
    background-color: rgb(22 101 52);
    color: rgb(134 239 172);
  }

  .arrow-down {
    padding: 0.25rem 0;
  }

  .stats-details {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1.5rem;
    padding: 0.75rem 1rem;
    border-top: 1px solid rgb(187 247 208);
    font-size: 0.75rem;
    color: rgb(75 85 99);
  }

  :global(.dark) .stats-details {
    border-color: rgb(22 101 52);
    color: rgb(156 163 175);
  }

  .detail-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }
</style>
