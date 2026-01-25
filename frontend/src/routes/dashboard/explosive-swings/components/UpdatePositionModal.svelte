<script lang="ts">
	/**
	 * UpdatePositionModal - Edit existing position details
	 * @version 1.0.0 - Apple ICT Level 7
	 * 
	 * Allows editing:
	 * - Entry price (if averaged in/out)
	 * - Current price (manual override)
	 * - Stop loss (trailing stop adjustment)
	 * - Targets (partial take profit, target adjustment)
	 * - Status (WATCHING → ENTRY → ACTIVE)
	 * - Notes
	 */
	import type { ActivePosition } from '../types';

	interface Props {
		isOpen: boolean;
		position: ActivePosition | null;
		roomSlug: string;
		onClose: () => void;
		onSuccess: () => void;
	}

	const { isOpen, position, roomSlug, onClose, onSuccess }: Props = $props();

	// Form state - initialized from position
	let entryPrice = $state('');
	let currentPrice = $state('');
	let stopLoss = $state('');
	let target1 = $state('');
	let target2 = $state('');
	let target3 = $state('');
	let status = $state<'WATCHING' | 'ENTRY' | 'ACTIVE'>('ACTIVE');
	let notes = $state('');

	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	// Reset form when position changes
	$effect(() => {
		if (position && isOpen) {
			entryPrice = position.entryPrice?.toString() ?? '';
			currentPrice = position.currentPrice?.toString() ?? '';
			stopLoss = position.stopLoss?.price?.toString() ?? '';
			target1 = position.targets?.[0]?.price?.toString() ?? '';
			target2 = position.targets?.[1]?.price?.toString() ?? '';
			target3 = position.targets?.[2]?.price?.toString() ?? '';
			status = position.status as 'WATCHING' | 'ENTRY' | 'ACTIVE';
			notes = position.notes ?? '';
			error = null;
		}
	});

	function handleClose() {
		if (!isSubmitting) {
			onClose();
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!position) return;

		isSubmitting = true;
		error = null;

		try {
			const payload = {
				entry_price: entryPrice ? parseFloat(entryPrice) : null,
				current_price: currentPrice ? parseFloat(currentPrice) : null,
				stop_loss: stopLoss ? parseFloat(stopLoss) : null,
				target_1: target1 ? parseFloat(target1) : null,
				target_2: target2 ? parseFloat(target2) : null,
				target_3: target3 ? parseFloat(target3) : null,
				status,
				notes: notes.trim() || null
			};

			const response = await fetch(`/api/trading-rooms/${roomSlug}/positions/${position.id}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to update position');
			}

			onSuccess();
			onClose();
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen && position}
	<div class="modal-backdrop" onclick={handleBackdropClick} onkeydown={handleKeydown} role="dialog" aria-modal="true" aria-labelledby="modal-title" tabindex="-1">
		<div class="modal">
			<header class="modal-header">
				<h2 id="modal-title">Update Position</h2>
				<span class="ticker-badge">{position.ticker}</span>
				<button type="button" class="close-btn" onclick={handleClose} aria-label="Close modal">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
						<path d="M18 6L6 18M6 6l12 12"/>
					</svg>
				</button>
			</header>

			{#if error}
				<div class="error-banner" role="alert">
					<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
						<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
					</svg>
					{error}
				</div>
			{/if}

			<form onsubmit={handleSubmit}>
				<div class="form-body">
					<!-- Status -->
					<div class="form-group">
						<label for="status">Status</label>
						<select id="status" bind:value={status}>
							<option value="WATCHING">Watching</option>
							<option value="ENTRY">Entry</option>
							<option value="ACTIVE">Active</option>
						</select>
					</div>

					<!-- Price Row -->
					<div class="form-row">
						<div class="form-group">
							<label for="entryPrice">Entry Price</label>
							<input 
								type="number" 
								id="entryPrice" 
								bind:value={entryPrice}
								step="0.01"
								placeholder="0.00"
							/>
						</div>
						<div class="form-group">
							<label for="currentPrice">Current Price</label>
							<input 
								type="number" 
								id="currentPrice" 
								bind:value={currentPrice}
								step="0.01"
								placeholder="0.00"
							/>
						</div>
					</div>

					<!-- Stop Loss -->
					<div class="form-group">
						<label for="stopLoss">Stop Loss</label>
						<input 
							type="number" 
							id="stopLoss" 
							bind:value={stopLoss}
							step="0.01"
							placeholder="0.00"
						/>
					</div>

					<!-- Targets Row -->
					<div class="form-row three-col">
						<div class="form-group">
							<label for="target1">Target 1</label>
							<input 
								type="number" 
								id="target1" 
								bind:value={target1}
								step="0.01"
								placeholder="0.00"
							/>
						</div>
						<div class="form-group">
							<label for="target2">Target 2</label>
							<input 
								type="number" 
								id="target2" 
								bind:value={target2}
								step="0.01"
								placeholder="0.00"
							/>
						</div>
						<div class="form-group">
							<label for="target3">Target 3</label>
							<input 
								type="number" 
								id="target3" 
								bind:value={target3}
								step="0.01"
								placeholder="0.00"
							/>
						</div>
					</div>

					<!-- Notes -->
					<div class="form-group">
						<label for="notes">Notes</label>
						<textarea 
							id="notes" 
							bind:value={notes}
							rows="3"
							placeholder="Add notes about this position..."
						></textarea>
					</div>
				</div>

				<footer class="modal-footer">
					<button type="button" class="btn-secondary" onclick={handleClose} disabled={isSubmitting}>
						Cancel
					</button>
					<button type="submit" class="btn-primary" disabled={isSubmitting}>
						{#if isSubmitting}
							<span class="spinner"></span>
							Updating...
						{:else}
							Update Position
						{/if}
					</button>
				</footer>
			</form>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
		z-index: 100;
		animation: fadeIn 0.15s ease-out;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.modal {
		background: var(--color-bg-card);
		border-radius: 12px;
		box-shadow: var(--shadow-xl);
		width: 100%;
		max-width: 480px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		animation: slideUp 0.2s ease-out;
	}

	@keyframes slideUp {
		from { opacity: 0; transform: translateY(20px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 20px;
		border-bottom: 1px solid var(--color-border-default);
	}

	.modal-header h2 {
		font-size: 18px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0;
	}

	.ticker-badge {
		font-size: 12px;
		font-weight: 700;
		padding: 4px 8px;
		background: var(--color-brand-primary);
		color: white;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.close-btn {
		margin-left: auto;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: background 0.15s, color 0.15s;
	}
	.close-btn:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: var(--color-loss-bg);
		color: var(--color-loss);
		font-size: 13px;
		font-weight: 500;
	}

	.form-body {
		padding: 20px;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group label {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 12px;
		font-size: 14px;
		border: 1px solid var(--color-border-default);
		border-radius: 8px;
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
		transition: border-color 0.15s, box-shadow 0.15s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--color-brand-primary);
		box-shadow: 0 0 0 3px var(--color-brand-primary-alpha);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
	}

	.form-row.three-col {
		grid-template-columns: 1fr 1fr 1fr;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 16px 20px;
		border-top: 1px solid var(--color-border-default);
		background: var(--color-bg-subtle);
	}

	.btn-secondary,
	.btn-primary {
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: background 0.15s, opacity 0.15s;
	}

	.btn-secondary {
		background: transparent;
		border: 1px solid var(--color-border-default);
		color: var(--color-text-secondary);
	}
	.btn-secondary:hover:not(:disabled) {
		background: var(--color-bg-subtle);
	}

	.btn-primary {
		background: var(--color-brand-primary);
		border: none;
		color: white;
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.btn-primary:hover:not(:disabled) {
		background: var(--color-brand-primary-hover);
	}
	.btn-primary:disabled,
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	@media (max-width: 520px) {
		.modal { max-width: 100%; }
		.form-row { grid-template-columns: 1fr; }
		.form-row.three-col { grid-template-columns: 1fr; }
	}
</style>
