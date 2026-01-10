<script lang="ts">
	/**
	 * Builder Canvas - Drop Zone & Block Display
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Main canvas area where blocks are displayed and can be reordered.
	 * Handles drag-drop interactions and block selection.
	 */

	import type { BuilderStore } from '../store.svelte';
	import type { PageBlock } from '../types';
	import { getComponentByType } from '../registry';
	import BlockWrapper from './BlockWrapper.svelte';
	import BlockPreview from './BlockPreview.svelte';

	interface Props {
		store: BuilderStore;
	}

	let { store }: Props = $props();

	let canvasElement: HTMLDivElement;

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'copy';
		}

		// Calculate drop index based on mouse position
		if (canvasElement && store.dragState.isDragging) {
			const blocks = canvasElement.querySelectorAll('.block-wrapper');
			let dropIndex = store.sortedBlocks.length;

			for (let i = 0; i < blocks.length; i++) {
				const block = blocks[i] as HTMLElement;
				const rect = block.getBoundingClientRect();
				const midpoint = rect.top + rect.height / 2;

				if (e.clientY < midpoint) {
					dropIndex = i;
					break;
				}
			}

			store.updateDropTarget(dropIndex);
		}
	}

	function handleDragLeave(e: DragEvent) {
		// Only clear if leaving the canvas entirely
		if (e.relatedTarget && !canvasElement?.contains(e.relatedTarget as Node)) {
			store.updateDropTarget(null);
		}
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		store.endDrag();
	}

	function handleCanvasClick(e: MouseEvent) {
		// Deselect if clicking on canvas background
		if (e.target === canvasElement || (e.target as HTMLElement).classList.contains('canvas-inner')) {
			store.clearSelection();
		}
	}
</script>

<div 
	class="canvas"
	class:preview-mode={store.isPreviewMode}
	class:dragging={store.dragState.isDragging}
>
	<div class="canvas-header">
		<input 
			type="text" 
			class="course-title-input"
			value={store.layout.title}
			oninput={(e) => store.updateLayoutMeta({ title: (e.target as HTMLInputElement).value })}
			placeholder="Course Title"
			disabled={store.isPreviewMode}
		/>
		<div class="canvas-actions">
			<button 
				class="action-btn"
				class:active={store.isPreviewMode}
				onclick={() => store.togglePreviewMode()}
			>
				{#if store.isPreviewMode}
					<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/></svg>
					Edit
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>
					Preview
				{/if}
			</button>
		</div>
	</div>

	<div 
		class="canvas-body"
		bind:this={canvasElement}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		onclick={handleCanvasClick}
		onkeydown={(e) => { if (e.key === 'Escape') store.clearSelection(); }}
		role="application"
		tabindex="0"
		aria-label="Page builder canvas - drag and drop components here"
	>
		<div class="canvas-inner">
			{#if store.sortedBlocks.length === 0 && !store.dragState.isDragging}
				<div class="empty-state">
					<div class="empty-icon">🎨</div>
					<h3>Start Building</h3>
					<p>Drag components from the left panel or click to add them to your course page.</p>
				</div>
			{:else}
				{#each store.sortedBlocks as block, index (block.id)}
					<!-- Drop indicator before block -->
					{#if store.dragState.isDragging && store.dragState.dropTargetIndex === index}
						<div class="drop-indicator">
							<div class="drop-line"></div>
						</div>
					{/if}

					<BlockWrapper 
						{block}
						{store}
						isSelected={store.selectedBlockId === block.id}
						isPreview={store.isPreviewMode}
					/>
				{/each}

				<!-- Drop indicator at end -->
				{#if store.dragState.isDragging && store.dragState.dropTargetIndex === store.sortedBlocks.length}
					<div class="drop-indicator">
						<div class="drop-line"></div>
					</div>
				{/if}
			{/if}

			{#if store.sortedBlocks.length === 0 && store.dragState.isDragging}
				<div class="drop-zone-empty">
					<div class="drop-zone-content">
						<span class="drop-icon">⬇️</span>
						<span>Drop here to add</span>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.canvas {
		flex: 1;
		display: flex;
		flex-direction: column;
		background: #F3F4F6;
		min-width: 0;
		overflow: hidden;
	}

	.canvas-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		background: #FFFFFF;
		border-bottom: 1px solid #E5E7EB;
		gap: 16px;
	}

	.course-title-input {
		flex: 1;
		font-size: 20px;
		font-weight: 600;
		color: #1F2937;
		border: none;
		background: transparent;
		padding: 8px 0;
		outline: none;
	}

	.course-title-input:focus {
		border-bottom: 2px solid #143E59;
	}

	.course-title-input:disabled {
		color: #374151;
	}

	.canvas-actions {
		display: flex;
		gap: 8px;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		background: #F3F4F6;
		border: 1px solid #E5E7EB;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn:hover {
		background: #E5E7EB;
	}

	.action-btn.active {
		background: #143E59;
		border-color: #143E59;
		color: white;
	}

	.canvas-body {
		flex: 1;
		overflow-y: auto;
		padding: 32px;
	}

	.canvas-inner {
		max-width: 900px;
		margin: 0 auto;
		min-height: 100%;
		background: #FFFFFF;
		border-radius: 12px;
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.08);
		padding: 24px;
	}

	.preview-mode .canvas-inner {
		box-shadow: none;
		border-radius: 0;
		padding: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 40px;
		text-align: center;
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #1F2937;
		margin: 0 0 8px 0;
	}

	.empty-state p {
		font-size: 14px;
		color: #6B7280;
		margin: 0;
		max-width: 300px;
	}

	.drop-indicator {
		padding: 8px 0;
	}

	.drop-line {
		height: 3px;
		background: linear-gradient(90deg, #143E59 0%, #1E73BE 100%);
		border-radius: 2px;
		animation: pulse 1s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.drop-zone-empty {
		border: 2px dashed #143E59;
		border-radius: 12px;
		padding: 60px;
		background: rgba(20, 62, 89, 0.05);
	}

	.drop-zone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		color: #143E59;
		font-weight: 500;
	}

	.drop-icon {
		font-size: 32px;
		animation: bounce 0.5s ease infinite;
	}

	@keyframes bounce {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(-5px); }
	}

	.dragging .canvas-inner {
		background: #FAFAFA;
	}

	/* Scrollbar */
	.canvas-body::-webkit-scrollbar {
		width: 8px;
	}

	.canvas-body::-webkit-scrollbar-track {
		background: transparent;
	}

	.canvas-body::-webkit-scrollbar-thumb {
		background: #D1D5DB;
		border-radius: 4px;
	}

	.canvas-body::-webkit-scrollbar-thumb:hover {
		background: #9CA3AF;
	}
</style>
