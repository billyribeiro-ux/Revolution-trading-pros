<script lang="ts">
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { Contact, ContactList } from '#lib/crm/types.js';
	let { id }: { id: string } = $props();
	let list = $state<ContactList | null>(null);
	let contacts = $state<Contact[]>([]);
	let selected = $state<string[]>([]);
	let contactIds = $state('');
	let search = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let total = $state(0);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let message = $state('');
	async function load() {
		loading = true;
		error = '';
		try {
			const [record, members] = await Promise.all([
				crmAPI.getContactList(id),
				crmAPI.getListContacts(id, { page: currentPage, search, per_page: 25 })
			]);
			list = record.list;
			contacts = members.data;
			total = members.meta.total;
			totalPages = members.meta.total_pages ?? 1;
			selected = [];
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load list contacts');
		} finally {
			loading = false;
		}
	}
	async function change(add: boolean) {
		if (saving) return;
		const ids = add ? contactIds.split(/[\s,]+/).filter(Boolean) : selected;
		if (ids.length === 0) {
			error = 'Choose contacts or enter their contact IDs';
			return;
		}
		saving = true;
		error = '';
		message = '';
		try {
			if (add) {
				const result = await crmAPI.addContactsToList(id, ids);
				message = `${result.added_count} contacts added`;
				contactIds = '';
			} else {
				const result = await crmAPI.removeContactsFromList(id, ids);
				message = `${result.removed_count} contacts removed from this list`;
			}
			currentPage = 1;
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to update list contacts');
		} finally {
			saving = false;
		}
	}
	onMount(() => {
		void load();
	});
</script>

<section class="list-members">
	<a href="/admin/crm/lists">← Contact Lists</a>
	<h1>{list?.title ?? 'List Contacts'}</h1>
	<a href="/admin/crm/lists/{id}/edit">Edit List</a>
	{#if error}<p role="alert">{error}</p>
		<button onclick={() => void load()}>Retry loading</button>{/if}
	{#if message}<p role="status">{message}</p>{/if}
	<form
		onsubmit={(event) => {
			event.preventDefault();
			void change(true);
		}}
	>
		<label for="membership-ids">Contact IDs</label>
		<input
			id="membership-ids"
			autocomplete="off"
			bind:value={contactIds}
			placeholder="Contact IDs separated by commas"
			required
		/>
		<button disabled={saving}>Add Contacts</button>
		<a href="/admin/crm">Browse Contacts</a>
	</form>
	<form
		onsubmit={(event) => {
			event.preventDefault();
			currentPage = 1;
			void load();
		}}
	>
		<label for="membership-search">Search list contacts</label>
		<input id="membership-search" type="search" bind:value={search} />
		<button>Search</button>
	</form>
	{#if loading}<p role="status">Loading contacts…</p>{:else}
		<p>{total} matching contacts</p>
		<button disabled={saving || selected.length === 0} onclick={() => void change(false)}
			>Remove Selected From List</button
		>
		{#if contacts.length === 0}<p>No matching contacts in this list.</p>{:else}
			<table>
				<thead><tr><th>Select</th><th>Contact ID</th><th>Email</th><th>Name</th></tr></thead><tbody>
					{#each contacts as contact (contact.id)}
						<tr
							><td
								><input
									type="checkbox"
									bind:group={selected}
									value={String(contact.id)}
									aria-label={`Select ${contact.email}`}
								/></td
							><td>{contact.id}</td><td
								><a href="/admin/crm/contacts/{contact.id}">{contact.email}</a></td
							><td>{contact.first_name ?? ''} {contact.last_name ?? ''}</td></tr
						>
					{/each}
				</tbody>
			</table>
		{/if}
		<nav aria-label="List contact pagination">
			<button
				disabled={currentPage <= 1}
				onclick={() => {
					currentPage--;
					void load();
				}}>Previous</button
			><span>Page {currentPage} of {totalPages}</span><button
				disabled={currentPage >= totalPages}
				onclick={() => {
					currentPage++;
					void load();
				}}>Next</button
			>
		</nav>
	{/if}
</section>

<style>
	.list-members {
		max-width: 70rem;
		margin: 0 auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		margin: 1rem 0;
		font-size: 1.75rem;
	}
	form,
	nav {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin: 1.5rem 0;
	}
	input:not([type='checkbox']) {
		padding: 0.75rem;
		border: 1px solid var(--border-color, #52525b);
		background: var(--bg-elevated, #18181b);
		color: inherit;
		border-radius: 0.5rem;
	}
	button {
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: var(--accent-primary, #6366f1);
		color: white;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		padding: 0.75rem;
		border-bottom: 1px solid var(--border-color, #52525b);
		text-align: left;
		overflow-wrap: anywhere;
	}
</style>
