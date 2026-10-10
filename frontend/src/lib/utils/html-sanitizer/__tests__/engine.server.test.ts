// @vitest-environment node
/**
 * Sanitizer on the server parser, with no DOM at all — the conditions SSR runs
 * under in Node and in Cloudflare's workerd. The `imports` condition that picks
 * the parser is a build-time concern, so it is pinned here explicitly.
 */

import { describe, expect, it, vi } from 'vitest';

vi.mock('#lib/utils/html-sanitizer/parse-fragment.js', () => import('../parse-fragment.server.js'));

import type { Element, Nodes } from 'hast';
import { extractText, sanitizeWithPolicy } from '#lib/utils/html-sanitizer/engine.js';
import { safeUrl } from '#lib/utils/sanitize.js';
import { parseHtmlFragment } from '../parse-fragment.server.js';
import { CASES, POLICIES, TEXT_CASES, XSS_PAYLOADS } from './fixtures.js';

const FORBIDDEN_ELEMENTS = [
	'script',
	'style',
	'svg',
	'math',
	'object',
	'embed',
	'base',
	'meta',
	'form',
	'input'
];

function* elements(node: Nodes): Generator<Element> {
	if (node.type === 'element') yield node;
	if ('children' in node) for (const child of node.children) yield* elements(child);
}

describe('html sanitizer — server parser', () => {
	it('runs without a DOM', () => {
		expect(typeof document).toBe('undefined');
		expect(typeof window).toBe('undefined');
	});

	describe.each(CASES)('$name', ({ html, policy, expected }) => {
		it(`produces the pinned output (${policy})`, () => {
			expect(sanitizeWithPolicy(html, POLICIES[policy])).toBe(expected);
		});
	});

	describe.each(TEXT_CASES)('text: $name', ({ html, expected }) => {
		it('extracts plain text', () => {
			expect(extractText(html)).toBe(expected);
		});
	});

	describe.each(Object.keys(POLICIES) as Array<keyof typeof POLICIES>)('policy %s', (policy) => {
		it.each(XSS_PAYLOADS)('leaves nothing executable once re-parsed for %s', (payload) => {
			// Re-parse the output as a browser would and inspect the tree, not the
			// string: payload text may legitimately survive inside quoted values.
			const reparsed = parseHtmlFragment(sanitizeWithPolicy(payload, POLICIES[policy]));
			for (const element of elements(reparsed)) {
				expect(FORBIDDEN_ELEMENTS).not.toContain(element.tagName);
				for (const [property, value] of Object.entries(element.properties)) {
					expect(property).not.toMatch(/^on/i);
					expect(String(value).trim()).not.toMatch(/^(javascript|vbscript):/i);
				}
			}
		});
	});

	it('caps tree depth the way Chromium does instead of exhausting the stack', () => {
		// parse5 has no depth limit; a few thousand nested tags used to throw a
		// RangeError and fail the whole SSR render.
		const html = '<div>'.repeat(3000) + 'deepest';
		const reparsed = parseHtmlFragment(sanitizeWithPolicy(html, POLICIES.standard));
		let maxDepth = 0;
		const stack: Array<[Nodes, number]> = [[reparsed, 0]];
		while (stack.length > 0) {
			const [node, depth] = stack.pop() as [Nodes, number];
			maxDepth = Math.max(maxDepth, depth);
			if ('children' in node) for (const child of node.children) stack.push([child, depth + 1]);
		}
		// Chromium/WebKit: nothing nests deeper than 511 elements below <body>
		// (plus the text node).
		expect(maxDepth).toBeLessThanOrEqual(512);
		expect(extractText(html)).toBe('deepest');
	});

	describe('safeUrl', () => {
		it.each([
			'/p',
			'#anchor',
			'../up',
			'./here',
			'?q=1',
			'/p:x',
			'https://x.test/a',
			'mailto:a@b.c',
			'tel:+1'
		])('keeps %s', (url) => {
			expect(safeUrl(url)).toBe(url);
		});
		it.each([
			'javascript:alert(1)',
			'java\tscript:alert(1)',
			' JAVASCRIPT:alert(1)',
			'\u0001javascript:alert(1)',
			'vbscript:x',
			'data:text/html,x',
			'file:///etc/passwd'
		])('rejects %j', (url) => {
			expect(safeUrl(url)).toBe('');
		});
		it('returns an empty string for missing input', () => {
			expect(safeUrl(undefined)).toBe('');
			expect(safeUrl(null)).toBe('');
		});
	});

	it('returns an empty string for empty input', () => {
		expect(sanitizeWithPolicy('', POLICIES.standard)).toBe('');
		expect(extractText('')).toBe('');
	});
});
