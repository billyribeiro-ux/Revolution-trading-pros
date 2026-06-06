<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { seoApi, type SeoAnalysis } from '$lib/api/seo';
	import { getForms, type Form } from '$lib/api/forms';
	import { IconSearch, IconChartBar } from '$lib/icons';

	let contentType = $state('posts');
	let contentId = $state('');
	let focusKeyword = $state('');
	let analyzedResult = $state<{ key: string; analysis: Partial<SeoAnalysis> } | null>(null);
	let analyzing = $state(false);

	// For content selector
	let _forms: Form[] = $state([]);

	const contentTypes = [
		{ value: 'posts', label: 'Blog Posts' },
		{ value: 'products', label: 'Products' },
		{ value: 'forms', label: 'Forms' }
	];

	let currentContentKey = $derived(contentId ? `${contentType}:${contentId}` : '');
	let savedAnalysis = $derived(
		currentContentKey
			? await seoApi.getAnalysis(contentType, parseInt(contentId, 10)).catch(() => null)
			: null
	);
	let analysis = $derived(
		analyzedResult?.key === currentContentKey ? analyzedResult.analysis : savedAnalysis
	);

	onMount(async () => {
		await loadForms();
	});

	async function loadForms() {
		try {
			const response = await getForms();
			_forms = response.forms || [];
		} catch (error) {
			console.error('Failed to load forms:', error);
		}
	}

	async function handleAnalyze() {
		if (!contentId) {
			addToast({ type: 'error', message: 'Please select content to analyze' });
			return;
		}

		try {
			analyzing = true;
			const result = await seoApi.analyze(
				contentType,
				parseInt(contentId, 10),
				focusKeyword || undefined
			);
			analyzedResult = {
				key: currentContentKey,
				analysis: result
			};
			addToast({ type: 'success', message: 'SEO analysis completed!' });
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to analyze content' });
		} finally {
			analyzing = false;
		}
	}

	function getScoreTone(score: number): 'good' | 'warning' | 'poor' {
		if (score >= 80) return 'good';
		if (score >= 60) return 'warning';
		return 'poor';
	}

	function getScoreClass(block: string, score: number): string {
		return `${block} ${block}--${getScoreTone(score)}`;
	}

	function getScoreLabel(score: number): string {
		if (score >= 80) return 'Good';
		if (score >= 60) return 'Needs Improvement';
		return 'Poor';
	}
</script>

<svelte:head>
	<title>SEO Analysis | Revolution Admin</title>
</svelte:head>

<div class="seo-analysis-page">
	<!-- Header -->
	<div class="page-header">
		<h1 class="page-title">SEO Analysis</h1>
		<p class="page-subtitle">Analyze and optimize your content for search engines</p>
	</div>

	<!-- Analysis Form -->
	<Card class="analysis-form-card">
		<h2 class="section-title">Analyze Content</h2>

		<div class="analysis-form-grid">
			<Select label="Content Type" options={contentTypes} bind:value={contentType} />

			<Input
				label="Content ID"
				type="number"
				bind:value={contentId}
				placeholder="Enter content ID"
			/>

			<Input
				label="Focus Keyword (optional)"
				bind:value={focusKeyword}
				placeholder="e.g., trading indicators"
			/>
		</div>

		<div class="form-actions">
			<Button onclick={handleAnalyze} loading={analyzing} disabled={!contentId}>
				<IconSearch size={20} />
				Analyze SEO
			</Button>
		</div>
	</Card>

	<!-- Analysis Results -->
	{#if $effect.pending()}
		<Card>
			<div class="loading-state">
				<div class="spinner" aria-label="Loading analysis"></div>
				<p class="loading-text">Loading analysis...</p>
			</div>
		</Card>
	{:else if analysis}
		<div class="results-stack">
			<!-- Score Card -->
			<Card>
				<div class="score-panel">
					<p class="score-label">SEO Score</p>
					<div class="score-ring">
						<svg aria-hidden="true" class="score-ring__svg">
							<circle
								cx="64"
								cy="64"
								r="56"
								stroke="currentColor"
								stroke-width="8"
								fill="none"
								class="score-ring__track"
							/>
							<circle
								cx="64"
								cy="64"
								r="56"
								stroke="currentColor"
								stroke-width="8"
								fill="none"
								class={getScoreClass('score-ring__progress', analysis.seo_score ?? 0)}
								stroke-dasharray={`${((analysis.seo_score ?? 0) / 100) * 351.86} 351.86`}
								stroke-linecap="round"
							/>
						</svg>
						<div class="score-ring__value">
							<p class={getScoreClass('score-number', analysis.seo_score ?? 0)}>
								{analysis.seo_score ?? 0}
							</p>
							<p class="score-total">/ 100</p>
						</div>
					</div>
					<p class={getScoreClass('score-status', analysis.seo_score ?? 0)}>
						{getScoreLabel(analysis.seo_score ?? 0)}
					</p>
				</div>
			</Card>

			<!-- Metrics Grid -->
			<div class="metrics-grid">
				<Card>
					<p class="metric-label">Keyword Density</p>
					<p class="metric-value">{(analysis.keyword_density ?? 0).toFixed(2)}%</p>
					<p class="metric-hint">Optimal: 0.5-2.5%</p>
				</Card>

				<Card>
					<p class="metric-label">Readability Score</p>
					<p class="metric-value">{analysis.readability_score}</p>
					<p class="metric-hint">Higher is better</p>
				</Card>

				<Card>
					<p class="metric-label">Meta Tags</p>
					<div class="badge-row">
						<Badge variant={analysis.has_meta_title ? 'success' : 'danger'}>
							Title {analysis.has_meta_title ? '✓' : '✗'}
						</Badge>
						<Badge variant={analysis.has_meta_description ? 'success' : 'danger'}>
							Description {analysis.has_meta_description ? '✓' : '✗'}
						</Badge>
					</div>
				</Card>
			</div>

			<!-- Suggestions -->
			{#if analysis.suggestions && analysis.suggestions.length > 0}
				<Card>
					<h3 class="section-title">Improvement Suggestions</h3>
					<ul class="insight-list">
						{#each analysis.suggestions as suggestion (suggestion)}
							<li class="insight-item">
								<span class="insight-item__icon insight-item__icon--warning">⚠</span>
								<span class="insight-item__text">{suggestion}</span>
							</li>
						{/each}
					</ul>
				</Card>
			{/if}

			<!-- Analysis Results -->
			{#if analysis.analysis_results && analysis.analysis_results.length > 0}
				<Card>
					<h3 class="section-title">Analysis Details</h3>
					<ul class="insight-list insight-list--compact">
						{#each analysis.analysis_results as result (result)}
							<li class="insight-item">
								<span class="insight-item__icon insight-item__icon--success">✓</span>
								<span class="insight-item__text">{result}</span>
							</li>
						{/each}
					</ul>
				</Card>
			{/if}
		</div>
	{:else}
		<Card>
			<div class="empty-state">
				<IconChartBar size={48} class="empty-state__icon" />
				<p class="empty-state__text">
					No analysis available. Select content and click "Analyze SEO" to get started.
				</p>
			</div>
		</Card>
	{/if}
</div>

<style>
	.seo-analysis-page {
		color: #111827;
	}

	.page-header {
		margin-bottom: 1.5rem;
	}

	.page-title {
		margin: 0;
		color: #111827;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.page-subtitle {
		margin: 0.25rem 0 0;
		color: #4b5563;
	}

	.analysis-form-card {
		margin-bottom: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.75rem;
	}

	.analysis-form-grid,
	.metrics-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.form-actions {
		margin-top: 1rem;
	}

	.loading-state,
	.score-panel,
	.empty-state {
		text-align: center;
	}

	.loading-state,
	.empty-state {
		padding-block: 3rem;
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		margin: 0 auto;
		border: 2px solid rgba(37, 99, 235, 0.18);
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.loading-text {
		margin: 1rem 0 0;
		color: #4b5563;
	}

	.results-stack {
		display: grid;
		gap: 1.5rem;
	}

	.score-label,
	.metric-label {
		margin: 0;
		color: #4b5563;
		font-size: 0.875rem;
	}

	.score-label {
		margin-bottom: 0.5rem;
	}

	.score-ring {
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 8rem;
		height: 8rem;
	}

	.score-ring__svg {
		width: 8rem;
		height: 8rem;
		transform: rotate(-90deg);
	}

	.score-ring__track {
		color: #e5e7eb;
	}

	.score-ring__progress--good,
	.score-number--good,
	.score-status--good {
		color: #16a34a;
	}

	.score-ring__progress--warning,
	.score-number--warning,
	.score-status--warning {
		color: #ca8a04;
	}

	.score-ring__progress--poor,
	.score-number--poor,
	.score-status--poor {
		color: #dc2626;
	}

	.score-ring__value {
		position: absolute;
		inset: 0;
		display: grid;
		place-content: center;
	}

	.score-number {
		margin: 0;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 2.25rem;
	}

	.score-total {
		margin: 0;
		color: #4b5563;
		font-size: 0.75rem;
	}

	.score-status {
		margin: 1rem 0 0;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.75rem;
	}

	.metric-value {
		margin: 0.25rem 0 0;
		color: #111827;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.metric-hint {
		margin: 0.25rem 0 0;
		color: #6b7280;
		font-size: 0.75rem;
	}

	.badge-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.insight-list {
		display: grid;
		gap: 0.75rem;
		margin: 0;
		padding: 0;
		list-style: none;
	}

	.insight-list--compact {
		gap: 0.5rem;
	}

	.insight-item {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.insight-item__icon {
		margin-top: 0.125rem;
		line-height: 1;
	}

	.insight-item__icon--warning {
		color: #eab308;
	}

	.insight-item__icon--success {
		color: #22c55e;
	}

	.insight-item__text {
		color: #374151;
	}

	.empty-state__icon {
		display: block;
		margin: 0 auto 1rem;
		color: #9ca3af;
	}

	.empty-state__text {
		margin: 0 auto;
		max-width: 34rem;
		color: #6b7280;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 768px) {
		.analysis-form-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.metrics-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.metrics-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}
</style>
