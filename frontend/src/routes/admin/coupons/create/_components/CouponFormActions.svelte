<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): sticky bottom save/cancel bar. Pure
	 * presentational — the submit button stays inside the parent's `<form>`
	 * via slot positioning (it's a sibling type=submit, parent owns the form's
	 * `onsubmit`).
	 */
	import { IconCheck, IconRefresh } from '#lib/icons/index.js';

	interface Props {
		saving: boolean;
		onCancel: () => void;
	}

	let { saving, onCancel }: Props = $props();
</script>

<div class="form-actions">
	<button type="button" class="btn-cancel" onclick={onCancel}> Cancel </button>
	<button type="submit" class="btn-save" disabled={saving}>
		{#if saving}
			<IconRefresh size={20} class="spinning" />
			Saving...
		{:else}
			<IconCheck size={20} />
			Save Coupon
		{/if}
	</button>
</div>

<style>
	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.5rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 12px;
		position: sticky;
		bottom: 1rem;
	}

	.btn-cancel {
		padding: 0.875rem 1.5rem;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-cancel:hover {
		background: var(--bg-hover, #252b33);
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 2rem;
		background: var(--primary-500);
		border: none;
		border-radius: 8px;
		color: var(--bg-base);
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-save:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(230, 184, 0, 0.3);
	}

	.btn-save:disabled {
		opacity: 0.7;
		cursor: not-allowed;
		transform: none;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 639.98px) {
		.form-actions {
			flex-direction: column;
		}

		.btn-save,
		.btn-cancel {
			width: 100%;
			justify-content: center;
		}
	}
</style>
