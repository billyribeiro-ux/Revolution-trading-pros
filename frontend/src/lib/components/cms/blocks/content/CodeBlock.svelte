<!--
/**
 * Code Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Syntax-highlighted code with language selection, copy functionality,
 * and dark theme code area with monospace font
 */
-->

<script lang="ts">
	import { IconCode, IconCopy, IconCheck } from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	let copied = $state(false);
	let copyTimeout: ReturnType<typeof setTimeout> | null = $state(null);

	const languages = [
		{ value: 'javascript', label: 'JavaScript' },
		{ value: 'typescript', label: 'TypeScript' },
		{ value: 'python', label: 'Python' },
		{ value: 'rust', label: 'Rust' },
		{ value: 'go', label: 'Go' },
		{ value: 'java', label: 'Java' },
		{ value: 'csharp', label: 'C#' },
		{ value: 'cpp', label: 'C++' },
		{ value: 'html', label: 'HTML' },
		{ value: 'css', label: 'CSS' },
		{ value: 'sql', label: 'SQL' },
		{ value: 'json', label: 'JSON' },
		{ value: 'yaml', label: 'YAML' },
		{ value: 'bash', label: 'Bash' }
	];

	const currentLanguage = $derived(
		languages.find((l) => l.value === props.block.content.language)?.label || 'JavaScript'
	);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handleCodeInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ code: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleKeyDown(e: KeyboardEvent): void {
		// Handle Tab key for indentation
		if (e.key === 'Tab') {
			e.preventDefault();
			document.execCommand('insertText', false, '  ');
		}
	}

	function handleLanguageChange(e: Event): void {
		const value = (e.target as HTMLSelectElement).value;
		updateContent({ language: value });
	}

	async function copyCode(): Promise<void> {
		try {
			await navigator.clipboard.writeText(props.block.content.code || '');
			copied = true;

			// Clear any existing timeout
			if (copyTimeout) {
				clearTimeout(copyTimeout);
			}

			copyTimeout = setTimeout(() => {
				copied = false;
				copyTimeout = null;
			}, 2000);
		} catch (error) {
			props.onError?.(error instanceof Error ? error : new Error('Failed to copy code'));
		}
	}
</script>

<div class="code-block" role="region" aria-label="Code block">
	<header class="code-block__header">
		<div class="code-block__header-left">
			<span class="code-block__icon" aria-hidden="true">
				<IconCode size={16} />
			</span>
			<select
				value={props.block.content.language || 'javascript'}
				onchange={handleLanguageChange}
				disabled={!props.isEditing}
				aria-label="Select programming language"
				class="code-block__language-select"
			>
				{#each languages as lang}
					<option value={lang.value}>{lang.label}</option>
				{/each}
			</select>
		</div>

		<div class="code-block__header-right">
			{#if !props.isEditing && props.block.content.code}
				<span class="code-block__language-badge">{currentLanguage}</span>
			{/if}
			<button
				type="button"
				class="code-block__copy-btn"
				class:code-block__copy-btn--copied={copied}
				onclick={copyCode}
				aria-label={copied ? 'Copied!' : 'Copy code'}
				disabled={!props.block.content.code}
			>
				{#if copied}
					<IconCheck size={16} aria-hidden="true" />
					<span class="code-block__copy-text">Copied!</span>
				{:else}
					<IconCopy size={16} aria-hidden="true" />
					<span class="code-block__copy-text">Copy</span>
				{/if}
			</button>
		</div>
	</header>

	<div class="code-block__content">
		<pre class="code-block__pre"><code
				contenteditable={props.isEditing}
				class="code-block__code language-{props.block.content.language || 'javascript'}"
				class:code-block__code--placeholder={!props.block.content.code}
				class:code-block__code--editable={props.isEditing}
				oninput={handleCodeInput}
				onpaste={handlePaste}
				onkeydown={handleKeyDown}
				data-placeholder="// Enter your code here..."
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Code editor' : 'Code content'}
				aria-multiline="true"
				spellcheck="false"
			>{props.block.content.code || ''}</code
			></pre>
	</div>

	{#if props.isEditing}
		<footer class="code-block__footer">
			<span class="code-block__hint">
				Press Tab for indentation. Paste code directly.
			</span>
		</footer>
	{/if}
</div>

<style>
	/* =========================================================================
	 * Code Block - BEM Structure
	 * ========================================================================= */

	.code-block {
		border-radius: 12px;
		overflow: hidden;
		background: #1e293b;
		border: 1px solid #334155;
		font-family: 'Fira Code', 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
	}

	/* Header
	 * ------------------------------------------------------------------------- */
	.code-block__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.625rem 1rem;
		background: #0f172a;
		border-bottom: 1px solid #334155;
		gap: 0.75rem;
	}

	.code-block__header-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.code-block__header-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.code-block__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		color: #60a5fa;
	}

	.code-block__language-select {
		padding: 0.375rem 0.75rem;
		padding-right: 1.75rem;
		background: #334155;
		color: #e2e8f0;
		border: 1px solid #475569;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		font-family: inherit;
		cursor: pointer;
		outline: none;
		transition: all 0.15s ease;
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.5rem center;
	}

	.code-block__language-select:hover:not(:disabled) {
		background-color: #475569;
		border-color: #64748b;
	}

	.code-block__language-select:focus-visible {
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.2);
	}

	.code-block__language-select:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.code-block__language-badge {
		padding: 0.25rem 0.625rem;
		background: #334155;
		color: #94a3b8;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	/* Copy Button
	 * ------------------------------------------------------------------------- */
	.code-block__copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: #334155;
		border: 1px solid #475569;
		border-radius: 6px;
		color: #e2e8f0;
		font-size: 0.8125rem;
		font-weight: 500;
		font-family: inherit;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.code-block__copy-btn:hover:not(:disabled) {
		background: #475569;
		border-color: #64748b;
	}

	.code-block__copy-btn:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.code-block__copy-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.code-block__copy-btn--copied {
		background: #065f46;
		border-color: #10b981;
		color: #a7f3d0;
	}

	.code-block__copy-btn--copied:hover:not(:disabled) {
		background: #047857;
	}

	.code-block__copy-text {
		display: none;
	}

	@media (min-width: 480px) {
		.code-block__copy-text {
			display: inline;
		}
	}

	/* Content Area
	 * ------------------------------------------------------------------------- */
	.code-block__content {
		position: relative;
	}

	.code-block__pre {
		margin: 0;
		padding: 1.25rem;
		overflow-x: auto;
		scrollbar-width: thin;
		scrollbar-color: #475569 #1e293b;
	}

	.code-block__pre::-webkit-scrollbar {
		height: 8px;
		width: 8px;
	}

	.code-block__pre::-webkit-scrollbar-track {
		background: #1e293b;
	}

	.code-block__pre::-webkit-scrollbar-thumb {
		background: #475569;
		border-radius: 4px;
	}

	.code-block__pre::-webkit-scrollbar-thumb:hover {
		background: #64748b;
	}

	.code-block__code {
		font-family: 'Fira Code', 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
		font-size: 0.875rem;
		line-height: 1.7;
		color: #e2e8f0;
		outline: none;
		display: block;
		white-space: pre;
		tab-size: 2;
		min-height: 1.7em;
	}

	.code-block__code--editable {
		cursor: text;
		border-radius: 4px;
		transition: background-color 0.15s ease;
	}

	.code-block__code--editable:focus {
		background: rgba(96, 165, 250, 0.05);
	}

	.code-block__code--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #64748b;
		pointer-events: none;
		font-style: italic;
	}

	/* Footer
	 * ------------------------------------------------------------------------- */
	.code-block__footer {
		padding: 0.5rem 1rem;
		background: #0f172a;
		border-top: 1px solid #334155;
	}

	.code-block__hint {
		font-size: 0.75rem;
		color: #64748b;
		font-family: system-ui, -apple-system, sans-serif;
	}

	/* =========================================================================
	 * Dark Mode (already dark by default, but for consistency)
	 * ========================================================================= */
	:global(.dark) .code-block {
		background: #0f172a;
		border-color: #1e293b;
	}

	:global(.dark) .code-block__header {
		background: #020617;
		border-color: #1e293b;
	}

	:global(.dark) .code-block__language-select {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .code-block__language-select:hover:not(:disabled) {
		background: #334155;
	}

	:global(.dark) .code-block__copy-btn {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .code-block__copy-btn:hover:not(:disabled) {
		background: #334155;
	}

	:global(.dark) .code-block__pre::-webkit-scrollbar-track {
		background: #0f172a;
	}

	:global(.dark) .code-block__footer {
		background: #020617;
		border-color: #1e293b;
	}
</style>
