<!--
	TradePlanModal — Add/Edit trade plan entry modal.
	Extracted from +page.svelte (R8-C) — 1 $bindable form prop, 2 callback props.
-->
<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { Bias, TradePlanEntry } from '$lib/api/room-content';

	export interface TradePlanFormData {
		ticker: string;
		bias: Bias;
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		options_strike: string;
		options_exp: string;
		notes: string;
	}

	interface Props {
		open: boolean;
		editingTradePlan: TradePlanEntry | null;
		form: TradePlanFormData;
		isSaving: boolean;
		isValid: boolean;
		onSave: () => void;
		onClose: () => void;
	}

	let {
		open,
		editingTradePlan,
		form = $bindable(),
		isSaving,
		isValid,
		onSave,
		onClose
	}: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		aria-hidden="true"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="trade-plan-modal-title"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2 id="trade-plan-modal-title">{editingTradePlan ? 'Edit' : 'Add'} Trade Plan Entry</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					onSave();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="ticker">Ticker *</label>
						<input
							id="ticker"
							name="ticker"
							type="text"
							bind:value={form.ticker}
							placeholder="NVDA"
							style="text-transform: uppercase"
						/>
					</div>
					<div class="form-group">
						<label for="bias">Bias *</label>
						<select id="bias" bind:value={form.bias}>
							<option value="BULLISH">BULLISH</option>
							<option value="BEARISH">BEARISH</option>
							<option value="NEUTRAL">NEUTRAL</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="entry">Entry</label>
						<input
							id="entry"
							name="entry"
							type="text"
							bind:value={form.entry}
							placeholder="$142.50"
						/>
					</div>
					<div class="form-group">
						<label for="stop">Stop</label>
						<input id="stop" name="stop" type="text" bind:value={form.stop} placeholder="$136.00" />
					</div>
				</div>

				<div class="form-row targets">
					<div class="form-group">
						<label for="target1">Target 1</label>
						<input
							id="target1"
							name="target1"
							type="text"
							bind:value={form.target1}
							placeholder="$148.00"
						/>
					</div>
					<div class="form-group">
						<label for="target2">Target 2</label>
						<input
							id="target2"
							name="target2"
							type="text"
							bind:value={form.target2}
							placeholder="$152.00"
						/>
					</div>
					<div class="form-group">
						<label for="target3">Target 3</label>
						<input
							id="target3"
							name="target3"
							type="text"
							bind:value={form.target3}
							placeholder="$158.00"
						/>
					</div>
					<div class="form-group">
						<label for="runner">Runner</label>
						<input
							id="runner"
							name="runner"
							type="text"
							bind:value={form.runner}
							placeholder="$165.00+"
						/>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="options_strike">Options Strike</label>
						<input
							id="options_strike"
							name="options_strike"
							type="text"
							bind:value={form.options_strike}
							placeholder="$145 Call"
						/>
					</div>
					<div class="form-group">
						<label for="options_exp">Options Exp</label>
						<input id="options_exp" name="options_exp" type="date" bind:value={form.options_exp} />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						bind:value={form.notes}
						placeholder="Breakout above consolidation. Wait for pullback to entry..."
						rows="3"
					></textarea>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={onClose}>Cancel</button>
					<button type="submit" class="btn-primary" disabled={isSaving || !isValid}>
						{isSaving ? 'Saving...' : editingTradePlan ? 'Update' : 'Add Entry'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* Modal */
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
		z-index: var(--z-modal, 400);
		padding: var(--space-2, 20px);
		isolation: isolate;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: #1e293b;
	}

	.modal-body {
		padding: 24px;
	}

	.form-row {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-row.targets {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
	}

	.form-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		margin-bottom: 20px;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: #475569;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 12px 14px;
		border: 1.5px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}

	.form-group textarea {
		resize: vertical;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f1f5f9;
		color: #475569;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	@media (max-width: 767.98px) {
		.form-row {
			flex-direction: column;
		}

		.form-row.targets {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
