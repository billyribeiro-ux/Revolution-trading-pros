<script lang="ts">
	/**
	 * InvalidatePositionModal - Mark trade as invalidated with reason
	 * @version 1.0.0
	 *
	 * For trades that didn't trigger or moved away before entry
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

	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	const reasons = [
		'Price moved away before entry',
		'Setup invalidated - pattern broken',
		'Market conditions changed',
		'Better opportunity elsewhere',
		'Other (specify in notes)'
	];

	let selectedReason = $state(reasons[0]);
	let customNotes = $state('');

	$effect(() => {
		if (isOpen && position) {
			selectedReason = reasons[0];
			customNotes = '';
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

		const finalReason =
			selectedReason === 'Other (specify in notes)'
				? customNotes
				: `${selectedReason}${customNotes ? ` - ${customNotes}` : ''}`;

		try {
			const response = await fetch(`/api/admin/trades/${position.id}/invalidate`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ reason: finalReason })
			});

			if (!response.ok) {
				const data = await response.json();
				throw new Error(data.error || 'Failed to invalidate position');
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
	<div
		class="modal-backdrop"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<div class="modal">
			<header class="modal-header">
				<h2 id="modal-title">Invalidate Position</h2>
				<span class="ticker-badge">{position.ticker}</span>
				<button type="button" class="close-btn" onclick={handleClose} aria-label="Close">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="20"
						height="20"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</header>

			{#if error}
				<div class="error-banner" role="alert">{error}</div>
			{/if}

			<form onsubmit={handleSubmit}>
				<div class="form-body">
					<p class="info-text">
						Mark this trade as invalidated. This is for setups that didn't trigger or moved away
						before entry.
					</p>

					<div class="form-group">
						<label for="reason">Reason</label>
						<select id="reason" bind:value={selectedReason}>
							{#each reasons as r}
								<option value={r}>{r}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="notes">Additional Notes (optional)</label>
						<textarea
							id="notes"
							bind:value={customNotes}
							rows="3"
							placeholder="Add any additional context..."
						></textarea>
					</div>
				</div>

				<footer class="modal-footer">
					<button type="button" class="btn-secondary" onclick={handleClose} disabled={isSubmitting}>
						Cancel
					</button>
					<button type="submit" class="btn-warning" disabled={isSubmitting}>
						{#if isSubmitting}
							Invalidating...
						{:else}
							Invalidate Trade
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
		z-index: 10000;
		animation: fadeIn 0.15s ease-out;
		isolation: isolate;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.modal {
		background: var(--color-bg-card);
		border-radius: 12px;
		box-shadow: var(--shadow-xl);
		width: 100%;
		max-width: 440px;
		overflow: hidden;
		animation: slideUp 0.2s ease-out;
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
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
		background: var(--color-warning, #f59e0b);
		color: white;
		border-radius: 4px;
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
	}
	.close-btn:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.error-banner {
		padding: 12px 20px;
		background: var(--color-loss-bg);
		color: var(--color-loss);
		font-size: 13px;
		font-weight: 500;
	}

	.form-body {
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.info-text {
		font-size: 14px;
		color: var(--color-text-secondary);
		margin: 0;
		line-height: 1.5;
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
	}

	.form-group select,
	.form-group textarea {
		padding: 10px 12px;
		font-size: 14px;
		border: 1px solid var(--color-border-default);
		border-radius: 8px;
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--color-brand-primary);
		box-shadow: 0 0 0 3px var(--color-brand-primary-alpha, rgba(20, 62, 89, 0.2));
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
	.btn-warning {
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
	}

	.btn-secondary {
		background: transparent;
		border: 1px solid var(--color-border-default);
		color: var(--color-text-secondary);
	}

	.btn-warning {
		background: var(--color-warning, #f59e0b);
		border: none;
		color: white;
	}
	.btn-warning:hover:not(:disabled) {
		background: #d97706;
	}
	.btn-warning:disabled,
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.modal-backdrop {
			padding: 16px;
		}

		.modal {
			max-width: 100%;
		}

		.form-body {
			padding: 16px;
		}

		.modal-footer {
			padding: 12px 16px;
		}

		.btn-secondary,
		.btn-warning {
			padding: 12px 16px;
		}
	}
</style>
