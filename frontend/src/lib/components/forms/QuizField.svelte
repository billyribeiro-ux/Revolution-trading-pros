<script lang="ts">
	/**
	 * QuizField - Interactive Quiz/Scoring Form Field
	 *
	 * Matches FluentForm Pro quiz features:
	 * - Multiple choice with scoring
	 * - True/False questions
	 * - Image-based options
	 * - Instant feedback (optional)
	 * - Score calculation
	 * - Time limits (optional)
	 * - Randomized options
	 */

	interface QuizOption {
		id: string;
		text: string;
		image?: string;
		score: number;
		isCorrect?: boolean;
		feedback?: string;
	}

	interface Props {
		id: string;
		label: string;
		description?: string;
		options: QuizOption[];
		questionType?: 'single' | 'multiple' | 'true_false';
		showScore?: boolean;
		showFeedback?: boolean;
		required?: boolean;
		randomizeOptions?: boolean;
		imageOptions?: boolean;
		value?: string | string[];
		error?: string[];
		onchange?: (value: string | string[], score: number) => void;
	}

	let {
		id,
		label,
		description,
		options: initialOptions,
		questionType = 'single',
		showScore = false,
		showFeedback = false,
		required = false,
		randomizeOptions = false,
		imageOptions = false,
		value = $bindable(),
		error,
		onchange
	}: Props = $props();

	// State
	let selectedOptions: Set<string> = $state(new Set());
	let answered = $state(false);
	let currentScore = $state(0);
	let feedbackMessage = $state('');
	let isCorrect = $state<boolean | null>(null);

	// Randomize options if needed
	let options = $state<QuizOption[]>([]);

	$effect(() => {
		if (randomizeOptions) {
			options = [...initialOptions].sort(() => Math.random() - 0.5);
		} else {
			options = initialOptions;
		}
	});

	// Initialize from value
	$effect(() => {
		if (value) {
			if (Array.isArray(value)) {
				selectedOptions = new Set(value);
			} else {
				selectedOptions = new Set([value]);
			}
		}
	});

	// Calculate score
	function calculateScore(): number {
		let score = 0;
		selectedOptions.forEach((optionId) => {
			const option = options.find((o) => o.id === optionId);
			if (option) {
				score += option.score;
			}
		});
		return score;
	}

	// Check if answer is correct
	function checkCorrectness(): boolean {
		if (questionType === 'single' || questionType === 'true_false') {
			const selected = Array.from(selectedOptions)[0];
			const selectedOption = options.find((o) => o.id === selected);
			return selectedOption?.isCorrect ?? false;
		} else {
			// Multiple choice - all correct must be selected, no incorrect
			const correctIds = new Set(options.filter((o) => o.isCorrect).map((o) => o.id));
			const selectedCorrect = [...selectedOptions].filter((id) => correctIds.has(id));
			const selectedIncorrect = [...selectedOptions].filter((id) => !correctIds.has(id));

			return (
				selectedCorrect.length === correctIds.size && selectedIncorrect.length === 0
			);
		}
	}

	// Get feedback for selected option
	function getFeedback(): string {
		if (questionType === 'single' || questionType === 'true_false') {
			const selected = Array.from(selectedOptions)[0];
			const selectedOption = options.find((o) => o.id === selected);
			return selectedOption?.feedback ?? '';
		}
		return '';
	}

	// Handle option selection
	function selectOption(optionId: string): void {
		if (answered && showFeedback) {
			// Don't allow changes after answering if feedback is shown
			return;
		}

		if (questionType === 'multiple') {
			// Toggle selection for multiple choice
			if (selectedOptions.has(optionId)) {
				selectedOptions.delete(optionId);
			} else {
				selectedOptions.add(optionId);
			}
			selectedOptions = new Set(selectedOptions);
		} else {
			// Single selection
			selectedOptions = new Set([optionId]);
		}

		// Update value
		const newValue =
			questionType === 'multiple'
				? Array.from(selectedOptions)
				: Array.from(selectedOptions)[0];

		value = newValue;
		answered = true;
		currentScore = calculateScore();
		isCorrect = checkCorrectness();
		feedbackMessage = getFeedback();

		onchange?.(newValue, currentScore);
	}

	// Get option classes
	function getOptionClass(option: QuizOption): string {
		const classes = ['quiz-option'];

		if (selectedOptions.has(option.id)) {
			classes.push('quiz-option--selected');

			if (showFeedback && answered) {
				classes.push(option.isCorrect ? 'quiz-option--correct' : 'quiz-option--incorrect');
			}
		}

		if (showFeedback && answered && option.isCorrect) {
			classes.push('quiz-option--show-correct');
		}

		if (imageOptions && option.image) {
			classes.push('quiz-option--with-image');
		}

		return classes.join(' ');
	}

	// Derived
	let inputType = $derived(questionType === 'multiple' ? 'checkbox' : 'radio');
</script>

<div class="quiz-field" class:quiz-field--error={error && error.length > 0}>
	<div class="quiz-field__header">
		<label for={id} class="quiz-field__label">
			{label}
			{#if required}
				<span class="quiz-field__required">*</span>
			{/if}
		</label>
		{#if showScore && answered}
			<div class="quiz-field__score">
				Score: <strong>{currentScore}</strong>
			</div>
		{/if}
	</div>

	{#if description}
		<p class="quiz-field__description">{description}</p>
	{/if}

	<div class="quiz-options" class:quiz-options--images={imageOptions}>
		{#each options as option (option.id)}
			<button
				type="button"
				class={getOptionClass(option)}
				onclick={() => selectOption(option.id)}
				disabled={answered && showFeedback}
			>
				<span class="quiz-option__indicator">
					{#if inputType === 'checkbox'}
						<svg
							class="quiz-option__check"
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="3"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polyline points="20 6 9 17 4 12"></polyline>
						</svg>
					{:else}
						<span class="quiz-option__radio-dot"></span>
					{/if}
				</span>

				{#if option.image && imageOptions}
					<div class="quiz-option__image">
						<img src={option.image} alt={option.text} />
					</div>
				{/if}

				<span class="quiz-option__text">{option.text}</span>

				{#if showFeedback && answered && selectedOptions.has(option.id)}
					<span class="quiz-option__feedback-icon">
						{#if option.isCorrect}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
								<polyline points="22 4 12 14.01 9 11.01"></polyline>
							</svg>
						{:else}
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<circle cx="12" cy="12" r="10"></circle>
								<line x1="15" y1="9" x2="9" y2="15"></line>
								<line x1="9" y1="9" x2="15" y2="15"></line>
							</svg>
						{/if}
					</span>
				{/if}
			</button>
		{/each}
	</div>

	{#if showFeedback && answered && feedbackMessage}
		<div class="quiz-field__feedback" class:quiz-field__feedback--correct={isCorrect}>
			{feedbackMessage}
		</div>
	{/if}

	{#if showFeedback && answered}
		<div class="quiz-field__result" class:quiz-field__result--correct={isCorrect}>
			{#if isCorrect}
				<span class="quiz-field__result-icon">✓</span>
				Correct!
			{:else}
				<span class="quiz-field__result-icon">✗</span>
				Incorrect
			{/if}
		</div>
	{/if}

	{#if error && error.length > 0}
		<div class="quiz-field__errors">
			{#each error as err}
				<p class="quiz-field__error">{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.quiz-field {
		margin-bottom: 1.5rem;
	}

	.quiz-field__header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.quiz-field__label {
		font-weight: 500;
		font-size: 1rem;
		color: #1a1a1a;
	}

	.quiz-field__required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.quiz-field__score {
		font-size: 0.875rem;
		color: #6b7280;
		background: #f3f4f6;
		padding: 0.25rem 0.75rem;
		border-radius: 1rem;
	}

	.quiz-field__score strong {
		color: #2563eb;
	}

	.quiz-field__description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem;
	}

	.quiz-options {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.quiz-options--images {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
	}

	.quiz-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border: 2px solid #e5e7eb;
		border-radius: 0.75rem;
		background: white;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.quiz-option:hover:not(:disabled) {
		border-color: #2563eb;
		background: #f0f7ff;
	}

	.quiz-option--selected {
		border-color: #2563eb;
		background: #eff6ff;
	}

	.quiz-option--correct {
		border-color: #10b981 !important;
		background: #d1fae5 !important;
	}

	.quiz-option--incorrect {
		border-color: #ef4444 !important;
		background: #fee2e2 !important;
	}

	.quiz-option--show-correct:not(.quiz-option--selected) {
		border-color: #10b981;
		background: #ecfdf5;
	}

	.quiz-option--with-image {
		flex-direction: column;
		text-align: center;
	}

	.quiz-option__indicator {
		width: 24px;
		height: 24px;
		border: 2px solid #d1d5db;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		transition: all 0.2s;
	}

	.quiz-option--selected .quiz-option__indicator {
		border-color: #2563eb;
		background: #2563eb;
		color: white;
	}

	.quiz-option--correct .quiz-option__indicator {
		border-color: #10b981;
		background: #10b981;
	}

	.quiz-option--incorrect .quiz-option__indicator {
		border-color: #ef4444;
		background: #ef4444;
	}

	.quiz-option__check,
	.quiz-option__radio-dot {
		opacity: 0;
		transition: opacity 0.2s;
	}

	.quiz-option--selected .quiz-option__check,
	.quiz-option--selected .quiz-option__radio-dot {
		opacity: 1;
	}

	.quiz-option__check {
		width: 14px;
		height: 14px;
	}

	.quiz-option__radio-dot {
		width: 10px;
		height: 10px;
		background: white;
		border-radius: 50%;
	}

	.quiz-option__image {
		width: 100%;
		height: 120px;
		border-radius: 0.5rem;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.quiz-option__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.quiz-option__text {
		flex: 1;
		font-size: 0.9375rem;
	}

	.quiz-option__feedback-icon {
		width: 24px;
		height: 24px;
		flex-shrink: 0;
	}

	.quiz-option__feedback-icon svg {
		width: 100%;
		height: 100%;
	}

	.quiz-option--correct .quiz-option__feedback-icon {
		color: #10b981;
	}

	.quiz-option--incorrect .quiz-option__feedback-icon {
		color: #ef4444;
	}

	.quiz-field__feedback {
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: #fef3c7;
		color: #92400e;
		font-size: 0.875rem;
	}

	.quiz-field__feedback--correct {
		background: #d1fae5;
		color: #065f46;
	}

	.quiz-field__result {
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: #fee2e2;
		color: #991b1b;
		font-size: 0.9375rem;
		font-weight: 500;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.quiz-field__result--correct {
		background: #d1fae5;
		color: #065f46;
	}

	.quiz-field__result-icon {
		font-size: 1.25rem;
	}

	.quiz-field__errors {
		margin-top: 0.5rem;
	}

	.quiz-field__error {
		font-size: 0.8125rem;
		color: #dc2626;
		margin: 0;
	}

	.quiz-field--error .quiz-option {
		border-color: #fca5a5;
	}

	.quiz-option:disabled {
		cursor: not-allowed;
		opacity: 0.7;
	}
</style>
