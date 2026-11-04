<script lang="ts">
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import type { Campaign, CampaignDelivery } from '#lib/crm/types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	let { campaign, onSaved }: { campaign: Campaign; onSaved: () => Promise<void> } = $props();
	let rows = $state<CampaignDelivery[]>([]),
		page = $state(1),
		total = $state(0),
		pages = $state(0),
		filter = $state('');
	let loading = $state(false),
		busy = $state(false),
		error = $state(''),
		message = $state('');
	let scheduled = $state(''),
		confirmation = $state<'send' | 'schedule' | 'resume' | null>(null);
	async function load() {
		if (loading) return;
		loading = true;
		error = '';
		try {
			const result = await crmAPI.getCampaignDeliveries(campaign.id, {
				page,
				per_page: 25,
				status: filter || undefined
			});
			rows = result.data;
			total = result.meta.total;
			pages = result.meta.total_pages;
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load delivery receipts');
		} finally {
			loading = false;
		}
	}
	async function mutate(action: 'send' | 'schedule' | 'pause' | 'resume') {
		if (busy) return;
		busy = true;
		error = '';
		message = '';
		confirmation = null;
		try {
			if (action === 'send') await crmAPI.sendCampaignNow(campaign.id);
			else if (action === 'pause') await crmAPI.pauseCampaign(campaign.id);
			else if (action === 'resume') await crmAPI.resumeCampaign(campaign.id);
			else {
				const date = new Date(scheduled);
				if (!scheduled || !Number.isFinite(date.getTime()) || date.getTime() <= Date.now()) {
					error = 'Choose a future date and time.';
					return;
				}
				await crmAPI.scheduleCampaign(campaign.id, date.toISOString());
			}
			message =
				action === 'pause'
					? 'Dispatch paused. An email already in flight may still finish.'
					: 'Delivery queue saved. Provider acceptance and delivery appear in the receipts below.';
			page = 1;
			await onSaved();
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to update campaign delivery');
		} finally {
			busy = false;
		}
	}
	async function changePage(next: number) {
		page = next;
		await load();
	}
	function date(value: string | null) {
		return value ? new Date(value).toLocaleString() : 'Not recorded';
	}
	onMount(() => {
		void load();
		const timer = setInterval(() => {
			if (!busy && !loading && ['scheduled', 'sending'].includes(campaign.status)) {
				void (async () => {
					try {
						await onSaved();
						await load();
					} catch (cause) {
						error = apiErrorMessage(cause, 'Unable to refresh campaign');
					}
				})();
			}
		}, 5000);
		return () => clearInterval(timer);
	});
</script>

<section class="campaign-delivery" aria-labelledby="delivery-title">
	<h2 id="delivery-title">Campaign delivery</h2>
	<p>
		Submission freezes the confirmed audience and email content. Accepted means the provider
		acknowledged submission; delivered requires a provider delivery event. Recorded opens and clicks
		can include automated activity.
	</p>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if message}<p role="status">{message}</p>{/if}
	{#if campaign.pause_reason === 'provider_unavailable'}<p role="status">
			The email provider is unavailable. Queued work resumes when valid provider configuration is
			available.
		</p>{/if}
	{#if campaign.pause_reason === 'uncertain_submission'}<p role="alert">
			A submission outcome is unknown. Sending is paused until authenticated provider events
			reconcile the receipt. Unknown submissions are never automatically retried.
		</p>{/if}
	<div class="controls">
		{#if campaign.status === 'draft'}
			<button
				disabled={busy}
				onclick={() => {
					confirmation = 'send';
				}}>Send Now</button
			>
			<label for="campaign-schedule">Schedule in your local time</label><input
				id="campaign-schedule"
				type="datetime-local"
				bind:value={scheduled}
			/>
			<button
				disabled={busy || !scheduled}
				onclick={() => {
					confirmation = 'schedule';
				}}>Schedule Campaign</button
			>
		{:else if ['scheduled', 'sending'].includes(campaign.status)}
			<button disabled={busy} onclick={() => void mutate('pause')}>Pause Campaign</button>
		{:else if ['paused', 'failed'].includes(campaign.status) && campaign.pause_reason !== 'uncertain_submission'}
			<button
				disabled={busy}
				onclick={() => {
					confirmation = 'resume';
				}}>Resume Campaign</button
			>
		{/if}
		<button disabled={busy || loading} onclick={() => void load()}>Refresh Receipts</button>
	</div>
	<dl class="delivery-counts">
		{#each ['queued', 'processing', 'accepted', 'delivered', 'failed', 'unknown', 'skipped', 'unsubscribed'] as metric (metric)}<div
			>
				<dt>{metric}</dt>
				<dd>
					{campaign.delivery_stats?.[metric] ?? (campaign.status === 'draft' ? 0 : 'Not recorded')}
				</dd>
			</div>{/each}
	</dl>
	<label for="delivery-status">Delivery status</label><select
		id="delivery-status"
		bind:value={filter}
		onchange={() => void changePage(1)}
		><option value="">All statuses</option
		>{#each ['queued', 'processing', 'accepted', 'failed', 'unknown', 'skipped'] as status (status)}<option
				value={status}>{status}</option
			>{/each}</select
	>
	{#if loading}<p role="status">Loading delivery receipts…</p>{/if}
	{#if rows.length}<div class="table-scroll">
			<table>
				<thead
					><tr
						><th>Recipient</th><th>State</th><th>Attempts</th><th>Provider acceptance</th><th
							>Delivery</th
						><th>Details</th></tr
					></thead
				><tbody
					>{#each rows as receipt (receipt.id)}<tr
							><td>{receipt.display_name}<br />{receipt.email}</td><td>{receipt.status}</td><td
								>{receipt.attempt_count}</td
							><td>{date(receipt.accepted_at)}</td><td>{date(receipt.delivered_at)}</td><td
								>{receipt.last_error ?? receipt.provider_message_id ?? 'No provider receipt'}</td
							></tr
						>{/each}</tbody
				>
			</table>
		</div>{:else if !loading && !error}<p>
			No delivery receipts match this filter. Draft campaigns have no frozen recipients.
		</p>{/if}
	<p>{total} receipts · Page {page} of {Math.max(1, pages)}</p>
	<div class="controls">
		<button disabled={loading || page <= 1} onclick={() => void changePage(page - 1)}
			>Previous Receipts</button
		><button disabled={loading || page >= pages} onclick={() => void changePage(page + 1)}
			>Next Receipts</button
		>
	</div>
</section>
<ConfirmationModal
	isOpen={confirmation !== null}
	title={confirmation === 'schedule'
		? 'Schedule Campaign'
		: confirmation === 'resume'
			? 'Resume Campaign'
			: 'Send Campaign'}
	message={confirmation === 'resume'
		? 'Resume with the saved audience and content? Accepted recipients will not be sent again. Known failures may be retried; unknown outcomes require provider reconciliation.'
		: 'Submit this campaign to its confirmed audience? This can send real email when the provider is configured.'}
	confirmText={confirmation === 'schedule'
		? 'Schedule'
		: confirmation === 'resume'
			? 'Resume'
			: 'Send'}
	onConfirm={() => {
		if (confirmation) void mutate(confirmation);
	}}
	onCancel={() => {
		confirmation = null;
	}}
/>

<style>
	.campaign-delivery {
		padding: 1.5rem;
		margin: 1.5rem 0;
		border: 1px solid var(--border-color, #334155);
		border-radius: 0.5rem;
	}
	p {
		line-height: 1.6;
	}
	.controls {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		align-items: center;
		margin: 1rem 0;
	}
	button,
	input,
	select {
		padding: 0.65rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.375rem;
		background: var(--bg-secondary, #18181b);
		color: inherit;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	.delivery-counts {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
	}
	dt {
		text-transform: capitalize;
	}
	dd {
		margin: 0.25rem 0;
		font-size: 1.3rem;
	}
	.table-scroll {
		overflow-x: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
		margin-top: 1rem;
	}
	th,
	td {
		padding: 0.75rem;
		text-align: left;
		vertical-align: top;
		border-bottom: 1px solid var(--border-color, #334155);
		overflow-wrap: anywhere;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
