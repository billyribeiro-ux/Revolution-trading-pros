<script lang="ts">
	import { IconX, IconDeviceFloppy } from '$lib/icons';

	interface Props {
		redirect?: any;
		onsaved?: () => void;
		oncancel?: () => void;
	}

	let { redirect = null, onsaved, oncancel }: Props = $props();

	// Form state initialized with defaults
	let form = $state({
		source_url: '',
		destination_url: '',
		redirect_type: '301',
		is_regex: false,
		is_active: true,
		notes: ''
	});

	// Sync redirect prop to form state
	$effect(() => {
		form.source_url = redirect?.source_url || '';
		form.destination_url = redirect?.destination_url || '';
		form.redirect_type = redirect?.redirect_type || '301';
		form.is_regex = redirect?.is_regex || false;
		form.is_active = redirect?.is_active !== undefined ? redirect.is_active : true;
		form.notes = redirect?.notes || '';
	});

	let saving = $state(false);
	let error = $state('');

	const redirectTypes = [
		{
			value: '301',
			label: '301 - Moved Permanently',
			description: 'Best for permanently moved content'
		},
		{ value: '302', label: '302 - Found (Temporary)', description: 'For temporary redirects' },
		{
			value: '307',
			label: '307 - Temporary Redirect',
			description: 'Temporary redirect, maintains request method'
		},
		{
			value: '308',
			label: '308 - Permanent Redirect',
			description: 'Permanent redirect, maintains request method'
		},
		{ value: '410', label: '410 - Gone', description: 'Content permanently removed' }
	];

	async function save() {
		error = '';
		saving = true;

		try {
			const url = redirect ? `/api/seo/redirects/${redirect.id}` : '/api/seo/redirects';

			const method = redirect ? 'PUT' : 'POST';

			const response = await fetch(url, {
				method,
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(form)
			});

			if (response.ok) {
				onsaved?.();
			} else {
				const data = await response.json();
				error = data.message || 'Failed to save redirect';
			}
		} catch (err) {
			error = 'An error occurred while saving';
		} finally {
			saving = false;
		}
	}

	function cancel() {
		oncancel?.();
	}
</script>

<div
	class="modal-overlay"
	role="button"
	tabindex="0"
	onclick={cancel}
	onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && cancel()}
>
	<div
		class="modal"
		role="dialog"
		tabindex="-1"
		onclick={(e: MouseEvent) => e.stopPropagation()}
		onkeypress={(e: KeyboardEvent) => e.stopPropagation()}
	>
		<div class="modal-header">
			<h2>{redirect ? 'Edit Redirect' : 'Create Redirect'}</h2>
			<button class="close-btn" onclick={cancel}>
				<IconX size={24} />
			</button>
		</div>

		<form
			onsubmit={(e: SubmitEvent) => {
				e.preventDefault();
				save();
			}}
		>
			<div class="modal-body">
				{#if error}
					<div class="error-message">{error}</div>
				{/if}

				<div class="form-group">
					<label for="source-url">Source URL <span class="required">*</span></label>
					<input
						id="source-url"
						type="text"
						bind:value={form.source_url}
						placeholder="/old-page"
						required
					/>
					<div class="hint">
						The URL to redirect from. {form.is_regex
							? 'Use regex pattern.'
							: 'Use absolute path (e.g., /old-page).'}
					</div>
				</div>

				<div class="form-group">
					<label for="is-regex">
						<input id="is-regex" type="checkbox" bind:checked={form.is_regex} />
						Use Regular Expression
					</label>
					<div class="hint">Enable regex for pattern matching (e.g., /product/(.*) → /shop/$1)</div>
				</div>

				<div class="form-group">
					<label for="destination-url">Destination URL <span class="required">*</span></label>
					<input
						id="destination-url"
						type="text"
						bind:value={form.destination_url}
						placeholder="/new-page"
						required={form.redirect_type !== '410'}
					/>
					<div class="hint">The URL to redirect to. Can be absolute or relative.</div>
				</div>

				<div class="form-group">
					<label for="redirect-type">Redirect Type <span class="required">*</span></label>
					<select id="redirect-type" bind:value={form.redirect_type} required>
						{#each redirectTypes as type}
							<option value={type.value}>
								{type.label}
							</option>
						{/each}
					</select>
					<div class="hint">
						{redirectTypes.find((t) => t.value === form.redirect_type)?.description}
					</div>
				</div>

				<div class="form-group">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						bind:value={form.notes}
						placeholder="Add notes about this redirect..."
						rows="3"
					></textarea>
				</div>

				<div class="form-group">
					<label for="is-active">
						<input id="is-active" type="checkbox" bind:checked={form.is_active} />
						Active
					</label>
					<div class="hint">Only active redirects will be processed</div>
				</div>
			</div>

			<div class="modal-footer">
				<button type="button" class="btn-secondary" onclick={cancel}> Cancel </button>
				<button type="submit" class="btn-primary" disabled={saving}>
					<IconDeviceFloppy size={18} />
					{saving ? 'Saving...' : 'Save Redirect'}
				</button>
			</div>
		</form>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #666;
		cursor: pointer;
		padding: 0.25rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: background 0.2s;
	}

	.close-btn:hover {
		background: #f0f0f0;
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.error-message {
		padding: 0.75rem 1rem;
		background: #fee2e2;
		color: #dc2626;
		border-radius: 6px;
		margin-bottom: 1rem;
		font-size: 0.9rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.required {
		color: #ef4444;
	}

	.form-group input[type='text'],
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		transition: border-color 0.2s;
	}

	.form-group input[type='text']:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-group input[type='checkbox'] {
		margin-right: 0.5rem;
		cursor: pointer;
	}

	.form-group label:has(input[type='checkbox']) {
		display: flex;
		align-items: center;
		cursor: pointer;
		font-weight: 400;
	}

	.hint {
		margin-top: 0.375rem;
		font-size: 0.85rem;
		color: #999;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid #e5e5e5;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.95rem;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f8f9fa;
	}
</style>
