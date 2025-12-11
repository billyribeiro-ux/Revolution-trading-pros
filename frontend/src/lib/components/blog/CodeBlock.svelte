<script lang="ts">
	/**
	 * Code Block Component with Syntax Highlighting - Svelte 5
	 * Uses Prism.js for syntax highlighting with copy functionality
	 *
	 * @version 1.0.0 - December 2024
	 */
	import { browser } from '$app/environment';

	interface Props {
		/** Code content */
		code: string;
		/** Programming language */
		language?: string;
		/** Show line numbers */
		showLineNumbers?: boolean;
		/** Filename to display */
		filename?: string;
		/** Highlight specific lines (e.g., "1,3-5,10") */
		highlightLines?: string;
		/** Max height before scrolling */
		maxHeight?: string;
	}

	let {
		code,
		language = 'plaintext',
		showLineNumbers = true,
		filename = '',
		highlightLines = '',
		maxHeight = '500px'
	}: Props = $props();

	let copied = $state(false);
	let highlightedCode = $state(code);

	// Parse highlight lines
	let highlightedLineSet = $derived.by(() => {
		if (!highlightLines) return new Set<number>();

		const lines = new Set<number>();
		const parts = highlightLines.split(',');

		for (const part of parts) {
			if (part.includes('-')) {
				const [start, end] = part.split('-').map(Number);
				for (let i = start; i <= end; i++) {
					lines.add(i);
				}
			} else {
				lines.add(Number(part));
			}
		}

		return lines;
	});

	// Language display names
	const languageNames: Record<string, string> = {
		js: 'JavaScript',
		javascript: 'JavaScript',
		ts: 'TypeScript',
		typescript: 'TypeScript',
		jsx: 'JSX',
		tsx: 'TSX',
		html: 'HTML',
		css: 'CSS',
		scss: 'SCSS',
		json: 'JSON',
		yaml: 'YAML',
		yml: 'YAML',
		md: 'Markdown',
		markdown: 'Markdown',
		bash: 'Bash',
		shell: 'Shell',
		sh: 'Shell',
		php: 'PHP',
		python: 'Python',
		py: 'Python',
		ruby: 'Ruby',
		rb: 'Ruby',
		go: 'Go',
		rust: 'Rust',
		java: 'Java',
		kotlin: 'Kotlin',
		swift: 'Swift',
		sql: 'SQL',
		graphql: 'GraphQL',
		dockerfile: 'Dockerfile',
		svelte: 'Svelte',
		vue: 'Vue',
		plaintext: 'Text'
	};

	let displayLanguage = $derived(languageNames[language.toLowerCase()] || language);

	// Split code into lines for line numbers
	let codeLines = $derived(code.split('\n'));

	// Simple syntax highlighting (no external dependency)
	function highlight(code: string, lang: string): string {
		// Basic keyword highlighting
		const keywords: Record<string, string[]> = {
			javascript: [
				'const',
				'let',
				'var',
				'function',
				'return',
				'if',
				'else',
				'for',
				'while',
				'class',
				'import',
				'export',
				'from',
				'async',
				'await',
				'try',
				'catch',
				'throw',
				'new',
				'this',
				'typeof',
				'instanceof'
			],
			typescript: [
				'const',
				'let',
				'var',
				'function',
				'return',
				'if',
				'else',
				'for',
				'while',
				'class',
				'import',
				'export',
				'from',
				'async',
				'await',
				'try',
				'catch',
				'throw',
				'new',
				'this',
				'typeof',
				'instanceof',
				'interface',
				'type',
				'enum',
				'implements',
				'extends',
				'public',
				'private',
				'protected'
			],
			php: [
				'function',
				'return',
				'if',
				'else',
				'for',
				'while',
				'foreach',
				'class',
				'public',
				'private',
				'protected',
				'static',
				'use',
				'namespace',
				'new',
				'try',
				'catch',
				'throw',
				'array',
				'echo',
				'require',
				'include'
			],
			python: [
				'def',
				'return',
				'if',
				'else',
				'elif',
				'for',
				'while',
				'class',
				'import',
				'from',
				'try',
				'except',
				'raise',
				'with',
				'as',
				'lambda',
				'yield',
				'pass',
				'break',
				'continue'
			]
		};

		let result = escapeHtml(code);

		// Get keywords for language
		const langKeywords = keywords[lang.toLowerCase()] || keywords['javascript'] || [];

		// Highlight strings
		result = result.replace(
			/(["'`])(?:(?!\1)[^\\]|\\.)*\1/g,
			'<span class="token string">$&</span>'
		);

		// Highlight comments
		result = result.replace(
			/(\/\/.*$|\/\*[\s\S]*?\*\/|#.*$)/gm,
			'<span class="token comment">$&</span>'
		);

		// Highlight keywords
		for (const keyword of langKeywords) {
			const regex = new RegExp(`\\b(${keyword})\\b`, 'g');
			result = result.replace(regex, '<span class="token keyword">$1</span>');
		}

		// Highlight numbers
		result = result.replace(/\b(\d+\.?\d*)\b/g, '<span class="token number">$1</span>');

		// Highlight function calls
		result = result.replace(/\b([a-zA-Z_]\w*)\s*\(/g, '<span class="token function">$1</span>(');

		return result;
	}

	function escapeHtml(text: string): string {
		return text
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;')
			.replace(/"/g, '&quot;')
			.replace(/'/g, '&#039;');
	}

	$effect(() => {
		highlightedCode = highlight(code, language);
	});

	async function copyCode() {
		try {
			await navigator.clipboard.writeText(code);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy:', err);
		}
	}
</script>

<div class="code-block" style="--max-height: {maxHeight}">
	<!-- Header -->
	<div class="code-header">
		<div class="code-info">
			{#if filename}
				<span class="filename">{filename}</span>
			{/if}
			<span class="language">{displayLanguage}</span>
		</div>
		<button class="copy-btn" class:copied onclick={copyCode} aria-label="Copy code">
			{#if copied}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12" />
				</svg>
				<span>Copied!</span>
			{:else}
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
				</svg>
				<span>Copy</span>
			{/if}
		</button>
	</div>

	<!-- Code content -->
	<div class="code-content">
		{#if showLineNumbers}
			<div class="line-numbers" aria-hidden="true">
				{#each codeLines as _, i}
					<span class="line-number" class:highlighted={highlightedLineSet.has(i + 1)}
						>{i + 1}</span
					>
				{/each}
			</div>
		{/if}
		<pre class="code-pre"><code class="language-{language}">{@html highlightedCode}</code></pre>
	</div>
</div>

<style>
	.code-block {
		border-radius: 12px;
		overflow: hidden;
		background: #0f172a;
		border: 1px solid rgba(148, 163, 184, 0.1);
		margin: 1.5rem 0;
	}

	.code-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.8);
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.code-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.filename {
		font-size: 0.875rem;
		font-weight: 500;
		color: #f1f5f9;
	}

	.language {
		font-size: 0.75rem;
		color: #64748b;
		background: rgba(148, 163, 184, 0.1);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.copy-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.copy-btn:hover {
		background: rgba(148, 163, 184, 0.2);
		color: #f1f5f9;
	}

	.copy-btn.copied {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.copy-btn svg {
		width: 14px;
		height: 14px;
	}

	.code-content {
		display: flex;
		overflow: auto;
		max-height: var(--max-height);
	}

	.line-numbers {
		display: flex;
		flex-direction: column;
		padding: 1rem 0;
		background: rgba(30, 41, 59, 0.5);
		border-right: 1px solid rgba(148, 163, 184, 0.1);
		user-select: none;
		position: sticky;
		left: 0;
	}

	.line-number {
		padding: 0 0.75rem;
		font-size: 0.8125rem;
		line-height: 1.7;
		color: #475569;
		text-align: right;
		min-width: 3rem;
	}

	.line-number.highlighted {
		background: rgba(96, 165, 250, 0.1);
		color: #60a5fa;
	}

	.code-pre {
		margin: 0;
		padding: 1rem;
		overflow-x: auto;
		flex: 1;
	}

	.code-pre code {
		font-family: 'JetBrains Mono', 'Fira Code', Consolas, Monaco, monospace;
		font-size: 0.875rem;
		line-height: 1.7;
		color: #e2e8f0;
		white-space: pre;
	}

	/* Syntax highlighting */
	:global(.token.keyword) {
		color: #c678dd;
	}

	:global(.token.string) {
		color: #98c379;
	}

	:global(.token.comment) {
		color: #5c6370;
		font-style: italic;
	}

	:global(.token.number) {
		color: #d19a66;
	}

	:global(.token.function) {
		color: #61afef;
	}
</style>
