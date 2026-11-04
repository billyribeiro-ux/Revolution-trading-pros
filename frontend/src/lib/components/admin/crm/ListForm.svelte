<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id }: { id?: string } = $props();
	let title = $state('');
	let description = $state('');
	let isPublic = $state(false);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	async function load() {
		if (!id) {
			loading = false;
			return;
		}
		loading = true;
		error = '';
		try {
			const { list } = await crmAPI.getContactList(id);
			title = list.title;
			description = list.description ?? '';
			isPublic = list.is_public;
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load list');
		} finally {
			loading = false;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving) return;
		saving = true;
		error = '';
		try {
			const fields = { title: title.trim(), description, is_public: isPublic };
			if (id) await crmAPI.updateContactList(id, fields);
			else await crmAPI.createContactList(fields);
			await goto('/admin/crm/lists', { invalidateAll: true });
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to save list');
		} finally {
			saving = false;
		}
	}
	onMount(() => {
		void load();
	});
</script>

<section class="list-editor">
	<a href="/admin/crm/lists">← Contact Lists</a>
	<h1>{id ? 'Edit Contact List' : 'New Contact List'}</h1>
	{#if error}<p role="alert">{error}</p>
		{#if id}<button onclick={() => void load()}>Retry loading</button>{/if}{/if}
	{#if loading}<p role="status">Loading list…</p>{:else}
		<form onsubmit={(event) => void save(event)}>
			<label for="list-title">List title</label>
			<input id="list-title" bind:value={title} required maxlength={255} autocomplete="off" />
			<label for="list-description">Description</label>
			<textarea id="list-description" bind:value={description} rows={5} maxlength={50000}
			></textarea>
			<label><input type="checkbox" bind:checked={isPublic} /> Public list</label>
			<div class="actions">
				<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save List'}</button><a
					href="/admin/crm/lists">Cancel</a
				>
			</div>
		</form>
	{/if}
</section>

<style>
	.list-editor {
		max-width: 48rem;
		margin: 0 auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		margin: 1rem 0 1.5rem;
		font-size: 1.75rem;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	input:not([type='checkbox']),
	textarea {
		padding: 0.75rem;
		border: 1px solid var(--border-color, #52525b);
		background: var(--bg-elevated, #18181b);
		color: inherit;
		border-radius: 0.5rem;
	}
	.actions {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-top: 1rem;
	}
	button {
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: var(--accent-primary, #6366f1);
		color: white;
	}
</style>
