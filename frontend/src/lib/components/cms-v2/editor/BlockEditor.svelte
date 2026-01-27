<!--
	Block Editor - Main Container Component
	═══════════════════════════════════════════════════════════════════════════════

	The main block-based content editor container. Orchestrates:
	- Block list with drag-and-drop reordering
	- Slash command menu for adding blocks
	- Block toolbar for selected block actions
	- Keyboard navigation between blocks

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { onMount, setContext, getContext } from 'svelte';
	import { browser } from '$app/environment';
	import { editorStore, type ContentBlock, type EditorContent } from '$lib/stores/editor.svelte';
	import BlockList from './BlockList.svelte';
	import BlockToolbar from './BlockToolbar.svelte';
	import SlashCommands from './SlashCommands.svelte';
	import AddBlockButton from './AddBlockButton.svelte';
	import type { SlashCommand } from './SlashCommands.svelte';

	// ==========================================================================
	// Types
	// ==========================================================================

	interface Props {
		content?: EditorContent;
		readonly?: boolean;
		autofocus?: boolean;
		placeholder?: string;
		onchange?: (blocks: ContentBlock[]) => void;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	let {
		content,
		readonly = false,
		autofocus = false,
		placeholder = 'Start writing or type / for commands...',
		onchange
	}: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	let editorRef = $state<HTMLDivElement | null>(null);
	let showSlashMenu = $state(false);
	let slashMenuPosition = $state({ x: 0, y: 0 });
	let slashFilter = $state('');
	let insertAfterBlockId = $state<string | null>(null);

	// ==========================================================================
	// Derived State
	// ==========================================================================

	let blocks = $derived(editorStore.contentBlocks);
	let selectedBlockId = $derived(editorStore.activeBlockId);
	let isEmpty = $derived(blocks.length === 0);

	// ==========================================================================
	// Context
	// ==========================================================================

	// Provide editor context to child components
	setContext('blockEditor', {
		get readonly() { return readonly; },
		get selectedBlockId() { return selectedBlockId; },
		selectBlock: (id: string | null) => editorStore.setActiveBlock(id),
		openSlashMenu: (position: { x: number; y: number }, afterBlockId?: string) => {
			slashMenuPosition = position;
			insertAfterBlockId = afterBlockId ?? null;
			slashFilter = '';
			showSlashMenu = true;
		},
		closeSlashMenu: () => {
			showSlashMenu = false;
			slashFilter = '';
		}
	});

	// ==========================================================================
	// Effects
	// ==========================================================================

	// Load content when provided
	$effect(() => {
		if (content) {
			editorStore.loadContent(content);
		}
	});

	// Notify parent of changes
	$effect(() => {
		if (blocks.length > 0) {
			onchange?.(blocks);
		}
	});

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleSlashSelect(command: SlashCommand) {
		showSlashMenu = false;
		slashFilter = '';

		// Map slash command ID to block type
		const blockTypeMap: Record<string, string> = {
			'heading1': 'heading',
			'heading2': 'heading',
			'heading3': 'heading',
			'paragraph': 'rich-text',
			'quote': 'quote',
			'divider': 'divider',
			'bullet-list': 'list',
			'numbered-list': 'list',
			'code-block': 'code',
			'image': 'image',
			'video': 'video',
			'table': 'table',
			'callout': 'callout',
			'alert-box': 'callout',
			'trade-setup': 'trade-setup',
			'performance-stats': 'performance-stats',
			'tradingview-chart': 'tradingview-chart',
			'youtube': 'video',
			'vimeo': 'video',
			'twitter': 'embed',
			'columns': 'columns',
			'reusable-block': 'reusable'
		};

		const blockType = blockTypeMap[command.id] || command.id;

		// Create block with appropriate default data
		let blockData: Record<string, unknown> = {};

		switch (command.id) {
			case 'heading1':
				blockData = { level: 1, text: '' };
				break;
			case 'heading2':
				blockData = { level: 2, text: '' };
				break;
			case 'heading3':
				blockData = { level: 3, text: '' };
				break;
			case 'bullet-list':
				blockData = { type: 'bullet', items: [''] };
				break;
			case 'numbered-list':
				blockData = { type: 'numbered', items: [''] };
				break;
			case 'callout':
				blockData = { type: 'info', title: '', content: '' };
				break;
			case 'alert-box':
				blockData = { type: 'warning', title: 'Warning', content: '' };
				break;
			case 'divider':
				blockData = { style: 'solid' };
				break;
			case 'code-block':
				blockData = { code: '', language: 'javascript', filename: '' };
				break;
			case 'quote':
				blockData = { content: '', citation: '' };
				break;
			default:
				blockData = { content: '' };
		}

		// Add block
		const newBlock = editorStore.addBlock(blockType, blockData, insertAfterBlockId ?? undefined);
		editorStore.setActiveBlock(newBlock.id);
		insertAfterBlockId = null;
	}

	function handleSlashClose() {
		showSlashMenu = false;
		slashFilter = '';
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (readonly) return;

		// Global slash command trigger
		if (e.key === '/' && !showSlashMenu && !e.ctrlKey && !e.metaKey) {
			// Only if not inside an input/contenteditable
			const target = e.target as HTMLElement;
			const isEditable = target.isContentEditable || 
				target.tagName === 'INPUT' || 
				target.tagName === 'TEXTAREA';

			if (!isEditable || isEmpty) {
				e.preventDefault();
				const rect = editorRef?.getBoundingClientRect();
				if (rect) {
					slashMenuPosition = {
						x: rect.left + 20,
						y: rect.top + (isEmpty ? 60 : rect.height / 2)
					};
				}
				showSlashMenu = true;
			}
		}

		// Escape to deselect
		if (e.key === 'Escape' && selectedBlockId && !showSlashMenu) {
			editorStore.setActiveBlock(null);
		}
	}

	function handleEditorClick(e: MouseEvent) {
		// Click on editor background deselects block
		if (e.target === editorRef) {
			editorStore.setActiveBlock(null);
		}
	}

	function handleAddBlock() {
		if (readonly) return;
		
		const rect = editorRef?.getBoundingClientRect();
		if (rect) {
			slashMenuPosition = {
				x: rect.left + rect.width / 2 - 160,
				y: rect.bottom - 200
			};
		}
		showSlashMenu = true;
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		if (browser && autofocus && editorRef) {
			editorRef.focus();
		}
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	bind:this={editorRef}
	class="block-editor"
	class:readonly
	class:empty={isEmpty}
	role="application"
	aria-label="Block editor"
	aria-roledescription="block editor"
	tabindex={readonly ? -1 : 0}
	onkeydown={handleKeyDown}
	onclick={handleEditorClick}
>
	<!-- Empty State -->
	{#if isEmpty && !readonly}
		<div class="empty-state">
			<div class="empty-icon">
				<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</div>
			<p class="empty-title">Start Creating</p>
			<p class="empty-hint">{placeholder}</p>
			<button type="button" class="empty-cta" onclick={handleAddBlock}>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
				Add your first block
			</button>
		</div>
	{:else}
		<!-- Block List -->
		<BlockList {readonly} />

		<!-- Add Block Button -->
		{#if !readonly}
			<AddBlockButton onclick={handleAddBlock} />
		{/if}
	{/if}

	<!-- Block Toolbar (floating) -->
	{#if selectedBlockId && !readonly}
		<BlockToolbar blockId={selectedBlockId} />
	{/if}

	<!-- Slash Command Menu -->
	<SlashCommands
		position={slashMenuPosition}
		visible={showSlashMenu}
		filter={slashFilter}
		onSelect={handleSlashSelect}
		onClose={handleSlashClose}
	/>
</div>

<style>
	.block-editor {
		position: relative;
		min-height: 400px;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		outline: none;
		transition: all 0.2s;
	}

	.block-editor:focus-within {
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.block-editor.readonly {
		background: rgba(15, 23, 42, 0.3);
		pointer-events: none;
	}

	.block-editor.empty {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		padding: 3rem 1.5rem;
	}

	.empty-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		margin-bottom: 1.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 2px dashed rgba(99, 102, 241, 0.3);
		border-radius: 50%;
		color: #6366f1;
	}

	.empty-title {
		margin: 0 0 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.empty-hint {
		margin: 0 0 1.5rem;
		font-size: 0.875rem;
		color: #64748b;
		max-width: 280px;
	}

	.empty-cta {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #4f46e5);
		border: none;
		border-radius: 0.5rem;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.empty-cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px -4px rgba(99, 102, 241, 0.4);
	}

	.empty-cta:active {
		transform: translateY(0);
	}
</style>
