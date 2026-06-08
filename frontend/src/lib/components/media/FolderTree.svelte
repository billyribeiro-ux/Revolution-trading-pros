<!--
	Folder Tree Component
	═══════════════════════════════════════════════════════════════════════════

	Hierarchical folder navigation with drag-and-drop support.
-->

<script lang="ts">
	import { mediaStore as _mediaStore, getCurrentFolders } from '$lib/stores/media.svelte';
	import type { MediaFolder } from '$lib/api/media';
	import { IconFolder, IconChevronRight, IconChevronDown, IconPlus } from '$lib/icons';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';

	interface Props {
		currentFolderId?: string | null;
		onFolderSelect?: (folderId: string | null) => void;
		onCreateFolder?: () => void;
	}

	let {
		currentFolderId = null,
		onFolderSelect = () => {},
		onCreateFolder = () => {}
	}: Props = $props();

	let expandedFolders = new SvelteSet<string>();

	type FolderNode = MediaFolder & { children: FolderNode[] };

	// Build folder tree structure
	const currentFolders = $derived(getCurrentFolders());
	let folderTree = $derived(buildFolderTree(currentFolders));

	function buildFolderTree(folders: MediaFolder[]): FolderNode[] {
		const tree: FolderNode[] = [];
		const map = new SvelteMap<string, FolderNode>();

		// Create map with children arrays
		folders.forEach((folder) => {
			map.set(folder.id, { ...folder, children: [] });
		});

		// Build tree
		folders.forEach((folder) => {
			const node = map.get(folder.id);
			if (!node) return;
			if (folder.parent_id) {
				const parent = map.get(folder.parent_id);
				if (parent) {
					parent.children.push(node);
				}
			} else {
				tree.push(node);
			}
		});

		return tree;
	}

	function toggleFolder(folderId: string) {
		if (expandedFolders.has(folderId)) {
			expandedFolders.delete(folderId);
		} else {
			expandedFolders.add(folderId);
		}
	}

	function selectFolder(folderId: string | null) {
		currentFolderId = folderId;
		onFolderSelect(folderId);
	}

	function renderFolder(folder: FolderNode, level: number = 0) {
		const isExpanded = expandedFolders.has(folder.id);
		const isSelected = currentFolderId === folder.id;
		const hasChildren = folder.children.length > 0;

		return { folder, level, isExpanded, isSelected, hasChildren };
	}
</script>

<div class="folder-tree">
	<div class="folder-tree-header">
		<h3 class="folder-tree-title">Folders</h3>
		<button class="btn-add-folder" onclick={onCreateFolder} aria-label="Create folder">
			<IconPlus size={18} />
		</button>
	</div>

	<!-- All Files -->
	<button
		class={['folder-item', { active: currentFolderId === null }]}
		onclick={() => selectFolder(null)}
	>
		<IconFolder size={20} class="folder-icon" />
		<span class="folder-name">All Files</span>
	</button>

	<!-- Folder Tree -->
	<div class="folder-list">
		{#each folderTree as folder (folder.id)}
			{const { level, isExpanded, isSelected, hasChildren } = renderFolder(folder)}
			<div class="folder-group">
				<div class="folder-item-wrapper">
					{#if hasChildren}
						<button
							class="expand-btn"
							onclick={() => toggleFolder(folder.id)}
							aria-label={isExpanded ? 'Collapse' : 'Expand'}
						>
							{#if isExpanded}
								<IconChevronDown size={16} />
							{:else}
								<IconChevronRight size={16} />
							{/if}
						</button>
					{:else}
						<span class="expand-spacer"></span>
					{/if}

					<button
						class={['folder-item', { active: isSelected }]}
						style:padding-left={`${level * 1.5 + 0.75}rem`}
						onclick={() => selectFolder(folder.id)}
					>
						<IconFolder size={20} class="folder-icon" />
						<span class="folder-name">{folder.name}</span>
						<span class="folder-count">{folder.file_count}</span>
					</button>
				</div>

				{#if hasChildren && isExpanded && folder.children}
					{#each folder.children as child (child.id)}
						{const childData = renderFolder(child, level + 1)}
						<div class="folder-item-wrapper">
							{#if childData.hasChildren}
								<button
									class="expand-btn"
									onclick={() => toggleFolder(child.id)}
									aria-label={childData.isExpanded ? 'Collapse' : 'Expand'}
								>
									{#if childData.isExpanded}
										<IconChevronDown size={16} />
									{:else}
										<IconChevronRight size={16} />
									{/if}
								</button>
							{:else}
								<span class="expand-spacer"></span>
							{/if}

							<button
								class={['folder-item', { active: childData.isSelected }]}
								style:padding-left={`${childData.level * 1.5 + 0.75}rem`}
								onclick={() => selectFolder(child.id)}
							>
								<IconFolder size={20} class="folder-icon" />
								<span class="folder-name">{child.name}</span>
								<span class="folder-count">{child.file_count}</span>
							</button>
						</div>
					{/each}
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.folder-tree {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.folder-tree-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.folder-tree-title {
		margin: 0;
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.btn-add-folder {
		padding: 0.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.btn-add-folder:hover {
		background: rgba(55, 65, 81, 0.5);
		color: #facc15;
	}

	.folder-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.folder-group {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.folder-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #d1d5db;
		font: inherit;
		text-align: left;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.folder-item:hover {
		background: rgba(55, 65, 81, 0.5);
	}

	.folder-item.active {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}

	.expand-btn {
		padding: 0.125rem;
		border: 0;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.expand-btn:hover {
		color: #ffffff;
	}

	.expand-spacer {
		width: 1.25rem;
		flex-shrink: 0;
	}

	.folder-item :global(.folder-icon) {
		flex-shrink: 0;
	}

	.folder-item.active :global(.folder-icon) {
		color: #facc15;
	}

	.folder-item-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.folder-name {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		font-size: 0.875rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.folder-count {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		background: rgba(55, 65, 81, 0.5);
		color: #6b7280;
		font-size: 0.75rem;
		white-space: nowrap;
	}

	.folder-item.active .folder-count {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}
</style>
