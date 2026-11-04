<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { CrmCompany } from '#lib/crm/types.js';

	let company = $state<CrmCompany | null>(null);
	let loading = $state(true);
	let error = $state('');
	async function load() {
		loading = true;
		error = '';
		try {
			const id = page.params.id;
			if (!id) throw new Error('Company ID is missing');
			company = await crmAPI.getCompany(id);
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load company');
		} finally {
			loading = false;
		}
	}
	onMount(() => {
		void load();
	});
	const website = $derived(
		company?.website && /^https?:\/\//i.test(company.website) ? company.website : null
	);
</script>

<section class="company-detail">
	<a href="/admin/crm/companies">← Companies</a>
	{#if loading}
		<p role="status">Loading company…</p>
	{:else if error}
		<p role="alert">{error}</p>
		<button onclick={() => void load()}>Try Again</button>
	{:else if company}
		<h1>{company.name}</h1>
		<a class="edit-link" href="/admin/crm/companies/{company.id}/edit">Edit Company</a>
		<dl>
			<dt>Website</dt>
			<dd>
				{#if website}<a href={website} rel="noopener noreferrer" target="_blank">{website}</a
					>{:else}{company.website || 'Not specified'}{/if}
			</dd>
			<dt>Industry</dt>
			<dd>{company.industry?.replaceAll('_', ' ') || 'Not specified'}</dd>
			<dt>Company size</dt>
			<dd>{company.size || 'Not specified'}</dd>
			<dt>Description</dt>
			<dd class="description">{company.description || 'Not specified'}</dd>
		</dl>
	{/if}
</section>

<style>
	.company-detail {
		max-width: 48rem;
		margin: 0 auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		margin: 1rem 0;
		font-size: 1.75rem;
	}
	.edit-link {
		display: inline-block;
		margin-bottom: 1.5rem;
	}
	dl {
		display: grid;
		grid-template-columns: minmax(7rem, 1fr) minmax(0, 3fr);
		gap: 1rem;
	}
	dt {
		font-weight: 600;
	}
	dd {
		margin: 0;
		overflow-wrap: anywhere;
	}
	.description {
		white-space: pre-wrap;
	}
</style>
