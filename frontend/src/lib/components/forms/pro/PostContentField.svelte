<script lang="ts">
	/**
	 * PostContentField Component (FluentForms Pro)
	 *
	 * Rich text editor for capturing post content in frontend post creation forms.
	 * Supports basic formatting with toolbar.
	 */

	interface Props {
		name?: string;
		value?: string;
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		minLength?: number;
		maxLength?: number;
		rows?: number;
		enableRichText?: boolean;
		error?: string;
		helpText?: string;
		onchange?: (value: string) => void;
	}

	let {
		name = 'post_content',
		value = '',
		label = 'Post Content',
		placeholder = 'Write your content here...',
		required = false,
		disabled = false,
		minLength,
		maxLength,
		rows = 10,
		enableRichText = true,
		error = '',
		helpText = '',
		onchange
	}: Props = $props();

	let content = $state('');
	let editorRef = $state<HTMLDivElement>();
	let showHtml = $state(false);

	// Sync content with value prop changes
	$effect(() => {
		content = value;
	});

	const charCount = $derived(content.replace(/<[^>]*>/g, '').length);
	const wordCount = $derived(
		content
			.replace(/<[^>]*>/g, '')
			.trim()
			.split(/\s+/)
			.filter((w) => w.length > 0).length
	);

	function handleInput(e: Event) {
		if (enableRichText) {
			const target = e.target as HTMLDivElement;
			content = target.innerHTML;
		} else {
			const target = e.target as HTMLTextAreaElement;
			content = target.value;
		}
		if (onchange) onchange(content);
	}

	function execCommand(command: string, value?: string) {
		document.execCommand(command, false, value);
		editorRef?.focus();
		content = editorRef?.innerHTML || '';
		if (onchange) onchange(content);
	}

	function insertLink() {
		const url = prompt('Enter URL:');
		if (url) {
			execCommand('createLink', url);
		}
	}

	function toggleHtmlView() {
		if (showHtml && editorRef) {
			// Switching back to visual mode
			editorRef.innerHTML = content;
		}
		showHtml = !showHtml;
	}
</script>

<div class="post-content-field" class:disabled class:has-error={error}>
	{#if label}
		<div id="content-label" class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</div>
	{/if}

	{#if enableRichText}
		<div class="editor-container" role="group" aria-labelledby="content-label">
			<div class="toolbar">
				<button type="button" onclick={() => execCommand('bold')} title="Bold" class="toolbar-btn">
					<strong>B</strong>
				</button>
				<button
					type="button"
					onclick={() => execCommand('italic')}
					title="Italic"
					class="toolbar-btn"
				>
					<em>I</em>
				</button>
				<button
					type="button"
					onclick={() => execCommand('underline')}
					title="Underline"
					class="toolbar-btn"
				>
					<u>U</u>
				</button>
				<span class="toolbar-divider"></span>
				<button
					type="button"
					onclick={() => execCommand('insertUnorderedList')}
					title="Bullet List"
					class="toolbar-btn"
				>
					• List
				</button>
				<button
					type="button"
					onclick={() => execCommand('insertOrderedList')}
					title="Numbered List"
					class="toolbar-btn"
				>
					1. List
				</button>
				<span class="toolbar-divider"></span>
				<button type="button" onclick={insertLink} title="Insert Link" class="toolbar-btn">
					🔗
				</button>
				<button
					type="button"
					onclick={() => execCommand('formatBlock', 'h2')}
					title="Heading"
					class="toolbar-btn"
				>
					H2
				</button>
				<button
					type="button"
					onclick={() => execCommand('formatBlock', 'h3')}
					title="Subheading"
					class="toolbar-btn"
				>
					H3
				</button>
				<button
					type="button"
					onclick={() => execCommand('formatBlock', 'blockquote')}
					title="Quote"
					class="toolbar-btn"
				>
					❝
				</button>
				<span class="toolbar-divider"></span>
				<button
					type="button"
					onclick={toggleHtmlView}
					title="Toggle HTML"
					class="toolbar-btn"
					class:active={showHtml}
				>
					&lt;/&gt;
				</button>
			</div>

			{#if showHtml}
				<textarea class="html-editor" bind:value={content} oninput={handleInput} {rows}></textarea>
			{:else}
				<div
					bind:this={editorRef}
					class="content-editor"
					contenteditable={!disabled}
					oninput={handleInput}
					role="textbox"
					aria-multiline="true"
					style="min-height: {rows * 1.5}rem"
				>
					{@html content || `<p>${placeholder}</p>`}
				</div>
			{/if}
		</div>
	{:else}
		<textarea
			id={name}
			{name}
			bind:value={content}
			{placeholder}
			{disabled}
			{rows}
			minlength={minLength}
			maxlength={maxLength}
			oninput={handleInput}
			class="plain-textarea"
			aria-labelledby={label ? "content-label" : undefined}
		></textarea>
	{/if}

	<div class="stats-bar">
		<span>{wordCount} words</span>
		<span>{charCount} characters</span>
		{#if maxLength}
			<span class:warning={charCount > maxLength * 0.9}>{charCount}/{maxLength}</span>
		{/if}
	</div>

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}

	<!-- Hidden input for form submission -->
	<input type="hidden" {name} value={content} />
</div>

<style>
	.post-content-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.required {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.editor-container {
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.toolbar {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		padding: 0.5rem;
		background-color: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 32px;
		padding: 0 0.5rem;
		border: none;
		background-color: transparent;
		border-radius: 0.25rem;
		cursor: pointer;
		font-size: 0.875rem;
		color: #374151;
		transition: background-color 0.15s;
	}

	.toolbar-btn:hover {
		background-color: #e5e7eb;
	}

	.toolbar-btn.active {
		background-color: #dbeafe;
		color: #2563eb;
	}

	.toolbar-divider {
		width: 1px;
		height: 24px;
		background-color: #d1d5db;
		margin: 0 0.25rem;
		align-self: center;
	}

	.content-editor {
		padding: 1rem;
		min-height: 200px;
		outline: none;
		line-height: 1.6;
	}

	.content-editor:focus {
		background-color: #fefefe;
	}

	.content-editor :global(p) {
		margin: 0 0 0.75rem 0;
	}

	.content-editor :global(h2) {
		font-size: 1.5rem;
		font-weight: 600;
		margin: 1rem 0 0.5rem 0;
	}

	.content-editor :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0.75rem 0 0.5rem 0;
	}

	.content-editor :global(blockquote) {
		border-left: 3px solid #d1d5db;
		padding-left: 1rem;
		margin: 0.75rem 0;
		color: #6b7280;
		font-style: italic;
	}

	.content-editor :global(ul),
	.content-editor :global(ol) {
		margin: 0.5rem 0;
		padding-left: 1.5rem;
	}

	.content-editor :global(a) {
		color: #2563eb;
		text-decoration: underline;
	}

	.html-editor {
		width: 100%;
		padding: 1rem;
		border: none;
		font-family: 'Monaco', 'Consolas', monospace;
		font-size: 0.875rem;
		resize: vertical;
		background-color: #1e1e1e;
		color: #d4d4d4;
	}

	.plain-textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		line-height: 1.6;
		resize: vertical;
	}

	.plain-textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.stats-bar {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.stats-bar .warning {
		color: #f59e0b;
	}

	.help-text {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.disabled {
		opacity: 0.6;
	}

	.has-error .editor-container,
	.has-error .plain-textarea {
		border-color: #fca5a5;
	}
</style>
