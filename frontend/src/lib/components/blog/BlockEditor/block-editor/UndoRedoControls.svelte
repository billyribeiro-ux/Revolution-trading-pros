<!--
	UndoRedoControls — undo/redo toolbar buttons.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconArrowBackUp, IconArrowForwardUp } from '$lib/icons';

	interface Props {
		canUndo: boolean;
		canRedo: boolean;
		onUndo: () => void;
		onRedo: () => void;
	}

	let { canUndo, canRedo, onUndo, onRedo }: Props = $props();
</script>

<div class="undo-redo" role="group" aria-label="Undo and redo actions">
	<button
		type="button"
		class="toolbar-btn"
		disabled={!canUndo}
		onclick={onUndo}
		title="Undo (Ctrl+Z)"
		aria-label="Undo last action"
		aria-keyshortcuts="Control+Z"
	>
		<IconArrowBackUp size={18} aria-hidden="true" />
	</button>
	<button
		type="button"
		class="toolbar-btn"
		disabled={!canRedo}
		onclick={onRedo}
		title="Redo (Ctrl+Shift+Z)"
		aria-label="Redo last undone action"
		aria-keyshortcuts="Control+Shift+Z"
	>
		<IconArrowForwardUp size={18} aria-hidden="true" />
	</button>
</div>

<style>
	.undo-redo {
		display: flex;
		gap: 0.25rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: #f0f0f0;
		color: #1a1a1a;
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.toolbar-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}
</style>
