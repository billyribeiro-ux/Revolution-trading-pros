<!--
	URL: /admin/forms/[id]/edit
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import FormBuilder from '$lib/components/forms/FormBuilder.svelte';
	import EmbedCodeGenerator from '$lib/components/forms/EmbedCodeGenerator.svelte';
	import ThemeCustomizer from '$lib/components/forms/ThemeCustomizer.svelte';
	import { getForm } from '$lib/api/forms';
	import type { Form } from '$lib/api/forms';
	import type { FormTheme } from '$lib/data/formTemplates';
	import { themes } from '$lib/data/formTemplates';

	let form = $state<Form | null>(null);
	let loading = $state(true);
	let error = $state('');
	let showEmbedCode = $state(false);
	let showThemeCustomizer = $state(false);
	let selectedTheme = $state<FormTheme | null>(null);

	let formId = $derived(parseInt(page.params['id']!));

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
				{showThemeCustomizer ? '📝 Edit Form' : '🎨 Customize Theme'}
			</button>
			{#if form && form.status === 'published'}
				<button class="btn-embed" onclick={toggleEmbedCode}>
					{showEmbedCode ? '📝 Edit Form' : '🔗 Get Embed Code'}
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
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.page-description {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-theme {
		padding: 0.875rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
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
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-embed:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(230, 184, 0, 0.4);
	}

	.theme-section {
		margin-bottom: 2rem;
	}

	.loading-state,
	.error-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.error-state {
		color: #f87171;
	}

	.btn-back {
		margin-top: 1rem;
		padding: 0.75rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	@media (max-width: 768px) {
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
