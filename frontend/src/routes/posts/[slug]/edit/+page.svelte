<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { setBlockStateManager, BlockStateManager } from '$lib/stores/blockState.svelte';
	import BlockRenderer from '$lib/components/cms/blocks/BlockRenderer.svelte';
	import { createBlock, serializeBlocks, deserializeBlocks } from '$lib/utils/blocks';
	import { IconPlus } from '$lib/icons';
	import type { Block } from '$lib/components/cms/blocks/types';
import type { BlockId } from '$lib/stores/blockState.svelte';

	// Initialize state manager
	const stateManager = new BlockStateManager();
	setBlockStateManager(stateManager);

	// State
	let title = $state('');
	let blocks = $state<Block[]>([]);
	let selectedBlockId = $state<BlockId | null>(null);
	let isEditing = $state(true);
	let isSaving = $state(false);

	// Load post data (in production, fetch from API)
	$effect(() => {
		loadPost();
	});

	async function loadPost(): Promise<void> {
		try {
			const response = await fetch(`/api/posts/${page.params.slug}`);
			if (!response.ok) throw new Error('Failed to load post');

			const data = await response.json();
			title = data.title;
			blocks = Array.isArray(data.content) ? data.content : deserializeBlocks(data.content);
		} catch (error) {
			console.error('Load error:', error);
		}
	}

	async function savePost(): Promise<void> {
		isSaving = true;

		try {
			const response = await fetch(`/api/posts/${page.params.slug}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					title,
					content: blocks
				})
			});

			if (!response.ok) throw new Error('Save failed');

			// Show success message
			alert('Post saved successfully!');
		} catch (error) {
			console.error('Save error:', error);
			alert('Failed to save post');
		} finally {
			isSaving = false;
		}
	}

	function handleBlockUpdate(blockId: BlockId, updates: Partial<Block>): void {
		blocks = blocks.map((block) => (block.id === blockId ? { ...block, ...updates } : block));
	}

	function selectBlock(blockId: BlockId): void {
		selectedBlockId = blockId;
	}

	function addBlock(type: string = 'paragraph'): void {
		const newBlock = createBlock(type as any);
		blocks = [...blocks, newBlock];
		selectedBlockId = newBlock.id;

		// Scroll to new block
		setTimeout(() => {
			const element = document.querySelector(`[data-block-id="${newBlock.id}"]`);
			element?.scrollIntoView({ behavior: 'smooth', block: 'center' });
		}, 100);
	}

	function deleteBlock(blockId: BlockId): void {
		blocks = blocks.filter((b) => b.id !== blockId);
		if (selectedBlockId === blockId) {
			selectedBlockId = null;
		}
	}

	function moveBlockUp(blockId: BlockId): void {
		const index = blocks.findIndex((b) => b.id === blockId);
		if (index > 0) {
			const newBlocks = [...blocks];
			[newBlocks[index - 1], newBlocks[index]] = [newBlocks[index], newBlocks[index - 1]];
			blocks = newBlocks;
		}
	}

	function moveBlockDown(blockId: BlockId): void {
		const index = blocks.findIndex((b) => b.id === blockId);
		if (index < blocks.length - 1) {
			const newBlocks = [...blocks];
			[newBlocks[index], newBlocks[index + 1]] = [newBlocks[index + 1], newBlocks[index]];
			blocks = newBlocks;
		}
	}

	function duplicateBlock(blockId: BlockId): void {
		const index = blocks.findIndex((b) => b.id === blockId);
		if (index !== -1) {
			const original = blocks[index];
			const duplicate = createBlock(original.type, {
				content: { ...original.content },
				settings: { ...original.settings }
			});
			blocks = [...blocks.slice(0, index + 1), duplicate, ...blocks.slice(index + 1)];
		}
	}

	function handleKeyDown(e: KeyboardEvent): void {
		// Cmd/Ctrl + S to save
		if ((e.metaKey || e.ctrlKey) && e.key === 's') {
			e.preventDefault();
			savePost();
		}

		// Delete to remove selected block
		if (e.key === 'Delete' && selectedBlockId && !isEditing) {
			e.preventDefault();
			deleteBlock(selectedBlockId);
		}
	}
</script>

<svelte:window onkeydown={handleKeyDown} />

<svelte:head>
	<title>Edit: {title || 'Untitled'}</title>
</svelte:head>

<div class="editor-layout">
	<!-- Toolbar -->
	<header class="editor-toolbar">
		<div class="toolbar-left">
			<button type="button" class="back-btn" onclick={() => goto('/posts')}>← Back</button>
			<input
				type="text"
				bind:value={title}
				placeholder="Post title..."
				class="title-input"
				disabled={!isEditing}
			/>
		</div>

		<div class="toolbar-right">
			<button type="button" class="preview-btn" onclick={() => (isEditing = !isEditing)}>
				{isEditing ? 'Preview' : 'Edit'}
			</button>
			<button type="button" class="save-btn" onclick={savePost} disabled={isSaving}>
				{isSaving ? 'Saving...' : 'Save'}
			</button>
		</div>
	</header>

	<!-- Main Editor -->
	<main class="editor-main">
		<div class="editor-canvas">
			{#each blocks as block, index (block.id)}
				<div class="block-wrapper-outer">
					{#if isEditing && selectedBlockId === block.id}
						<div class="block-controls">
							<button type="button" onclick={() => moveBlockUp(block.id)} disabled={index === 0}>
								↑
							</button>
							<button
								type="button"
								onclick={() => moveBlockDown(block.id)}
								disabled={index === blocks.length - 1}
							>
								↓
							</button>
							<button type="button" onclick={() => duplicateBlock(block.id)}>⎘</button>
							<button type="button" onclick={() => deleteBlock(block.id)} class="delete-btn">
								×
							</button>
						</div>
					{/if}

					<div
						onclick={() => selectBlock(block.id)}
						onkeydown={(e) => e.key === 'Enter' && selectBlock(block.id)}
						role="button"
						tabindex="0"
					>
						<BlockRenderer
							{block}
							blockId={block.id}
							{isEditing}
							isSelected={selectedBlockId === block.id}
							onUpdate={(updates) => handleBlockUpdate(block.id, updates)}
						/>
					</div>
				</div>
			{/each}

			{#if blocks.length === 0}
				<div class="empty-state">
					<p>No blocks yet. Add your first block to get started!</p>
					<button type="button" class="add-first-block" onclick={() => addBlock()}>
						<IconPlus size={20} />
						Add Block
					</button>
				</div>
			{/if}
		</div>

		<!-- Add Block Menu -->
		{#if isEditing}
			<div class="add-block-menu">
				<button type="button" class="add-block-btn" onclick={() => addBlock('paragraph')}>
					<IconPlus size={16} />
					Paragraph
				</button>
				<button type="button" class="add-block-btn" onclick={() => addBlock('heading')}>
					<IconPlus size={16} />
					Heading
				</button>
				<button type="button" class="add-block-btn" onclick={() => addBlock('image')}>
					<IconPlus size={16} />
					Image
				</button>
				<button type="button" class="add-block-btn" onclick={() => addBlock('code')}>
					<IconPlus size={16} />
					Code
				</button>
			</div>
		{/if}
	</main>
</div>

<style>
	.editor-layout {
		min-height: 100vh;
		background: #f9fafb;
	}

	.editor-toolbar {
		position: sticky;
		top: 0;
		z-index: 100;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 2rem;
		background: white;
		border-bottom: 1px solid #e5e7eb;
	}

	.toolbar-left {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex: 1;
	}

	.back-btn {
		padding: 0.5rem 1rem;
		background: transparent;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		color: #374151;
		cursor: pointer;
	}

	.back-btn:hover {
		background: #f3f4f6;
	}

	.title-input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.title-input:disabled {
		background: transparent;
		border-color: transparent;
	}

	.toolbar-right {
		display: flex;
		gap: 0.75rem;
	}

	.preview-btn,
	.save-btn {
		padding: 0.625rem 1.25rem;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.preview-btn {
		background: white;
		color: #374151;
	}

	.preview-btn:hover {
		background: #f3f4f6;
	}

	.save-btn {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.save-btn:hover:not(:disabled) {
		background: #2563eb;
	}

	.save-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.editor-main {
		max-width: 900px;
		margin: 0 auto;
		padding: 3rem 2rem;
	}

	.editor-canvas {
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 2rem;
		min-height: 500px;
	}

	.block-wrapper-outer {
		position: relative;
		margin: 1rem 0;
	}

	.block-controls {
		position: absolute;
		left: -60px;
		top: 0;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.block-wrapper-outer:hover .block-controls {
		opacity: 1;
	}

	.block-controls button {
		width: 32px;
		height: 32px;
		padding: 0;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.block-controls button:hover:not(:disabled) {
		background: #f3f4f6;
		border-color: #3b82f6;
	}

	.block-controls button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.block-controls .delete-btn:hover {
		background: #fee2e2;
		border-color: #dc2626;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #6b7280;
	}

	.add-first-block {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 1rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		color: white;
		font-weight: 500;
		cursor: pointer;
	}

	.add-block-menu {
		display: flex;
		gap: 0.5rem;
		margin-top: 2rem;
		padding: 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
	}

	.add-block-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 6px;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-block-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	/* Dark Mode */
	:global(.dark) .editor-layout {
		background: #0f172a;
	}

	:global(.dark) .editor-toolbar {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .back-btn {
		background: transparent;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .back-btn:hover {
		background: #334155;
	}

	:global(.dark) .title-input {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .preview-btn {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .preview-btn:hover {
		background: #334155;
	}

	:global(.dark) .editor-canvas {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .add-block-menu {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .add-block-btn {
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .add-block-btn:hover {
		border-color: #60a5fa;
		color: #60a5fa;
	}
</style>
