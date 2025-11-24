<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Input, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { seoApi, type SeoAnalysis } from '$lib/api/seo';
	import { getForms, type Form } from '$lib/api/forms';
	import { IconSearch, IconRefresh, IconChartBar } from '@tabler/icons-svelte';

	let contentType = 'posts';
	let contentId = '';
	let focusKeyword = '';
	let analysis: Partial<SeoAnalysis> | null = null;
	let loading = false;
	let analyzing = false;

	// For content selector
	let forms: Form[] = [];
	let selectedContent: Record<string, unknown> | null = null;

	const contentTypes = [
		{ value: 'posts', label: 'Blog Posts' },
		{ value: 'products', label: 'Products' },
		{ value: 'forms', label: 'Forms' }
	];

	onMount(async () => {
		await loadForms();
	});

	async function loadForms() {
		try {
			const response = await getForms();
			forms = response.forms || [];
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
			const response = await seoApi.analyze(
				contentType,
				parseInt(contentId),
				focusKeyword || undefined
			);
			analysis = response;
			addToast({ type: 'success', message: 'SEO analysis completed!' });
		} catch (error) {
			addToast({ type: 'error', message: 'Failed to analyze content' });
		} finally {
			analyzing = false;
		}
	}

	async function loadAnalysis() {
		if (!contentId) return;

		try {
			loading = true;
			const response = await seoApi.getAnalysis(contentType, parseInt(contentId));
			analysis = response;
		} catch (error) {
			analysis = null;
		} finally {
			loading = false;
		}
	}

	function getScoreColor(score: number): string {
		if (score >= 80) return 'text-green-600';
		if (score >= 60) return 'text-yellow-600';
		return 'text-red-600';
	}

	function getScoreLabel(score: number): string {
		if (score >= 80) return 'Good';
		if (score >= 60) return 'Needs Improvement';
		return 'Poor';
	}

	$: if (contentId) {
		loadAnalysis();
	}
</script>

<svelte:head>
	<title>SEO Analysis | Revolution Admin</title>
</svelte:head>

<div>
	<!-- Header -->
	<div class="mb-6">
		<h1 class="text-3xl font-bold text-gray-900">SEO Analysis</h1>
		<p class="text-gray-600 mt-1">Analyze and optimize your content for search engines</p>
	</div>

	<!-- Analysis Form -->
	<Card class="mb-6">
		<h2 class="text-xl font-bold text-gray-900 mb-4">Analyze Content</h2>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
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

		<div class="mt-4">
			<Button on:click={handleAnalyze} loading={analyzing} disabled={!contentId}>
				<IconSearch size={20} />
				Analyze SEO
			</Button>
		</div>
	</Card>

	<!-- Analysis Results -->
	{#if loading}
		<Card>
			<div class="text-center py-12">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
				<p class="mt-4 text-gray-600">Loading analysis...</p>
			</div>
		</Card>
	{:else if analysis}
		<div class="space-y-6">
			<!-- Score Card -->
			<Card>
				<div class="text-center">
					<p class="text-sm text-gray-600 mb-2">SEO Score</p>
					<div class="relative inline-flex items-center justify-center w-32 h-32">
						<svg class="transform -rotate-90 w-32 h-32">
							<circle
								cx="64"
								cy="64"
								r="56"
								stroke="currentColor"
								stroke-width="8"
								fill="none"
								class="text-gray-200"
							/>
							<circle
								cx="64"
								cy="64"
								r="56"
								stroke="currentColor"
								stroke-width="8"
								fill="none"
								class={getScoreColor(analysis.seo_score)}
								stroke-dasharray={`${(analysis.seo_score / 100) * 351.86} 351.86`}
								stroke-linecap="round"
							/>
						</svg>
						<div class="absolute">
							<p class="text-3xl font-bold {getScoreColor(analysis.seo_score)}">
								{analysis.seo_score}
							</p>
							<p class="text-xs text-gray-600">/ 100</p>
						</div>
					</div>
					<p class="mt-4 text-lg font-semibold {getScoreColor(analysis.seo_score)}">
						{getScoreLabel(analysis.seo_score)}
					</p>
				</div>
			</Card>

			<!-- Metrics Grid -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				<Card>
					<p class="text-sm text-gray-600">Keyword Density</p>
					<p class="text-2xl font-bold mt-1">{analysis.keyword_density.toFixed(2)}%</p>
					<p class="text-xs text-gray-500 mt-1">Optimal: 0.5-2.5%</p>
				</Card>

				<Card>
					<p class="text-sm text-gray-600">Readability Score</p>
					<p class="text-2xl font-bold mt-1">{analysis.readability_score}</p>
					<p class="text-xs text-gray-500 mt-1">Higher is better</p>
				</Card>

				<Card>
					<p class="text-sm text-gray-600">Meta Tags</p>
					<div class="flex gap-2 mt-2">
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
			{#if analysis.suggestions.length > 0}
				<Card>
					<h3 class="text-lg font-bold text-gray-900 mb-4">Improvement Suggestions</h3>
					<ul class="space-y-3">
						{#each analysis.suggestions as suggestion}
							<li class="flex items-start gap-3">
								<span class="text-yellow-500 mt-1">⚠</span>
								<span class="text-gray-700">{suggestion}</span>
							</li>
						{/each}
					</ul>
				</Card>
			{/if}

			<!-- Analysis Results -->
			{#if analysis.analysis_results.length > 0}
				<Card>
					<h3 class="text-lg font-bold text-gray-900 mb-4">Analysis Details</h3>
					<ul class="space-y-2">
						{#each analysis.analysis_results as result}
							<li class="flex items-start gap-3">
								<span class="text-green-500 mt-1">✓</span>
								<span class="text-gray-700">{result}</span>
							</li>
						{/each}
					</ul>
				</Card>
			{/if}
		</div>
	{:else}
		<Card>
			<div class="text-center py-12">
				<IconChartBar size={48} class="mx-auto text-gray-400 mb-4" />
				<p class="text-gray-500">
					No analysis available. Select content and click "Analyze SEO" to get started.
				</p>
			</div>
		</Card>
	{/if}
</div>
