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

	it('returns an empty string for empty input', () => {
		expect(sanitizeWithPolicy('', POLICIES.standard)).toBe('');
		expect(extractText('')).toBe('');
	});
});
