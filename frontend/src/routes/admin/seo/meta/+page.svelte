<script lang="ts">
	import { browser } from '$app/environment';
	import { IconSearch, IconFileText, IconEye, IconDeviceFloppy, IconRefresh } from '$lib/icons';
	import SeoMetaEditor from '$lib/components/seo/SeoMetaEditor.svelte';
	import SeoAnalyzer from '$lib/components/seo/SeoAnalyzer.svelte';

	let entities: any[] = $state([]);
	let selectedEntity: any = $state(null);
	let loading = $state(false);
	let searchQuery = $state('');
	let initialized = $state(false);

	$effect(() => {
		if (browser && !initialized) {
			initialized = true;
			loadEntities();
		}
	});

	async function loadEntities() {
		// This would load entities (pages, posts, products, etc.)
		// For now, we'll just create a placeholder
		entities = [];
	}

	let filteredEntities = $derived(
		entities.filter((entity) => entity.title.toLowerCase().includes(searchQuery.toLowerCase()))
	);
</script>

<svelte:head>
	<title>SEO Meta Manager | Admin</title>
</svelte:head>

<div class="seo-meta-page">
	<header class="page-header">
		<div>
			<h1>SEO Meta Manager</h1>
			<p>Manage meta tags, Open Graph, and Twitter Cards for your content</p>
		</div>
	</header>

	<div class="page-content">
		<div class="entities-sidebar">
			<div class="search-box">
				<IconSearch size={20} />
				<label for="search-entities" class="sr-only">Search entities</label>
				<input
					type="text"
					id="search-entities"
					name="search"
					bind:value={searchQuery}
					placeholder="Search entities..."
				/>
			</div>

			<div class="entities-list">
				{#if loading}
					<div class="loading">Loading...</div>
				{:else if filteredEntities.length === 0}
					<div class="empty-state">
						<IconFileText size={48} />
						<p>No content found</p>
						<p class="hint">Create pages, posts, or products to optimize their SEO</p>
					</div>
				{:else}
					{#each filteredEntities as entity}
						<button
							class="entity-item"
							class:active={selectedEntity?.id === entity.id}
							onclick={() => (selectedEntity = entity)}
						>
							<div class="entity-info">
								<h4>{entity.title}</h4>
								<span class="entity-type">{entity.type}</span>
							</div>
							{#if entity.seo_score}
								<div
									class="score-badge"
									class:good={entity.seo_score >= 70}
									class:warning={entity.seo_score >= 40 && entity.seo_score < 70}
									class:poor={entity.seo_score < 40}
								>
									{entity.seo_score}
								</div>
							{/if}
						</button>
					{/each}
				{/if}
			</div>
		</div>

		<div class="editor-area">
			{#if selectedEntity}
				<SeoMetaEditor entity={selectedEntity} />
			{:else}
				<div class="no-selection">
					<IconEye size={64} />
					<h3>Select Content to Optimize</h3>
					<p>Choose a page, post, or product from the left sidebar to manage its SEO settings</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.seo-meta-page {
		display: flex;
		flex-direction: column;
		background: #f8f9fa;
	}

	.page-header {
		padding: 2rem;
		background: white;
		border-bottom: 1px solid #e5e5e5;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.page-content {
		display: grid;
		grid-template-columns: 320px 1fr;
		flex: 1;
		overflow: hidden;
	}

	.entities-sidebar {
		background: white;
		border-right: 1px solid #e5e5e5;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.95rem;
		color: #1a1a1a;
	}

	.search-box input::placeholder {
		color: #999;
	}

	.entities-list {
		flex: 1;
		overflow-y: auto;
	}

	.entity-item {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border: none;
		border-bottom: 1px solid #f0f0f0;
		background: white;
		cursor: pointer;
		transition: background 0.2s;
		text-align: left;
	}

	.entity-item:hover {
		background: #f8f9fa;
	}

	.entity-item.active {
		background: #eff6ff;
		border-left: 3px solid #3b82f6;
	}

	.entity-info h4 {
		font-size: 0.95rem;
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.25rem;
	}

	.entity-type {
		font-size: 0.8rem;
		color: #999;
		text-transform: capitalize;
	}

	.score-badge {
		width: 42px;
		height: 42px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		font-size: 0.9rem;
	}

	.score-badge.good {
		background: #dcfce7;
		color: #16a34a;
	}

	.score-badge.warning {
		background: #fef3c7;
		color: #d97706;
	}

	.score-badge.poor {
		background: #fee2e2;
		color: #dc2626;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1.5rem;
		text-align: center;
		color: #999;
	}

	.empty-state p {
		margin: 0.5rem 0 0;
		font-size: 0.95rem;
	}

	.empty-state .hint {
		font-size: 0.85rem;
		color: #bbb;
	}

	.editor-area {
		overflow-y: auto;
		background: #f8f9fa;
	}

	.no-selection {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #999;
		text-align: center;
	}

	.no-selection h3 {
		margin: 1rem 0 0.5rem;
		color: #666;
		font-weight: 500;
	}

	.no-selection p {
		max-width: 400px;
		font-size: 0.95rem;
	}

	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		color: #999;
	}

	@media (max-width: 1024px) {
		.page-content {
			grid-template-columns: 280px 1fr;
		}
	}

	@media (max-width: 768px) {
		.page-content {
			grid-template-columns: 1fr;
		}

		.entities-sidebar {
			display: none;
		}
	}
</style>
