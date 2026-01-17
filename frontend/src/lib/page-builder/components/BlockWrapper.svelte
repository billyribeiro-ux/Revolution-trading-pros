<script lang="ts">
	/**
	 * Block Wrapper - Individual Block Container
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Wraps each block with selection, drag handles, and action buttons.
	 */

	import type { BuilderStore } from '../store.svelte';
	import type { PageBlock } from '../types';
	import { getComponentByType } from '../registry';
	import BlockPreview from './BlockPreview.svelte';

	interface Props {
		block: PageBlock;
		store: BuilderStore;
		isSelected: boolean;
		isPreview: boolean;
	}

	let { block, store, isSelected, isPreview }: Props = $props();

	const componentInfo = $derived(getComponentByType(block.type));

	function handleClick(e: MouseEvent) {
		e.stopPropagation();
		if (!isPreview) {
			store.selectBlock(block.id);
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			if (!isPreview) {
				store.selectBlock(block.id);
			}
		}
		if (e.key === 'Delete' || e.key === 'Backspace') {
			if (isSelected && !isPreview) {
				store.removeBlock(block.id);
			}
		}
	}

	function handleDragStart(e: DragEvent) {
		if (isPreview) {
			e.preventDefault();
			return;
		}
		store.startDragExisting(block.id);
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', block.id);
		}
	}

	function handleDragEnd() {
		store.endDrag();
	}

	function handleDuplicate() {
		store.duplicateBlock(block.id);
	}

	function handleDelete() {
		store.removeBlock(block.id);
	}

	function handleMoveUp() {
		if (block.order > 0) {
			store.moveBlock(block.id, block.order - 1);
		}
	}

	function handleMoveDown() {
		if (block.order < store.blockCount - 1) {
			store.moveBlock(block.id, block.order + 1);
		}
	}
</script>

<div
	class="block-wrapper"
	class:selected={isSelected}
	class:preview={isPreview}
	class:dragging={store.dragState.draggedBlockId === block.id}
	draggable={!isPreview}
	ondragstart={handleDragStart}
	ondragend={handleDragEnd}
	onclick={handleClick}
	onkeydown={handleKeyDown}
	tabindex={isPreview ? -1 : 0}
	role="button"
	aria-label={`${componentInfo?.name ?? 'Block'} - Click to select`}
	aria-pressed={isSelected}
>
	{#if !isPreview}
		<!-- Block Label -->
		<div class="block-label">
			<span class="block-icon">{componentInfo?.icon ?? '📦'}</span>
			<span class="block-name">{componentInfo?.name ?? 'Block'}</span>
		</div>

		<!-- Drag Handle -->
		<div class="drag-handle" aria-hidden="true">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="currentColor"
			>
				<circle cx="9" cy="5" r="1.5" />
				<circle cx="15" cy="5" r="1.5" />
				<circle cx="9" cy="12" r="1.5" />
				<circle cx="15" cy="12" r="1.5" />
				<circle cx="9" cy="19" r="1.5" />
				<circle cx="15" cy="19" r="1.5" />
			</svg>
		</div>

		<!-- Action Buttons (visible on hover/select) -->
		<div class="block-actions">
			<button
				class="action-btn"
				onclick={(e) => {
					e.stopPropagation();
					handleMoveUp();
				}}
				disabled={block.order === 0}
				title="Move up"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"><path d="m18 15-6-6-6 6" /></svg
				>
			</button>
			<button
				class="action-btn"
				onclick={(e) => {
					e.stopPropagation();
					handleMoveDown();
				}}
				disabled={block.order === store.blockCount - 1}
				title="Move down"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"><path d="m6 9 6 6 6-6" /></svg
				>
			</button>
			<button
				class="action-btn"
				onclick={(e) => {
					e.stopPropagation();
					handleDuplicate();
				}}
				title="Duplicate"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><rect width="14" height="14" x="8" y="8" rx="2" /><path
						d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
					/></svg
				>
			</button>
			<button
				class="action-btn delete"
				onclick={(e) => {
					e.stopPropagation();
					handleDelete();
				}}
				title="Delete"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
						d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
					/></svg
				>
			</button>
		</div>
	{/if}

	<!-- Block Content -->
	<div class="block-content">
		<BlockPreview {block} {isPreview} />
	</div>
</div>

<style>
	.block-wrapper {
		position: relative;
		margin-bottom: 16px;
		border: 2px solid transparent;
		border-radius: 8px;
		transition: all 0.15s ease;
		cursor: pointer;
		outline: none;
	}

	.block-wrapper:last-child {
		margin-bottom: 0;
	}

	.block-wrapper:hover:not(.preview) {
		border-color: #e5e7eb;
	}

	.block-wrapper.selected {
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.block-wrapper.preview {
		cursor: default;
		margin-bottom: 0;
	}

	.block-wrapper.dragging {
		opacity: 0.5;
	}

	.block-wrapper:focus-visible {
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.2);
	}

	/* Block Label */
	.block-label {
		position: absolute;
		top: -10px;
		left: 12px;
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 2px 8px;
		background: #143e59;
		color: white;
		font-size: 10px;
		font-weight: 600;
		border-radius: 4px;
		opacity: 0;
		transform: translateY(4px);
		transition: all 0.15s ease;
		z-index: 1;
	}

	.block-wrapper:hover .block-label,
	.block-wrapper.selected .block-label {
		opacity: 1;
		transform: translateY(0);
	}

	.block-icon {
		font-size: 12px;
	}

	/* Drag Handle */
	.drag-handle {
		position: absolute;
		left: -24px;
		top: 50%;
		transform: translateY(-50%);
		width: 20px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #9ca3af;
		cursor: grab;
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.block-wrapper:hover .drag-handle,
	.block-wrapper.selected .drag-handle {
		opacity: 1;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	/* Action Buttons */
	.block-actions {
		position: absolute;
		top: -10px;
		right: 12px;
		display: flex;
		gap: 4px;
		opacity: 0;
		transform: translateY(4px);
		transition: all 0.15s ease;
		z-index: 1;
	}

	.block-wrapper:hover .block-actions,
	.block-wrapper.selected .block-actions {
		opacity: 1;
		transform: translateY(0);
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: #ffffff;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.1s ease;
	}

	.action-btn:hover:not(:disabled) {
		background: #f3f4f6;
		border-color: #143e59;
		color: #143e59;
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.action-btn.delete:hover:not(:disabled) {
		background: #fee2e2;
		border-color: #dc2626;
		color: #dc2626;
	}

	/* Block Content */
	.block-content {
		position: relative;
	}

	.preview .block-content {
		pointer-events: auto;
	}
</style>
