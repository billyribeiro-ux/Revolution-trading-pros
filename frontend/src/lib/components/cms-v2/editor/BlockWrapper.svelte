<!--
	Block Wrapper - Wraps each block with toolbar and drag handle
	═══════════════════════════════════════════════════════════════════════════════

	Provides:
	- Drag handle for reordering
	- Selection indicator
	- Hover actions (move, duplicate, delete)
	- Block type indicator

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { getContext } from 'svelte';
	import { fade, scale } from 'svelte/transition';
	import { editorStore, type ContentBlock } from '$lib/stores/editor.svelte';
	import BlockRenderer from './BlockRenderer.svelte';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte/icons/arrow-down';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconDotsVertical from '@tabler/icons-svelte/icons/dots-vertical';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
	}

	let { block, readonly = false }: Props = $props();

	// ==========================================================================
	// Context
	// ==========================================================================

	const editorContext = getContext<{
		selectedBlockId: string | null;
		selectBlock: (id: string | null) => void;
		openSlashMenu: (position: { x: number; y: number }, afterBlockId?: string) => void;
	}>('blockEditor');

	// ==========================================================================
	// State
	// ==========================================================================

	let wrapperRef = $state<HTMLDivElement | null>(null);
	let isHovered = $state(false);
	let showMoreMenu = $state(false);

	// ==========================================================================
	// Derived
	// ==========================================================================

	let isSelected = $derived(editorContext?.selectedBlockId === block.id);
	let blocks = $derived(editorStore.contentBlocks);
	let blockIndex = $derived(blocks.findIndex(b => b.id === block.id));
	let canMoveUp = $derived(blockIndex > 0);
	let canMoveDown = $derived(blockIndex < blocks.length - 1);

	// Block type display name
	let blockTypeName = $derived.by(() => {
		const typeNames: Record<string, string> = {
			'rich-text': 'Text',
			'heading': 'Heading',
			'image': 'Image',
			'video': 'Video',
			'divider': 'Divider',
			'spacer': 'Spacer',
			'quote': 'Quote',
			'code': 'Code',
			'list': 'List',
			'callout': 'Callout',
			'trade-setup': 'Trade Setup',
			'performance-stats': 'Stats',
			'tradingview-chart': 'Chart',
			'group': 'Group',
			'columns': 'Columns'
		};
		return typeNames[block.blockType] || block.blockType;
	});

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleSelect() {
		if (readonly) return;
		editorContext?.selectBlock(block.id);
	}

	function handleMoveUp() {
		if (!canMoveUp) return;
		editorStore.moveBlock(block.id, blockIndex - 1);
	}

	function handleMoveDown() {
		if (!canMoveDown) return;
		editorStore.moveBlock(block.id, blockIndex + 1);
	}

	function handleDuplicate() {
		const duplicated = editorStore.duplicateBlock(block.id);
		if (duplicated) {
			editorStore.setActiveBlock(duplicated.id);
		}
	}

	function handleDelete() {
		const nextBlockId = blocks[blockIndex + 1]?.id || blocks[blockIndex - 1]?.id || null;
		editorStore.deleteBlock(block.id);
		if (nextBlockId) {
			editorStore.setActiveBlock(nextBlockId);
		}
	}

	function handleAddAfter() {
		const rect = wrapperRef?.getBoundingClientRect();
		if (rect) {
			editorContext?.openSlashMenu(
				{ x: rect.left + 40, y: rect.bottom + 10 },
				block.id
			);
		}
	}

	function handleBlockUpdate(data: Record<string, unknown>) {
		editorStore.updateBlockData(block.id, data);
	}

	// Wrapper to stop propagation (Svelte 5 style)
	function withStopPropagation(handler: () => void) {
		return (e: MouseEvent) => {
			e.stopPropagation();
			handler();
		};
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	bind:this={wrapperRef}
	class="block-wrapper"
	class:selected={isSelected}
	class:hovered={isHovered}
	role="group"
	tabindex={readonly ? -1 : 0}
	aria-label="{blockTypeName} block"
	onclick={handleSelect}
	onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleSelect(); }}}
	onmouseenter={() => !readonly && (isHovered = true)}
	onmouseleave={() => {
		isHovered = false;
		showMoreMenu = false;
	}}
>
	<!-- Selection/Hover Indicator -->
	{#if (isSelected || isHovered) && !readonly}
		<div
			class="block-indicator"
			class:selected={isSelected}
			transition:fade={{ duration: 100 }}
			aria-hidden="true"
		></div>
	{/if}

	<!-- Drag Handle -->
	{#if (isHovered || isSelected) && !readonly}
		<div
			class="drag-handle"
			data-drag-handle
			title="Drag to reorder"
			aria-label="Drag to reorder block"
			transition:fade={{ duration: 100 }}
		>
			<IconGripVertical size={16} />
		</div>
	{/if}

	<!-- Block Type Badge -->
	{#if (isHovered || isSelected) && !readonly}
		<div
			class="block-type-badge"
			transition:fade={{ duration: 100 }}
		>
			{blockTypeName}
		</div>
	{/if}

	<!-- Block Content -->
	<div class="block-content">
		<BlockRenderer {block} {readonly} onUpdate={handleBlockUpdate} />
	</div>

	<!-- Quick Actions -->
	{#if (isHovered || isSelected) && !readonly}
		<div
			class="block-actions"
			transition:scale={{ duration: 150, start: 0.9 }}
		>
			<button
				type="button"
				class="action-btn"
				class:disabled={!canMoveUp}
				onclick={withStopPropagation(handleMoveUp)}
				title="Move up (⌥↑)"
				aria-label="Move block up"
				disabled={!canMoveUp}
			>
				<IconArrowUp size={14} />
			</button>

			<button
				type="button"
				class="action-btn"
				class:disabled={!canMoveDown}
				onclick={withStopPropagation(handleMoveDown)}
				title="Move down (⌥↓)"
				aria-label="Move block down"
				disabled={!canMoveDown}
			>
				<IconArrowDown size={14} />
			</button>

			<div class="action-divider"></div>

			<button
				type="button"
				class="action-btn"
				onclick={withStopPropagation(handleDuplicate)}
				title="Duplicate (⌘D)"
				aria-label="Duplicate block"
			>
				<IconCopy size={14} />
			</button>

			<button
				type="button"
				class="action-btn"
				onclick={withStopPropagation(handleAddAfter)}
				title="Add block after (⇧Enter)"
				aria-label="Add block after"
			>
				<IconPlus size={14} />
			</button>

			<div class="action-divider"></div>

			<button
				type="button"
				class="action-btn delete"
				onclick={withStopPropagation(handleDelete)}
				title="Delete (⌘⌫)"
				aria-label="Delete block"
			>
				<IconTrash size={14} />
			</button>
		</div>
	{/if}
</div>

<style>
	.block-wrapper {
		position: relative;
		padding: 0.5rem 0.5rem 0.5rem 2.5rem;
		border-radius: 0.5rem;
		transition: all 0.15s;
	}

	.block-wrapper:hover {
		background: rgba(99, 102, 241, 0.03);
	}

	.block-wrapper.selected {
		background: rgba(99, 102, 241, 0.06);
	}

	/* Selection Indicator */
	.block-indicator {
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 3px;
		background: rgba(99, 102, 241, 0.3);
		border-radius: 2px;
		transition: background 0.15s;
	}

	.block-indicator.selected {
		background: #6366f1;
	}

	/* Drag Handle */
	.drag-handle {
		position: absolute;
		left: 0.5rem;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		color: #64748b;
		cursor: grab;
		border-radius: 4px;
		transition: all 0.15s;
	}

	.drag-handle:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #818cf8;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	/* Block Type Badge */
	.block-type-badge {
		position: absolute;
		top: 0.25rem;
		right: 0.5rem;
		padding: 0.125rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
		color: #818cf8;
	}

	/* Block Content */
	.block-content {
		position: relative;
		min-height: 40px;
	}

	/* Quick Actions */
	.block-actions {
		position: absolute;
		right: 0.5rem;
		bottom: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.125rem;
		padding: 0.25rem;
		background: rgba(30, 41, 59, 0.95);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		backdrop-filter: blur(8px);
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 26px;
		height: 26px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	.action-btn.disabled,
	.action-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.action-btn.delete:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.action-divider {
		width: 1px;
		height: 16px;
		background: rgba(51, 65, 85, 0.5);
		margin: 0 0.125rem;
	}
</style>
