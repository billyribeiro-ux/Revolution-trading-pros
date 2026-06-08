<script lang="ts">
	/**
	 * ContextMenu — right-click action menu for a single media item.
	 * Extracted from `admin/media/+page.svelte` (R10-C).
	 *
	 * Caller controls visibility by passing/binding `menu` (null = hidden).
	 * Single discriminated `onAction(action)` callback keeps the prop surface
	 * tight; the caller dispatches to its own handlers + resets `menu = null`.
	 *
	 * Props:
	 *   - menu: { x, y, item } | null  ($bindable — child resets to null after action)
	 *   - aiEnabled: boolean
	 *   - onAction: (action: ContextAction) => void
	 *
	 * 1 bindable scalar, 1 callback.
	 */
	import { scale } from 'svelte/transition';
	import type { MediaItem } from '$lib/api/media';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconCrop from '@tabler/icons-svelte-runes/icons/crop';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconClipboardCopy from '@tabler/icons-svelte-runes/icons/clipboard-copy';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';

	export type ContextAction = 'view' | 'optimize' | 'crop' | 'ai-analyze' | 'copy-url' | 'delete';

	let {
		menu = $bindable(null),
		aiEnabled,
		onAction
	}: {
		menu?: { x: number; y: number; item: MediaItem } | null;
		aiEnabled: boolean;
		onAction: (action: ContextAction) => void;
	} = $props();

	function trigger(action: ContextAction) {
		onAction(action);
		menu = null;
	}
</script>

{#if menu}
	<div
		class="context-menu"
		style:left={`${menu.x}px`}
		style:top={`${menu.y}px`}
		transition:scale={{ duration: 150, start: 0.9 }}
	>
		<button onclick={() => trigger('view')}>
			<IconEye size={16} aria-hidden="true" />
			View
		</button>
		{#if menu.item.file_type === 'image'}
			<button onclick={() => trigger('optimize')}>
				<IconBolt size={16} aria-hidden="true" />
				Optimize
			</button>
			<button onclick={() => trigger('crop')}>
				<IconCrop size={16} aria-hidden="true" />
				Crop
			</button>
			{#if aiEnabled}
				<button onclick={() => trigger('ai-analyze')}>
					<IconEye size={16} aria-hidden="true" />
					AI Analyze
				</button>
			{/if}
		{/if}
		<hr />
		<a href={menu.item.url} download={menu.item.filename} onclick={() => (menu = null)}>
			<IconDownload size={16} aria-hidden="true" />
			Download
		</a>
		<button onclick={() => trigger('copy-url')}>
			<IconClipboardCopy size={16} aria-hidden="true" />
			Copy URL
		</button>
		<hr />
		<button class="danger" onclick={() => trigger('delete')}>
			<IconTrash size={16} aria-hidden="true" />
			Delete
		</button>
	</div>
{/if}

<style>
	.context-menu {
		position: fixed;
		background: rgba(15, 23, 42, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		padding: 0.25rem;
		min-width: 160px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
		z-index: 1000;
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
	}

	.context-menu button,
	.context-menu a {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.625rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		text-align: left;
		font-size: 0.8125rem;
		color: #e2e8f0;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.context-menu button:hover,
	.context-menu a:hover {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	.context-menu button.danger {
		color: #f87171;
	}

	.context-menu button.danger:hover {
		background: rgba(239, 68, 68, 0.15);
	}

	.context-menu button :global(svg),
	.context-menu a :global(svg) {
		width: 16px;
		height: 16px;
		color: #94a3b8;
	}

	.context-menu button:hover :global(svg),
	.context-menu a:hover :global(svg) {
		color: #a5b4fc;
	}

	.context-menu button.danger :global(svg) {
		color: #f87171;
	}

	.context-menu hr {
		border: none;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		margin: 0.25rem 0;
	}
</style>
