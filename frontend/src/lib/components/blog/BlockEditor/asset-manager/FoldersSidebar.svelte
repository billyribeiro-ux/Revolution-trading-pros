<!--
/**
 * FoldersSidebar - Left sidebar listing root-level folders
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no binds.
 * Note: rendered with display:none below 1024px (handled by parent layout).
 */
-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';

	interface Folder {
		id: string;
		name: string;
		slug: string;
		parent_id: string | null;
		path: string;
		depth: number;
		color: string | null;
		icon: string | null;
		asset_count: number;
	}

	interface Props {
		folders: Folder[];
		currentFolderId: string | null;
		onNavigate: (folder: Folder | null) => void;
	}

	const { folders, currentFolderId, onNavigate }: Props = $props();

	const rootFolders = $derived(folders.filter((f) => f.parent_id === null));
</script>

<aside class="folders-sidebar">
	<div class="sidebar-header">
		<h4>Folders</h4>
		<button class="new-folder-btn" title="New folder">
			<Icon name="IconPlus" size={14} />
		</button>
	</div>
	<nav class="folder-tree">
		<button
			class={['folder-item', { active: currentFolderId === null }]}
			onclick={() => onNavigate(null)}
		>
			<Icon name="IconHome" size={16} />
			<span>All Assets</span>
		</button>
		{#each rootFolders as folder (folder.id)}
			<button
				class={['folder-item', { active: currentFolderId === folder.id }]}
				onclick={() => onNavigate(folder)}
			>
				<Icon name="IconFolder" size={16} color={folder.color || undefined} />
				<span>{folder.name}</span>
				<span class="folder-count">{folder.asset_count}</span>
			</button>
		{/each}
	</nav>
</aside>

<style>
	.folders-sidebar {
		width: 200px;
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		overflow-y: auto;
		flex-shrink: 0;
	}

	.sidebar-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.sidebar-header h4 {
		margin: 0;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.new-folder-btn {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.new-folder-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #94a3b8;
	}

	.folder-tree {
		padding: 0.5rem;
	}

	.folder-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		color: #94a3b8;
		cursor: pointer;
		text-align: left;
		transition: all 0.2s;
	}

	.folder-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #f1f5f9;
	}

	.folder-item.active {
		background: rgba(59, 130, 246, 0.15);
		color: #3b82f6;
	}

	.folder-item :global(svg) {
		flex-shrink: 0;
	}

	.folder-item span {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.folder-count {
		font-size: 0.6875rem;
		color: #64748b;
		flex-shrink: 0;
	}

	@media (max-width: 1023.98px) {
		.folders-sidebar {
			display: none;
		}
	}
</style>
