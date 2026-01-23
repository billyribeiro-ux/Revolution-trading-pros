<!--
	Trade Entry Modal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Full CRUD modal for trade plan entries with dark theme matching dashboard.
	Supports Create, Edit, and Delete operations.
	
	@version 2.0.0 - Complete CRUD with dark theme
-->
<script lang="ts">
	import { tradePlanApi, type Bias, type TradePlanEntry } from '$lib/api/room-content';
	import { onMount } from 'svelte';

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		editEntry?: TradePlanEntry | null;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, roomSlug, editEntry = null, onClose, onSuccess }: Props = $props();

	let isSaving = $state(false);
	let isDeleting = $state(false);
	let errorMessage = $state('');
	let showDeleteConfirm = $state(false);
	let modalRef = $state<HTMLDivElement | null>(null);

	let form = $state({
		ticker: '',
		bias: 'BULLISH' as Bias,
		entry: '',
		target1: '',
		target2: '',
		target3: '',
		runner: '',
		runner_stop: '',
		stop: '',
		options_strike: '',
		options_exp: '',
		notes: ''
	});

	const isEditMode = $derived(editEntry !== null && editEntry !== undefined);
	const isFormValid = $derived(form.ticker.trim() !== '');
	const modalTitle = $derived(isEditMode ? 'Edit Trade Entry' : 'Add Trade Entry');
	const submitLabel = $derived(isEditMode ? 'Save Changes' : 'Add Entry');

	// Populate form when editing
	$effect(() => {
		if (editEntry && isOpen) {
			form = {
				ticker: editEntry.ticker || '',
				bias: (editEntry.bias as Bias) || 'BULLISH',
				entry: editEntry.entry || '',
				target1: editEntry.target1 || '',
				target2: editEntry.target2 || '',
				target3: editEntry.target3 || '',
				runner: editEntry.runner || '',
				runner_stop: (editEntry as any).runner_stop || '',
				stop: editEntry.stop || '',
				options_strike: editEntry.options_strike || '',
				options_exp: editEntry.options_exp || '',
				notes: editEntry.notes || ''
			};
		}
	});

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
			bias: 'BULLISH',
			entry: '',
			target1: '',
			target2: '',
			target3: '',
			runner: '',
			runner_stop: '',
			stop: '',
			options_strike: '',
			options_exp: '',
			notes: ''
		};
		errorMessage = '';
		showDeleteConfirm = false;
	}

	async function handleSubmit() {
		if (!isFormValid) return;
		isSaving = true;
		errorMessage = '';

		try {
			if (isEditMode && editEntry?.id) {
				// Update existing entry
				await tradePlanApi.update(editEntry.id, {
					ticker: form.ticker.toUpperCase(),
					bias: form.bias,
					entry: form.entry || undefined,
					target1: form.target1 || undefined,
					target2: form.target2 || undefined,
					target3: form.target3 || undefined,
					runner: form.runner || undefined,
					runner_stop: form.runner_stop || undefined,
					stop: form.stop || undefined,
					options_strike: form.options_strike || undefined,
					options_exp: form.options_exp || undefined,
					notes: form.notes || undefined,
					room_slug: roomSlug
				} as any);
			} else {
				// Create new entry
				await tradePlanApi.create({
					room_slug: roomSlug,
					ticker: form.ticker.toUpperCase(),
					bias: form.bias,
					entry: form.entry || undefined,
					target1: form.target1 || undefined,
					target2: form.target2 || undefined,
					target3: form.target3 || undefined,
					runner: form.runner || undefined,
					runner_stop: form.runner_stop || undefined,
					stop: form.stop || undefined,
					options_strike: form.options_strike || undefined,
					options_exp: form.options_exp || undefined,
					notes: form.notes || undefined
				});
			}

			resetForm();
			onSuccess?.();
			onClose();
		} catch (err) {
			errorMessage = isEditMode 
				? 'Failed to update entry. Please try again.'
				: 'Failed to add entry. Please try again.';
			console.error(err);
		} finally {
			isSaving = false;
		}
	}

	async function handleDelete() {
		if (!editEntry?.id) return;
		isDeleting = true;
		errorMessage = '';

		try {
			await tradePlanApi.delete(editEntry.id);
			resetForm();
			onSuccess?.();
			onClose();
		} catch (err) {
			errorMessage = 'Failed to delete entry. Please try again.';
			console.error(err);
		} finally {
			isDeleting = false;
			showDeleteConfirm = false;
		}
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (showDeleteConfirm) {
				showDeleteConfirm = false;
			} else {
				handleClose();
			}
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
							<path d="M12 20V10M18 20V4M6 20v-4" />
						</svg>
					</div>
					<div class="header-text">
						<h3 id="modal-title">{modalTitle}</h3>
						<p class="header-subtitle">
							{#if isEditMode}
								Editing {form.ticker || 'entry'}
							{:else}
								Add a new trade plan entry
							{/if}
						</p>
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

			<!-- Delete Confirmation Overlay -->
			{#if showDeleteConfirm}
				<div class="delete-confirm-overlay">
					<div class="delete-confirm-card">
						<div class="delete-icon">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="32" height="32">
								<path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6M8 6V4a2 2 0 012-2h4a2 2 0 012 2v2" />
							</svg>
						</div>
						<h4>Delete {form.ticker}?</h4>
						<p>This action cannot be undone. The trade entry will be permanently removed.</p>
						<div class="delete-actions">
							<button class="btn-cancel-delete" onclick={() => showDeleteConfirm = false}>
								Cancel
							</button>
							<button class="btn-confirm-delete" onclick={handleDelete} disabled={isDeleting}>
								{#if isDeleting}
									<span class="spinner"></span>
									Deleting...
								{:else}
									Delete Entry
								{/if}
							</button>
						</div>
					</div>
				</div>
			{/if}

			<!-- Form Body -->
			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<!-- Ticker & Bias Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot"></span>
						Primary Setup
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="ticker">Ticker Symbol *</label>
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
							<label for="bias">Market Bias *</label>
							<div class="select-wrapper">
								<select id="bias" bind:value={form.bias} class="form-select">
									<option value="BULLISH">🟢 BULLISH</option>
									<option value="BEARISH">🔴 BEARISH</option>
									<option value="NEUTRAL">🟡 NEUTRAL</option>
								</select>
							</div>
						</div>
					</div>
				</div>

				<!-- Entry & Stop Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot"></span>
						Entry & Risk
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="entry">Entry Price</label>
							<div class="price-input-wrapper">
								<span class="price-prefix">$</span>
								<input id="entry" type="text" bind:value={form.entry} placeholder="142.50" class="form-input price-input" />
							</div>
						</div>
						<div class="form-group">
							<label for="stop">Stop Loss</label>
							<div class="price-input-wrapper stop-wrapper">
								<span class="price-prefix">$</span>
								<input id="stop" type="text" bind:value={form.stop} placeholder="136.00" class="form-input price-input stop-input" />
							</div>
						</div>
					</div>
				</div>

				<!-- Targets Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot target-dot"></span>
						Price Targets
					</div>
					<div class="form-row targets">
						<div class="form-group">
							<label for="target1">T1</label>
							<div class="price-input-wrapper target-wrapper">
								<span class="price-prefix">$</span>
								<input id="target1" type="text" bind:value={form.target1} placeholder="148" class="form-input price-input target-input" />
							</div>
						</div>
						<div class="form-group">
							<label for="target2">T2</label>
							<div class="price-input-wrapper target-wrapper">
								<span class="price-prefix">$</span>
								<input id="target2" type="text" bind:value={form.target2} placeholder="155" class="form-input price-input target-input" />
							</div>
						</div>
						<div class="form-group">
							<label for="target3">T3</label>
							<div class="price-input-wrapper target-wrapper">
								<span class="price-prefix">$</span>
								<input id="target3" type="text" bind:value={form.target3} placeholder="162" class="form-input price-input target-input" />
							</div>
						</div>
						<div class="form-group">
							<label for="runner">Runner</label>
							<div class="price-input-wrapper runner-wrapper">
								<span class="price-prefix">$</span>
								<input id="runner" type="text" bind:value={form.runner} placeholder="170+" class="form-input price-input runner-input" />
							</div>
						</div>
					</div>
				</div>

				<!-- Options Row -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot options-dot"></span>
						Options Play
					</div>
					<div class="form-row">
						<div class="form-group">
							<label for="options_strike">Strike Price</label>
							<input id="options_strike" type="text" bind:value={form.options_strike} placeholder="$145C" class="form-input" />
						</div>
						<div class="form-group">
							<label for="options_exp">Expiration</label>
							<input id="options_exp" type="text" bind:value={form.options_exp} placeholder="Feb 21" class="form-input" />
						</div>
					</div>
				</div>

				<!-- Notes -->
				<div class="form-section">
					<div class="section-label">
						<span class="section-dot notes-dot"></span>
						Trade Notes
					</div>
					<div class="form-group full-width">
						<textarea
							id="notes"
							bind:value={form.notes}
							placeholder="Entry thesis, key levels, catalysts, position sizing..."
							class="form-textarea"
							rows="3"
						></textarea>
					</div>
				</div>

				<!-- Actions Footer -->
				<div class="form-actions">
					{#if isEditMode}
						<button type="button" class="btn-delete" onclick={() => showDeleteConfirm = true}>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
								<path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6M8 6V4a2 2 0 012-2h4a2 2 0 012 2v2" />
							</svg>
							Delete
						</button>
					{/if}
					<div class="action-right">
						<button type="button" class="btn-cancel" onclick={handleClose}>Cancel</button>
						<button type="submit" class="btn-save" disabled={!isFormValid || isSaving}>
							{#if isSaving}
								<span class="spinner"></span>
								{isEditMode ? 'Saving...' : 'Adding...'}
							{:else}
								{submitLabel}
							{/if}
						</button>
					</div>
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
	   MODAL CONTAINER - Proper viewport-safe sizing
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		background: #ffffff;
		border-radius: 20px;
		width: 100%;
		max-width: 560px;
		max-height: calc(100vh - 32px);
		max-height: calc(100dvh - 32px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 
			0 0 0 1px rgba(0, 0, 0, 0.05),
			0 25px 50px -12px rgba(0, 0, 0, 0.4),
			0 0 100px -20px rgba(20, 62, 89, 0.3);
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
	   HEADER - Dark theme matching dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		background: linear-gradient(135deg, #143E59 0%, #1a4d6e 100%);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 14px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		color: #fff;
	}

	.header-text h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 700;
		color: #fff;
		letter-spacing: -0.3px;
	}

	.header-subtitle {
		margin: 2px 0 0;
		font-size: 13px;
		color: rgba(255, 255, 255, 0.7);
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		padding: 0;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 10px;
		color: rgba(255, 255, 255, 0.8);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.modal-close:hover {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
		transform: scale(1.05);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ERROR BANNER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.error-banner {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 16px 24px 0;
		padding: 14px 16px;
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		border: 1px solid #fecaca;
		border-radius: 12px;
		color: #dc2626;
		font-size: 14px;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DELETE CONFIRMATION OVERLAY
	   ═══════════════════════════════════════════════════════════════════════════ */
	.delete-confirm-overlay {
		position: absolute;
		inset: 0;
		background: rgba(15, 23, 42, 0.9);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 10;
		padding: 24px;
		animation: overlayFadeIn 0.2s ease-out;
	}

	.delete-confirm-card {
		background: #fff;
		border-radius: 16px;
		padding: 32px;
		text-align: center;
		max-width: 340px;
		animation: modalSlideUp 0.2s ease-out;
	}

	.delete-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: #fef2f2;
		border-radius: 50%;
		color: #dc2626;
		margin-bottom: 16px;
	}

	.delete-confirm-card h4 {
		margin: 0 0 8px;
		font-size: 18px;
		font-weight: 700;
		color: #1e293b;
	}

	.delete-confirm-card p {
		margin: 0 0 24px;
		font-size: 14px;
		color: #64748b;
		line-height: 1.5;
	}

	.delete-actions {
		display: flex;
		gap: 12px;
	}

	.btn-cancel-delete {
		flex: 1;
		padding: 12px 20px;
		background: #f1f5f9;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel-delete:hover {
		background: #e2e8f0;
	}

	.btn-confirm-delete {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 20px;
		background: #dc2626;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-confirm-delete:hover:not(:disabled) {
		background: #b91c1c;
	}

	.btn-confirm-delete:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FORM BODY
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-form {
		flex: 1;
		overflow-y: auto;
		padding: 24px;
	}

	.form-section {
		margin-bottom: 20px;
	}

	.section-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 11px;
		font-weight: 700;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.8px;
		margin-bottom: 12px;
	}

	.section-dot {
		width: 8px;
		height: 8px;
		background: #143E59;
		border-radius: 50%;
	}

	.section-dot.target-dot { background: #10b981; }
	.section-dot.options-dot { background: #8b5cf6; }
	.section-dot.notes-dot { background: #f59e0b; }

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 14px;
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
	}

	.form-input,
	.form-select,
	.form-textarea {
		padding: 12px 14px;
		border: 2px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		background: #f8fafc;
		color: #1e293b;
		transition: all 0.15s ease;
	}

	.form-input:hover,
	.form-select:hover,
	.form-textarea:hover {
		border-color: #cbd5e1;
	}

	.form-input:focus,
	.form-select:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #143E59;
		background: #fff;
		box-shadow: 0 0 0 4px rgba(20, 62, 89, 0.1);
	}

	.ticker-input {
		text-transform: uppercase;
		font-weight: 700;
		font-size: 16px;
		letter-spacing: 0.5px;
	}

	.stop-input:focus {
		border-color: #dc2626;
		box-shadow: 0 0 0 4px rgba(220, 38, 38, 0.1);
	}

	.target-input:focus {
		border-color: #10b981;
		box-shadow: 0 0 0 4px rgba(16, 185, 129, 0.1);
	}

	.runner-input:focus {
		border-color: #8b5cf6;
		box-shadow: 0 0 0 4px rgba(139, 92, 246, 0.1);
	}

	/* Price Input with $ Prefix */
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
		color: #64748b;
		pointer-events: none;
		z-index: 1;
	}

	.price-input {
		padding-left: 28px !important;
	}

	.target-wrapper .price-prefix { color: #10b981; }
	.stop-wrapper .price-prefix { color: #dc2626; }
	.runner-wrapper .price-prefix { color: #8b5cf6; }

	.select-wrapper {
		position: relative;
	}

	.form-select {
		appearance: none;
		cursor: pointer;
		padding-right: 40px;
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
		min-height: 80px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTIONS FOOTER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.form-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 2px solid #f1f5f9;
	}

	.action-right {
		display: flex;
		gap: 12px;
		margin-left: auto;
	}

	.btn-delete {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: transparent;
		border: 2px solid #fecaca;
		border-radius: 10px;
		font-size: 13px;
		font-weight: 600;
		color: #dc2626;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-delete:hover {
		background: #fef2f2;
		border-color: #dc2626;
	}

	.btn-cancel {
		padding: 12px 24px;
		background: #f1f5f9;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: #e2e8f0;
	}

	.btn-save {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 28px;
		background: linear-gradient(135deg, #143E59 0%, #1a4d6e 100%);
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		color: white;
		cursor: pointer;
		transition: all 0.15s;
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.25);
	}

	.btn-save:hover:not(:disabled) {
		background: linear-gradient(135deg, #0f2d42 0%, #143E59 100%);
		transform: translateY(-1px);
		box-shadow: 0 6px 16px rgba(20, 62, 89, 0.35);
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
		box-shadow: none;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.modal-overlay {
			padding: 0;
			align-items: flex-end;
		}

		.modal-container {
			max-height: 95vh;
			max-height: 95dvh;
			border-radius: 20px 20px 0 0;
			animation: modalSlideUpMobile 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		}

		@keyframes modalSlideUpMobile {
			from {
				opacity: 0;
				transform: translateY(100%);
			}
			to {
				opacity: 1;
				transform: translateY(0);
			}
		}

		.modal-header {
			padding: 16px 20px;
		}

		.header-icon {
			width: 40px;
			height: 40px;
		}

		.modal-form {
			padding: 20px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-row.targets {
			grid-template-columns: repeat(2, 1fr);
		}

		.form-actions {
			flex-direction: column;
			gap: 12px;
		}

		.action-right {
			width: 100%;
			margin-left: 0;
		}

		.btn-delete {
			width: 100%;
			justify-content: center;
			order: 3;
		}

		.btn-cancel,
		.btn-save {
			flex: 1;
			justify-content: center;
		}
	}
</style>
