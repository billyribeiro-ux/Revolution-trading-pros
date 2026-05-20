<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): cancel + save buttons row. Discrete
	 * callbacks (R8-C style): `onCancel`, `onSave` — each names its action.
	 */
	import { IconCheck } from '$lib/icons';

	interface Props {
		saving: boolean;
		disabled: boolean;
		onCancel: () => void;
		onSave: () => void;
	}

	let { saving, disabled, onCancel, onSave }: Props = $props();
</script>

<div class="form-actions">
	<button type="button" class="btn-secondary" onclick={onCancel}>Cancel</button>
	<button type="button" class="btn-primary" onclick={onSave} {disabled}>
		{#if saving}
			<div class="btn-spinner"></div>
			Creating...
		{:else}
			<IconCheck size={18} />
			Create Product
		{/if}
	</button>
</div>

<style>
	/* Form Actions */
	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(148, 163, 184, 0.15);
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
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		color: #cbd5e1;
	}

	.btn-secondary:hover {
		background: rgba(30, 41, 59, 0.8);
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
