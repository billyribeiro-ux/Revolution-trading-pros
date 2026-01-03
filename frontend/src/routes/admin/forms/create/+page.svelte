<!--
	URL: /admin/forms/create
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import FormBuilder from '$lib/components/forms/FormBuilder.svelte';
	import FormTemplateSelector from '$lib/components/forms/FormTemplateSelector.svelte';
	import ThemeCustomizer from '$lib/components/forms/ThemeCustomizer.svelte';
	import type { Form } from '$lib/api/forms';
	import type { FormTemplate, FormTheme } from '$lib/data/formTemplates';

	let showTemplateSelector = true;
	let selectedTemplate: FormTemplate | null = null;
	let formData: Partial<Form> | null = null;
	let selectedTheme: FormTheme | null = null;
	let showThemeCustomizer = false;

	function handleTemplateSelect(template: FormTemplate) {
		// Svelte 5: Callback props receive the value directly
		selectedTemplate = template;
		showTemplateSelector = false;
		selectedTheme = selectedTemplate.theme;

		// Create initial form data from template
		formData = {
			title: selectedTemplate.name,
			description: selectedTemplate.description,
			fields: selectedTemplate.fields.map((field, index) => ({
				...field,
				id: index + 1,
				order: index,
				field_group_id: null,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString()
			})) as any,
			settings: {
				...selectedTemplate.settings,
				theme: selectedTheme
			},
			status: 'draft'
		};
	}

	function handleStartFromScratch() {
		selectedTemplate = null;
		showTemplateSelector = false;
		formData = null;
	}

	function handleThemeChange(theme: FormTheme) {
		selectedTheme = theme;
		if (formData) {
			formData = {
				...formData,
				settings: {
					...formData.settings,
					theme: selectedTheme
				}
			};
		}
	}

	function handleSave() {
		goto('/admin/forms');
	}

	function handleCancel() {
		if (!showTemplateSelector && (selectedTemplate || formData)) {
			// Go back to template selector
			showTemplateSelector = true;
			selectedTemplate = null;
			formData = null;
		} else {
			// Go back to forms list
			goto('/admin/forms');
		}
	}

	function toggleThemeCustomizer() {
		showThemeCustomizer = !showThemeCustomizer;
	}
</script>

<svelte:head>
	<title>Create Form - Admin Dashboard</title>
</svelte:head>

<div class="create-form-page">
	{#if showTemplateSelector}
		<FormTemplateSelector onSelect={handleTemplateSelect} onCancel={handleStartFromScratch} />
	{:else}
		<div class="page-header">
			<div>
				<button class="btn-back" onclick={handleCancel}>
					← Back {selectedTemplate ? 'to Templates' : ''}
				</button>
				<h1>Create New Form</h1>
				<p class="page-description">
					{#if selectedTemplate}
						Using template: {selectedTemplate.name}
					{:else}
						Build a custom form with conditional logic and validation
					{/if}
				</p>
			</div>
			{#if selectedTheme}
				<button class="btn-theme" onclick={toggleThemeCustomizer}>
					🎨 {showThemeCustomizer ? 'Hide' : 'Customize'} Theme
				</button>
			{/if}
		</div>

		{#if showThemeCustomizer && selectedTheme}
			<div class="theme-section">
				<ThemeCustomizer {selectedTheme} onchange={handleThemeChange} />
			</div>
		{/if}

		<FormBuilder form={formData} onsave={handleSave} oncancel={handleCancel} />
	{/if}
</div>

<style>
	.create-form-page {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-back {
		background: none;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		padding: 0.5rem 0;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}

	.btn-back:hover {
		color: #a5b4fc;
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

	.btn-theme {
		padding: 0.875rem 1.5rem;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-theme:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.theme-section {
		margin-bottom: 2rem;
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.btn-theme {
			width: 100%;
		}
	}
</style>
