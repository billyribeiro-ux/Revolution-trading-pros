<!--
	URL: /admin/forms/[id]/edit
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import IconPencil from '@tabler/icons-svelte-runes/icons/pencil';
	import IconPalette from '@tabler/icons-svelte-runes/icons/palette';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import FormBuilder from '#lib/components/forms/FormBuilder.svelte';
	import EmbedCodeGenerator from '#lib/components/forms/EmbedCodeGenerator.svelte';
	import ThemeCustomizer from '#lib/components/forms/ThemeCustomizer.svelte';
	import { getForm } from '#lib/api/forms.js';
	import type { Form } from '#lib/api/forms.js';
	import type { FormTheme } from '#lib/data/formTemplates.js';
	import { themes } from '#lib/data/formTemplates.js';

	let form = $state<Form | null>(null);
	let loading = $state(true);
	let error = $state('');
	let showEmbedCode = $state(false);
	let showThemeCustomizer = $state(false);
	let selectedTheme = $state<FormTheme | null>(null);

	let formId = $derived(parseInt(page.params['id'] ?? ''));

	onMount(async () => {
		try {
			form = await getForm(formId);
			// Load theme from form settings if available
			if (form?.settings?.theme) {
				selectedTheme = form.settings.theme;
			} else {
				selectedTheme = themes[0] ?? null; // Default theme
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load form';
		} finally {
			loading = false;
		}
	});

	function handleSave() {
		goto('/admin/forms');
	}

	function handleCancel() {
		goto('/admin/forms');
	}

	function toggleEmbedCode() {
		showEmbedCode = !showEmbedCode;
		showThemeCustomizer = false;
	}

	function toggleThemeCustomizer() {
		showThemeCustomizer = !showThemeCustomizer;
		showEmbedCode = false;
	}

	function handleThemeChange(theme: FormTheme) {
		selectedTheme = theme;
		if (form) {
			form = {
				...form,
				settings: {
					...form.settings,
					theme: selectedTheme
				}
			};
		}
	}
</script>

<svelte:head>
	<title>Edit Form - Admin Dashboard</title>
</svelte:head>

<div class="edit-form-page">
	<div class="page-header">
		<div>
			<h1>Edit Form</h1>
			{#if form}
				<p class="page-description">Editing: {form.title}</p>
			{/if}
		</div>
		<div class="header-actions">
			<button class="btn-theme" onclick={toggleThemeCustomizer}>
				{#if showThemeCustomizer}
					<IconPencil size={16} aria-hidden="true" /> Edit Form
				{:else}
					<IconPalette size={16} aria-hidden="true" /> Customize Theme
				{/if}
			</button>
			{#if form && form.status === 'published'}
				<button class="btn-embed" onclick={toggleEmbedCode}>
					{#if showEmbedCode}
						<IconPencil size={16} aria-hidden="true" /> Edit Form
					{:else}
						<IconLink size={16} aria-hidden="true" /> Get Embed Code
					{/if}
				</button>
			{/if}
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<p>Loading form...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button class="btn-back" onclick={() => goto('/admin/forms')}> Back to Forms </button>
		</div>
	{:else if form}
		{#if showEmbedCode && form.status === 'published'}
			<EmbedCodeGenerator {form} />
		{:else if showThemeCustomizer && selectedTheme}
			<div class="theme-section">
				<ThemeCustomizer {selectedTheme} onchange={handleThemeChange} />
			</div>
			<FormBuilder {form} isEditing={true} onsave={handleSave} oncancel={handleCancel} />
		{:else}
			<FormBuilder {form} isEditing={true} onsave={handleSave} oncancel={handleCancel} />
		{/if}
	{/if}
</div>

<style>
	.edit-form-page {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.page-description {
		color: var(--text-secondary, #8b949e);
		font-size: 0.9375rem;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-theme {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-theme:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
	}

	.btn-embed {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
		
	}


	.theme-section {
		margin-bottom: 2rem;
	}

	.loading-state,
	.error-state {
		text-align: center;
		padding: 4rem 2rem;
		color: var(--text-secondary, #8b949e);
	}

	.error-state {
		color: var(--error-emphasis, #f85149);
	}

	.btn-back {
		margin-top: 1rem;
		padding: 0.75rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	@media (max-width: 767.98px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.btn-theme,
		.btn-embed {
			width: 100%;
		}
	}
</style>
