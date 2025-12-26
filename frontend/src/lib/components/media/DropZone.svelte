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
  import type { Snippet } from 'svelte';

  // Svelte 5 Props with callback pattern
  interface Props {
    accept?: string;
    multiple?: boolean;
    maxFiles?: number;
    maxSize?: number;
    disabled?: boolean;
    className?: string;
    children?: Snippet;
    onfiles?: (files: File[]) => void;
    onerror?: (error: { message: string; files?: File[] }) => void;
  }

  let {
    accept = 'image/*',
    multiple = true,
    maxFiles = 10,
    maxSize = 50 * 1024 * 1024, // 50MB
    disabled = false,
    className = '',
    children,
    onfiles,
    onerror
  }: Props = $props();

  // Svelte 5 State
  let isDragOver = $state(false);
  let fileInput = $state<HTMLInputElement | null>(null);
  let dragCounter = $state(0);

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
      onerror?.({ message: errors.join('\n'), files: valid });
    }

    if (valid.length > 0) {
      onfiles?.(valid);
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
  onclick={openFileDialog}
  onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && openFileDialog()}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondragover={handleDragOver}
  ondrop={handleDrop}
>
  <input
    bind:this={fileInput}
    type="file"
    {accept}
    {multiple}
    {disabled}
    onchange={handleFileInput}
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
      {#if children}
        {@render children()}
      {:else}
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
      {/if}
    {/if}
  </div>
</div>

<style>
  .dropzone {
    border: 2px dashed #d1d5db;
    border-radius: 0.75rem;
    padding: 2rem;
    background-color: #f9fafb;
    cursor: pointer;
    transition: all 200ms ease;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  :global(.dark) .dropzone {
    border-color: #4b5563;
    background-color: rgba(31, 41, 55, 0.5);
  }

  .dropzone:hover:not(.dropzone-disabled) {
    border-color: #60a5fa;
    background-color: rgba(239, 246, 255, 0.5);
  }

  :global(.dark) .dropzone:hover:not(.dropzone-disabled) {
    border-color: #3b82f6;
    background-color: rgba(30, 58, 138, 0.1);
  }

  .dropzone:focus:not(.dropzone-disabled) {
    outline: none;
    box-shadow: 0 0 0 2px #3b82f6, 0 0 0 4px white;
  }

  :global(.dark) .dropzone:focus:not(.dropzone-disabled) {
    box-shadow: 0 0 0 2px #3b82f6, 0 0 0 4px #111827;
  }

  .dropzone-active {
    border-color: #3b82f6;
    border-style: solid;
    background-color: #eff6ff;
    transform: scale(1.02);
  }

  :global(.dark) .dropzone-active {
    background-color: rgba(30, 58, 138, 0.2);
  }

  .dropzone-disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .dropzone-content {
    text-align: center;
  }

  .drop-indicator {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .default-content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
