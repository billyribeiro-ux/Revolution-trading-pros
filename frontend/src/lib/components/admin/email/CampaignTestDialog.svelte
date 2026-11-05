<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getCampaign,
		getCampaignTestHistory,
		sendTestEmail,
		type Campaign
	} from '#lib/api/campaigns.js';
	import type { CampaignTestBatch } from '#lib/api/campaign-test.js';
	let { campaignId, onclose }: { campaignId: number; onclose: () => void } = $props();
	const uid = $props.id();
	let campaign = $state<Campaign | null>(null);
	let email = $state('');
	let variant = $state<'a' | 'b'>('a');
	let requestId = $state<string | null>(null);
	let requests = $state<CampaignTestBatch[]>([]);
	let busy = $state(false);
	let loading = $state(true);
	let error = $state('');
	let historyError = $state('');
	const labels = {
		accepted: 'Provider accepted',
		rejected: 'Provider rejected',
		unavailable: 'Provider unavailable',
		unknown: 'Acceptance unknown — reconcile before another test',
		processing: 'Submission in progress — check receipts'
	};
	function modal(node: HTMLDialogElement) {
		node.showModal();
		return () => node.close();
	}
	async function history() {
		historyError = '';
		try {
			requests = await getCampaignTestHistory(campaignId);
		} catch (failure) {
			historyError = failure instanceof Error ? failure.message : 'Test history is unavailable';
		}
	}
	async function load() {
		loading = true;
		error = '';
		try {
			campaign = await getCampaign(campaignId);
		} catch (failure) {
			error = failure instanceof Error ? failure.message : 'Campaign is unavailable';
		}
		await history();
		loading = false;
	}
	async function send() {
		if (busy || !campaign) return;
		busy = true;
		error = '';
		requestId ??= crypto.randomUUID();
		try {
			const receipt = await sendTestEmail(campaignId, email, {
				request_id: requestId,
				variant,
				expected_version: campaign.authoring_version
			});
			requests = [
				receipt,
				...requests.filter((item) => item.request_id !== receipt.request_id)
			].slice(0, 20);
		} catch (failure) {
			error =
				failure instanceof Error
					? failure.message
					: 'Test outcome is unconfirmed; check saved receipts';
		}
		await history();
		busy = false;
	}
	async function check() {
		if (busy) return;
		busy = true;
		await history();
		busy = false;
	}
	function another() {
		requestId = null;
		error = '';
	}
	onMount(() => {
		void load();
	});
</script>

<dialog
	{@attach modal}
	aria-labelledby="{uid}-title"
	oncancel={(event) => {
		event.preventDefault();
		if (!busy) onclose();
	}}
>
	<header>
		<h2 id="{uid}-title">Campaign test message</h2>
		<button type="button" disabled={busy} onclick={onclose} aria-label="Close test message"
			>×</button
		>
	</header>
	<p>
		Send saved campaign content to an explicit test recipient. Provider acceptance does not confirm
		inbox delivery. Campaign audiences and production metrics stay unchanged.
	</p>
	{#if loading}<p role="status">Loading saved campaign and test receipts…</p>
	{:else if campaign}
		<h3>{campaign.name}</h3>
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void send();
			}}
		>
			<label for="{uid}-email">Test recipient</label>
			<input
				id="{uid}-email"
				type="email"
				required
				maxlength="255"
				bind:value={email}
				disabled={busy || requestId !== null}
			/>
			{#if campaign.ab_test_config?.enabled}
				<label for="{uid}-variant">Subject variant</label>
				<select id="{uid}-variant" bind:value={variant} disabled={busy || requestId !== null}
					><option value="a">A — {campaign.subject}</option><option value="b"
						>B — {campaign.ab_test_config.subject_b}</option
					></select
				>
			{:else}<p>Subject: {campaign.subject}</p>{/if}
			{#if !requestId}<button type="submit" disabled={busy}
					>{busy ? 'Submitting test…' : 'Send test message'}</button
				>
			{:else}<p>
					Request: <code>{requestId}</code>. Repeating this request reads its existing outcome
					without sending again.
				</p>
				<div class="actions">
					<button type="submit" disabled={busy}>Retry same request</button><button
						type="button"
						disabled={busy}
						onclick={another}>Prepare another test</button
					>
				</div>
			{/if}
		</form>
	{/if}
	{#if error}<p role="alert">{error}</p>
		{#if !campaign}<button type="button" onclick={load}>Retry loading campaign</button>{/if}{/if}
	<section aria-label="Saved test receipts">
		<div class="actions">
			<h3>Saved test receipts</h3>
			<button type="button" disabled={busy || loading} onclick={check}>Check receipts</button>
		</div>
		{#if historyError}<p role="alert">
				{historyError}
			</p>{:else if !loading && requests.length === 0}<p>No recorded campaign tests.</p>{/if}
		{#each requests as request (request.request_id)}
			<article>
				<p>
					Variant {request.variant.toUpperCase()} · saved version {request.authoring_version} ·
					<time datetime={request.created_at}>{request.created_at}</time>
				</p>
				{#each request.submissions as receipt (receipt.id)}
					<p>{receipt.recipient}: <strong>{labels[receipt.status]}</strong> ({receipt.provider})</p>
					{#if receipt.provider_message_id}<p>
							Provider receipt: <code>{receipt.provider_message_id}</code>
						</p>{/if}
					{#if receipt.error}<p>{receipt.error}</p>{/if}
				{/each}
			</article>
		{/each}
	</section>
</dialog>

<style>
	dialog {
		color: #e2e8f0;
		background: #111827;
		border: 1px solid #475569;
		border-radius: 16px;
		padding: 24px;
		width: min(680px, calc(100vw - 32px));
		max-height: calc(100vh - 48px);
		overflow: auto;
	}
	dialog::backdrop {
		background: rgb(0 0 0 / 65%);
	}
	header,
	.actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		flex-wrap: wrap;
	}
	h2,
	h3 {
		margin: 0;
	}
	p {
		margin: 12px 0;
		overflow-wrap: anywhere;
	}
	form {
		display: grid;
		gap: 10px;
		margin: 20px 0;
	}
	input,
	select,
	button {
		font: inherit;
		color: inherit;
		background: #1e293b;
		border: 1px solid #64748b;
		border-radius: 8px;
		padding: 10px 12px;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.6;
		cursor: wait;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	article {
		border-top: 1px solid #334155;
		margin-top: 16px;
		padding-top: 8px;
	}
</style>
