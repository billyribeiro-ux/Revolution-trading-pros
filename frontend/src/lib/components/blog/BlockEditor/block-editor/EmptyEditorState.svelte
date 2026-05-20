<!--
	EmptyEditorState — empty state shown when the editor has no blocks.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { fade } from 'svelte/transition';
	import { IconPlus } from '$lib/icons';

	interface Props {
		onAddBlock: () => void;
	}

	let { onAddBlock }: Props = $props();
</script>

<div class="empty-state" transition:fade>
	<div class="empty-icon">
		<IconPlus size={48} />
	</div>
	<h2>Start creating your content</h2>
	<p>Click the button below to add your first block</p>
	<button type="button" class="btn-add-first" onclick={onAddBlock}>
		<IconPlus size={18} />
		Add Block
	</button>
</div>

<style>
	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		text-align: center;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f0f0f0;
		border-radius: 16px;
		color: #999;
		margin-bottom: 1.5rem;
	}

	.empty-state h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 0.5rem;
	}

	.empty-state p {
		color: #666;
		margin: 0 0 1.5rem;
	}

	.btn-add-first {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-add-first:hover {
		background: #2563eb;
	}

	.btn-add-first:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}
</style>
