<script lang="ts">
	/**
	 * FormStepField Component (FluentForms Pro 6.1.8)
	 *
	 * Multi-step/wizard form functionality.
	 * Allows splitting long forms into manageable steps.
	 */

	import type { Snippet } from 'svelte';

	interface StepConfig {
		id: string;
		title: string;
		icon?: string;
		description?: string;
	}

	interface Props {
		steps: StepConfig[];
		currentStep?: number;
		showProgressBar?: boolean;
		showStepNumbers?: boolean;
		showNavigation?: boolean;
		allowSkip?: boolean;
		prevButtonText?: string;
		nextButtonText?: string;
		submitButtonText?: string;
		disabled?: boolean;
		validationErrors?: Record<string, string[]>;
		onstepchange?: (step: number, direction: 'next' | 'prev') => boolean | void;
		onsubmit?: () => void;
		children?: Snippet<[{ activeStep: number; currentStepData: StepConfig }]>;
	}

	let {
		steps,
		currentStep = 0,
		showProgressBar = true,
		showStepNumbers = true,
		showNavigation = true,
		allowSkip = false,
		prevButtonText = 'Previous',
		nextButtonText = 'Next',
		submitButtonText = 'Submit',
		disabled = false,
		validationErrors = {},
		onstepchange,
		onsubmit,
		children
	}: Props = $props();

	let activeStep = $state(0);

	// Sync activeStep with currentStep prop changes
	$effect(() => {
		activeStep = currentStep;
	});
	let completedSteps = $state<Set<number>>(new Set());
	let visitedSteps = $state<Set<number>>(new Set([0]));

	const totalSteps = $derived(steps.length);
	const progress = $derived(((activeStep + 1) / totalSteps) * 100);
	const isFirstStep = $derived(activeStep === 0);
	const isLastStep = $derived(activeStep === totalSteps - 1);
	const currentStepData = $derived(steps[activeStep]);

	function goToStep(stepIndex: number) {
		if (disabled) return;
		if (stepIndex < 0 || stepIndex >= totalSteps) return;
		if (!allowSkip && stepIndex > activeStep && !completedSteps.has(activeStep)) return;

		const direction = stepIndex > activeStep ? 'next' : 'prev';

		if (onstepchange) {
			const canProceed = onstepchange(stepIndex, direction);
			if (canProceed === false) return;
		}

		if (direction === 'next') {
			completedSteps.add(activeStep);
			completedSteps = completedSteps;
		}

		visitedSteps.add(stepIndex);
		visitedSteps = visitedSteps;
		activeStep = stepIndex;
	}

	function handlePrev() {
		if (isFirstStep) return;
		goToStep(activeStep - 1);
	}

	function handleNext() {
		if (isLastStep) {
			if (onsubmit) onsubmit();
		} else {
			goToStep(activeStep + 1);
		}
	}

	function getStepStatus(index: number): 'completed' | 'active' | 'upcoming' | 'error' {
		if (completedSteps.has(index)) return 'completed';
		if (index === activeStep) return 'active';
		if (validationErrors[steps[index]?.id]) return 'error';
		return 'upcoming';
	}

	function canClickStep(index: number): boolean {
		if (disabled) return false;
		if (allowSkip) return true;
		return index <= activeStep || completedSteps.has(index - 1);
	}
</script>

<div class="form-step" class:disabled>
	<!-- Progress Bar -->
	{#if showProgressBar}
		<div class="progress-bar">
			<div class="progress-fill" style="width: {progress}%"></div>
		</div>
	{/if}

	<!-- Step Indicators -->
	<div class="step-indicators">
		{#each steps as step, index}
			<button
				type="button"
				class="step-indicator {getStepStatus(index)}"
				disabled={!canClickStep(index)}
				onclick={() => goToStep(index)}
			>
				{#if showStepNumbers}
					<span class="step-number">
						{#if getStepStatus(index) === 'completed'}
							<svg viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
									clip-rule="evenodd"
								/>
							</svg>
						{:else if getStepStatus(index) === 'error'}
							<svg viewBox="0 0 20 20" fill="currentColor">
								<path
									fill-rule="evenodd"
									d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
									clip-rule="evenodd"
								/>
							</svg>
						{:else}
							{index + 1}
						{/if}
					</span>
				{/if}
				<span class="step-title">{step.title}</span>
				{#if step.description && index === activeStep}
					<span class="step-description">{step.description}</span>
				{/if}
			</button>

			{#if index < steps.length - 1}
				<div class="step-connector" class:completed={completedSteps.has(index)}></div>
			{/if}
		{/each}
	</div>

	<!-- Step Content -->
	<div class="step-content">
		{@render children?.({ activeStep, currentStepData })}
	</div>

	<!-- Navigation Buttons -->
	{#if showNavigation}
		<div class="step-navigation">
			<button
				type="button"
				class="nav-btn prev-btn"
				onclick={handlePrev}
				disabled={isFirstStep || disabled}
			>
				<svg viewBox="0 0 20 20" fill="currentColor">
					<path
						fill-rule="evenodd"
						d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
						clip-rule="evenodd"
					/>
				</svg>
				{prevButtonText}
			</button>

			<div class="step-counter">
				Step {activeStep + 1} of {totalSteps}
			</div>

			<button type="button" class="nav-btn next-btn" onclick={handleNext} {disabled}>
				{isLastStep ? submitButtonText : nextButtonText}
				{#if !isLastStep}
					<svg viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
							clip-rule="evenodd"
						/>
					</svg>
				{/if}
			</button>
		</div>
	{/if}
</div>

<style>
	.form-step {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.progress-bar {
		height: 4px;
		background-color: #e5e7eb;
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background-color: #3b82f6;
		transition: width 0.3s ease;
	}

	.step-indicators {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
	}

	.step-indicator {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		background: none;
		border: none;
		padding: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		flex: 0 0 auto;
		max-width: 120px;
	}

	.step-indicator:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.step-number {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 50%;
		font-size: 0.875rem;
		font-weight: 600;
		transition: all 0.2s;
	}

	.step-number svg {
		width: 18px;
		height: 18px;
	}

	.step-indicator.upcoming .step-number {
		background-color: #e5e7eb;
		color: #6b7280;
	}

	.step-indicator.active .step-number {
		background-color: #3b82f6;
		color: white;
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.2);
	}

	.step-indicator.completed .step-number {
		background-color: #10b981;
		color: white;
	}

	.step-indicator.error .step-number {
		background-color: #ef4444;
		color: white;
	}

	.step-title {
		font-size: 0.75rem;
		font-weight: 500;
		color: #374151;
		text-align: center;
	}

	.step-indicator.active .step-title {
		color: #3b82f6;
	}

	.step-indicator.completed .step-title {
		color: #10b981;
	}

	.step-description {
		font-size: 0.6875rem;
		color: #6b7280;
		text-align: center;
	}

	.step-connector {
		flex: 1;
		height: 2px;
		background-color: #e5e7eb;
		margin-top: 16px;
		margin-left: -0.5rem;
		margin-right: -0.5rem;
		transition: background-color 0.3s;
	}

	.step-connector.completed {
		background-color: #10b981;
	}

	.step-content {
		min-height: 200px;
	}

	.step-navigation {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	.nav-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border: none;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.nav-btn svg {
		width: 18px;
		height: 18px;
	}

	.prev-btn {
		background-color: #f3f4f6;
		color: #374151;
	}

	.prev-btn:hover:not(:disabled) {
		background-color: #e5e7eb;
	}

	.prev-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.next-btn {
		background-color: #3b82f6;
		color: white;
	}

	.next-btn:hover:not(:disabled) {
		background-color: #2563eb;
	}

	.next-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.step-counter {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.step-indicators {
			flex-wrap: nowrap;
			overflow-x: auto;
			padding-bottom: 0.5rem;
		}

		.step-connector {
			min-width: 20px;
		}

		.step-indicator {
			min-width: 60px;
		}

		.step-title {
			display: none;
		}

		.step-description {
			display: none;
		}
	}
</style>
