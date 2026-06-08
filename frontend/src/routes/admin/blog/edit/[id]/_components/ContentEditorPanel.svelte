<script lang="ts">
	import IconKeyboard from '@tabler/icons-svelte-runes/icons/keyboard';
	import IconMaximize from '@tabler/icons-svelte-runes/icons/maximize';
	import IconMinimize from '@tabler/icons-svelte-runes/icons/minimize';
	import { BlockEditor, type Block } from '$lib/components/blog/BlockEditor';

	type Props = {
		contentBlocks: Block[];
		postTitle: string;
		postSlug: string;
		metaDescription: string;
		focusKeyword: string;
		isFullscreen: boolean;
		onsave: (blocks: Block[]) => void;
		onchange: (blocks: Block[]) => void;
	};

	let {
		contentBlocks = $bindable(),
		postTitle,
		postSlug,
		metaDescription,
		focusKeyword,
		isFullscreen = $bindable(),
		onsave,
		onchange
	}: Props = $props();
</script>

<div class={{ 'form-group': true, 'editor-container': true, fullscreen: isFullscreen }}>
	<div class="editor-toolbar">
		<h3 class="editor-label">Content</h3>
		<div class="editor-actions">
			<button
				type="button"
				class="toolbar-btn"
				title="Keyboard shortcuts"
				onclick={() => {
					/* Handled by BlockEditor */
				}}
			>
				<IconKeyboard size={18} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}
				onclick={() => (isFullscreen = !isFullscreen)}
			>
				{#if isFullscreen}
					<IconMinimize size={18} />
				{:else}
					<IconMaximize size={18} />
				{/if}
			</button>
		</div>
	</div>
	<BlockEditor
		bind:blocks={contentBlocks}
		{postTitle}
		{postSlug}
		{metaDescription}
		{focusKeyword}
		{onsave}
		{onchange}
		autosaveInterval={30000}
	/>
</div>

<style>
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.editor-container {
		display: flex;
		flex-direction: column;
		background: white;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid #e5e5e5;
		min-height: 600px;
	}

	.editor-container.fullscreen {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 1000;
		border-radius: 0;
	}

	.editor-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid #e5e5e5;
		background: #f8f9fa;
	}

	.editor-label {
		font-weight: 600;
		color: #1a1a1a;
		font-size: 0.95rem;
		margin: 0;
	}

	.editor-actions {
		display: flex;
		gap: 0.5rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.toolbar-btn:hover {
		background: #f0f0f0;
		color: #3b82f6;
	}

	/* Make BlockEditor take full height */
	.editor-container :global(.block-editor) {
		flex: 1;
		min-height: 500px;
	}

	.editor-container.fullscreen :global(.block-editor) {
		min-height: calc(100vh - 60px);
	}
</style>
