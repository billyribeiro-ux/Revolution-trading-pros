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
    class="media-skeleton-grid {className}"
    style="--columns: {columns};"
  >
    {#each Array(count) as _, i}
      <div class="skeleton-card" style="animation-delay: {i * 0.05}s;">
        <div class="skeleton-thumbnail" style="aspect-ratio: {aspectRatio};">
          <div class="skeleton-shimmer"></div>
        </div>
        {#if showText}
          <div class="skeleton-info">
            <div class="skeleton-title"></div>
            <div class="skeleton-meta"></div>
          </div>
        {/if}
      </div>
    {/each}
  </div>

{:else if type === 'card'}
  <!-- Single card skeleton -->
  <div class="skeleton-card-large {className}">
    <div class="skeleton-thumbnail-large" style="aspect-ratio: {aspectRatio};">
      <div class="skeleton-shimmer"></div>
    </div>
    {#if showText}
      <div class="skeleton-content">
        <div class="skeleton-heading"></div>
        <div class="skeleton-text"></div>
        <div class="skeleton-text short"></div>
        <div class="skeleton-actions">
          <div class="skeleton-button"></div>
          <div class="skeleton-button"></div>
        </div>
      </div>
    {/if}
  </div>

{:else if type === 'list'}
  <!-- List/table skeleton -->
  <div class="media-skeleton-list {className}">
    {#each Array(count) as _, i}
      <div class="skeleton-row" style="animation-delay: {i * 0.05}s;">
        <div class="skeleton-checkbox"></div>
        <div class="skeleton-thumb-small">
          <div class="skeleton-shimmer"></div>
        </div>
        <div class="skeleton-row-content">
          <div class="skeleton-row-title"></div>
          <div class="skeleton-row-meta"></div>
        </div>
        <div class="skeleton-row-size"></div>
        <div class="skeleton-row-date"></div>
        <div class="skeleton-row-actions"></div>
      </div>
    {/each}
  </div>

{:else if type === 'upload'}
  <!-- Upload progress skeleton -->
  <div class="skeleton-upload {className}">
    <div class="skeleton-upload-header">
      <div class="skeleton-upload-title"></div>
      <div class="skeleton-progress-bar">
        <div class="skeleton-shimmer"></div>
      </div>
    </div>
    {#each Array(count) as _, i}
      <div class="skeleton-upload-item" style="animation-delay: {i * 0.1}s;">
        <div class="skeleton-upload-thumb">
          <div class="skeleton-shimmer"></div>
        </div>
        <div class="skeleton-upload-info">
          <div class="skeleton-upload-name"></div>
          <div class="skeleton-upload-progress">
            <div class="skeleton-shimmer"></div>
          </div>
        </div>
        <div class="skeleton-upload-action"></div>
      </div>
    {/each}
  </div>

{:else if type === 'single'}
  <!-- Single image skeleton -->
  <div class="skeleton-single {className}" style="aspect-ratio: {aspectRatio};">
    <div class="skeleton-shimmer"></div>
    <div class="skeleton-icon">
      <svg class="w-12 h-12 text-gray-300 dark:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
    </div>
  </div>
{/if}

<style>
  @reference "tailwindcss";

  /* Base skeleton styles */
  .skeleton-shimmer {
    @apply absolute inset-0;
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

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  /* Grid skeleton */
  .media-skeleton-grid {
    display: grid;
    grid-template-columns: repeat(var(--columns), 1fr);
    @apply gap-4;
  }

  .skeleton-card {
    @apply bg-gray-100 dark:bg-gray-800 rounded-lg overflow-hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-thumbnail {
    @apply relative bg-gray-200 dark:bg-gray-700 overflow-hidden;
  }

  .skeleton-info {
    @apply p-3 space-y-2;
  }

  .skeleton-title {
    @apply h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4;
  }

  .skeleton-meta {
    @apply h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2;
  }

  /* Large card skeleton */
  .skeleton-card-large {
    @apply bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-thumbnail-large {
    @apply relative bg-gray-200 dark:bg-gray-700 overflow-hidden;
  }

  .skeleton-content {
    @apply p-6 space-y-4;
  }

  .skeleton-heading {
    @apply h-6 bg-gray-200 dark:bg-gray-700 rounded w-2/3;
  }

  .skeleton-text {
    @apply h-4 bg-gray-200 dark:bg-gray-700 rounded;
  }

  .skeleton-text.short {
    @apply w-1/2;
  }

  .skeleton-actions {
    @apply flex gap-3 pt-2;
  }

  .skeleton-button {
    @apply h-10 w-24 bg-gray-200 dark:bg-gray-700 rounded-lg;
  }

  /* List skeleton */
  .media-skeleton-list {
    @apply space-y-2;
  }

  .skeleton-row {
    @apply flex items-center gap-4 p-3 bg-gray-100 dark:bg-gray-800 rounded-lg;
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-checkbox {
    @apply w-5 h-5 bg-gray-200 dark:bg-gray-700 rounded;
  }

  .skeleton-thumb-small {
    @apply w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg relative overflow-hidden;
  }

  .skeleton-row-content {
    @apply flex-1 space-y-2;
  }

  .skeleton-row-title {
    @apply h-4 bg-gray-200 dark:bg-gray-700 rounded w-48;
  }

  .skeleton-row-meta {
    @apply h-3 bg-gray-200 dark:bg-gray-700 rounded w-32;
  }

  .skeleton-row-size {
    @apply h-4 w-16 bg-gray-200 dark:bg-gray-700 rounded;
  }

  .skeleton-row-date {
    @apply h-4 w-24 bg-gray-200 dark:bg-gray-700 rounded;
  }

  .skeleton-row-actions {
    @apply h-8 w-20 bg-gray-200 dark:bg-gray-700 rounded;
  }

  /* Upload skeleton */
  .skeleton-upload {
    @apply bg-gray-100 dark:bg-gray-800 rounded-xl overflow-hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-upload-header {
    @apply p-4 border-b border-gray-200 dark:border-gray-700 space-y-2;
  }

  .skeleton-upload-title {
    @apply h-5 bg-gray-200 dark:bg-gray-700 rounded w-32;
  }

  .skeleton-progress-bar {
    @apply h-1 bg-gray-200 dark:bg-gray-700 rounded-full relative overflow-hidden;
  }

  .skeleton-upload-item {
    @apply flex items-center gap-3 p-4 border-b border-gray-200 dark:border-gray-700 last:border-0;
  }

  .skeleton-upload-thumb {
    @apply w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg relative overflow-hidden;
  }

  .skeleton-upload-info {
    @apply flex-1 space-y-2;
  }

  .skeleton-upload-name {
    @apply h-4 bg-gray-200 dark:bg-gray-700 rounded w-40;
  }

  .skeleton-upload-progress {
    @apply h-2 bg-gray-200 dark:bg-gray-700 rounded-full relative overflow-hidden;
  }

  .skeleton-upload-action {
    @apply w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full;
  }

  /* Single image skeleton */
  .skeleton-single {
    @apply relative bg-gray-200 dark:bg-gray-700 rounded-lg overflow-hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  .skeleton-icon {
    @apply absolute inset-0 flex items-center justify-center;
  }

  /* Responsive adjustments */
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
