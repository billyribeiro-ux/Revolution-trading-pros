<script lang="ts">
	/**
	 * Related Forms - Display related form suggestions
	 *
	 * Features:
	 * - Similar forms by content
	 * - Category-based suggestions
	 * - Trending forms
	 * - Sequential form chains
	 *
	 * @version 1.0.0
	 */

	import { onMount } from 'svelte';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	interface Props {
		formId: number;
		limit?: number;
		showTrending?: boolean;
	}

	interface RelatedForm {
		id: number;
		title: string;
		slug: string;
		description: string | null;
		category: string | null;
		score?: number;
		relation_type?: string;
	}

	let { formId, limit = 5, showTrending = false }: Props = $props();

	// State
	let relatedForms = $state<RelatedForm[]>([]);
	let trendingForms = $state<RelatedForm[]>([]);
	let loading = $state(true);
	let activeTab = $state<'related' | 'trending'>('related');

	// Fetch related forms
	async function fetchRelatedForms() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/related?limit=${limit}`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				relatedForms = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch related forms:', error);
		}
	}

	// Fetch trending forms
	async function fetchTrendingForms() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/trending?limit=${limit}`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				trendingForms = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch trending forms:', error);
		}
	}

	// Get relation badge
	function getRelationBadge(type: string | undefined): string {
		switch (type) {
			case 'similar':
				return 'Similar';
			case 'sequel':
				return 'Next Step';
			case 'category':
				return 'Same Category';
			default:
				return 'Related';
		}
	}

	// Get relation color
	function getRelationColor(type: string | undefined): string {
		switch (type) {
			case 'similar':
				return '#3b82f6';
			case 'sequel':
				return '#10b981';
			case 'category':
				return '#8b5cf6';
			default:
				return '#6b7280';
		}
	}

	// Initial load
	onMount(async () => {
		loading = true;
		await Promise.all([
			fetchRelatedForms(),
			showTrending ? fetchTrendingForms() : Promise.resolve()
		]);
		loading = false;
	});
</script>

<div class="related-forms">
	<div class="panel-header">
		<h3>Discover More</h3>
	</div>

	{#if showTrending}
		<div class="tabs">
			<button
				class="tab"
				class:active={activeTab === 'related'}
				onclick={() => (activeTab = 'related')}
			>
				Related
			</button>
			<button
				class="tab"
				class:active={activeTab === 'trending'}
				onclick={() => (activeTab = 'trending')}
			>
				Trending
			</button>
		</div>
	{/if}

	<div class="panel-content">
		{#if loading}
			<div class="loading">
				<div class="loading-spinner"></div>
				<span>Finding related forms...</span>
			</div>
		{:else if activeTab === 'related'}
			{#if relatedForms.length > 0}
				<div class="forms-list">
					{#each relatedForms as form}
						<a href="/forms/{form.slug}" class="form-card">
							<div class="form-header">
								<span
									class="relation-badge"
									style="background-color: {getRelationColor(form.relation_type)}"
								>
									{getRelationBadge(form.relation_type)}
								</span>
								{#if form.score}
									<span class="score-badge" title="Relevance score">
										{Math.round((form.score || 0) * 100)}%
									</span>
								{/if}
							</div>
							<h4 class="form-title">{form.title}</h4>
							{#if form.description}
								<p class="form-description">{form.description.slice(0, 80)}...</p>
							{/if}
							{#if form.category}
								<span class="category-tag">{form.category}</span>
							{/if}
						</a>
					{/each}
				</div>
			{:else}
				<div class="empty-state">
					<span class="empty-icon">🔍</span>
					<p>No related forms found</p>
				</div>
			{/if}
		{:else if activeTab === 'trending'}
			{#if trendingForms.length > 0}
				<div class="forms-list">
					{#each trendingForms as form, index}
						<a href="/forms/{form.slug}" class="form-card trending">
							<div class="rank-badge">#{index + 1}</div>
							<div class="form-info">
								<h4 class="form-title">{form.title}</h4>
								{#if form.description}
									<p class="form-description">{form.description.slice(0, 60)}...</p>
								{/if}
							</div>
							<div class="trending-indicator">
								<span class="trending-icon">🔥</span>
							</div>
						</a>
					{/each}
				</div>
			{:else}
				<div class="empty-state">
					<span class="empty-icon">📊</span>
					<p>No trending forms yet</p>
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.related-forms {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.panel-header {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.panel-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid #e5e7eb;
	}

	.tab {
		flex: 1;
		padding: 0.625rem;
		background: none;
		border: none;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #374151;
	}

	.tab.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
	}

	.panel-content {
		padding: 1rem;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 2rem;
		color: #6b7280;
	}

	.loading-spinner {
		width: 24px;
		height: 24px;
		border: 2px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.forms-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.form-card {
		display: block;
		padding: 0.875rem;
		background: #f9fafb;
		border-radius: 0.5rem;
		border: 1px solid #e5e7eb;
		text-decoration: none;
		transition: all 0.2s;
	}

	.form-card:hover {
		border-color: #3b82f6;
		background: #eff6ff;
		transform: translateY(-1px);
	}

	.form-card.trending {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.form-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.relation-badge {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 600;
		color: white;
		text-transform: uppercase;
	}

	.score-badge {
		padding: 0.125rem 0.375rem;
		background: #f3f4f6;
		border-radius: 0.25rem;
		font-size: 0.625rem;
		color: #6b7280;
	}

	.rank-badge {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
		color: white;
		font-size: 0.75rem;
		font-weight: 700;
		border-radius: 0.25rem;
		flex-shrink: 0;
	}

	.form-info {
		flex: 1;
		min-width: 0;
	}

	.form-title {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: #111827;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.form-description {
		margin: 0.25rem 0 0 0;
		font-size: 0.75rem;
		color: #6b7280;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.category-tag {
		display: inline-block;
		margin-top: 0.5rem;
		padding: 0.125rem 0.5rem;
		background: #e0e7ff;
		color: #4338ca;
		border-radius: 9999px;
		font-size: 0.625rem;
		font-weight: 500;
	}

	.trending-indicator {
		flex-shrink: 0;
	}

	.trending-icon {
		font-size: 1.25rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem;
		color: #9ca3af;
	}

	.empty-icon {
		font-size: 2rem;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		margin: 0;
		font-size: 0.875rem;
	}
</style>
