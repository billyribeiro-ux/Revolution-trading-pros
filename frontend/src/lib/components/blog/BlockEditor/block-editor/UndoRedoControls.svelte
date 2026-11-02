<!--
	UndoRedoControls — undo/redo toolbar buttons.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconArrowBackUp, IconArrowForwardUp } from '#lib/icons/index.js';

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
		color: var(--text-secondary, #8b949e);
		transition: all 0.15s;
	}

	.toolbar-btn:hover:not(:disabled) {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.toolbar-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.toolbar-btn:focus-visible {
		outline: 2px solid var(--primary-500, #e6b800);
		outline-offset: 2px;
	}
</style>
