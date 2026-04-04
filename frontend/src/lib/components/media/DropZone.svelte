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
	class="dz-zone {className}"
	data-active={isDragOver || undefined}
	data-disabled={disabled || undefined}
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
		class="dz-input"
	/>

	<div class="dz-content">
		{#if isDragOver}
			<!-- Drop active state -->
			<div class="dz-indicator">
				<svg class="dz-icon-lg dz-icon-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3-3m0 0l3 3m-3-3v12" />
				</svg>
				<p class="dz-drop-text">Drop files here</p>
			</div>
		{:else}
			<!-- Default state -->
			{#if children}
				{@render children()}
			{:else}
				<div class="dz-default">
					<svg class="dz-icon-md dz-icon-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
					</svg>
					<p class="dz-heading">Drag and drop images here</p>
					<p class="dz-subtext">or click to browse</p>
					<div class="dz-hints">
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
	.dz-zone {
		position: relative;
		border: 2px dashed oklch(0.85 0.005 265);
		border-radius: var(--radius-xl);
		padding: var(--space-8);
		background-color: oklch(0.98 0.002 265);
		cursor: pointer;
		transition: all 200ms var(--ease-default);
		display: flex;
		align-items: center;
		justify-content: center;
		min-block-size: 200px;

		&:hover:not([data-disabled]) {
			border-color: oklch(0.7 0.15 260);
			background-color: oklch(0.97 0.02 260 / 50%);
		}

		&:focus:not([data-disabled]) {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.6 0.2 260), 0 0 0 4px oklch(1 0 0);
		}

		&[data-active] {
			border-color: oklch(0.6 0.2 260);
			border-style: solid;
			background-color: oklch(0.96 0.03 260);
			transform: scale(1.02);
		}

		&[data-disabled] {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}

	.dz-input {
		position: absolute;
		inline-size: 1px;
		block-size: 1px;
		overflow: hidden;
		clip: rect(0 0 0 0);
		white-space: nowrap;
		border: 0;
	}

	.dz-content { text-align: center; }

	.dz-indicator {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.dz-icon-lg {
		inline-size: 4rem;
		block-size: 4rem;
		animation: bounce 1s infinite;
	}

	.dz-icon-md {
		inline-size: 3rem;
		block-size: 3rem;
		margin-block-end: var(--space-4);
	}

	.dz-icon-accent { color: oklch(0.6 0.2 260); }
	.dz-icon-muted { color: oklch(0.65 0.01 265); }

	.dz-drop-text {
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(0.5 0.18 260);
	}

	.dz-default {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.dz-heading {
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(0.35 0.01 265);
		margin-block-end: var(--space-1);
	}

	.dz-subtext {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		margin-block-end: var(--space-4);
	}

	.dz-hints {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 265);
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	@keyframes bounce {
		0%, 100% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); }
		50% { transform: translateY(0); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); }
	}
</style>
