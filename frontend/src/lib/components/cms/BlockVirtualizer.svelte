<!--
/**
 * Block Virtualizer
 * ═══════════════════════════════════════════════════════════════════════════
 * Virtual scrolling for documents with 100+ blocks
 * Target: Render 1000 blocks with 60fps
 */
-->

<script lang="ts">
	import { createVirtualizer } from '@tanstack/svelte-virtual';
	import { get } from 'svelte/store';
	import type { Block } from './blocks/types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		blocks: Block[];
		containerHeight: number;
		isEditing: boolean;
		selectedBlockId: BlockId | null;
		onUpdate: (blockId: string, updates: Partial<Block>) => void;
		onSelect: (blockId: string) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	let scrollElement = $state<HTMLElement | null>(null);

	let virtualizerStore = $derived(
		scrollElement
			? createVirtualizer({
					count: props.blocks.length,
					getScrollElement: () => scrollElement,
					estimateSize: (index) => {
						const block = props.blocks[index];
						return getEstimatedHeight(block);
					},
					overscan: 5
				})
			: null
	);

	let virtualItems = $derived(virtualizerStore ? get(virtualizerStore).getVirtualItems() : []);
	let totalSize = $derived(virtualizerStore ? get(virtualizerStore).getTotalSize() : 0);

	function getEstimatedHeight(block: Block): number {
		switch (block.type) {
			case 'heading':
				return 60;
			case 'paragraph':
				return 80;
			case 'image':
			case 'video':
				return 300;
			case 'gallery':
				return 400;
			case 'accordion':
			case 'tabs':
				return 200;
			case 'code':
				return 150;
			default:
				return 100;
		}
	}

	function _handleBlockUpdate(blockId: string, updates: Partial<Block>): void {
		props.onUpdate(blockId, updates);
	}
	void _handleBlockUpdate;
</script>

<div
	bind:this={scrollElement}
	class="block-virtualizer"
	style="height: {props.containerHeight}px;"
	role="list"
	aria-label="Document blocks"
>
	<div class="virtual-container" style="height: {totalSize}px;">
		{#each virtualItems as virtualRow (virtualRow.key)}
			{@const block = props.blocks[virtualRow.index]}
			{@const isSelected = props.selectedBlockId === block.id}
			<div
				data-index={virtualRow.index}
				class="virtual-item"
				class:selected={isSelected}
				style="transform: translateY({virtualRow.start}px);"
				role="listitem"
			>
				<button
					type="button"
					class="block-wrapper"
					onclick={() => props.onSelect(block.id)}
					onkeydown={(e) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							props.onSelect(block.id);
						}
					}}
				>
					<div class="block-type-indicator">{block.type}</div>
					<div class="block-preview">
						{#if block.content.text}
							<p class="preview-text">{block.content.text.slice(0, 100)}...</p>
						{:else if block.content.html}
							<p class="preview-text">
								{block.content.html.replace(/<[^>]*>/g, '').slice(0, 100)}...
							</p>
						{:else if block.content.mediaUrl}
							<span class="preview-media">Media: {block.content.mediaUrl.split('/').pop()}</span>
						{:else}
							<span class="preview-empty">Empty {block.type} block</span>
						{/if}
					</div>
				</button>
			</div>
		{/each}
	</div>
</div>

<style>
	.block-virtualizer {
		overflow-y: auto;
		overflow-x: hidden;
		contain: strict;
		will-change: scroll-position;
		scrollbar-width: thin;
		scrollbar-color: #cbd5e1 transparent;
	}

	.block-virtualizer::-webkit-scrollbar {
		width: 8px;
	}

	.block-virtualizer::-webkit-scrollbar-track {
		background: transparent;
	}

	.block-virtualizer::-webkit-scrollbar-thumb {
		background: #cbd5e1;
		border-radius: 4px;
	}

	.virtual-container {
		width: 100%;
		position: relative;
	}

	.virtual-item {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		padding: 0.5rem;
	}

	.virtual-item.selected .block-wrapper {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	.block-wrapper {
		width: 100%;
		padding: 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
		display: block;
	}

	.block-wrapper:hover {
		border-color: #3b82f6;
		background: #f8fafc;
	}

	.block-wrapper:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.block-type-indicator {
		display: inline-block;
		padding: 0.25rem 0.5rem;
		background: #f1f5f9;
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		color: #64748b;
		margin-bottom: 0.5rem;
	}

	.block-preview {
		font-size: 0.875rem;
		color: #374151;
		line-height: 1.5;
	}

	.preview-text {
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.preview-media {
		color: #3b82f6;
		font-weight: 500;
	}

	.preview-empty {
		color: #9ca3af;
		font-style: italic;
	}

	:global(.dark) .block-wrapper {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .block-wrapper:hover {
		background: #334155;
	}

	:global(.dark) .block-type-indicator {
		background: #334155;
		color: #94a3b8;
	}

	:global(.dark) .block-preview {
		color: #e2e8f0;
	}
</style>
