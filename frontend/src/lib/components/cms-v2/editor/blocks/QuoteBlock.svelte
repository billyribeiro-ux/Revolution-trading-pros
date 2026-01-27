<!--
	Quote Block - Block quote with citation
	═══════════════════════════════════════════════════════════════════════════════

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(block.data as { content?: string; citation?: string });

	function handleContentChange(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		onUpdate?.({ ...data, content: target.value });
	}

	function handleCitationChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, citation: target.value });
	}
</script>

<div class="quote-block">
	<blockquote class="quote-content">
		{#if readonly}
			<p>{data.content || ''}</p>
		{:else}
			<textarea
				class="quote-input"
				value={data.content ?? ''}
				oninput={handleContentChange}
				placeholder="Enter quote text..."
				rows="3"
			></textarea>
		{/if}
	</blockquote>
	{#if data.citation || !readonly}
		<div class="quote-citation">
			{#if readonly}
				<span>— {data.citation}</span>
			{:else}
				<span class="citation-prefix">—</span>
				<input
					type="text"
					class="citation-input"
					value={data.citation ?? ''}
					oninput={handleCitationChange}
					placeholder="Author or source..."
				/>
			{/if}
		</div>
	{/if}
</div>

<style>
	.quote-block {
		padding-left: 1rem;
		border-left: 4px solid #e6b800;
	}

	.quote-content {
		margin: 0;
		padding: 0.5rem 0;
	}

	.quote-content p {
		margin: 0;
		font-size: 1.125rem;
		font-style: italic;
		color: #cbd5e1;
		line-height: 1.6;
	}

	.quote-input {
		width: 100%;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.375rem;
		color: #cbd5e1;
		font-size: 1.125rem;
		font-style: italic;
		line-height: 1.6;
		resize: vertical;
	}

	.quote-input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.quote-citation {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.citation-prefix {
		color: #64748b;
	}

	.citation-input {
		flex: 1;
		padding: 0.375rem 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.25rem;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.citation-input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}
</style>
