<script lang="ts">
	import { IconX, IconCalendar } from '#lib/icons/index.js';
	import type { MemberMembership } from '#lib/api/member-profile.js';
	import { recordedWallClock } from './profile-display.js';

	interface Props {
		open: boolean;
		subscription: MemberMembership | null;
		newExpiry: string;
		onNewExpiryChange: (value: string) => void;
		error?: string;
		extendDays: number;
		extending: boolean;
		onDaysChange: (value: number) => void;
		onClose: () => void;
		onExtend: () => void;
	}

	let {
		open,
		subscription,
		newExpiry,
		onNewExpiryChange,
		error = '',
		extendDays,
		extending,
		onDaysChange,
		onClose,
		onExtend
	}: Props = $props();

	// Pre-defined quick-pick values. Bounded set keeps selected-state styling
	// limited to these six values.
	const PRESETS = [7, 14, 30, 60, 90, 365] as const;
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
		aria-label="Extend Membership"
	>
		<div class="modal-content" role="document">
			<div class="modal-header">
				<h2>Extend Membership</h2>
				<button class="close-btn" onclick={onClose} aria-label="Close extension dialog">
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="extend-info">
					Extending: <strong
						>{subscription.plan_name ?? `Plan #${subscription.plan_id} (name not recorded)`}</strong
					>
				</p>
				<p>Recorded expiry: {recordedWallClock(subscription.expires_at)}</p>
				<p>
					Existing historical timestamps have no verified timezone. New expiry writes use
					operational UTC.
				</p>
				{#if error}<p role="alert">{error}</p>{/if}
				<div class="form-group">
					<label for="extend-days">Extend by (days)</label>
					<div class="extend-options">
						{#each PRESETS as days (days)}
							<button
								type="button"
								class={['extend-option', { selected: extendDays === days }]}
								onclick={() => onDaysChange(days)}
								disabled={extending}
							>
								{days} days
							</button>
						{/each}
					</div>
					<input
						autocomplete="off"
						id="extend-days"
						disabled={extending}
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
				<div class="form-group">
					<label for="extend-new-expiry">New expiry (operational UTC)</label><input
						id="extend-new-expiry"
						type="datetime-local"
						step="1"
						value={newExpiry}
						disabled={extending}
						oninput={(event) => onNewExpiryChange(event.currentTarget.value)}
					/><small
						>For an open-ended or unrepresentable recorded expiry, choose the new expiry explicitly.
						Day presets require a finite recorded expiry.</small
					>
				</div>
				<p class="extend-preview">
					New expiration: <strong
						>{newExpiry
							? `${newExpiry} UTC (operational convention)`
							: 'Choose a new expiry'}</strong
					>
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Cancel</button>
				<button class="btn-primary" onclick={onExtend} disabled={extending || !newExpiry}>
					<IconCalendar size={18} />
					{extending ? 'Extending...' : 'Save new expiry'}
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
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
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
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: none;
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
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
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.5rem;
	}

	.extend-info {
		color: var(--text-secondary, #8b949e);
		font-size: 0.9375rem;
		margin-bottom: 1.25rem;
	}

	.extend-info strong {
		color: var(--text-primary, #f0f6fc);
	}

	.extend-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.extend-option {
		padding: 0.5rem 0.875rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
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
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
	}

	.extend-preview {
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
	}

	.extend-preview strong {
		color: var(--success-emphasis, #3fb950);
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
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}
</style>
