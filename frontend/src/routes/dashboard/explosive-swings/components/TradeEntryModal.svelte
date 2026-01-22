<!--
	Trade Entry Modal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Inline modal for adding/editing trade plan entries directly from the dashboard.
	Visible only to admins and super admins.
	
	@version 1.0.0
-->
<script lang="ts">
	import { tradePlanApi, type Bias } from '$lib/api/room-content';

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, roomSlug, onClose, onSuccess }: Props = $props();

	let isSaving = $state(false);
	let errorMessage = $state('');

	let form = $state({
		ticker: '',
		bias: 'BULLISH' as Bias,
		entry: '',
		target1: '',
		target2: '',
		target3: '',
		runner: '',
		stop: '',
		options_strike: '',
		options_exp: '',
		notes: ''
	});

	const isFormValid = $derived(form.ticker.trim() !== '');

	function resetForm() {
		form = {
			ticker: '',
			bias: 'BULLISH',
			entry: '',
			target1: '',
			target2: '',
			target3: '',
			runner: '',
			stop: '',
			options_strike: '',
			options_exp: '',
			notes: ''
		};
		errorMessage = '';
	}

	async function handleSubmit() {
		if (!isFormValid) return;
		isSaving = true;
		errorMessage = '';

		try {
			await tradePlanApi.create({
				room_slug: roomSlug,
				ticker: form.ticker.toUpperCase(),
				bias: form.bias,
				entry: form.entry || undefined,
				target1: form.target1 || undefined,
				target2: form.target2 || undefined,
				target3: form.target3 || undefined,
				runner: form.runner || undefined,
				stop: form.stop || undefined,
				options_strike: form.options_strike || undefined,
				options_exp: form.options_exp || undefined,
				notes: form.notes || undefined
			});

			resetForm();
			onSuccess?.();
			onClose();
		} catch (err) {
			errorMessage = 'Failed to add trade entry. Please try again.';
			console.error(err);
		} finally {
			isSaving = false;
		}
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleClose();
	}
</script>

{#if isOpen}
	<div 
		class="modal-overlay" 
		role="dialog" 
		aria-modal="true" 
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={handleClose}
		onkeydown={handleKeydown}
	>
		<div class="modal-content">
			<div class="modal-header">
				<h3 id="modal-title">Add Trade Entry</h3>
				<button class="modal-close" onclick={handleClose} aria-label="Close modal">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			{#if errorMessage}
				<div class="error-banner">{errorMessage}</div>
			{/if}

			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<div class="form-row">
					<div class="form-group">
						<label for="ticker">Ticker *</label>
						<input
							id="ticker"
							type="text"
							bind:value={form.ticker}
							placeholder="NVDA"
							class="form-input ticker-input"
							required
						/>
					</div>
					<div class="form-group">
						<label for="bias">Bias *</label>
						<select id="bias" bind:value={form.bias} class="form-select">
							<option value="BULLISH">BULLISH</option>
							<option value="BEARISH">BEARISH</option>
							<option value="NEUTRAL">NEUTRAL</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="entry">Entry Price</label>
						<input id="entry" type="text" bind:value={form.entry} placeholder="$142.50" class="form-input" />
					</div>
					<div class="form-group">
						<label for="stop">Stop Loss</label>
						<input id="stop" type="text" bind:value={form.stop} placeholder="$136.00" class="form-input" />
					</div>
				</div>

				<div class="form-row targets">
					<div class="form-group">
						<label for="target1">T1</label>
						<input id="target1" type="text" bind:value={form.target1} placeholder="$148" class="form-input" />
					</div>
					<div class="form-group">
						<label for="target2">T2</label>
						<input id="target2" type="text" bind:value={form.target2} placeholder="$155" class="form-input" />
					</div>
					<div class="form-group">
						<label for="target3">T3</label>
						<input id="target3" type="text" bind:value={form.target3} placeholder="$162" class="form-input" />
					</div>
					<div class="form-group">
						<label for="runner">Runner</label>
						<input id="runner" type="text" bind:value={form.runner} placeholder="$170+" class="form-input" />
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="options_strike">Options Strike</label>
						<input id="options_strike" type="text" bind:value={form.options_strike} placeholder="$145C" class="form-input" />
					</div>
					<div class="form-group">
						<label for="options_exp">Expiration</label>
						<input id="options_exp" type="text" bind:value={form.options_exp} placeholder="Feb 21" class="form-input" />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						bind:value={form.notes}
						placeholder="Entry thesis, key levels, catalysts..."
						class="form-textarea"
						rows="3"
					></textarea>
				</div>

				<div class="form-actions">
					<button type="button" class="btn-cancel" onclick={handleClose}>Cancel</button>
					<button type="submit" class="btn-save" disabled={!isFormValid || isSaving}>
						{#if isSaving}
							<span class="spinner"></span>
							Adding...
						{:else}
							Add Entry
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 24px;
		backdrop-filter: blur(4px);
	}

	.modal-content {
		background: white;
		border-radius: 16px;
		width: 100%;
		max-width: 580px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
		color: #1e293b;
	}

	.modal-close {
		padding: 8px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.modal-close:hover {
		background: #f1f5f9;
		color: #1e293b;
	}

	.error-banner {
		margin: 16px 24px 0;
		padding: 12px 16px;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		font-size: 14px;
	}

	.modal-form {
		padding: 24px;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
		margin-bottom: 16px;
	}

	.form-row.targets {
		grid-template-columns: repeat(4, 1fr);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.form-input,
	.form-select,
	.form-textarea {
		padding: 10px 12px;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		transition: border-color 0.15s, box-shadow 0.15s;
	}

	.form-input:focus,
	.form-select:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #143E59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.ticker-input {
		text-transform: uppercase;
		font-weight: 600;
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	.btn-cancel {
		padding: 10px 20px;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: #f1f5f9;
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 24px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-save:hover:not(:disabled) {
		background: #0f2d42;
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255,255,255,0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	@media (max-width: 640px) {
		.form-row {
			grid-template-columns: 1fr;
		}

		.form-row.targets {
			grid-template-columns: repeat(2, 1fr);
		}

		.form-actions {
			flex-direction: column;
		}

		.btn-cancel,
		.btn-save {
			width: 100%;
			justify-content: center;
		}
	}
</style>
