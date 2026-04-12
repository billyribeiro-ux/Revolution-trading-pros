<script lang="ts">
	/**
	 * MediaGrid Component
	 *
	 * Displays media items in a responsive grid with selection,
	 * preview, and bulk actions support.
	 */
	import type { MediaItem } from '$lib/api/media';

	interface Props {
		items?: MediaItem[];
		selectedIds?: string[];
		selectable?: boolean;
		showStatus?: boolean;
		columns?: number;
		loading?: boolean;
		onselect?: (item: MediaItem) => void;
		onpreview?: (item: MediaItem) => void;
		ondelete?: (item: MediaItem) => void;
		onoptimize?: (item: MediaItem) => void;
		onselectionchange?: (ids: string[]) => void;
	}

	let {
		items = [],
		selectedIds = $bindable([]),
		selectable = true,
		showStatus = true,
		columns = 4,
		loading = false,
		onselect,
		onpreview,
		ondelete,
		onoptimize,
		onselectionchange
	}: Props = $props();

	// State
	let shiftKeyHeld = $state(false);
	let lastSelectedId: string | null = $state(null);

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Shift') shiftKeyHeld = true;
	}

	function handleKeyUp(e: KeyboardEvent) {
		if (e.key === 'Shift') shiftKeyHeld = false;
	}

	function toggleSelection(item: MediaItem) {
		if (!selectable) return;

		const idx = selectedIds.indexOf(item.id);

		if (shiftKeyHeld && lastSelectedId !== null) {
			// Range selection
			const lastIdx = items.findIndex((i) => i.id === lastSelectedId);
			const currentIdx = items.findIndex((i) => i.id === item.id);
			const sorted = [lastIdx, currentIdx].sort((a, b) => a - b);
			const start = sorted[0] ?? 0;
			const end = sorted[1] ?? 0;
			const rangeIds = items.slice(start, end + 1).map((i) => i.id);

			if (idx === -1) {
				// Add range
				selectedIds = [...new Set([...selectedIds, ...rangeIds])];
			} else {
				// Remove range
				selectedIds = selectedIds.filter((id) => !rangeIds.includes(id));
			}
		} else {
			// Single selection
			if (idx === -1) {
				selectedIds = [...selectedIds, item.id];
			} else {
				selectedIds = selectedIds.filter((id) => id !== item.id);
			}
		}

		lastSelectedId = item.id;
		onselectionchange?.(selectedIds);
	}

	function handleItemClick(item: MediaItem, e: MouseEvent) {
		if ((e.target as HTMLElement).closest('.item-actions')) {
			return; // Don't select when clicking actions
		}
		onselect?.(item);
	}

	function handlePreview(item: MediaItem, e: MouseEvent) {
		e.stopPropagation();
		onpreview?.(item);
	}

	function handleDelete(item: MediaItem, e: MouseEvent) {
		e.stopPropagation();
		ondelete?.(item);
	}

	function handleOptimize(item: MediaItem, e: MouseEvent) {
		e.stopPropagation();
		onoptimize?.(item);
	}

	function getStatusLevel(status: string): string {
		switch (status) {
			case 'completed':
				return 'completed';
			case 'processing':
				return 'processing';
			case 'failed':
				return 'failed';
			default:
				return 'pending';
		}
	}

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / 1024 / 1024).toFixed(2) + ' MB';
	}
</script>

<svelte:window onkeydown={handleKeyDown} onkeyup={handleKeyUp} />

<div class="media-grid" style="--columns: {columns}">
	{#if loading}
		<!-- Loading skeleton -->
		{#each Array(8) as _, i (i)}
			<div class="grid-item skeleton">
				<div class="skeleton-image"></div>
				<div class="skeleton-text"></div>
			</div>
		{/each}
	{:else if items.length === 0}
		<!-- Empty state -->
		<div class="empty-state">
			<svg
				width="64"
				height="64"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
			>
				<rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
				<circle cx="8.5" cy="8.5" r="1.5" />
				<path d="M21 15l-5-5L5 21" />
			</svg>
			<span>No media found</span>
		</div>
	{:else}
		{#each items as item (item.id)}
			{@const isSelected = selectedIds.includes(item.id)}
			<div
				class="grid-item"
				data-selected={isSelected || undefined}
				role="button"
				tabindex="0"
				onclick={(e: MouseEvent) => handleItemClick(item, e)}
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onselect?.(item)}
			>
				<!-- Selection checkbox -->
				{#if selectable}
					<div class="item-checkbox">
						<input
							type="checkbox"
							checked={isSelected}
							onclick={(e: MouseEvent) => {
								e.stopPropagation();
								toggleSelection(item);
							}}
							onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
						/>
					</div>
				{/if}

				<!-- Thumbnail -->
				<div class="item-thumbnail">
					{#if item.file_type === 'image'}
						<img
							src={item.thumbnail_url || item.url}
							alt={item.alt_text || item.filename}
							width="240"
							height="240"
							loading="lazy"
							decoding="async"
						/>
					{:else if item.file_type === 'video'}
						<div class="type-icon video">
							<svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
								<path d="M8 5v14l11-7z" />
							</svg>
						</div>
					{:else if item.file_type === 'document'}
						<div class="type-icon document">
							<svg
								width="32"
								height="32"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
								<path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" />
							</svg>
						</div>
					{:else}
						<div class="type-icon other">
							<svg
								width="32"
								height="32"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M13 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V9z" />
								<path d="M13 2v7h7" />
							</svg>
						</div>
					{/if}

					<!-- Optimization status badge -->
					{#if showStatus && item.file_type === 'image'}
						<div
							class="status-badge"
							data-status={getStatusLevel(item.processing_status || 'pending')}
						>
							{#if item.is_optimized}
								<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
									<path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
								</svg>
							{:else if item.processing_status === 'processing'}
								<svg
									class="animate-spin"
									width="12"
									height="12"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="3"
								>
									<circle cx="12" cy="12" r="10" stroke-dasharray="32" stroke-dashoffset="32" />
								</svg>
							{:else if item.processing_status === 'failed'}
								<svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
									<path
										d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
									/>
								</svg>
							{:else}
								<svg
									width="12"
									height="12"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="3"
								>
									<circle cx="12" cy="12" r="10" />
								</svg>
							{/if}
						</div>
					{/if}

					<!-- Actions overlay -->
					<div class="item-actions">
						<button
							type="button"
							class="action-btn"
							title="Preview"
							onclick={(e: MouseEvent) => handlePreview(item, e)}
						>
							<svg
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
								<circle cx="12" cy="12" r="3" />
							</svg>
						</button>
						{#if item.file_type === 'image' && !item.is_optimized}
							<button
								type="button"
								class="action-btn optimize"
								title="Optimize"
								onclick={(e: MouseEvent) => handleOptimize(item, e)}
							>
								<svg
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
								>
									<path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
								</svg>
							</button>
						{/if}
						<button
							type="button"
							class="action-btn delete"
							title="Delete"
							onclick={(e: MouseEvent) => handleDelete(item, e)}
						>
							<svg
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path
									d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"
								/>
							</svg>
						</button>
					</div>
				</div>

				<!-- Item info -->
				<div class="item-info">
					<span class="item-name" title={item.filename}>{item.filename}</span>
					<div class="item-meta">
						{#if item.width && item.height}
							<span>{item.width}x{item.height}</span>
						{/if}
						<span>{formatBytes(item.file_size)}</span>
					</div>
				</div>
			</div>
		{/each}
	{/if}
</div>

<style>
	.media-grid {
		display: grid;
		grid-template-columns: repeat(var(--columns), 1fr);
		gap: var(--space-4);
		padding-block: var(--space-4);

		@media (max-width: 1024px) {
			--columns: 3 !important;
		}
		@media (max-width: 768px) {
			--columns: 2 !important;
		}
		@media (max-width: 480px) {
			--columns: 1 !important;
		}
	}

	.grid-item {
		position: relative;
		background: oklch(1 0 0);
		border: 2px solid transparent;
		border-radius: var(--radius-lg);
		overflow: hidden;
		cursor: pointer;
		transition: all 200ms var(--ease-default);

		&:hover {
			border-color: oklch(0.6 0.2 260);
			box-shadow: 0 4px 12px oklch(0 0 0 / 10%);
		}

		&[data-selected] {
			border-color: oklch(0.6 0.2 260);
			background: oklch(0.96 0.03 260);
		}
	}

	.item-checkbox {
		position: absolute;
		inset-block-start: 8px;
		inset-inline-start: 8px;
		z-index: 10;
	}

	.item-checkbox input {
		inline-size: 18px;
		block-size: 18px;
		cursor: pointer;
	}

	.item-thumbnail {
		position: relative;
		aspect-ratio: 1;
		background: oklch(0.96 0.002 265);
		overflow: hidden;
	}

	.item-thumbnail img {
		inline-size: 100%;
		block-size: 100%;
		object-fit: cover;
	}

	.type-icon {
		inline-size: 100%;
		block-size: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: oklch(0.65 0.01 265);
	}

	.status-badge {
		position: absolute;
		inset-block-start: 8px;
		inset-inline-end: 8px;
		inline-size: 20px;
		block-size: 20px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: oklch(1 0 0);

		&[data-status='completed'] {
			background-color: oklch(0.6 0.18 160);
		}
		&[data-status='processing'] {
			background-color: oklch(0.8 0.18 90);
		}
		&[data-status='failed'] {
			background-color: oklch(0.6 0.25 25);
		}
		&[data-status='pending'] {
			background-color: oklch(0.65 0.01 265);
		}
	}

	.item-actions {
		position: absolute;
		inset-block-end: 0;
		inset-inline-start: 0;
		inset-inline-end: 0;
		display: flex;
		justify-content: center;
		gap: var(--space-2);
		padding: var(--space-2);
		background: linear-gradient(transparent, oklch(0 0 0 / 70%));
		opacity: 0;
		transition: opacity 200ms var(--ease-default);
	}

	.grid-item:hover .item-actions {
		opacity: 1;
	}

	.action-btn {
		inline-size: 32px;
		block-size: 32px;
		border: none;
		border-radius: var(--radius-md);
		background: oklch(1 0 0 / 90%);
		color: oklch(0.15 0.01 265);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 200ms var(--ease-default);

		&:hover {
			background: oklch(1 0 0);
			transform: scale(1.1);
		}

		&.optimize:hover {
			background: oklch(0.6 0.2 260);
			color: oklch(1 0 0);
		}
		&.delete:hover {
			background: oklch(0.6 0.25 25);
			color: oklch(1 0 0);
		}
	}

	.item-info {
		padding: var(--space-3);
	}

	.item-name {
		display: block;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-meta {
		display: flex;
		gap: var(--space-2);
		margin-block-start: 0.25rem;
		font-size: var(--text-xs);
		color: oklch(0.5 0.01 265);
	}

	/* ─── Skeleton ─── */
	.skeleton {
		pointer-events: none;
	}

	.skeleton-image {
		aspect-ratio: 1;
		background: linear-gradient(
			90deg,
			oklch(0.95 0.002 265) 25%,
			oklch(0.9 0.005 265) 50%,
			oklch(0.95 0.002 265) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-text {
		block-size: 40px;
		margin: var(--space-3);
		background: linear-gradient(
			90deg,
			oklch(0.95 0.002 265) 25%,
			oklch(0.9 0.005 265) 50%,
			oklch(0.95 0.002 265) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}

	/* ─── Empty state ─── */
	.empty-state {
		grid-column: 1 / -1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		padding-block: var(--space-16);
		padding-inline: var(--space-8);
		color: oklch(0.65 0.01 265);
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}
</style>
