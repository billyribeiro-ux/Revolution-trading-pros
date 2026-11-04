<script lang="ts">
	/**
	 * ResponsivePreviewModal — full-screen modal wrapping `#lib/components/media/ResponsivePreview`.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - isOpen: boolean    ($bindable — clicking the overlay / close button sets false)
	 *   - item: MediaItem | null  (variants come from item.variants)
	 *
	 * 1 bindable scalar, 0 callbacks.
	 */
	import { fade, scale } from 'svelte/transition';
	import type { MediaItem } from '#lib/api/media.js';
	import ResponsivePreview from '#lib/components/media/ResponsivePreview.svelte';
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	let {
		isOpen = $bindable(false),
		item
	}: {
		isOpen?: boolean;
		item: MediaItem | null;
	} = $props();
</script>

{#if isOpen && item}
	<div
		class="modal-overlay"
		onclick={() => (isOpen = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (isOpen = false)}
		role="dialog"
		aria-modal="true"
		aria-labelledby="preview-modal-title"
		tabindex="-1"
		transition:fade={{ duration: 200 }}
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="presentation"
			transition:scale={{ duration: 300, start: 0.95 }}
		>
			<div class="modal-header">
				<h2 id="preview-modal-title">Responsive Preview</h2>
				<button class="btn-icon" onclick={() => (isOpen = false)} aria-label="Close preview">
					<IconX size={20} aria-hidden="true" />
				</button>
			</div>
			<ResponsivePreview
				originalSize={item.file_size ?? 0}
				showSizes={item.file_size !== null}
				variants={item.variants?.length
					? item.variants.map((v) => ({
							sizeName: `${v.size ?? v.type} · ${v.format} · ${v.width}w${v.is_retina ? ' 2x' : ''}`,
							width: v.width,
							height: v.height,
							url: v.preview_url || v.url,
							size: v.file_size
						}))
					: item.width && item.height
						? [
								{
									sizeName: 'Original',
									width: item.width,
									height: item.height,
									url: item.preview_url || item.url,
									size: item.file_size ?? 0
								}
							]
						: []}
			/>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1.5rem;
	}

	.modal-content {
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 16px;
		max-width: 1200px;
		max-height: 90vh;
		width: 100%;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.modal-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: var(--text-secondary, #8b949e);
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400, #ffd11a);
	}

	.btn-icon :global(svg) {
		width: 18px;
		height: 18px;
	}
</style>
