<script lang="ts">
	import { onMount } from 'svelte';
	import { IconSearch, IconFileText, IconEye } from '#lib/icons/index.js';
	import SeoMetaEditor from '#lib/components/seo/SeoMetaEditor.svelte';
	import type { ComponentProps } from 'svelte';

	type SeoEntity = ComponentProps<typeof SeoMetaEditor>['entity'] & {
		seo_score?: number;
	};

	let entities: SeoEntity[] = $state([]);
	let selectedEntity: SeoEntity | null = $state(null);
	let loading = $state(false);
	let searchQuery = $state('');

	onMount(() => {
		loadEntities();
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
				<input autocomplete="off"
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
					{#each filteredEntities as entity (entity.id)}
						<button
							class={['entity-item', { active: selectedEntity?.id === entity.id }]}
							onclick={() => (selectedEntity = entity)}
						>
							<div class="entity-info">
								<h4>{entity.title}</h4>
								<span class="entity-type">{entity.type}</span>
							</div>
							{#if entity.seo_score}
								<div
									class={[
										'score-badge',
										{
											good: entity.seo_score >= 70,
											warning: entity.seo_score >= 40 && entity.seo_score < 70,
											poor: entity.seo_score < 40
										}
									]}
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
		background: var(--bg-elevated, #161b22);
	}

	.page-header {
		padding: 2rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: var(--text-secondary, #8b949e);
		font-size: 0.95rem;
	}

	.page-content {
		display: grid;
		grid-template-columns: 320px 1fr;
		flex: 1;
		overflow: hidden;
	}

	.entities-sidebar {
		background: var(--bg-elevated, #161b22);
		border-right: 1px solid var(--border-default, #30363d);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.95rem;
		color: var(--text-primary, #f0f6fc);
	}

	.search-box input::placeholder {
		color: var(--text-tertiary, #6e7681);
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
		border-bottom: 1px solid var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		cursor: pointer;
		transition: background 0.2s;
		text-align: left;
	}

	.entity-item:hover {
		background: var(--bg-elevated, #161b22);
	}

	.entity-item.active {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		border-left: 3px solid var(--primary-500, #e6b800);
	}

	.entity-info h4 {
		font-size: 0.95rem;
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.25rem;
	}

	.entity-type {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
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
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.score-badge.warning {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}

	.score-badge.poor {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
		color: var(--error-emphasis, #f85149);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1.5rem;
		text-align: center;
		color: var(--text-tertiary, #6e7681);
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
		background: var(--bg-elevated, #161b22);
	}

	.no-selection {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}

	.no-selection h3 {
		margin: 1rem 0 0.5rem;
		color: var(--text-secondary, #8b949e);
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
		color: var(--text-tertiary, #6e7681);
	}

	@media (max-width: 1023.98px) {
		.page-content {
			grid-template-columns: 280px 1fr;
		}
	}

	@media (max-width: 767.98px) {
		.page-content {
			grid-template-columns: 1fr;
		}

		.entities-sidebar {
			display: none;
		}
	}
</style>
