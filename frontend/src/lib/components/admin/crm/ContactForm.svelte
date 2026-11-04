<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { apiClient } from '#lib/api/client.svelte.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id, kind = 'contacts' }: { id?: string; kind?: 'contacts' | 'leads' } = $props();
	type Fields = { email: string; first_name: string; last_name: string; phone: string; company: string; job_title: string; status: string };
	let fields = $state<Fields>({ email: '', first_name: '', last_name: '', phone: '', company: '', job_title: '', status: 'lead' });
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	const contactStatuses = ['lead', 'prospect', 'customer', 'churned', 'unqualified'];
	const leadStatuses = ['new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost', 'lead'];
	let statuses = $derived(kind === 'leads' ? leadStatuses : [...new Set([...contactStatuses, fields.status])]);
	let endpoint = $derived(`/admin/crm/${kind}${id ? `/${id}` : ''}`);
	let back = $derived(kind === 'leads' ? '/admin/crm/leads' : '/admin/crm');
	async function load() {
		if (!id) { loading = false; return; }
		loading = true; error = '';
		try {
			const record = await apiClient.get<Partial<Fields> & { company_name?: string }>(endpoint, { cache: { enabled: false } });
			fields = { email: record.email ?? '', first_name: record.first_name ?? '', last_name: record.last_name ?? '', phone: record.phone ?? '', company: record.company ?? record.company_name ?? '', job_title: record.job_title ?? '', status: record.status ?? (kind === 'leads' ? 'new' : 'lead') };
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to load contact'); }
		finally { loading = false; }
	}
	async function save(event: SubmitEvent) {
		event.preventDefault(); if (saving) return;
		saving = true; error = '';
		try { if (id) await apiClient.put(endpoint, fields); else await apiClient.post(endpoint, fields); await goto(back, { invalidateAll: true }); }
		catch (cause) { error = apiErrorMessage(cause, 'Unable to save contact'); }
		finally { saving = false; }
	}
	onMount(() => { void load(); });
</script>
<section>
	<a href={back}>← {kind === 'leads' ? 'Leads' : 'Contacts'}</a><h1>{id ? 'Edit' : 'New'} {kind === 'leads' ? 'Lead' : 'Contact'}</h1>
	{#if error}<p role="alert">{error}</p><button onclick={() => void load()}>Retry loading</button>{/if}
	{#if loading}<p role="status">Loading contact…</p>{:else}
	<form onsubmit={(event) => void save(event)}>
		<label for="contact-email">Email address</label><input id="contact-email" type="email" bind:value={fields.email} required maxlength={255} autocomplete="email" />
		<label for="contact-first">First name</label><input id="contact-first" bind:value={fields.first_name} maxlength={255} autocomplete="given-name" />
		<label for="contact-last">Last name</label><input id="contact-last" bind:value={fields.last_name} maxlength={255} autocomplete="family-name" />
		<label for="contact-phone">Phone</label><input id="contact-phone" type="tel" bind:value={fields.phone} maxlength={50} autocomplete="tel" />
		<label for="contact-company">Company</label><input id="contact-company" bind:value={fields.company} maxlength={255} autocomplete="organization" />
		<label for="contact-job">Job title</label><input id="contact-job" bind:value={fields.job_title} maxlength={255} autocomplete="organization-title" />
		<label for="contact-status">Status</label><select id="contact-status" bind:value={fields.status}>{#each statuses as status (status)}<option value={status}>{status}</option>{/each}</select>
		<div><button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Contact'}</button> <a href={back}>Cancel</a></div>
	</form>{/if}
</section>
<style>section { max-width: 56rem; padding: 2rem; color: #e2e8f0; } form { display: grid; gap: 0.75rem; } input, select, button { padding: 0.75rem; border: 1px solid #475569; border-radius: 0.5rem; background: #0f172a; color: inherit; } a { color: #7dd3fc; } [role='alert'] { color: #fca5a5; }</style>
