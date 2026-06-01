<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): Add/Edit tag modal. Same shape as
	 * `CategoryFormModal.svelte` minus the `description` field — kept
	 * separate so labels (id="tag-…") and form types stay explicit.
	 */
	import { IconAlertCircle } from '$lib/icons';
	import type { TagFormData } from './types';

	interface Props {
		open: boolean;
		isEdit: boolean;
		form: TagFormData;
		errors: string[];
		saving: boolean;
		onSlugInput: () => void;
		onSave: () => void;
		onClose: () => void;
	}

	let {
		open,
		isEdit,
		form = $bindable(),
		errors,
		saving,
		onSlugInput,
		onSave,
		onClose
	}: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h3>{isEdit ? 'Edit' : 'Add'} Tag</h3>
				<button class="close-btn" onclick={onClose} aria-label="Close dialog">×</button>
			</div>

			<div class="modal-body">
				{#if errors.length > 0}
					<div class="error-banner">
						<IconAlertCircle size={18} />
						<div>
							{#each errors as error (error)}
								<p>{error}</p>
							{/each}
						</div>
					</div>
				{/if}

				<div class="form-group">
					<label for="tag-name">Name *</label>
					<input
						id="tag-name"
						name="tag-name"
						type="text"
						bind:value={form.name}
						placeholder="Tag name"
						required
					/>
				</div>

				<div class="form-group">
					<label for="tag-slug">Slug *</label>
					<!-- FIX-2026-04-26 (P0-4): mark as user-edited on input. -->
					<input
						id="tag-slug"
						name="tag-slug"
						type="text"
						bind:value={form.slug}
						oninput={onSlugInput}
						placeholder="tag-slug"
						required
					/>
					<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="tag-color">Color</label>
						<input id="tag-color" name="tag-color" type="color" bind:value={form.color} />
					</div>

					<div class="form-group">
						<label class="checkbox-label">
							<input
								id="page-tagform-is-visible"
								name="page-tagform-is-visible"
								type="checkbox"
								bind:checked={form.is_visible}
							/>
							<span>Visible</span>
						</label>
						<p class="help-text">Hidden tags won't appear in public listings</p>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose} disabled={saving}>Cancel</button>
				<button class="btn-primary" onclick={onSave} disabled={saving}>
					{#if saving}
						Saving...
					{:else}
						Save
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		width: 100%;
		max-width: 500px;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
		color: #f1f5f9;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 2rem;
		color: #94a3b8;
		cursor: pointer;
		line-height: 1;
	}

	.close-btn:hover {
		color: #f1f5f9;
	}

	.modal-body {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-weight: 500;
		color: #cbd5e1;
		font-size: 0.95rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		background: rgba(15, 23, 42, 0.6);
		color: #f1f5f9;
	}

	.form-group input:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group input[type='color'] {
		height: 50px;
		cursor: pointer;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		user-select: none;
		color: #cbd5e1;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.help-text {
		font-size: 0.8rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	.error-banner {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		margin-bottom: 1rem;
	}

	.error-banner p {
		margin: 0;
		font-size: 0.9rem;
	}

	@media (max-width: 1023.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
