<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { apiClient } from '#lib/api/client.svelte.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	type Segment = { id: string | number; title: string; contacts_count: number; match_type: string; is_active: boolean };
	type Contact = { id: string | number; email: string; first_name?: string; last_name?: string; status: string };
	let segment = $state<Segment | null>(null);
	let contacts = $state<Contact[]>([]);
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	let loading = $state(true);
	let error = $state('');
	let id = $derived(page.params.id);
	async function load() {
		loading = true; error = '';
		try {
			const [record, members] = await Promise.all([
				apiClient.get<Segment>(`/admin/crm/segments/${id}`, { cache: { enabled: false } }),
				apiClient.get<{ data: Contact[]; meta: { total: number; total_pages: number } }>(`/admin/crm/segments/${id}/contacts`, { cache: { enabled: false }, preserveEnvelope: true, params: { page: currentPage, per_page: 25 } })
			]);
			segment = record; contacts = members.data; total = members.meta.total; totalPages = members.meta.total_pages;
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to load segment contacts'); }
		finally { loading = false; }
	}
	onMount(() => { void load(); });
</script>
<section>
	<a href="/admin/crm/segments">← Segments</a><h1>{segment?.title ?? 'Segment Contacts'}</h1>
	<a href="/admin/crm/segments/{id}/edit">Edit Segment</a>
	{#if error}<p role="alert">{error}</p><button onclick={() => void load()}>Retry</button>{/if}
	{#if loading}<p role="status">Loading matching contacts…</p>{:else if !error}
	<p>{total} matching contacts</p><button onclick={() => void load()}>Refresh Matches</button>
	{#if contacts.length === 0}<p>No contacts currently match these rules.</p>{:else}
	<table><thead><tr><th>Email</th><th>Name</th><th>Status</th></tr></thead><tbody>{#each contacts as contact (contact.id)}<tr><td><a href="/admin/crm/contacts/{contact.id}">{contact.email}</a></td><td>{contact.first_name ?? ''} {contact.last_name ?? ''}</td><td>{contact.status}</td></tr>{/each}</tbody></table>{/if}
	<nav aria-label="Segment contacts pagination"><button disabled={currentPage <= 1} onclick={() => { currentPage--; void load(); }}>Previous</button><span>Page {currentPage} of {totalPages}</span><button disabled={currentPage >= totalPages} onclick={() => { currentPage++; void load(); }}>Next</button></nav>
	{/if}
</section>
<style>section { padding: 2rem; color: #e2e8f0; } a { color: #7dd3fc; } td,th { padding: 0.75rem; text-align: left; } nav { display: flex; gap: 1rem; margin-top: 1rem; } [role='alert'] { color: #fca5a5; }</style>
