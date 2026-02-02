<!--
/**
 * Code Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Syntax-highlighted code with language selection
 */
-->

<script lang="ts">
	import { IconCopy, IconCheck } from '$lib/icons';
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

	const props: Props = $props();

	let copied = $state(false);

	const languages = [
		'javascript',
		'typescript',
		'python',
		'html',
		'css',
		'json',
		'bash',
		'sql',
		'php',
		'rust',
		'go',
		'java',
		'csharp',
		'ruby'
	];

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

	function handleLanguageChange(e: Event): void {
		const value = (e.target as HTMLSelectElement).value;
		updateContent({ language: value });
	}

	async function copyCode(): Promise<void> {
		try {
			await navigator.clipboard.writeText(props.block.content.code || '');
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (error) {
			props.onError?.(error instanceof Error ? error : new Error('Failed to copy code'));
		}
	}
</script>

<div class="code-block" role="region" aria-label="Code block">
	<div class="code-header">
		<select
			value={props.block.content.language || 'javascript'}
			onchange={handleLanguageChange}
			disabled={!props.isEditing}
			aria-label="Select programming language"
			class="language-select"
		>
			{#each languages as lang}
				<option value={lang}>{lang.charAt(0).toUpperCase() + lang.slice(1)}</option>
			{/each}
		</select>

		{#if !props.isEditing && props.block.content.code}
			<button
				type="button"
				class="copy-btn"
				onclick={copyCode}
				aria-label={copied ? 'Copied!' : 'Copy code'}
			>
				{#if copied}
					<IconCheck size={16} aria-hidden="true" />
					<span>Copied!</span>
				{:else}
					<IconCopy size={16} aria-hidden="true" />
					<span>Copy</span>
				{/if}
			</button>
		{/if}
	</div>

	<pre class="code-content"><code
			contenteditable={props.isEditing}
			class="language-{props.block.content.language || 'javascript'}"
			class:placeholder={!props.block.content.code}
			oninput={handleCodeInput}
			onpaste={handlePaste}
			data-placeholder="// Enter your code here..."
			role={props.isEditing ? 'textbox' : undefined}
			aria-label={props.isEditing ? 'Code editor' : undefined}
			aria-multiline="true"
			spellcheck="false"
		>{props.block.content.code || ''}</code
		></pre>
</div>

<style>
	.code-block {
		border-radius: 12px;
		overflow: hidden;
		background: #1e293b;
		border: 1px solid #334155;
	}

	.code-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.625rem 1rem;
		background: #0f172a;
		border-bottom: 1px solid #334155;
	}

	.language-select {
		padding: 0.375rem 0.75rem;
		background: #334155;
		color: #e2e8f0;
		border: 1px solid #475569;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		outline: none;
		transition: all 0.15s;
	}

	.language-select:hover:not(:disabled) {
		background: #475569;
	}

	.language-select:focus-visible {
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.2);
	}

	.language-select:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.copy-btn {
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
		cursor: pointer;
		transition: all 0.15s;
	}

	.copy-btn:hover {
		background: #475569;
	}

	.copy-btn:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.code-content {
		margin: 0;
		padding: 1.25rem;
		overflow-x: auto;
		scrollbar-width: thin;
		scrollbar-color: #475569 #1e293b;
	}

	.code-content::-webkit-scrollbar {
		height: 8px;
	}

	.code-content::-webkit-scrollbar-track {
		background: #1e293b;
	}

	.code-content::-webkit-scrollbar-thumb {
		background: #475569;
		border-radius: 4px;
	}

	.code-content::-webkit-scrollbar-thumb:hover {
		background: #64748b;
	}

	.code-content code {
		font-family: 'Fira Code', 'JetBrains Mono', 'Consolas', monospace;
		font-size: 0.875rem;
		line-height: 1.6;
		color: #e2e8f0;
		outline: none;
		display: block;
		white-space: pre;
	}

	code.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #64748b;
		pointer-events: none;
	}

	code {
		tab-size: 2;
	}
</style>
