<!--
	Add Block Button - Triggers slash command menu
	═══════════════════════════════════════════════════════════════════════════════

	A floating button that appears at the bottom of the block list
	to add new blocks via the slash command menu.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import IconPlus from '@tabler/icons-svelte/icons/plus';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		onclick?: () => void;
	}

	let { onclick }: Props = $props();
</script>

<div class="add-block-container">
	<button
		type="button"
		class="add-block-btn"
		onclick={onclick}
		title="Add block (type / for commands)"
		aria-label="Add new block"
	>
		<span class="add-icon">
			<IconPlus size={18} />
		</span>
		<span class="add-text">Add block</span>
		<span class="add-hint">or type <kbd>/</kbd></span>
	</button>
</div>

<style>
	.add-block-container {
		display: flex;
		justify-content: center;
		padding: 1.5rem 0 0.5rem;
	}

	.add-block-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: transparent;
		border: 2px dashed rgba(99, 102, 241, 0.25);
		border-radius: 0.5rem;
		color: #64748b;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-block-btn:hover {
		border-color: rgba(99, 102, 241, 0.5);
		background: rgba(99, 102, 241, 0.05);
		color: #94a3b8;
	}

	.add-block-btn:hover .add-icon {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.add-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 0.375rem;
		color: #6366f1;
		transition: all 0.2s;
	}

	.add-text {
		font-weight: 500;
	}

	.add-hint {
		font-size: 0.75rem;
		color: #475569;
	}

	.add-hint kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 18px;
		height: 18px;
		padding: 0 0.25rem;
		margin-left: 0.25rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		font-family: inherit;
		font-size: 0.6875rem;
		color: #818cf8;
	}
</style>
