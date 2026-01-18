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

	// Svelte 5 runes for reactive state
	let showTemplateSelector = $state(true);
	let selectedTemplate = $state<FormTemplate | null>(null);
	let formData = $state<Partial<Form> | null>(null);
	let selectedTheme = $state<FormTheme | null>(null);
	let showThemeCustomizer = $state(false);

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

<div class="admin-forms-create">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		{#if showTemplateSelector}
		<FormTemplateSelector onSelect={handleTemplateSelect} onCancel={handleStartFromScratch} />
	{:else}
		<div class="page-header">
			<h1>Create New Form</h1>
			<p class="subtitle">
				{#if selectedTemplate}
					Using template: {selectedTemplate.name}
				{:else}
					Build a custom form with conditional logic and validation
				{/if}
			</p>
			<div class="actions">
				<button class="btn-secondary" onclick={handleCancel}>
					Back {selectedTemplate ? 'to Templates' : ''}
				</button>
				{#if selectedTheme}
					<button class="btn-secondary" onclick={toggleThemeCustomizer}>
						{showThemeCustomizer ? 'Hide' : 'Customize'} Theme
					</button>
				{/if}
			</div>
		</div>

		{#if showThemeCustomizer && selectedTheme}
			<div class="theme-section">
				<ThemeCustomizer {selectedTheme} onchange={handleThemeChange} />
			</div>
		{/if}

		<FormBuilder form={formData} onsave={handleSave} oncancel={handleCancel} />
	{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	.admin-forms-create {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	/* Unused - keeping for future use
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
		color: #E6B800;
	}*/

	h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
	}

	/* Unused - keeping for future use
	.btn-theme {
		padding: 0.875rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: #E6B800;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}*/

	/* .btn-theme:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
	}*/

	.theme-section {
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 768px) {
		.admin-forms-create {
			padding: 1rem;
		}

		.actions {
			flex-direction: column;
		}

		.btn-secondary {
			width: 100%;
		}
	}
</style>
