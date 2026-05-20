<!--
	MultiSelectionBadge — floating badge shown when >1 block is selected.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { fade } from 'svelte/transition';

	interface Props {
		count: number;
		onClear: () => void;
	}

	let { count, onClear }: Props = $props();
</script>

<div class="multi-selection-badge" transition:fade={{ duration: 150 }}>
	{count} blocks selected
	<button type="button" class="clear-selection-btn" onclick={onClear} aria-label="Clear selection">
		Clear
	</button>
</div>

<style>
	.multi-selection-badge {
		position: fixed;
		bottom: 24px;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%);
		color: white;
		border-radius: 12px;
		font-size: 14px;
		font-weight: 500;
		box-shadow:
			0 10px 40px rgba(30, 27, 75, 0.4),
			0 4px 12px rgba(0, 0, 0, 0.2);
		z-index: 1000;
		backdrop-filter: blur(8px);
	}

	.clear-selection-btn {
		padding: 6px 12px;
		background: rgba(255, 255, 255, 0.15);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		color: white;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.clear-selection-btn:hover {
		background: rgba(255, 255, 255, 0.25);
		border-color: rgba(255, 255, 255, 0.3);
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.multi-selection-badge {
			background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
		}
	}
</style>
