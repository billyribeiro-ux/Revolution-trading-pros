<script lang="ts">
	import { IconCreditCard, IconCalendar, IconTrash, IconGift } from '#lib/icons/index.js';
	import type { MemberProfile, MemberMembership } from '#lib/api/member-profile.js';
	import { recordedWallClock, recordedInstant } from './profile-display.js';
	import HistoryControls from './HistoryControls.svelte';
	let {
		result,
		loading,
		error,
		busy = false,
		onPage,
		onPerPage,
		onRefresh,
		onGrant,
		onExtend,
		onRevoke
	}: {
		result: MemberProfile['memberships'] | null;
		loading: boolean;
		error: string;
		busy?: boolean;
		onPage: (page: number) => void;
		onPerPage: (size: number) => void;
		onRefresh: () => void;
		onGrant: () => void;
		onExtend: (membership: MemberMembership) => void;
		onRevoke: (id: string) => void;
	} = $props();
</script>

<section class="panel" aria-label="Membership history">
	<div class="panel-header">
		<h3>Subscription History</h3>
		<button onclick={onGrant} disabled={busy}><IconGift size={16} />Grant Membership</button>
	</div>
	<div class="content">
		<p class="hint">
			Membership and entitlement records are separate from account restrictions. Catalogue prices
			are not charged amounts; expiry dates are not promised next-payment dates.
		</p>
		{#if result && !loading && !error}{#if result.memberships.length === 0}<div class="empty-state">
					<IconCreditCard size={48} />
					<h4>No recorded memberships on this page</h4>
				</div>{:else}<div class="rows">
					{#each result.memberships as sub (sub.id)}<article class="card">
							<div class="card-header">
								<div>
									<h4>{sub.plan_name ?? `Plan #${sub.plan_id} (name not recorded)`}</h4>
									<span>Membership #{sub.id}</span>
								</div>
								<span class="status">Recorded status: {sub.status ?? 'Not recorded'}</span>
							</div>
							<dl>
								<div>
									<dt>Catalogue price</dt>
									<dd>
										{sub.catalog_price_units === null
											? 'Not recorded'
											: `${sub.catalog_price_units} units (currency not recorded)`}
									</dd>
								</div>
								<div>
									<dt>Billing cycle</dt>
									<dd>{sub.billing_cycle ?? 'Not recorded'}</dd>
								</div>
								<div>
									<dt>Started</dt>
									<dd>{recordedWallClock(sub.starts_at)}</dd>
								</div>
								<div>
									<dt>Entitlement expiry</dt>
									<dd>{recordedWallClock(sub.expires_at)}</dd>
								</div>
								<div>
									<dt>Trial end</dt>
									<dd>{recordedWallClock(sub.trial_ends_at)}</dd>
								</div>
								<div>
									<dt>Current period end</dt>
									<dd>{recordedWallClock(sub.current_period_end)}</dd>
								</div>
								<div>
									<dt>Cancellation</dt>
									<dd>{recordedWallClock(sub.cancelled_at)}</dd>
								</div>
								<div>
									<dt>Next payment</dt>
									<dd>Not measured</dd>
								</div>
								<div>
									<dt>Total paid</dt>
									<dd>Not measured</dd>
								</div>
							</dl>
							<p class="hint">{sub.unmeasured_reason}</p>
							{#if sub.deleted_at}<p class="hint">
									Deleted source record: {recordedInstant(sub.deleted_at)}
								</p>{/if}
							<div class="actions">
								<button onclick={() => onExtend(sub)} disabled={busy || sub.deleted_at !== null}
									><IconCalendar size={14} />Extend</button
								><button onclick={() => onRevoke(sub.id)} disabled={busy || sub.deleted_at !== null}
									><IconTrash size={14} />Revoke</button
								>
							</div>
						</article>{/each}
				</div>{/if}{/if}
	</div>
	<HistoryControls
		label="memberships"
		pagination={result?.pagination ?? null}
		{loading}
		{error}
		{busy}
		{onPage}
		{onPerPage}
		{onRefresh}
	/>
</section>

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
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}
	h3 {
		font-size: 1rem;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
	}
	button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.6rem 0.9rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 9px;
		cursor: pointer;
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	.content {
		padding: 1.25rem 1.5rem;
	}
	.hint,
	.empty-state {
		color: var(--text-secondary, #8b949e);
		line-height: 1.6;
	}
	.empty-state {
		padding: 2rem;
		text-align: center;
	}
	.rows {
		display: grid;
		gap: 1rem;
	}
	.card {
		padding: 1rem;
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 12px;
		background: var(--bg-base, #0d1117);
	}
	.card-header {
		display: flex;
		justify-content: space-between;
		gap: 0.75rem;
		align-items: center;
		flex-wrap: wrap;
	}
	h4 {
		margin: 0 0 0.5rem;
		color: var(--text-primary, #f0f6fc);
	}
	dl {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}
	dt {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}
	dd {
		margin: 0.35rem 0 0;
		overflow-wrap: anywhere;
	}
	.status {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 99px;
		padding: 0.25rem 0.65rem;
		font-size: 0.8rem;
	}
	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 1rem;
	}
</style>
