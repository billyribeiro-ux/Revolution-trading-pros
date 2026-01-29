/**
 * Markdown Parser Utility
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Lightweight markdown to HTML converter for the CMS editor.
 * Supports common markdown syntax without external dependencies.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

export interface ParseOptions {
	/** Enable GitHub-flavored markdown extensions */
	gfm?: boolean;
	/** Enable syntax highlighting for code blocks */
	highlight?: boolean;
	/** Sanitize HTML output */
	sanitize?: boolean;
	/** Custom link target */
	linkTarget?: '_self' | '_blank';
}

const defaultOptions: ParseOptions = {
	gfm: true,
	highlight: false,
	sanitize: true,
	linkTarget: '_blank'
};

/**
 * Convert markdown to HTML
 */
export function markdownToHtml(markdown: string, options: ParseOptions = {}): string {
	const opts = { ...defaultOptions, ...options };
	let html = markdown;

	// Escape HTML entities if sanitizing
	if (opts.sanitize) {
		html = html.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
	}

	// Code blocks (fenced with ```)
	html = html.replace(/```(\w+)?\n([\s\S]*?)```/g, (_, lang, code) => {
		const language = lang ? ` class="language-${lang}"` : '';
		return `<pre><code${language}>${code.trim()}</code></pre>`;
	});

	// Inline code
	html = html.replace(/`([^`]+)`/g, '<code>$1</code>');

	// Headers
	html = html.replace(/^######\s+(.+)$/gm, '<h6>$1</h6>');
	html = html.replace(/^#####\s+(.+)$/gm, '<h5>$1</h5>');
	html = html.replace(/^####\s+(.+)$/gm, '<h4>$1</h4>');
	html = html.replace(/^###\s+(.+)$/gm, '<h3>$1</h3>');
	html = html.replace(/^##\s+(.+)$/gm, '<h2>$1</h2>');
	html = html.replace(/^#\s+(.+)$/gm, '<h1>$1</h1>');

	// Horizontal rules
	html = html.replace(/^(?:---|\*\*\*|___)$/gm, '<hr>');

	// Blockquotes
	html = html.replace(/^>\s+(.+)$/gm, '<blockquote>$1</blockquote>');
	// Merge adjacent blockquotes
	html = html.replace(/<\/blockquote>\n<blockquote>/g, '\n');

	// Unordered lists
	html = html.replace(/^[-*+]\s+(.+)$/gm, '<li>$1</li>');
	html = html.replace(/(<li>.*<\/li>\n?)+/g, (match) => `<ul>\n${match}</ul>\n`);

	// Ordered lists
	html = html.replace(/^\d+\.\s+(.+)$/gm, '<li>$1</li>');
	// Note: This is a simplified approach - proper parsing would distinguish between ul and ol

	// Bold and italic (order matters)
	html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
	html = html.replace(/___(.+?)___/g, '<strong><em>$1</em></strong>');
	html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
	html = html.replace(/__(.+?)__/g, '<strong>$1</strong>');
	html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');
	html = html.replace(/_(.+?)_/g, '<em>$1</em>');

	// Strikethrough (GFM)
	if (opts.gfm) {
		html = html.replace(/~~(.+?)~~/g, '<del>$1</del>');
	}

	// Links
	const target = opts.linkTarget ? ` target="${opts.linkTarget}"` : '';
	html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, `<a href="$2"${target}>$1</a>`);

	// Images
	html = html.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1">');

	// Task lists (GFM)
	if (opts.gfm) {
		html = html.replace(
			/<li>\[x\]\s*/gi,
			'<li class="task-item task-done"><input type="checkbox" checked disabled> '
		);
		html = html.replace(
			/<li>\[ \]\s*/gi,
			'<li class="task-item"><input type="checkbox" disabled> '
		);
	}

	// Tables (GFM) - basic support
	if (opts.gfm) {
		html = parseTable(html);
	}

	// Paragraphs (wrap remaining text blocks)
	html = html
		.split(/\n\n+/)
		.map((block) => {
			const trimmed = block.trim();
			if (!trimmed) return '';
			// Don't wrap if already a block element
			if (/^<(h[1-6]|p|div|ul|ol|li|blockquote|pre|table|hr)/i.test(trimmed)) {
				return trimmed;
			}
			return `<p>${trimmed}</p>`;
		})
		.join('\n\n');

	// Clean up extra whitespace
	html = html.replace(/\n{3,}/g, '\n\n').trim();

	return html;
}

/**
 * Parse GFM tables
 */
function parseTable(html: string): string {
	const tableRegex = /^(\|.+\|)\n(\|[-:| ]+\|)\n((?:\|.+\|\n?)+)/gm;

	return html.replace(tableRegex, (_, headerRow, separatorRow, bodyRows) => {
		// Parse header
		const headers = headerRow
			.split('|')
			.filter((c: string) => c.trim())
			.map((c: string) => `<th>${c.trim()}</th>`)
			.join('');

		// Parse alignment from separator
		const alignments = separatorRow
			.split('|')
			.filter((c: string) => c.trim())
			.map((c: string) => {
				const trimmed = c.trim();
				if (trimmed.startsWith(':') && trimmed.endsWith(':')) return 'center';
				if (trimmed.endsWith(':')) return 'right';
				return 'left';
			});

		// Parse body rows
		const rows = bodyRows
			.trim()
			.split('\n')
			.map((row: string) => {
				const cells = row
					.split('|')
					.filter((c: string) => c.trim())
					.map((c: string, i: number) => {
						const align = alignments[i] ? ` style="text-align:${alignments[i]}"` : '';
						return `<td${align}>${c.trim()}</td>`;
					})
					.join('');
				return `<tr>${cells}</tr>`;
			})
			.join('\n');

		return `<table>\n<thead>\n<tr>${headers}</tr>\n</thead>\n<tbody>\n${rows}\n</tbody>\n</table>`;
	});
}

/**
 * Convert HTML to Markdown
 */
export function htmlToMarkdown(html: string): string {
	let md = html;

	// Remove doctype, html, head, body tags
	md = md.replace(/<!DOCTYPE[^>]*>/gi, '');
	md = md.replace(/<\/?(?:html|head|body)[^>]*>/gi, '');

	// Code blocks
	md = md.replace(
		/<pre><code(?:\s+class="language-(\w+)")?>([\s\S]*?)<\/code><\/pre>/gi,
		(_, lang, code) => {
			const language = lang || '';
			return `\`\`\`${language}\n${decodeHtmlEntities(code)}\n\`\`\``;
		}
	);

	// Inline code
	md = md.replace(/<code>([^<]+)<\/code>/gi, '`$1`');

	// Headers
	md = md.replace(/<h1[^>]*>(.*?)<\/h1>/gi, '# $1\n');
	md = md.replace(/<h2[^>]*>(.*?)<\/h2>/gi, '## $1\n');
	md = md.replace(/<h3[^>]*>(.*?)<\/h3>/gi, '### $1\n');
	md = md.replace(/<h4[^>]*>(.*?)<\/h4>/gi, '#### $1\n');
	md = md.replace(/<h5[^>]*>(.*?)<\/h5>/gi, '##### $1\n');
	md = md.replace(/<h6[^>]*>(.*?)<\/h6>/gi, '###### $1\n');

	// Bold and italic
	md = md.replace(/<strong><em>(.*?)<\/em><\/strong>/gi, '***$1***');
	md = md.replace(/<em><strong>(.*?)<\/strong><\/em>/gi, '***$1***');
	md = md.replace(/<strong>(.*?)<\/strong>/gi, '**$1**');
	md = md.replace(/<b>(.*?)<\/b>/gi, '**$1**');
	md = md.replace(/<em>(.*?)<\/em>/gi, '*$1*');
	md = md.replace(/<i>(.*?)<\/i>/gi, '*$1*');

	// Strikethrough
	md = md.replace(/<del>(.*?)<\/del>/gi, '~~$1~~');
	md = md.replace(/<s>(.*?)<\/s>/gi, '~~$1~~');

	// Links
	md = md.replace(/<a[^>]+href="([^"]*)"[^>]*>(.*?)<\/a>/gi, '[$2]($1)');

	// Images
	md = md.replace(/<img[^>]+src="([^"]*)"[^>]*alt="([^"]*)"[^>]*\/?>/gi, '![$2]($1)');
	md = md.replace(/<img[^>]+alt="([^"]*)"[^>]*src="([^"]*)"[^>]*\/?>/gi, '![$1]($2)');
	md = md.replace(/<img[^>]+src="([^"]*)"[^>]*\/?>/gi, '![]($1)');

	// Unordered lists
	md = md.replace(/<ul[^>]*>([\s\S]*?)<\/ul>/gi, (_, content) => {
		return content.replace(/<li[^>]*>(.*?)<\/li>/gi, '- $1\n') + '\n';
	});

	// Ordered lists
	md = md.replace(/<ol[^>]*>([\s\S]*?)<\/ol>/gi, (_, content) => {
		let index = 1;
		return content.replace(/<li[^>]*>(.*?)<\/li>/gi, () => `${index++}. $1\n`) + '\n';
	});

	// Blockquotes
	md = md.replace(/<blockquote[^>]*>([\s\S]*?)<\/blockquote>/gi, (_, content) => {
		return (
			content
				.trim()
				.split('\n')
				.map((line: string) => `> ${line}`)
				.join('\n') + '\n'
		);
	});

	// Horizontal rules
	md = md.replace(/<hr\s*\/?>/gi, '\n---\n');

	// Paragraphs
	md = md.replace(/<p[^>]*>(.*?)<\/p>/gi, '$1\n\n');

	// Line breaks
	md = md.replace(/<br\s*\/?>/gi, '\n');

	// Remove remaining HTML tags
	md = md.replace(/<[^>]+>/g, '');

	// Decode HTML entities
	md = decodeHtmlEntities(md);

	// Clean up whitespace
	md = md.replace(/\n{3,}/g, '\n\n').trim();

	return md;
}

/**
 * Decode HTML entities
 */
function decodeHtmlEntities(text: string): string {
	return text
		.replace(/&amp;/g, '&')
		.replace(/&lt;/g, '<')
		.replace(/&gt;/g, '>')
		.replace(/&quot;/g, '"')
		.replace(/&#39;/g, "'")
		.replace(/&nbsp;/g, ' ');
}

/**
 * Check if content appears to be markdown
 */
export function isMarkdown(content: string): boolean {
	const markdownIndicators = [
		/^#{1,6}\s+/m, // Headers
		/\*\*[^*]+\*\*/, // Bold
		/\*[^*]+\*/, // Italic
		/^\s*[-*+]\s+/m, // Unordered list
		/^\s*\d+\.\s+/m, // Ordered list
		/\[.+\]\(.+\)/, // Links
		/!\[.+\]\(.+\)/, // Images
		/^```/m, // Code blocks
		/^>/m, // Blockquotes
		/^---$/m // Horizontal rules
	];

	return markdownIndicators.some((pattern) => pattern.test(content));
}

/**
 * Check if content appears to be HTML
 */
export function isHtml(content: string): boolean {
	return /<[a-z][^>]*>/i.test(content);
}

/**
 * Auto-detect content format
 */
export function detectContentFormat(content: string): 'markdown' | 'html' | 'raw' {
	if (!content || content.trim() === '') return 'raw';
	if (isHtml(content)) return 'html';
	if (isMarkdown(content)) return 'markdown';
	return 'raw';
}
