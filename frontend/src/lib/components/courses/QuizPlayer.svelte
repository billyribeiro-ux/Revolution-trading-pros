<script lang="ts">
	/**
	 * QuizPlayer Component
	 * Apple Principal Engineer ICT 7 Grade - February 2026
	 * Complete quiz taking experience with timer, scoring, and results
	 */

	import { apiFetch } from '$lib/api/config';

	interface Question {
		id: number;
		question_type: string;
		question_text: string;
		points: number;
		answers: Answer[];
	}

	interface Answer {
		id: number;
		answer_text: string;
	}

	interface QuizData {
		attempt_id: number;
		attempt_number: number;
		quiz_title: string;
		time_limit_minutes?: number;
		max_score: number;
		questions: Question[];
	}

	interface QuizResult {
		score: number;
		max_score: number;
		score_percent: number;
		passed: boolean;
		passing_score: number;
		time_spent_seconds: number;
		results?: { question_id: number; correct: boolean; points: number }[];
	}

	interface QuizStartResponse {
		success: boolean;
		data?: QuizData;
		error?: string;
	}

	interface QuizSubmitResponse {
		success: boolean;
		data?: QuizResult;
		error?: string;
	}

	interface Props {
		courseSlug: string;
		quizId: number;
		onComplete?: (result: QuizResult) => void;
		onClose?: () => void;
	}

	let { courseSlug, quizId, onComplete, onClose }: Props = $props();

	let quizData: QuizData | null = $state(null);
	let currentQuestionIndex = $state(0);
	let selectedAnswers: Map<number, number> = $state(new Map());
	let timeRemaining = $state(0);
	let timerInterval: ReturnType<typeof setInterval> | null = $state(null);
	let isLoading = $state(true);
	let isSubmitting = $state(false);
	let result: QuizResult | null = $state(null);
	let error = $state('');

	const currentQuestion = $derived.by(() => {
		if (!quizData || !quizData.questions) return undefined;
		return quizData.questions[currentQuestionIndex];
	});
	
	const isLastQuestion = $derived.by(() => {
		if (!quizData || !quizData.questions) return false;
		return currentQuestionIndex === quizData.questions.length - 1;
	});
	
	const progress = $derived.by(() => {
		if (!quizData || !quizData.questions) return 0;
		return Math.round(((currentQuestionIndex + 1) / quizData.questions.length) * 100);
	});
	
	const answeredCount = $derived(selectedAnswers.size);
	
	const totalQuestions = $derived.by(() => {
		if (!quizData || !quizData.questions) return 0;
		return quizData.questions.length;
	});

	const formatTime = (seconds: number): string => {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	};

	async function startQuiz() {
		try {
			isLoading = true;
			error = '';
			const res = await apiFetch<QuizStartResponse>(`/member/courses/${courseSlug}/quizzes/${quizId}/start`, {
				method: 'POST'
			});
			if (res.success && res.data) {
				quizData = res.data;
				if (res.data.time_limit_minutes) {
					timeRemaining = res.data.time_limit_minutes * 60;
					startTimer();
				}
			} else {
				error = res.error || 'Failed to start quiz';
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to start quiz';
		} finally {
			isLoading = false;
		}
	}

	function startTimer() {
		timerInterval = setInterval(() => {
			timeRemaining -= 1;
			if (timeRemaining <= 0) {
				submitQuiz();
			}
		}, 1000);
	}

	function selectAnswer(questionId: number, answerId: number) {
		selectedAnswers.set(questionId, answerId);
		selectedAnswers = new Map(selectedAnswers);
	}

	function nextQuestion() {
		if (!isLastQuestion) {
			currentQuestionIndex += 1;
		}
	}

	function prevQuestion() {
		if (currentQuestionIndex > 0) {
			currentQuestionIndex -= 1;
		}
	}

	function goToQuestion(index: number) {
		currentQuestionIndex = index;
	}

	async function submitQuiz() {
		if (!quizData) return;

		if (timerInterval) {
			clearInterval(timerInterval);
			timerInterval = null;
		}

		try {
			isSubmitting = true;
			const answers = Array.from(selectedAnswers.entries()).map(([questionId, answerId]) => ({
				question_id: questionId,
				answer_id: answerId
			}));

			const res = await apiFetch<QuizSubmitResponse>(
				`/member/courses/${courseSlug}/quizzes/${quizId}/attempts/${quizData.attempt_id}/submit`,
				{
					method: 'POST',
					body: JSON.stringify({ answers })
				}
			);

			if (res.success && res.data) {
				result = res.data;
				onComplete?.(res.data);
			} else {
				error = res.error || 'Failed to submit quiz';
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to submit quiz';
		} finally {
			isSubmitting = false;
		}
	}

	$effect(() => {
		startQuiz();
		return () => {
			if (timerInterval) clearInterval(timerInterval);
		};
	});
</script>

<div class="quiz-player">
	{#if isLoading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading quiz...</p>
		</div>
	{:else if error}
		<div class="error">
			<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
			</svg>
			<p>{error}</p>
			<button class="btn secondary" onclick={onClose}>Close</button>
		</div>
	{:else if result}
		<div class="result">
			<div class="result-icon" class:passed={result.passed} class:failed={!result.passed}>
				{#if result.passed}
					<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/>
					</svg>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/>
					</svg>
				{/if}
			</div>
			<h2>{result.passed ? 'Congratulations!' : 'Keep Learning!'}</h2>
			<p class="result-message">
				{result.passed
					? 'You passed the quiz successfully!'
					: `You scored ${result.score_percent.toFixed(0)}%. You need ${result.passing_score}% to pass.`}
			</p>
			<div class="score-display">
				<div class="score-circle" style="--percent: {result.score_percent}%">
					<span class="score-value">{result.score_percent.toFixed(0)}%</span>
				</div>
				<div class="score-details">
					<p><strong>Score:</strong> {result.score} / {result.max_score} points</p>
					<p><strong>Time:</strong> {formatTime(result.time_spent_seconds)}</p>
					<p><strong>Passing Score:</strong> {result.passing_score}%</p>
				</div>
			</div>
			{#if result.results}
				<div class="question-results">
					<h3>Question Results</h3>
					<div class="results-grid">
						{#each result.results as qr, i}
							<div class="question-result" class:correct={qr.correct} class:incorrect={!qr.correct}>
								<span class="q-num">Q{i + 1}</span>
								<span class="q-status">{qr.correct ? '+' : '-'}{qr.points}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}
			<div class="result-actions">
				<button class="btn primary" onclick={onClose}>Continue Learning</button>
			</div>
		</div>
	{:else if quizData && currentQuestion}
		<div class="quiz-header">
			<div class="quiz-info">
				<h2>{quizData.quiz_title}</h2>
				<p>Question {currentQuestionIndex + 1} of {totalQuestions}</p>
			</div>
			{#if timeRemaining > 0}
				<div class="timer" class:warning={timeRemaining < 60}>
					<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
					</svg>
					<span>{formatTime(timeRemaining)}</span>
				</div>
			{/if}
		</div>

		<div class="progress-bar">
			<div class="progress-fill" style="width: {progress}%"></div>
		</div>

		<div class="question-nav">
			{#each quizData.questions as q, i}
				<button
					class="nav-dot"
					class:current={i === currentQuestionIndex}
					class:answered={selectedAnswers.has(q.id)}
					onclick={() => goToQuestion(i)}
					aria-label="Go to question {i + 1}"
				>
					{i + 1}
				</button>
			{/each}
		</div>

		<div class="question-card">
			<div class="question-header">
				<span class="points">{currentQuestion.points} point{currentQuestion.points !== 1 ? 's' : ''}</span>
				<span class="q-type">{currentQuestion.question_type.replace('_', ' ')}</span>
			</div>
			<h3 class="question-text">{currentQuestion.question_text}</h3>

			<div class="answers">
				{#each currentQuestion.answers as answer}
					<button
						class="answer-option"
						class:selected={selectedAnswers.get(currentQuestion.id) === answer.id}
						onclick={() => selectAnswer(currentQuestion.id, answer.id)}
					>
						<span class="answer-radio"></span>
						<span class="answer-text">{answer.answer_text}</span>
					</button>
				{/each}
			</div>
		</div>

		<div class="quiz-footer">
			<div class="nav-buttons">
				<button class="btn secondary" onclick={prevQuestion} disabled={currentQuestionIndex === 0}>
					Previous
				</button>
				{#if isLastQuestion}
					<button
						class="btn primary"
						onclick={submitQuiz}
						disabled={isSubmitting || answeredCount < totalQuestions}
					>
						{isSubmitting ? 'Submitting...' : 'Submit Quiz'}
					</button>
				{:else}
					<button class="btn primary" onclick={nextQuestion}>
						Next
					</button>
				{/if}
			</div>
			<p class="answer-status">{answeredCount} of {totalQuestions} answered</p>
		</div>
	{/if}
</div>

<style>
	.quiz-player {
		max-width: 800px;
		margin: 0 auto;
		padding: 24px;
		min-height: 500px;
	}

	.loading, .error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 16px;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.error svg { color: #ef4444; }

	.quiz-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 16px;
	}

	.quiz-info h2 {
		margin: 0 0 4px;
		font-size: 20px;
		font-weight: 600;
		color: #1f2937;
	}

	.quiz-info p {
		margin: 0;
		font-size: 14px;
		color: #6b7280;
	}

	.timer {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: #f3f4f6;
		border-radius: 8px;
		font-weight: 600;
		color: #1f2937;
	}

	.timer.warning {
		background: #fef3c7;
		color: #d97706;
		animation: pulse 1s infinite;
	}

	@keyframes pulse {
		50% { opacity: 0.7; }
	}

	.progress-bar {
		height: 4px;
		background: #e5e7eb;
		border-radius: 2px;
		margin-bottom: 16px;
	}

	.progress-fill {
		height: 100%;
		background: #10b981;
		border-radius: 2px;
		transition: width 0.3s;
	}

	.question-nav {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-bottom: 24px;
	}

	.nav-dot {
		width: 32px;
		height: 32px;
		border: 2px solid #e5e7eb;
		border-radius: 8px;
		background: #fff;
		font-size: 12px;
		font-weight: 600;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
	}

	.nav-dot:hover { border-color: #143e59; }
	.nav-dot.current { border-color: #143e59; background: #143e59; color: #fff; }
	.nav-dot.answered { border-color: #10b981; background: #d1fae5; color: #10b981; }
	.nav-dot.current.answered { background: #10b981; color: #fff; }

	.question-card {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}

	.question-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 16px;
	}

	.points {
		font-size: 12px;
		font-weight: 600;
		color: #6b7280;
		background: #f3f4f6;
		padding: 4px 8px;
		border-radius: 4px;
	}

	.q-type {
		font-size: 12px;
		color: #6b7280;
		text-transform: capitalize;
	}

	.question-text {
		font-size: 18px;
		font-weight: 500;
		color: #1f2937;
		margin: 0 0 24px;
		line-height: 1.5;
	}

	.answers {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.answer-option {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		border: 2px solid #e5e7eb;
		border-radius: 8px;
		background: #fff;
		text-align: left;
		cursor: pointer;
		transition: all 0.2s;
	}

	.answer-option:hover { border-color: #143e59; }
	.answer-option.selected { border-color: #143e59; background: #f0f7ff; }

	.answer-radio {
		width: 20px;
		height: 20px;
		border: 2px solid #d1d5db;
		border-radius: 50%;
		flex-shrink: 0;
		position: relative;
	}

	.answer-option.selected .answer-radio {
		border-color: #143e59;
	}

	.answer-option.selected .answer-radio::after {
		content: '';
		position: absolute;
		top: 4px;
		left: 4px;
		width: 8px;
		height: 8px;
		background: #143e59;
		border-radius: 50%;
	}

	.answer-text {
		font-size: 15px;
		color: #374151;
	}

	.quiz-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 16px;
	}

	.nav-buttons {
		display: flex;
		gap: 12px;
	}

	.answer-status {
		font-size: 14px;
		color: #6b7280;
		margin: 0;
	}

	.btn {
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn.primary {
		background: #143e59;
		color: #fff;
	}

	.btn.primary:hover:not(:disabled) { background: #0f2f45; }
	.btn.primary:disabled { opacity: 0.5; cursor: not-allowed; }

	.btn.secondary {
		background: #fff;
		color: #374151;
		border: 1px solid #e5e7eb;
	}

	.btn.secondary:hover:not(:disabled) { background: #f9fafb; }
	.btn.secondary:disabled { opacity: 0.5; cursor: not-allowed; }

	/* Results styling */
	.result {
		text-align: center;
		padding: 40px 20px;
	}

	.result-icon {
		margin-bottom: 24px;
	}

	.result-icon.passed svg { color: #10b981; }
	.result-icon.failed svg { color: #ef4444; }

	.result h2 {
		font-size: 28px;
		margin: 0 0 8px;
		color: #1f2937;
	}

	.result-message {
		font-size: 16px;
		color: #6b7280;
		margin-bottom: 32px;
	}

	.score-display {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 32px;
		margin-bottom: 32px;
		flex-wrap: wrap;
	}

	.score-circle {
		width: 120px;
		height: 120px;
		border-radius: 50%;
		background: conic-gradient(#10b981 var(--percent), #e5e7eb 0);
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.score-circle::before {
		content: '';
		position: absolute;
		width: 100px;
		height: 100px;
		background: #fff;
		border-radius: 50%;
	}

	.score-value {
		position: relative;
		font-size: 28px;
		font-weight: 700;
		color: #1f2937;
	}

	.score-details {
		text-align: left;
	}

	.score-details p {
		margin: 8px 0;
		color: #6b7280;
	}

	.question-results {
		margin-bottom: 32px;
	}

	.question-results h3 {
		font-size: 16px;
		margin: 0 0 16px;
		color: #374151;
	}

	.results-grid {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 8px;
	}

	.question-result {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 8px 12px;
		border-radius: 8px;
		font-size: 12px;
	}

	.question-result.correct { background: #d1fae5; color: #10b981; }
	.question-result.incorrect { background: #fee2e2; color: #ef4444; }

	.q-num { font-weight: 600; }
	.q-status { font-size: 11px; }

	.result-actions {
		display: flex;
		justify-content: center;
		gap: 12px;
	}

	/* Responsive */
	@media (max-width: 639px) {
		.quiz-player { padding: 16px; }
		.quiz-header { flex-direction: column; gap: 12px; }
		.question-card { padding: 16px; }
		.question-text { font-size: 16px; }
		.answer-option { padding: 12px; }
		.answer-text { font-size: 14px; }
		.quiz-footer { flex-direction: column; }
		.nav-buttons { width: 100%; }
		.nav-buttons .btn { flex: 1; }
		.score-display { flex-direction: column; }
		.score-details { text-align: center; }
	}
</style>
