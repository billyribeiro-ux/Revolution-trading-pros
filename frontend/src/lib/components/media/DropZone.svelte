<!--
  DropZone Component
  ═══════════════════════════════════════════════════════════════════════════

  Drag-and-drop file upload zone with:
  - Visual feedback on drag over
  - File type validation
  - Multiple file support
  - Click to browse fallback
  - Preview of dropped files

  @version 1.0.0
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  // Props
  export let accept: string = 'image/*';
  export let multiple: boolean = true;
  export let maxFiles: number = 10;
  export let maxSize: number = 50 * 1024 * 1024; // 50MB
  export let disabled: boolean = false;
  export let className: string = '';

  // State
  let isDragOver = false;
  let fileInput: HTMLInputElement;
  let dragCounter = 0;

  const dispatch = createEventDispatcher<{
    files: File[];
    error: { message: string; files?: File[] };
  }>();

  // Validate files
  function validateFiles(files: File[]): { valid: File[]; errors: string[] } {
    const valid: File[] = [];
    const errors: string[] = [];

    for (const file of files) {
      // Check file type
      if (accept !== '*' && !matchesAccept(file, accept)) {
        errors.push(`${file.name}: Invalid file type`);
        continue;
      }

      // Check file size
      if (file.size > maxSize) {
        errors.push(`${file.name}: File too large (max ${formatBytes(maxSize)})`);
        continue;
      }

      valid.push(file);
    }

    // Check max files
    if (valid.length > maxFiles) {
      errors.push(`Too many files. Maximum ${maxFiles} allowed.`);
      return { valid: valid.slice(0, maxFiles), errors };
    }

    return { valid, errors };
  }

  // Check if file matches accept pattern
  function matchesAccept(file: File, accept: string): boolean {
    const acceptTypes = accept.split(',').map((t) => t.trim());

    for (const type of acceptTypes) {
      if (type.startsWith('.')) {
        // Extension match
        if (file.name.toLowerCase().endsWith(type.toLowerCase())) return true;
      } else if (type.endsWith('/*')) {
        // MIME type wildcard
        const baseType = type.slice(0, -2);
        if (file.type.startsWith(baseType)) return true;
      } else {
        // Exact MIME type match
        if (file.type === type) return true;
      }
    }

    return false;
  }

  // Format bytes to human readable
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  // Handle drag events
  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    dragCounter++;
    if (e.dataTransfer?.types.includes('Files')) {
      isDragOver = true;
    }
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    dragCounter--;
    if (dragCounter === 0) {
      isDragOver = false;
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragOver = false;
    dragCounter = 0;

    if (disabled) return;

    const files = Array.from(e.dataTransfer?.files || []);
    processFiles(files);
  }

  // Handle file input change
  function handleFileInput(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files || []);
    processFiles(files);
    input.value = ''; // Reset input
  }

  // Process dropped/selected files
  function processFiles(files: File[]) {
    if (files.length === 0) return;

    const { valid, errors } = validateFiles(files);

    if (errors.length > 0) {
      dispatch('error', { message: errors.join('\n'), files: valid });
    }

    if (valid.length > 0) {
      dispatch('files', valid);
    }
  }

  // Click to open file dialog
  function openFileDialog() {
    if (!disabled) {
      fileInput?.click();
    }
  }
</script>

<div
  class="dropzone relative {className}"
  class:dropzone-active={isDragOver}
  class:dropzone-disabled={disabled}
  role="button"
  tabindex={disabled ? -1 : 0}
  on:click={openFileDialog}
  on:keydown={(e) => e.key === 'Enter' && openFileDialog()}
  on:dragenter={handleDragEnter}
  on:dragleave={handleDragLeave}
  on:dragover={handleDragOver}
  on:drop={handleDrop}
>
  <input
    bind:this={fileInput}
    type="file"
    {accept}
    {multiple}
    {disabled}
    on:change={handleFileInput}
    class="hidden"
  />

  <div class="dropzone-content">
    {#if isDragOver}
      <!-- Drop active state -->
      <div class="drop-indicator">
        <svg class="w-16 h-16 text-blue-500 animate-bounce" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3-3m0 0l3 3m-3-3v12" />
        </svg>
        <p class="text-lg font-medium text-blue-600">Drop files here</p>
      </div>
    {:else}
      <!-- Default state -->
      <slot>
        <div class="default-content">
          <svg class="w-12 h-12 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
          <p class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-1">
            Drag and drop images here
          </p>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
            or click to browse
          </p>
          <div class="text-xs text-gray-400 dark:text-gray-500 space-y-1">
            <p>Supported: JPG, PNG, WebP, GIF, AVIF</p>
            <p>Max size: {formatBytes(maxSize)} per file</p>
            {#if multiple}
              <p>Up to {maxFiles} files at once</p>
            {/if}
          </div>
        </div>
      </slot>
    {/if}
  </div>
</div>

<style>
  @reference "tailwindcss";

  .dropzone {
    @apply border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-xl p-8;
    @apply bg-gray-50 dark:bg-gray-800/50;
    @apply cursor-pointer transition-all duration-200;
    @apply flex items-center justify-center min-h-[200px];
  }

  .dropzone:hover:not(.dropzone-disabled) {
    @apply border-blue-400 dark:border-blue-500 bg-blue-50/50 dark:bg-blue-900/10;
  }

  .dropzone:focus:not(.dropzone-disabled) {
    @apply outline-none ring-2 ring-blue-500 ring-offset-2 dark:ring-offset-gray-900;
  }

  .dropzone-active {
    @apply border-blue-500 bg-blue-50 dark:bg-blue-900/20 scale-[1.02];
    @apply border-solid;
  }

  .dropzone-disabled {
    @apply opacity-50 cursor-not-allowed;
  }

  .dropzone-content {
    @apply text-center;
  }

  .drop-indicator {
    @apply flex flex-col items-center justify-center;
  }

  .default-content {
    @apply flex flex-col items-center;
  }
</style>
