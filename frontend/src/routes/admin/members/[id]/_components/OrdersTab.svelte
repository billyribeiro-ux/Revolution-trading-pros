<script lang="ts">
	import { IconReceipt, IconDownload, IconExternalLink } from '#lib/icons/index.js';
	import type { MemberProfile, MemberOrderRecord } from '#lib/api/member-profile.js';
	import { recordedWallClock, recordedUnits } from './profile-display.js';
	import HistoryControls from './HistoryControls.svelte';
	let {
		result,
		loading,
		error,
		busy = false,
		onPage,
		onPerPage,
		onRefresh,
		onView,
		onExport
	}: {
		result: MemberProfile['orders'] | null;
		loading: boolean;
		error: string;
		busy?: boolean;
		onPage: (page: number) => void;
		onPerPage: (size: number) => void;
		onRefresh: () => void;
		onView: (order: MemberOrderRecord) => void;
		onExport: () => void;
	} = $props();
</script>

<section class="panel" aria-label="Recorded order history">
	<div class="panel-header">
		<h3>Order History</h3>
		<button onclick={onExport} disabled={busy || loading || !result || !!error}
			><IconDownload size={16} />Export</button
		>
	</div>
	<div class="content">
		<p class="hint">
			Recorded order and refund amounts retain their own currency. Net recorded value does not
			establish provider settlement or lifetime value.
		</p>
		{#if result && !loading && !error}{#if result.orders.length === 0}<div class="empty-state">
					<IconReceipt size={48} />
					<h4>No recorded orders on this page</h4>
				</div>{:else}<div class="table-scroll">
					<table>
						<thead
							><tr
								><th>Order</th><th>Recorded date</th><th>Status</th><th>Recorded total</th><th
									>Recorded refund</th
								><th>Net recorded amount</th><th>Actions</th></tr
							></thead
						><tbody
							>{#each result.orders as order (order.id)}<tr
									><td>{order.order_number ?? `Order #${order.id}`}</td><td
										>{recordedWallClock(order.created_at)}</td
									><td>{order.status ?? 'Not recorded'}</td><td
										>{recordedUnits(order.total_units, order.currency)}</td
									><td>{recordedUnits(order.refund_units, order.currency)}</td><td
										>{recordedUnits(order.net_recorded_units, order.currency)}</td
									><td
										><button
											aria-label={`View order ${order.order_number ?? order.id}`}
											onclick={() => onView(order)}
											disabled={busy}><IconExternalLink size={16} /></button
										></td
									></tr
								>{/each}</tbody
						>
					</table>
				</div>{/if}{/if}
	</div>
	<HistoryControls
		label="orders"
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
	h4 {
		color: var(--text-primary, #f0f6fc);
	}
	.table-scroll {
		overflow-x: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		padding: 1rem;
		text-align: left;
		border-bottom: 1px solid var(--border-muted, #21262d);
		vertical-align: top;
		overflow-wrap: anywhere;
	}
	th {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}
</style>
