<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): Cancel + Submit actions for the
	 * admin user-edit form. The submit button is `type="submit"` and
	 * relies on the parent's `<form onsubmit>` — Cancel is a plain
	 * button that the parent wires to `goto('/admin/users')`.
	 */
	import { IconCheck } from '$lib/icons';

	interface Props {
		saving: boolean;
		onCancel: () => void;
	}

	let { saving, onCancel }: Props = $props();
</script>

<div class="form-actions">
	<button type="button" class="btn-secondary" onclick={onCancel}>Cancel</button>
	<button type="submit" class="btn-primary" disabled={saving}>
		{#if saving}
			Saving...
		{:else}
			<IconCheck size={18} />
			Update User
		{/if}
	</button>
</div>

<style>
	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #cbd5e1;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	@media (max-width: 768px) {
		.form-actions {
			flex-direction: column-reverse;
		}

		.form-actions button {
			width: 100%;
		}
	}
</style>
