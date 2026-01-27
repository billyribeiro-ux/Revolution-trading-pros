<!--
	Block Toolbar - Floating toolbar for selected block
	═══════════════════════════════════════════════════════════════════════════════

	Shows contextual actions for the currently selected block.
	Positioned relative to the selected block.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { editorStore } from '$lib/stores/editor.svelte';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte/icons/arrow-down';
	import IconDotsVertical from '@tabler/icons-svelte/icons/dots-vertical';
	import IconTransform from '@tabler/icons-svelte/icons/transform';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		blockId: string;
	}

	let { blockId }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let showMoreMenu = $state(false);

	// ==========================================================================
	// Derived
	// ==========================================================================

	let blocks = $derived(editorStore.contentBlocks);
	let blockIndex = $derived(blocks.findIndex(b => b.id === blockId));
	let block = $derived(blocks[blockIndex]);
	let canMoveUp = $derived(blockIndex > 0);
	let canMoveDown = $derived(blockIndex < blocks.length - 1);

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleMoveUp() {
		if (!canMoveUp) return;
		editorStore.moveBlock(blockId, blockIndex - 1);
	}

	function handleMoveDown() {
		if (!canMoveDown) return;
		editorStore.moveBlock(blockId, blockIndex + 1);
	}

	function handleDuplicate() {
		const duplicated = editorStore.duplicateBlock(blockId);
		if (duplicated) {
			editorStore.setActiveBlock(duplicated.id);
		}
	}

	function handleDelete() {
		const nextBlockId = blocks[blockIndex + 1]?.id || blocks[blockIndex - 1]?.id || null;
		editorStore.deleteBlock(blockId);
		if (nextBlockId) {
			editorStore.setActiveBlock(nextBlockId);
		}
	}

	function handleToggleMoreMenu() {
		showMoreMenu = !showMoreMenu;
	}
</script>

{#if block}
	<div
		class="block-toolbar"
		role="toolbar"
		aria-label="Block actions"
		transition:scale={{ duration: 150, start: 0.95 }}
	>
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={handleMoveUp}
				disabled={!canMoveUp}
				title="Move up (⌥↑)"
				aria-label="Move block up"
			>
				<IconArrowUp size={16} />
			</button>

			<button
				type="button"
				class="toolbar-btn"
				onclick={handleMoveDown}
				disabled={!canMoveDown}
				title="Move down (⌥↓)"
				aria-label="Move block down"
			>
				<IconArrowDown size={16} />
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={handleDuplicate}
				title="Duplicate (⌘D)"
				aria-label="Duplicate block"
			>
				<IconCopy size={16} />
			</button>

			<button
				type="button"
				class="toolbar-btn delete"
				onclick={handleDelete}
				title="Delete (⌘⌫)"
				aria-label="Delete block"
			>
				<IconTrash size={16} />
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<div class="toolbar-group">
			<div class="more-menu-container">
				<button
					type="button"
					class="toolbar-btn"
					class:active={showMoreMenu}
					onclick={handleToggleMoreMenu}
					title="More options"
					aria-label="More options"
					aria-expanded={showMoreMenu}
				>
					<IconDotsVertical size={16} />
				</button>

				{#if showMoreMenu}
					<div
						class="more-menu"
						transition:scale={{ duration: 150, start: 0.95 }}
					>
						<button
							type="button"
							class="more-menu-item"
							onclick={() => {
								// Transform block type
								showMoreMenu = false;
							}}
						>
							<IconTransform size={16} />
							<span>Transform to...</span>
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.block-toolbar {
		position: fixed;
		bottom: 1.5rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem;
		background: rgba(15, 23, 42, 0.95);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 0.75rem;
		box-shadow:
			0 20px 40px -8px rgba(0, 0, 0, 0.5),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset;
		backdrop-filter: blur(12px);
		z-index: 100;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.125rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	.toolbar-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.toolbar-btn.active {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.toolbar-btn.delete:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.toolbar-divider {
		width: 1px;
		height: 24px;
		background: rgba(51, 65, 85, 0.5);
		margin: 0 0.25rem;
	}

	/* More Menu */
	.more-menu-container {
		position: relative;
	}

	.more-menu {
		position: absolute;
		bottom: calc(100% + 0.5rem);
		right: 0;
		min-width: 180px;
		padding: 0.375rem;
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 0.5rem;
		box-shadow: 0 10px 30px -4px rgba(0, 0, 0, 0.4);
	}

	.more-menu-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #cbd5e1;
		font-size: 0.8125rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
	}

	.more-menu-item:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}
</style>
