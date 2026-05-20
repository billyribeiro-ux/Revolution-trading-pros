<!--
/**
 * DamFooter - AssetManager modal footer with Cancel / Insert buttons
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 */
-->
<script lang="ts">
	interface Props {
		selectedCount: number;
		onClose: () => void;
		onInsert: () => void;
	}

	const { selectedCount, onClose, onInsert }: Props = $props();
</script>

<footer class="dam-footer">
	<div class="footer-info">
		{#if selectedCount > 0}
			{selectedCount} asset{selectedCount !== 1 ? 's' : ''} selected
		{:else}
			Select an asset to insert
		{/if}
	</div>
	<div class="footer-actions">
		<button class="cancel-btn" onclick={onClose}>Cancel</button>
		<button class="insert-btn" onclick={onInsert} disabled={selectedCount === 0}>
			Insert Selected
		</button>
	</div>
</footer>

<style>
	.dam-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.02);
	}

	.footer-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	.footer-actions {
		display: flex;
		gap: 0.75rem;
	}

	.cancel-btn,
	.insert-btn {
		padding: 0.625rem 1.25rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn {
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #94a3b8;
	}

	.cancel-btn:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.insert-btn {
		background: #3b82f6;
		border: none;
		color: white;
	}

	.insert-btn:hover:not(:disabled) {
		background: #2563eb;
	}

	.insert-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
