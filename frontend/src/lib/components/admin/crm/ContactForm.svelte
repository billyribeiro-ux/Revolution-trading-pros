<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiClient } from '#lib/api/client.svelte.js';
	import { contactId, contactFormWrite, contactRecord } from '#lib/api/crm-contact-contract.js';
	import {
		contactStatuses,
		lifecycleStages,
		type ContactFormFields
	} from '#lib/crm/contact-types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id, kind = 'contacts' }: { id?: string; kind?: 'contacts' | 'leads' } = $props();
	type Fields = ContactFormFields;
	let fields = $state<Fields>({
		email: '',
		first_name: '',
		last_name: '',
		phone: '',
		company: '',
		job_title: '',
		status: 'lead',
		lifecycle_stage: ''
	});
	let loading = $state(true),
		loaded = $state(false),
		saving = $state(false),
		error = $state('');
	let disposed = false;
	let initial: Fields | null = null;
	const leadStatuses = [
		'new',
		'contacted',
		'qualified',
		'proposal',
		'negotiation',
		'won',
		'lost',
		'lead'
	];
	let statuses = $derived([
		...new Set([
			...(kind === 'leads' ? leadStatuses : contactStatuses),
			...(fields.status ? [fields.status] : [])
		])
	]);
	let back = $derived(kind === 'leads' ? '/admin/crm/leads' : '/admin/crm');
	async function load() {
		if (!id) {
			loading = false;
			loaded = true;
			return;
		}
		loading = true;
		loaded = false;
		error = '';
		try {
			const record =
				kind === 'contacts'
					? await crmAPI.getContact(id)
					: contactRecord(
							await apiClient.get<unknown>(`/admin/crm/leads/${contactId(id)}`, {
								cache: { enabled: false }
							}),
							id,
							true
						);
			if (contactId(record.id) !== contactId(id))
				throw new Error('The service returned a different contact');
			if (disposed) return;
			fields = {
				email: record.email ?? '',
				first_name: record.first_name ?? '',
				last_name: record.last_name ?? '',
				phone: record.phone ?? '',
				company: record.company ?? '',
				job_title: record.job_title ?? '',
				status: record.status ?? '',
				lifecycle_stage: record.lifecycle_stage ?? ''
			};
			initial = { ...fields };
			loaded = true;
		} catch (cause) {
			if (!disposed) error = apiErrorMessage(cause, 'Unable to load contact');
		} finally {
			if (!disposed) loading = false;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving || !loaded) return;
		saving = true;
		error = '';
		try {
			const payload = contactFormWrite(fields, initial);
			const config = { retry: { attempts: 1, condition: () => false } },
				path = `/admin/crm/leads${id ? `/${contactId(id)}` : ''}`;
			const saved =
				kind === 'contacts'
					? id
						? await crmAPI.updateContact(id, payload)
						: await crmAPI.createContact(payload)
					: contactRecord(
							id
								? await apiClient.put<unknown>(path, payload, config)
								: await apiClient.post<unknown>(path, payload, config),
							id
						);
			for (const key of [
				'email',
				'first_name',
				'last_name',
				'phone',
				'company',
				'job_title',
				'status',
				'lifecycle_stage'
			] as const)
				if (payload[key] !== undefined && saved[key] !== payload[key])
					throw new Error(`The service did not confirm the saved ${key.replaceAll('_', ' ')}`);
			const savedId = saved.id;
			if (!disposed) await goto(`/admin/crm/${kind}/${savedId}`, { refreshAll: true });
		} catch (cause) {
			if (!disposed) error = apiErrorMessage(cause, 'Unable to save contact');
		} finally {
			if (!disposed) saving = false;
		}
	}
	onMount(() => {
		void load();
		return () => {
			disposed = true;
		};
	});
</script>

<section>
	<a href={back}>← {kind === 'leads' ? 'Leads' : 'Contacts'}</a>
	<h1>{id ? 'Edit' : 'New'} {kind === 'leads' ? 'Lead' : 'Contact'}</h1>
	{#if error}<p role="alert">{error}</p>
		{#if !loaded}<button onclick={() => void load()} disabled={loading}>Retry loading</button
			>{/if}{/if}
	{#if loading}<p role="status">Loading contact…</p>{:else if loaded}
		<form onsubmit={(event) => void save(event)}>
			<label for="contact-email">Email address</label><input
				id="contact-email"
				type="email"
				bind:value={fields.email}
				required
				maxlength={255}
				autocomplete="email"
				disabled={saving}
			/>
			<label for="contact-first">First name</label><input
				id="contact-first"
				bind:value={fields.first_name}
				maxlength={255}
				autocomplete="given-name"
				disabled={saving}
			/>
			<label for="contact-last">Last name</label><input
				id="contact-last"
				bind:value={fields.last_name}
				maxlength={255}
				autocomplete="family-name"
				disabled={saving}
			/>
			<label for="contact-phone">Phone</label><input
				id="contact-phone"
				type="tel"
				bind:value={fields.phone}
				maxlength={50}
				autocomplete="tel"
				disabled={saving}
			/>
			<label for="contact-company">Company</label><input
				id="contact-company"
				bind:value={fields.company}
				maxlength={255}
				autocomplete="organization"
				disabled={saving}
			/>
			<label for="contact-job">Job title</label><input
				id="contact-job"
				bind:value={fields.job_title}
				maxlength={255}
				autocomplete="organization-title"
				disabled={saving}
			/>
			<label for="contact-status">Status</label><select
				id="contact-status"
				bind:value={fields.status}
				disabled={saving}
				><option value="" disabled>Unknown (preserve current status)</option
				>{#each statuses as status (status)}<option value={status}>{status}</option>{/each}</select
			>
			<label for="contact-lifecycle">Lifecycle stage</label><select
				id="contact-lifecycle"
				bind:value={fields.lifecycle_stage}
				disabled={saving}
				><option value="">Unknown</option>{#each lifecycleStages as stage (stage)}<option
						value={stage}>{stage}</option
					>{/each}</select
			>
			<div>
				<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Contact'}</button>
				<a href={back}>Cancel</a>
			</div>
		</form>{/if}
</section>

<style>
	section {
		max-width: 56rem;
		padding: 2rem;
		color: #e2e8f0;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	input,
	select,
	button {
		padding: 0.75rem;
		border: 1px solid #475569;
		border-radius: 0.5rem;
		background: #0f172a;
		color: inherit;
	}
	a {
		color: #7dd3fc;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
