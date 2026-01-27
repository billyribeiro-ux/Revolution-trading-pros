<!--
	Block Renderer - Renders the correct component based on block type
	═══════════════════════════════════════════════════════════════════════════════

	Maps block types to their corresponding Svelte components.
	Handles unknown block types gracefully.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';

	// Block Components
	import RichTextBlock from './blocks/RichTextBlock.svelte';
	import HeadingBlock from './blocks/HeadingBlock.svelte';
	import ImageBlock from './blocks/ImageBlock.svelte';
	import DividerBlock from './blocks/DividerBlock.svelte';
	import SpacerBlock from './blocks/SpacerBlock.svelte';
	import QuoteBlock from './blocks/QuoteBlock.svelte';
	import CodeBlock from './blocks/CodeBlock.svelte';
	import ListBlock from './blocks/ListBlock.svelte';
	import CalloutBlock from './blocks/CalloutBlock.svelte';
	import TradeSetupBlock from './blocks/TradeSetupBlock.svelte';
	import VideoBlock from './blocks/VideoBlock.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	// ==========================================================================
	// Block Component Map
	// ==========================================================================

	const blockComponents: Record<string, typeof RichTextBlock> = {
		'rich-text': RichTextBlock,
		'paragraph': RichTextBlock,
		'text': RichTextBlock,
		'heading': HeadingBlock,
		'image': ImageBlock,
		'video': VideoBlock,
		'divider': DividerBlock,
		'spacer': SpacerBlock,
		'quote': QuoteBlock,
		'blockquote': QuoteBlock,
		'code': CodeBlock,
		'code-block': CodeBlock,
		'list': ListBlock,
		'bullet-list': ListBlock,
		'numbered-list': ListBlock,
		'callout': CalloutBlock,
		'alert': CalloutBlock,
		'trade-setup': TradeSetupBlock
	};

	// ==========================================================================
	// Derived
	// ==========================================================================

	let Component = $derived(blockComponents[block.blockType]);
</script>

{#if Component}
	<svelte:component
		this={Component}
		{block}
		{readonly}
		{onUpdate}
	/>
{:else}
	<!-- Unknown Block Type -->
	<div class="unknown-block">
		<div class="unknown-icon">
			<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"/>
				<path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
				<line x1="12" y1="17" x2="12.01" y2="17"/>
			</svg>
		</div>
		<div class="unknown-content">
			<span class="unknown-type">Unknown block type: <code>{block.blockType}</code></span>
			<span class="unknown-hint">This block type is not yet supported</span>
		</div>
		{#if !readonly}
			<details class="unknown-data">
				<summary>View data</summary>
				<pre>{JSON.stringify(block.data, null, 2)}</pre>
			</details>
		{/if}
	</div>
{/if}

<style>
	.unknown-block {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px dashed rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
	}

	.unknown-icon {
		color: #ef4444;
	}

	.unknown-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.unknown-type {
		font-size: 0.875rem;
		color: #f1f5f9;
	}

	.unknown-type code {
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 0.25rem;
		font-family: 'Fira Code', monospace;
		font-size: 0.8125rem;
		color: #ef4444;
	}

	.unknown-hint {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.unknown-data {
		margin-top: 0.5rem;
	}

	.unknown-data summary {
		font-size: 0.75rem;
		color: #64748b;
		cursor: pointer;
	}

	.unknown-data summary:hover {
		color: #94a3b8;
	}

	.unknown-data pre {
		margin-top: 0.5rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 0.375rem;
		font-family: 'Fira Code', monospace;
		font-size: 0.75rem;
		color: #94a3b8;
		overflow-x: auto;
	}
</style>
