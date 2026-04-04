<!--
  UploadProgress Component
  ═══════════════════════════════════════════════════════════════════════════

  Visual upload progress panel with:
  - Individual file progress bars
  - File thumbnails/previews
  - Upload status indicators
  - Cancel/retry actions
  - Total progress overview

  @version 2.0.0
-->
<script lang="ts">
	import { fade, slide } from 'svelte/transition';

	interface UploadItem {
		id: string;
		file: File;
		progress: number;
		status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
		error?: string;
		result?: {
			id: string;
			url: string;
			thumbnailUrl?: string;
			stats?: {
				originalSize: number;
				optimizedSize: number;
				savingsPercent: number;
			};
		};
	}

	// Props
	let {
		uploads = [],
		showStats = true,
		autoHideDelay: _autoHideDelay = 5000,
		className = '',
		onCancel,
		onRetry,
		onRemove,
		onClear
	}: {
		uploads?: UploadItem[];
		showStats?: boolean;
		autoHideDelay?: number; // Prefixed as _autoHideDelay for future use
		className?: string;
		onCancel?: (id: string) => void;
		onRetry?: (id: string) => void;
		onRemove?: (id: string) => void;
		onClear?: () => void;
	} = $props();

	// Computed values
	let completedCount = $derived(uploads.filter((u) => u.status === 'complete').length);
	let errorCount = $derived(uploads.filter((u) => u.status === 'error').length);
	let inProgressCount = $derived(
		uploads.filter((u) => u.status === 'uploading' || u.status === 'processing').length
	);
	let totalProgress = $derived(
		uploads.length > 0
			? Math.round(uploads.reduce((sum, u) => sum + u.progress, 0) / uploads.length)
			: 0
	);
	let isComplete = $derived(completedCount + errorCount === uploads.length && uploads.length > 0);

	// Format bytes
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Get file thumbnail
	function getFileThumbnail(file: File): string {
		if (file.type.startsWith('image/')) {
			return URL.createObjectURL(file);
		}
		return '';
	}


	// Get status text
	function getStatusText(status: UploadItem['status']): string {
		switch (status) {
			case 'pending':
				return 'Waiting...';
			case 'uploading':
				return 'Uploading...';
			case 'processing':
				return 'Processing...';
			case 'complete':
				return 'Complete';
			case 'error':
				return 'Failed';
			default:
				return '';
		}
	}
</script>

{#if uploads.length > 0}
	<div
		class="up-container {className}"
		transition:slide
		role="region"
		aria-label="File uploads"
		aria-live="polite"
	>
		<!-- Header -->
		<div class="up-header">
			<div class="up-header-left">
				<h3 class="up-title">
					{#if isComplete}
						Upload Complete
					{:else}
						Uploading {inProgressCount} of {uploads.length}
					{/if}
				</h3>
				{#if !isComplete}
					<div class="up-pct">{totalProgress}%</div>
				{/if}
			</div>

			<div class="up-header-right">
				{#if completedCount > 0}
					<span class="up-badge" data-status="complete">{completedCount} done</span>
				{/if}
				{#if errorCount > 0}
					<span class="up-badge" data-status="error">{errorCount} failed</span>
				{/if}
				<button
					class="up-close-btn"
					onclick={() => onClear?.()}
					title="Clear all"
					aria-label="Clear all uploads"
				>
					<svg class="up-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		</div>

		<!-- Overall progress bar -->
		{#if !isComplete}
			<div
				class="up-progress-track"
				role="progressbar"
				aria-valuenow={totalProgress}
				aria-valuemin={0}
				aria-valuemax={100}
				aria-label="Overall upload progress: {totalProgress}%"
			>
				<div class="up-progress-fill" style="width: {totalProgress}%"></div>
			</div>
		{/if}

		<!-- Upload items -->
		<div class="up-items">
			{#each uploads as upload (upload.id)}
				<div class="up-item" transition:slide>
					<!-- Thumbnail -->
					<div class="up-thumb" aria-hidden="true">
						{#if upload.result?.thumbnailUrl}
							<img src={upload.result.thumbnailUrl} alt="Thumbnail preview of {upload.file.name}" class="up-thumb-img" />
						{:else if upload.file.type.startsWith('image/')}
							<img src={getFileThumbnail(upload.file)} alt="Preview of {upload.file.name}" class="up-thumb-img" />
						{:else}
							<svg class="up-icon-md up-icon-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
							</svg>
						{/if}
					</div>

					<!-- Info -->
					<div class="up-info">
						<div class="up-filename">{upload.file.name}</div>
						<div class="up-meta">
							<span>{formatBytes(upload.file.size)}</span>
							<span class="up-status-dot" data-status={upload.status}></span>
							<span>{getStatusText(upload.status)}</span>
						</div>

						<!-- Progress bar for individual file -->
						{#if upload.status === 'uploading' || upload.status === 'processing'}
							<div class="up-file-track">
								<div
									class="up-file-fill"
									data-processing={upload.status === 'processing' || undefined}
									style="width: {upload.progress}%"
								></div>
							</div>
						{/if}

						<!-- Error message -->
						{#if upload.status === 'error' && upload.error}
							<div class="up-error">{upload.error}</div>
						{/if}

						<!-- Optimization stats -->
						{#if showStats && upload.status === 'complete' && upload.result?.stats}
							<div class="up-opt-stats" transition:fade>
								<span class="up-opt-original">{formatBytes(upload.result.stats.originalSize)}</span>
								<svg class="up-icon-xs up-icon-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
								</svg>
								<span class="up-opt-optimized">{formatBytes(upload.result.stats.optimizedSize)}</span>
								<span class="up-opt-savings">-{upload.result.stats.savingsPercent}%</span>
							</div>
						{/if}
					</div>

					<!-- Actions -->
					<div class="up-actions">
						{#if upload.status === 'uploading' || upload.status === 'pending'}
							<button
								class="up-action-btn" data-action="cancel"
								onclick={() => onCancel?.(upload.id)}
								title="Cancel"
								aria-label="Cancel upload of {upload.file.name}"
							>
								<svg class="up-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
								</svg>
							</button>
						{:else if upload.status === 'error'}
							<button
								class="up-action-btn" data-action="retry"
								onclick={() => onRetry?.(upload.id)}
								title="Retry"
								aria-label="Retry upload of {upload.file.name}"
							>
								<svg class="up-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
								</svg>
							</button>
						{:else if upload.status === 'complete'}
							<button
								class="up-action-btn" data-action="done"
								onclick={() => onRemove?.(upload.id)}
								title="Remove"
								aria-label="Remove {upload.file.name} from list"
							>
								<svg class="up-icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.up-container {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-xl);
		box-shadow: 0 10px 25px oklch(0 0 0 / 10%);
		border: 1px solid oklch(0.9 0.005 265);
		overflow: hidden;
	}

	.up-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-block-end: 1px solid oklch(0.9 0.005 265);
	}

	.up-header-left { display: flex; align-items: center; gap: var(--space-3); }
	.up-header-right { display: flex; align-items: center; gap: var(--space-2); }

	.up-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.up-pct { font-size: var(--text-xs); color: oklch(0.55 0.01 265); }

	.up-badge {
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		border-radius: 9999px;

		&[data-status='complete'] { background-color: oklch(0.92 0.06 160); color: oklch(0.4 0.12 160); }
		&[data-status='error'] { background-color: oklch(0.92 0.06 25); color: oklch(0.45 0.18 25); }
	}

	.up-close-btn {
		padding: 0.375rem;
		color: oklch(0.65 0.01 265);
		border-radius: var(--radius-lg);
		transition: all var(--duration-fast) var(--ease-default);
		background: none;
		border: none;
		cursor: pointer;

		&:hover { color: oklch(0.4 0.01 265); background-color: oklch(0.95 0.002 265); }
	}

	.up-icon-xs { inline-size: 0.75rem; block-size: 0.75rem; }
	.up-icon-sm { inline-size: 1rem; block-size: 1rem; }
	.up-icon-md { inline-size: 1.5rem; block-size: 1.5rem; }
	.up-icon-muted { color: oklch(0.65 0.01 265); }

	.up-progress-track {
		block-size: 0.25rem;
		background-color: oklch(0.9 0.005 265);
	}

	.up-progress-fill {
		block-size: 100%;
		background-color: oklch(0.6 0.2 260);
		transition: all 300ms var(--ease-default);
	}

	.up-items {
		max-block-size: 300px;
		overflow-y: auto;
	}

	.up-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-block-end: 1px solid oklch(0.95 0.002 265);

		&:last-child { border-block-end: none; }
	}

	.up-thumb {
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: var(--radius-lg);
		overflow: hidden;
		flex-shrink: 0;
		background-color: oklch(0.95 0.002 265);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.up-thumb-img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
	}

	.up-info { flex: 1; min-inline-size: 0; }

	.up-filename {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.up-meta {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.up-status-dot {
		inline-size: 0.5rem;
		block-size: 0.5rem;
		border-radius: 9999px;

		&[data-status='pending'] { background-color: oklch(0.75 0.005 265); }
		&[data-status='uploading'] { background-color: oklch(0.6 0.2 260); }
		&[data-status='processing'] { background-color: oklch(0.8 0.18 90); }
		&[data-status='complete'] { background-color: oklch(0.6 0.18 160); }
		&[data-status='error'] { background-color: oklch(0.6 0.25 25); }
	}

	.up-file-track {
		block-size: 0.25rem;
		background-color: oklch(0.9 0.005 265);
		border-radius: 9999px;
		margin-block-start: var(--space-2);
		overflow: hidden;
	}

	.up-file-fill {
		block-size: 100%;
		background-color: oklch(0.6 0.2 260);
		transition: all 300ms var(--ease-default);
		border-radius: 9999px;

		&[data-processing] { animation: pulse 2s ease-in-out infinite; }
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.up-error {
		font-size: var(--text-xs);
		color: oklch(0.55 0.2 25);
		margin-block-start: var(--space-1);
	}

	.up-opt-stats {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: var(--text-xs);
		margin-block-start: var(--space-1);
	}

	.up-opt-original { color: oklch(0.65 0.01 265); text-decoration: line-through; }
	.up-opt-optimized { color: oklch(0.4 0.01 265); }
	.up-opt-savings { color: oklch(0.55 0.18 160); font-weight: var(--weight-medium); }

	.up-actions { flex-shrink: 0; }

	.up-action-btn {
		padding: 0.375rem;
		border-radius: var(--radius-lg);
		transition: all var(--duration-fast) var(--ease-default);
		color: oklch(0.65 0.01 265);
		background: none;
		border: none;
		cursor: pointer;

		&[data-action='cancel']:hover { color: oklch(0.55 0.2 25); background-color: oklch(0.95 0.02 25); }
		&[data-action='retry']:hover { color: oklch(0.55 0.2 260); background-color: oklch(0.95 0.02 260); }
		&[data-action='done'] { color: oklch(0.55 0.18 160); }
		&[data-action='done']:hover { color: oklch(0.45 0.18 160); background-color: oklch(0.95 0.03 160); }
	}
</style>
