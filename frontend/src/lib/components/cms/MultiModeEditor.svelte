<!--
	Multi-Mode Content Editor
	═══════════════════════════════════════════════════════════════════════════════

	Comprehensive editor supporting three content modes:
	1. WYSIWYG (Visual) - Rich text editing with formatting toolbar
	2. Markdown - Plain text markdown with live preview
	3. HTML - Raw HTML editing with syntax awareness

	Features:
	- Seamless mode switching with content conversion
	- Live preview for markdown mode
	- Auto-detection of content format
	- Undo/redo support
	- Keyboard shortcuts

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
		IconAlignRight,
		IconEye,
		IconEyeOff,
		IconArrowLeft,
		IconArrowRight
	} from '$lib/icons';
	import {
		markdownToHtml,
		htmlToMarkdown,
		detectContentFormat
	} from './markdown-parser';

	// Types
	type ContentMode = 'visual' | 'markdown' | 'html';

	// Props
	interface Props {
		id?: string;
		value?: string;
		format?: 'html' | 'markdown' | 'raw';
		placeholder?: string;
		minHeight?: string;
		disabled?: boolean;
		showModeSelector?: boolean;
		onchange?: (content: string, format: 'html' | 'markdown' | 'raw') => void;
	}

	let {
		id,
		value = '',
		format = 'html',
		placeholder = 'Start writing your content...',
		minHeight = '400px',
		disabled = false,
		showModeSelector = true,
		onchange
	}: Props = $props();

	// State
	let mode = $state<ContentMode>('visual');
	let editorElement = $state<HTMLDivElement | null>(null);
	let markdownContent = $state('');
	let htmlContent = $state('');
	let showPreview = $state(true);
	let undoStack = $state<string[]>([]);
	let redoStack = $state<string[]>([]);

	// Initialize based on format prop
	$effect(() => {
		if (!browser) return;

		const detectedFormat = format === 'raw' ? detectContentFormat(value) : format;

		if (detectedFormat === 'markdown') {
			mode = 'markdown';
			markdownContent = value;
			htmlContent = markdownToHtml(value);
		} else {
			htmlContent = value;
			markdownContent = htmlToMarkdown(value);
		}

		if (editorElement && mode === 'visual') {
			editorElement.innerHTML = htmlContent;
		}
	});

	// Derived state
	let currentContent = $derived.by(() => {
		switch (mode) {
			case 'visual':
				return editorElement?.innerHTML || htmlContent;
			case 'markdown':
				return markdownContent;
			case 'html':
				return htmlContent;
		}
	});

	let previewHtml = $derived(markdownToHtml(markdownContent));

	// Mode switching
	function switchMode(newMode: ContentMode) {
		if (newMode === mode) return;

		// Save current state before switching
		saveToUndoStack();

		// Convert content between modes
		if (mode === 'visual') {
			htmlContent = editorElement?.innerHTML || '';
			markdownContent = htmlToMarkdown(htmlContent);
		} else if (mode === 'markdown') {
			htmlContent = markdownToHtml(markdownContent);
		} else if (mode === 'html') {
			markdownContent = htmlToMarkdown(htmlContent);
		}

		mode = newMode;

		// Update visual editor if switching to it
		if (newMode === 'visual' && editorElement) {
			requestAnimationFrame(() => {
				if (editorElement) {
					editorElement.innerHTML = htmlContent;
				}
			});
		}
	}

	// Handle input changes
	function handleVisualInput() {
		if (!editorElement) return;
		htmlContent = editorElement.innerHTML;
		emitChange();
	}

	function handleMarkdownInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		markdownContent = target.value;
		htmlContent = markdownToHtml(markdownContent);
		emitChange();
	}

	function handleHtmlInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		htmlContent = target.value;
		emitChange();
	}

	function emitChange() {
		const outputFormat = mode === 'markdown' ? 'markdown' : 'html';
		const outputContent = mode === 'markdown' ? markdownContent : htmlContent;
		onchange?.(outputContent, outputFormat);
	}

	// Undo/Redo
	function saveToUndoStack() {
		undoStack = [...undoStack, currentContent || ''];
		redoStack = [];
	}

	function undo() {
		if (undoStack.length === 0) return;
		const content = undoStack[undoStack.length - 1];
		undoStack = undoStack.slice(0, -1);
		redoStack = [...redoStack, currentContent || ''];
		restoreContent(content);
	}

	function redo() {
		if (redoStack.length === 0) return;
		const content = redoStack[redoStack.length - 1];
		redoStack = redoStack.slice(0, -1);
		undoStack = [...undoStack, currentContent || ''];
		restoreContent(content);
	}

	function restoreContent(content: string) {
		if (mode === 'visual' && editorElement) {
			editorElement.innerHTML = content;
			htmlContent = content;
		} else if (mode === 'markdown') {
			markdownContent = content;
			htmlContent = markdownToHtml(content);
		} else {
			htmlContent = content;
		}
		emitChange();
	}

	// Format commands (for visual mode)
	function execCommand(command: string, commandValue: string | undefined = undefined) {
		if (mode !== 'visual') return;
		saveToUndoStack();
		document.execCommand(command, false, commandValue);
		editorElement?.focus();
		handleVisualInput();
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
		if (!browser || mode !== 'visual') return false;
		try {
			return document.queryCommandState(command);
		} catch {
			return false;
		}
	}

	// Markdown shortcuts
	function insertMarkdown(syntax: string, wrap = true) {
		if (mode !== 'markdown') return;

		const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
		if (!textarea) return;

		const start = textarea.selectionStart;
		const end = textarea.selectionEnd;
		const selectedText = markdownContent.substring(start, end);

		let newText: string;
		let newCursorPos: number;

		if (wrap && selectedText) {
			newText =
				markdownContent.substring(0, start) +
				syntax +
				selectedText +
				syntax +
				markdownContent.substring(end);
			newCursorPos = end + syntax.length * 2;
		} else {
			newText = markdownContent.substring(0, start) + syntax + markdownContent.substring(end);
			newCursorPos = start + syntax.length;
		}

		markdownContent = newText;
		htmlContent = markdownToHtml(markdownContent);
		emitChange();

		requestAnimationFrame(() => {
			textarea.focus();
			textarea.setSelectionRange(newCursorPos, newCursorPos);
		});
	}

	function insertMarkdownBlock(prefix: string) {
		if (mode !== 'markdown') return;

		const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
		if (!textarea) return;

		const start = textarea.selectionStart;
		const lineStart = markdownContent.lastIndexOf('\n', start - 1) + 1;

		const newText =
			markdownContent.substring(0, lineStart) +
			prefix +
			markdownContent.substring(lineStart);

		markdownContent = newText;
		htmlContent = markdownToHtml(markdownContent);
		emitChange();

		const newCursorPos = start + prefix.length;
		requestAnimationFrame(() => {
			textarea.focus();
			textarea.setSelectionRange(newCursorPos, newCursorPos);
		});
	}

	// Keyboard shortcuts
	function handleKeyDown(e: KeyboardEvent) {
		if (e.ctrlKey || e.metaKey) {
			switch (e.key.toLowerCase()) {
				case 'b':
					e.preventDefault();
					if (mode === 'visual') execCommand('bold');
					else if (mode === 'markdown') insertMarkdown('**');
					break;
				case 'i':
					e.preventDefault();
					if (mode === 'visual') execCommand('italic');
					else if (mode === 'markdown') insertMarkdown('*');
					break;
				case 'u':
					e.preventDefault();
					if (mode === 'visual') execCommand('underline');
					break;
				case 'z':
					e.preventDefault();
					if (e.shiftKey) redo();
					else undo();
					break;
				case 'y':
					e.preventDefault();
					redo();
					break;
			}
		}
	}
</script>

<div class="multi-mode-editor" class:disabled onkeydown={handleKeyDown}>
	<!-- Mode Selector -->
	{#if showModeSelector}
		<div class="mode-selector">
			<button
				type="button"
				class="mode-btn"
				class:active={mode === 'visual'}
				onclick={() => switchMode('visual')}
				{disabled}
			>
				<IconEye size={16} />
				<span>Visual</span>
			</button>
			<button
				type="button"
				class="mode-btn"
				class:active={mode === 'markdown'}
				onclick={() => switchMode('markdown')}
				{disabled}
			>
				<span class="mode-icon">MD</span>
				<span>Markdown</span>
			</button>
			<button
				type="button"
				class="mode-btn"
				class:active={mode === 'html'}
				onclick={() => switchMode('html')}
				{disabled}
			>
				<IconSourceCode size={16} />
				<span>HTML</span>
			</button>

			<div class="mode-actions">
				<button
					type="button"
					class="action-btn"
					onclick={undo}
					disabled={disabled || undoStack.length === 0}
					title="Undo (Ctrl+Z)"
				>
					<IconArrowLeft size={16} />
				</button>
				<button
					type="button"
					class="action-btn"
					onclick={redo}
					disabled={disabled || redoStack.length === 0}
					title="Redo (Ctrl+Shift+Z)"
				>
					<IconArrowRight size={16} />
				</button>
			</div>
		</div>
	{/if}

	<!-- Visual Mode -->
	{#if mode === 'visual'}
		<div class="visual-editor">
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
			</div>

			<!-- Content Area -->
			<div
				bind:this={editorElement}
				class="editor-content"
				contenteditable={!disabled}
				oninput={handleVisualInput}
				onfocus={() => saveToUndoStack()}
				data-placeholder={placeholder}
				style="min-height: {minHeight}"
				role="textbox"
				aria-multiline="true"
			></div>
		</div>
	{/if}

	<!-- Markdown Mode -->
	{#if mode === 'markdown'}
		<div class="markdown-editor">
			<!-- Markdown Toolbar -->
			<div class="editor-toolbar">
				<div class="toolbar-group">
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdown('**')}
						title="Bold (Ctrl+B)"
						{disabled}
					>
						<IconBold size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdown('*')}
						title="Italic (Ctrl+I)"
						{disabled}
					>
						<IconItalic size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdown('~~')}
						title="Strikethrough"
						{disabled}
					>
						<IconStrikethrough size={16} />
					</button>
				</div>

				<div class="toolbar-divider"></div>

				<div class="toolbar-group">
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('# ')}
						title="Heading 1"
						{disabled}
					>
						<IconH1 size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('## ')}
						title="Heading 2"
						{disabled}
					>
						<IconH2 size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('### ')}
						title="Heading 3"
						{disabled}
					>
						<IconH3 size={16} />
					</button>
				</div>

				<div class="toolbar-divider"></div>

				<div class="toolbar-group">
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('- ')}
						title="Bullet List"
						{disabled}
					>
						<IconList size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('1. ')}
						title="Numbered List"
						{disabled}
					>
						<IconListNumbers size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdownBlock('> ')}
						title="Blockquote"
						{disabled}
					>
						<IconQuote size={16} />
					</button>
				</div>

				<div class="toolbar-divider"></div>

				<div class="toolbar-group">
					<button
						type="button"
						class="toolbar-btn"
						onclick={() => insertMarkdown('`', true)}
						title="Inline Code"
						{disabled}
					>
						<IconCode size={16} />
					</button>
					<button
						type="button"
						class="toolbar-btn"
						onclick={() =>
							insertMarkdown('\n```\n', false) ||
							(markdownContent += '\n```')}
						title="Code Block"
						{disabled}
					>
						<IconSourceCode size={16} />
					</button>
				</div>

				<div class="toolbar-spacer"></div>

				<button
					type="button"
					class="toolbar-btn preview-toggle"
					class:active={showPreview}
					onclick={() => (showPreview = !showPreview)}
					title={showPreview ? 'Hide Preview' : 'Show Preview'}
				>
					{#if showPreview}
						<IconEyeOff size={16} />
					{:else}
						<IconEye size={16} />
					{/if}
					<span>Preview</span>
				</button>
			</div>

			<!-- Markdown Content -->
			<div class="markdown-content" class:with-preview={showPreview}>
				<div class="markdown-input-wrapper">
					<textarea
						class="markdown-textarea"
						bind:value={markdownContent}
						oninput={handleMarkdownInput}
						{placeholder}
						{disabled}
						style="min-height: {minHeight}"
						spellcheck="false"
					></textarea>
				</div>

				{#if showPreview}
					<div class="markdown-preview-wrapper">
						<div class="preview-label">Preview</div>
						<div
							class="markdown-preview"
							style="min-height: {minHeight}"
						>
							{@html previewHtml || `<p class="preview-placeholder">${placeholder}</p>`}
						</div>
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- HTML Mode -->
	{#if mode === 'html'}
		<div class="html-editor">
			<div class="editor-toolbar">
				<div class="toolbar-info">
					<IconSourceCode size={16} />
					<span>HTML Source Code</span>
				</div>
				<div class="toolbar-spacer"></div>
				<div class="toolbar-hint">
					Edit raw HTML. Be careful with syntax!
				</div>
			</div>

			<textarea
				class="html-textarea"
				bind:value={htmlContent}
				oninput={handleHtmlInput}
				placeholder="<p>Your HTML content here...</p>"
				{disabled}
				style="min-height: {minHeight}"
				spellcheck="false"
			></textarea>
		</div>
	{/if}
</div>

<style>
	.multi-mode-editor {
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.multi-mode-editor.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	/* Mode Selector */
	.mode-selector {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.3);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.mode-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.mode-btn:hover {
		background: rgba(51, 65, 85, 0.3);
		color: #f1f5f9;
	}

	.mode-btn.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	.mode-icon {
		font-weight: 700;
		font-size: 0.6875rem;
		letter-spacing: 0.025em;
	}

	.mode-actions {
		display: flex;
		gap: 0.25rem;
		margin-left: auto;
		padding-left: 0.5rem;
		border-left: 1px solid rgba(51, 65, 85, 0.5);
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.3);
		color: #f1f5f9;
	}

	.action-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* Toolbar (shared) */
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

	.toolbar-btn:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.toolbar-btn.active {
		background: rgba(230, 184, 0, 0.2);
		color: #e6b800;
	}

	.toolbar-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	.toolbar-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		font-size: 0.8125rem;
	}

	.toolbar-hint {
		font-size: 0.75rem;
		color: #64748b;
	}

	.preview-toggle {
		width: auto;
		padding: 0 0.75rem;
		gap: 0.375rem;
	}

	/* Visual Editor Content */
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
	.editor-content :global(h1),
	.markdown-preview :global(h1) {
		font-size: 2rem;
		font-weight: 700;
		margin: 1.5rem 0 1rem;
		color: #f1f5f9;
	}

	.editor-content :global(h2),
	.markdown-preview :global(h2) {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 1.25rem 0 0.75rem;
		color: #f1f5f9;
	}

	.editor-content :global(h3),
	.markdown-preview :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 1rem 0 0.5rem;
		color: #f1f5f9;
	}

	.editor-content :global(p),
	.markdown-preview :global(p) {
		margin: 0.75rem 0;
	}

	.editor-content :global(ul),
	.editor-content :global(ol),
	.markdown-preview :global(ul),
	.markdown-preview :global(ol) {
		margin: 0.75rem 0;
		padding-left: 1.5rem;
	}

	.editor-content :global(li),
	.markdown-preview :global(li) {
		margin: 0.25rem 0;
	}

	.editor-content :global(a),
	.markdown-preview :global(a) {
		color: #e6b800;
		text-decoration: underline;
	}

	.editor-content :global(blockquote),
	.markdown-preview :global(blockquote) {
		margin: 1rem 0;
		padding: 0.75rem 1rem;
		border-left: 3px solid #e6b800;
		background: rgba(230, 184, 0, 0.1);
		color: #cbd5e1;
		font-style: italic;
	}

	.editor-content :global(pre),
	.markdown-preview :global(pre) {
		margin: 1rem 0;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 0.5rem;
		overflow-x: auto;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.editor-content :global(code),
	.markdown-preview :global(code) {
		padding: 0.125rem 0.375rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.25rem;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875em;
	}

	.editor-content :global(pre code),
	.markdown-preview :global(pre code) {
		padding: 0;
		background: none;
	}

	.editor-content :global(img),
	.markdown-preview :global(img) {
		max-width: 100%;
		height: auto;
		border-radius: 0.5rem;
		margin: 1rem 0;
	}

	.markdown-preview :global(table) {
		width: 100%;
		border-collapse: collapse;
		margin: 1rem 0;
	}

	.markdown-preview :global(th),
	.markdown-preview :global(td) {
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
		text-align: left;
	}

	.markdown-preview :global(th) {
		background: rgba(0, 0, 0, 0.2);
		font-weight: 600;
	}

	.markdown-preview :global(hr) {
		border: none;
		height: 1px;
		background: rgba(51, 65, 85, 0.5);
		margin: 1.5rem 0;
	}

	/* Markdown Editor */
	.markdown-content {
		display: flex;
	}

	.markdown-content.with-preview {
		display: grid;
		grid-template-columns: 1fr 1fr;
	}

	.markdown-input-wrapper {
		display: flex;
		flex-direction: column;
	}

	.markdown-textarea {
		width: 100%;
		padding: 1rem;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		resize: vertical;
		outline: none;
	}

	.markdown-textarea::placeholder {
		color: #64748b;
	}

	.markdown-preview-wrapper {
		border-left: 1px solid rgba(51, 65, 85, 0.5);
		display: flex;
		flex-direction: column;
	}

	.preview-label {
		padding: 0.5rem 1rem;
		background: rgba(0, 0, 0, 0.1);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.markdown-preview {
		flex: 1;
		padding: 1rem;
		color: #f1f5f9;
		font-size: 1rem;
		line-height: 1.7;
		overflow-y: auto;
	}

	.markdown-preview :global(.preview-placeholder) {
		color: #64748b;
	}

	/* HTML Editor */
	.html-textarea {
		width: 100%;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border: none;
		color: #f1f5f9;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		resize: vertical;
		outline: none;
	}

	.html-textarea::placeholder {
		color: #64748b;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.mode-selector {
			flex-wrap: wrap;
		}

		.mode-btn span:not(.mode-icon) {
			display: none;
		}

		.mode-btn {
			padding: 0.5rem;
		}

		.markdown-content.with-preview {
			grid-template-columns: 1fr;
		}

		.markdown-preview-wrapper {
			border-left: none;
			border-top: 1px solid rgba(51, 65, 85, 0.5);
		}

		.preview-toggle span {
			display: none;
		}

		.preview-toggle {
			padding: 0;
			width: 32px;
		}
	}
</style>
