<!--
	Folder Tree Component
	═══════════════════════════════════════════════════════════════════════════
	
	Hierarchical folder navigation with drag-and-drop support.
-->

<script lang="ts">
	import { mediaStore, currentFolders } from '$lib/stores/media';
	import type { MediaFolder } from '$lib/api/media';
	import { IconFolder, IconChevronRight, IconChevronDown, IconPlus } from '@tabler/icons-svelte';

	export let currentFolderId: string | null = null;
	export let onFolderSelect: (folderId: string | null) => void = () => {};
	export let onCreateFolder: () => void = () => {};

	let expandedFolders = new Set<string>();

	type FolderNode = MediaFolder & { children: FolderNode[] };

	// Build folder tree structure
	$: folderTree = buildFolderTree($currentFolders);

	function buildFolderTree(folders: MediaFolder[]): FolderNode[] {
		const tree: FolderNode[] = [];
		const map = new Map<string, FolderNode>();

		// Create map with children arrays
		folders.forEach((folder) => {
			map.set(folder.id, { ...folder, children: [] });
		});

		// Build tree
		folders.forEach((folder) => {
			const node = map.get(folder.id)!;
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
		expandedFolders = expandedFolders; // Trigger reactivity
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
		<button class="btn-add-folder" on:click={onCreateFolder} aria-label="Create folder">
			<IconPlus size={18} />
		</button>
	</div>

	<!-- All Files -->
	<button
		class="folder-item"
		class:active={currentFolderId === null}
		on:click={() => selectFolder(null)}
	>
		<IconFolder size={20} class="folder-icon" />
		<span class="folder-name">All Files</span>
	</button>

	<!-- Folder Tree -->
	<div class="folder-list">
		{#each folderTree as folder}
			{@const { level, isExpanded, isSelected, hasChildren } = renderFolder(folder)}
			<div class="folder-group">
				<div class="folder-item-wrapper">
					{#if hasChildren}
						<button
							class="expand-btn"
							on:click={() => toggleFolder(folder.id)}
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
						class="folder-item"
						class:active={isSelected}
						style="padding-left: {level * 1.5 + 0.75}rem"
						on:click={() => selectFolder(folder.id)}
					>
						<IconFolder size={20} class="folder-icon" />
						<span class="folder-name">{folder.name}</span>
						<span class="folder-count">{folder.file_count}</span>
					</button>
				</div>

				{#if hasChildren && isExpanded && folder.children}
					{#each folder.children as child}
						{@const childData = renderFolder(child, level + 1)}
						<div class="folder-item-wrapper">
							{#if childData.hasChildren}
								<button
									class="expand-btn"
									on:click={() => toggleFolder(child.id)}
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
								class="folder-item"
								class:active={childData.isSelected}
								style="padding-left: {childData.level * 1.5 + 0.75}rem"
								on:click={() => selectFolder(child.id)}
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

<style lang="postcss">
	.folder-tree {
		@apply bg-gray-800/50 rounded-xl p-4 border border-gray-700/50;
	}

	.folder-tree-header {
		@apply flex items-center justify-between mb-4;
	}

	.folder-tree-title {
		@apply text-lg font-semibold text-white;
	}

	.btn-add-folder {
		@apply p-2 text-gray-400 hover:text-yellow-400 hover:bg-gray-700/50 rounded-lg transition-colors;
	}

	.folder-list {
		@apply space-y-1;
	}

	.folder-group {
		@apply space-y-1;
	}

	.folder-item {
		@apply w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left;
		@apply text-gray-300 hover:bg-gray-700/50 transition-colors;
	}

	.folder-item.active {
		@apply bg-yellow-500/20 text-yellow-400;
	}

	.expand-btn {
		@apply p-0.5 text-gray-400 hover:text-white transition-colors;
	}

	.expand-spacer {
		@apply w-5;
	}

	.folder-item :global(.folder-icon) {
		@apply flex-shrink-0;
	}

	.folder-item.active :global(.folder-icon) {
		@apply text-yellow-400;
	}

	.folder-item-wrapper {
		@apply flex items-center gap-2;
	}

	.folder-name {
		@apply flex-1 truncate text-sm;
	}

	.folder-count {
		@apply text-xs text-gray-500 bg-gray-700/50 px-2 py-0.5 rounded;
	}

	.folder-item.active .folder-count {
		@apply bg-yellow-500/20 text-yellow-400;
	}
</style>
