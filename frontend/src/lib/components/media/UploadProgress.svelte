<!--
  UploadProgress Component
  ═══════════════════════════════════════════════════════════════════════════

  Visual upload progress panel with:
  - Individual file progress bars
  - File thumbnails/previews
  - Upload status indicators
  - Cancel/retry actions
  - Total progress overview

  @version 2.0.0
-->
<script lang="ts">
  import { fade, slide } from 'svelte/transition';

  interface UploadItem {
    id: string;
    file: File;
    progress: number;
    status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
    error?: string;
    result?: {
      id: string;
      url: string;
      thumbnailUrl?: string;
      stats?: {
        originalSize: number;
        optimizedSize: number;
        savingsPercent: number;
      };
    };
  }

  // Props
  let {
    uploads = [],
    showStats = true,
    autoHideDelay = 5000,
    className = '',
    onCancel,
    onRetry,
    onRemove,
    onClear
  }: {
    uploads?: UploadItem[];
    showStats?: boolean;
    autoHideDelay?: number;
    className?: string;
    onCancel?: (id: string) => void;
    onRetry?: (id: string) => void;
    onRemove?: (id: string) => void;
    onClear?: () => void;
  } = $props();

  // Computed values
  let completedCount = $derived(uploads.filter((u) => u.status === 'complete').length);
  let errorCount = $derived(uploads.filter((u) => u.status === 'error').length);
  let inProgressCount = $derived(uploads.filter((u) => u.status === 'uploading' || u.status === 'processing').length);
  let totalProgress = $derived(
    uploads.length > 0
      ? Math.round(uploads.reduce((sum, u) => sum + u.progress, 0) / uploads.length)
      : 0
  );
  let isComplete = $derived(completedCount + errorCount === uploads.length && uploads.length > 0);

  // Format bytes
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  // Get file thumbnail
  function getFileThumbnail(file: File): string {
    if (file.type.startsWith('image/')) {
      return URL.createObjectURL(file);
    }
    return '';
  }

  // Get status color
  function getStatusColor(status: UploadItem['status']): string {
    switch (status) {
      case 'pending': return 'bg-gray-300 dark:bg-gray-600';
      case 'uploading': return 'bg-blue-500';
      case 'processing': return 'bg-yellow-500';
      case 'complete': return 'bg-green-500';
      case 'error': return 'bg-red-500';
      default: return 'bg-gray-300';
    }
  }

  // Get status text
  function getStatusText(status: UploadItem['status']): string {
    switch (status) {
      case 'pending': return 'Waiting...';
      case 'uploading': return 'Uploading...';
      case 'processing': return 'Processing...';
      case 'complete': return 'Complete';
      case 'error': return 'Failed';
      default: return '';
    }
  }
</script>

{#if uploads.length > 0}
  <div class="upload-progress-panel {className}" transition:slide>
    <!-- Header -->
    <div class="panel-header">
      <div class="flex items-center gap-3">
        <h3 class="text-sm font-semibold text-gray-900 dark:text-white">
          {#if isComplete}
            Upload Complete
          {:else}
            Uploading {inProgressCount} of {uploads.length}
          {/if}
        </h3>
        {#if !isComplete}
          <div class="text-xs text-gray-500">{totalProgress}%</div>
        {/if}
      </div>

      <div class="flex items-center gap-2">
        {#if completedCount > 0}
          <span class="badge badge-success">{completedCount} done</span>
        {/if}
        {#if errorCount > 0}
          <span class="badge badge-error">{errorCount} failed</span>
        {/if}
        <button
          class="clear-btn"
          onclick={() => onClear?.()}
          title="Clear all"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Overall progress bar -->
    {#if !isComplete}
      <div class="overall-progress">
        <div class="progress-bar" style="width: {totalProgress}%"></div>
      </div>
    {/if}

    <!-- Upload items -->
    <div class="upload-list">
      {#each uploads as upload (upload.id)}
        <div class="upload-item" transition:slide>
          <!-- Thumbnail -->
          <div class="upload-thumbnail">
            {#if upload.result?.thumbnailUrl}
              <img src={upload.result.thumbnailUrl} alt="" class="w-full h-full object-cover" />
            {:else if upload.file.type.startsWith('image/')}
              <img src={getFileThumbnail(upload.file)} alt="" class="w-full h-full object-cover" />
            {:else}
              <svg class="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
            {/if}
          </div>

          <!-- Info -->
          <div class="upload-info">
            <div class="file-name">{upload.file.name}</div>
            <div class="file-meta">
              <span>{formatBytes(upload.file.size)}</span>
              <span class="status-dot {getStatusColor(upload.status)}"></span>
              <span class="status-text">{getStatusText(upload.status)}</span>
            </div>

            <!-- Progress bar for individual file -->
            {#if upload.status === 'uploading' || upload.status === 'processing'}
              <div class="file-progress">
                <div
                  class="file-progress-bar {upload.status === 'processing' ? 'animate-pulse' : ''}"
                  style="width: {upload.progress}%"
                ></div>
              </div>
            {/if}

            <!-- Error message -->
            {#if upload.status === 'error' && upload.error}
              <div class="error-message">{upload.error}</div>
            {/if}

            <!-- Optimization stats -->
            {#if showStats && upload.status === 'complete' && upload.result?.stats}
              <div class="optimization-stats" transition:fade>
                <span class="stat-original">{formatBytes(upload.result.stats.originalSize)}</span>
                <svg class="w-3 h-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
                <span class="stat-optimized">{formatBytes(upload.result.stats.optimizedSize)}</span>
                <span class="stat-savings">-{upload.result.stats.savingsPercent}%</span>
              </div>
            {/if}
          </div>

          <!-- Actions -->
          <div class="upload-actions">
            {#if upload.status === 'uploading' || upload.status === 'pending'}
              <button
                class="action-btn action-cancel"
                onclick={() => onCancel?.(upload.id)}
                title="Cancel"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            {:else if upload.status === 'error'}
              <button
                class="action-btn action-retry"
                onclick={() => onRetry?.(upload.id)}
                title="Retry"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </button>
            {:else if upload.status === 'complete'}
              <button
                class="action-btn action-remove"
                onclick={() => onRemove?.(upload.id)}
                title="Remove"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .upload-progress-panel {
    background-color: #fff;
    border-radius: 0.75rem;
    box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05);
    border: 1px solid rgb(229 231 235);
    overflow: hidden;
  }

  :global(.dark) .upload-progress-panel {
    background-color: rgb(31 41 55);
    border-color: rgb(55 65 81);
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid rgb(229 231 235);
  }

  :global(.dark) .panel-header {
    border-color: rgb(55 65 81);
  }

  .overall-progress {
    height: 0.25rem;
    background-color: rgb(229 231 235);
  }

  :global(.dark) .overall-progress {
    background-color: rgb(55 65 81);
  }

  .progress-bar {
    height: 100%;
    background-color: rgb(59 130 246);
    transition: all 0.3s;
  }

  .upload-list {
    max-height: 300px;
    overflow-y: auto;
  }

  .upload-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid rgb(243 244 246);
  }

  .upload-item:last-child {
    border-bottom: 0;
  }

  :global(.dark) .upload-item {
    border-color: rgb(55 65 81);
  }

  .upload-thumbnail {
    width: 3rem;
    height: 3rem;
    border-radius: 0.5rem;
    overflow: hidden;
    flex-shrink: 0;
    background-color: rgb(243 244 246);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.dark) .upload-thumbnail {
    background-color: rgb(55 65 81);
  }

  .upload-info {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-size: 0.875rem;
    font-weight: 500;
    color: rgb(17 24 39);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.dark) .file-name {
    color: #fff;
  }

  .file-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
    color: rgb(107 114 128);
  }

  :global(.dark) .file-meta {
    color: rgb(156 163 175);
  }

  .status-dot {
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 9999px;
  }

  .status-text {
    font-size: 0.75rem;
  }

  .file-progress {
    height: 0.25rem;
    background-color: rgb(229 231 235);
    border-radius: 9999px;
    margin-top: 0.5rem;
    overflow: hidden;
  }

  :global(.dark) .file-progress {
    background-color: rgb(55 65 81);
  }

  .file-progress-bar {
    height: 100%;
    background-color: rgb(59 130 246);
    transition: all 0.3s;
    border-radius: 9999px;
  }

  .error-message {
    font-size: 0.75rem;
    color: rgb(239 68 68);
    margin-top: 0.25rem;
  }

  .optimization-stats {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    margin-top: 0.25rem;
  }

  .stat-original {
    color: rgb(156 163 175);
    text-decoration: line-through;
  }

  .stat-optimized {
    color: rgb(75 85 99);
  }

  :global(.dark) .stat-optimized {
    color: rgb(209 213 219);
  }

  .stat-savings {
    color: rgb(34 197 94);
    font-weight: 500;
  }

  .upload-actions {
    flex-shrink: 0;
  }

  .action-btn {
    padding: 0.375rem;
    border-radius: 0.5rem;
    transition: color 0.15s, background-color 0.15s;
  }

  .action-cancel {
    color: rgb(156 163 175);
  }

  .action-cancel:hover {
    color: rgb(239 68 68);
    background-color: rgb(254 242 242);
  }

  :global(.dark) .action-cancel:hover {
    background-color: rgba(127, 29, 29, 0.2);
  }

  .action-retry {
    color: rgb(156 163 175);
  }

  .action-retry:hover {
    color: rgb(59 130 246);
    background-color: rgb(239 246 255);
  }

  :global(.dark) .action-retry:hover {
    background-color: rgba(30, 58, 138, 0.2);
  }

  .action-remove {
    color: rgb(34 197 94);
  }

  .action-remove:hover {
    color: rgb(22 163 74);
    background-color: rgb(240 253 244);
  }

  :global(.dark) .action-remove:hover {
    background-color: rgba(20, 83, 45, 0.2);
  }

  .clear-btn {
    padding: 0.375rem;
    color: rgb(156 163 175);
    border-radius: 0.5rem;
    transition: color 0.15s, background-color 0.15s;
  }

  .clear-btn:hover {
    color: rgb(75 85 99);
    background-color: rgb(243 244 246);
  }

  :global(.dark) .clear-btn:hover {
    color: rgb(209 213 219);
    background-color: rgb(55 65 81);
  }

  .badge {
    padding: 0.125rem 0.5rem;
    font-size: 0.75rem;
    font-weight: 500;
    border-radius: 9999px;
  }

  .badge-success {
    background-color: rgb(220 252 231);
    color: rgb(21 128 61);
  }

  :global(.dark) .badge-success {
    background-color: rgba(20, 83, 45, 0.3);
    color: rgb(74 222 128);
  }

  .badge-error {
    background-color: rgb(254 226 226);
    color: rgb(185 28 28);
  }

  :global(.dark) .badge-error {
    background-color: rgba(127, 29, 29, 0.3);
    color: rgb(248 113 113);
  }
</style>
