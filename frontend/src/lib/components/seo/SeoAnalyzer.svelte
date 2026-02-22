<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import {
		IconCircleCheck,
		IconAlertTriangle,
		IconCircleX,
		IconRefresh,
		IconTrendingUp
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES - ICT 7+ Strict Type Safety
	// ═══════════════════════════════════════════════════════════════════════════

	interface SEOAnalysisResult {
		test: string;
		status: 'good' | 'warning' | 'error';
		message: string;
	}

	interface SEOAnalysis {
		score: number;
		grade?: 'A' | 'B' | 'C' | 'D' | 'F';
		passed: number;
		warnings: number;
		errors: number;
		results: SEOAnalysisResult[];
	}

	interface ReadabilityDetails {
		words: number;
		sentences: number;
		avg_words_per_sentence: number;
		avg_syllables_per_word: number;
		complex_words: number;
	}

	interface ReadabilityMetrics {
		score: number;
		grade: string;
		level: string;
		details: ReadabilityDetails;
	}

	interface Props {
		content?: string;
		title?: string;
		description?: string;
		focusKeyword?: string;
		additionalKeywords?: string[];
	}

	let {
		content = '',
		title = '',
		description = '',
		focusKeyword = '',
		additionalKeywords = []
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Properly Typed (No 'any')
	// ═══════════════════════════════════════════════════════════════════════════

	let analysis: SEOAnalysis | null = $state(null);
	let readability: ReadabilityMetrics | null = $state(null);
	let loading = $state(false);

	$effect(() => {
		if (content || title || description || focusKeyword) {
			analyzeDebounced();
		}
	});

	let debounceTimer: ReturnType<typeof setTimeout> | undefined;
	function analyzeDebounced() {
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			analyze();
		}, 1000);
	}

	async function analyze() {
		if (!content && !title && !description) return;

		loading = true;
		try {
			const response = await fetch('/api/seo/meta/analyze', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					content,
					title,
					description,
					focus_keyword: focusKeyword,
					additional_keywords: additionalKeywords
				})
			});

			if (response.ok) {
				const data = await response.json();
				analysis = data.seo_analysis;
				readability = data.readability;
			}
		} catch (error) {
			logger.error('Analysis failed:', error);
		} finally {
			loading = false;
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'good':
				return IconCircleCheck;
			case 'warning':
				return IconAlertTriangle;
			case 'error':
				return IconCircleX;
			default:
				return IconAlertTriangle;
		}
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'good':
				return 'green';
			case 'warning':
				return 'yellow';
			case 'error':
				return 'red';
			default:
				return 'gray';
		}
	}

	onMount(() => {
		analyze();
	});
</script>

<div class="seo-analyzer">
	<div class="analyzer-header">
		<div class="scores">
			{#if analysis}
				<div class="score-card">
					<div class="score-circle" style="--progress: {analysis.score}">
						<span class="score-value">{analysis.score}</span>
					</div>
					<div class="score-label">SEO Score</div>
				</div>
			{/if}

			{#if readability}
				<div class="score-card">
					<div class="score-circle" style="--progress: {readability.score}">
						<span class="score-value">{readability.score}</span>
					</div>
					<div class="score-label">Readability</div>
					<div class="score-sublabel">{readability.level}</div>
				</div>
			{/if}
		</div>

		<button class="refresh-btn" onclick={analyze} disabled={loading}>
			<IconRefresh size={18} class={loading ? 'spinning' : ''} />
			{loading ? 'Analyzing...' : 'Refresh'}
		</button>
	</div>

	{#if analysis}
		<div class="analysis-stats">
			<div class="stat">
				<div class="stat-value">{analysis.passed}</div>
				<div class="stat-label">Passed</div>
			</div>
			<div class="stat warning">
				<div class="stat-value">{analysis.warnings}</div>
				<div class="stat-label">Warnings</div>
			</div>
			<div class="stat error">
				<div class="stat-value">{analysis.errors}</div>
				<div class="stat-label">Errors</div>
			</div>
		</div>

		<div class="analysis-results">
			<h4>Analysis Results</h4>

			<div class="results-list">
				{#each analysis.results as result}
					{@const StatusIcon = getStatusIcon(result.status)}
					<div class="result-item {getStatusColor(result.status)}">
						<div class="result-icon">
							<StatusIcon size={20} />
						</div>
						<div class="result-content">
							<div class="result-test">{result.test.replace(/_/g, ' ')}</div>
							<div class="result-message">{result.message}</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{:else}
		<div class="empty-state">
			<IconTrendingUp size={48} />
			<h4>No Analysis Yet</h4>
			<p>Add content to see SEO analysis results</p>
		</div>
	{/if}

	{#if readability}
		<div class="readability-details">
			<h4>Readability Details</h4>
			<div class="details-grid">
				<div class="detail-item">
					<span class="detail-label">Words</span>
					<span class="detail-value">{readability.details.words}</span>
				</div>
				<div class="detail-item">
					<span class="detail-label">Sentences</span>
					<span class="detail-value">{readability.details.sentences}</span>
				</div>
				<div class="detail-item">
					<span class="detail-label">Avg. Words/Sentence</span>
					<span class="detail-value">{readability.details.avg_words_per_sentence}</span>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.seo-analyzer {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
	}

	.analyzer-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.scores {
		display: flex;
		gap: 2rem;
	}

	.score-card {
		text-align: center;
	}

	.score-circle {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: conic-gradient(
			#3b82f6 calc(var(--progress) * 1%),
			#f0f0f0 calc(var(--progress) * 1%)
		);
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 0.75rem;
		position: relative;
	}

	.score-circle::before {
		content: '';
		position: absolute;
		width: 64px;
		height: 64px;
		border-radius: 50%;
		background: white;
	}

	.score-value {
		position: relative;
		z-index: 1;
		font-size: 1.5rem;
		font-weight: 700;
		color: #1a1a1a;
	}

	.score-label {
		font-weight: 500;
		color: #666;
		font-size: 0.9rem;
	}

	.score-sublabel {
		font-size: 0.8rem;
		color: #999;
		margin-top: 0.25rem;
	}

	.refresh-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		color: #666;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.refresh-btn:hover:not(:disabled) {
		background: #f8f9fa;
	}

	.refresh-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.analysis-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat {
		text-align: center;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 6px;
	}

	.stat.warning {
		background: #fef3c7;
	}

	.stat.error {
		background: #fee2e2;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.25rem;
	}

	.stat-label {
		font-size: 0.85rem;
		color: #666;
	}

	.analysis-results h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.results-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.result-item {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 6px;
		border: 1px solid;
	}

	.result-item.green {
		background: #dcfce7;
		border-color: #86efac;
		color: #16a34a;
	}

	.result-item.yellow {
		background: #fef3c7;
		border-color: #fde047;
		color: #d97706;
	}

	.result-item.red {
		background: #fee2e2;
		border-color: #fca5a5;
		color: #dc2626;
	}

	.result-icon {
		flex-shrink: 0;
	}

	.result-content {
		flex: 1;
	}

	.result-test {
		font-weight: 600;
		text-transform: capitalize;
		margin-bottom: 0.25rem;
	}

	.result-message {
		font-size: 0.9rem;
		opacity: 0.9;
	}

	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: #999;
	}

	.empty-state h4 {
		margin: 1rem 0 0.5rem;
		color: #666;
		font-weight: 500;
	}

	.empty-state p {
		font-size: 0.95rem;
	}

	.readability-details {
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid #e5e5e5;
	}

	.readability-details h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.details-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	.detail-item {
		display: flex;
		flex-direction: column;
		padding: 1rem;
		background: #f8f9fa;
		border-radius: 6px;
	}

	.detail-label {
		font-size: 0.85rem;
		color: #666;
		margin-bottom: 0.5rem;
	}

	.detail-value {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
	}
</style>
