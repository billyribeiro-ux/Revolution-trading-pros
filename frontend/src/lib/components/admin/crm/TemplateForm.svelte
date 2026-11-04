<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id }: { id?: string } = $props();
	let title = $state('');
	let subject = $state('');
	let content = $state('');
	let category = $state('');
	let design = $state<'raw' | 'visual'>('raw');
	let configuration = $state('{}');
	let isDefault = $state(false);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	async function load() {
		loading = true;
		error = '';
		try {
			if (id) {
				const template = await crmAPI.getEmailTemplate(id);
				title = template.title;
				subject = template.subject ?? '';
				content = template.content;
				category = template.category ?? '';
				design = template.design_template;
				configuration = JSON.stringify(template.template_config ?? {}, null, 2);
				isDefault = template.is_default;
			}
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load template');
		} finally { loading = false; }
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving) return;
		saving = true;
		error = '';
		try {
			const config: unknown = JSON.parse(configuration);
			if (config === null || typeof config !== 'object' || Array.isArray(config)) {
				throw new Error('Template configuration must be a JSON object');
			}
			const fields = { title, subject, content, category, design_template: design,
				template_config: config as Record<string, unknown>, is_default: isDefault };
			if (id) await crmAPI.updateEmailTemplate(id, fields);
			else await crmAPI.createEmailTemplate(fields);
			await goto('/admin/crm/templates', { invalidateAll: true });
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to save template'); }
		finally { saving = false; }
	}
	onMount(() => { void load(); });
</script>

<section class="template-form">
	<a href="/admin/crm/templates">← Templates</a>
	<h1>{id ? 'Edit Email Template' : 'New Email Template'}</h1>
	{#if error}<p role="alert">{error}</p>{#if id}<button onclick={() => void load()}>Retry loading</button>{/if}{/if}
	{#if loading}<p role="status">Loading template…</p>{:else}
	<form onsubmit={(event) => void save(event)}>
		<label for="template-title">Template title</label><input id="template-title" bind:value={title} required maxlength={255} />
		<label for="template-subject">Email subject</label><input id="template-subject" bind:value={subject} maxlength={500} />
		<label for="template-category">Category</label><input id="template-category" bind:value={category} maxlength={100} />
		<label for="template-design">Design format</label><select id="template-design" bind:value={design}><option value="raw">HTML</option><option value="visual">Visual template</option></select>
		<label for="template-html">Email HTML</label><textarea id="template-html" bind:value={content} required maxlength={1000000} rows={18}></textarea>
		<label for="template-config">Template configuration (JSON)</label><textarea id="template-config" bind:value={configuration} required maxlength={256000} rows={6}></textarea>
		<label class="check"><input type="checkbox" bind:checked={isDefault} /> Default template</label>
		<div><button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Template'}</button> <a href="/admin/crm/templates">Cancel</a></div>
	</form>{/if}
</section>

<style>
	.template-form { max-width: 64rem; padding: 2rem; color: #e2e8f0; }
	form { display: grid; gap: 0.75rem; }
	input, select, textarea, button { padding: 0.75rem; border: 1px solid #475569; border-radius: 0.5rem; background: #0f172a; color: inherit; }
	textarea { font-family: monospace; }
	.check { display: flex; gap: 0.5rem; align-items: center; }
	[role='alert'] { color: #fca5a5; }
	a { color: #7dd3fc; }
</style>
