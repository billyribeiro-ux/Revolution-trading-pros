<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { emailTemplatesApi, isEmailTemplate, type EmailTemplate } from '#lib/api/admin.js';
	import TemplateForm from '#lib/components/admin/TemplateForm.svelte';
	let { id }: { id: string } = $props();
	let loading = $state(true),
		error = $state('');
	let template = $state<EmailTemplate | null>(null);
	let generation = 0;
	async function load() {
		const current = ++generation;
		loading = true;
		error = '';
		template = null;
		try {
			const key = Number(id);
			if (!Number.isSafeInteger(key) || key <= 0) throw new Error('Invalid template identifier');
			const response = await emailTemplatesApi.get(key);
			if (!isEmailTemplate(response.data)) throw new Error('Invalid email template response');
			if (current === generation) template = response.data;
		} catch (cause) {
			if (current === generation)
				error = cause instanceof Error ? cause.message : 'Unable to load email template';
		} finally {
			if (current === generation) loading = false;
		}
	}
	onMount(() => {
		void load();
	});
	onDestroy(() => {
		generation++;
	});
</script>

<div class="edit-template-page">
	{#if loading}<p class="text-muted" role="status">Loading...</p>
	{:else if error}<p class="alert alert-error" role="alert">{error}</p>
		<button onclick={() => void load()}>Retry loading template</button>
	{:else if template}{#key template.id}<TemplateForm {template} isEdit={true} />{/key}{/if}
</div>

<style>
	.edit-template-page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}
	.text-muted {
		color: var(--text-secondary, #8b949e);
	}
	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
