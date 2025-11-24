<script lang="ts">
	import { onMount } from 'svelte';
	import { workflowApi } from '$lib/api/workflow';

	export let workflowId: number;

	let insights: any = null;
	let isLoading = true;
	let activeTab: 'predictions' | 'suggestions' | 'anomalies' = 'predictions';

	async function loadInsights() {
		isLoading = true;
		try {
			// In production: const data = await workflowApi.getWorkflowInsights(workflowId);
			insights = {
				workflow_id: workflowId,
				predictions: {
					confidence: 'high',
					predicted_success: 0.92,
					estimated_duration_ms: 3500,
					recommendation: 'High probability of success. Safe to execute.'
				},
				suggestions: [
					{
						type: 'performance',
						priority: 'medium',
						issue: 'Slow execution time detected',
						suggestion: 'Consider adding parallel execution for independent actions'
					},
					{
						type: 'reliability',
						priority: 'high',
						issue: 'Success rate below 85%',
						suggestion: 'Add error handling and retry logic to critical actions'
					}
				],
				anomalies: []
			};
		} catch (error) {
			console.error('Failed to load insights:', error);
		} finally {
			isLoading = false;
		}
	}

	function getPriorityColor(priority: string): string {
		const colors: Record<string, string> = {
			high: '#ef4444',
			medium: '#f59e0b',
			low: '#10b981'
		};
		return colors[priority] || '#6b7280';
	}

	// Reactive statement: reload insights when workflowId changes
	$: if (workflowId) {
		loadInsights();
	}
</script>

<div class="ai-insights">
	<div class="insights-header">
		<h3>🤖 AI Insights</h3>
		<p class="subtitle">Powered by machine learning</p>
	</div>

	{#if isLoading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Analyzing workflow...</p>
		</div>
	{:else if insights}
		<div class="tabs">
			<button
				class="tab"
				class:active={activeTab === 'predictions'}
				on:click={() => (activeTab = 'predictions')}
			>
				Predictions
			</button>
			<button
				class="tab"
				class:active={activeTab === 'suggestions'}
				on:click={() => (activeTab = 'suggestions')}
			>
				Suggestions
			</button>
			<button
				class="tab"
				class:active={activeTab === 'anomalies'}
				on:click={() => (activeTab = 'anomalies')}
			>
				Anomalies
			</button>
		</div>

		<div class="tab-content">
			{#if activeTab === 'predictions'}
				<div class="prediction-card">
					<div class="metric">
						<span class="label">Success Probability</span>
						<span class="value success"
							>{Math.round(insights.predictions.predicted_success * 100)}%</span
						>
					</div>
					<div class="metric">
						<span class="label">Estimated Duration</span>
						<span class="value"
							>{(insights.predictions.estimated_duration_ms / 1000).toFixed(1)}s</span
						>
					</div>
					<div class="metric">
						<span class="label">Confidence</span>
						<span class="value confidence">{insights.predictions.confidence}</span>
					</div>
					<div class="recommendation">
						<strong>Recommendation:</strong>
						<p>{insights.predictions.recommendation}</p>
					</div>
				</div>
			{:else if activeTab === 'suggestions'}
				<div class="suggestions-list">
					{#each insights.suggestions as suggestion}
						<div class="suggestion-card">
							<div class="suggestion-header">
								<span
									class="priority-badge"
									style="background: {getPriorityColor(
										suggestion.priority
									)}20; color: {getPriorityColor(suggestion.priority)};"
								>
									{suggestion.priority}
								</span>
								<span class="type-badge">{suggestion.type}</span>
							</div>
							<div class="suggestion-body">
								<p class="issue"><strong>Issue:</strong> {suggestion.issue}</p>
								<p class="suggestion-text"><strong>Suggestion:</strong> {suggestion.suggestion}</p>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="anomalies-list">
					{#if insights.anomalies.length === 0}
						<div class="no-anomalies">
							<svg
								width="48"
								height="48"
								viewBox="0 0 24 24"
								fill="none"
								stroke="#10b981"
								stroke-width="2"
							>
								<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
								<polyline points="22 4 12 14.01 9 11.01" />
							</svg>
							<p>No anomalies detected</p>
							<span>Your workflow is performing normally</span>
						</div>
					{:else}
						{#each insights.anomalies as anomaly}
							<div class="anomaly-card">
								<p>{anomaly.message}</p>
							</div>
						{/each}
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.ai-insights {
		background: white;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.insights-header {
		padding: 1.5rem;
		border-bottom: 1px solid #e5e7eb;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: white;
	}

	.insights-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		font-size: 0.875rem;
		opacity: 0.9;
		margin: 0;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 3rem;
		gap: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #e5e7eb;
		border-top-color: #667eea;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid #e5e7eb;
	}

	.tab {
		flex: 1;
		padding: 0.75rem 1rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #6b7280;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #374151;
		background: #f9fafb;
	}

	.tab.active {
		color: #667eea;
		border-bottom-color: #667eea;
	}

	.tab-content {
		padding: 1.5rem;
	}

	.prediction-card {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.metric {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
	}

	.label {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.value {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
	}

	.value.success {
		color: #10b981;
	}

	.value.confidence {
		text-transform: capitalize;
		color: #667eea;
	}

	.recommendation {
		padding: 1rem;
		background: #eff6ff;
		border-left: 3px solid #3b82f6;
		border-radius: 8px;
	}

	.recommendation strong {
		color: #1e40af;
	}

	.recommendation p {
		margin: 0.5rem 0 0 0;
		color: #1f2937;
		line-height: 1.5;
	}

	.suggestions-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.suggestion-card {
		padding: 1rem;
		background: #f9fafb;
		border-radius: 8px;
		border-left: 3px solid #667eea;
	}

	.suggestion-header {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.priority-badge,
	.type-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.type-badge {
		background: #e5e7eb;
		color: #374151;
	}

	.suggestion-body {
		font-size: 0.875rem;
	}

	.issue {
		margin: 0 0 0.5rem 0;
		color: #6b7280;
	}

	.suggestion-text {
		margin: 0;
		color: #1f2937;
	}

	.no-anomalies {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem;
		text-align: center;
	}

	.no-anomalies p {
		margin: 1rem 0 0.25rem 0;
		font-weight: 600;
		color: #1f2937;
	}

	.no-anomalies span {
		font-size: 0.875rem;
		color: #6b7280;
	}
</style>
