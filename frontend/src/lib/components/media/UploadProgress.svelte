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
	import type { Attachment } from 'svelte/attachments';
	import { fade, slide } from 'svelte/transition';
	import Icon from '$lib/components/Icon.svelte';

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

	// Attach local preview URLs to the image lifecycle so browser object URLs are revoked.
	function localPreview(file: File): Attachment<HTMLImageElement> {
		return (image) => {
			const url = URL.createObjectURL(file);
			image.src = url;

			return () => {
				URL.revokeObjectURL(url);
			};
		};
	}

	// Get status class
	function getStatusClass(status: UploadItem['status']): string {
		switch (status) {
			case 'pending':
				return 'upload-progress__status-dot--pending';
			case 'uploading':
				return 'upload-progress__status-dot--uploading';
			case 'processing':
				return 'upload-progress__status-dot--processing';
			case 'complete':
				return 'upload-progress__status-dot--complete';
			case 'error':
				return 'upload-progress__status-dot--error';
			default:
				return 'upload-progress__status-dot--pending';
		}
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
		class={['upload-progress', className]}
		transition:slide
		role="region"
		aria-label="File uploads"
		aria-live="polite"
	>
		<!-- Header -->
		<div class="upload-progress__header">
			<div class="upload-progress__summary">
				<h3 class="upload-progress__title">
					{#if isComplete}
						Upload Complete
					{:else}
						Uploading {inProgressCount} of {uploads.length}
					{/if}
				</h3>
				{#if !isComplete}
					<div class="upload-progress__percent">{totalProgress}%</div>
				{/if}
			</div>

			<div class="upload-progress__actions">
				{#if completedCount > 0}
					<span class="upload-progress__badge upload-progress__badge--success"
						>{completedCount} done</span
					>
				{/if}
				{#if errorCount > 0}
					<span class="upload-progress__badge upload-progress__badge--error"
						>{errorCount} failed</span
					>
				{/if}
				<button
					class="upload-progress__icon-button upload-progress__icon-button--neutral"
					onclick={() => onClear?.()}
					title="Clear all"
					aria-label="Clear all uploads"
				>
					<Icon name="IconX" size={16} />
				</button>
			</div>
		</div>

		<!-- Overall progress bar -->
		{#if !isComplete}
			<div
				class="upload-progress__total-track"
				role="progressbar"
				aria-valuenow={totalProgress}
				aria-valuemin={0}
				aria-valuemax={100}
				aria-label="Overall upload progress: {totalProgress}%"
			>
				<div class="upload-progress__total-fill" style:width={`${totalProgress}%`}></div>
			</div>
		{/if}

		<!-- Upload items -->
		<div class="upload-progress__list">
			{#each uploads as upload (upload.id)}
				<div class="upload-progress__item" transition:slide>
					<!-- Thumbnail -->
					<div class="upload-progress__thumbnail" aria-hidden="true">
						{#if upload.result?.thumbnailUrl}
							<img
								src={upload.result.thumbnailUrl}
								alt="Thumbnail preview of {upload.file.name}"
								class="upload-progress__thumbnail-image"
								width="48"
								height="48"
							/>
						{:else if upload.file.type.startsWith('image/')}
							<img
								{@attach localPreview(upload.file)}
								alt="Preview of {upload.file.name}"
								class="upload-progress__thumbnail-image"
								width="48"
								height="48"
							/>
						{:else}
							<Icon name="IconPhoto" size={24} class="upload-progress__icon-muted" />
						{/if}
					</div>

					<!-- Info -->
					<div class="upload-progress__details">
						<div class="upload-progress__file-name">
							{upload.file.name}
						</div>
						<div class="upload-progress__meta">
							<span>{formatBytes(upload.file.size)}</span>
							<span class={['upload-progress__status-dot', getStatusClass(upload.status)]}></span>
							<span>{getStatusText(upload.status)}</span>
						</div>

						<!-- Progress bar for individual file -->
						{#if upload.status === 'uploading' || upload.status === 'processing'}
							<div class="upload-progress__file-track">
								<div
									class={[
										'upload-progress__file-fill',
										upload.status === 'processing' && 'upload-progress__file-fill--processing'
									]}
									style:width={`${upload.progress}%`}
								></div>
							</div>
						{/if}

						<!-- Error message -->
						{#if upload.status === 'error' && upload.error}
							<div class="upload-progress__error">{upload.error}</div>
						{/if}

						<!-- Optimization stats -->
						{#if showStats && upload.status === 'complete' && upload.result?.stats}
							<div class="upload-progress__stats" transition:fade>
								<span class="upload-progress__original-size"
									>{formatBytes(upload.result.stats.originalSize)}</span
								>
								<Icon name="IconArrowRight" size={12} class="upload-progress__icon-muted" />
								<span class="upload-progress__optimized-size"
									>{formatBytes(upload.result.stats.optimizedSize)}</span
								>
								<span class="upload-progress__savings">-{upload.result.stats.savingsPercent}%</span>
							</div>
						{/if}
					</div>

					<!-- Actions -->
					<div class="upload-progress__item-actions">
						{#if upload.status === 'uploading' || upload.status === 'pending'}
							<button
								class="upload-progress__icon-button upload-progress__icon-button--danger"
								onclick={() => onCancel?.(upload.id)}
								title="Cancel"
								aria-label="Cancel upload of {upload.file.name}"
							>
								<Icon name="IconX" size={16} />
							</button>
						{:else if upload.status === 'error'}
							<button
								class="upload-progress__icon-button upload-progress__icon-button--info"
								onclick={() => onRetry?.(upload.id)}
								title="Retry"
								aria-label="Retry upload of {upload.file.name}"
							>
								<Icon name="IconRefresh" size={16} />
							</button>
						{:else if upload.status === 'complete'}
							<button
								class="upload-progress__icon-button upload-progress__icon-button--success"
								onclick={() => onRemove?.(upload.id)}
								title="Remove"
								aria-label="Remove {upload.file.name} from list"
							>
								<Icon name="IconCheck" size={16} />
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.upload-progress {
		overflow: hidden;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		box-shadow:
			0 10px 15px -3px rgb(0 0 0 / 0.1),
			0 4px 6px -4px rgb(0 0 0 / 0.1);
	}

	:global(.dark) .upload-progress {
		border-color: #374151;
		background: #1f2937;
	}

	.upload-progress__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	:global(.dark) .upload-progress__header {
		border-bottom-color: #374151;
	}

	.upload-progress__summary,
	.upload-progress__actions,
	.upload-progress__meta,
	.upload-progress__stats {
		display: flex;
		align-items: center;
	}

	.upload-progress__summary {
		gap: 0.75rem;
		min-width: 0;
	}

	.upload-progress__actions {
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.upload-progress__title {
		margin: 0;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	:global(.dark) .upload-progress__title {
		color: #ffffff;
	}

	.upload-progress__percent,
	.upload-progress__meta,
	.upload-progress__stats,
	.upload-progress__error {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.upload-progress__percent {
		color: #6b7280;
	}

	.upload-progress__badge {
		display: inline-flex;
		align-items: center;
		border-radius: 9999px;
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1rem;
		white-space: nowrap;
	}

	.upload-progress__badge--success {
		background: #dcfce7;
		color: #15803d;
	}

	:global(.dark) .upload-progress__badge--success {
		background: rgb(20 83 45 / 0.3);
		color: #4ade80;
	}

	.upload-progress__badge--error {
		background: #fee2e2;
		color: #b91c1c;
	}

	:global(.dark) .upload-progress__badge--error {
		background: rgb(127 29 29 / 0.3);
		color: #f87171;
	}

	.upload-progress__icon-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 1.75rem;
		height: 1.75rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		transition:
			background-color 0.15s ease,
			color 0.15s ease;
	}

	.upload-progress__icon-button--neutral:hover {
		background: #f3f4f6;
		color: #4b5563;
	}

	:global(.dark) .upload-progress__icon-button--neutral:hover {
		background: #374151;
		color: #d1d5db;
	}

	.upload-progress__icon-button--danger:hover {
		background: #fef2f2;
		color: #ef4444;
	}

	:global(.dark) .upload-progress__icon-button--danger:hover {
		background: rgb(127 29 29 / 0.2);
	}

	.upload-progress__icon-button--info:hover {
		background: #eff6ff;
		color: #3b82f6;
	}

	:global(.dark) .upload-progress__icon-button--info:hover {
		background: rgb(30 58 138 / 0.2);
	}

	.upload-progress__icon-button--success {
		color: #22c55e;
	}

	.upload-progress__icon-button--success:hover {
		background: #f0fdf4;
		color: #16a34a;
	}

	:global(.dark) .upload-progress__icon-button--success:hover {
		background: rgb(20 83 45 / 0.2);
	}

	.upload-progress__total-track,
	.upload-progress__file-track {
		overflow: hidden;
		background: #e5e7eb;
	}

	:global(.dark) .upload-progress__total-track,
	:global(.dark) .upload-progress__file-track {
		background: #374151;
	}

	.upload-progress__total-track {
		height: 0.25rem;
	}

	.upload-progress__total-fill,
	.upload-progress__file-fill {
		height: 100%;
		background: #3b82f6;
		transition: width 0.3s ease;
	}

	.upload-progress__list {
		max-height: 300px;
		overflow-y: auto;
	}

	.upload-progress__item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid #f3f4f6;
	}

	.upload-progress__item:last-child {
		border-bottom: 0;
	}

	:global(.dark) .upload-progress__item {
		border-bottom-color: #374151;
	}

	.upload-progress__thumbnail {
		display: flex;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		overflow: hidden;
		border-radius: 0.5rem;
		background: #f3f4f6;
	}

	:global(.dark) .upload-progress__thumbnail {
		background: #374151;
	}

	.upload-progress__thumbnail-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.upload-progress :global(.upload-progress__icon-muted) {
		color: #9ca3af;
	}

	.upload-progress__details {
		flex: 1 1 auto;
		min-width: 0;
	}

	.upload-progress__file-name {
		overflow: hidden;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	:global(.dark) .upload-progress__file-name {
		color: #ffffff;
	}

	.upload-progress__meta {
		gap: 0.5rem;
		color: #6b7280;
	}

	:global(.dark) .upload-progress__meta {
		color: #9ca3af;
	}

	.upload-progress__status-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 9999px;
	}

	.upload-progress__status-dot--pending {
		background: #d1d5db;
	}

	:global(.dark) .upload-progress__status-dot--pending {
		background: #4b5563;
	}

	.upload-progress__status-dot--uploading {
		background: #3b82f6;
	}

	.upload-progress__status-dot--processing {
		background: #eab308;
	}

	.upload-progress__status-dot--complete {
		background: #22c55e;
	}

	.upload-progress__status-dot--error {
		background: #ef4444;
	}

	.upload-progress__file-track {
		height: 0.25rem;
		margin-top: 0.5rem;
		border-radius: 9999px;
	}

	.upload-progress__file-fill {
		border-radius: inherit;
	}

	.upload-progress__file-fill--processing {
		animation: upload-progress-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.upload-progress__error {
		margin-top: 0.25rem;
		color: #ef4444;
	}

	.upload-progress__stats {
		gap: 0.25rem;
		margin-top: 0.25rem;
	}

	.upload-progress__original-size {
		color: #9ca3af;
		text-decoration: line-through;
	}

	.upload-progress__optimized-size {
		color: #4b5563;
	}

	:global(.dark) .upload-progress__optimized-size {
		color: #d1d5db;
	}

	.upload-progress__savings {
		color: #22c55e;
		font-weight: 500;
	}

	.upload-progress__item-actions {
		flex: 0 0 auto;
	}

	@keyframes upload-progress-pulse {
		50% {
			opacity: 0.5;
		}
	}
</style>
