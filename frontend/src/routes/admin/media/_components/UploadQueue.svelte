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
		background: var(--bg-base, #0d1117);
		border-radius: 8px;
		border: 1px solid var(--border-muted, #21262d);
	}

	.upload-item.complete {
		background: rgba(63, 185, 80, 0.1);
		border-color: rgba(63, 185, 80, 0.3);
	}

	.upload-item.error {
		background: rgba(248, 81, 73, 0.1);
		border-color: rgba(248, 81, 73, 0.3);
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
		color: var(--success-emphasis, #3fb950);
	}
	.upload-icon :global(.text-red) {
		color: var(--error-emphasis, #f85149);
	}

	.upload-spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(230, 184, 0, 0.2);
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
		color: var(--text-primary, #f0f6fc);
	}

	.upload-size {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.upload-progress-bar {
		width: 100px;
		height: 4px;
		background: var(--bg-surface, #1c2128);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary-500);
		transition: width 0.3s;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.75rem;
		padding: 0.5rem 1rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
