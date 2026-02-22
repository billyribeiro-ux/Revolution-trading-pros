<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import type { Form, FormField } from '$lib/api/forms';
	import { submitForm } from '$lib/api/forms';
	import FormFieldRenderer from './FormFieldRenderer.svelte';

	interface FormStep {
		id: number;
		title: string;
		description?: string;
		fields: FormField[];
	}

	interface Props {
		form: Form;
		onSuccess?: (submissionId: string) => void;
		onError?: (error: string) => void;
		onStepChange?: (step: number, totalSteps: number) => void;
		enableSaveProgress?: boolean;
		showProgressBar?: boolean;
		showStepNumbers?: boolean;
		animationType?: 'slide' | 'fade' | 'none';
	}

	let {
		form,
		onSuccess,
		onError,
		onStepChange,
		enableSaveProgress = true,
		showProgressBar = true,
		showStepNumbers = true,
		animationType = 'slide'
	}: Props = $props();

	// Form state
	let formData: Record<string, any> = $state({});
	let errors: Record<string, string[]> = $state({});
	let isSubmitting = $state(false);
	let submitSuccess = $state(false);
	let submitMessage = $state('');
	let visibleFields: Set<number> = $state(new Set());

	// Multi-step state
	let currentStep = $state(0);
	let steps: FormStep[] = $state([]);
	let stepValidation: Record<number, boolean> = $state({});
	let visitedSteps: Set<number> = $state(new Set([0]));
	let animationDirection = $state<'forward' | 'backward'>('forward');
	let isAnimating = $state(false);

	// Parse form fields into steps
	function parseSteps(): FormStep[] {
		if (!form.fields || form.fields.length === 0) {
			return [{ id: 0, title: 'Form', fields: [] }];
		}

		const sortedFields = [...form.fields].sort((a, b) => a.order - b.order);

		// Check if form has step dividers
		const hasStepDividers = sortedFields.some(
			(f) => f.field_type === 'step' || f.field_type === 'page_break'
		);

		if (!hasStepDividers) {
			// Single step form - return all fields in one step
			return [
				{
					id: 0,
					title: form.title || 'Form',
					description: form.description ?? '',
					fields: sortedFields
				}
			];
		}

		// Parse multi-step form
		const parsedSteps: FormStep[] = [];
		let currentStepFields: FormField[] = [];
		let stepIndex = 0;
		let stepTitle = 'Step 1';
		let stepDescription: string | undefined;

		for (const field of sortedFields) {
			if (field.field_type === 'step' || field.field_type === 'page_break') {
				// Save current step if has fields
				if (currentStepFields.length > 0) {
					parsedSteps.push({
						id: stepIndex,
						title: stepTitle,
						description: stepDescription ?? '',
						fields: currentStepFields
					});
					stepIndex++;
				}

				// Start new step
				currentStepFields = [];
				stepTitle = field.label || `Step ${stepIndex + 1}`;
				stepDescription = field.help_text;
			} else {
				currentStepFields.push(field);
			}
		}

		// Add final step
		if (currentStepFields.length > 0) {
			parsedSteps.push({
				id: stepIndex,
				title: stepTitle,
				description: stepDescription ?? '',
				fields: currentStepFields
			});
		}

		return parsedSteps.length > 0 ? parsedSteps : [{ id: 0, title: 'Form', fields: sortedFields }];
	}

	// Initialize form
	onMount(() => {
		steps = parseSteps();

		// Initialize form data with default values
		if (form.fields) {
			form.fields.forEach((field) => {
				if (field.default_value) {
					formData[field.name] = field.default_value;
				}
			});
		}

		// Try to restore saved progress
		if (enableSaveProgress) {
			restoreProgress();
		}

		updateVisibleFields();
	});

	// Save progress to localStorage
	function saveProgress() {
		if (!enableSaveProgress || !form.slug) return;

		const progressData = {
			formData,
			currentStep,
			visitedSteps: Array.from(visitedSteps),
			timestamp: Date.now()
		};

		try {
			localStorage.setItem(`form_progress_${form.slug}`, JSON.stringify(progressData));
		} catch (e) {
			logger.warn('Failed to save form progress:', e);
		}
	}

	// Restore progress from localStorage
	function restoreProgress() {
		if (!form.slug) return;

		try {
			const saved = localStorage.getItem(`form_progress_${form.slug}`);
			if (!saved) return;

			const progressData = JSON.parse(saved);

			// Check if progress is less than 24 hours old
			const maxAge = 24 * 60 * 60 * 1000;
			if (Date.now() - progressData.timestamp > maxAge) {
				localStorage.removeItem(`form_progress_${form.slug}`);
				return;
			}

			formData = progressData.formData || {};
			currentStep = Math.min(progressData.currentStep || 0, steps.length - 1);
			visitedSteps = new Set(progressData.visitedSteps || [0]);
		} catch (e) {
			logger.warn('Failed to restore form progress:', e);
		}
	}

	// Clear saved progress
	function clearProgress() {
		if (form.slug) {
			localStorage.removeItem(`form_progress_${form.slug}`);
		}
	}

	// Evaluate conditional logic
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

	// Update visible fields
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

	// Handle field change
	function handleFieldChange(fieldName: string, value: any) {
		formData[fieldName] = value;
		updateVisibleFields();

		// Clear error for this field
		if (errors[fieldName]) {
			const { [fieldName]: _, ...rest } = errors;
			errors = rest;
		}

		// Auto-save progress
		saveProgress();
	}

	// Validate current step
	function validateCurrentStep(): boolean {
		const currentStepFields = steps[currentStep]?.fields || [];
		let isValid = true;
		const newErrors: Record<string, string[]> = {};

		for (const field of currentStepFields) {
			// Skip hidden fields
			if (field.id && !visibleFields.has(field.id)) continue;

			const value = formData[field.name];

			// Required validation
			if (field.required && (!value || value === '')) {
				newErrors[field.name] = [`${field.label || field.name} is required`];
				isValid = false;
				continue;
			}

			// Additional validation rules
			if (value && field.validation) {
				const validationErrors: string[] = [];

				// Email validation
				if (field.field_type === 'email') {
					const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
					if (!emailRegex.test(value)) {
						validationErrors.push('Please enter a valid email address');
					}
				}

				// Min/Max length
				if (field.validation.min_length && String(value).length < field.validation.min_length) {
					validationErrors.push(`Minimum ${field.validation.min_length} characters required`);
				}
				if (field.validation.max_length && String(value).length > field.validation.max_length) {
					validationErrors.push(`Maximum ${field.validation.max_length} characters allowed`);
				}

				// Min/Max value for numbers
				if (field.validation.min !== undefined && Number(value) < field.validation.min) {
					validationErrors.push(`Minimum value is ${field.validation.min}`);
				}
				if (field.validation.max !== undefined && Number(value) > field.validation.max) {
					validationErrors.push(`Maximum value is ${field.validation.max}`);
				}

				// Pattern validation
				if (field.validation.pattern) {
					const regex = new RegExp(field.validation.pattern);
					if (!regex.test(value)) {
						validationErrors.push(field.validation.pattern_message || 'Invalid format');
					}
				}

				if (validationErrors.length > 0) {
					newErrors[field.name] = validationErrors;
					isValid = false;
				}
			}
		}

		errors = newErrors;
		stepValidation[currentStep] = isValid;
		return isValid;
	}

	// Navigate to next step
	async function nextStep() {
		if (isAnimating) return;

		if (!validateCurrentStep()) {
			return;
		}

		if (currentStep < steps.length - 1) {
			isAnimating = true;
			animationDirection = 'forward';

			await new Promise((resolve) => setTimeout(resolve, 50));
			currentStep++;
			visitedSteps.add(currentStep);
			saveProgress();

			onStepChange?.(currentStep, steps.length);

			setTimeout(() => {
				isAnimating = false;
			}, 300);
		}
	}

	// Navigate to previous step
	async function previousStep() {
		if (isAnimating) return;

		if (currentStep > 0) {
			isAnimating = true;
			animationDirection = 'backward';

			await new Promise((resolve) => setTimeout(resolve, 50));
			currentStep--;
			saveProgress();

			onStepChange?.(currentStep, steps.length);

			setTimeout(() => {
				isAnimating = false;
			}, 300);
		}
	}

	// Go to specific step
	function goToStep(stepIndex: number) {
		if (isAnimating) return;
		if (stepIndex < 0 || stepIndex >= steps.length) return;

		// Only allow going to visited steps or the next unvisited step
		if (!visitedSteps.has(stepIndex) && stepIndex !== Math.max(...visitedSteps) + 1) {
			return;
		}

		// Validate all steps up to the target
		if (stepIndex > currentStep) {
			for (let i = currentStep; i < stepIndex; i++) {
				const originalStep = currentStep;
				currentStep = i;
				if (!validateCurrentStep()) {
					currentStep = originalStep;
					return;
				}
			}
		}

		animationDirection = stepIndex > currentStep ? 'forward' : 'backward';
		currentStep = stepIndex;
		visitedSteps.add(stepIndex);
		saveProgress();
		onStepChange?.(currentStep, steps.length);
	}

	// Handle form submission
	async function handleSubmit(event: Event) {
		event.preventDefault();

		if (isSubmitting) return;

		// Validate final step
		if (!validateCurrentStep()) return;

		errors = {};
		isSubmitting = true;
		submitSuccess = false;
		submitMessage = '';

		try {
			const result = await submitForm(form.slug, formData);

			if (result.success) {
				submitSuccess = true;
				submitMessage = result.message || 'Form submitted successfully!';

				// Clear saved progress
				clearProgress();

				// Reset form
				formData = {};
				currentStep = 0;
				visitedSteps = new Set([0]);
				if (form.fields) {
					form.fields.forEach((field) => {
						if (field.default_value) {
							formData[field.name] = field.default_value;
						}
					});
				}
				updateVisibleFields();

				if (onSuccess && result.submission_id) {
					onSuccess(result.submission_id);
				}

				if (result.redirect_url) {
					window.location.href = result.redirect_url;
				}
			} else if (result.errors) {
				errors = result.errors;
				submitMessage = 'Please correct the errors below.';
				onError?.(submitMessage);
			}
		} catch (error) {
			const errorMessage =
				error instanceof Error ? error.message : 'Submission failed. Please try again.';
			submitMessage = errorMessage;
			onError?.(errorMessage);
		} finally {
			isSubmitting = false;
		}
	}

	// Calculate progress percentage
	$effect(() => {
		// Reactive progress calculation
	});

	let progressPercentage = $derived(((currentStep + 1) / steps.length) * 100);
	let isLastStep = $derived(currentStep === steps.length - 1);
	let isFirstStep = $derived(currentStep === 0);
	let currentStepData = $derived(steps[currentStep]);
</script>

<div class="multi-step-form" class:single-step={steps.length === 1}>
	{#if submitSuccess && submitMessage}
		<div class="alert alert-success" role="alert">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
				<polyline points="22 4 12 14.01 9 11.01" />
			</svg>
			<span>{submitMessage}</span>
		</div>
	{:else}
		{#if steps.length > 1}
			<!-- Progress Indicator -->
			{#if showProgressBar}
				<div class="progress-container">
					<div class="progress-bar">
						<div class="progress-fill" style="width: {progressPercentage}%"></div>
					</div>
					<div class="progress-text">
						Step {currentStep + 1} of {steps.length}
					</div>
				</div>
			{/if}

			<!-- Step Indicators -->
			{#if showStepNumbers}
				<div class="steps-indicator">
					{#each steps as step, index}
						<button
							type="button"
							class="step-dot"
							class:active={index === currentStep}
							class:completed={index < currentStep}
							class:visited={visitedSteps.has(index)}
							disabled={!visitedSteps.has(index) && index !== Math.max(...visitedSteps) + 1}
							onclick={() => goToStep(index)}
							aria-label={`Go to ${step.title}`}
						>
							{#if index < currentStep}
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
									<polyline points="20 6 9 17 4 12" />
								</svg>
							{:else}
								{index + 1}
							{/if}
						</button>
						{#if index < steps.length - 1}
							<div class="step-connector" class:completed={index < currentStep}></div>
						{/if}
					{/each}
				</div>
			{/if}
		{/if}

		<!-- Form Content -->
		<form onsubmit={handleSubmit} class="form-content">
			{#if !submitSuccess && submitMessage}
				<div class="alert alert-error" role="alert">
					{submitMessage}
				</div>
			{/if}

			<!-- Step Content -->
			<div
				class="step-content"
				class:slide-forward={animationType === 'slide' && animationDirection === 'forward'}
				class:slide-backward={animationType === 'slide' && animationDirection === 'backward'}
				class:fade={animationType === 'fade'}
			>
				{#if currentStepData}
					{#if currentStepData.title && steps.length > 1}
						<h3 class="step-title">{currentStepData.title}</h3>
					{/if}
					{#if currentStepData.description}
						<p class="step-description">{currentStepData.description}</p>
					{/if}

					<div class="fields-container">
						{#each currentStepData.fields.sort((a, b) => a.order - b.order) as field (field.id)}
							{#if field.id && visibleFields.has(field.id)}
								<div class="field-wrapper" style="width: {field.width || 100}%">
									<FormFieldRenderer
										{field}
										value={formData[field.name]}
										error={errors[field.name] ?? []}
										onchange={(val) => handleFieldChange(field.name, val)}
									/>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>

			<!-- Navigation Buttons -->
			<div class="form-navigation">
				{#if !isFirstStep}
					<button
						type="button"
						class="btn btn-secondary"
						onclick={previousStep}
						disabled={isSubmitting || isAnimating}
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<polyline points="15 18 9 12 15 6" />
						</svg>
						Previous
					</button>
				{:else}
					<div></div>
				{/if}

				{#if isLastStep}
					<button type="submit" class="btn btn-primary" disabled={isSubmitting || isAnimating}>
						{#if isSubmitting}
							<span class="spinner"></span>
							Submitting...
						{:else}
							{form.settings?.submit_text || 'Submit'}
						{/if}
					</button>
				{:else}
					<button
						type="button"
						class="btn btn-primary"
						onclick={nextStep}
						disabled={isSubmitting || isAnimating}
					>
						Next
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<polyline points="9 18 15 12 9 6" />
						</svg>
					</button>
				{/if}
			</div>
		</form>

		<!-- Save Progress Indicator -->
		{#if enableSaveProgress && steps.length > 1}
			<div class="save-indicator">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
					<polyline points="17 21 17 13 7 13 7 21" />
					<polyline points="7 3 7 8 15 8" />
				</svg>
				<span>Progress auto-saved</span>
			</div>
		{/if}
	{/if}
</div>

<style>
	.multi-step-form {
		max-width: 800px;
		margin: 0 auto;
		padding: 2rem;
	}

	.progress-container {
		margin-bottom: 2rem;
	}

	.progress-bar {
		height: 6px;
		background-color: #e5e7eb;
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #8b5cf6);
		transition: width 0.3s ease;
	}

	.progress-text {
		text-align: center;
		font-size: 0.875rem;
		color: #6b7280;
		margin-top: 0.5rem;
	}

	.steps-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 2rem;
		gap: 0;
	}

	.step-dot {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		border: 2px solid #e5e7eb;
		background: white;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.875rem;
		font-weight: 600;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.2s;
	}

	.step-dot:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.step-dot.visited:not(:disabled):hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.step-dot.active {
		border-color: #3b82f6;
		background: #3b82f6;
		color: white;
	}

	.step-dot.completed {
		border-color: #22c55e;
		background: #22c55e;
		color: white;
	}

	.step-dot svg {
		width: 16px;
		height: 16px;
	}

	.step-connector {
		width: 60px;
		height: 2px;
		background: #e5e7eb;
		transition: background 0.3s;
	}

	.step-connector.completed {
		background: #22c55e;
	}

	.form-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.step-content {
		animation: fadeIn 0.3s ease;
	}

	.step-content.slide-forward {
		animation: slideInRight 0.3s ease;
	}

	.step-content.slide-backward {
		animation: slideInLeft 0.3s ease;
	}

	.step-content.fade {
		animation: fadeIn 0.3s ease;
	}

	@keyframes slideInRight {
		from {
			opacity: 0;
			transform: translateX(30px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	@keyframes slideInLeft {
		from {
			opacity: 0;
			transform: translateX(-30px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.step-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: #111827;
		margin-bottom: 0.5rem;
	}

	.step-description {
		color: #6b7280;
		margin-bottom: 1.5rem;
		line-height: 1.5;
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

		.step-connector {
			width: 30px;
		}

		.step-dot {
			width: 32px;
			height: 32px;
			font-size: 0.75rem;
		}
	}

	.form-navigation {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
		transform: translateY(-1px);
	}

	.btn-secondary {
		background: white;
		color: #374151;
		border: 1px solid #d1d5db;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #f9fafb;
		border-color: #9ca3af;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none !important;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 0.5rem;
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

	.alert .icon {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	.save-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.save-indicator svg {
		color: #22c55e;
	}

	.single-step .progress-container,
	.single-step .steps-indicator,
	.single-step .save-indicator {
		display: none;
	}
</style>
