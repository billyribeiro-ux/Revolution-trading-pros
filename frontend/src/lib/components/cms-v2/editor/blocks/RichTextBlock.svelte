<!--
	Rich Text Block - Wraps the existing RichTextEditor
	═══════════════════════════════════════════════════════════════════════════════

	Uses the existing RichTextEditor component as a block within the block editor.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import RichTextEditor from '$lib/components/cms/RichTextEditor.svelte';

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
	// Derived
	// ==========================================================================

	let content = $derived((block.data as { content?: string })?.content ?? '');

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleChange(newContent: string) {
		onUpdate?.({ content: newContent });
	}
</script>

<div class="rich-text-block">
	{#if readonly}
		<div class="rich-text-preview">
			{@html content}
		</div>
	{:else}
		<RichTextEditor
			value={content}
			onchange={handleChange}
			placeholder="Start writing..."
			minHeight="100px"
			disabled={readonly}
		/>
	{/if}
</div>

<style>
	.rich-text-block {
		width: 100%;
	}

	.rich-text-preview {
		padding: 0.5rem;
		color: #f1f5f9;
		line-height: 1.7;
	}

	.rich-text-preview :global(h1) {
		font-size: 2rem;
		font-weight: 700;
		margin: 1rem 0 0.75rem;
		color: #f1f5f9;
	}

	.rich-text-preview :global(h2) {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 0.875rem 0 0.625rem;
		color: #f1f5f9;
	}

	.rich-text-preview :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0.75rem 0 0.5rem;
		color: #f1f5f9;
	}

	.rich-text-preview :global(p) {
		margin: 0.625rem 0;
	}

	.rich-text-preview :global(a) {
		color: #e6b800;
		text-decoration: underline;
	}

	.rich-text-preview :global(ul),
	.rich-text-preview :global(ol) {
		margin: 0.625rem 0;
		padding-left: 1.5rem;
	}

	.rich-text-preview :global(blockquote) {
		margin: 0.75rem 0;
		padding: 0.625rem 1rem;
		border-left: 3px solid #e6b800;
		background: rgba(230, 184, 0, 0.1);
		color: #cbd5e1;
		font-style: italic;
	}
</style>
