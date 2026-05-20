<script lang="ts">
	/**
	 * ResponsivePreviewModal — full-screen modal wrapping `$lib/components/media/ResponsivePreview`.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Props:
	 *   - isOpen: boolean    ($bindable — clicking the overlay / close button sets false)
	 *   - item: MediaItem | null  (variants come from item.variants)
	 *
	 * 1 bindable scalar, 0 callbacks.
	 */
	import { fade, scale } from 'svelte/transition';
	import type { MediaItem } from '$lib/api/media';
	import ResponsivePreview from '$lib/components/media/ResponsivePreview.svelte';
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
				variants={(item.variants ?? []).map((v) => ({
					sizeName: v.size ?? v.type,
					width: v.width,
					height: v.height,
					url: v.url,
					size: v.file_size
				}))}
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
		background: rgba(15, 23, 42, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		max-width: 1200px;
		max-height: 90vh;
		width: 100%;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon :global(svg) {
		width: 18px;
		height: 18px;
	}
</style>
