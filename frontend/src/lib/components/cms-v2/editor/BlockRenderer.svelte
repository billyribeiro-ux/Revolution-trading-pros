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
	import type { Component as SvelteComponent } from 'svelte';

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
	// Types
	// ==========================================================================

	interface BlockProps {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

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

	// Using 'any' for component map to avoid complex Svelte 5 component typing
	const blockComponents: Record<string, SvelteComponent<BlockProps>> = {
		'rich-text': RichTextBlock as unknown as SvelteComponent<BlockProps>,
		'paragraph': RichTextBlock as unknown as SvelteComponent<BlockProps>,
		'text': RichTextBlock as unknown as SvelteComponent<BlockProps>,
		'heading': HeadingBlock as unknown as SvelteComponent<BlockProps>,
		'image': ImageBlock as unknown as SvelteComponent<BlockProps>,
		'video': VideoBlock as unknown as SvelteComponent<BlockProps>,
		'divider': DividerBlock as unknown as SvelteComponent<BlockProps>,
		'spacer': SpacerBlock as unknown as SvelteComponent<BlockProps>,
		'quote': QuoteBlock as unknown as SvelteComponent<BlockProps>,
		'blockquote': QuoteBlock as unknown as SvelteComponent<BlockProps>,
		'code': CodeBlock as unknown as SvelteComponent<BlockProps>,
		'code-block': CodeBlock as unknown as SvelteComponent<BlockProps>,
		'list': ListBlock as unknown as SvelteComponent<BlockProps>,
		'bullet-list': ListBlock as unknown as SvelteComponent<BlockProps>,
		'numbered-list': ListBlock as unknown as SvelteComponent<BlockProps>,
		'callout': CalloutBlock as unknown as SvelteComponent<BlockProps>,
		'alert': CalloutBlock as unknown as SvelteComponent<BlockProps>,
		'trade-setup': TradeSetupBlock as unknown as SvelteComponent<BlockProps>
	};
</script>

{#if blockComponents[block.blockType]}
	{@const BlockComponent = blockComponents[block.blockType]}
	<BlockComponent {block} {readonly} {onUpdate} />
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
