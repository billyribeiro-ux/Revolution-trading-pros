<!--
	Add Trade Modal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Full trade creation modal with all fields matching ActivePositionCard display.
	Dark theme matching dashboard (#143E59).
	
	@version 1.0.0
-->
<script lang="ts">
	import type { TradeType, OptionType, ContractType, TradeSetup } from '$lib/types/trading';

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, roomSlug, onClose, onSuccess }: Props = $props();

	let isSaving = $state(false);
	let errorMessage = $state('');
	let modalRef = $state<HTMLDivElement | null>(null);

	let form = $state({
		ticker: '',
		trade_type: 'shares' as TradeType,
		direction: 'long' as 'long' | 'short',
		quantity: 100,
		option_type: undefined as OptionType | undefined,
		strike: undefined as number | undefined,
		expiration: '',
		contract_type: undefined as ContractType | undefined,
		entry_price: '',
		entry_date: new Date().toISOString().split('T')[0],
		setup: undefined as TradeSetup | undefined,
		notes: ''
	});

	const isFormValid = $derived(
		form.ticker.trim() !== '' &&
		form.entry_price !== '' &&
		parseFloat(form.entry_price) > 0 &&
		form.quantity > 0
	);

	// Focus trap and body scroll lock
	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
			setTimeout(() => modalRef?.focus(), 50);
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	function resetForm() {
		form = {
			ticker: '',
			trade_type: 'shares',
			direction: 'long',
			quantity: 100,
			option_type: undefined,
			strike: undefined,
			expiration: '',
			contract_type: undefined,
			entry_price: '',
			entry_date: new Date().toISOString().split('T')[0],
			setup: undefined,
			notes: ''
		};
		errorMessage = '';
	}

	async function handleSubmit() {
		if (!isFormValid) return;
		isSaving = true;
		errorMessage = '';

		try {
			const payload = {
				room_slug: roomSlug,
				ticker: form.ticker.toUpperCase(),
				trade_type: form.trade_type,
				direction: form.direction,
				quantity: form.quantity,
				option_type: form.option_type,
				strike: form.strike,
				expiration: form.expiration || undefined,
				contract_type: form.contract_type,
				entry_price: parseFloat(form.entry_price),
				entry_date: form.entry_date,
				setup: form.setup,
				notes: form.notes || undefined
			};

			const response = await fetch(`/api/trades/${roomSlug}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const error = await response.json().catch(() => ({ error: 'Failed to create trade' }));
				throw new Error(error.error || 'Failed to create trade');
			}

			resetForm();
			onSuccess?.();
			onClose();
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Failed to create trade. Please try again.';
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
		if (e.key === 'Escape') {
			handleClose();
		}
	}

	function handleOverlayClick(e: MouseEvent) {
		if (e.target === e.currentTarget) handleClose();
	}
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		bind:this={modalRef}
		class="modal-overlay" 
		role="dialog" 
		aria-modal="true" 
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={handleOverlayClick}
		onkeydown={handleKeydown}
	>
		<div class="modal-container">
			<!-- Dark Header -->
			<div class="modal-header">
				<div class="header-content">
					<div class="header-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="22" height="22">
							<path d="M12 4v16m8-8H4" />
						</svg>
					</div>
					<div class="header-text">
						<h3 id="modal-title">Add New Trade</h3>
						<p class="header-subtitle">Open a new position</p>
					</div>
				</div>
				<button class="modal-close" onclick={handleClose} aria-label="Close modal">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Error Banner -->
			{#if errorMessage}
				<div class="error-banner">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
						<circle cx="12" cy="12" r="10" />
						<path d="M12 8v4M12 16h.01" />
					</svg>
					{errorMessage}
				</div>
			{/if}

			<!-- Form Body -->
			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<!-- Ticker & Type Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot"></span>
						Trade Setup
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="ticker">Ticker Symbol *</label>
							<input
								id="ticker"
								name="ticker"
								type="text"
								bind:value={form.ticker}
								placeholder="NVDA"
								class="form-input ticker-input"
								required
							/>
						</div>
						<div class="form-group">
							<label for="trade_type">Trade Type *</label>
							<div class="select-wrapper">
								<select id="trade_type" name="trade_type" bind:value={form.trade_type} class="form-select">
									<option value="shares">Shares</option>
									<option value="options">Options</option>
								</select>
							</div>
						</div>
					</div>
				</div>

				<!-- Direction & Quantity Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot"></span>
						Position Details
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="direction">Direction *</label>
							<div class="select-wrapper">
								<select id="direction" name="direction" bind:value={form.direction} class="form-select">
									<option value="long">🟢 Long</option>
									<option value="short">🔴 Short</option>
								</select>
							</div>
						</div>
						<div class="form-group">
							<label for="quantity">Quantity *</label>
							<input
								id="quantity"
								name="quantity"
								type="number"
								bind:value={form.quantity}
								placeholder="100"
								class="form-input"
								min="1"
								required
							/>
						</div>
					</div>
				</div>

				<!-- Options Fields (if option type) -->
				{#if form.trade_type === 'options'}
					<div class="form-section">
						<div class="section-label">
							<span class="section-dot options-dot"></span>
							Options Details
						</div>
						<div class="form-row">
							<div class="form-group">
								<label for="option_type">Option Type</label>
								<div class="select-wrapper">
									<select id="option_type" name="option_type" bind:value={form.option_type} class="form-select">
										<option value={undefined}>Select...</option>
										<option value="call">Call</option>
										<option value="put">Put</option>
									</select>
								</div>
							</div>
							<div class="form-group">
								<label for="strike">Strike Price</label>
								<input
									id="strike"
									name="strike"
									type="number"
									step="0.01"
									bind:value={form.strike}
									placeholder="145.00"
									class="form-input"
								/>
							</div>
						</div>
						<div class="form-row">
							<div class="form-group">
								<label for="expiration">Expiration Date</label>
								<input
									id="expiration"
									name="expiration"
									type="date"
									bind:value={form.expiration}
									class="form-input"
								/>
							</div>
							<div class="form-group">
								<label for="contract_type">Contract Type</label>
								<div class="select-wrapper">
									<select id="contract_type" name="contract_type" bind:value={form.contract_type} class="form-select">
										<option value={undefined}>Select...</option>
										<option value="weekly">Weekly</option>
										<option value="monthly">Monthly</option>
										<option value="leap">LEAP</option>
									</select>
								</div>
							</div>
						</div>
					</div>
				{/if}

				<!-- Entry & Date Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot"></span>
						Entry & Risk
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="entry_price">Entry Price *</label>
							<div class="price-input-wrapper">
								<span class="price-prefix">$</span>
								<input
									id="entry_price"
									name="entry_price"
									type="text"
									bind:value={form.entry_price}
									placeholder="142.50"
									class="form-input price-input"
									required
								/>
							</div>
						</div>
						<div class="form-group">
							<label for="entry_date">Entry Date *</label>
							<input
								id="entry_date"
								name="entry_date"
								type="date"
								bind:value={form.entry_date}
								class="form-input"
								required
							/>
						</div>
					</div>
				</div>

				<!-- Setup & Notes -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot notes-dot"></span>
						Additional Info
					</div>
					<div class="form-group">
						<label for="setup">Trade Setup</label>
						<div class="select-wrapper">
							<select id="setup" name="setup" bind:value={form.setup} class="form-select">
								<option value={undefined}>Select...</option>
								<option value="Breakout">Breakout</option>
								<option value="Momentum">Momentum</option>
								<option value="Reversal">Reversal</option>
								<option value="Earnings">Earnings</option>
								<option value="Pullback">Pullback</option>
							</select>
						</div>
					</div>
					<div class="form-group full-width">
						<label for="notes">Trade Notes</label>
						<textarea
							id="notes"
							name="notes"
							bind:value={form.notes}
							placeholder="Entry thesis, key levels, catalysts, position sizing..."
							class="form-textarea"
							rows="3"
						></textarea>
					</div>
				</div>

				<!-- Actions Footer -->
				<div class="form-actions">
					<button type="button" class="btn-cancel" onclick={handleClose}>Cancel</button>
					<button type="submit" class="btn-save" disabled={!isFormValid || isSaving}>
						{#if isSaving}
							<span class="spinner"></span>
							Adding...
						{:else}
							Add Trade
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL OVERLAY - Fixed viewport positioning
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(15, 23, 42, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 99999;
		padding: 60px 16px 16px;
		backdrop-filter: blur(8px);
		animation: overlayFadeIn 0.2s ease-out;
		isolation: isolate;
		overflow-y: auto;
	}

	@keyframes overlayFadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER - Proper viewport-safe sizing
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		background: #ffffff;
		border-radius: 20px;
		width: 100%;
		max-width: 640px;
		max-height: calc(100vh - 32px);
		max-height: calc(100dvh - 32px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 
			0 25px 50px -12px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(0, 0, 0, 0.05);
		animation: modalSlideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modalSlideUp {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.98);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL HEADER - Dark theme with icon
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		background: linear-gradient(135deg, #143E59 0%, #0f2d42 100%);
		padding: 24px 28px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-shrink: 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.header-icon {
		width: 44px;
		height: 44px;
		background: rgba(255, 255, 255, 0.12);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffffff;
		flex-shrink: 0;
	}

	.header-text h3 {
		margin: 0;
		font-size: 20px;
		font-weight: 700;
		color: #ffffff;
		line-height: 1.2;
	}

	.header-subtitle {
		margin: 4px 0 0 0;
		font-size: 13px;
		color: rgba(255, 255, 255, 0.7);
		font-weight: 500;
	}

	.modal-close {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		background: rgba(255, 255, 255, 0.08);
		border: 1px solid rgba(255, 255, 255, 0.12);
		color: rgba(255, 255, 255, 0.9);
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s ease;
		flex-shrink: 0;
	}

	.modal-close:hover {
		background: rgba(255, 255, 255, 0.15);
		border-color: rgba(255, 255, 255, 0.2);
		transform: scale(1.05);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR BANNER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.error-banner {
		background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
		color: #991b1b;
		padding: 14px 20px;
		display: flex;
		align-items: center;
		gap: 12px;
		font-size: 14px;
		font-weight: 600;
		border-bottom: 1px solid #fca5a5;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM BODY - Scrollable content
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-form {
		padding: 28px;
		overflow-y: auto;
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM SECTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-section {
		display: flex;
		flex-direction: column;
		gap: 14px;
	}

	.section-label {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 13px;
		font-weight: 700;
		color: #334155;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 4px;
	}

	.section-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: linear-gradient(135deg, #143E59 0%, #0f2d42 100%);
		flex-shrink: 0;
	}

	.options-dot {
		background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
	}

	.notes-dot {
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM ROWS & GROUPS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	label {
		font-size: 13px;
		font-weight: 600;
		color: #475569;
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM INPUTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-input,
	.form-select,
	.form-textarea {
		width: 100%;
		padding: 12px 14px;
		border: 2px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		color: #0f172a;
		background: #ffffff;
		transition: all 0.2s ease;
		font-family: inherit;
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
		font-weight: 700;
		letter-spacing: 0.05em;
	}

	.form-textarea {
		resize: vertical;
		min-height: 80px;
		line-height: 1.5;
	}

	/* Price Input Wrapper */
	.price-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-prefix {
		position: absolute;
		left: 14px;
		font-size: 14px;
		font-weight: 700;
		color: #64748b;
		pointer-events: none;
	}

	.price-input {
		padding-left: 30px;
		font-variant-numeric: tabular-nums;
	}

	/* Select Wrapper */
	.select-wrapper {
		position: relative;
	}

	.select-wrapper::after {
		content: '';
		position: absolute;
		right: 14px;
		top: 50%;
		transform: translateY(-50%);
		width: 0;
		height: 0;
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
		border-top: 6px solid #64748b;
		pointer-events: none;
	}

	.form-select {
		appearance: none;
		cursor: pointer;
		padding-right: 40px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-actions {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
		padding-top: 8px;
		border-top: 1px solid #f1f5f9;
		flex-shrink: 0;
	}

	.btn-cancel,
	.btn-save {
		padding: 12px 24px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
		display: flex;
		align-items: center;
		gap: 8px;
		font-family: inherit;
	}

	.btn-cancel {
		background: #f1f5f9;
		color: #475569;
	}

	.btn-cancel:hover {
		background: #e2e8f0;
		transform: translateY(-1px);
	}

	.btn-save {
		background: linear-gradient(135deg, #143E59 0%, #0f2d42 100%);
		color: #ffffff;
		box-shadow: 0 2px 8px rgba(20, 62, 89, 0.2);
	}

	.btn-save:hover:not(:disabled) {
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
		transform: translateY(-2px);
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Spinner */
	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #ffffff;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.form-row {
			grid-template-columns: 1fr;
		}

		.modal-container {
			max-width: calc(100vw - 24px);
		}
	}
</style>
