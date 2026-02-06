<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Snippet Editor - Code Editor for Component Children
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Provides a code editor for editing Snippet children content
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 */

	interface Props {
		value: string;
		onChange: (value: string) => void;
		hasSnippets: boolean;
	}

	let { value, onChange, hasSnippets }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isExpanded = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// TEMPLATES
	// ═══════════════════════════════════════════════════════════════════════════

	const templates = [
		{ name: 'Simple Text', content: 'Hello, World!' },
		{
			name: 'Paragraph',
			content: '<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>'
		},
		{ name: 'Button', content: '<button>Click Me</button>' },
		{ name: 'Card Content', content: '<h3>Card Title</h3>\n<p>Card description goes here.</p>' },
		{
			name: 'List',
			content: '<ul>\n  <li>Item 1</li>\n  <li>Item 2</li>\n  <li>Item 3</li>\n</ul>'
		},
		{
			name: 'Form',
			content:
				'<form>\n  <input type="text" placeholder="Name" />\n  <button type="submit">Submit</button>\n</form>'
		}
	];

	function applyTemplate(content: string) {
		onChange(content);
	}

	function handleInput(event: Event) {
		const target = event.target as HTMLTextAreaElement;
		onChange(target.value);
	}
</script>

<div class="snippet-editor" class:expanded={isExpanded}>
	<header class="editor-header">
		<button class="toggle-btn" onclick={() => (isExpanded = !isExpanded)}>
			<span class="toggle-icon">{isExpanded ? '▼' : '▶'}</span>
			<h3 class="editor-title">
				Children Content
				{#if hasSnippets}
					<span class="badge">Snippet</span>
				{/if}
			</h3>
		</button>
	</header>

	{#if isExpanded}
		<div class="editor-content">
			{#if !hasSnippets}
				<p class="no-snippet-note">This component doesn't accept children content.</p>
			{:else}
				<!-- Templates -->
				<div class="templates">
					<span class="templates-label">Templates:</span>
					{#each templates as template}
						<button class="template-btn" onclick={() => applyTemplate(template.content)}>
							{template.name}
						</button>
					{/each}
				</div>

				<!-- Editor -->
				<div class="code-editor">
					<textarea
						class="code-textarea"
						{value}
						oninput={handleInput}
						placeholder="Enter HTML content for children..."
						rows="8"
						spellcheck="false"
					></textarea>
					<div class="editor-footer">
						<span class="char-count">{value.length} chars</span>
						<button class="clear-btn" onclick={() => onChange('')}> Clear </button>
					</div>
				</div>

				<!-- Preview -->
				<div class="preview">
					<span class="preview-label">Preview:</span>
					<div class="preview-content">
						{@html value || '<em>No content</em>'}
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.snippet-editor {
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		background: #111;
	}

	.editor-header {
		padding: 0;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		color: #fafafa;
		cursor: pointer;
		transition: background 0.15s;
		text-align: left;
	}

	.toggle-btn:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.toggle-icon {
		font-size: 0.625rem;
		color: #71717a;
	}

	.editor-title {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.badge {
		padding: 0.125rem 0.375rem;
		background: rgba(139, 92, 246, 0.2);
		border: 1px solid rgba(139, 92, 246, 0.4);
		border-radius: 0.25rem;
		font-size: 0.625rem;
		font-weight: 500;
		color: #c4b5fd;
	}

	.editor-content {
		padding: 0.75rem 1rem 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.no-snippet-note {
		margin: 0;
		padding: 1rem;
		text-align: center;
		color: #71717a;
		font-size: 0.875rem;
		font-style: italic;
	}

	.templates {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.375rem;
		margin-bottom: 0.75rem;
	}

	.templates-label {
		font-size: 0.75rem;
		color: #71717a;
		margin-right: 0.25rem;
	}

	.template-btn {
		padding: 0.25rem 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		color: #a1a1aa;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.template-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #fafafa;
	}

	.code-editor {
		margin-bottom: 0.75rem;
	}

	.code-textarea {
		width: 100%;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.375rem;
		color: #fafafa;
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
		font-size: 0.8125rem;
		line-height: 1.5;
		resize: vertical;
		tab-size: 2;
	}

	.code-textarea:focus {
		outline: none;
		border-color: rgba(139, 92, 246, 0.5);
	}

	.code-textarea::placeholder {
		color: #52525b;
	}

	.editor-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.375rem;
	}

	.char-count {
		font-size: 0.6875rem;
		color: #52525b;
	}

	.clear-btn {
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		color: #71717a;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.clear-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fafafa;
	}

	.preview {
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.375rem;
	}

	.preview-label {
		display: block;
		margin-bottom: 0.5rem;
		font-size: 0.6875rem;
		color: #52525b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.preview-content {
		font-size: 0.875rem;
		color: #a1a1aa;
	}

	.preview-content :global(em) {
		color: #52525b;
	}
</style>
