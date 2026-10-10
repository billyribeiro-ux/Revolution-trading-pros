/**
 * Sanitizer on the browser parser (jsdom). `#lib/utils/html-sanitizer/parse-fragment.js`
 * resolves through the package.json `imports` `browser` condition here, exactly
 * as it does in the client build.
 *
 * The pinned outputs are shared with `engine.server.test.ts`: both suites
 * passing means server-rendered `{@html}` content is byte-identical to the
 * client's.
 */

import type { Nodes } from 'hast';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { extractText, sanitizeWithPolicy } from '#lib/utils/html-sanitizer/engine.js';
import { parseHtmlFragment as parseInBrowser } from '../parse-fragment.browser.js';
import { parseHtmlFragment as parseOnServer } from '../parse-fragment.server.js';
import { CASES, POLICIES, TEXT_CASES, XSS_PAYLOADS } from './fixtures.js';

/**
 * The tree without parser bookkeeping: source positions and `data.quirksMode`
 * are recorded only by the parse5 path and never reach the serializer.
 */
function structure(node: Nodes): unknown {
	return JSON.parse(
		JSON.stringify(node, (key, value: unknown) =>
			key === 'position' || key === 'data' ? undefined : value
		)
	);
}

describe('html sanitizer — browser parser', () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it('parses with the browser’s DOMParser under the browser condition', () => {
		const parseFromString = vi.spyOn(DOMParser.prototype, 'parseFromString');
		sanitizeWithPolicy('<p>x</p>', POLICIES.standard);
		expect(parseFromString).toHaveBeenCalledOnce();
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
		it.each(XSS_PAYLOADS)('leaves nothing executable in the live DOM for %s', (payload) => {
			const host = document.createElement('div');
			host.innerHTML = sanitizeWithPolicy(payload, POLICIES[policy]);
			for (const element of host.querySelectorAll('*')) {
				expect([
					'SCRIPT',
					'STYLE',
					'OBJECT',
					'EMBED',
					'BASE',
					'META',
					'FORM',
					'INPUT'
				]).not.toContain(element.tagName);
				expect(element.namespaceURI).toBe('http://www.w3.org/1999/xhtml');
				for (const attribute of element.attributes) {
					expect(attribute.name).not.toMatch(/^on/i);
					expect(attribute.value.trim()).not.toMatch(/^(javascript|vbscript):/i);
				}
			}
		});
	});

	describe('parsers agree', () => {
		const inputs = [...CASES.map(({ html }) => html), ...XSS_PAYLOADS];
		it.each(inputs)('builds the same tree on server and browser for %s', (html) => {
			expect(structure(parseOnServer(html))).toEqual(structure(parseInBrowser(html)));
		});
	});
});
