<script lang="ts">
	/**
	 * Bulk Upload Queue - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { bulkUploadApi, type BatchStatus, type UploadQueueItem } from '$lib/api/video-advanced';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconLoader from '@tabler/icons-svelte-runes/icons/loader-2';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconFile from '@tabler/icons-svelte-runes/icons/file';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';

	interface Props {
		contentType?: string;
		traderId?: number | null;
		roomIds?: number[];
		uploadToAll?: boolean;
		tags?: string[];
		onComplete?: () => void;
		onClose?: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const contentType = $derived(props.contentType ?? 'daily_video');
	const traderId = $derived(props.traderId ?? null);
	const roomIds = $derived(props.roomIds ?? []);
	const uploadToAll = $derived(props.uploadToAll ?? false);
	const tags = $derived(props.tags ?? []);
	const onComplete = $derived(props.onComplete);
	const onClose = $derived(props.onClose);

	let files = $state<File[]>([]);
	let batchId = $state<string | null>(null);
	let batchStatus = $state<BatchStatus | null>(null);
	let isInitializing = $state(false);
	let error = $state('');
	let isDragging = $state(false);

	// TUS upload progress tracking
	let uploadProgress = $state<Record<number, number>>({});

	let fileInput = $state<HTMLInputElement | null>(null);
	let pollInterval: ReturnType<typeof setInterval> | null = null;

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

		const droppedFiles = Array.from(e.dataTransfer?.files || []);
		addFiles(droppedFiles);
	}

	function handleFileSelect(e: Event) {
		const input = e.target as HTMLInputElement;
		const selectedFiles = Array.from(input.files || []);
		addFiles(selectedFiles);
	}

	function addFiles(newFiles: File[]) {
		const validFiles = newFiles.filter((file) => {
			const validTypes = [
				'video/mp4',
				'video/webm',
				'video/quicktime',
				'video/x-msvideo',
				'video/x-matroska'
			];
			return validTypes.includes(file.type) || file.name.match(/\.(mp4|webm|mov|avi|mkv)$/i);
		});

		if (validFiles.length !== newFiles.length) {
			error = 'Some files were skipped. Only video files are allowed.';
		}

		files = [...files, ...validFiles];
	}

	function removeFile(index: number) {
		files = files.filter((_, i) => i !== index);
	}

	async function startBulkUpload() {
		if (files.length === 0) {
			error = 'Please select files to upload';
			return;
		}

		isInitializing = true;
		error = '';

		const result = await bulkUploadApi.init({
			files: files.map((f) => ({
				filename: f.name,
				file_size_bytes: f.size,
				content_type: f.type,
				title: f.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' ')
			})),
			default_metadata: {
				content_type: contentType,
				video_date: new Date().toISOString().split('T')[0],
				trader_id: traderId || undefined,
				room_ids: roomIds.length > 0 ? roomIds : undefined,
				upload_to_all: uploadToAll,
				is_published: false,
				tags: tags.length > 0 ? tags : undefined
			}
		});

		if (result.success && result.data) {
			batchId = result.data.batch_id;
			// Start uploading files via TUS
			startTusUploads(result.data.uploads);
			// Start polling for status
			startPolling();
		} else {
			error = result.error || 'Failed to initialize bulk upload';
		}

		isInitializing = false;
	}

	async function startTusUploads(uploads: UploadQueueItem[]) {
		for (let i = 0; i < uploads.length; i++) {
			const upload = uploads[i];
			const file = files[i];

			if (!file) continue;

			try {
				await uploadViaTus(file, upload);
			} catch (err) {
				console.error(`Failed to upload ${file.name}:`, err);
				await bulkUploadApi.updateItemStatus(upload.id, {
					status: 'failed',
					error_message: err instanceof Error ? err.message : 'Upload failed'
				});
			}
		}
	}

	async function uploadViaTus(file: File, queueItem: UploadQueueItem) {
		return new Promise<void>((resolve, reject) => {
			const xhr = new XMLHttpRequest();

			xhr.upload.addEventListener('progress', (e) => {
				if (e.lengthComputable) {
					const percent = Math.round((e.loaded / e.total) * 100);
					uploadProgress[queueItem.id] = percent;
					uploadProgress = { ...uploadProgress };
				}
			});

			xhr.addEventListener('load', async () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					await bulkUploadApi.updateItemStatus(queueItem.id, {
						status: 'completed',
						progress_percent: 100
					});
					resolve();
				} else {
					reject(new Error(`Upload failed with status ${xhr.status}`));
				}
			});

			xhr.addEventListener('error', () => {
				reject(new Error('Network error during upload'));
			});

			xhr.open('PUT', queueItem.upload_url);
			xhr.setRequestHeader('Content-Type', 'application/octet-stream');
			xhr.send(file);
		});
	}

	function startPolling() {
		if (pollInterval) clearInterval(pollInterval);

		pollInterval = setInterval(async () => {
			if (!batchId) return;

			const result = await bulkUploadApi.getBatchStatus(batchId);

			if (result.success && result.data) {
				batchStatus = result.data;

				// Check if all uploads are complete
				if (result.data.completed + result.data.failed === result.data.total_files) {
					stopPolling();
					if (onComplete) onComplete();
				}
			}
		}, 2000);
	}

	function stopPolling() {
		if (pollInterval) {
			clearInterval(pollInterval);
			pollInterval = null;
		}
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
		return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'completed':
				return IconCheck;
			case 'failed':
				return IconAlertCircle;
			case 'uploading':
			case 'processing':
				return IconLoader;
			default:
				return IconFile;
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return '#22c55e';
			case 'failed':
				return '#ef4444';
			case 'uploading':
			case 'processing':
				return '#f59e0b';
			default:
				return '#6b7280';
		}
	}

	$effect(() => {
		return () => {
			stopPolling();
		};
	});
</script>

<div class="bulk-upload">
	<div class="upload-header">
		<h3>Bulk Video Upload</h3>
		{#if onClose}
			<button type="button" class="btn-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		{/if}
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	{#if !batchId}
		<!-- File Selection Phase -->
		<div
			class="drop-zone"
			class:dragging={isDragging}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			role="button"
			tabindex="0"
			onclick={() => fileInput?.click()}
			onkeypress={(e) => e.key === 'Enter' && fileInput?.click()}
		>
			<IconUpload size={48} />
			<p>Drag & drop video files here</p>
			<p class="hint">or click to browse</p>
			<p class="formats">MP4, WebM, MOV, AVI, MKV</p>
		</div>

		<input
			type="file"
			accept="video/*"
			multiple
			bind:this={fileInput}
			onchange={handleFileSelect}
			style="display: none;"
		/>

		{#if files.length > 0}
			<div class="file-list">
				<div class="file-list-header">
					<span>{files.length} file{files.length > 1 ? 's' : ''} selected</span>
					<span class="total-size">
						{formatFileSize(files.reduce((sum, f) => sum + f.size, 0))}
					</span>
				</div>

				{#each files as file, index (index)}
					<div class="file-item">
						<IconFile size={20} />
						<div class="file-info">
							<div class="file-name">{file.name}</div>
							<div class="file-size">{formatFileSize(file.size)}</div>
						</div>
						<button type="button" class="btn-remove" onclick={() => removeFile(index)}>
							<IconX size={16} />
						</button>
					</div>
				{/each}
			</div>

			<button type="button" class="btn-start" onclick={startBulkUpload} disabled={isInitializing}>
				{#if isInitializing}
					<IconLoader size={20} class="spinning" /> Initializing...
				{:else}
					<IconUpload size={20} /> Start Upload ({files.length} files)
				{/if}
			</button>
		{/if}
	{:else}
		<!-- Upload Progress Phase -->
		<div class="upload-progress">
			{#if batchStatus}
				<div class="progress-summary">
					<div class="progress-bar">
						<div
							class="progress-fill"
							style="width: {((batchStatus.completed + batchStatus.failed) /
								batchStatus.total_files) *
								100}%"
						></div>
					</div>
					<div class="progress-stats">
						<span class="stat completed">
							<IconCheck size={16} />
							{batchStatus.completed} completed
						</span>
						{#if batchStatus.failed > 0}
							<span class="stat failed">
								<IconAlertCircle size={16} />
								{batchStatus.failed} failed
							</span>
						{/if}
						{#if batchStatus.in_progress > 0}
							<span class="stat in-progress">
								<IconLoader size={16} class="spinning" />
								{batchStatus.in_progress} uploading
							</span>
						{/if}
						{#if batchStatus.pending > 0}
							<span class="stat pending">{batchStatus.pending} pending</span>
						{/if}
					</div>
				</div>

				<div class="upload-items">
					{#each batchStatus.uploads as item (item.id)}
						{@const StatusIcon = getStatusIcon(item.status)}
						<div
							class="upload-item"
							class:completed={item.status === 'completed'}
							class:failed={item.status === 'failed'}
						>
							<div class="item-icon" style="color: {getStatusColor(item.status)}">
								<StatusIcon size={20} class={item.status === 'uploading' ? 'spinning' : ''} />
							</div>
							<div class="item-info">
								<div class="item-name">{item.filename}</div>
								{#if item.status === 'uploading' || item.status === 'processing'}
									<div class="item-progress-bar">
										<div
											class="item-progress-fill"
											style="width: {uploadProgress[item.id] || item.progress_percent}%"
										></div>
									</div>
								{:else if item.error_message}
									<div class="item-error">{item.error_message}</div>
								{:else if item.created_video_id}
									<div class="item-success">Video ID: {item.created_video_id}</div>
								{/if}
							</div>
							<div class="item-status">{item.status}</div>
						</div>
					{/each}
				</div>

				{#if batchStatus.completed + batchStatus.failed === batchStatus.total_files}
					<div class="upload-complete">
						<IconCheck size={32} />
						<p>Upload complete!</p>
						<button type="button" class="btn-done" onclick={onClose}>Done</button>
					</div>
				{/if}
			{:else}
				<div class="loading">
					<IconLoader size={32} class="spinning" />
					<p>Preparing uploads...</p>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.bulk-upload {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 600px;
		width: 100%;
	}

	.upload-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.upload-header h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.error-message {
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.drop-zone {
		border: 2px dashed var(--border-color, #444);
		border-radius: 12px;
		padding: 3rem 2rem;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s;
		color: var(--text-secondary);
	}

	.drop-zone:hover,
	.drop-zone.dragging {
		border-color: var(--primary, #e6b800);
		background: var(--primary-alpha, #e6b8001a);
	}

	.drop-zone p {
		margin: 0.5rem 0;
	}

	.drop-zone .hint {
		font-size: 0.875rem;
	}

	.drop-zone .formats {
		font-size: 0.75rem;
		margin-top: 1rem;
	}

	.file-list {
		margin-top: 1rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.file-list-header {
		display: flex;
		justify-content: space-between;
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin-bottom: 0.75rem;
	}

	.file-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		margin-bottom: 0.5rem;
	}

	.file-info {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		font-size: 0.875rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-size {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.btn-remove {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.btn-remove:hover {
		color: #ef4444;
	}

	.btn-start {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.875rem;
		margin-top: 1rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border: none;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-start:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.upload-progress {
		min-height: 200px;
	}

	.progress-summary {
		margin-bottom: 1.5rem;
	}

	.progress-bar {
		height: 8px;
		background: var(--bg-tertiary, #252542);
		border-radius: 4px;
		overflow: hidden;
		margin-bottom: 0.75rem;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary, #e6b800);
		transition: width 0.3s ease;
	}

	.progress-stats {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		font-size: 0.875rem;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.stat.completed {
		color: #22c55e;
	}

	.stat.failed {
		color: #ef4444;
	}

	.stat.in-progress {
		color: #f59e0b;
	}

	.stat.pending {
		color: var(--text-secondary);
	}

	.upload-items {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.upload-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.upload-item.completed {
		opacity: 0.7;
	}

	.upload-item.failed {
		border: 1px solid #ef444440;
	}

	.item-icon {
		flex-shrink: 0;
	}

	.item-info {
		flex: 1;
		min-width: 0;
	}

	.item-name {
		font-size: 0.875rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-progress-bar {
		height: 4px;
		background: var(--bg-primary, #0f0f1a);
		border-radius: 2px;
		margin-top: 0.375rem;
		overflow: hidden;
	}

	.item-progress-fill {
		height: 100%;
		background: var(--primary, #e6b800);
		transition: width 0.3s ease;
	}

	.item-error {
		font-size: 0.75rem;
		color: #ef4444;
		margin-top: 0.25rem;
	}

	.item-success {
		font-size: 0.75rem;
		color: #22c55e;
		margin-top: 0.25rem;
	}

	.item-status {
		font-size: 0.75rem;
		color: var(--text-secondary);
		text-transform: capitalize;
	}

	.upload-complete {
		text-align: center;
		padding: 2rem;
		color: #22c55e;
	}

	.upload-complete p {
		margin: 0.5rem 0 1rem;
		font-size: 1.125rem;
	}

	.btn-done {
		padding: 0.625rem 1.5rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border: none;
		border-radius: 8px;
		cursor: pointer;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: var(--text-secondary);
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
