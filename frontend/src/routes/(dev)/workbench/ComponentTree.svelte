<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Component Tree - Searchable Sidebar
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays all components organized by folder with search and favorites
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 */
	import type { ComponentInfo, ComponentTree } from './+page.server';

	interface Props {
		tree: ComponentTree;
		selectedComponent: string | null;
		onSelect: (component: ComponentInfo) => void;
	}

	let { tree, selectedComponent, onSelect }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let expandedFolders = $state<Set<string>>(new Set(['root']));
	let favorites = $state<Set<string>>(new Set());
	let recentComponents = $state<string[]>([]);

	// Load favorites and recent from localStorage
	$effect(() => {
		if (typeof window !== 'undefined') {
			const savedFavorites = localStorage.getItem('workbench-favorites');
			const savedRecent = localStorage.getItem('workbench-recent');

			if (savedFavorites) {
				favorites = new Set(JSON.parse(savedFavorites));
			}
			if (savedRecent) {
				recentComponents = JSON.parse(savedRecent);
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredTree = $derived.by(() => {
		if (!searchQuery.trim()) return tree;

		const query = searchQuery.toLowerCase();
		const result: ComponentTree = {};

		for (const [folder, components] of Object.entries(tree)) {
			const filtered = components.filter(
				(c) => c.name.toLowerCase().includes(query) || c.relativePath.toLowerCase().includes(query)
			);

			if (filtered.length > 0) {
				result[folder] = filtered;
			}
		}

		return result;
	});

	const totalFiltered = $derived(
		Object.values(filteredTree).reduce((sum, arr) => sum + arr.length, 0)
	);

	const favoriteComponents = $derived.by(() => {
		const favs: ComponentInfo[] = [];
		for (const components of Object.values(tree)) {
			for (const comp of components) {
				if (favorites.has(comp.relativePath)) {
					favs.push(comp);
				}
			}
		}
		return favs;
	});

	const recentComponentsInfo = $derived.by(() => {
		const recent: ComponentInfo[] = [];
		for (const path of recentComponents.slice(0, 5)) {
			for (const components of Object.values(tree)) {
				const found = components.find((c) => c.relativePath === path);
				if (found) {
					recent.push(found);
					break;
				}
			}
		}
		return recent;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleFolder(folder: string) {
		if (expandedFolders.has(folder)) {
			expandedFolders.delete(folder);
		} else {
			expandedFolders.add(folder);
		}
		expandedFolders = new Set(expandedFolders);
	}

	function handleSelect(component: ComponentInfo) {
		// Add to recent
		recentComponents = [
			component.relativePath,
			...recentComponents.filter((p) => p !== component.relativePath)
		].slice(0, 10);

		if (typeof window !== 'undefined') {
			localStorage.setItem('workbench-recent', JSON.stringify(recentComponents));
		}

		onSelect(component);
	}

	function toggleFavorite(component: ComponentInfo, event: MouseEvent | KeyboardEvent) {
		event.stopPropagation();

		if (favorites.has(component.relativePath)) {
			favorites.delete(component.relativePath);
		} else {
			favorites.add(component.relativePath);
		}
		favorites = new Set(favorites);

		if (typeof window !== 'undefined') {
			localStorage.setItem('workbench-favorites', JSON.stringify([...favorites]));
		}
	}

	function formatSize(bytes: number): string {
		if (bytes < 1024) return `${bytes}B`;
		return `${(bytes / 1024).toFixed(1)}KB`;
	}
</script>

<aside class="tree-container">
	<!-- Search -->
	<div class="search-box">
		<input
			type="search"
			placeholder="Search components..."
			bind:value={searchQuery}
			class="search-input"
		/>
		<span class="search-count">{totalFiltered}</span>
	</div>

	<!-- Favorites Section -->
	{#if favoriteComponents.length > 0}
		<section class="tree-section">
			<h3 class="section-title">
				<span class="section-icon">⭐</span>
				Favorites
			</h3>
			<ul class="component-list">
				{#each favoriteComponents as component}
					<li>
						<button
							class="component-item"
							class:selected={selectedComponent === component.relativePath}
							onclick={() => handleSelect(component)}
						>
							<span class="component-name">{component.name}</span>
							<span class="component-props">{component.props.length} props</span>
						</button>
					</li>
				{/each}
			</ul>
		</section>
	{/if}

	<!-- Recent Section -->
	{#if recentComponentsInfo.length > 0 && !searchQuery}
		<section class="tree-section">
			<h3 class="section-title">
				<span class="section-icon">🕐</span>
				Recent
			</h3>
			<ul class="component-list">
				{#each recentComponentsInfo as component}
					<li>
						<button
							class="component-item"
							class:selected={selectedComponent === component.relativePath}
							onclick={() => handleSelect(component)}
						>
							<span class="component-name">{component.name}</span>
							<span class="component-folder">{component.folder}</span>
						</button>
					</li>
				{/each}
			</ul>
		</section>
	{/if}

	<!-- Component Tree -->
	<section class="tree-section tree-section--main">
		<h3 class="section-title">
			<span class="section-icon">📦</span>
			Components
		</h3>

		{#each Object.entries(filteredTree) as [folder, components]}
			<div class="folder">
				<button class="folder-header" onclick={() => toggleFolder(folder)}>
					<span class="folder-icon">
						{expandedFolders.has(folder) ? '📂' : '📁'}
					</span>
					<span class="folder-name">{folder === 'root' ? 'Root' : folder}</span>
					<span class="folder-count">{components.length}</span>
				</button>

				{#if expandedFolders.has(folder)}
					<ul class="component-list">
						{#each components as component}
							<li>
								<button
									class="component-item"
									class:selected={selectedComponent === component.relativePath}
									onclick={() => handleSelect(component)}
								>
									<span
										class="favorite-btn"
										class:active={favorites.has(component.relativePath)}
										onclick={(e) => toggleFavorite(component, e)}
										onkeydown={(e) => {
											if (e.key === 'Enter' || e.key === ' ') {
												e.preventDefault();
												toggleFavorite(component, e);
											}
										}}
										role="button"
										tabindex="0"
										aria-label={favorites.has(component.relativePath)
											? 'Remove from favorites'
											: 'Add to favorites'}
									>
										{favorites.has(component.relativePath) ? '⭐' : '☆'}
									</span>
									<span class="component-name">{component.name}</span>
									<span class="component-meta">
										{#if component.hasSnippets}
											<span class="meta-badge" title="Has children">👶</span>
										{/if}
										<span class="meta-props">{component.props.length}</span>
										<span class="meta-size">{formatSize(component.size)}</span>
									</span>
								</button>
							</li>
						{/each}
					</ul>
				{/if}
			</div>
		{/each}
	</section>
</aside>

<style>
	.tree-container {
		width: 280px;
		min-width: 280px;
		height: 100%;
		overflow-y: auto;
		background: #111;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		flex-direction: column;
	}

	.search-box {
		position: sticky;
		top: 0;
		z-index: 10;
		padding: 0.75rem;
		background: #111;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.search-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #fafafa;
		font-size: 0.875rem;
	}

	.search-input:focus {
		outline: none;
		border-color: rgba(139, 92, 246, 0.5);
	}

	.search-count {
		padding: 0.25rem 0.5rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		font-size: 0.75rem;
		color: #a1a1aa;
	}

	.tree-section {
		padding: 0.75rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.tree-section--main {
		flex: 1;
		border-bottom: none;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0 0 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #71717a;
	}

	.section-icon {
		font-size: 0.875rem;
	}

	.folder {
		margin-bottom: 0.25rem;
	}

	.folder-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #fafafa;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s;
	}

	.folder-header:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.folder-icon {
		font-size: 1rem;
	}

	.folder-name {
		flex: 1;
		text-align: left;
	}

	.folder-count {
		padding: 0.125rem 0.375rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 9999px;
		font-size: 0.625rem;
		color: #a1a1aa;
	}

	.component-list {
		list-style: none;
		margin: 0;
		padding: 0 0 0 1rem;
	}

	.component-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.375rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		color: #a1a1aa;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.component-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fafafa;
	}

	.component-item.selected {
		background: rgba(139, 92, 246, 0.2);
		color: #c4b5fd;
	}

	.favorite-btn {
		padding: 0;
		background: none;
		border: none;
		font-size: 0.75rem;
		cursor: pointer;
		opacity: 0.3;
		transition: opacity 0.15s;
	}

	.favorite-btn:hover,
	.favorite-btn.active {
		opacity: 1;
	}

	.component-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.component-meta {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.625rem;
		color: #52525b;
	}

	.meta-badge {
		font-size: 0.625rem;
	}

	.meta-props,
	.meta-size {
		padding: 0.0625rem 0.25rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 0.125rem;
	}

	.component-props,
	.component-folder {
		font-size: 0.625rem;
		color: #52525b;
	}
</style>
