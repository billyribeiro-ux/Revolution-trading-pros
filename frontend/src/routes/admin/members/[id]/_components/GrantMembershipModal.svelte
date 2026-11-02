<script lang="ts">
	import { IconX, IconGift } from '#lib/icons/index.js';
	import type { MembershipPlan } from './helpers';

	interface Props {
		open: boolean;
		memberName: string;
		availablePlans: MembershipPlan[];
		selectedPlanId: number | null;
		expiresAt: string;
		granting: boolean;
		onPlanChange: (value: number | null) => void;
		onExpiresAtChange: (value: string) => void;
		onClose: () => void;
		onGrant: () => void;
	}

	let {
		open,
		memberName,
		availablePlans,
		selectedPlanId,
		expiresAt,
		granting,
		onPlanChange,
		onExpiresAtChange,
		onClose,
		onGrant
	}: Props = $props();

	let minDate = $derived(new Date().toISOString().split('T')[0]);
</script>

{#if open}
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
				<h2>Grant Membership</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="grant-info">
					Granting membership to: <strong>{memberName}</strong>
				</p>
				<div class="form-group">
					<label for="grant-plan">Select Plan</label>
					<select
						id="grant-plan"
						value={selectedPlanId}
						onchange={(e) => {
							const raw = (e.target as HTMLSelectElement).value;
							onPlanChange(raw ? Number(raw) : null);
						}}
					>
						<option value="">Select a plan...</option>
						{#each availablePlans as plan (plan.id)}
							<option value={plan.id}>{plan.name}</option>
						{/each}
					</select>
				</div>
				<div class="form-group">
					<label for="grant-expires">Expiration Date (optional)</label>
					<input autocomplete="off"
						id="grant-expires"
						name="grant-expires"
						type="date"
						value={expiresAt}
						oninput={(e) => onExpiresAtChange((e.target as HTMLInputElement).value)}
						min={minDate}
					/>
					<small class="form-hint">Leave empty for no expiration (lifetime)</small>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Cancel</button>
				<button class="btn-primary" onclick={onGrant} disabled={granting || !selectedPlanId}>
					<IconGift size={18} />
					{granting ? 'Granting...' : 'Grant Membership'}
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

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
		font-family: inherit;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
		font-family: inherit;
		cursor: pointer;
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.grant-info {
		color: var(--text-secondary, #8b949e);
		font-size: 0.9375rem;
		margin-bottom: 1.25rem;
	}

	.grant-info strong {
		color: var(--text-primary, #f0f6fc);
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
