<script lang="ts">
	/**
	 * Form AI Assistant - AI-powered form generation and optimization
	 *
	 * Features:
	 * - Natural language form generation
	 * - Field suggestions
	 * - Form optimization analysis
	 * - Content generation
	 * - Smart field detection
	 *
	 * @version 1.0.0
	 */

	import { getAuthToken } from '$lib/stores/auth.svelte';

	interface Props {
		props.formId?: number;
		props.onFieldsGenerated?: (fields: any[]) => void;
		props.onFormGenerated?: (form: any) => void;
	}

	interface Suggestion {
		type: string;
		label: string;
		name: string;
		reason: string;
	}

	interface AnalysisResult {
		score: number;
		issues: { severity: string; message: string }[];
		recommendations: { category: string; priority: string; suggestion: string; impact: string }[];
	}

	let props: Props = $props();

	// State
	let prompt = $state('');
	let isGenerating = $state(false);
	let isAnalyzing = $state(false);
	let generatedFields = $state<any[]>([]);
	let suggestions = $state<Suggestion[]>([]);
	let analysis = $state<AnalysisResult | null>(null);
	let activeTab = $state<'generate' | 'suggest' | 'analyze'>('generate');
	let error = $state('');

	// Example prompts
	const examplePrompts = [
		'Create a contact form with name, email, phone, and message',
		'Build a job application form with resume upload',
		'Design a customer feedback survey with rating scales',
		'Make an event registration form with date/time selection',
		'Create an order form with product selection and payment details'
	];

	// Generate form from description
	async function generateForm() {
		if (!prompt.trim()) return;

		isGenerating = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch('/api/forms/ai/generate', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					description: prompt,
					options: {
						style: 'professional'
					}
				})
			});

			if (response.ok) {
				const data = await response.json();
				generatedFields = data.form?.fields || [];
				props.onFormGenerated?.(data.form);
			} else {
				const errData = await response.json();
				error = errData.message || 'Failed to generate form';
			}
		} catch (err) {
			error = 'Failed to connect to AI service';
			console.error('Generation error:', err);
		}

		isGenerating = false;
	}

	// Get field suggestions
	async function getSuggestions() {
		if (!props.formId) return;

		isGenerating = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${props.formId}/ai/suggest`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				suggestions = await response.json();
			}
		} catch (err) {
			error = 'Failed to get suggestions';
			console.error('Suggestion error:', err);
		}

		isGenerating = false;
	}

	// Analyze form
	async function analyzeForm() {
		if (!props.formId) return;

		isAnalyzing = true;
		error = '';

		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${props.formId}/ai/analyze`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				analysis = await response.json();
			}
		} catch (err) {
			error = 'Failed to analyze form';
			console.error('Analysis error:', err);
		}

		isAnalyzing = false;
	}

	// Add suggested field
	function addSuggestedField(suggestion: Suggestion) {
		const field = {
			type: suggestion.type,
			label: suggestion.label,
			name: suggestion.name,
			required: false
		};
		props.onFieldsGenerated?.([field]);

		// Remove from suggestions
		suggestions = suggestions.filter((s) => s.name !== suggestion.name);
	}

	// Get score color
	function getScoreColor(score: number): string {
		if (score >= 80) return '#10b981';
		if (score >= 60) return '#f59e0b';
		return '#ef4444';
	}

	// Get priority badge color
	function getPriorityColor(priority: string): string {
		switch (priority) {
			case 'high':
				return '#ef4444';
			case 'medium':
				return '#f59e0b';
			default:
				return '#6b7280';
		}
	}
</script>

<div class="ai-assistant">
	<div class="assistant-header">
		<div class="header-icon">🤖</div>
		<div class="header-content">
			<h3>AI Assistant</h3>
			<p>Let AI help you build better forms</p>
		</div>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'generate'}
			onclick={() => (activeTab = 'generate')}
		>
			Generate
		</button>
		{#if props.formId}
			<button
				class="tab"
				class:active={activeTab === 'suggest'}
				onclick={() => (activeTab = 'suggest')}
			>
				Suggest Fields
			</button>
			<button
				class="tab"
				class:active={activeTab === 'analyze'}
				onclick={() => (activeTab = 'analyze')}
			>
				Analyze
			</button>
		{/if}
	</div>

	<div class="assistant-content">
		{#if error}
			<div class="error-message">{error}</div>
		{/if}

		<!-- Generate Tab -->
		{#if activeTab === 'generate'}
			<div class="generate-section">
				<label class="prompt-label" for="ai-prompt-input">Describe your form</label>
				<textarea
					id="ai-prompt-input"
					bind:value={prompt}
					placeholder="E.g., Create a customer feedback form with rating, comments, and contact info"
					class="prompt-input"
					rows="3"
				></textarea>

				<div class="example-prompts">
					<span class="examples-label">Try:</span>
					{#each examplePrompts.slice(0, 3) as example}
						<button class="example-btn" onclick={() => (prompt = example)}>
							{example.slice(0, 30)}...
						</button>
					{/each}
				</div>

				<button
					class="btn-generate"
					onclick={generateForm}
					disabled={isGenerating || !prompt.trim()}
				>
					{#if isGenerating}
						<span class="spinner"></span>
						Generating...
					{:else}
						Generate Form
					{/if}
				</button>

				{#if generatedFields.length > 0}
					<div class="generated-preview">
						<h4>Generated Fields ({generatedFields.length})</h4>
						<div class="fields-list">
							{#each generatedFields as field}
								<div class="field-preview">
									<span class="field-type-badge">{field.type}</span>
									<span class="field-label">{field.label}</span>
									{#if field.required}
										<span class="required-badge">Required</span>
									{/if}
								</div>
							{/each}
						</div>
						<button
							class="btn-apply"
							onclick={() => {
								props.onFieldsGenerated?.(generatedFields);
								generatedFields = [];
							}}
						>
							Apply to Form
						</button>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Suggest Tab -->
		{#if activeTab === 'suggest'}
			<div class="suggest-section">
				<p class="section-description">
					AI analyzes your current form and suggests additional fields that could improve it.
				</p>

				<button class="btn-suggest" onclick={getSuggestions} disabled={isGenerating}>
					{#if isGenerating}
						<span class="spinner"></span>
						Analyzing...
					{:else}
						Get Suggestions
					{/if}
				</button>

				{#if suggestions.length > 0}
					<div class="suggestions-list">
						{#each suggestions as suggestion}
							<div class="suggestion-card">
								<div class="suggestion-header">
									<span class="field-type-badge">{suggestion.type}</span>
									<span class="suggestion-label">{suggestion.label}</span>
								</div>
								<p class="suggestion-reason">{suggestion.reason}</p>
								<button class="btn-add-field" onclick={() => addSuggestedField(suggestion)}>
									Add Field
								</button>
							</div>
						{/each}
					</div>
				{:else if !isGenerating}
					<div class="empty-suggestions">
						<p>Click "Get Suggestions" to see AI-powered field recommendations</p>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Analyze Tab -->
		{#if activeTab === 'analyze'}
			<div class="analyze-section">
				<p class="section-description">
					Get AI-powered insights to optimize your form for better conversion.
				</p>

				<button class="btn-analyze" onclick={analyzeForm} disabled={isAnalyzing}>
					{#if isAnalyzing}
						<span class="spinner"></span>
						Analyzing...
					{:else}
						Analyze Form
					{/if}
				</button>

				{#if analysis}
					<div class="analysis-results">
						<!-- Score -->
						<div class="score-card">
							<div class="score-circle" style="--score-color: {getScoreColor(analysis.score)}">
								<span class="score-value">{analysis.score}</span>
								<span class="score-label">/ 100</span>
							</div>
							<div class="score-text">Form Optimization Score</div>
						</div>

						<!-- Issues -->
						{#if analysis.issues.length > 0}
							<div class="issues-section">
								<h4>Issues Found</h4>
								{#each analysis.issues as issue}
									<div
										class="issue-item"
										class:warning={issue.severity === 'warning'}
										class:info={issue.severity === 'info'}
									>
										<span class="issue-icon">
											{issue.severity === 'warning' ? '⚠️' : 'ℹ️'}
										</span>
										<span class="issue-message">{issue.message}</span>
									</div>
								{/each}
							</div>
						{/if}

						<!-- Recommendations -->
						{#if analysis.recommendations.length > 0}
							<div class="recommendations-section">
								<h4>Recommendations</h4>
								{#each analysis.recommendations as rec}
									<div class="recommendation-card">
										<div class="rec-header">
											<span
												class="priority-badge"
												style="background-color: {getPriorityColor(rec.priority)}"
											>
												{rec.priority}
											</span>
											<span class="rec-category">{rec.category}</span>
										</div>
										<p class="rec-suggestion">{rec.suggestion}</p>
										{#if rec.impact}
											<p class="rec-impact">Impact: {rec.impact}</p>
										{/if}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{:else if !isAnalyzing}
					<div class="empty-analysis">
						<p>Click "Analyze Form" to get optimization insights</p>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.ai-assistant {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.assistant-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
		color: white;
	}

	.header-icon {
		font-size: 2rem;
	}

	.header-content h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.header-content p {
		margin: 0.25rem 0 0 0;
		font-size: 0.875rem;
		opacity: 0.9;
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid #e5e7eb;
	}

	.tab {
		flex: 1;
		padding: 0.75rem;
		background: none;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
		border-bottom: 2px solid transparent;
	}

	.tab:hover {
		color: #374151;
		background: #f9fafb;
	}

	.tab.active {
		color: #8b5cf6;
		border-bottom-color: #8b5cf6;
	}

	.assistant-content {
		padding: 1.25rem;
	}

	.error-message {
		padding: 0.75rem;
		background: #fee2e2;
		color: #dc2626;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		margin-bottom: 1rem;
	}

	.prompt-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.prompt-input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		resize: vertical;
		font-family: inherit;
	}

	.prompt-input:focus {
		outline: none;
		border-color: #8b5cf6;
		box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.1);
	}

	.example-prompts {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.examples-label {
		font-size: 0.75rem;
		color: #6b7280;
		align-self: center;
	}

	.example-btn {
		padding: 0.25rem 0.5rem;
		background: #f3f4f6;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: #4b5563;
		cursor: pointer;
		transition: all 0.2s;
	}

	.example-btn:hover {
		background: #e5e7eb;
	}

	.btn-generate,
	.btn-suggest,
	.btn-analyze {
		width: 100%;
		padding: 0.75rem;
		background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		margin-top: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		transition: opacity 0.2s;
	}

	.btn-generate:hover:not(:disabled),
	.btn-suggest:hover:not(:disabled),
	.btn-analyze:hover:not(:disabled) {
		opacity: 0.9;
	}

	.btn-generate:disabled,
	.btn-suggest:disabled,
	.btn-analyze:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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

	.generated-preview {
		margin-top: 1.5rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 0.5rem;
	}

	.generated-preview h4 {
		margin: 0 0 0.75rem 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #374151;
	}

	.fields-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.field-preview {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: white;
		border-radius: 0.375rem;
		border: 1px solid #e5e7eb;
	}

	.field-type-badge {
		padding: 0.125rem 0.5rem;
		background: #ede9fe;
		color: #7c3aed;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.field-label {
		flex: 1;
		font-size: 0.875rem;
		color: #374151;
	}

	.required-badge {
		padding: 0.125rem 0.375rem;
		background: #fee2e2;
		color: #dc2626;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 500;
	}

	.btn-apply {
		width: 100%;
		padding: 0.5rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-apply:hover {
		background: #059669;
	}

	.section-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1rem 0;
	}

	.suggestions-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 1rem;
	}

	.suggestion-card {
		padding: 1rem;
		background: #f9fafb;
		border-radius: 0.5rem;
		border: 1px solid #e5e7eb;
	}

	.suggestion-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.suggestion-label {
		font-weight: 500;
		color: #374151;
	}

	.suggestion-reason {
		font-size: 0.8125rem;
		color: #6b7280;
		margin: 0 0 0.75rem 0;
	}

	.btn-add-field {
		padding: 0.375rem 0.75rem;
		background: #8b5cf6;
		color: white;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-add-field:hover {
		background: #7c3aed;
	}

	.empty-suggestions,
	.empty-analysis {
		text-align: center;
		padding: 2rem;
		color: #9ca3af;
	}

	.analysis-results {
		margin-top: 1rem;
	}

	.score-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 0.5rem;
		margin-bottom: 1rem;
	}

	.score-circle {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: conic-gradient(var(--score-color) calc(var(--score, 0) * 1%), #e5e7eb 0);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.score-circle::before {
		content: '';
		position: absolute;
		inset: 8px;
		background: white;
		border-radius: 50%;
	}

	.score-value {
		position: relative;
		font-size: 1.5rem;
		font-weight: 700;
		color: #111827;
	}

	.score-label {
		position: relative;
		font-size: 0.625rem;
		color: #6b7280;
	}

	.score-text {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.issues-section,
	.recommendations-section {
		margin-bottom: 1rem;
	}

	.issues-section h4,
	.recommendations-section h4 {
		margin: 0 0 0.75rem 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #374151;
	}

	.issue-item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #fef3c7;
		border-radius: 0.375rem;
		margin-bottom: 0.5rem;
	}

	.issue-item.info {
		background: #dbeafe;
	}

	.issue-icon {
		flex-shrink: 0;
	}

	.issue-message {
		font-size: 0.8125rem;
		color: #374151;
	}

	.recommendation-card {
		padding: 1rem;
		background: #f9fafb;
		border-radius: 0.5rem;
		margin-bottom: 0.75rem;
		border: 1px solid #e5e7eb;
	}

	.rec-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.priority-badge {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		color: white;
		text-transform: uppercase;
	}

	.rec-category {
		font-size: 0.75rem;
		color: #6b7280;
		text-transform: capitalize;
	}

	.rec-suggestion {
		font-size: 0.875rem;
		color: #374151;
		margin: 0 0 0.5rem 0;
	}

	.rec-impact {
		font-size: 0.75rem;
		color: #10b981;
		margin: 0;
	}
</style>
