<!--
	Folder Tree Component
	═══════════════════════════════════════════════════════════════════════════
	
	Hierarchical folder navigation with drag-and-drop support.
-->

<script lang="ts">
	import { mediaStore as _mediaStore, getCurrentFolders } from '$lib/stores/media.svelte';
	import type { MediaFolder } from '$lib/api/media';
	import { Icon, IconChevronDown, IconChevronRight, IconFolder, IconPlus } from '$lib/icons';

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

	let expandedFolders = $state(new Set<string>());

	type FolderNode = MediaFolder & { children: FolderNode[] };

	// Build folder tree structure
	const currentFolders = $derived(getCurrentFolders());
	let folderTree = $derived(buildFolderTree(currentFolders));

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
		<button class="btn-add-folder" onclick={onCreateFolder} aria-label="Create folder">
			<Icon icon={IconPlus} size={18} />
		</button>
	</div>

	<!-- All Files -->
	<button
		class="folder-item"
		class:active={currentFolderId === null}
		onclick={() => selectFolder(null)}
	>
		<Icon icon={IconFolder} size={20} class="folder-icon" />
		<span class="folder-name">All Files</span>
	</button>

	<!-- Folder Tree -->
	<div class="folder-list">
		{#each folderTree as folder (folder.id)}
			{@const { level, isExpanded, isSelected, hasChildren } = renderFolder(folder)}
			<div class="folder-group">
				<div class="folder-item-wrapper">
					{#if hasChildren}
						<button
							class="expand-btn"
							onclick={() => toggleFolder(folder.id)}
							aria-label={isExpanded ? 'Collapse' : 'Expand'}
						>
							{#if isExpanded}
								<Icon icon={IconChevronDown} size={16} />
							{:else}
								<Icon icon={IconChevronRight} size={16} />
							{/if}
						</button>
					{:else}
						<span class="expand-spacer"></span>
					{/if}

					<button
						class="folder-item"
						class:active={isSelected}
						style="padding-left: {level * 1.5 + 0.75}rem"
						onclick={() => selectFolder(folder.id)}
					>
						<Icon icon={IconFolder} size={20} class="folder-icon" />
						<span class="folder-name">{folder.name}</span>
						<span class="folder-count">{folder.file_count}</span>
					</button>
				</div>

				{#if hasChildren && isExpanded && folder.children}
					{#each folder.children as child (child.id)}
						{@const childData = renderFolder(child, level + 1)}
						<div class="folder-item-wrapper">
							{#if childData.hasChildren}
								<button
									class="expand-btn"
									onclick={() => toggleFolder(child.id)}
									aria-label={childData.isExpanded ? 'Collapse' : 'Expand'}
								>
									{#if childData.isExpanded}
										<Icon icon={IconChevronDown} size={16} />
									{:else}
										<Icon icon={IconChevronRight} size={16} />
									{/if}
								</button>
							{:else}
								<span class="expand-spacer"></span>
							{/if}

							<button
								class="folder-item"
								class:active={childData.isSelected}
								style="padding-left: {childData.level * 1.5 + 0.75}rem"
								onclick={() => selectFolder(child.id)}
							>
								<Icon icon={IconFolder} size={20} class="folder-icon" />
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
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-4);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.folder-tree-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.folder-tree-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.btn-add-folder {
		padding: var(--space-2);
		color: oklch(0.65 0.01 250);
		border: none;
		cursor: pointer;
		border-radius: var(--radius-lg);
		transition:
			color var(--duration-fast) var(--ease-default),
			background-color var(--duration-fast) var(--ease-default);
		&:hover {
			color: oklch(0.8 0.18 90);
			background-color: oklch(0.38 0.01 250 / 50%);
		}
	}

	.folder-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.folder-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.folder-item {
		inline-size: 100%;
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-3);
		padding-block: var(--space-2);
		border-radius: var(--radius-lg);
		text-align: start;
		color: oklch(0.75 0.01 250);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.38 0.01 250 / 50%);
		}

		&.active {
			background-color: oklch(0.8 0.18 90 / 20%);
			color: oklch(0.8 0.18 90);
		}
	}

	.expand-btn {
		padding: 0.125rem;
		color: oklch(0.65 0.01 250);
		border: none;
		cursor: pointer;
		transition: color var(--duration-fast) var(--ease-default);
		&:hover {
			color: oklch(1 0 0);
		}
	}

	.expand-spacer {
		inline-size: 1.25rem;
	}

	.folder-item :global(.folder-icon) {
		flex-shrink: 0;
	}

	.folder-item.active :global(.folder-icon) {
		color: oklch(0.8 0.18 90);
	}

	.folder-item-wrapper {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.folder-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: var(--text-sm);
	}

	.folder-count {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
		background-color: oklch(0.38 0.01 250 / 50%);
		padding-inline: var(--space-2);
		padding-block: 0.125rem;
		border-radius: var(--radius-sm);
	}

	.folder-item.active .folder-count {
		background-color: oklch(0.8 0.18 90 / 20%);
		color: oklch(0.8 0.18 90);
	}
</style>
