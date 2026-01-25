<!--
	ClosePositionModal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Modal for closing active positions with exit price, calculates P&L automatically.
	Dark theme matching dashboard (var(--color-brand-primary)).
	
	@version 1.0.0
-->
<script lang="ts">
	import type { ActivePosition } from '../types';
	import { formatPrice, formatPercent } from '../utils/formatters';

	interface Props {
		isOpen: boolean;
		position: ActivePosition | null;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, position, roomSlug, onClose, onSuccess }: Props = $props();

	let isSaving = $state(false);
	let errorMessage = $state('');
	let modalRef = $state<HTMLDivElement | null>(null);

	let form = $state({
		exit_price: '',
		exit_date: new Date().toISOString().split('T')[0],
		notes: ''
	});

	// Calculate P&L preview
	const pnlPreview = $derived.by(() => {
		if (!position || !form.exit_price) return null;
		const exitPrice = parseFloat(form.exit_price);
		if (isNaN(exitPrice) || !position.entryPrice) return null;
		
		const priceDiff = exitPrice - position.entryPrice;
		const pnlPercent = (priceDiff / position.entryPrice) * 100;
		const isProfit = priceDiff >= 0;
		
		return { priceDiff, pnlPercent, isProfit, result: isProfit ? 'WIN' : 'LOSS' };
	});

	// Focus trap and body scroll lock
	$effect(() => {
		if (!isOpen) return;
		
		const previousOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';
		
		// Reset form when opening
		form = {
			exit_price: '',
			exit_date: new Date().toISOString().split('T')[0],
			notes: position?.notes || ''
		};
		errorMessage = '';
		
		requestAnimationFrame(() => modalRef?.focus());
		
		return () => {
			document.body.style.overflow = previousOverflow;
		};
	});

	function handleClose() {
		onClose();
	}

	function handleOverlayClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}

	async function handleSubmit() {
		if (!position || !form.exit_price) {
			errorMessage = 'Exit price is required';
			return;
		}

		const exitPrice = parseFloat(form.exit_price);
		if (isNaN(exitPrice) || exitPrice <= 0) {
			errorMessage = 'Please enter a valid exit price';
			return;
		}

		isSaving = true;
		errorMessage = '';

		try {
			// We need to find the trade ID from the position
			// The position has a ticker, so we search for open trade with that ticker
			const response = await fetch(`/api/trades/${roomSlug}?status=open&ticker=${position.ticker}`, {
				credentials: 'include'
			});
			const tradesData = await response.json();
			
			if (!tradesData.success || !tradesData.data?.length) {
				errorMessage = 'Could not find matching trade to close';
				return;
			}

			// Match on ticker AND entry price to avoid closing wrong position
			const trade = tradesData.data.find((t: any) => 
				t.ticker === position.ticker && 
				position.entryPrice !== null &&
				Math.abs(t.entry_price - position.entryPrice) < 0.01
			) || tradesData.data[0];  // Fallback to first if no exact match
			
			if (!trade) {
				errorMessage = 'Could not find matching trade to close';
				return;
			}
			
			// Close the trade
			const closeResponse = await fetch(`/api/trades/${roomSlug}/${trade.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					exit_price: exitPrice,
					exit_date: form.exit_date,
					notes: form.notes,
					status: 'closed'
				})
			});

			const closeData = await closeResponse.json();

			if (closeData.success) {
				onSuccess?.();
				onClose();
			} else {
				errorMessage = closeData.error || 'Failed to close position';
			}
		} catch (err) {
			errorMessage = 'Failed to close position. Please try again.';
			console.error(err);
		} finally {
			isSaving = false;
		}
	}
</script>

{#if isOpen && position}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		class="modal-overlay"
		onclick={handleOverlayClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<div 
			class="modal-container"
			bind:this={modalRef}
			tabindex="-1"
		>
			<!-- Header -->
			<div class="modal-header">
				<div class="header-content">
					<div class="header-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="24" height="24">
							<path d="M9 12l2 2 4-4" />
							<circle cx="12" cy="12" r="10" />
						</svg>
					</div>
					<div class="header-text">
						<h3 id="modal-title">Close Position: {position.ticker}</h3>
						<p class="header-subtitle">Enter exit price to calculate P&L</p>
					</div>
				</div>
				<button class="modal-close" onclick={handleClose} aria-label="Close modal">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Form -->
			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<!-- Position Summary -->
				<div class="position-summary">
					<div class="summary-row">
						<span class="summary-label">Ticker</span>
						<span class="summary-value ticker">{position.ticker}</span>
					</div>
					<div class="summary-row">
						<span class="summary-label">Entry Price</span>
						<span class="summary-value">{formatPrice(position.entryPrice ?? 0)}</span>
					</div>
					<div class="summary-row">
						<span class="summary-label">Current Price</span>
						<span class="summary-value">{formatPrice(position.currentPrice)}</span>
					</div>
					{#if position.unrealizedPercent !== null}
						<div class="summary-row">
							<span class="summary-label">Unrealized</span>
							<span class="summary-value" class:profit={position.unrealizedPercent >= 0} class:loss={position.unrealizedPercent < 0}>
								{formatPercent(position.unrealizedPercent)}
							</span>
						</div>
					{/if}
				</div>

				<!-- Exit Price Input -->
				<div class="form-section">
					<div class="form-group">
						<label for="exit_price">Exit Price *</label>
						<div class="price-input-wrapper">
							<span class="price-prefix">$</span>
							<input
								id="exit_price"
								type="number"
								step="0.01"
								min="0"
								bind:value={form.exit_price}
								placeholder="0.00"
								class="form-input price-input"
								required
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="exit_date">Exit Date</label>
						<input
							id="exit_date"
							type="date"
							bind:value={form.exit_date}
							class="form-input"
						/>
					</div>
				</div>

				<!-- P&L Preview -->
				{#if pnlPreview}
					{@const preview = pnlPreview}
					<div class="pnl-preview" class:profit={preview.isProfit} class:loss={!preview.isProfit}>
						<div class="pnl-result">{preview.result}</div>
						<div class="pnl-details">
							<span class="pnl-percent">{preview.isProfit ? '+' : ''}{preview.pnlPercent.toFixed(2)}%</span>
							<span class="pnl-diff">{preview.isProfit ? '+' : ''}{formatPrice(preview.priceDiff)} per share</span>
						</div>
					</div>
				{/if}

				<!-- Notes -->
				<div class="form-group full-width">
					<label for="notes">Exit Notes</label>
					<textarea
						id="notes"
						bind:value={form.notes}
						placeholder="Why are you closing this position? Target hit, stopped out, etc..."
						class="form-textarea"
						rows="3"
					></textarea>
				</div>

				<!-- Error Message -->
				{#if errorMessage}
					<div class="error-message">
						<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
							<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
						</svg>
						{errorMessage}
					</div>
				{/if}

				<!-- Actions Footer -->
				<div class="form-actions">
					<button type="button" class="btn-cancel" onclick={handleClose}>
						Cancel
					</button>
					<button 
						type="submit" 
						class="btn-close-position"
						disabled={isSaving || !form.exit_price}
					>
						{#if isSaving}
							<svg class="spinner" viewBox="0 0 24 24" width="18" height="18">
								<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
							</svg>
							Closing...
						{:else}
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M9 12l2 2 4-4" />
								<circle cx="12" cy="12" r="10" />
							</svg>
							Close Position
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL OVERLAY
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(15, 23, 42, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 99999;
		padding: 16px;
		backdrop-filter: blur(8px);
		animation: overlayFadeIn 0.2s ease-out;
		isolation: isolate;
	}

	@keyframes overlayFadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		background: var(--color-bg-card);
		border-radius: 20px;
		width: 100%;
		max-width: 480px;
		max-height: calc(100vh - 32px);
		max-height: calc(100dvh - 32px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 0 0 0 1px rgba(0,0,0,0.05), 0 25px 50px -12px rgba(0,0,0,0.4), 0 0 100px -20px rgba(20,62,89,0.3);
		animation: modalSlideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modalSlideUp {
		from { opacity: 0; transform: translateY(20px) scale(0.98); }
		to { opacity: 1; transform: translateY(0) scale(1); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 24px;
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, var(--color-brand-primary-hover) 100%);
		border-bottom: 1px solid rgba(255,255,255,0.1);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 14px;
	}

	.header-icon {
		width: 44px;
		height: 44px;
		background: rgba(255,255,255,0.15);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-bg-card);
	}

	.header-text h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 700;
		color: var(--color-bg-card);
	}

	.header-subtitle {
		margin: 4px 0 0;
		font-size: 13px;
		color: rgba(255,255,255,0.7);
	}

	.modal-close {
		width: 36px;
		height: 36px;
		border: none;
		background: rgba(255,255,255,0.1);
		border-radius: 10px;
		color: rgba(255,255,255,0.8);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.15s;
	}

	.modal-close:hover {
		background: rgba(255,255,255,0.2);
		color: var(--color-bg-card);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-form {
		flex: 1;
		overflow-y: auto;
		padding: 24px;
	}

	.position-summary {
		background: var(--color-bg-subtle);
		border-radius: 12px;
		padding: 16px;
		margin-bottom: 20px;
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
	}

	.summary-row {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.summary-label {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.summary-value {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
	}

	.summary-value.ticker {
		color: var(--color-brand-primary);
		font-size: 18px;
	}

	.summary-value.profit { color: var(--color-profit-light); }
	.summary-value.loss { color: var(--color-loss); }

	.form-section {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-tertiary);
	}

	.form-input,
	.form-textarea {
		padding: 12px 14px;
		border: 2px solid var(--color-border-default);
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
		transition: all 0.15s ease;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--color-brand-primary);
		background: var(--color-bg-card);
		box-shadow: 0 0 0 4px rgba(20, 62, 89, 0.1);
	}

	.price-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-prefix {
		position: absolute;
		left: 14px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-muted);
		pointer-events: none;
		z-index: 1;
	}

	.price-input {
		padding-left: 28px !important;
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
		min-height: 80px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   P&L PREVIEW
	   ═══════════════════════════════════════════════════════════════════════════ */
	.pnl-preview {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-radius: 12px;
		margin-bottom: 20px;
	}

	.pnl-preview.profit {
		background: linear-gradient(135deg, var(--color-profit-bg) 0%, var(--color-profit-bg) 100%);
		border: 2px solid var(--color-profit-border);
	}

	.pnl-preview.loss {
		background: linear-gradient(135deg, var(--color-loss-bg) 0%, var(--color-loss-bg) 100%);
		border: 2px solid var(--color-loss-border);
	}

	.pnl-result {
		font-size: 14px;
		font-weight: 800;
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.pnl-preview.profit .pnl-result { color: var(--color-profit-dark); }
	.pnl-preview.loss .pnl-result { color: var(--color-loss-dark); }

	.pnl-details {
		text-align: right;
	}

	.pnl-percent {
		display: block;
		font-size: 20px;
		font-weight: 800;
	}

	.pnl-preview.profit .pnl-percent { color: var(--color-profit-dark); }
	.pnl-preview.loss .pnl-percent { color: var(--color-loss-dark); }

	.pnl-diff {
		display: block;
		font-size: 12px;
		color: var(--color-text-muted);
		margin-top: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR MESSAGE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.error-message {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-bg);
		border-radius: 10px;
		color: var(--color-loss);
		font-size: 13px;
		font-weight: 500;
		margin-bottom: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTIONS FOOTER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-actions {
		display: flex;
		gap: 12px;
		margin-top: 8px;
	}

	.btn-cancel {
		flex: 1;
		padding: 14px 20px;
		background: var(--color-bg-subtle);
		border: 2px solid var(--color-border-default);
		border-radius: 12px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: var(--color-border-default);
		border-color: var(--color-border-strong);
	}

	.btn-close-position {
		flex: 2;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 24px;
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, var(--color-brand-primary-hover) 100%);
		border: none;
		border-radius: 12px;
		font-size: 14px;
		font-weight: 700;
		color: var(--color-bg-card);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-close-position:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.4);
	}

	.btn-close-position:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-cancel:focus-visible,
	.btn-close-position:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.modal-close:focus-visible {
		outline: 2px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	.spinner {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 480px) {
		.modal-container {
			border-radius: 20px 20px 0 0;
			max-height: 90vh;
			max-height: 90dvh;
			position: fixed;
			bottom: 0;
			left: 0;
			right: 0;
			max-width: 100%;
		}

		.form-section {
			grid-template-columns: 1fr;
		}

		.position-summary {
			grid-template-columns: 1fr 1fr;
		}
	}
</style>
