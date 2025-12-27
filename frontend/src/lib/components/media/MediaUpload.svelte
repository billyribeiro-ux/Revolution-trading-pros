<script lang="ts">
  /**
   * MediaUpload Component
   *
   * Drag-and-drop file upload with progress tracking
   * and optimization queue integration.
   */
  import { mediaApi, type MediaItem, type UploadOptions } from '$lib/api/media';

  interface Props {
    collection?: string;
    preset?: string;
    multiple?: boolean;
    accept?: string;
    maxFiles?: number;
    maxSizeMB?: number;
    processImmediately?: boolean;
    onupload?: (item: MediaItem) => void;
    onerror?: (data: { file: File; error: string }) => void;
    oncomplete?: (data: { uploaded: MediaItem[]; failed: { file: File; error: string }[] }) => void;
  }

  let { collection, preset, multiple = true, accept = 'image/*', maxFiles = 20, maxSizeMB = 50, processImmediately = false, onupload, onerror, oncomplete }: Props = $props();

  // State
  let isDragging = $state(false);
  let isUploading = $state(false);
  let uploadProgress: Map<string, number> = $state(new Map());
  let uploadQueue: File[] = $state([]);
  let fileInput: HTMLInputElement;

  let totalProgress = $derived(uploadQueue.length > 0
    ? Array.from(uploadProgress.values()).reduce((sum, p) => sum + p, 0) / uploadQueue.length
    : 0);

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (files) {
      handleFiles(Array.from(files));
    }
  }

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files) {
      handleFiles(Array.from(input.files));
    }
    input.value = '';
  }

  function handleFiles(files: File[]) {
    // Filter valid files
    const validFiles = files.filter((file) => {
      // Check file type
      if (accept !== '*' && !file.type.match(accept.replace('*', '.*'))) {
        onerror?.({ file, error: `Invalid file type: ${file.type}` });
        return false;
      }

      // Check file size
      if (file.size > maxSizeMB * 1024 * 1024) {
        onerror?.({ file, error: `File too large: ${(file.size / 1024 / 1024).toFixed(2)}MB (max ${maxSizeMB}MB)` });
        return false;
      }

      return true;
    });

    // Limit number of files
    const filesToUpload = validFiles.slice(0, maxFiles);

    if (filesToUpload.length > 0) {
      uploadFiles(filesToUpload);
    }
  }

  async function uploadFiles(files: File[]) {
    isUploading = true;
    uploadQueue = files;
    uploadProgress = new Map(files.map((f) => [f.name, 0]));

    const uploaded: MediaItem[] = [];
    const failed: { file: File; error: string }[] = [];

    for (const file of files) {
      try {
        const options: UploadOptions = {
          ...(collection && { collection }),
          ...(preset && { preset }),
          process_immediately: processImmediately,
        };

        // Simulate progress (real progress would need XHR)
        uploadProgress.set(file.name, 30);
        uploadProgress = uploadProgress;

        const result = await mediaApi.upload(file, options);

        uploadProgress.set(file.name, 100);
        uploadProgress = uploadProgress;

        uploaded.push(result.data);
        onupload?.(result.data);
      } catch (error: any) {
        const message = error?.response?.data?.message || error?.message || 'Upload failed';
        failed.push({ file, error: message });
        onerror?.({ file, error: message });
        uploadProgress.set(file.name, -1);
        uploadProgress = uploadProgress;
      }
    }

    oncomplete?.({ uploaded, failed });
    isUploading = false;
    uploadQueue = [];
    uploadProgress = new Map();
  }

  function openFilePicker() {
    fileInput?.click();
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1024 / 1024).toFixed(2) + ' MB';
  }
</script>

<div class="media-upload">
  <!-- Hidden file input -->
  <input
    type="file"
    bind:this={fileInput}
    onchange={handleFileSelect}
    {accept}
    multiple={multiple}
    class="hidden"
  />

  <!-- Drop zone -->
  <div
    class="drop-zone"
    class:dragging={isDragging}
    class:uploading={isUploading}
    role="button"
    tabindex="0"
    onclick={openFilePicker}
    onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && openFilePicker()}
    ondragenter={handleDragEnter}
    ondragleave={handleDragLeave}
    ondragover={handleDragOver}
    ondrop={handleDrop}
  >
    {#if isUploading}
      <!-- Upload progress -->
      <div class="upload-progress">
        <div class="progress-icon">
          <svg class="animate-spin" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="10" stroke-width="2" stroke-dasharray="32" stroke-dashoffset="32" />
          </svg>
        </div>
        <div class="progress-text">
          <span class="progress-percent">{Math.round(totalProgress)}%</span>
          <span class="progress-label">Uploading {uploadQueue.length} file{uploadQueue.length !== 1 ? 's' : ''}...</span>
        </div>
        <div class="progress-bar">
          <div class="progress-fill" style="width: {totalProgress}%"></div>
        </div>

        <!-- File list -->
        <div class="file-list">
          {#each uploadQueue as file}
            {@const progress = uploadProgress.get(file.name) || 0}
            <div class="file-item" class:error={progress < 0}>
              <span class="file-name">{file.name}</span>
              <span class="file-size">{formatBytes(file.size)}</span>
              {#if progress >= 0}
                <span class="file-progress">{progress}%</span>
              {:else}
                <span class="file-error">Failed</span>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <!-- Default state -->
      <div class="drop-content">
        <div class="drop-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
          </svg>
        </div>
        <div class="drop-text">
          <span class="drop-title">
            {isDragging ? 'Drop files here' : 'Drag & drop files here'}
          </span>
          <span class="drop-subtitle">or click to browse</span>
        </div>
        <div class="drop-info">
          <span>Supported: JPEG, PNG, GIF, WebP</span>
          <span>Max size: {maxSizeMB}MB per file</span>
          {#if multiple}
            <span>Up to {maxFiles} files at once</span>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .media-upload {
    width: 100%;
  }

  .hidden {
    display: none;
  }

  .drop-zone {
    border: 2px dashed var(--border-color, #d1d5db);
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.2s ease;
    background: var(--bg-secondary, #f9fafb);
  }

  .drop-zone:hover,
  .drop-zone.dragging {
    border-color: var(--primary-color, #3b82f6);
    background: var(--bg-hover, #eff6ff);
  }

  .drop-zone.uploading {
    cursor: default;
    border-style: solid;
    border-color: var(--primary-color, #3b82f6);
  }

  .drop-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .drop-icon {
    color: var(--text-muted, #9ca3af);
  }

  .drop-zone:hover .drop-icon,
  .drop-zone.dragging .drop-icon {
    color: var(--primary-color, #3b82f6);
  }

  .drop-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .drop-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
  }

  .drop-subtitle {
    font-size: 0.875rem;
    color: var(--text-muted, #6b7280);
  }

  .drop-info {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.5rem 1rem;
    font-size: 0.75rem;
    color: var(--text-muted, #9ca3af);
  }

  .upload-progress {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
  }

  .progress-icon {
    color: var(--primary-color, #3b82f6);
  }

  .progress-text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .progress-percent {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--primary-color, #3b82f6);
  }

  .progress-label {
    font-size: 0.875rem;
    color: var(--text-muted, #6b7280);
  }

  .progress-bar {
    width: 100%;
    max-width: 300px;
    height: 8px;
    background: var(--bg-tertiary, #e5e7eb);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--primary-color, #3b82f6);
    transition: width 0.3s ease;
  }

  .file-list {
    width: 100%;
    max-width: 400px;
    max-height: 200px;
    overflow-y: auto;
    margin-top: 0.5rem;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    font-size: 0.75rem;
    background: var(--bg-primary, white);
    border-radius: 4px;
    margin-bottom: 0.25rem;
  }

  .file-item.error {
    background: var(--bg-error, #fef2f2);
  }

  .file-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-primary, #111827);
  }

  .file-size {
    color: var(--text-muted, #9ca3af);
  }

  .file-progress {
    color: var(--primary-color, #3b82f6);
    font-weight: 500;
  }

  .file-error {
    color: var(--error-color, #ef4444);
    font-weight: 500;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
