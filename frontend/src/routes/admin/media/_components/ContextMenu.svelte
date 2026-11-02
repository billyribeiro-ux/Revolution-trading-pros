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
	import type { MediaItem } from '#lib/api/media.js';
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
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		padding: 0.25rem;
		min-width: 160px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
		z-index: 1000;
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
		color: var(--text-primary, #f0f6fc);
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.context-menu button:hover,
	.context-menu a:hover {
		background: rgba(230, 184, 0, 0.15);
		color: var(--text-primary, #f0f6fc);
	}

	.context-menu button.danger {
		color: var(--error-emphasis, #f85149);
	}

	.context-menu button.danger:hover {
		background: rgba(248, 81, 73, 0.15);
	}

	.context-menu button :global(svg),
	.context-menu a :global(svg) {
		width: 16px;
		height: 16px;
		color: var(--text-secondary, #8b949e);
	}

	.context-menu button:hover :global(svg),
	.context-menu a:hover :global(svg) {
		color: var(--primary-400, #ffd11a);
	}

	.context-menu button.danger :global(svg) {
		color: var(--error-emphasis, #f85149);
	}

	.context-menu hr {
		border: none;
		border-top: 1px solid var(--border-muted, #21262d);
		margin: 0.25rem 0;
	}
</style>
