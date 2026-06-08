<!--
	BlockLayersPanel — sidebar panel listing all blocks in order; clicking selects a block.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconGripVertical } from '$lib/icons';
	import type { Block } from '../types';
	import { BLOCK_DEFINITIONS } from '../types';

	interface Props {
		blocks: Block[];
		selectedBlockId: string | null;
		onSelectBlock: (id: string) => void;
	}

	let { blocks, selectedBlockId, onSelectBlock }: Props = $props();
</script>

<div id="panel-layers" role="tabpanel" aria-labelledby="tab-layers">
	<div class="panel-header">
		<h3>Block Layers</h3>
	</div>
	<div class="layers-list" role="listbox" aria-label="Content blocks">
		{#each blocks as block, i (block.id)}
			<button
				type="button"
				class={['layer-item', { selected: block.id === selectedBlockId }]}
				onclick={() => onSelectBlock(block.id)}
				role="option"
				aria-selected={block.id === selectedBlockId}
				aria-label="{BLOCK_DEFINITIONS[block.type]?.name || block.type}, position {i + 1}"
			>
				<IconGripVertical size={14} aria-hidden="true" />
				<span class="layer-type">{BLOCK_DEFINITIONS[block.type]?.name || block.type}</span>
				<span class="layer-index">#{i + 1}</span>
			</button>
		{/each}
	</div>
</div>

<style>
	.panel-header {
		margin-bottom: 1rem;
	}

	.panel-header h3 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.layers-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.layer-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.75rem;
		background: #f5f5f5;
		border: 1px solid transparent;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.8125rem;
		text-align: left;
		transition: all 0.15s;
	}

	.layer-item:hover {
		background: #e5e5e5;
	}

	.layer-item.selected {
		background: #dbeafe;
		border-color: #3b82f6;
	}

	.layer-item:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.layer-type {
		flex: 1;
		font-weight: 500;
	}

	.layer-index {
		color: #999;
		font-size: 0.75rem;
	}
</style>
