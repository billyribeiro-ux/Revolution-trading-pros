<!--
	Code Block - Syntax-highlighted code
	═══════════════════════════════════════════════════════════════════════════════

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(block.data as { code?: string; language?: string; filename?: string });

	const languages = [
		'javascript', 'typescript', 'python', 'rust', 'go', 'java',
		'html', 'css', 'json', 'sql', 'bash', 'markdown', 'text'
	];

	function handleCodeChange(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		onUpdate?.({ ...data, code: target.value });
	}

	function handleLanguageChange(e: Event) {
		const target = e.target as HTMLSelectElement;
		onUpdate?.({ ...data, language: target.value });
	}

	function handleFilenameChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, filename: target.value });
	}
</script>

<div class="code-block">
	<div class="code-header">
		{#if !readonly}
			<select
				class="language-select"
				value={data.language ?? 'javascript'}
				onchange={handleLanguageChange}
			>
				{#each languages as lang}
					<option value={lang}>{lang}</option>
				{/each}
			</select>
			<input
				type="text"
				class="filename-input"
				value={data.filename ?? ''}
				oninput={handleFilenameChange}
				placeholder="filename.js"
			/>
		{:else if data.filename}
			<span class="filename">{data.filename}</span>
		{:else}
			<span class="language">{data.language ?? 'javascript'}</span>
		{/if}
	</div>
	<div class="code-content">
		{#if readonly}
			<pre><code>{data.code || ''}</code></pre>
		{:else}
			<textarea
				class="code-input"
				value={data.code ?? ''}
				oninput={handleCodeChange}
				placeholder="// Enter your code here..."
				spellcheck="false"
				rows="8"
			></textarea>
		{/if}
	</div>
</div>

<style>
	.code-block {
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.code-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.language-select {
		padding: 0.25rem 0.5rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.25rem;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.filename-input {
		flex: 1;
		padding: 0.25rem 0.5rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.25rem;
		color: #f1f5f9;
		font-size: 0.75rem;
		font-family: 'Fira Code', monospace;
	}

	.filename, .language {
		font-size: 0.75rem;
		color: #64748b;
		font-family: 'Fira Code', monospace;
	}

	.code-content {
		background: rgba(0, 0, 0, 0.2);
	}

	.code-content pre {
		margin: 0;
		padding: 1rem;
		overflow-x: auto;
	}

	.code-content code {
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.8125rem;
		color: #e2e8f0;
		line-height: 1.6;
		tab-size: 2;
	}

	.code-input {
		width: 100%;
		padding: 1rem;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-family: 'Fira Code', 'Monaco', monospace;
		font-size: 0.8125rem;
		line-height: 1.6;
		tab-size: 2;
		resize: vertical;
	}

	.code-input:focus {
		outline: none;
	}
</style>
