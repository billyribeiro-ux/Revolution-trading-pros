<script lang="ts">
	import type { ContactRecord } from '#lib/crm/contact-types.js';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { DealPriority, Pipeline, CrmCompany, DealWrite } from '#lib/crm/types.js';

	let { id }: { id?: string } = $props();
	let loading = $state(true);
	let loaded = $state(false);
	let saving = $state(false);
	let error = $state('');
	let name = $state('');
	let amount = $state('0.00');
	let currency = $state('USD');
	let pipelineId = $state('');
	let stageId = $state('');
	let probability = $state<number | undefined>(50);
	let priority = $state<DealPriority>('medium');
	let expectedCloseDate = $state('');
	let sourceChannel = $state('');
	let sourceCampaign = $state('');
	let tags = $state('');
	let customFields = $state('{}');
	let lostReason = $state('');
	let pipelines = $state<Pipeline[]>([]);
	let contactId = $state('');
	let contactLabel = $state('');
	let companyId = $state('');
	let companyLabel = $state('');
	let contactSearch = $state('');
	let companySearch = $state('');
	let contacts = $state<ContactRecord[]>([]);
	let companies = $state<CrmCompany[]>([]);
	let searching = $state<'contact' | 'company' | null>(null);
	let searchError = $state('');
	const stages = $derived(pipelines.find((p) => p.id === pipelineId)?.stages ?? []);
	const targetStage = $derived(stages.find((s) => s.id === stageId));

	function contactName(contact: ContactRecord): string {
		return (
			contact.full_name ||
			[contact.first_name, contact.last_name].filter(Boolean).join(' ') ||
			contact.email
		);
	}
	async function load() {
		loading = true;
		loaded = false;
		error = '';
		try {
			const [available, deal] = await Promise.all([
				crmAPI.getPipelines(),
				id ? crmAPI.getDeal(id) : Promise.resolve(null)
			]);
			pipelines = available;
			if (deal) {
				name = deal.name;
				amount = `${Math.floor(deal.amount_cents / 100)}.${String(deal.amount_cents % 100).padStart(2, '0')}`;
				currency = deal.currency;
				pipelineId = deal.pipeline_id;
				stageId = deal.stage_id;
				probability = deal.probability;
				priority = deal.priority;
				expectedCloseDate = deal.expected_close_date ?? '';
				sourceChannel = deal.source_channel ?? '';
				sourceCampaign = deal.source_campaign ?? '';
				tags = (deal.tags ?? []).join(', ');
				customFields = JSON.stringify(deal.custom_fields ?? {}, null, 2);
				contactId = deal.contact_id ?? '';
				contactLabel = deal.contact?.full_name ?? deal.contact?.email ?? contactId;
				companyId = deal.company_id ?? '';
				companyLabel = deal.company?.name ?? companyId;
				lostReason = deal.lost_reason ?? '';
			} else {
				pipelineId = (available.find((p) => p.is_default) ?? available[0])?.id ?? '';
				choosePipeline();
			}
			loaded = true;
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load deal');
		} finally {
			loading = false;
		}
	}
	function choosePipeline() {
		const stage = pipelines
			.find((p) => p.id === pipelineId)
			?.stages?.find((s) => !s.is_closed_won && !s.is_closed_lost);
		stageId = stage?.id ?? '';
		probability = stage?.probability ?? 50;
	}
	async function search(kind: 'contact' | 'company') {
		if (searching) return;
		searching = kind;
		searchError = '';
		try {
			if (kind === 'contact')
				contacts = (await crmAPI.getContacts({ search: contactSearch, per_page: 25 })).data;
			else companies = (await crmAPI.getCompanies({ search: companySearch, per_page: 25 })).data;
		} catch (cause) {
			searchError = apiErrorMessage(cause, 'Unable to search records');
		} finally {
			searching = null;
		}
	}
	function numericId(value: string): number {
		const parsed = Number(value);
		if (!Number.isSafeInteger(parsed) || parsed < 1)
			throw new Error('The selected record ID is outside the supported range.');
		return parsed;
	}

	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving || !loaded) return;
		saving = true;
		error = '';
		try {
			const match = /^(\d{1,13})(?:\.(\d{1,2}))?$/.exec(amount.trim());
			if (!match) throw new Error('Enter a nonnegative amount with at most two decimal places.');
			const cents = Number(match[1]) * 100 + Number((match[2] ?? '').padEnd(2, '0'));
			if (!Number.isSafeInteger(cents) || cents > 999999999999999)
				throw new Error('Amount exceeds the supported range.');
			const custom: unknown = JSON.parse(customFields);
			if (!custom || typeof custom !== 'object' || Array.isArray(custom))
				throw new Error('Custom fields must be a JSON object.');
			const fields: DealWrite = {
				name: name.trim(),
				amount_cents: cents,
				currency,
				pipeline_id: numericId(pipelineId),
				stage_id: numericId(stageId),
				probability,
				priority,
				expected_close_date: expectedCloseDate || null,
				contact_id: contactId ? numericId(contactId) : null,
				company_id: companyId ? numericId(companyId) : null,
				source_channel: sourceChannel || null,
				source_campaign: sourceCampaign || null,
				tags: tags
					.split(',')
					.map((t) => t.trim())
					.filter(Boolean),
				custom_fields: custom as Record<string, unknown>,
				...(targetStage?.is_closed_lost ? { lost_reason: lostReason } : {})
			};
			const saved = id ? await crmAPI.updateDeal(id, fields) : await crmAPI.createDeal(fields);
			await goto(`/admin/crm/deals/${saved.id}`, { invalidateAll: true });
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to save deal');
		} finally {
			saving = false;
		}
	}
	onMount(() => {
		void load();
	});
</script>

<section class="deal-editor">
	<a href="/admin/crm/deals">← Deals</a>
	<h1>{id ? 'Edit Deal' : 'New Deal'}</h1>
	{#if error}<div role="alert">{error}</div>{/if}
	{#if loading}<p role="status">Loading deal…</p>
	{:else if !loaded}<button type="button" onclick={() => void load()}>Retry loading</button>
	{:else}
		<form onsubmit={(event) => void save(event)}>
			<label for="deal-name">Deal name</label><input
				id="deal-name"
				bind:value={name}
				required
				maxlength={255}
			/>
			<div class="columns">
				<div>
					<label for="deal-amount">Amount</label><input
						id="deal-amount"
						inputmode="decimal"
						bind:value={amount}
						required
					/>
				</div>
				<div>
					<label for="deal-currency">Currency</label><input
						id="deal-currency"
						bind:value={currency}
						required
						pattern={'[A-Z]{3}'}
						minlength={3}
						maxlength={3}
					/>
				</div>
			</div>
			<label for="deal-pipeline">Pipeline</label><select
				id="deal-pipeline"
				bind:value={pipelineId}
				onchange={choosePipeline}
				required
				><option value="">Select pipeline</option>{#each pipelines as pipeline (pipeline.id)}<option
						value={pipeline.id}>{pipeline.name}</option
					>{/each}</select
			>
			<label for="deal-stage">Stage</label><select
				id="deal-stage"
				bind:value={stageId}
				onchange={() => {
					probability = targetStage?.probability ?? 50;
				}}
				required
				><option value="">Select stage</option
				>{#each stages.filter((s) => id || (!s.is_closed_won && !s.is_closed_lost)) as stage (stage.id)}<option
						value={stage.id}>{stage.name}</option
					>{/each}</select
			>
			{#if !pipelines.length}<p>
					No active pipelines. A pipeline with an open stage is required before saving a deal.
				</p>{/if}
			{#if targetStage?.is_closed_lost}<label for="deal-lost-reason">Lost reason</label><textarea
					id="deal-lost-reason"
					bind:value={lostReason}
					maxlength={10000}
					required></textarea>{/if}
			<div class="columns">
				<div>
					<label for="deal-probability">Probability (%)</label><input
						id="deal-probability"
						type="number"
						min={0}
						max={100}
						step={1}
						required
						bind:value={probability}
						disabled={targetStage?.is_closed_won || targetStage?.is_closed_lost}
					/>
				</div>
				<div>
					<label for="deal-priority">Priority</label><select
						id="deal-priority"
						bind:value={priority}
						>{#each ['low', 'medium', 'high', 'urgent'] as option (option)}<option value={option}
								>{option}</option
							>{/each}</select
					>
				</div>
			</div>
			<label for="deal-close">Expected close date</label><input
				id="deal-close"
				type="date"
				bind:value={expectedCloseDate}
			/>
			<fieldset>
				<legend>Contact</legend>
				<p>{contactId ? contactLabel : 'No linked contact'}</p>
				{#if contactId}<button
						type="button"
						onclick={() => {
							contactId = '';
							contactLabel = '';
						}}>Clear contact</button
					>{/if}
				<label for="deal-contact-search">Find contact</label>
				<div class="search">
					<input id="deal-contact-search" bind:value={contactSearch} /><button
						type="button"
						disabled={searching !== null}
						onclick={() => void search('contact')}>Search contacts</button
					>
				</div>
				{#if searching === 'contact'}<p role="status">Searching…</p>{/if}
				<ul>
					{#each contacts as contact (contact.id)}<li>
							<button
								type="button"
								onclick={() => {
									contactId = contact.id;
									contactLabel = contactName(contact);
									contacts = [];
								}}>{contactName(contact)} · {contact.email}</button
							>
						</li>{/each}
				</ul>
				<small
					>Search returns up to 25 matches. Refine the name or email to find another contact.</small
				>
			</fieldset>
			<fieldset>
				<legend>Company</legend>
				<p>{companyId ? companyLabel : 'No linked company'}</p>
				{#if companyId}<button
						type="button"
						onclick={() => {
							companyId = '';
							companyLabel = '';
						}}>Clear company</button
					>{/if}
				<label for="deal-company-search">Find company</label>
				<div class="search">
					<input id="deal-company-search" bind:value={companySearch} /><button
						type="button"
						disabled={searching !== null}
						onclick={() => void search('company')}>Search companies</button
					>
				</div>
				{#if searching === 'company'}<p role="status">Searching…</p>{/if}
				<ul>
					{#each companies as company (company.id)}<li>
							<button
								type="button"
								onclick={() => {
									companyId = company.id;
									companyLabel = company.name;
									companies = [];
								}}>{company.name}</button
							>
						</li>{/each}
				</ul>
				<small
					>Search returns up to 25 matches. Refine the company name to find another record.</small
				>
			</fieldset>
			{#if searchError}<p role="alert">{searchError}</p>{/if}
			<label for="deal-source">Source channel</label><input
				id="deal-source"
				bind:value={sourceChannel}
				maxlength={100}
			/>
			<label for="deal-campaign">Source campaign</label><input
				id="deal-campaign"
				bind:value={sourceCampaign}
				maxlength={255}
			/>
			<label for="deal-tags">Tags, separated by commas</label><input
				id="deal-tags"
				bind:value={tags}
			/>
			<label for="deal-custom">Custom fields (JSON object)</label><textarea
				id="deal-custom"
				bind:value={customFields}
				rows={5}
				maxlength={65536}></textarea>
			<div class="actions">
				<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Deal'}</button><a
					href={id ? `/admin/crm/deals/${id}` : '/admin/crm/deals'}>Cancel</a
				>
			</div>
		</form>
	{/if}
</section>

<style>
	.deal-editor {
		max-width: 52rem;
		margin: 0 auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		font-size: 1.75rem;
		margin: 1rem 0 1.5rem;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	label {
		display: block;
		font-weight: 600;
		margin-top: 0.5rem;
	}
	input,
	select,
	textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid var(--border-color, #52525b);
		border-radius: 0.5rem;
		background: var(--bg-secondary, #18181b);
		color: inherit;
	}
	button {
		padding: 0.65rem 1rem;
		border-radius: 0.5rem;
		background: var(--bg-tertiary, #27272a);
		color: inherit;
		border: 1px solid var(--border-color, #52525b);
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.6;
		cursor: wait;
	}
	[role='alert'] {
		padding: 1rem;
		background: #7f1d1d;
		color: #fff;
		border-radius: 0.5rem;
	}
	.columns {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}
	.search,
	.actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.search input {
		flex: 1;
		min-width: 0;
	}
	.search button {
		flex-shrink: 0;
	}
	fieldset {
		padding: 1rem;
		border: 1px solid var(--border-color, #52525b);
		border-radius: 0.5rem;
	}
	ul {
		list-style: none;
		padding: 0;
	}
	li {
		margin: 0.4rem 0;
	}
	small {
		color: var(--text-secondary);
	}
	@media (max-width: 600px) {
		.columns {
			grid-template-columns: 1fr;
		}
		.search {
			flex-direction: column;
			align-items: stretch;
		}
		.deal-editor {
			padding: 1rem;
		}
	}
</style>
