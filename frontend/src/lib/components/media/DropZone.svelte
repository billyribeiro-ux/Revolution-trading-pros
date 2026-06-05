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
	import Icon from '$lib/components/Icon.svelte';

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

	function captureFileInput(element: HTMLInputElement) {
		fileInput = element;

		return () => {
			if (fileInput === element) {
				fileInput = null;
			}
		};
	}

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
	class={['dropzone', className]}
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
		type="file"
		{accept}
		{multiple}
		{disabled}
		onchange={handleFileInput}
		class="dropzone__input"
		{@attach captureFileInput}
	/>

	<div class="dropzone-content">
		{#if isDragOver}
			<!-- Drop active state -->
			<div class="drop-indicator">
				<Icon name="IconCloudUpload" size={64} class="dropzone__upload-icon" />
				<p class="dropzone__drop-label">Drop files here</p>
			</div>
		{:else}
			<!-- Default state -->
			{#if children}
				{@render children()}
			{:else}
				<div class="default-content">
					<Icon name="IconPhoto" size={48} class="dropzone__photo-icon" />
					<p class="dropzone__title">Drag and drop images here</p>
					<p class="dropzone__subtitle">or click to browse</p>
					<div class="dropzone__requirements">
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
		position: relative;
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

	.dropzone__input {
		display: none;
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
		box-shadow:
			0 0 0 2px #3b82f6,
			0 0 0 4px white;
	}

	:global(.dark) .dropzone:focus:not(.dropzone-disabled) {
		box-shadow:
			0 0 0 2px #3b82f6,
			0 0 0 4px #111827;
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

	.dropzone :global(.dropzone__upload-icon) {
		color: #3b82f6;
		animation: dropzone-bounce 1s infinite;
	}

	.dropzone :global(.dropzone__photo-icon) {
		margin-bottom: 1rem;
		color: #9ca3af;
	}

	.dropzone__drop-label,
	.dropzone__title {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 500;
		line-height: 1.75rem;
	}

	.dropzone__drop-label {
		color: #2563eb;
	}

	.dropzone__title {
		margin-bottom: 0.25rem;
		color: #374151;
	}

	:global(.dark) .dropzone__title {
		color: #d1d5db;
	}

	.dropzone__subtitle {
		margin: 0 0 1rem;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	:global(.dark) .dropzone__subtitle {
		color: #9ca3af;
	}

	.dropzone__requirements {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		color: #9ca3af;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.dropzone__requirements p {
		margin: 0;
	}

	:global(.dark) .dropzone__requirements {
		color: #6b7280;
	}

	@keyframes dropzone-bounce {
		0%,
		100% {
			transform: translateY(-25%);
			animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
		}

		50% {
			transform: none;
			animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
		}
	}
</style>
