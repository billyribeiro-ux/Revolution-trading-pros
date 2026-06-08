<script lang="ts">
	/**
	 * UploadQueue — list of in-flight / completed / errored file uploads with progress bars.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - uploads: UploadItem[]
	 *   - isUploading: boolean   (hides Clear Queue while any upload is in flight)
	 *   - onClearQueue: () => void
	 *
	 * 0 binds, 1 callback.
	 */
	import IconCircleCheckFilled from '@tabler/icons-svelte-runes/icons/circle-check-filled';
	import IconCircleXFilled from '@tabler/icons-svelte-runes/icons/circle-x-filled';

	type UploadItem = {
		id: string;
		file: File;
		progress: number;
		status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
		error?: string;
	};

	let {
		uploads,
		isUploading,
		onClearQueue
	}: {
		uploads: UploadItem[];
		isUploading: boolean;
		onClearQueue: () => void;
	} = $props();

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
</script>

{#if uploads.length > 0}
	<div class="upload-queue">
		{#each uploads as upload (upload.id)}
			<div
				class={[
					'upload-item',
					{
						complete: upload.status === 'complete',
						error: upload.status === 'error'
					}
				]}
			>
				<div class="upload-icon">
					{#if upload.status === 'complete'}
						<IconCircleCheckFilled size={20} aria-hidden="true" class="text-green" />
					{:else if upload.status === 'error'}
						<IconCircleXFilled size={20} aria-hidden="true" class="text-red" />
					{:else}
						<div class="upload-spinner"></div>
					{/if}
				</div>
				<div class="upload-info">
					<span class="upload-name">{upload.file.name}</span>
					<span class="upload-size">{formatBytes(upload.file.size)}</span>
				</div>
				<div class="upload-progress-bar">
					<div class="progress-fill" style:width={`${upload.progress}%`}></div>
				</div>
			</div>
		{/each}
	</div>

	{#if !isUploading}
		<button class="btn-secondary" onclick={onClearQueue}>Clear Queue</button>
	{/if}
{/if}

<style>
	.upload-queue {
		margin-top: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.upload-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.1);
	}

	.upload-item.complete {
		background: rgba(34, 197, 94, 0.1);
		border-color: rgba(34, 197, 94, 0.3);
	}

	.upload-item.error {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
	}

	.upload-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.upload-icon :global(svg) {
		width: 20px;
		height: 20px;
	}

	.upload-icon :global(.text-green) {
		color: #22c55e;
	}
	.upload-icon :global(.text-red) {
		color: #ef4444;
	}

	.upload-spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.upload-info {
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.upload-name {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #f1f5f9;
	}

	.upload-size {
		font-size: 0.75rem;
		color: #64748b;
	}

	.upload-progress-bar {
		width: 100px;
		height: 4px;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		transition: width 0.3s;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.75rem;
		padding: 0.5rem 1rem;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.4);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
