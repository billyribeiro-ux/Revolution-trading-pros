<script lang="ts">
	// Confirm-publish modal shown when overall quality score < 70.
	// Behaviour preserved 1:1 from courses/create +page.svelte (lines 1616-1627).
	interface Props {
		open: boolean;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { open, onConfirm, onCancel }: Props = $props();
</script>

{#if open}
	<div class="modal-overlay">
		<div class="modal-content">
			<h3>Publish Warning</h3>
			<p>Your course quality score is below 70%. Are you sure you want to publish?</p>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={onCancel}>Cancel</button>
				<button class="btn-primary" onclick={onConfirm}>Publish Anyway</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Mirrors the original +page.svelte rules so the modal looks identical. */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 16px;
		padding: 2rem;
		max-width: 500px;
		width: 90%;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-content h3 {
		color: var(--text-primary, #f0f6fc);
		font-size: 1.25rem;
		margin-bottom: 1rem;
	}

	.modal-content p {
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-secondary {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
