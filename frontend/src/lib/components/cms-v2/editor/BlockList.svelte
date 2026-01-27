<!--
	Block List - Renders blocks with drag-and-drop reordering
	═══════════════════════════════════════════════════════════════════════════════

	Renders the array of content blocks with:
	- Drag-and-drop reordering
	- Keyboard navigation between blocks
	- Drop zone indicators
	- Smooth animations

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { flip } from 'svelte/animate';
	import { fade } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { getContext } from 'svelte';
	import { editorStore, type ContentBlock } from '$lib/stores/editor.svelte';
	import BlockWrapper from './BlockWrapper.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		readonly?: boolean;
	}

	let { readonly = false }: Props = $props();

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

	let listRef = $state<HTMLDivElement | null>(null);
	let isDragging = $state(false);
	let draggedBlockId = $state<string | null>(null);
	let dropTargetIndex = $state<number | null>(null);

	// ==========================================================================
	// Derived
	// ==========================================================================

	let blocks = $derived(editorStore.contentBlocks);

	// ==========================================================================
	// Drag and Drop Handlers
	// ==========================================================================

	function handleDragStart(e: DragEvent, block: ContentBlock) {
		if (readonly) return;

		isDragging = true;
		draggedBlockId = block.id;

		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', block.id);
			e.dataTransfer.setData('application/x-block-id', block.id);

			// Create drag image
			const target = e.target as HTMLElement;
			const rect = target.getBoundingClientRect();
			e.dataTransfer.setDragImage(target, rect.width / 2, 20);
		}

		// Select the dragged block
		editorStore.setActiveBlock(block.id);
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (!isDragging || !draggedBlockId) return;

		// Calculate if we're in the top or bottom half
		const target = e.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();
		const midpoint = rect.top + rect.height / 2;
		const isAboveMidpoint = e.clientY < midpoint;

		// Set drop target
		const currentIndex = blocks.findIndex(b => b.id === draggedBlockId);
		let newDropIndex = isAboveMidpoint ? index : index + 1;

		// Don't show indicator if dropping in same position
		if (newDropIndex === currentIndex || newDropIndex === currentIndex + 1) {
			dropTargetIndex = null;
		} else {
			dropTargetIndex = newDropIndex;
		}

		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
	}

	function handleDragLeave(e: DragEvent) {
		const relatedTarget = e.relatedTarget as HTMLElement;
		if (!listRef?.contains(relatedTarget)) {
			dropTargetIndex = null;
		}
	}

	function handleDrop(e: DragEvent, targetIndex: number) {
		e.preventDefault();

		if (!draggedBlockId) {
			resetDragState();
			return;
		}

		const currentIndex = blocks.findIndex(b => b.id === draggedBlockId);
		if (currentIndex === -1) {
			resetDragState();
			return;
		}

		// Calculate final index
		let finalIndex = dropTargetIndex ?? targetIndex;
		if (finalIndex > currentIndex) {
			finalIndex--;
		}

		if (finalIndex !== currentIndex && finalIndex >= 0) {
			editorStore.moveBlock(draggedBlockId, finalIndex);
		}

		resetDragState();
	}

	function handleDragEnd() {
		resetDragState();
	}

	function resetDragState() {
		isDragging = false;
		draggedBlockId = null;
		dropTargetIndex = null;
	}

	// ==========================================================================
	// Keyboard Navigation
	// ==========================================================================

	function handleKeyDown(e: KeyboardEvent, block: ContentBlock, index: number) {
		if (readonly) return;

		const isSelected = editorContext?.selectedBlockId === block.id;
		if (!isSelected) return;

		switch (e.key) {
			case 'ArrowUp':
				if (e.altKey || e.metaKey) {
					// Move block up
					e.preventDefault();
					if (index > 0) {
						editorStore.moveBlock(block.id, index - 1);
					}
				} else if (!e.shiftKey) {
					// Navigate to previous block
					e.preventDefault();
					if (index > 0) {
						editorStore.setActiveBlock(blocks[index - 1].id);
					}
				}
				break;

			case 'ArrowDown':
				if (e.altKey || e.metaKey) {
					// Move block down
					e.preventDefault();
					if (index < blocks.length - 1) {
						editorStore.moveBlock(block.id, index + 1);
					}
				} else if (!e.shiftKey) {
					// Navigate to next block
					e.preventDefault();
					if (index < blocks.length - 1) {
						editorStore.setActiveBlock(blocks[index + 1].id);
					}
				}
				break;

			case 'Delete':
			case 'Backspace':
				if (e.metaKey || e.ctrlKey) {
					e.preventDefault();
					const nextBlockId = blocks[index + 1]?.id || blocks[index - 1]?.id || null;
					editorStore.deleteBlock(block.id);
					if (nextBlockId) {
						editorStore.setActiveBlock(nextBlockId);
					}
				}
				break;

			case 'd':
				if (e.metaKey || e.ctrlKey) {
					e.preventDefault();
					const duplicated = editorStore.duplicateBlock(block.id);
					if (duplicated) {
						editorStore.setActiveBlock(duplicated.id);
					}
				}
				break;

			case 'Enter':
				if (e.shiftKey) {
					// Add block after current
					e.preventDefault();
					const wrapper = e.currentTarget as HTMLElement;
					const rect = wrapper.getBoundingClientRect();
					editorContext?.openSlashMenu(
						{ x: rect.left + 40, y: rect.bottom + 10 },
						block.id
					);
				}
				break;
		}
	}
</script>

<div
	bind:this={listRef}
	class="block-list"
	role="list"
	aria-label="Content blocks"
>
	{#each blocks as block, index (block.id)}
		{@const isSelected = editorContext?.selectedBlockId === block.id}
		{@const isDraggedBlock = draggedBlockId === block.id}

		<!-- Drop Zone Indicator (before) -->
		{#if dropTargetIndex === index && !isDraggedBlock}
			<div
				class="drop-indicator"
				transition:fade={{ duration: 100 }}
				aria-hidden="true"
			>
				<div class="drop-line"></div>
			</div>
		{/if}

		<div
			class="block-list-item"
			class:dragging={isDraggedBlock}
			class:selected={isSelected}
			role="listitem"
			draggable={!readonly}
			ondragstart={(e) => handleDragStart(e, block)}
			ondragover={(e) => handleDragOver(e, index)}
			ondragleave={handleDragLeave}
			ondrop={(e) => handleDrop(e, index)}
			ondragend={handleDragEnd}
			onkeydown={(e) => handleKeyDown(e, block, index)}
			tabindex={readonly ? -1 : 0}
			animate:flip={{ duration: 250, easing: quintOut }}
		>
			<BlockWrapper {block} {readonly} />
		</div>

		<!-- Drop Zone Indicator (after last) -->
		{#if index === blocks.length - 1 && dropTargetIndex === blocks.length && !isDraggedBlock}
			<div
				class="drop-indicator"
				transition:fade={{ duration: 100 }}
				aria-hidden="true"
			>
				<div class="drop-line"></div>
			</div>
		{/if}
	{/each}
</div>

<style>
	.block-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.block-list-item {
		position: relative;
		outline: none;
		transition: opacity 0.2s, transform 0.2s;
	}

	.block-list-item:focus-visible {
		outline: 2px solid rgba(99, 102, 241, 0.5);
		outline-offset: 2px;
		border-radius: 0.5rem;
	}

	.block-list-item.dragging {
		opacity: 0.5;
		transform: scale(0.98);
	}

	.block-list-item.selected {
		z-index: 1;
	}

	/* Drop Indicator */
	.drop-indicator {
		position: relative;
		height: 4px;
		margin: -2px 0;
	}

	.drop-line {
		position: absolute;
		left: 0;
		right: 0;
		top: 50%;
		height: 3px;
		background: linear-gradient(90deg, #6366f1, #8b5cf6);
		border-radius: 2px;
		transform: translateY(-50%);
		box-shadow: 0 0 8px rgba(99, 102, 241, 0.5);
	}

	.drop-line::before,
	.drop-line::after {
		content: '';
		position: absolute;
		top: 50%;
		width: 8px;
		height: 8px;
		background: #6366f1;
		border-radius: 50%;
		transform: translateY(-50%);
	}

	.drop-line::before {
		left: 0;
	}

	.drop-line::after {
		right: 0;
	}
</style>
