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
		} catch (_err) {
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
	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 RESPONSIVE CREATE REDIRECT MODAL - Mobile-First Design
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: flex-end; /* Mobile: bottom sheet */
		justify-content: center;
		z-index: 1000;
		padding: 0;
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
	}

	.modal {
		position: fixed;
		inset: 0;
		background: white;
		border-radius: 0;
		width: 100%;
		max-height: 100dvh;
		display: flex;
		flex-direction: column;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
		overflow: hidden;
		/* Safe area insets */
		padding-top: env(safe-area-inset-top, 0);
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	/* Swipe indicator for mobile */
	.modal::before {
		content: '';
		position: absolute;
		top: 8px;
		left: 50%;
		transform: translateX(-50%);
		width: 36px;
		height: 4px;
		background: rgba(0, 0, 0, 0.15);
		border-radius: 2px;
		z-index: 11;
	}

	.modal-header {
		position: sticky;
		top: 0;
		z-index: 10;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		padding-top: calc(1.25rem + env(safe-area-inset-top, 0));
		border-bottom: 1px solid #e5e5e5;
		background: rgba(255, 255, 255, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.modal-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	/* Touch target: 44x44px minimum */
	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 44px;
		min-height: 44px;
		width: 44px;
		height: 44px;
		background: none;
		border: none;
		color: #666;
		cursor: pointer;
		padding: 0;
		border-radius: 0.75rem;
		transition: background 0.2s;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.close-btn:hover {
		background: #f0f0f0;
	}

	.close-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.modal-body {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
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

	/* Sticky footer */
	.modal-footer {
		position: sticky;
		bottom: 0;
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
		padding-bottom: calc(1rem + env(safe-area-inset-bottom, 0));
		border-top: 1px solid #e5e5e5;
		background: #f9fafb;
		flex-shrink: 0;
	}

	/* Touch target: 44px minimum */
	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		min-height: 44px;
		padding: 0.75rem 1.5rem;
		border-radius: 0.75rem;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.95rem;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
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

	.btn-secondary:focus-visible {
		outline: 2px solid #6b7280;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* sm: 640px+ - Centered modal */
	@media (min-width: 640px) {
		.modal-overlay {
			align-items: center;
			padding: 1.5rem;
		}

		.modal {
			position: relative;
			inset: auto;
			max-width: 500px;
			max-height: 85vh;
			border-radius: 0.75rem;
			padding-top: 0;
			padding-bottom: 0;
		}

		.modal::before {
			display: none;
		}

		.modal-header {
			padding: 1.25rem 1.5rem;
			padding-top: 1.25rem;
			border-radius: 0.75rem 0.75rem 0 0;
		}

		.modal-header h2 {
			font-size: 1.25rem;
		}

		.modal-body {
			padding: 1.5rem;
		}

		.modal-footer {
			flex-direction: row;
			justify-content: flex-end;
			padding: 1rem 1.5rem;
			padding-bottom: 1rem;
			border-radius: 0 0 0.75rem 0.75rem;
		}
	}

	/* Landscape orientation */
	@media (max-height: 500px) and (orientation: landscape) {
		.modal {
			max-height: 100dvh;
		}

		.modal-header {
			padding: 0.75rem 1rem;
		}

		.modal-body {
			padding: 0.75rem 1rem;
		}

		.modal-footer {
			padding: 0.75rem 1rem;
		}
	}

	/* Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.close-btn,
		.btn-primary,
		.btn-secondary {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.modal {
			border: 2px solid #000;
		}

		.btn-primary,
		.btn-secondary {
			border: 2px solid currentColor;
		}
	}
</style>
