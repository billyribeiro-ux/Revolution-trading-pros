<script lang="ts">
	import { IconCreditCard, IconCalendar, IconTrash, IconGift } from '#lib/icons/index.js';
	import type { Subscription } from '#lib/api/members.js';
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
							class={[
								'status-badge',
								sub.status === 'active' ? 'status-active' : 'status-inactive'
							]}
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}

	.empty-state h4 {
		color: var(--text-primary, #f0f6fc);
		margin: 1rem 0 0.5rem;
	}

	.subscriptions-list {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.subscription-card {
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-muted, #21262d);
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
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.subscription-plan {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.subscription-details {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.subscription-detail .label {
		display: block;
		font-size: 0.6875rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.subscription-detail .value {
		font-size: 0.875rem;
		color: var(--text-primary, #f0f6fc);
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

	.status-active {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
		border-color: rgba(63, 185, 80, 0.35);
	}

	.status-inactive {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border-color: var(--border-default, #30363d);
	}

	.subscription-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid var(--border-muted, #21262d);
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

	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
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
		border: 1px solid rgba(248, 81, 73, 0.4);
		border-radius: 8px;
		color: var(--error-emphasis, #f85149);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline:hover {
		background: rgba(248, 81, 73, 0.1);
		border-color: rgba(248, 81, 73, 0.6);
	}

	@media (max-width: 1023.98px) {
		.subscription-details {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
