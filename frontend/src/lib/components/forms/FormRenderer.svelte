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

	let props: Props = $props();

	let formData: Record<string, any> = $state({});
	let errors: Record<string, string[]> = $state({});
	let isSubmitting = $state(false);
	let submitSuccess = $state(false);
	let submitMessage = $state('');
	let visibleFields: Set<number> = $state(new Set());

	// ICT 7 Fix: Honeypot fields for spam protection (hidden from users, visible to bots)
	let honeypotWebsite = $state('');
	let honeypotEmail = $state('');

	// Initialize form data with default values and track form view
	onMount(() => {
		if (props.form.fields) {
			props.form.fields.forEach((field) => {
				if (field.default_value) {
					formData[field.name] = field.default_value;
				}
			});
			updateVisibleFields();
		}

		// ICT 7 Fix: Track form view for analytics
		trackFormView();
	});

	// Track form view for conversion analytics
	async function trackFormView() {
		try {
			await fetch(`/api/forms/${props.form.slug}/view`, { method: 'POST' });
		} catch {
			// Silent fail - analytics shouldn't break the form
		}
	}

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
		if (!props.form.fields) return;

		const newVisibleFields = new Set<number>();
		props.form.fields.forEach((field) => {
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
			// ICT 7 Fix: Include honeypot fields in submission for server-side spam detection
			const submissionData = {
				...formData,
				_hp_website: honeypotWebsite,
				_hp_email: honeypotEmail
			};

			const result = await submitForm(props.form.slug, submissionData);

			if (result.success) {
				submitSuccess = true;
				submitMessage = result.message || 'Form submitted successfully!';

				// Reset form
				formData = {};
				if (props.form.fields) {
					props.form.fields.forEach((field) => {
						if (field.default_value) {
							formData[field.name] = field.default_value;
						}
					});
				}
				updateVisibleFields();

				// Call success callback
				if (props.onSuccess && result.submission_id) {
					props.onSuccess(result.submission_id);
				}

				// Redirect if specified
				if (result.redirect_url) {
					window.location.href = result.redirect_url;
				}
			} else if (result.errors) {
				errors = result.errors;
				submitMessage = 'Please correct the errors below.';

				if (props.onError) {
					props.onError(submitMessage);
				}
			}
		} catch (error) {
			const errorMessage =
				error instanceof Error ? error.message : 'Submission failed. Please try again.';
			submitMessage = errorMessage;

			if (props.onError) {
				props.onError(errorMessage);
			}
		} finally {
			isSubmitting = false;
		}
	}

	// Get CSS classes for the form
	function getFormClasses(): string {
		const baseClasses = 'revolution-form';
		const customClasses = props.form.styles?.['form_class'] || '';
		return `${baseClasses} ${customClasses}`.trim();
	}

	// Get custom styles
	function getFormStyles(): string {
		if (!props.form.styles) return '';

		const styles: string[] = [];

		if (props.form.styles.background_color) {
			styles.push(`background-color: ${props.form.styles.background_color}`);
		}
		if (props.form.styles.padding) {
			styles.push(`padding: ${props.form.styles.padding}`);
		}
		if (props.form.styles.border_radius) {
			styles.push(`border-radius: ${props.form.styles.border_radius}`);
		}

		return styles.join('; ');
	}
</script>

<div class={getFormClasses()} style={getFormStyles()}>
	{#if props.form.description}
		<div class="form-description">
			<p>{props.form.description}</p>
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
		<!-- ICT 7 Fix: Honeypot fields for spam protection (hidden from users via CSS) -->
		<div class="honeypot" aria-hidden="true" tabindex="-1">
			<label for="hp_website">Website</label>
			<input
				type="text"
				id="hp_website"
				name="_hp_website"
				bind:value={honeypotWebsite}
				autocomplete="off"
				tabindex="-1"
			/>
			<label for="hp_email">Email confirm</label>
			<input
				type="email"
				id="hp_email"
				name="_hp_email"
				bind:value={honeypotEmail}
				autocomplete="off"
				tabindex="-1"
			/>
		</div>

		<div class="fields-container">
			{#if props.form.fields}
				{#each props.form.fields.sort((a, b) => a.order - b.order) as field (field.id)}
					{#if field.id && visibleFields.has(field.id)}
						<!-- ICT 7 Fix: Use CSS custom property for proper width inheritance -->
						<div class="field-wrapper" style="--field-width: {field.width ?? 100}%">
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
				{isSubmitting ? 'Submitting...' : props.form.settings?.submit_text || 'Submit'}
			</button>
		</div>
	</form>
</div>

<style>
	/* ICT 7 Fix: Honeypot fields - hidden from users but visible to bots */
	.honeypot {
		position: absolute;
		left: -9999px;
		top: -9999px;
		width: 1px;
		height: 1px;
		overflow: hidden;
		opacity: 0;
		pointer-events: none;
		z-index: -1;
	}

	/* 2026 Mobile-First Form Container */
	.revolution-form {
		max-width: 800px;
		margin: 0 auto;
		padding: 1rem;
		position: relative;
	}

	@media (min-width: 640px) {
		.revolution-form {
			padding: 2rem;
		}
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
		gap: 1.25rem;
	}

	/* 2026 Mobile-First: Stack all fields vertically on mobile */
	.fields-container {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	@media (min-width: 640px) {
		.fields-container {
			flex-direction: row;
			flex-wrap: wrap;
			gap: 1.5rem;
		}
	}

	/* ICT 7 Fix: Field width properly respects the --field-width custom property on desktop */
	.field-wrapper {
		min-width: 0;
		flex-shrink: 0;
		width: 100%; /* Full width on mobile */
	}

	@media (min-width: 640px) {
		.field-wrapper {
			/* Calculate proper width accounting for gap */
			width: calc(var(--field-width, 100%) - 0.75rem);
			flex-basis: calc(var(--field-width, 100%) - 0.75rem);
			flex-grow: 0;
		}

		/* Full width fields don't need gap adjustment */
		.field-wrapper[style*="--field-width: 100%"] {
			width: 100%;
			flex-basis: 100%;
		}
	}

	/* Force full width on small screens regardless of field width setting */
	@media (max-width: 639px) {
		.field-wrapper {
			width: 100% !important;
			flex-basis: 100% !important;
		}
	}

	/* 2026 Mobile-First: Safe area for notched devices */
	.form-actions {
		margin-top: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	@media (min-width: 640px) {
		.form-actions {
			flex-direction: row;
			justify-content: flex-end;
		}
	}

	/* 2026 Mobile-First Responsive Buttons */
	.btn {
		padding: 0.875rem 1.5rem;
		border-radius: 0.375rem;
		font-size: 1rem; /* 16px prevents iOS zoom */
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		min-height: 48px; /* Enhanced touch target */
		width: 100%; /* Full width on mobile */
		touch-action: manipulation;
		-webkit-appearance: none;
		appearance: none;
	}

	@media (min-width: 640px) {
		.btn {
			width: auto;
		}
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
