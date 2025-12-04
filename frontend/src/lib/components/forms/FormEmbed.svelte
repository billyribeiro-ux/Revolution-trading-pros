<script lang="ts">
	/**
	 * Form Component - Enterprise Form Embedding
	 *
	 * Multiple ways to reference forms (pick one):
	 *
	 * 1. RECOMMENDED: Using Form Registry (type-safe)
	 *    import { FORMS } from '$lib/config/forms';
	 *    <Form form={FORMS.CONTACT} />
	 *
	 * 2. By slug:
	 *    <Form slug="contact-us" />
	 *
	 * 3. By ID:
	 *    <Form id="contact" />
	 *
	 * With options:
	 *    <Form
	 *      form={FORMS.CONTACT}
	 *      theme="card"
	 *      hideTitle
	 *      onSuccess={(id) => console.log('Submitted:', id)}
	 *    />
	 */

	import { onMount } from 'svelte';
	import { previewForm, submitForm, type Form as FormType } from '$lib/api/forms';
	import type { FormDefinition } from '$lib/config/forms';
	import FormFieldRenderer from './FormFieldRenderer.svelte';

	interface Props {
		/** Form from registry - RECOMMENDED: import { FORMS } from '$lib/config/forms' */
		form?: FormDefinition;
		/** Form slug (alternative to form prop) */
		slug?: string;
		/** Form ID (alternative to form prop) */
		id?: string;
		/** Hide the form title */
		hideTitle?: boolean;
		/** Hide the form description */
		hideDescription?: boolean;
		/** Custom submit button text (overrides form setting) */
		submitText?: string;
		/** Custom success message (overrides form setting) */
		successMessage?: string;
		/** Theme variant: 'default' | 'minimal' | 'bordered' | 'card' */
		theme?: 'default' | 'minimal' | 'bordered' | 'card';
		/** Custom CSS class */
		class?: string;
		/** Callback on successful submission */
		onSuccess?: (submissionId: string) => void;
		/** Callback on error */
		onError?: (error: string) => void;
		/** Redirect URL after submission (overrides form setting) */
		redirectUrl?: string;
		/** Show loading skeleton while loading */
		showSkeleton?: boolean;
	}

	let {
		form: formConfig,
		slug: slugProp,
		id: idProp,
		hideTitle = false,
		hideDescription = false,
		submitText,
		successMessage,
		theme = 'default',
		class: className = '',
		onSuccess,
		onError,
		redirectUrl,
		showSkeleton = true
	}: Props = $props();

	// Resolve the form slug from props (priority: form > slug > id)
	function getFormSlug(): string {
		if (formConfig?.slug) return formConfig.slug;
		if (slugProp) return slugProp;
		if (idProp) return idProp; // ID can be used as slug fallback
		throw new Error('Form component requires either form, slug, or id prop');
	}

	// Get the resolved slug
	let resolvedSlug = $derived(getFormSlug());

	let formInstance: FormType | null = $state(null);
	let formData: Record<string, any> = $state({});
	let errors: Record<string, string[]> = $state({});
	let loading = $state(true);
	let loadError = $state('');
	let isSubmitting = $state(false);
	let submitted = $state(false);
	let submitMessage = $state('');
	let visibleFields: Set<number> = $state(new Set());

	onMount(async () => {
		await loadForm();
	});

	async function loadForm() {
		loading = true;
		loadError = '';
		try {
			formInstance = await previewForm(resolvedSlug);
			if (formInstance?.fields) {
				formInstance.fields.forEach((field) => {
					if (field.default_value) {
						formData[field.name] = field.default_value;
					}
				});
				updateVisibleFields();
			}
		} catch (err) {
			loadError = err instanceof Error ? err.message : 'Form not found';
		} finally {
			loading = false;
		}
	}

	function shouldDisplayField(field: any): boolean {
		if (!field.conditional_logic?.enabled) return true;

		const logic = field.conditional_logic;
		const results: boolean[] = [];

		logic.rules?.forEach((rule: any) => {
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
				case 'is_empty':
					result = !fieldValue || fieldValue === '';
					break;
				case 'is_not_empty':
					result = !!fieldValue && fieldValue !== '';
					break;
				default:
					result = true;
			}

			results.push(result);
		});

		const conditionsMet = logic.logic === 'all' ? !results.includes(false) : results.includes(true);
		return logic.action === 'show' ? conditionsMet : !conditionsMet;
	}

	function updateVisibleFields() {
		if (!formInstance?.fields) return;
		const newVisible = new Set<number>();
		formInstance.fields.forEach((field) => {
			if (field.id && shouldDisplayField(field)) {
				newVisible.add(field.id);
			}
		});
		visibleFields = newVisible;
	}

	function handleFieldChange(fieldName: string, value: any) {
		formData[fieldName] = value;
		updateVisibleFields();
		if (errors[fieldName]) {
			const { [fieldName]: _, ...rest } = errors;
			errors = rest;
		}
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		if (isSubmitting || !formInstance) return;

		errors = {};
		isSubmitting = true;

		try {
			const result = await submitForm(resolvedSlug, formData);

			if (result.success) {
				submitted = true;
				submitMessage = successMessage || result.message || formInstance.settings?.success_message || 'Thank you for your submission!';

				// Reset form
				formData = {};
				formInstance.fields?.forEach((field) => {
					if (field.default_value) {
						formData[field.name] = field.default_value;
					}
				});

				onSuccess?.(result.submission_id || '');

				// Redirect if specified
				const redirect = redirectUrl || result.redirect_url;
				if (redirect) {
					setTimeout(() => {
						window.location.href = redirect;
					}, 1500);
				}
			} else if (result.errors) {
				errors = result.errors;
				onError?.('Please correct the errors below.');
			}
		} catch (err) {
			const errorMsg = err instanceof Error ? err.message : 'Submission failed';
			submitMessage = errorMsg;
			onError?.(errorMsg);
		} finally {
			isSubmitting = false;
		}
	}

	function resetForm() {
		submitted = false;
		submitMessage = '';
		errors = {};
	}

</script>

<div class="form-embed form-embed--{theme} {className}" data-form-slug={resolvedSlug}>
	{#if loading}
		{#if showSkeleton}
			<div class="form-skeleton">
				<div class="skeleton-title"></div>
				<div class="skeleton-field"></div>
				<div class="skeleton-field"></div>
				<div class="skeleton-field short"></div>
				<div class="skeleton-button"></div>
			</div>
		{:else}
			<div class="form-loading">
				<div class="spinner"></div>
			</div>
		{/if}
	{:else if loadError}
		<div class="form-error">
			<p>{loadError}</p>
		</div>
	{:else if submitted}
		<div class="form-success">
			<div class="success-icon">✓</div>
			<p>{submitMessage}</p>
			<button type="button" class="btn-reset" onclick={resetForm}>Submit Another Response</button>
		</div>
	{:else if formInstance}
		{#if !hideTitle && formInstance.title}
			<h3 class="form-title">{formInstance.title}</h3>
		{/if}

		{#if !hideDescription && formInstance.description}
			<p class="form-description">{formInstance.description}</p>
		{/if}

		<form onsubmit={handleSubmit} class="form-fields">
			{#if formInstance.fields}
				{#each formInstance.fields.sort((a, b) => a.order - b.order) as field (field.id)}
					{#if field.id && visibleFields.has(field.id)}
						<div class="field-wrapper" style="--field-width: {field.width || 100}%">
							<FormFieldRenderer
								{field}
								value={formData[field.name]}
								error={errors[field.name]}
								onchange={(val) => handleFieldChange(field.name, val)}
							/>
						</div>
					{/if}
				{/each}
			{/if}

			<div class="form-actions">
				<button type="submit" class="btn-submit" disabled={isSubmitting}>
					{#if isSubmitting}
						<span class="btn-spinner"></span>
						Submitting...
					{:else}
						{submitText || formInstance.settings?.submit_text || 'Submit'}
					{/if}
				</button>
			</div>
		</form>
	{/if}
</div>

<style>
	.form-embed {
		--fe-primary: #6366f1;
		--fe-primary-hover: #4f46e5;
		--fe-success: #10b981;
		--fe-error: #ef4444;
		--fe-text: #1f2937;
		--fe-text-muted: #6b7280;
		--fe-border: #e5e7eb;
		--fe-bg: #ffffff;
		--fe-radius: 0.5rem;
	}

	/* Theme: Default */
	.form-embed--default {
		padding: 1.5rem;
	}

	/* Theme: Minimal */
	.form-embed--minimal {
		padding: 0;
	}

	.form-embed--minimal .form-title {
		font-size: 1.125rem;
	}

	/* Theme: Bordered */
	.form-embed--bordered {
		padding: 1.5rem;
		border: 1px solid var(--fe-border);
		border-radius: var(--fe-radius);
	}

	/* Theme: Card */
	.form-embed--card {
		padding: 2rem;
		background: var(--fe-bg);
		border-radius: calc(var(--fe-radius) * 2);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);
	}

	.form-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--fe-text);
		margin: 0 0 0.5rem 0;
	}

	.form-description {
		color: var(--fe-text-muted);
		margin: 0 0 1.5rem 0;
		font-size: 0.9375rem;
	}

	.form-fields {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.field-wrapper {
		width: var(--field-width, 100%);
		min-width: 0;
	}

	@media (max-width: 640px) {
		.field-wrapper {
			width: 100% !important;
		}
	}

	.form-actions {
		width: 100%;
		margin-top: 0.5rem;
	}

	.btn-submit {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: var(--fe-primary);
		color: white;
		border: none;
		border-radius: var(--fe-radius);
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.btn-submit:hover:not(:disabled) {
		background: var(--fe-primary-hover);
	}

	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 1rem;
		height: 1rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	/* Loading States */
	.form-loading {
		display: flex;
		justify-content: center;
		padding: 3rem;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 3px solid var(--fe-border);
		border-top-color: var(--fe-primary);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Skeleton Loading */
	.form-skeleton {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.skeleton-title,
	.skeleton-field,
	.skeleton-button {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--fe-radius);
	}

	.skeleton-title {
		height: 1.75rem;
		width: 60%;
	}

	.skeleton-field {
		height: 3rem;
	}

	.skeleton-field.short {
		width: 50%;
	}

	.skeleton-button {
		height: 2.75rem;
		width: 120px;
		margin-top: 0.5rem;
	}

	@keyframes shimmer {
		0% {
			background-position: -200% 0;
		}
		100% {
			background-position: 200% 0;
		}
	}

	/* Error State */
	.form-error {
		padding: 1.5rem;
		text-align: center;
		color: var(--fe-error);
		background: #fef2f2;
		border-radius: var(--fe-radius);
	}

	/* Success State */
	.form-success {
		text-align: center;
		padding: 2rem;
	}

	.success-icon {
		width: 3rem;
		height: 3rem;
		margin: 0 auto 1rem;
		background: var(--fe-success);
		color: white;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		font-weight: bold;
	}

	.form-success p {
		color: var(--fe-text);
		margin: 0 0 1rem 0;
	}

	.btn-reset {
		background: none;
		border: 1px solid var(--fe-border);
		padding: 0.5rem 1rem;
		border-radius: var(--fe-radius);
		color: var(--fe-text-muted);
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-reset:hover {
		background: #f9fafb;
	}
</style>
