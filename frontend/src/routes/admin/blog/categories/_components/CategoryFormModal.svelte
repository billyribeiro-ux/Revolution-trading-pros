<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): Add/Edit category modal. Parent
	 * `$bindable`s the form data; this child handles the markup, slug
	 * `oninput` notification, and click/keyboard dismissal. Errors and
	 * saving state are read-only props from the parent (the parent is
	 * the source of truth for the API call result).
	 */
	import { IconAlertCircle } from '#lib/icons/index.js';
	import type { CategoryFormData } from './types';

	interface Props {
		open: boolean;
		isEdit: boolean;
		form: CategoryFormData;
		errors: string[];
		saving: boolean;
		onNameInput: (name: string) => void;
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
		onNameInput,
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
				<h3>{isEdit ? 'Edit' : 'Add'} Category</h3>
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
					<input autocomplete="off"
						id="cat-name"
						name="cat-name"
						type="text"
						bind:value={form.name}
						oninput={(event) => onNameInput(event.currentTarget.value)}
						placeholder="Category name"
						required
					/>
				</div>

				<div class="form-group">
					<label for="cat-slug">Slug *</label>
					<!-- FIX-2026-04-26 (P0-4): mark as user-edited on input so the
					     auto-generator stops clobbering manual slug edits. -->
					<input autocomplete="off"
						id="cat-slug"
						name="cat-slug"
						type="text"
						bind:value={form.slug}
						oninput={onSlugInput}
						placeholder="category-slug"
						required
					/>
					<p class="help-text">Lowercase letters, numbers, and hyphens only</p>
				</div>

				<div class="form-group">
					<label for="cat-desc">Description</label>
					<textarea
						id="cat-desc"
						bind:value={form.description}
						placeholder="Brief description"
						rows="3"></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="cat-color">Color</label>
						<input id="cat-color" name="cat-color" type="color" bind:value={form.color} />
					</div>

					<div class="form-group">
						<label class="checkbox-label">
							<input
								id="page-categoryform-is-visible"
								name="page-categoryform-is-visible"
								type="checkbox"
								bind:checked={form.is_visible}
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
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
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 2rem;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		line-height: 1;
	}

	.close-btn:hover {
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-secondary, #8b949e);
		font-size: 0.95rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		background: var(--bg-base, #0d1117);
		color: var(--text-primary, #f0f6fc);
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

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		user-select: none;
		color: var(--text-secondary, #8b949e);
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}


	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.help-text {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
		margin-top: 0.25rem;
	}

	.error-banner {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 6px;
		color: var(--error-emphasis, #f85149);
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
