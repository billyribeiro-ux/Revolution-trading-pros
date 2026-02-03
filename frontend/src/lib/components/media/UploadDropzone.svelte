<!--
	Upload Dropzone Component
	═══════════════════════════════════════════════════════════════════════════
	
	Drag-and-drop file upload with progress tracking and chunked upload support.
-->

<script lang="ts">
	import { uploadStore } from '$lib/stores/media.svelte';
	import { IconUpload, IconX, IconCheck, IconAlertCircle } from '$lib/icons';

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
		<IconUpload size={48} class="upload-icon" />
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
								<IconCheck size={20} class="text-green-400" />
							{:else if upload.status === 'error'}
								<IconAlertCircle size={20} class="text-red-400" />
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
							<IconX size={16} />
						</button>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.upload-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.dropzone {
		@apply border-2 border-dashed border-gray-600 rounded-xl p-12;
		@apply flex flex-col items-center justify-center text-center;
		@apply cursor-pointer transition-all;
		@apply hover:border-yellow-500 hover:bg-gray-800/30;
	}

	.dropzone.dragging {
		@apply border-yellow-500 bg-yellow-500/10;
	}

	.dropzone :global(.upload-icon) {
		@apply text-gray-400 mb-4;
	}

	.dropzone.dragging :global(.upload-icon) {
		@apply text-yellow-400;
	}

	.upload-title {
		@apply text-xl font-semibold text-white mb-2;
	}

	.upload-subtitle {
		@apply text-sm text-gray-400;
	}

	.hidden {
		@apply sr-only;
	}

	.upload-list {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.upload-list-title {
		@apply text-sm font-semibold text-white mb-3;
	}

	.upload-item {
		@apply flex items-center gap-3 py-2 border-b border-gray-700/50 last:border-0;
	}

	.upload-item-info {
		@apply flex items-center gap-3 flex-1 min-w-0;
	}

	.upload-item-icon {
		@apply flex-shrink-0;
	}

	.upload-item-details {
		@apply flex-1 min-w-0;
	}

	.upload-item-name {
		@apply text-sm text-white truncate;
	}

	.upload-item-size {
		@apply text-xs text-gray-400;
	}

	.upload-item-progress {
		@apply flex items-center gap-2;
	}

	.progress-bar {
		@apply w-32 h-2 bg-gray-700 rounded-full overflow-hidden;
	}

	.progress-fill {
		@apply h-full bg-yellow-500 transition-all duration-300;
	}

	.progress-text {
		@apply text-xs text-gray-400 w-10 text-right;
	}

	.status-text {
		@apply text-xs font-medium;
	}

	.status-text.success {
		@apply text-green-400;
	}

	.status-text.error {
		@apply text-red-400;
	}

	.remove-btn {
		@apply p-1 text-gray-400 hover:text-white transition-colors;
	}

	.spinner-small {
		@apply w-5 h-5 border-2 border-gray-600 border-t-yellow-400 rounded-full animate-spin;
	}
</style>
