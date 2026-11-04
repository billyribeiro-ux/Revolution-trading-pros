<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import type { EmailTemplate } from '#lib/crm/types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let templates = $state<EmailTemplate[]>([]);
	let loading = $state(true);
	let error = $state('');
	let copying = $state(false);
	async function load() {
		loading = true;
		error = '';
		try { templates = await crmAPI.getPrebuiltTemplates(); }
		catch (cause) { error = apiErrorMessage(cause, 'Unable to load prebuilt templates'); }
		finally { loading = false; }
	}
	async function useTemplate(id: string) {
		if (copying) return;
		copying = true;
		error = '';
		try {
			const copy = await crmAPI.duplicateEmailTemplate(id);
			await goto(`/admin/crm/templates/${copy.id}/edit`);
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to copy template'); }
		finally { copying = false; }
	}
	onMount(() => { void load(); });
</script>
<section>
	<a href="/admin/crm/templates">← Templates</a><h1>Prebuilt Email Templates</h1>
	{#if error}<p role="alert">{error}</p><button onclick={() => void load()}>Retry</button>{/if}
	{#if loading}<p role="status">Loading templates…</p>
	{:else if templates.length === 0}<p>No prebuilt templates are installed.</p>
	{:else}<ul>{#each templates as template (template.id)}<li><h2>{template.title}</h2><p>{template.subject ?? ''}</p><p>{template.category ?? ''}</p><button disabled={copying} onclick={() => void useTemplate(template.id)}>Use Template</button></li>{/each}</ul>{/if}
</section>
<style>section { padding: 2rem; color: #e2e8f0; } a { color: #7dd3fc; } li { padding: 1rem; } [role='alert'] { color: #fca5a5; }</style>
