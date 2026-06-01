<!--
	CloseTradeModal — confirm + capture exit details for an open trade.
	Extracted from +page.svelte (R8-C) — 1 $bindable form prop, 2 callback props.
-->
<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { RoomTrade } from '$lib/api/room-content';

	export interface CloseTradeFormData {
		exit_price: string;
		exit_date: string;
		notes: string;
	}

	interface Props {
		open: boolean;
		closingTrade: RoomTrade | null;
		form: CloseTradeFormData;
		isClosing: boolean;
		isValid: boolean;
		formatCurrency: (value: number | null) => string;
		onClose: () => void;
		onSubmit: () => void;
	}

	let {
		open,
		closingTrade,
		form = $bindable(),
		isClosing,
		isValid,
		formatCurrency,
		onClose,
		onSubmit
	}: Props = $props();
</script>

{#if open && closingTrade}
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
			aria-labelledby="close-trade-modal-title"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2 id="close-trade-modal-title">Close Trade: {closingTrade.ticker}</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					onSubmit();
				}}
			>
				<div class="trade-summary">
					<div class="trade-summary-row">
						<span class="label">Ticker:</span>
						<span class="value"><strong>{closingTrade.ticker}</strong></span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Direction:</span>
						<span
							class="value direction-badge"
							class:long={closingTrade.direction === 'long'}
							class:short={closingTrade.direction === 'short'}
						>
							{closingTrade.direction.toUpperCase()}
						</span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Entry Price:</span>
						<span class="value">{formatCurrency(closingTrade.entry_price)}</span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Entry Date:</span>
						<span class="value">{new Date(closingTrade.entry_date).toLocaleDateString()}</span>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="exit_price">Exit Price *</label>
						<input
							id="exit_price"
							name="exit_price"
							type="number"
							step="0.01"
							bind:value={form.exit_price}
							placeholder="145.50"
							required
						/>
					</div>
					<div class="form-group">
						<label for="exit_date">Exit Date</label>
						<input id="exit_date" name="exit_date" type="date" bind:value={form.exit_date} />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="close_notes">Notes</label>
					<textarea
						id="close_notes"
						bind:value={form.notes}
						placeholder="Optional notes about the exit..."
						rows="2"
					></textarea>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={onClose}>Cancel</button>
					<button type="submit" class="btn-primary" disabled={isClosing || !isValid}>
						{isClosing ? 'Closing...' : 'Close Trade'}
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
	.form-group textarea {
		padding: 12px 14px;
		border: 1.5px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
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

	/* Trade Summary in Modal */
	.trade-summary {
		background: #f8fafc;
		border-radius: 10px;
		padding: 16px;
		margin-bottom: 20px;
	}

	.trade-summary-row {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #e2e8f0;
	}

	.trade-summary-row:last-child {
		border-bottom: none;
	}

	.trade-summary-row .label {
		color: #64748b;
		font-size: 14px;
	}

	.trade-summary-row .value {
		color: #1e293b;
		font-size: 14px;
	}

	.direction-badge {
		display: inline-block;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.direction-badge.long {
		background: #dcfce7;
		color: #166534;
	}

	.direction-badge.short {
		background: #fef2f2;
		color: #991b1b;
	}

	@media (max-width: 767.98px) {
		.form-row {
			flex-direction: column;
		}
	}
</style>
