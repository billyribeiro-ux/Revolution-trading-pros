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
  /* Base skeleton styles */
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
    gap: 1rem;
  }

  .skeleton-card {
    background-color: rgb(243 244 246);
    border-radius: 0.5rem;
    overflow: hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  :global(.dark) .skeleton-card {
    background-color: rgb(31 41 55);
  }

  .skeleton-thumbnail {
    position: relative;
    background-color: rgb(229 231 235);
    overflow: hidden;
  }

  :global(.dark) .skeleton-thumbnail {
    background-color: rgb(55 65 81);
  }

  .skeleton-info {
    padding: 0.75rem;
  }

  .skeleton-info > * + * {
    margin-top: 0.5rem;
  }

  .skeleton-title {
    height: 1rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 75%;
  }

  :global(.dark) .skeleton-title {
    background-color: rgb(55 65 81);
  }

  .skeleton-meta {
    height: 0.75rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 50%;
  }

  :global(.dark) .skeleton-meta {
    background-color: rgb(55 65 81);
  }

  /* Large card skeleton */
  .skeleton-card-large {
    background-color: rgb(243 244 246);
    border-radius: 0.75rem;
    overflow: hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  :global(.dark) .skeleton-card-large {
    background-color: rgb(31 41 55);
  }

  .skeleton-thumbnail-large {
    position: relative;
    background-color: rgb(229 231 235);
    overflow: hidden;
  }

  :global(.dark) .skeleton-thumbnail-large {
    background-color: rgb(55 65 81);
  }

  .skeleton-content {
    padding: 1.5rem;
  }

  .skeleton-content > * + * {
    margin-top: 1rem;
  }

  .skeleton-heading {
    height: 1.5rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 66.666667%;
  }

  :global(.dark) .skeleton-heading {
    background-color: rgb(55 65 81);
  }

  .skeleton-text {
    height: 1rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
  }

  :global(.dark) .skeleton-text {
    background-color: rgb(55 65 81);
  }

  .skeleton-text.short {
    width: 50%;
  }

  .skeleton-actions {
    display: flex;
    gap: 0.75rem;
    padding-top: 0.5rem;
  }

  .skeleton-button {
    height: 2.5rem;
    width: 6rem;
    background-color: rgb(229 231 235);
    border-radius: 0.5rem;
  }

  :global(.dark) .skeleton-button {
    background-color: rgb(55 65 81);
  }

  /* List skeleton */
  .media-skeleton-list > * + * {
    margin-top: 0.5rem;
  }

  .skeleton-row {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background-color: rgb(243 244 246);
    border-radius: 0.5rem;
    animation: pulse 2s ease-in-out infinite;
  }

  :global(.dark) .skeleton-row {
    background-color: rgb(31 41 55);
  }

  .skeleton-checkbox {
    width: 1.25rem;
    height: 1.25rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
  }

  :global(.dark) .skeleton-checkbox {
    background-color: rgb(55 65 81);
  }

  .skeleton-thumb-small {
    width: 3rem;
    height: 3rem;
    background-color: rgb(229 231 235);
    border-radius: 0.5rem;
    position: relative;
    overflow: hidden;
  }

  :global(.dark) .skeleton-thumb-small {
    background-color: rgb(55 65 81);
  }

  .skeleton-row-content {
    flex: 1;
  }

  .skeleton-row-content > * + * {
    margin-top: 0.5rem;
  }

  .skeleton-row-title {
    height: 1rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 12rem;
  }

  :global(.dark) .skeleton-row-title {
    background-color: rgb(55 65 81);
  }

  .skeleton-row-meta {
    height: 0.75rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 8rem;
  }

  :global(.dark) .skeleton-row-meta {
    background-color: rgb(55 65 81);
  }

  .skeleton-row-size {
    height: 1rem;
    width: 4rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
  }

  :global(.dark) .skeleton-row-size {
    background-color: rgb(55 65 81);
  }

  .skeleton-row-date {
    height: 1rem;
    width: 6rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
  }

  :global(.dark) .skeleton-row-date {
    background-color: rgb(55 65 81);
  }

  .skeleton-row-actions {
    height: 2rem;
    width: 5rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
  }

  :global(.dark) .skeleton-row-actions {
    background-color: rgb(55 65 81);
  }

  /* Upload skeleton */
  .skeleton-upload {
    background-color: rgb(243 244 246);
    border-radius: 0.75rem;
    overflow: hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  :global(.dark) .skeleton-upload {
    background-color: rgb(31 41 55);
  }

  .skeleton-upload-header {
    padding: 1rem;
    border-bottom: 1px solid rgb(229 231 235);
  }

  .skeleton-upload-header > * + * {
    margin-top: 0.5rem;
  }

  :global(.dark) .skeleton-upload-header {
    border-color: rgb(55 65 81);
  }

  .skeleton-upload-title {
    height: 1.25rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 8rem;
  }

  :global(.dark) .skeleton-upload-title {
    background-color: rgb(55 65 81);
  }

  .skeleton-progress-bar {
    height: 0.25rem;
    background-color: rgb(229 231 235);
    border-radius: 9999px;
    position: relative;
    overflow: hidden;
  }

  :global(.dark) .skeleton-progress-bar {
    background-color: rgb(55 65 81);
  }

  .skeleton-upload-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-bottom: 1px solid rgb(229 231 235);
  }

  .skeleton-upload-item:last-child {
    border-bottom: 0;
  }

  :global(.dark) .skeleton-upload-item {
    border-color: rgb(55 65 81);
  }

  .skeleton-upload-thumb {
    width: 3rem;
    height: 3rem;
    background-color: rgb(229 231 235);
    border-radius: 0.5rem;
    position: relative;
    overflow: hidden;
  }

  :global(.dark) .skeleton-upload-thumb {
    background-color: rgb(55 65 81);
  }

  .skeleton-upload-info {
    flex: 1;
  }

  .skeleton-upload-info > * + * {
    margin-top: 0.5rem;
  }

  .skeleton-upload-name {
    height: 1rem;
    background-color: rgb(229 231 235);
    border-radius: 0.25rem;
    width: 10rem;
  }

  :global(.dark) .skeleton-upload-name {
    background-color: rgb(55 65 81);
  }

  .skeleton-upload-progress {
    height: 0.5rem;
    background-color: rgb(229 231 235);
    border-radius: 9999px;
    position: relative;
    overflow: hidden;
  }

  :global(.dark) .skeleton-upload-progress {
    background-color: rgb(55 65 81);
  }

  .skeleton-upload-action {
    width: 2rem;
    height: 2rem;
    background-color: rgb(229 231 235);
    border-radius: 9999px;
  }

  :global(.dark) .skeleton-upload-action {
    background-color: rgb(55 65 81);
  }

  /* Single image skeleton */
  .skeleton-single {
    position: relative;
    background-color: rgb(229 231 235);
    border-radius: 0.5rem;
    overflow: hidden;
    animation: pulse 2s ease-in-out infinite;
  }

  :global(.dark) .skeleton-single {
    background-color: rgb(55 65 81);
  }

  .skeleton-icon {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
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
