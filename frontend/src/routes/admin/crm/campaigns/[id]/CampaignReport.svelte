<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import { sanitizeHtml } from '#lib/utils/sanitize.js';
	import type { Campaign } from '#lib/crm/types.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import CampaignAudience from '#lib/components/admin/crm/CampaignAudience.svelte';
	import CampaignDelivery from '#lib/components/admin/crm/CampaignDelivery.svelte';
	let { id }: { id: string } = $props();
	let campaign = $state<Campaign | null>(null),
		loading = $state(true),
		busy = $state(false),
		error = $state(''),
		confirmDelete = $state(false);
	const preview = $derived(sanitizeHtml(campaign?.email_body));
	async function load() {
		loading = true;
		error = '';
		try {
			campaign = await crmAPI.getCampaign(id);
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load campaign report');
		} finally {
			loading = false;
		}
	}
	async function duplicate() {
		if (busy) return;
		busy = true;
		error = '';
		try {
			const copy = await crmAPI.duplicateCampaign(id);
			await goto(`/admin/crm/campaigns/${copy.id}/edit`);
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to duplicate campaign');
		} finally {
			busy = false;
		}
	}
	async function remove() {
		if (busy) return;
		busy = true;
		error = '';
		confirmDelete = false;
		try {
			await crmAPI.deleteCampaign(id);
			await goto('/admin/crm/campaigns');
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to delete campaign');
		} finally {
			busy = false;
		}
	}
	function date(value: string | undefined) {
		return value ? new Date(value).toLocaleString() : 'Not recorded';
	}
	onMount(() => {
		void load();
	});
</script>

<section class="campaign-report">
	<a href="/admin/crm/campaigns">← Campaigns</a>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if loading && !campaign}<p role="status">Loading campaign report…</p>{:else if campaign}
		<header>
			<div>
				<h1>{campaign.title}</h1>
				<p>Status: <strong>{campaign.status}</strong></p>
			</div>
			<div class="actions">
				<button onclick={() => void load()} disabled={loading}>Refresh</button><a
					href={`/admin/crm/campaigns/${id}/edit`}>Edit Campaign</a
				><button onclick={() => void duplicate()} disabled={busy}>Duplicate Campaign</button><button
					onclick={() => {
						confirmDelete = true;
					}}
					disabled={busy}>Delete Campaign</button
				>
			</div>
		</header>
		{#if campaign.last_error}<p role="alert">{campaign.last_error}</p>{/if}
		<dl class="metrics">
			<div>
				<dt>Snapshot recipients</dt>
				<dd>{campaign.recipients_count}</dd>
			</div>
			<div>
				<dt>Provider accepted</dt>
				<dd>{campaign.sent_count}</dd>
			</div>
			<div>
				<dt>Failed</dt>
				<dd>{campaign.failed_count ?? 'Unknown'}</dd>
			</div>
			<div>
				<dt>Recorded opens</dt>
				<dd>{campaign.open_count}</dd>
			</div>
			<div>
				<dt>Recorded clicks</dt>
				<dd>{campaign.click_count}</dd>
			</div>
		</dl>
		<CampaignDelivery {campaign} onSaved={load} />
		<dl>
			<dt>Subject</dt>
			<dd>{campaign.email_subject || 'No subject'}</dd>
			<dt>Preheader</dt>
			<dd>{campaign.email_pre_header || 'None'}</dd>
			<dt>Scheduled</dt>
			<dd>{date(campaign.scheduled_at)}</dd>
			<dt>Sent at</dt>
			<dd>{date(campaign.sent_at)}</dd>
			<dt>Created</dt>
			<dd>{date(campaign.created_at)}</dd>
			<dt>Updated</dt>
			<dd>{date(campaign.updated_at)}</dd>
		</dl>
		{#key campaign.updated_at}<CampaignAudience
				{id}
				settings={campaign.subscriber_settings}
				editable={['draft', 'paused', 'failed'].includes(campaign.status)}
				onSaved={load}
			/>{/key}

		<h2>Email preview</h2>
		{#if campaign.type === 'plain_text'}<pre>{campaign.email_body}</pre>{:else}<iframe
				title="Campaign email preview"
				sandbox=""
				srcdoc={preview}
			></iframe>{/if}
	{:else}<button onclick={() => void load()}>Retry loading</button>{/if}
</section>
<ConfirmationModal
	isOpen={confirmDelete}
	title="Delete Campaign"
	message={`Delete "${campaign?.title ?? ''}"? This cannot be undone.`}
	confirmText="Delete"
	variant="danger"
	onConfirm={() => void remove()}
	onCancel={() => {
		confirmDelete = false;
	}}
/>

<style>
	.campaign-report {
		max-width: 72rem;
		margin: auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		font-size: 1.75rem;
		margin: 1rem 0;
	}
	h2 {
		font-size: 1.25rem;
		margin: 1rem 0;
	}
	header,
	.actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		flex-wrap: wrap;
	}
	.metrics {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));
		gap: 1rem;
	}
	.metrics div {
		padding: 1rem;
		background: var(--bg-secondary, #18181b);
		border-radius: 0.5rem;
	}
	dt {
		font-weight: 600;
		margin-top: 0.5rem;
	}
	dd {
		margin: 0.25rem 0 1rem;
		overflow-wrap: anywhere;
	}
	.metrics dd {
		font-size: 1.5rem;
		margin: 0;
	}
	button,
	.actions a {
		padding: 0.65rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
		background: var(--bg-secondary, #18181b);
		color: inherit;
		cursor: pointer;
	}
	iframe {
		width: 100%;
		min-height: 30rem;
		background: white;
		border: 0;
	}
	pre {
		white-space: pre-wrap;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
