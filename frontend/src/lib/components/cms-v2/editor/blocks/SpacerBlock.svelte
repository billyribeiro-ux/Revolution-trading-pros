<!--
	Spacer Block - Vertical spacing element
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

	let height = $derived((block.data as { height?: string })?.height ?? 'md');

	const heightMap: Record<string, string> = {
		sm: '1rem',
		md: '2rem',
		lg: '3rem',
		xl: '4rem'
	};

	function handleHeightChange(newHeight: string) {
		onUpdate?.({ height: newHeight });
	}
</script>

<div class="spacer-block">
	{#if !readonly}
		<div class="height-selector">
			{#each ['sm', 'md', 'lg', 'xl'] as h}
				<button
					type="button"
					class="height-btn"
					class:active={height === h}
					onclick={() => handleHeightChange(h)}
				>
					{h.toUpperCase()}
				</button>
			{/each}
		</div>
	{/if}
	<div class="spacer" style:height={heightMap[height]}>
		{#if !readonly}
			<span class="spacer-label">{heightMap[height]} spacer</span>
		{/if}
	</div>
</div>

<style>
	.spacer-block {
		position: relative;
	}

	.height-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.25rem;
	}

	.height-btn {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid transparent;
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.625rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.height-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #94a3b8;
	}

	.height-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.3);
		color: #818cf8;
	}

	.spacer {
		display: flex;
		align-items: center;
		justify-content: center;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 5px,
			rgba(99, 102, 241, 0.05) 5px,
			rgba(99, 102, 241, 0.05) 10px
		);
		border: 1px dashed rgba(99, 102, 241, 0.2);
		border-radius: 0.25rem;
	}

	.spacer-label {
		font-size: 0.6875rem;
		color: #64748b;
	}
</style>
