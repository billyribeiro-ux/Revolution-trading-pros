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
  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 overflow-hidden {className}" transition:slide>
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700">
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
          <span class="px-2 py-0.5 text-xs font-medium rounded-full bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400">{completedCount} done</span>
        {/if}
        {#if errorCount > 0}
          <span class="px-2 py-0.5 text-xs font-medium rounded-full bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400">{errorCount} failed</span>
        {/if}
        <button
          class="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
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
      <div class="h-1 bg-gray-200 dark:bg-gray-700">
        <div class="h-full bg-blue-500 transition-all duration-300" style="width: {totalProgress}%"></div>
      </div>
    {/if}

    <!-- Upload items -->
    <div class="max-h-[300px] overflow-y-auto">
      {#each uploads as upload (upload.id)}
        <div class="flex items-center gap-3 px-4 py-3 border-b border-gray-100 dark:border-gray-700 last:border-b-0" transition:slide>
          <!-- Thumbnail -->
          <div class="w-12 h-12 rounded-lg overflow-hidden flex-shrink-0 bg-gray-100 dark:bg-gray-700 flex items-center justify-center">
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
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-white truncate">{upload.file.name}</div>
            <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
              <span>{formatBytes(upload.file.size)}</span>
              <span class="w-2 h-2 rounded-full {getStatusColor(upload.status)}"></span>
              <span class="text-xs">{getStatusText(upload.status)}</span>
            </div>

            <!-- Progress bar for individual file -->
            {#if upload.status === 'uploading' || upload.status === 'processing'}
              <div class="h-1 bg-gray-200 dark:bg-gray-700 rounded-full mt-2 overflow-hidden">
                <div
                  class="h-full bg-blue-500 transition-all duration-300 rounded-full {upload.status === 'processing' ? 'animate-pulse' : ''}"
                  style="width: {upload.progress}%"
                ></div>
              </div>
            {/if}

            <!-- Error message -->
            {#if upload.status === 'error' && upload.error}
              <div class="text-xs text-red-500 mt-1">{upload.error}</div>
            {/if}

            <!-- Optimization stats -->
            {#if showStats && upload.status === 'complete' && upload.result?.stats}
              <div class="flex items-center gap-1 text-xs mt-1" transition:fade>
                <span class="text-gray-400 line-through">{formatBytes(upload.result.stats.originalSize)}</span>
                <svg class="w-3 h-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
                <span class="text-gray-600 dark:text-gray-300">{formatBytes(upload.result.stats.optimizedSize)}</span>
                <span class="text-green-500 font-medium">-{upload.result.stats.savingsPercent}%</span>
              </div>
            {/if}
          </div>

          <!-- Actions -->
          <div class="flex-shrink-0">
            {#if upload.status === 'uploading' || upload.status === 'pending'}
              <button
                class="p-1.5 rounded-lg transition-colors text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20"
                onclick={() => onCancel?.(upload.id)}
                title="Cancel"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            {:else if upload.status === 'error'}
              <button
                class="p-1.5 rounded-lg transition-colors text-gray-400 hover:text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20"
                onclick={() => onRetry?.(upload.id)}
                title="Retry"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </button>
            {:else if upload.status === 'complete'}
              <button
                class="p-1.5 rounded-lg transition-colors text-green-500 hover:text-green-600 hover:bg-green-50 dark:hover:bg-green-900/20"
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
