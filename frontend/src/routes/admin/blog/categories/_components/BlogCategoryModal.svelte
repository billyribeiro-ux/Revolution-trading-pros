<script lang="ts">
	/**
	 * R27-E extraction (2026-05-20): create/edit modal for a blog
	 * Category. Owns:
	 *   - the modal overlay (click-outside + Escape to close)
	 *   - the form rendering (name / slug / description / color /
	 *     visibility)
	 *   - the per-input slug-auto-generation guard
	 *     (FIX-2026-04-26 P0-4: once the user manually edits the
	 *     slug, stop clobbering it on every name keystroke)
	 *
	 * Parent owns: `formData` ($bindable), `errors`, the `saving`
	 * flag, and the actual API call via `onSave`. `isEditing`
	 * drives the title ("Add" vs "Edit") and is also the signal
	 * the slug-auto-generator uses to never auto-fill the slug
	 * when editing an existing entity.
	 */
	import { IconAlertCircle } from '$lib/icons';
	import type { CategoryFormData } from './types';

	interface Props {
		isOpen: boolean;
		isEditing: boolean;
		formData: CategoryFormData;
		errors: string[];
		saving: boolean;
		onClose: () => void;
		onSave: () => void;
		onSlugManuallyEdited: () => void;
	}

	let {
		isOpen,
		isEditing,
		formData = $bindable(),
		errors,
		saving,
		onClose,
		onSave,
		onSlugManuallyEdited
	}: Props = $props();
</script>

{#if isOpen}
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
				<h3>{isEditing ? 'Edit' : 'Add'} Category</h3>
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
					<label for="cat-name">Name *</label>
					<input
						id="cat-name"
						name="cat-name"
						type="text"
						bind:value={formData.name}
						placeholder="Category name"
						required
					/>
				</div>

				<div class="form-group">
					<label for="cat-slug">Slug *</label>
					<input
						id="cat-slug"
						name="cat-slug"
						type="text"
						bind:value={formData.slug}
						oninput={onSlugManuallyEdited}
						placeholder="category-slug"
						required
					/>
					<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
				</div>

				<div class="form-group">
					<label for="cat-desc">Description</label>
					<textarea
						id="cat-desc"
						bind:value={formData.description}
						placeholder="Brief description"
						rows="3"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="cat-color">Color</label>
						<input id="cat-color" name="cat-color" type="color" bind:value={formData.color} />
					</div>

					<div class="form-group">
						<label class="checkbox-label">
							<input
								id="cat-is-visible"
								name="cat-is-visible"
								type="checkbox"
								bind:checked={formData.is_visible}
							/>
							<span>Visible</span>
						</label>
						<p class="help-text">Hidden categories won't appear in public listings</p>
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

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		background: rgba(15, 23, 42, 0.6);
		color: #f1f5f9;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500);
	}

	.form-group input[type='color'] {
		height: 50px;
		cursor: pointer;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.help-text {
		font-size: 0.8rem;
		color: #64748b;
		margin-top: 0.25rem;
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

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	.btn-primary:disabled,
	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	@media (max-width: 1024px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
