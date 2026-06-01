<script lang="ts">
	import { IconCreditCard, IconCalendar, IconTrash, IconGift } from '$lib/icons';
	import type { Subscription } from '$lib/api/members';
	import { formatCurrency, formatDate } from './helpers';

	interface Props {
		subscriptions: Subscription[] | undefined;
		onGrant: () => void;
		onExtend: (sub: Subscription) => void;
		onRevoke: (subId: number) => void;
	}

	let { subscriptions, onGrant, onExtend, onRevoke }: Props = $props();
</script>

<div class="panel">
	<div class="panel-header">
		<h3>Subscription History</h3>
		<button class="btn-primary small" onclick={onGrant}>
			<IconGift size={16} />
			Grant Membership
		</button>
	</div>
	{#if !subscriptions || subscriptions.length === 0}
		<div class="empty-state">
			<IconCreditCard size={48} stroke={1} />
			<h4>No Subscriptions</h4>
			<p>This member has no subscription history</p>
		</div>
	{:else}
		<div class="subscriptions-list">
			{#each subscriptions as sub (sub.id)}
				<div class="subscription-card">
					<div class="subscription-header">
						<div class="subscription-product">
							<h4>{sub.product || 'Unknown Product'}</h4>
							<span class="subscription-plan">{sub.plan || 'Standard'}</span>
						</div>
						<span
							class="status-badge {sub.status === 'active'
								? 'bg-emerald-500/20 text-emerald-400'
								: 'bg-slate-500/20 text-slate-400'}"
						>
							{sub.status}
						</span>
					</div>
					<div class="subscription-details">
						<div class="subscription-detail">
							<span class="label">Price</span>
							<span class="value">{formatCurrency(sub.price)}/{sub.interval}</span>
						</div>
						<div class="subscription-detail">
							<span class="label">Started</span>
							<span class="value">{sub.start_date ? formatDate(sub.start_date) : 'N/A'}</span>
						</div>
						<div class="subscription-detail">
							<span class="label">Next Payment</span>
							<span class="value">{sub.next_payment ? formatDate(sub.next_payment) : 'N/A'}</span>
						</div>
						<div class="subscription-detail">
							<span class="label">Total Paid</span>
							<span class="value">{formatCurrency(sub.total_paid)}</span>
						</div>
					</div>
					<div class="subscription-actions">
						<button class="btn-secondary small" onclick={() => onExtend(sub)}>
							<IconCalendar size={14} />
							Extend
						</button>
						<button class="btn-danger-outline small" onclick={() => onRevoke(sub.id)}>
							<IconTrash size={14} />
							Revoke
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.panel {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h4 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.subscriptions-list {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.subscription-card {
		background: rgba(15, 23, 42, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.subscription-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.subscription-product h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.subscription-plan {
		font-size: 0.75rem;
		color: #64748b;
	}

	.subscription-details {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.subscription-detail .label {
		display: block;
		font-size: 0.6875rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.subscription-detail .value {
		font-size: 0.875rem;
		color: #f1f5f9;
		font-weight: 500;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
		text-transform: capitalize;
	}

	.subscription-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
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

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.btn-primary.small,
	.btn-secondary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	.btn-danger-outline {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.4);
		border-radius: 8px;
		color: #f87171;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.6);
	}

	@media (max-width: 1023.98px) {
		.subscription-details {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
