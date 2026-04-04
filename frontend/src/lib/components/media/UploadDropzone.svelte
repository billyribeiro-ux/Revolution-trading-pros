<!--
	Upload Dropzone Component
	═══════════════════════════════════════════════════════════════════════════
	
	Drag-and-drop file upload with progress tracking and chunked upload support.
-->

<script lang="ts">
	import { uploadStore } from '$lib/stores/media.svelte';
	import { Icon, IconAlertCircle, IconCheck, IconUpload, IconX } from '$lib/icons';

	interface Props {
		folderId?: string | null;
		accept?: string;
		maxSize?: number;
		multiple?: boolean;
	}

	let {
		folderId = null,
		accept = 'image/*,video/*,application/pdf',
		maxSize = 100 * 1024 * 1024,
		multiple = true
	}: Props = $props();

	let isDragging = $state(false);
	let fileInput: HTMLInputElement;

	// Local derived from getter
	const uploads = $derived(uploadStore.uploads);

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;

		const files = Array.from(e.dataTransfer?.files || []);
		handleFiles(files);
	}

	function handleFileSelect(e: Event) {
		const target = e.target as HTMLInputElement;
		const files = Array.from(target.files || []);
		handleFiles(files);
		target.value = ''; // Reset input
	}

	function handleFiles(files: File[]) {
		// Filter by size
		const validFiles = files.filter((file) => {
			if (file.size > maxSize) {
				alert(`File ${file.name} is too large. Maximum size is ${maxSize / 1024 / 1024}MB`);
				return false;
			}
			return true;
		});

		if (validFiles.length > 0) {
			uploadStore.uploadFiles(validFiles, {
				...(folderId && { folder_id: folderId }),
				optimize: true,
				generate_webp: true
			});
		}
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / 1024 / 1024).toFixed(1) + ' MB';
	}
</script>

<div class="upload-container">
	<!-- Dropzone -->
	<div
		class="dropzone"
		class:dragging={isDragging}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		role="button"
		tabindex="0"
		onclick={() => fileInput.click()}
		onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && fileInput.click()}
	>
		<Icon icon={IconUpload} size={48} class="upload-icon" />
		<h3 class="upload-title">Drop files here or click to browse</h3>
		<p class="upload-subtitle">
			Supports images, videos, and documents up to {maxSize / 1024 / 1024}MB
		</p>
		<input
			bind:this={fileInput}
			type="file"
			{accept}
			{multiple}
			onchange={handleFileSelect}
			class="hidden"
		/>
	</div>

	<!-- Upload Progress -->
	{#if uploads.size > 0}
		<div class="upload-list">
			<h4 class="upload-list-title">Uploading {uploads.size} file(s)</h4>
			{#each Array.from(uploads.entries()) as [id, upload]}
				<div class="upload-item">
					<div class="upload-item-info">
						<div class="upload-item-icon">
							{#if upload.status === 'complete'}
								<Icon icon={IconCheck} size={20} />
							{:else if upload.status === 'error'}
								<Icon icon={IconAlertCircle} size={20} />
							{:else}
								<div class="spinner-small"></div>
							{/if}
						</div>
						<div class="upload-item-details">
							<div class="upload-item-name">{upload.file.name}</div>
							<div class="upload-item-size">{formatFileSize(upload.file.size)}</div>
						</div>
					</div>

					<div class="upload-item-progress">
						{#if upload.status === 'uploading' || upload.status === 'processing'}
							<div class="progress-bar">
								<div class="progress-fill" style="width: {upload.progress}%"></div>
							</div>
							<span class="progress-text">{Math.round(upload.progress)}%</span>
						{:else if upload.status === 'complete'}
							<span class="status-text success">Complete</span>
						{:else if upload.status === 'error'}
							<span class="status-text error">{upload.error || 'Failed'}</span>
						{/if}
					</div>

					{#if upload.status === 'error' || upload.status === 'complete'}
						<button
							class="remove-btn"
							onclick={() => uploadStore.removeUpload(id)}
							aria-label="Remove"
						>
							<Icon icon={IconX} size={16} />
						</button>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.upload-container {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.dropzone {
		border: 2px dashed oklch(0.45 0.01 250);
		border-radius: var(--radius-xl);
		padding: var(--space-12);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		cursor: pointer;
		transition: border-color var(--duration-fast) var(--ease-default), background-color var(--duration-fast) var(--ease-default);

		&:hover {
			border-color: oklch(0.8 0.18 90);
			background-color: oklch(0.3 0.01 250 / 30%);
		}

		&.dragging {
			border-color: oklch(0.8 0.18 90);
			background-color: oklch(0.8 0.18 90 / 10%);
		}
	}

	.dropzone :global(.upload-icon) {
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-4);
	}

	.dropzone.dragging :global(.upload-icon) {
		color: oklch(0.8 0.18 90);
	}

	.upload-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-2);
	}

	.upload-subtitle {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.hidden {
		position: absolute;
		inline-size: 1px;
		block-size: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		border: 0;
	}

	.upload-list {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-4);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.upload-list-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-3);
	}

	.upload-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-block: var(--space-2);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
		&:last-child { border-block-end: none; }
	}

	.upload-item-info {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex: 1;
		min-inline-size: 0;
	}

	.upload-item-icon {
		flex-shrink: 0;
		color: oklch(0.7 0.18 160);

		&:has(:global(.text-red-400)) { color: oklch(0.7 0.2 25); }
	}

	.upload-item-details {
		flex: 1;
		min-inline-size: 0;
	}

	.upload-item-name {
		font-size: var(--text-sm);
		color: oklch(1 0 0);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.upload-item-size {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
	}

	.upload-item-progress {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.progress-bar {
		inline-size: 8rem;
		block-size: 0.5rem;
		background-color: oklch(0.38 0.01 250);
		border-radius: 9999px;
		overflow: hidden;
	}

	.progress-fill {
		block-size: 100%;
		background-color: oklch(0.8 0.18 90);
		transition: inline-size 300ms var(--ease-default);
	}

	.progress-text {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		inline-size: 2.5rem;
		text-align: end;
	}

	.status-text {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		&.success { color: oklch(0.7 0.18 160); }
		&.error { color: oklch(0.7 0.2 25); }
	}

	.remove-btn {
		padding: var(--space-1);
		color: oklch(0.65 0.01 250);
		border: none;
		cursor: pointer;
		transition: color var(--duration-fast) var(--ease-default);
		&:hover { color: oklch(1 0 0); }
	}

	.spinner-small {
		inline-size: 1.25rem;
		block-size: 1.25rem;
		border: 2px solid oklch(0.45 0.01 250);
		border-block-start-color: oklch(0.8 0.18 90);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin { to { transform: rotate(360deg); } }
</style>
