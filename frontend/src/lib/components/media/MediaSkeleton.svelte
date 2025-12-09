<!--
  MediaSkeleton Component
  ═══════════════════════════════════════════════════════════════════════════

  Content-shaped skeleton loaders for media components:
  - Grid skeleton for gallery views
  - Card skeleton for individual items
  - List skeleton for table views
  - Upload skeleton for upload progress

  @version 2.0.0
-->
<script lang="ts">
  // Props
  let {
    type = 'grid',
    count = 8,
    columns = 4,
    aspectRatio = '1/1',
    showText = true,
    className = ''
  }: {
    type?: 'grid' | 'card' | 'list' | 'upload' | 'single';
    count?: number;
    columns?: number;
    aspectRatio?: string;
    showText?: boolean;
    className?: string;
  } = $props();
</script>

{#if type === 'grid'}
  <!-- Grid skeleton for gallery views -->
  <div
    class="media-skeleton-grid gap-4 {className}"
    style="--columns: {columns};"
  >
    {#each Array(count) as _, i}
      <div class="skeleton-card bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden" style="animation-delay: {i * 0.05}s;">
        <div class="relative bg-gray-200 dark:bg-gray-700 overflow-hidden" style="aspect-ratio: {aspectRatio};">
          <div class="skeleton-shimmer"></div>
        </div>
        {#if showText}
          <div class="p-3 space-y-2">
            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
          </div>
        {/if}
      </div>
    {/each}
  </div>

{:else if type === 'card'}
  <!-- Single card skeleton -->
  <div class="skeleton-card-large bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden {className}">
    <div class="relative bg-gray-200 dark:bg-gray-700 overflow-hidden" style="aspect-ratio: {aspectRatio};">
      <div class="skeleton-shimmer"></div>
    </div>
    {#if showText}
      <div class="p-6 space-y-4">
        <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
        <div class="flex gap-3 pt-2">
          <div class="h-10 w-24 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
          <div class="h-10 w-24 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
        </div>
      </div>
    {/if}
  </div>

{:else if type === 'list'}
  <!-- List/table skeleton -->
  <div class="space-y-2 {className}">
    {#each Array(count) as _, i}
      <div class="skeleton-row flex items-center gap-4 p-3 bg-gray-100 dark:bg-gray-800 rounded-lg" style="animation-delay: {i * 0.05}s;">
        <div class="w-5 h-5 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg relative overflow-hidden">
          <div class="skeleton-shimmer"></div>
        </div>
        <div class="flex-1 space-y-2">
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-48"></div>
          <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-32"></div>
        </div>
        <div class="h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-4 w-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="h-8 w-20 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    {/each}
  </div>

{:else if type === 'upload'}
  <!-- Upload progress skeleton -->
  <div class="skeleton-upload bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden {className}">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 space-y-2">
      <div class="h-5 bg-gray-200 dark:bg-gray-700 rounded w-32"></div>
      <div class="h-1 bg-gray-200 dark:bg-gray-700 rounded-full relative overflow-hidden">
        <div class="skeleton-shimmer"></div>
      </div>
    </div>
    {#each Array(count) as _, i}
      <div class="skeleton-upload-item flex items-center gap-3 p-4 border-b border-gray-200 dark:border-gray-700 last:border-0" style="animation-delay: {i * 0.1}s;">
        <div class="w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg relative overflow-hidden">
          <div class="skeleton-shimmer"></div>
        </div>
        <div class="flex-1 space-y-2">
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-40"></div>
          <div class="h-2 bg-gray-200 dark:bg-gray-700 rounded-full relative overflow-hidden">
            <div class="skeleton-shimmer"></div>
          </div>
        </div>
        <div class="w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
      </div>
    {/each}
  </div>

{:else if type === 'single'}
  <!-- Single image skeleton -->
  <div class="skeleton-single relative bg-gray-200 dark:bg-gray-700 rounded-lg overflow-hidden {className}" style="aspect-ratio: {aspectRatio};">
    <div class="skeleton-shimmer"></div>
    <div class="absolute inset-0 flex items-center justify-center">
      <svg class="w-12 h-12 text-gray-300 dark:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
    </div>
  </div>
{/if}

<style>
  /* Shimmer gradient animation */
  .skeleton-shimmer {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.4) 50%,
      transparent 100%
    );
    animation: shimmer 1.5s infinite;
  }

  :global(.dark) .skeleton-shimmer {
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.1) 50%,
      transparent 100%
    );
  }

  /* Keyframe animations */
  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  /* Grid layout with CSS variable */
  .media-skeleton-grid {
    display: grid;
    grid-template-columns: repeat(var(--columns), 1fr);
  }

  /* Pulse animations for skeleton elements */
  .skeleton-card {
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-card-large {
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-row {
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-upload {
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-upload-item {
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-single {
    animation: pulse 2s ease-in-out infinite;
  }

  /* Responsive grid adjustments */
  @media (max-width: 768px) {
    .media-skeleton-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 480px) {
    .media-skeleton-grid {
      grid-template-columns: repeat(1, 1fr);
    }
  }
</style>
