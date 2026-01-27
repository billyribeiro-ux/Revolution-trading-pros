<!--
	Divider Block - Horizontal rule separator
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

	let style = $derived((block.data as { style?: string })?.style ?? 'solid');

	function handleStyleChange(newStyle: string) {
		onUpdate?.({ style: newStyle });
	}
</script>

<div class="divider-block">
	{#if !readonly}
		<div class="style-selector">
			{#each ['solid', 'dashed', 'dotted', 'gradient'] as s}
				<button
					type="button"
					class="style-btn"
					class:active={style === s}
					onclick={() => handleStyleChange(s)}
				>
					{s}
				</button>
			{/each}
		</div>
	{/if}
	<hr class="divider divider-{style}" />
</div>

<style>
	.divider-block {
		padding: 0.5rem 0;
	}

	.style-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
	}

	.style-btn {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid transparent;
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.6875rem;
		text-transform: capitalize;
		cursor: pointer;
		transition: all 0.15s;
	}

	.style-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #94a3b8;
	}

	.style-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.3);
		color: #818cf8;
	}

	.divider {
		margin: 0;
		border: none;
		height: 2px;
	}

	.divider-solid {
		background: rgba(51, 65, 85, 0.5);
	}

	.divider-dashed {
		background: repeating-linear-gradient(
			90deg,
			rgba(51, 65, 85, 0.5) 0,
			rgba(51, 65, 85, 0.5) 8px,
			transparent 8px,
			transparent 16px
		);
	}

	.divider-dotted {
		background: repeating-linear-gradient(
			90deg,
			rgba(51, 65, 85, 0.5) 0,
			rgba(51, 65, 85, 0.5) 4px,
			transparent 4px,
			transparent 10px
		);
	}

	.divider-gradient {
		background: linear-gradient(90deg, transparent, rgba(99, 102, 241, 0.5), transparent);
	}
</style>
