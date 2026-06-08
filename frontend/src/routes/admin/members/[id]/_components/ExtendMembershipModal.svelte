<script lang="ts">
	import { IconX, IconCalendar } from '$lib/icons';
	import type { Subscription } from '$lib/api/members';
	import { formatDate } from './helpers';

	interface Props {
		open: boolean;
		subscription: Subscription | null;
		extendDays: number;
		extending: boolean;
		onDaysChange: (value: number) => void;
		onClose: () => void;
		onExtend: () => void;
	}

	let { open, subscription, extendDays, extending, onDaysChange, onClose, onExtend }: Props =
		$props();

	// Pre-defined quick-pick values. Bounded set keeps selected-state styling
	// limited to these six values.
	const PRESETS = [7, 14, 30, 60, 90, 365] as const;

	let previewDate = $derived.by(() => {
		if (!subscription) return '';
		const base = subscription.next_payment ? new Date(subscription.next_payment) : new Date();
		return formatDate(new Date(base.getTime() + extendDays * 24 * 60 * 60 * 1000).toISOString());
	});
</script>

{#if open && subscription}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) onClose();
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div class="modal-content" role="document">
			<div class="modal-header">
				<h2>Extend Membership</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="extend-info">
					Extending: <strong>{subscription.product || 'Membership'}</strong>
				</p>
				<div class="form-group">
					<label for="extend-days">Extend by (days)</label>
					<div class="extend-options">
						{#each PRESETS as days (days)}
							<button
								type="button"
								class={['extend-option', { selected: extendDays === days }]}
								onclick={() => onDaysChange(days)}
							>
								{days} days
							</button>
						{/each}
					</div>
					<input
						id="extend-days"
						name="extend-days"
						type="number"
						value={extendDays}
						oninput={(e) => onDaysChange(Number((e.target as HTMLInputElement).value))}
						min="1"
						max="3650"
						class="extend-custom"
						placeholder="Custom days..."
					/>
				</div>
				<p class="extend-preview">
					New expiration: <strong>{previewDate}</strong>
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Cancel</button>
				<button class="btn-primary" onclick={onExtend} disabled={extending || extendDays < 1}>
					<IconCalendar size={18} />
					{extending ? 'Extending...' : `Extend by ${extendDays} Days`}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.extend-info {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin-bottom: 1.25rem;
	}

	.extend-info strong {
		color: #f1f5f9;
	}

	.extend-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.extend-option {
		padding: 0.5rem 0.875rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.extend-option:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.extend-option.selected {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.5);
		color: var(--primary-400);
	}

	.extend-custom {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.extend-preview {
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.extend-preview strong {
		color: #34d399;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}
</style>
