<script lang="ts">
	import { IconX, IconDeviceFloppy } from '$lib/icons';

	interface Props {
		url404?: string;
		oncreated?: () => void;
		oncancel?: () => void;
	}

	let { url404 = '', oncreated, oncancel }: Props = $props();

	// Form state initialized with defaults
	let form = $state({
		destination_url: '',
		redirect_type: '301',
		notes: ''
	});

	// Sync url404 prop to form.notes
	$effect(() => {
		form.notes = `Redirect created from 404: ${url404 || ''}`;
	});

	let saving = $state(false);
	let error = $state('');

	async function save() {
		error = '';
		saving = true;

		try {
			// First, create the redirect
			const response = await fetch('/api/seo/redirects', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					source_url: url404,
					destination_url: form.destination_url,
					redirect_type: form.redirect_type,
					is_active: true,
					is_regex: false,
					notes: form.notes
				})
			});

			if (response.ok) {
				oncreated?.();
			} else {
				const data = await response.json();
				error = data.message || 'Failed to create redirect';
			}
		} catch (err) {
			error = 'An error occurred while creating redirect';
		} finally {
			saving = false;
		}
	}

	function cancel() {
		oncancel?.();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			cancel();
		}
	}
</script>

<div class="modal-overlay" onclick={cancel} onkeydown={handleKeydown} role="presentation">
	<div
		class="modal"
		onclick={(e: MouseEvent) => e.stopPropagation()}
		onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div class="modal-header">
			<h2>Create Redirect from 404</h2>
			<button class="close-btn" onclick={cancel} aria-label="Close modal">
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
					<div class="field-label">404 URL</div>
					<div class="readonly-field">{url404}</div>
				</div>

				<div class="form-group">
					<label for="destination-url">Redirect To <span class="required">*</span></label>
					<input
						id="destination-url"
						type="text"
						bind:value={form.destination_url}
						placeholder="/new-page"
						required
					/>
					<div class="hint">Enter the destination URL (e.g., /new-page or https://example.com)</div>
				</div>

				<div class="form-group">
					<label for="redirect-type">Redirect Type</label>
					<select id="redirect-type" bind:value={form.redirect_type}>
						<option value="301">301 - Moved Permanently</option>
						<option value="302">302 - Found (Temporary)</option>
						<option value="307">307 - Temporary Redirect</option>
						<option value="308">308 - Permanent Redirect</option>
					</select>
				</div>

				<div class="form-group">
					<label for="notes">Notes</label>
					<textarea id="notes" bind:value={form.notes} rows="2"></textarea>
				</div>
			</div>

			<div class="modal-footer">
				<button type="button" class="btn-secondary" onclick={cancel}> Cancel </button>
				<button type="submit" class="btn-primary" disabled={saving}>
					<IconDeviceFloppy size={18} />
					{saving ? 'Creating...' : 'Create Redirect'}
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
		max-width: 500px;
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

	.form-group label,
	.form-group .field-label {
		display: block;
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.required {
		color: #ef4444;
	}

	.readonly-field {
		padding: 0.75rem;
		background: #f8f9fa;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-family: monospace;
		font-size: 0.9rem;
		color: #dc2626;
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
