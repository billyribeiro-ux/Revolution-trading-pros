<!--
	Rich Text Editor Component
	═══════════════════════════════════════════════════════════════════════════════

	WYSIWYG editor using contenteditable with formatting toolbar.
	Ready to be upgraded to Tiptap when installed.

	Features:
	- Basic formatting (bold, italic, underline, strikethrough)
	- Headings (H1-H4)
	- Lists (ordered, unordered)
	- Links and images
	- Code blocks
	- Block quotes
	- HTML mode toggle

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import {
		IconBold,
		IconItalic,
		IconUnderline,
		IconStrikethrough,
		IconH1,
		IconH2,
		IconH3,
		IconList,
		IconListNumbers,
		IconLink,
		IconPhoto,
		IconCode,
		IconQuote,
		IconSourceCode,
		IconAlignLeft,
		IconAlignCenter,
		IconAlignRight
	} from '$lib/icons';

	// Props
	interface Props {
		value?: string;
		placeholder?: string;
		minHeight?: string;
		disabled?: boolean;
		onchange?: (content: string) => void;
	}

	let {
		value = '',
		placeholder = 'Start writing...',
		minHeight = '300px',
		disabled = false,
		onchange
	}: Props = $props();

	// State
	let editorElement = $state<HTMLDivElement | null>(null);
	let showHtml = $state(false);
	let htmlContent = $state('');

	// Initialize content
	$effect(() => {
		if (browser && editorElement && value !== editorElement.innerHTML) {
			editorElement.innerHTML = value;
		}
	});

	// Sync HTML mode
	$effect(() => {
		if (showHtml && editorElement) {
			htmlContent = editorElement.innerHTML;
		}
	});

	// Handle input
	function handleInput() {
		if (!editorElement) return;
		const content = editorElement.innerHTML;
		onchange?.(content);
	}

	// Apply HTML changes
	function applyHtml() {
		if (!editorElement) return;
		editorElement.innerHTML = htmlContent;
		onchange?.(htmlContent);
		showHtml = false;
	}

	// Format commands
	function execCommand(command: string, value: string | undefined = undefined) {
		document.execCommand(command, false, value);
		editorElement?.focus();
		handleInput();
	}

	function formatBlock(tag: string) {
		execCommand('formatBlock', tag);
	}

	function insertLink() {
		const url = prompt('Enter URL:');
		if (url) {
			execCommand('createLink', url);
		}
	}

	function insertImage() {
		const url = prompt('Enter image URL:');
		if (url) {
			execCommand('insertImage', url);
		}
	}

	function isFormatActive(command: string): boolean {
		if (!browser) return false;
		return document.queryCommandState(command);
	}
</script>

<div class="rich-text-editor" class:disabled>
	<!-- Toolbar -->
	<div class="editor-toolbar">
		<!-- Text Formatting -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				class:active={isFormatActive('bold')}
				onclick={() => execCommand('bold')}
				title="Bold (Ctrl+B)"
				{disabled}
			>
				<IconBold size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				class:active={isFormatActive('italic')}
				onclick={() => execCommand('italic')}
				title="Italic (Ctrl+I)"
				{disabled}
			>
				<IconItalic size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				class:active={isFormatActive('underline')}
				onclick={() => execCommand('underline')}
				title="Underline (Ctrl+U)"
				{disabled}
			>
				<IconUnderline size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				class:active={isFormatActive('strikeThrough')}
				onclick={() => execCommand('strikeThrough')}
				title="Strikethrough"
				{disabled}
			>
				<IconStrikethrough size={16} />
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Headings -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('h1')}
				title="Heading 1"
				{disabled}
			>
				<IconH1 size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('h2')}
				title="Heading 2"
				{disabled}
			>
				<IconH2 size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('h3')}
				title="Heading 3"
				{disabled}
			>
				<IconH3 size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('p')}
				title="Paragraph"
				{disabled}
			>
				P
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Lists -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('insertUnorderedList')}
				title="Bullet List"
				{disabled}
			>
				<IconList size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('insertOrderedList')}
				title="Numbered List"
				{disabled}
			>
				<IconListNumbers size={16} />
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Alignment -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyLeft')}
				title="Align Left"
				{disabled}
			>
				<IconAlignLeft size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyCenter')}
				title="Align Center"
				{disabled}
			>
				<IconAlignCenter size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => execCommand('justifyRight')}
				title="Align Right"
				{disabled}
			>
				<IconAlignRight size={16} />
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Insert -->
		<div class="toolbar-group">
			<button
				type="button"
				class="toolbar-btn"
				onclick={insertLink}
				title="Insert Link"
				{disabled}
			>
				<IconLink size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={insertImage}
				title="Insert Image"
				{disabled}
			>
				<IconPhoto size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('pre')}
				title="Code Block"
				{disabled}
			>
				<IconCode size={16} />
			</button>
			<button
				type="button"
				class="toolbar-btn"
				onclick={() => formatBlock('blockquote')}
				title="Block Quote"
				{disabled}
			>
				<IconQuote size={16} />
			</button>
		</div>

		<!-- HTML Toggle -->
		<div class="toolbar-spacer"></div>
		<button
			type="button"
			class="toolbar-btn"
			class:active={showHtml}
			onclick={() => (showHtml = !showHtml)}
			title="Toggle HTML View"
			{disabled}
		>
			<IconSourceCode size={16} />
		</button>
	</div>

	<!-- Editor Content -->
	{#if showHtml}
		<div class="html-editor">
			<textarea
				class="html-textarea"
				bind:value={htmlContent}
				style="min-height: {minHeight}"
				{placeholder}
				{disabled}
			></textarea>
			<div class="html-actions">
				<button
					type="button"
					class="btn-secondary"
					onclick={() => (showHtml = false)}
				>
					Cancel
				</button>
				<button type="button" class="btn-primary" onclick={applyHtml}>
					Apply Changes
				</button>
			</div>
		</div>
	{:else}
		<div
			bind:this={editorElement}
			class="editor-content"
			contenteditable={!disabled}
			oninput={handleInput}
			data-placeholder={placeholder}
			style="min-height: {minHeight}"
			role="textbox"
			aria-multiline="true"
		></div>
	{/if}
</div>

<style>
	.rich-text-editor {
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.rich-text-editor.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Toolbar */
	.editor-toolbar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
		flex-wrap: wrap;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.125rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toolbar-btn:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.toolbar-btn.active {
		background: rgba(230, 184, 0, 0.2);
		color: #e6b800;
	}

	.toolbar-divider {
		width: 1px;
		height: 24px;
		background: rgba(51, 65, 85, 0.5);
		margin: 0 0.5rem;
	}

	.toolbar-spacer {
		flex: 1;
	}

	/* Editor Content */
	.editor-content {
		padding: 1rem;
		color: #f1f5f9;
		font-size: 1rem;
		line-height: 1.7;
		outline: none;
		overflow-y: auto;
	}

	.editor-content:empty::before {
		content: attr(data-placeholder);
		color: #64748b;
		pointer-events: none;
	}

	/* Editor content styling */
	.editor-content :global(h1) {
		font-size: 2rem;
		font-weight: 700;
		margin: 1.5rem 0 1rem;
		color: #f1f5f9;
	}

	.editor-content :global(h2) {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 1.25rem 0 0.75rem;
		color: #f1f5f9;
	}

	.editor-content :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 1rem 0 0.5rem;
		color: #f1f5f9;
	}

	.editor-content :global(p) {
		margin: 0.75rem 0;
	}

	.editor-content :global(ul),
	.editor-content :global(ol) {
		margin: 0.75rem 0;
		padding-left: 1.5rem;
	}

	.editor-content :global(li) {
		margin: 0.25rem 0;
	}

	.editor-content :global(a) {
		color: #e6b800;
		text-decoration: underline;
	}

	.editor-content :global(blockquote) {
		margin: 1rem 0;
		padding: 0.75rem 1rem;
		border-left: 3px solid #e6b800;
		background: rgba(230, 184, 0, 0.1);
		color: #cbd5e1;
		font-style: italic;
	}

	.editor-content :global(pre) {
		margin: 1rem 0;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 0.5rem;
		overflow-x: auto;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.editor-content :global(code) {
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.25rem;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875em;
	}

	.editor-content :global(img) {
		max-width: 100%;
		height: auto;
		border-radius: 0.5rem;
		margin: 1rem 0;
	}

	/* HTML Editor */
	.html-editor {
		display: flex;
		flex-direction: column;
	}

	.html-textarea {
		width: 100%;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.3);
		border: none;
		color: #f1f5f9;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		resize: vertical;
		outline: none;
	}

	.html-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.2);
		border-top: 1px solid rgba(51, 65, 85, 0.5);
	}

	.btn-secondary {
		padding: 0.5rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}
</style>
