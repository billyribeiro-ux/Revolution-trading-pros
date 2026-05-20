<!--
/**
 * DragOverlay - Full-modal drop indicator shown while dragging files
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 * Parent gates rendering on `isDragging` via {#if} so this component
 * unconditionally renders + relies on Svelte transitions externally.
 */
-->
<script lang="ts">
	import { fade } from 'svelte/transition';
	import Icon from '$lib/components/Icon.svelte';
</script>

<div class="drag-overlay" transition:fade={{ duration: 150 }}>
	<div class="drag-indicator">
		<Icon name="IconUpload" size={48} />
		<span>Drop files to upload</span>
	</div>
</div>

<style>
	.drag-overlay {
		position: absolute;
		inset: 0;
		background: rgba(15, 23, 42, 0.95);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
	}

	.drag-indicator {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		color: #3b82f6;
	}

	.drag-indicator span {
		font-size: 1.25rem;
		font-weight: 500;
	}
</style>
