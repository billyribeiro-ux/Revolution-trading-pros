<!--
/**
 * HTML Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Raw HTML editor with DOMPurify sanitization and preview toggle
 * Production-ready with XSS protection, accessibility, and dark mode
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import {
		IconCode,
		IconEye,
		IconAlertTriangle,
		IconShieldCheck,
		IconMaximize,
		IconMinimize
	} from '$lib/icons';
	import { sanitizeHTML } from '$lib/utils/sanitization';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	type EditorHeight = 'small' | 'medium' | 'large' | 'auto';
	type ViewMode = 'edit' | 'preview' | 'split';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props & State
	// =========================================================================

	let props: Props = $props();

	let viewMode = $state<ViewMode>('edit');
	let isExpanded = $state(false);

	// =========================================================================
	// Derived Values
	// =========================================================================

	let rawHtml = $derived(props.block.content.html || '');
	let editorHeight = $derived((props.block.settings.htmlEditorHeight as EditorHeight) || 'medium');
	let showWarning = $derived(props.block.settings.htmlShowWarning !== false);

	// Sanitize HTML using DOMPurify with custom mode for extended tags
	let safeHtml = $derived(sanitizeHTML(rawHtml || '', { mode: 'custom' }));

	// Check if HTML was modified during sanitization
	let wasSanitized = $derived(rawHtml !== safeHtml && rawHtml.length > 0);

	// Height values mapping
	let editorHeightValue = $derived(
		{
			small: '150px',
			medium: '250px',
			large: '400px',
			auto: 'auto'
		}[editorHeight]
	);

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function handleInput(e: Event): void {
		const textarea = e.target as HTMLTextAreaElement;
		updateContent({ html: textarea.value });
	}

	function handleKeyDown(e: KeyboardEvent): void {
		// Allow Tab key for indentation
		if (e.key === 'Tab') {
			e.preventDefault();
			const textarea = e.target as HTMLTextAreaElement;
			const start = textarea.selectionStart;
			const end = textarea.selectionEnd;
			const value = textarea.value;

			// Insert tab character
			textarea.value = value.substring(0, start) + '\t' + value.substring(end);
			textarea.selectionStart = textarea.selectionEnd = start + 1;

			// Trigger update
			updateContent({ html: textarea.value });
		}
	}

	function toggleExpand(): void {
		isExpanded = !isExpanded;
	}

	function setViewMode(mode: ViewMode): void {
		viewMode = mode;
	}
</script>

<div class="html-block" class:expanded={isExpanded} role="region" aria-label="Custom HTML block">
	{#if props.isEditing}
		<!-- Edit Mode Header -->
		<header class="html-header">
			<div class="header-title">
				<IconCode size={18} aria-hidden="true" />
				<span>Custom HTML</span>
			</div>

			<div class="header-actions">
				<!-- View Mode Toggle -->
				<div class="view-toggle">
					<button
						type="button"
						class="toggle-btn"
						class:active={viewMode === 'edit'}
						aria-label="Edit mode"
						aria-pressed={viewMode === 'edit'}
						onclick={() => setViewMode('edit')}
					>
						<IconCode size={16} aria-hidden="true" />
						<span>Edit</span>
					</button>
					<button
						type="button"
						class="toggle-btn"
						class:active={viewMode === 'preview'}
						aria-label="Preview mode"
						aria-pressed={viewMode === 'preview'}
						onclick={() => setViewMode('preview')}
					>
						<IconEye size={16} aria-hidden="true" />
						<span>Preview</span>
					</button>
					<button
						type="button"
						class="toggle-btn"
						class:active={viewMode === 'split'}
						aria-label="Split mode"
						aria-pressed={viewMode === 'split'}
						onclick={() => setViewMode('split')}
					>
						<span>Split</span>
					</button>
				</div>

				<!-- Expand Button -->
				<button
					type="button"
					class="expand-btn"
					aria-label={isExpanded ? 'Collapse editor' : 'Expand editor'}
					onclick={toggleExpand}
				>
					{#if isExpanded}
						<IconMinimize size={16} aria-hidden="true" />
					{:else}
						<IconMaximize size={16} aria-hidden="true" />
					{/if}
				</button>
			</div>
		</header>

		<!-- XSS Warning Banner -->
		{#if showWarning}
			<div class="xss-warning" role="alert">
				<IconAlertTriangle size={16} aria-hidden="true" />
				<span>
					<strong>Security Notice:</strong> HTML is sanitized to prevent XSS attacks. Some tags and attributes
					may be removed.
				</span>
			</div>
		{/if}

		<!-- Editor Content -->
		<div class="html-editor-container view-{viewMode}">
			<!-- Code Editor -->
			{#if viewMode === 'edit' || viewMode === 'split'}
				<div class="editor-pane">
					<label class="sr-only" for="html-editor-{props.blockId}"> HTML Code Editor </label>
					<textarea
						id="html-editor-{props.blockId}"
						class="html-textarea"
						style:height={editorHeightValue}
						style:min-height={editorHeightValue}
						placeholder="Enter your HTML code here...

Example:
<div class='my-custom-section'>
  <h2>Custom Content</h2>
  <p>Your content here...</p>
</div>"
						value={rawHtml}
						oninput={handleInput}
						onkeydown={handleKeyDown}
						spellcheck="false"
						autocomplete="off"
						autocapitalize="off"
					></textarea>

					<!-- Editor Footer -->
					<div class="editor-footer">
						<div class="char-count">
							{rawHtml.length} characters
						</div>
						{#if wasSanitized}
							<div class="sanitized-badge">
								<IconShieldCheck size={14} aria-hidden="true" />
								<span>Content sanitized</span>
							</div>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Preview Pane -->
			{#if viewMode === 'preview' || viewMode === 'split'}
				<div class="preview-pane">
					<div class="preview-label">Preview</div>
					<div class="html-preview" style:min-height={editorHeightValue}>
						{#if safeHtml}
							{@html safeHtml}
						{:else}
							<div class="preview-placeholder">
								<IconEye size={24} aria-hidden="true" />
								<span>Enter HTML to see preview</span>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>

		<!-- Settings Panel -->
		{#if props.isSelected}
			<div class="html-settings">
				<label class="setting-field">
					<span>Editor Height:</span>
					<select
						value={editorHeight}
						onchange={(e) =>
							updateSettings({
								htmlEditorHeight: (e.target as HTMLSelectElement).value as EditorHeight
							})}
					>
						<option value="small">Small (150px)</option>
						<option value="medium">Medium (250px)</option>
						<option value="large">Large (400px)</option>
						<option value="auto">Auto</option>
					</select>
				</label>

				<label class="setting-checkbox">
					<input
						type="checkbox"
						checked={showWarning}
						onchange={(e) =>
							updateSettings({ htmlShowWarning: (e.target as HTMLInputElement).checked })}
					/>
					<span>Show security warning</span>
				</label>
			</div>
		{/if}
	{:else}
		<!-- View Mode: Render Sanitized HTML -->
		<div class="html-output" role="region" aria-label="Custom HTML content">
			{#if safeHtml}
				{@html safeHtml}
			{/if}
		</div>
	{/if}
</div>

<style>
	/* Screen Reader Only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* Container */
	.html-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		background: white;
	}

	.html-block.expanded {
		position: fixed;
		top: 1rem;
		left: 1rem;
		right: 1rem;
		bottom: 1rem;
		z-index: 1000;
		border-radius: 16px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	/* Header */
	.html-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border-bottom: 1px solid #e5e7eb;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 600;
		font-size: 0.875rem;
		color: #475569;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	/* View Toggle */
	.view-toggle {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		background: #e2e8f0;
		border-radius: 6px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		border: none;
		border-radius: 4px;
		background: transparent;
		color: #64748b;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toggle-btn:hover {
		color: #3b82f6;
	}

	.toggle-btn.active {
		background: white;
		color: #3b82f6;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.expand-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		border-radius: 6px;
		background: transparent;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.expand-btn:hover {
		background: #e2e8f0;
		color: #3b82f6;
	}

	/* XSS Warning */
	.xss-warning {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #fef3c7;
		border-bottom: 1px solid #fcd34d;
		color: #92400e;
		font-size: 0.8125rem;
	}

	.xss-warning strong {
		font-weight: 600;
	}

	/* Editor Container */
	.html-editor-container {
		display: flex;
	}

	.html-editor-container.view-edit,
	.html-editor-container.view-preview {
		flex-direction: column;
	}

	.html-editor-container.view-split {
		flex-direction: row;
	}

	.html-editor-container.view-split .editor-pane,
	.html-editor-container.view-split .preview-pane {
		flex: 1;
		min-width: 0;
	}

	.html-editor-container.view-split .preview-pane {
		border-left: 1px solid #e5e7eb;
	}

	/* Editor Pane */
	.editor-pane {
		display: flex;
		flex-direction: column;
	}

	.html-textarea {
		width: 100%;
		padding: 1rem;
		border: none;
		resize: vertical;
		font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		color: #1e293b;
		background: #f8fafc;
		tab-size: 2;
	}

	.html-textarea:focus {
		outline: none;
		background: white;
	}

	.html-textarea::placeholder {
		color: #94a3b8;
	}

	.expanded .html-textarea {
		flex: 1;
		resize: none;
	}

	/* Editor Footer */
	.editor-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 1rem;
		background: #f1f5f9;
		border-top: 1px solid #e5e7eb;
		font-size: 0.75rem;
		color: #64748b;
	}

	.char-count {
		font-family: 'JetBrains Mono', monospace;
	}

	.sanitized-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: #22c55e;
	}

	/* Preview Pane */
	.preview-pane {
		display: flex;
		flex-direction: column;
	}

	.preview-label {
		padding: 0.5rem 1rem;
		background: #f8fafc;
		border-bottom: 1px solid #e5e7eb;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.html-preview {
		padding: 1rem;
		overflow: auto;
	}

	.preview-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		height: 100%;
		min-height: 150px;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	/* View Mode Output */
	.html-output {
		padding: 1rem;
	}

	/* Settings Panel */
	.html-settings {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
		align-items: center;
		padding: 1rem;
		background: #f8fafc;
		border-top: 1px solid #e5e7eb;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #475569;
	}

	.setting-field select {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-checkbox {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		color: #475569;
		cursor: pointer;
	}

	.setting-checkbox input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	/* Dark Mode */
	:global(.dark) .html-block {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .html-header {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .header-title {
		color: #94a3b8;
	}

	:global(.dark) .view-toggle {
		background: #334155;
	}

	:global(.dark) .toggle-btn {
		color: #94a3b8;
	}

	:global(.dark) .toggle-btn:hover {
		color: #60a5fa;
	}

	:global(.dark) .toggle-btn.active {
		background: #1e293b;
		color: #60a5fa;
	}

	:global(.dark) .expand-btn {
		color: #94a3b8;
	}

	:global(.dark) .expand-btn:hover {
		background: #334155;
		color: #60a5fa;
	}

	:global(.dark) .xss-warning {
		background: rgba(234, 179, 8, 0.15);
		border-color: rgba(234, 179, 8, 0.3);
		color: #fde047;
	}

	:global(.dark) .html-textarea {
		background: #0f172a;
		color: #e2e8f0;
	}

	:global(.dark) .html-textarea:focus {
		background: #1e293b;
	}

	:global(.dark) .html-textarea::placeholder {
		color: #64748b;
	}

	:global(.dark) .editor-footer {
		background: #0f172a;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .preview-label {
		background: #0f172a;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .preview-placeholder {
		color: #64748b;
	}

	:global(.dark) .html-editor-container.view-split .preview-pane {
		border-color: #334155;
	}

	:global(.dark) .html-settings {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .setting-field,
	:global(.dark) .setting-checkbox {
		color: #94a3b8;
	}

	:global(.dark) .setting-field select {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}

	/* HTML Preview Content Styles */
	.html-preview :global(h1),
	.html-preview :global(h2),
	.html-preview :global(h3),
	.html-preview :global(h4),
	.html-preview :global(h5),
	.html-preview :global(h6) {
		margin-top: 1em;
		margin-bottom: 0.5em;
		font-weight: 700;
		line-height: 1.3;
	}

	.html-preview :global(p) {
		margin-bottom: 1em;
		line-height: 1.6;
	}

	.html-preview :global(ul),
	.html-preview :global(ol) {
		margin-bottom: 1em;
		padding-left: 1.5em;
	}

	.html-preview :global(a) {
		color: #3b82f6;
		text-decoration: underline;
	}

	.html-preview :global(img) {
		max-width: 100%;
		height: auto;
	}

	.html-preview :global(table) {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 1em;
	}

	.html-preview :global(th),
	.html-preview :global(td) {
		padding: 0.5rem;
		border: 1px solid #e5e7eb;
		text-align: left;
	}

	:global(.dark) .html-preview :global(a) {
		color: #60a5fa;
	}

	:global(.dark) .html-preview :global(th),
	:global(.dark) .html-preview :global(td) {
		border-color: #334155;
	}
</style>
