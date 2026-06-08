<!--
	Upload Dropzone Component
	═══════════════════════════════════════════════════════════════════════════
	
	Drag-and-drop file upload with progress tracking and chunked upload support.
-->

<script lang="ts">
	import { uploadStore } from '$lib/stores/media.svelte';
	import { IconUpload, IconX, IconCheck, IconAlertCircle } from '$lib/icons';
	import { toastStore } from '$lib/stores/toast.svelte';

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
	let fileInput: HTMLInputElement | undefined;

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
				toastStore.error(
					`File ${file.name} is too large. Maximum size is ${maxSize / 1024 / 1024}MB`
				);
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

	function captureFileInput(node: HTMLInputElement) {
		fileInput = node;

		return () => {
			if (fileInput === node) {
				fileInput = undefined;
			}
		};
	}
</script>

<div class="upload-container">
	<!-- Dropzone -->
	<div
		class={['dropzone', { dragging: isDragging }]}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		role="button"
		tabindex="0"
		onclick={() => fileInput?.click()}
		onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && fileInput?.click()}
	>
		<IconUpload size={48} class="upload-icon" />
		<h3 class="upload-title">Drop files here or click to browse</h3>
		<p class="upload-subtitle">
			Supports images, videos, and documents up to {maxSize / 1024 / 1024}MB
		</p>
		<input
			type="file"
			{accept}
			{multiple}
			onchange={handleFileSelect}
			class="hidden"
			{@attach captureFileInput}
		/>
	</div>

	<!-- Upload Progress -->
	{#if uploads.size > 0}
		<div class="upload-list">
			<h4 class="upload-list-title">Uploading {uploads.size} file(s)</h4>
			{#each Array.from(uploads.entries()) as [id, upload] (id)}
				<div class="upload-item">
					<div class="upload-item-info">
						<div class="upload-item-icon">
							{#if upload.status === 'complete'}
								<IconCheck size={20} class="upload-status-icon upload-status-icon--success" />
							{:else if upload.status === 'error'}
								<IconAlertCircle size={20} class="upload-status-icon upload-status-icon--error" />
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
								<div class="progress-fill" style:width={`${upload.progress}%`}></div>
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

<style>
	.upload-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.dropzone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		border: 2px dashed #4b5563;
		border-radius: 0.75rem;
		cursor: pointer;
		text-align: center;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease;
	}

	.dropzone:hover {
		border-color: #eab308;
		background: rgba(31, 41, 55, 0.3);
	}

	.dropzone.dragging {
		border-color: #eab308;
		background: rgba(234, 179, 8, 0.1);
	}

	.dropzone :global(.upload-icon) {
		margin-bottom: 1rem;
		color: #9ca3af;
	}

	.dropzone.dragging :global(.upload-icon) {
		color: #facc15;
	}

	.upload-title {
		margin: 0 0 0.5rem;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.upload-subtitle {
		margin: 0;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	.upload-list {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.upload-list-title {
		margin: 0 0 0.75rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #ffffff;
	}

	.upload-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding-block: 0.5rem;
		border-bottom: 1px solid rgba(55, 65, 81, 0.5);
	}

	.upload-item:last-child {
		border-bottom: 0;
	}

	.upload-item-info {
		display: flex;
		align-items: center;
		flex: 1;
		gap: 0.75rem;
		min-width: 0;
	}

	.upload-item-icon {
		flex-shrink: 0;
	}

	.upload-item-icon :global(.upload-status-icon--success) {
		color: #4ade80;
	}

	.upload-item-icon :global(.upload-status-icon--error) {
		color: #f87171;
	}

	.upload-item-details {
		flex: 1;
		min-width: 0;
	}

	.upload-item-name {
		overflow: hidden;
		color: #ffffff;
		font-size: 0.875rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.upload-item-size {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.upload-item-progress {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.progress-bar {
		width: 8rem;
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: #374151;
	}

	.progress-fill {
		height: 100%;
		background: #eab308;
		transition: width 0.3s ease;
	}

	.progress-text {
		width: 2.5rem;
		color: #9ca3af;
		font-size: 0.75rem;
		text-align: right;
	}

	.status-text {
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-text.success {
		color: #4ade80;
	}

	.status-text.error {
		color: #f87171;
	}

	.remove-btn {
		padding: 0.25rem;
		border: 0;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.remove-btn:hover {
		color: #ffffff;
	}

	.spinner-small {
		width: 1.25rem;
		height: 1.25rem;
		border: 2px solid #4b5563;
		border-top-color: #facc15;
		border-radius: 999px;
		animation: upload-spin 1s linear infinite;
	}

	@media (max-width: 640px) {
		.dropzone {
			padding: 2rem 1rem;
		}

		.upload-item {
			align-items: flex-start;
			flex-direction: column;
		}

		.upload-item-progress {
			width: 100%;
		}

		.progress-bar {
			flex: 1;
			width: auto;
		}
	}

	@keyframes upload-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
