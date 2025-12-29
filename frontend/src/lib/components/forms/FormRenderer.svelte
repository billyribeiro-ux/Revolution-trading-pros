<script lang="ts">
	import { onMount } from 'svelte';
	import type { Form, FormField } from '$lib/api/forms';
	import { submitForm } from '$lib/api/forms';
	import FormFieldRenderer from './FormFieldRenderer.svelte';

	interface Props {
		form: Form;
		onSuccess?: (submissionId: string) => void;
		onError?: (error: string) => void;
	}

	let { form, onSuccess, onError }: Props = $props();

	let formData: Record<string, any> = $state({});
	let errors: Record<string, string[]> = $state({});
	let isSubmitting = $state(false);
	let submitSuccess = $state(false);
	let submitMessage = $state('');
	let visibleFields: Set<number> = $state(new Set());

	// Initialize form data with default values
	onMount(() => {
		if (form.fields) {
			form.fields.forEach((field) => {
				if (field.default_value) {
					formData[field.name] = field.default_value;
				}
			});
			updateVisibleFields();
		}
	});

	// Evaluate if a field should be displayed based on conditional logic
	function shouldDisplayField(field: FormField): boolean {
		if (!field.conditional_logic) return true;

		const logic = field.conditional_logic;
		const results: boolean[] = [];

		logic.rules.forEach((rule) => {
			const fieldValue = formData[rule.field];
			let result = false;

			switch (rule.operator) {
				case 'equals':
					result = fieldValue == rule.value;
					break;
				case 'not_equals':
					result = fieldValue != rule.value;
					break;
				case 'contains':
					result = String(fieldValue || '').includes(String(rule.value));
					break;
				case 'greater_than':
					result = parseFloat(String(fieldValue)) > parseFloat(String(rule.value));
					break;
				case 'less_than':
					result = parseFloat(String(fieldValue)) < parseFloat(String(rule.value));
					break;
				case 'is_empty':
					result = !fieldValue || fieldValue === '';
					break;
				case 'is_not_empty':
					result = !!fieldValue && fieldValue !== '';
					break;
			}

			results.push(result);
		});

		const conditionsMet = logic.logic === 'all' ? !results.includes(false) : results.includes(true);

		return logic.action === 'show' ? conditionsMet : !conditionsMet;
	}

	// Update which fields are visible
	function updateVisibleFields() {
		if (!form.fields) return;

		const newVisibleFields = new Set<number>();
		form.fields.forEach((field) => {
			if (field.id && shouldDisplayField(field)) {
				newVisibleFields.add(field.id);
			}
		});
		visibleFields = newVisibleFields;
	}

	// Handle field value change
	function handleFieldChange(fieldName: string, value: any) {
		formData[fieldName] = value;
		updateVisibleFields();

		// Clear error for this field
		if (errors[fieldName]) {
			const { [fieldName]: _, ...rest } = errors;
			errors = rest;
		}
	}

	// Handle form submission
	async function handleSubmit(event: Event) {
		event.preventDefault();

		if (isSubmitting) return;

		errors = {};
		isSubmitting = true;
		submitSuccess = false;
		submitMessage = '';

		try {
			const result = await submitForm(form.slug, formData);

			if (result.success) {
				submitSuccess = true;
				submitMessage = result.message || 'Form submitted successfully!';

				// Reset form
				formData = {};
				if (form.fields) {
					form.fields.forEach((field) => {
						if (field.default_value) {
							formData[field.name] = field.default_value;
						}
					});
				}
				updateVisibleFields();

				// Call success callback
				if (onSuccess && result.submission_id) {
					onSuccess(result.submission_id);
				}

				// Redirect if specified
				if (result.redirect_url) {
					window.location.href = result.redirect_url;
				}
			} else if (result.errors) {
				errors = result.errors;
				submitMessage = 'Please correct the errors below.';

				if (onError) {
					onError(submitMessage);
				}
			}
		} catch (error) {
			const errorMessage =
				error instanceof Error ? error.message : 'Submission failed. Please try again.';
			submitMessage = errorMessage;

			if (onError) {
				onError(errorMessage);
			}
		} finally {
			isSubmitting = false;
		}
	}

	// Get CSS classes for the form
	function getFormClasses(): string {
		const baseClasses = 'revolution-form';
		const customClasses = form.styles?.['form_class'] || '';
		return `${baseClasses} ${customClasses}`.trim();
	}

	// Get custom styles
	function getFormStyles(): string {
		if (!form.styles) return '';

		const styles: string[] = [];

		if (form.styles.background_color) {
			styles.push(`background-color: ${form.styles.background_color}`);
		}
		if (form.styles.padding) {
			styles.push(`padding: ${form.styles.padding}`);
		}
		if (form.styles.border_radius) {
			styles.push(`border-radius: ${form.styles.border_radius}`);
		}

		return styles.join('; ');
	}
</script>

<div class={getFormClasses()} style={getFormStyles()}>
	{#if form.description}
		<div class="form-description">
			<p>{form.description}</p>
		</div>
	{/if}

	{#if submitSuccess && submitMessage}
		<div class="alert alert-success" role="alert">
			{submitMessage}
		</div>
	{/if}

	{#if !submitSuccess && submitMessage}
		<div class="alert alert-error" role="alert">
			{submitMessage}
		</div>
	{/if}

	<form onsubmit={handleSubmit} class="form-fields">
		<div class="fields-container">
			{#if form.fields}
				{#each form.fields.sort((a, b) => a.order - b.order) as field (field.id)}
					{#if field.id && visibleFields.has(field.id)}
						<div class="field-wrapper" style="width: {field.width}%">
							<FormFieldRenderer
								{field}
								value={formData[field.name]}
								error={errors[field.name] ?? []}
								onchange={(val) => handleFieldChange(field.name, val)}
							/>
						</div>
					{/if}
				{/each}
			{/if}
		</div>

		<div class="form-actions">
			<button type="submit" class="btn btn-primary" disabled={isSubmitting}>
				{isSubmitting ? 'Submitting...' : form.settings?.submit_text || 'Submit'}
			</button>
		</div>
	</form>
</div>

<style>
	.revolution-form {
		max-width: 800px;
		margin: 0 auto;
		padding: 2rem;
	}

	.form-description {
		margin-bottom: 2rem;
	}

	.form-description p {
		color: #666;
		font-size: 1rem;
		line-height: 1.5;
	}

	.alert {
		padding: 1rem;
		margin-bottom: 1.5rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.alert-success {
		background-color: #d1fae5;
		color: #065f46;
		border: 1px solid #6ee7b7;
	}

	.alert-error {
		background-color: #fee2e2;
		color: #991b1b;
		border: 1px solid #fca5a5;
	}

	.form-fields {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.fields-container {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.field-wrapper {
		min-width: 0;
		flex-shrink: 0;
	}

	@media (max-width: 768px) {
		.field-wrapper {
			width: 100% !important;
		}
	}

	.form-actions {
		margin-top: 1.5rem;
		display: flex;
		justify-content: flex-end;
	}

	.btn {
		padding: 0.75rem 1.5rem;
		border-radius: 0.375rem;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background-color: #2563eb;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background-color: #1d4ed8;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
