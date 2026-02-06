<!--
/**
 * Spacer Block Component (Advanced)
 * ═══════════════════════════════════════════════════════════════════════════
 * Simple vertical spacing component with preset height options
 * Visual indicators in edit mode, invisible separator in view mode
 */
-->

<script lang="ts">
	import type { Block, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// Height presets matching the spec
	const HEIGHT_PRESETS = {
		small: '1rem',
		medium: '2rem',
		large: '4rem',
		xlarge: '6rem'
	} as const;

	type HeightSize = keyof typeof HEIGHT_PRESETS;

	// Derived state
	const currentSize = $derived.by<HeightSize>(() => {
		const height = props.block.settings.height as string;
		for (const [size, value] of Object.entries(HEIGHT_PRESETS)) {
			if (height === value) return size as HeightSize;
		}
		return 'medium';
	});

	const height = $derived(HEIGHT_PRESETS[currentSize] || HEIGHT_PRESETS.medium);

	// Convert height to pixels for display
	const heightInPx = $derived.by(() => {
		const remValue = parseFloat(height);
		return Math.round(remValue * 16);
	});

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function setSize(size: HeightSize): void {
		updateSettings({ height: HEIGHT_PRESETS[size] });
	}

	function getSizeLabel(size: HeightSize): string {
		const labels: Record<HeightSize, string> = {
			small: 'S',
			medium: 'M',
			large: 'L',
			xlarge: 'XL'
		};
		return labels[size];
	}
</script>

<div
	class="spacer-block"
	class:editing={props.isEditing}
	class:selected={props.isSelected}
	style:height
	role="separator"
	aria-label="Vertical spacing: {currentSize}"
>
	{#if props.isEditing}
		<div class="spacer-indicator">
			<span class="spacer-label">{heightInPx}px</span>
		</div>

		{#if props.isSelected}
			<div class="size-selector" role="toolbar" aria-label="Spacer size options">
				{#each Object.keys(HEIGHT_PRESETS) as size}
					<button
						type="button"
						class="size-btn"
						class:active={currentSize === size}
						onclick={() => setSize(size as HeightSize)}
						aria-pressed={currentSize === size}
						title="{size} ({HEIGHT_PRESETS[size as HeightSize]})"
					>
						{getSizeLabel(size as HeightSize)}
					</button>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<style>
	.spacer-block {
		position: relative;
		width: 100%;
		transition: height 0.2s ease-out;
	}

	.spacer-block.editing {
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px dashed #cbd5e1;
		border-radius: 8px;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 8px,
			rgba(203, 213, 225, 0.15) 8px,
			rgba(203, 213, 225, 0.15) 16px
		);
	}

	.spacer-block.editing.selected {
		border-color: #3b82f6;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 8px,
			rgba(59, 130, 246, 0.08) 8px,
			rgba(59, 130, 246, 0.08) 16px
		);
	}

	.spacer-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.spacer-label {
		padding: 0.25rem 0.625rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		font-family: ui-monospace, SFMono-Regular, monospace;
		color: #64748b;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.size-selector {
		position: absolute;
		top: 50%;
		right: 0.75rem;
		transform: translateY(-50%);
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
	}

	.size-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 28px;
		height: 26px;
		padding: 0 0.5rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.size-btn:hover {
		background: #f1f5f9;
		color: #475569;
	}

	.size-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 1px;
	}

	.size-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.size-btn.active:hover {
		background: #2563eb;
	}

	/* Dark mode support */
	:global(.dark) .spacer-block.editing {
		border-color: #475569;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 8px,
			rgba(71, 85, 105, 0.2) 8px,
			rgba(71, 85, 105, 0.2) 16px
		);
	}

	:global(.dark) .spacer-block.editing.selected {
		border-color: #3b82f6;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 8px,
			rgba(59, 130, 246, 0.15) 8px,
			rgba(59, 130, 246, 0.15) 16px
		);
	}

	:global(.dark) .spacer-label {
		background: #334155;
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .size-selector {
		background: #1e293b;
		border-color: #334155;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	:global(.dark) .size-btn {
		color: #94a3b8;
	}

	:global(.dark) .size-btn:hover {
		background: #334155;
		color: #e2e8f0;
	}

	:global(.dark) .size-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}
</style>
