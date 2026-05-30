<script lang="ts">
	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/config';
	import TemplateForm from '$lib/components/admin/TemplateForm.svelte';
	import type { EmailTemplate } from '$lib/api/admin';
	import { page } from '$app/state';

	/**
	 * R26-A LB-R26-1: the backend returns `{ data: EmailTemplate, message? }`
	 * as the envelope shape (see `api/src/routes/email_templates.rs:144`).
	 * `apiFetch` returns `T` raw, so we must unwrap `.data` here before
	 * handing the row to `<TemplateForm>` — the form's `$effect` initialises
	 * its inputs from `template.name`, `template.subject`, etc., and reading
	 * those off the envelope produced `undefined` for every field. Edit pages
	 * were rendering an empty form on load.
	 */
	let loading = $state(true);
	let error = $state('');
	let template: Partial<EmailTemplate> | null = $state(null);

	const id = page.params['id'];

	onMount(async () => {
		try {
			const response = await apiFetch<{ data: EmailTemplate } | EmailTemplate>(
				`/admin/email/templates/${id}`
			);
			// Defensive unwrap — backend envelope is `{ data: ... }`, but
			// keep the legacy un-enveloped path working too.
			template =
				response && typeof response === 'object' && 'data' in response
					? (response as { data: EmailTemplate }).data
					: (response as EmailTemplate);
		} catch (e) {
			error = (e as Error).message;
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Edit Email Template | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="edit-template-page">
	{#if loading}
		<p class="text-muted">Loading...</p>
	{:else if error}
		<p class="alert alert-error">{error}</p>
	{:else}
		<TemplateForm {template} isEdit={true} />
	{/if}
</div>

<style>
	.edit-template-page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}
	.text-muted {
		color: #94a3b8;
	}
	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
